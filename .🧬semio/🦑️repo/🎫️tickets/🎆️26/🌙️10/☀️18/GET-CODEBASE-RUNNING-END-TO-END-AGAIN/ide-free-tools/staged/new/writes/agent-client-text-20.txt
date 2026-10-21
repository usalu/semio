// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚦️run carries hook execution, the hook artifacts and the hook event dispatch.

// #endregion 🧲️Header

package hooks

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	io "io"
	os "os"
	filepath "path/filepath"
	strconv "strconv"
	strings "strings"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	statutes "github.com/usalu/semio/repo/statutes"
	testrunner "github.com/usalu/semio/repo/testrunner"
	todos "github.com/usalu/semio/repo/todos"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🦀️Hooks

// 🏃️runHookExecution runs the hook pipeline (shared by CLI `hook` and per-IDE `RunHookFor`).
func runHookExecution(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	return RunHookExecutionCtx(context.Background(), client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, jsonMode, stdout, stderr)
}

func RunHookExecutionCtx(ctx context.Context, client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	done := make(chan error, 1)
	go func() {
		done <- runHookExecutionInner(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot, input, jsonMode, stdout, stderr)
	}()
	select {
	case err := <-done:
		return err
	case <-ctx.Done():
		return fmt.Errorf("hook timed out: %w", ctx.Err())
	}
}

func runHookExecutionInner(client, eventStr, toolName, toolArgs, filePath, parentInfo, repoRoot string, input json.RawMessage, jsonMode bool, stdout, stderr io.Writer) error {
	if toolName == "" {
		if tn := extractToolNameFromStdin(input); tn != "" {
			toolName = tn
		}
	}
	event, resolvedParent, err := ResolveHookEvent(eventStr, client, toolName, input)
	if err != nil {
		return err
	}
	if parentInfo == "" {
		parentInfo = resolvedParent
	}
	hctx := model.HookContext{
		Event:      event,
		Client:     client,
		Second:     time.Now().UTC().Format(time.RFC3339),
		RepoRoot:   repoRoot,
		ToolName:   toolName,
		ToolArgs:   toolArgs,
		FilePath:   filePath,
		ParentInfo: parentInfo,
		Input:      input,
	}
	result := RunHook(hctx)
	provider := providers.GetAgentClientProvider(client)
	if provider != nil && provider.Kind() == "copilot-chat" {
		hookEventName := extractHookEventNameFromStdin(input)
		if hookEventName == "" {
			hookEventName = provider.NativeEventFromHookEvent(event, parentInfo)
		}
		out := provider.FormatHookOutput(hookEventName, result)
		fmt.Fprintln(stdout, out)
		return nil
	}
	if jsonMode {
		out, _ := json.Marshal(result)
		fmt.Fprintln(stdout, string(out))
		return nil
	}
	if !result.IsAllowed() {
		fmt.Fprintln(stderr, result.GetMessage())
		return workspace.ExitError{Code: 2}
	}
	msg := result.GetMessage()
	if msg != "" {
		fmt.Fprintln(stdout, msg)
	}
	return nil
}

// #endregion 🦀️Hooks

// #region 📰️Todos

// 🔶️ResolveHookEvent resolves a hook event.
func ResolveHookEvent(name string, client string, toolName string, input json.RawMessage) (model.HookEvent, string, error) {
	if event, err := model.ValidateHookEvent(name); err == nil {
		return event, "", nil
	}
	kind := model.ToolKindGeneric
	if strings.TrimSpace(toolName) != "" {
		kind = todos.ClassifyTool(toolName)
	}
	if command := extractCommandFromStdin(input); command != "" {
		segment, _ := testrunner.ExtractTestSegmentFromCommand(command)
		commandKind := model.ClassifyCommandKind(todos.FirstNonEmpty(segment, command))
		if kind == model.ToolKindGeneric || kind == model.ToolKindTerminal || commandKind == model.ToolKindTest || commandKind == model.ToolKindBuild || commandKind == model.ToolKindCodeSearch || commandKind == model.ToolKindCodeEdit {
			kind = commandKind
		}
	}
	if toolName == "Glob" && len(input) > 0 {
		kind = model.ToolKindGeneric
	}
	provider := providers.GetAgentClientProvider(client)
	if provider != nil {
		return provider.ResolveNativeEvent(name, kind)
	}
	return providers.ResolveClaudeCompatibleEvent(name, kind)
}

