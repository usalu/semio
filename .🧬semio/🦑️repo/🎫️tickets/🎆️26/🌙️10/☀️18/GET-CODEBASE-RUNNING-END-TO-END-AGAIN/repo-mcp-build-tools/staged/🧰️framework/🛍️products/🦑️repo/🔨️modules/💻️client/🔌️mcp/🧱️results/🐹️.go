// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The structured result of a build operation and its text form, both bounded so a response never exceeds the payload limit.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"strings"
	"time"
)

// #region 🧱️BuildResult

// 🪪️ResultAgent is the identity the call ran under.
type ResultAgent struct {
	ID      string `json:"id"`
	Session string `json:"session"`
	Lane    string `json:"lane"`
}

// 🧱️Truncation tells which streams lost their beginning.
type Truncation struct {
	Stdout bool `json:"stdout"`
	Stderr bool `json:"stderr"`
}

// 🧱️BuildResult is `$defs.Result` of the module schema.
type BuildResult struct {
	Version        int             `json:"version"`
	Tool           string          `json:"tool"`
	Operation      string          `json:"operation"`
	Subcommand     string          `json:"subcommand,omitempty"`
	BuildTool      string          `json:"buildTool,omitempty"`
	OK             bool            `json:"ok"`
	ExitCode       int             `json:"exitCode"`
	Cancelled      bool            `json:"cancelled"`
	DurationMs     int64           `json:"durationMs"`
	Command        []string        `json:"command"`
	Agent          ResultAgent     `json:"agent"`
	Data           json.RawMessage `json:"data,omitempty"`
	DataOmitted    int             `json:"dataOmitted,omitempty"`
	Stdout         string          `json:"stdout"`
	Stderr         string          `json:"stderr"`
	Truncated      Truncation      `json:"truncated"`
	ProgressEvents int             `json:"progressEvents"`
	Hint           string          `json:"hint,omitempty"`
}

// 🧱️NewBuildResult builds the result of one finished child; the limits bound every part.
func NewBuildResult(tool BuildTool, invocation Invocation, agent AgentIdentity, outcome ProcessOutcome) BuildResult {
	limits := tool.Limits
	command := append([]string{"repo"}, invocation.Argv...)
	result := BuildResult{Version: 1, Tool: tool.Name, Operation: tool.Operation, Subcommand: tool.Subcommand, BuildTool: invocation.BuildTool, ExitCode: outcome.ExitCode, Cancelled: outcome.Cancelled, DurationMs: outcome.Elapsed.Milliseconds(), Command: command, Agent: ResultAgent{ID: agent.ID, Session: agent.Session, Lane: agent.Lane}, ProgressEvents: outcome.ProgressEvents}
	result.OK = outcome.Started && outcome.ExitCode == 0 && !outcome.Cancelled
	stdout := outcome.Stdout
	trimmed := bytes.TrimSpace(stdout.Head)
	if !stdout.Overflow && len(trimmed) > 0 && (trimmed[0] == '{' || trimmed[0] == '[') && json.Valid(trimmed) {
		var compact bytes.Buffer
		if json.Compact(&compact, trimmed) == nil {
			if compact.Len() <= limits.DataBytes {
				result.Data = json.RawMessage(compact.Bytes())
			} else {
				result.DataOmitted = compact.Len()
				result.Hint = "The result is larger than the data limit; pass file to write the report to a file."
			}
		}
	}
	if result.Data == nil {
		result.Stdout = Printable(stdout.Tail)
		result.Truncated.Stdout = stdout.Truncated
	}
	result.Stderr = Printable(outcome.Stderr.Tail)
	result.Truncated.Stderr = outcome.Stderr.Truncated
	if !outcome.Started {
		result.Hint = "The repo executable could not be started: " + outcome.StartError
	} else if !result.OK && !outcome.Cancelled {
		if strings.Contains(result.Stderr, "SEMIO_LANGUAGE") || strings.Contains(strings.ToLower(result.Stderr), "no language") {
			result.Hint = "No language resolved: pass language en or de, or set SEMIO_LANGUAGE. Keine Sprache bestimmt: language en oder de angeben."
		} else if strings.Contains(result.Stderr, "unknown command") {
			result.Hint = "The installed repo executable does not know this operation; rebuild it with repo run repo-mcp:build."
		}
	}
	return result
}

// 📝️Text is the readable form of a result: one status line, the agent, then the JSON or the output tail, bounded by the text limit. It carries everything an agent needs when its client shows only text.
func (result BuildResult) Text(limit int) string {
	var out strings.Builder
	status := "OK"
	switch {
	case result.Cancelled:
		status = "CANCELLED"
	case !result.OK:
		status = "FAILED"
	}
	out.WriteString(fmt.Sprintf("%s repo %s (exit %d, %s)", status, strings.Join(result.Command[1:], " "), result.ExitCode, time.Duration(result.DurationMs)*time.Millisecond))
	out.WriteString("\nagent: " + result.Agent.ID)
	if result.Hint != "" {
		out.WriteString("\nhint: " + result.Hint)
	}
	body := ""
	headKeep := false
	switch {
	case result.Data != nil:
		var pretty bytes.Buffer
		if json.Indent(&pretty, result.Data, "", "  ") == nil {
			body = pretty.String()
		} else {
			body = string(result.Data)
		}
		headKeep = true
	case result.Stdout != "":
		body = result.Stdout
	}
	if body != "" {
		out.WriteString("\n\n")
		out.WriteString(bound(body, limit*3/4, headKeep))
	}
	if result.Stderr != "" {
		out.WriteString("\n\n--- stderr ---\n")
		out.WriteString(bound(result.Stderr, limit/4, false))
	}
	return bound(out.String(), limit, false)
}

// ✂️bound shortens text to the limit, keeping its beginning or its end, on a rune boundary.
func bound(text string, limit int, keepHead bool) string {
	if len(text) <= limit {
		return text
	}
	if keepHead {
		return strings.ToValidUTF8(text[:limit], "") + "\n… truncated, " + fmt.Sprint(len(text)-limit) + " more bytes"
	}
	return "… truncated, " + fmt.Sprint(len(text)-limit) + " earlier bytes\n" + strings.ToValidUTF8(text[len(text)-limit:], "")
}

// #endregion 🧱️BuildResult
