// 🔶️ Tests of running hooks and rendering their output.

package hooks

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestRunHookAgentEvents(t *testing.T) {
	cases := []struct {
		name    string
		event   model.HookEvent
		allowed bool
	}{
		{"agent starting", model.HookAgentStarted, true},
		{"agent ended", model.HookAgentEnded, true},
		{"agent prompt submitting", model.HookAgentPromptSubmitting, true},
		{"agent compacting", model.HookAgentCompacting, true},
		{"agent tool ended", model.HookAgentToolEnded, true},
		{"agent tool code searching", model.HookAgentToolSearchStarting, true},
		{"agent tool searched", model.HookAgentToolSearchEnded, true},
		{"agent tool code editing", model.HookAgentToolCodeEditStarting, true},
		{"agent tool code edited", model.HookAgentToolCodeEditEnded, true},
		{"agent tool plan updating starting", model.HookAgentToolPlanUpdatingStarting, true},
		{"agent tool plan updating ended", model.HookAgentToolPlanUpdatingEnded, true},
		{"agent tool terminal starting", model.HookAgentToolTerminalStarting, true},
		{"agent tool terminal ended", model.HookAgentToolTerminalEnded, true},
		{"agent thinking starting", model.HookAgentThinkingStarting, true},
		{"agent thinking ended", model.HookAgentThinkingEnded, true},
		{"version checkpoint ended", model.HookVersionCheckpointEnded, true},
		{"version checkin starting", model.HookVersionCheckinStarting, true},
		{"version checkin ended", model.HookVersionCheckinEnded, true},
		{"version checkout starting", model.HookVersionCheckoutStarting, true},
		{"version checkout ended", model.HookVersionCheckoutEnded, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := model.HookContext{
				Event:    tc.event,
				Client:   "copilot-chat",
				Second:   time.Now().UTC().Format(time.RFC3339),
				RepoRoot: t.TempDir(),
			}
			result := RunHook(hctx)
			if result.IsAllowed() != tc.allowed {
				t.Errorf("expected allowed=%v, got allowed=%v: %s", tc.allowed, result.IsAllowed(), result.GetMessage())
			}
		})
	}
	t.Run("version checkpoint starting", func(t *testing.T) {
		hctx := model.HookContext{
			Event:    model.HookVersionCheckpointStarting,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)

		_, ok := result.(model.HookResultVersionCheckpointStarting)
		if !ok {
			t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
		}
	})
}

func TestRunHookUnknownEvent(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookEvent("unknown.event"),
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected unknown event to be denied")
	}
}

func TestRunHookThinkingEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow run hook thinking events test in short mode")
	}
	t.Run("thinking ended extracts text from top-level", func(t *testing.T) {
		hctx := model.HookContext{
			Event:  model.HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think","text":"Planning repo modifications"}`),
		}
		result := RunHook(hctx)
		res, ok := result.(model.HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Planning repo modifications" {
			t.Errorf("expected thinking text in message, got %q", res.GetMessage())
		}
		if !res.IsAllowed() {
			t.Error("expected thinking ended to be allowed")
		}
	})
	t.Run("thinking ended extracts text from native.event", func(t *testing.T) {
		input := `{"native":{"event":{"text":"Thinking deeply","hook_event_name":"afterAgentThought"}},"session_id":"sess-think2"}`
		hctx := model.HookContext{
			Event:  model.HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(input),
		}
		result := RunHook(hctx)
		res, ok := result.(model.HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Thinking deeply" {
			t.Errorf("expected thinking text from native.event, got %q", res.GetMessage())
		}
	})
	t.Run("thinking starting is allowed", func(t *testing.T) {
		hctx := model.HookContext{
			Event:  model.HookAgentThinkingStarting,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think3","text":"About to think"}`),
		}
		result := RunHook(hctx)
		if _, ok := result.(model.HookResultAgentThinkingStarting); !ok {
			t.Fatalf("expected HookResultAgentThinkingStarting, got %T", result)
		}
		if !result.IsAllowed() {
			t.Error("expected thinking starting to be allowed")
		}
	})
}

func TestHookCommandJSONOutput(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	out, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON marshaling, got: %v", err)
	}
	var parsed model.HookResultAgentStarted
	if err := json.Unmarshal(out, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	if !parsed.Allowed {
		t.Error("expected allowed=true")
	}
	if parsed.Second == "" {
		t.Error("expected non-empty second")
	}
}