// 🔹️RunHook runs a hook.
func RunHook(ctx model.HookContext) model.HookResult {
	previousRoot := workspace.GetRootDir()
	defer func() {
		workspace.SetRootDir(previousRoot)
		workspace.GitignoreMutex.Lock()
		workspace.CachedGitignore = nil
		workspace.GitignoreLoaded = false
		workspace.GitignoreMutex.Unlock()
		codebase.InvalidateTechnologyCache()
	}()
	result := runHookCore(ctx)
	writeHookArtifacts(ctx, result, SystemEnvironment{})
	trackHookInOpenTicket(ctx, result)
	return result
}

func runHookCore(ctx model.HookContext) model.HookResult {
	repoRoot := ctx.RepoRoot
	if repoRoot == "" {
		repoRoot = workspace.GetRootDir()
	}
	if repoRoot != "" {
		workspace.SetRootDir(repoRoot)
	}
	result := dispatchHook(ctx)
	hookResult, ok := result.(model.HookResult)
	if !ok {
		return model.HookResultBase{Allowed: false, Message: "invalid hook result"}
	}
	return hookResult
}

func writeHookArtifacts(ctx model.HookContext, result model.HookResult, environment HookEnvironment) {
	if HookEventKind(ctx.Event) == model.HookKindVersion {
		return
	}
	repoRoot := ctx.RepoRoot
	if repoRoot == "" {
		repoRoot = workspace.GetRootDir()
	}
	if repoRoot == "" {
		return
	}
	repoRoot = workspace.FindRepoRoot(repoRoot)
	workspace.SetRootDir(repoRoot)
	cfg := workspace.LoadRepoConfig(repoRoot)
	if !cfg.Logging.Session {
		return
	}
	now := time.Now().UTC()
	sessionID := ExtractSessionIDFromInput(ctx.Input)
	if sessionID == "" && ctx.Client == "kiro-cli" {
		sessionID = fmt.Sprintf("kiro-%d", os.Getppid())
	}
	if sessionID == "" {
		sessionID = "unknown"
	}
	logDir := filepath.Join(workspace.RepoMetaDirForRoot(repoRoot), "⚡️cache", "🤖️generated",
		model.FormatYearDir(now.Year()%100),
		model.FormatMonthDir(int(now.Month())),
		model.FormatDayDir(now.Day()),
		sessionID,
	)
	if err := os.MkdirAll(logDir, 0o755); err != nil {
		return
	}
	writeSessionHookLog(ctx, result, logDir, sessionID, cfg.Logging, environment)
	if cfg.Logging.Operations {
		logRepoOperationHook(ctx, result, logDir, now, sessionID, cfg.Logging)
	}
}

// 📓️writeSessionHookLog records one hook invocation through the session store port, so the
// filesystem is one implementation of where a session log lives rather than the only one.
func writeSessionHookLog(ctx model.HookContext, result model.HookResult, logDir string, sessionID string, lg workspace.LoggingConfig, environment HookEnvironment) {
	store := NewDirectorySessionStore(filepath.Dir(logDir))
	_, _ = RecordSessionHook(store, HookContextOf(ctx), HookResultOf(result), sessionID, lg, environment)
}

// 🎯️HookContextOf lifts the model-level hook context into the port-isolated one this module dispatches on.
func HookContextOf(ctx model.HookContext) HookContext {
	return HookContext{Event: string(ctx.Event), Client: ctx.Client, Second: ctx.Second, RepoRoot: ctx.RepoRoot, ToolName: ctx.ToolName, ToolArgs: ctx.ToolArgs, FilePath: ctx.FilePath, ParentInfo: ctx.ParentInfo, Extra: ctx.Extra, Input: ctx.Input}
}

// 🔶️HookResultOf reduces a typed hook result to the flat record the session log and the formatter read.
func HookResultOf(result model.HookResult) HookResult {
	return HookResult{Allowed: result.IsAllowed(), Message: result.GetMessage()}
}

// ⬛️logRepoOperationHook logs repo operation hook.
func logRepoOperationHook(hctx model.HookContext, result interface{}, operation string, t time.Time, msg string, lg workspace.LoggingConfig) {
	if !lg.Operations {
		return
	}
	logDir := strings.TrimSpace(operation)
	sessionID := strings.TrimSpace(msg)
	if logDir == "" || sessionID == "" {
		return
	}
	resultToolName, resultCommand := todos.ExtractHookResultToolInfo(result)
	toolName := todos.FirstNonEmpty(resultToolName, hctx.ToolName, extractToolNameFromStdin(hctx.Input))
	repoOperation := todos.DeriveRepoOpFromMCPTool(toolName)
	if repoOperation == "" {
		repoOperation = todos.DeriveRepoOpFromCLICommand(todos.FirstNonEmpty(resultCommand, hctx.ToolArgs, extractCommandFromStdin(hctx.Input)))
	}
	if repoOperation == "" {
		return
	}
	phase := "starting"
	switch hctx.Event {
	case model.HookAgentToolEnded, model.HookAgentToolPlanUpdatingEnded, model.HookAgentToolSearchEnded, model.HookAgentToolCodeEditEnded, model.HookAgentToolTestEnded, model.HookAgentToolBuildEnded, model.HookAgentToolTerminalEnded:
		phase = "ended"
	}
	metaPath := filepath.Join(logDir, "session.json")
	var meta model.SessionMeta
	if data, err := os.ReadFile(metaPath); err == nil {
		_ = json.Unmarshal(data, &meta)
	}
	inputMap := decodeHookInputMap(hctx.Input)
	second := todos.FirstNonEmpty(findNestedStringValue(inputMap, "second"), hctx.Second, t.UTC().Format(time.RFC3339))
	checkpoint := todos.ResolveEventCheckpointID(extractCheckpointSHAFromInput(hctx.Input))
	eventJSON, _ := json.Marshal(map[string]interface{}{
		"kind":        "agent." + repoOperation + "." + phase,
		"client":      hctx.Client,
		"session":     ResolveEventSessionID(sessionID),
		"second":      ResolveEventSecondID(second),
		"checkpoint":  checkpoint,
		"contributor": "unknown",
	})
	entry := model.EventEntry{Event: eventJSON}
	meta.Events = append(meta.Events, entry)
	_ = workspace.WriteJSONFile(metaPath, meta)
}

func extractSearchDefinitionReadsFromInput(input json.RawMessage, toolArgs string) []model.HookSearchDefinition {
	pages, ranges := todos.ExtractSearchFromInput(input, toolArgs)
	_ = pages
	filesToLines := map[string]map[int]struct{}{}
	addLine := func(path string, line int) {
		path = workspace.NormalizeHookPath(path)
		if path == "" {
			return
		}
		if _, ok := filesToLines[path]; !ok {
			filesToLines[path] = map[int]struct{}{}
		}
		if line > 0 {
			filesToLines[path][line] = struct{}{}
		}
	}
	for _, ref := range ranges {
		parts := strings.SplitN(ref, "#L", 2)
		path := parts[0]
		if len(parts) == 1 {
			addLine(path, 0)
			continue
		}
		lineSpec := parts[1]
		if strings.Contains(lineSpec, "-L") {
			rangeParts := strings.SplitN(lineSpec, "-L", 2)
			start, _ := strconv.Atoi(rangeParts[0])
			end, _ := strconv.Atoi(rangeParts[1])
			for line := start; line <= end; line++ {
				addLine(path, line)
			}
		} else if line, err := strconv.Atoi(lineSpec); err == nil {
			addLine(path, line)
		}
	}
	data := decodeHookInputMap(input)
	if toolInfo, ok := data["tool_info"].(map[string]interface{}); ok {
		if filePath, _ := toolInfo["file_path"].(string); filePath != "" {
			addLine(filePath, 0)
		}
		commandLine, _ := toolInfo["command_line"].(string)
		cwd, _ := toolInfo["cwd"].(string)
		if commandLine != "" && shouldExecuteSearchCommand(commandLine) {
			commandFiles, pattern := todos.ExtractSearchCommandFilesAndPattern(commandLine, cwd)
			for _, filePath := range commandFiles {
				matchedLines := todos.SearchLinesInFile(filePath, pattern)
				if len(matchedLines) == 0 {
					addLine(filePath, 0)
					continue
				}
				for _, line := range matchedLines {
					addLine(filePath, line)
				}
			}
		}
	}
	return buildDefinitionReads(filesToLines)
}