func TestHookCommandCopilotChatVSCodeOutput(t *testing.T) {
	t.Run("PreToolUse allow produces VS Code JSON", func(t *testing.T) {
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "copilot-chat",
			Second:   time.Now().UTC().Format(time.RFC3339),
			ToolName: "read_file",
			ToolArgs: "/tmp/file.ts",
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)
		if !result.IsAllowed() {
			t.Fatal("expected allowed")
		}
		out := providers.FormatCopilotHookOutput("PreToolUse", result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "allow" {
			t.Errorf("expected allow, got %v", hso["permissionDecision"])
		}
	})
	t.Run("PreToolUse blocked produces VS Code deny JSON", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git checkout main"}}`)
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "copilot-chat",
			Second:   time.Now().UTC().Format(time.RFC3339),
			ToolName: "run_in_terminal",
			Input:    payload,
			RepoRoot: t.TempDir(),
		}
		result := RunHook(hctx)
		if result.IsAllowed() {
			t.Fatal("expected blocked")
		}
		hookEventName := extractHookEventNameFromStdin(payload)
		out := providers.FormatCopilotHookOutput(hookEventName, result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "deny" {
			t.Errorf("expected deny, got %v", hso["permissionDecision"])
		}
		reason, _ := hso["permissionDecisionReason"].(string)
		if !strings.Contains(reason, "blocked") {
			t.Errorf("expected blocked reason, got: %s", reason)
		}
	})
	t.Run("tool_name extracted from stdin", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git stash"}}`)
		toolName := extractToolNameFromStdin(payload)
		if toolName != "run_in_terminal" {
			t.Errorf("expected run_in_terminal, got %s", toolName)
		}
		cmd := extractCommandFromStdin(payload)
		if cmd != "git stash" {
			t.Errorf("expected git stash, got %s", cmd)
		}
	})
}

func TestRunHookAgentToolStartingDerivedRepoEvents(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	sessionID := "test-repo-events-session"
	payload := json.RawMessage(`{"session_id":"test-repo-events-session","tool_name":"mcp__repo__ticket_open","tool_input":{"title":"My Ticket","prompt":"My Prompt","client":"claude-code","llm":"sonnet-4-5","goal":"MY-GOAL"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-03-05T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)

	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), sessionID)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 || entries[0].Name() != "session.json" {
		names := make([]string, len(entries))
		for i, e := range entries {
			names[i] = e.Name()
		}
		t.Fatalf("expected only session.json, got %d: %v", len(entries), names)
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta model.SessionMeta
	json.Unmarshal(data, &meta)
	if len(meta.Events) != 2 {
		t.Fatalf("expected 2 session events (agent + derived repo operation), got %d", len(meta.Events))
	}
	found := false
	for _, entry := range meta.Events {
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] == "agent.ticket.open.starting" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected derived agent.ticket.open.starting event in session.json events")
	}
}

func TestRunHookSearchStartingIncludesDefinitions(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc Alpha() {\n\tprintln(\"one\")\n}\n\nfunc Beta() {\n\tprintln(\"needle\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	payload := json.RawMessage(`{"trajectory_id":"trace-1","tool_info":{"command_line":"grep -n \"needle\" sample.go","cwd":"` + filepath.ToSlash(tmpDir) + `"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected definitions in event, got %v", res.Definitions)
	}
	if res.Definitions[0].Loc <= 0 {
		t.Fatalf("expected loc > 0, got %d", res.Definitions[0].Loc)
	}
}

func TestBuildDefinitionReadsFullFileReturnsFileID(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "full.go")
	content := "package main\n\nfunc Alpha() {}\n\nfunc Beta() {}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	sandbox.BindRoot(t, tmpDir)

	lineSet := map[int]struct{}{1: {}, 2: {}, 3: {}, 4: {}, 5: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) != 1 {
		t.Fatalf("expected exactly one file-id entry for full file read, got %d: %v", len(definitions), definitions)
	}
	if definitions[0].Loc != 0 {
		t.Errorf("expected loc=0 for file-id entry, got %d", definitions[0].Loc)
	}
	if definitions[0].ID == "" {
		t.Errorf("expected non-empty file ID")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	if strings.Contains(definitions[0].ID, absPrefix) {
		t.Errorf("file ID %q contains absolute tmpDir path %q", definitions[0].ID, absPrefix)
	}
}

func TestBuildDefinitionReadsUsesRepoRelativePaths(t *testing.T) {
	tmpDir := t.TempDir()
	subDir := filepath.Join(tmpDir, "pkg")
	if err := os.MkdirAll(subDir, 0o755); err != nil {
		t.Fatalf("failed to create subdir: %v", err)
	}
	filePath := filepath.Join(subDir, "util.go")
	content := "package pkg\n\nfunc Helper() {\n\tprintln(\"found\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	sandbox.BindRoot(t, tmpDir)
	lineSet := map[int]struct{}{3: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition, got none")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute tmpDir path %q", def.ID, absPrefix)
		}
	}
}

func TestRunHookPreReadCodeDefinitionIDsAreRepoRelative(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "cmd.go")
	content := "package main\n\nfunc Run() {\n\tprintln(\"run\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	payload := json.RawMessage(`{"tool_info":{"file_path":"` + filepath.ToSlash(filePath) + `"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected at least one definition")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range res.Definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute path %q", def.ID, absPrefix)
		}
	}
}

func TestShouldExecuteSearchCommand(t *testing.T) {
	tests := []struct {
		name    string
		command string
		want    bool
	}{
		{name: "grep command", command: "grep -n needle file.go", want: true},
		{name: "cat command", command: "cat file.go", want: true},
		{name: "blocked redirection", command: "grep -n needle file.go > out.txt", want: false},
		{name: "edit command", command: "sed -i 's/a/b/' file.go", want: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := shouldExecuteSearchCommand(tt.command)
			if got != tt.want {
				t.Fatalf("shouldExecuteSearchCommand(%q) = %v, want %v", tt.command, got, tt.want)
			}
		})
	}
}