func buildDefinitionReads(filesToLines map[string]map[int]struct{}) []model.HookSearchDefinition {
	var result []model.HookSearchDefinition
	for filePath, lines := range filesToLines {
		normalized := workspace.NormalizeHookPath(filePath)
		if normalized == "" {
			continue
		}
		absPath := normalized
		if !filepath.IsAbs(absPath) {
			absPath = filepath.Join(workspace.GetRootDir(), normalized)
		}
		content, err := os.ReadFile(absPath)
		if err != nil {
			continue
		}
		if _, ok := lines[0]; ok {
			result = append(result, model.HookSearchDefinition{ID: codebase.ResolvePathToFileID(normalized), Loc: 0})
			continue
		}
		lineCount := strings.Count(string(content), "\n") + 1
		if len(lines) == 0 || spansEntireFile(lines, string(content), lineCount) {
			result = append(result, model.HookSearchDefinition{ID: codebase.ResolvePathToFileID(normalized), Loc: 0})
			continue
		}
		definitions := languages.ParseDefinitions(string(content), normalized)
		matched := false
		for _, definition := range definitions {
			loc := 0
			for line := range lines {
				if line >= definition.StartLine && line <= definition.EndLine {
					loc++
				}
			}
			if loc > 0 {
				matched = true
				result = append(result, model.HookSearchDefinition{ID: definition.GetID(), Loc: loc})
			}
		}
		if !matched {
			result = append(result, model.HookSearchDefinition{ID: codebase.ResolvePathToFileID(normalized), Loc: 0})
		}
	}
	return result
}

func shouldExecuteSearchCommand(command string) bool {
	command = strings.TrimSpace(command)
	if command == "" {
		return false
	}
	if strings.Contains(command, ">") || strings.Contains(command, "sed -i") {
		return false
	}
	segments := workspace.SplitCommandSegments(command)
	if len(segments) == 0 {
		segments = []string{command}
	}
	for _, segment := range segments {
		fields := strings.Fields(segment)
		if len(fields) == 0 {
			continue
		}
		switch filepath.Base(fields[0]) {
		case "grep", "rg", "ripgrep", "ag", "ack", "cat", "bat", "batcat", "less", "head", "tail":
			return true
		}
	}
	return false
}

func dispatchHook(ctx model.HookContext) interface{} {
	base := model.HookResultAgentBase{
		HookResultBase: model.HookResultBase{Allowed: true},
		Checkpoint:     extractCheckpointSHAFromInput(ctx.Input),
		Session:        ExtractSessionIDFromInput(ctx.Input),
		Second:         todos.FirstNonEmpty(findNestedStringValue(decodeHookInputMap(ctx.Input), "second"), ctx.Second),
		Client:         ctx.Client,
		LLM:            extractLLMFromInput(ctx.Input),
		Effort:         extractEffortFromInput(ctx.Input),
		Transcript:     ExtractTranscriptFromInput(ctx.Input),
		MessageID:      extractMessageIDFromInput(ctx.Input),
		Parent:         resolveParentSessionID("", ctx.Input),
	}
	if ctx.ParentInfo != "" {
		base.Parent = resolveParentSessionID(ctx.ParentInfo, ctx.Input)
		if ctx.ParentInfo == "subagent" {
			if agentID := findNestedStringValue(decodeHookInputMap(ctx.Input), "agent_id", "agentId"); agentID != "" {
				parentSession := extractParentFromInput(ctx.Input)
				if parentSession == "" {
					parentSession = ExtractSessionIDFromInput(ctx.Input)
				}
				base.Session = agentID
				base.Parent = normalizeParentSessionID(parentSession)
			} else if base.Parent == "" {
				base.Parent = base.Session
			}
		}
	}
	if len(ctx.Input) > 0 {
		var raw map[string]interface{}
		if err := json.Unmarshal(ctx.Input, &raw); err == nil {
			base.Raw = raw
		}
	}
	switch ctx.Event {
	case model.HookAgentStarted:
		return model.HookResultAgentStarted{HookResultAgentBase: base}
	case model.HookAgentEnded:
		return model.HookResultAgentEnded{HookResultAgentBase: base, Report: extractReportFromInput(ctx.Input)}
	case model.HookAgentPromptSubmitting:
		return model.HookResultAgentPromptSubmitting{HookResultAgentBase: base, Prompt: findNestedStringValue(decodeHookInputMap(ctx.Input), "prompt", "text")}
	case model.HookAgentCompacting:
		return model.HookResultAgentCompacting{HookResultAgentBase: base, Chat: extractChatFromInput(ctx.Input)}
	case model.HookAgentToolStarting:
		toolName := todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input))
		toolInput := todos.ExtractToolInputFromStdin(ctx.Input)
		blocked, reason := IsToolBlocked(toolName, todos.FirstNonEmpty(ctx.ToolArgs, extractTerminalCommandFromInput(ctx.Input, ctx.ToolArgs), extractCommandFromStdin(ctx.Input)))
		base.Allowed = !blocked
		base.Message = reason
		return model.HookResultAgentToolStarting{HookResultAgentBase: base, Name: toolName, Input: toolInput}
	case model.HookAgentToolEnded:
		response := todos.ExtractToolResponseFromStdin(ctx.Input)
		if len(response) == 0 {
			if output := findNestedStringValue(decodeHookInputMap(ctx.Input), "tool_output", "toolOutput", "output", "response"); output != "" {
				response, _ = json.Marshal(output)
			}
		}
		return model.HookResultAgentToolEnded{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: todos.ExtractToolInputFromStdin(ctx.Input), Response: response}
	case model.HookAgentToolPlanUpdatingStarting, model.HookAgentToolPlanUpdatingEnded:
		return model.HookResultAgentToolPlanUpdating{HookResultAgentBase: base, Steps: todos.ExtractPlanStepsFromInput(ctx.Input, ctx.ToolArgs)}
	case model.HookAgentToolSearchStarting:
		pages, ranges := todos.ExtractSearchFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolSearchStarting{HookResultAgentBase: base, Pages: pages, Ranges: ranges, Definitions: extractSearchDefinitionReadsFromInput(ctx.Input, ctx.ToolArgs)}
	case model.HookAgentToolSearchEnded:
		pages, ranges := todos.ExtractSearchFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolSearchEnded{HookResultAgentBase: base, Pages: pages, Ranges: ranges, Definitions: extractSearchDefinitionReadsFromInput(ctx.Input, ctx.ToolArgs)}
	case model.HookAgentToolCodeEditStarting:
		path, oldText, newText, all := extractCodeEditFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolCodeEditStarting{HookResultAgentBase: base, Path: path, Old: oldText, New: newText, All: all}
	case model.HookAgentToolCodeEditEnded:
		path, oldText, newText, _ := extractCodeEditFromInput(ctx.Input, ctx.ToolArgs)
		// Auto-format the edited file.
		if path != "" {
			statutes.RunFormatterForFile(path)
		}
		return model.HookResultAgentToolCodeEditEnded{HookResultAgentBase: base, Path: path, Old: oldText, New: newText}
	case model.HookAgentToolTestStarting:
		labs, tests, timeout := extractTestStartingFromInput(ctx.Input, ctx.ToolArgs)
		return model.HookResultAgentToolTestStarting{HookResultAgentBase: base, Labs: labs, Tests: tests, Timeout: timeout}
	case model.HookAgentToolTestEnded:
		files, succeeded, failed := extractTestEndedFromInput(ctx.Input)
		return model.HookResultAgentToolTestEnded{HookResultAgentBase: base, Files: files, Succeeded: succeeded, Failed: failed}
	case model.HookAgentToolBuildStarting:
		bundles := extractBuildBundlesFromInput(ctx.Input, ctx.ToolArgs)
		payload, _ := json.Marshal(map[string]interface{}{"bundles": bundles})
		return model.HookResultAgentToolStarting{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, "build"), Input: payload}
	case model.HookAgentToolBuildEnded:
		succeeded, failed := extractBuildEndedFromInput(ctx.Input)
		payload, _ := json.Marshal(map[string]interface{}{"succeeded": succeeded, "failed": failed})
		return model.HookResultAgentToolEnded{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, "build"), Response: payload}
	case model.HookAgentToolTerminalStarting:
		command := extractTerminalCommandFromInput(ctx.Input, ctx.ToolArgs)
		blocked, reason := IsToolBlocked(todos.FirstNonEmpty(ctx.ToolName, "run_in_terminal"), todos.FirstNonEmpty(command, ctx.ToolArgs))
		base.Allowed = !blocked
		base.Message = reason
		return model.HookResultAgentToolTerminalStarting{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: todos.ExtractToolInputFromStdin(ctx.Input), Command: command}
	case model.HookAgentToolTerminalEnded:
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(ctx.Input)
		pidValue, _ := strconv.Atoi(pid)
		stdoutJSON, _ := json.Marshal(stdout)
		stderrJSON, _ := json.Marshal(stderr)
		return model.HookResultAgentToolTerminalEnded{HookResultAgentBase: base, Name: todos.FirstNonEmpty(ctx.ToolName, extractToolNameFromStdin(ctx.Input)), Input: todos.ExtractToolInputFromStdin(ctx.Input), Output: stdoutJSON, Command: command, PID: pidValue, Terminated: terminated, Stdout: stdoutJSON, Stderr: stderrJSON}
	case model.HookAgentThinkingStarting:
		base.Message = findNestedStringValue(decodeHookInputMap(ctx.Input), "text")
		return model.HookResultAgentThinkingStarting{HookResultAgentBase: base}
	case model.HookAgentThinkingEnded:
		base.Message = findNestedStringValue(decodeHookInputMap(ctx.Input), "text")
		return model.HookResultAgentThinkingEnded{HookResultAgentBase: base}
	case model.HookVersionCheckpointStarting:
		return runCheckpointStartingHook(ctx)
	case model.HookVersionCheckpointEnded:
		return model.HookResultVersionCheckpointEnded{HookResultBase: model.HookResultBase{Allowed: true}, Checkpoint: extractCheckpointSHAFromInput(ctx.Input), Description: extractCheckpointMessageFromInput(ctx.Input, ctx.RepoRoot)}
	case model.HookVersionCheckinStarting, model.HookVersionCheckinEnded, model.HookVersionCheckoutStarting, model.HookVersionCheckoutEnded:
		return model.HookResultVersionCheckpointEnded{HookResultBase: model.HookResultBase{Allowed: true}, Checkpoint: extractCheckpointSHAFromInput(ctx.Input)}
	default:
		return model.HookResultBase{Allowed: false, Message: "unknown event: " + string(ctx.Event)}
	}
}

// #endregion 📰️Todos
