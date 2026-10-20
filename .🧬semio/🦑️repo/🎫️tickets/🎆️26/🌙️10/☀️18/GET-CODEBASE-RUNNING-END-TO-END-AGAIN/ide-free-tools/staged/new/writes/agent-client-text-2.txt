// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🎆️agent-clients carries the agent-client providers: Copilot, Cursor, Windsurf, Claude Code, Droid, Codex, Antigravity and Kiro.

// #endregion 🧲️Header

package providers

import (
	json "encoding/json"

	model "github.com/usalu/semio/repo/model"
)

// #region 🎆️AgentClientProviders

// 🔌️CopilotAgentClientProvider holds the data fields for a copilot agent-client provider record.
type CopilotAgentClientProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *CopilotAgentClientProvider) Kind() string { return "copilot-chat" }

// 📡️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CopilotAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveCopilotEvent(nativeEvent, toolKind)
}

// 📋️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CopilotAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	return FormatCopilotHookOutput(hookEventName, result)
}

// 🔷️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CopilotAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return copilotEventFromHookEvent(event, parentInfo)
}

// 📝️CursorAgentClientProvider holds the data fields for a cursor agent-client provider record.
type CursorAgentClientProvider struct{}

// 🏷️Kind holds the data fields for a Kind record.
func (p *CursorAgentClientProvider) Kind() string { return "cursor-chat" }

// 🔸️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CursorAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveCursorEvent(nativeEvent, toolKind)
}

// 🔺️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CursorAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🔻️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CursorAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💻️WindsurfAgentClientProvider holds the data fields for a windsurf agent-client provider record.
type WindsurfAgentClientProvider struct{}

// 🟥️Kind holds the data fields for a Kind record.
func (p *WindsurfAgentClientProvider) Kind() string { return "windsurf-chat" }

// 🟨️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *WindsurfAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveWindsurfEvent(nativeEvent, toolKind)
}

// 🟩️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *WindsurfAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🟦️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *WindsurfAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💠️ClaudeCodeAgentClientProvider holds the data fields for a claude code agent-client provider record.
type ClaudeCodeAgentClientProvider struct{}

// 🔳️Kind holds the data fields for a Kind record.
func (p *ClaudeCodeAgentClientProvider) Kind() string { return "claude-code" }

// ▪️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *ClaudeCodeAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// ▫️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *ClaudeCodeAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// ◾NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *ClaudeCodeAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// ◼DroidAgentClientProvider holds the data fields for a droid agent-client provider record.
type DroidAgentClientProvider struct{}

// 🔵️Kind holds the data fields for a Kind record.
func (p *DroidAgentClientProvider) Kind() string { return "droid" }

// 🟠️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *DroidAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🟡️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *DroidAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🟢️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *DroidAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// ⚪️CodexAgentClientProvider holds the data fields for a codex agent-client provider record.
type CodexAgentClientProvider struct{}

// ⚫️Kind holds the data fields for a Kind record.
func (p *CodexAgentClientProvider) Kind() string { return "codex" }

// 🩶️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *CodexAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🩷️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *CodexAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 💜️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *CodexAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💛️AntigravityAgentClientProvider holds the data fields for an antigravity agent-client provider record.
type AntigravityAgentClientProvider struct{}

// 🧡️Kind holds the data fields for a Kind record.
func (p *AntigravityAgentClientProvider) Kind() string { return "antigravity-chat" }

// 🤍️ResolveNativeEvent holds the data fields for a ResolveNativeEvent record.
func (p *AntigravityAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return ResolveClaudeCompatibleEvent(nativeEvent, toolKind)
}

// 🖤️FormatHookOutput holds the data fields for a FormatHookOutput record.
func (p *AntigravityAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

// 🤎️NativeEventFromHookEvent holds the data fields for a NativeEventFromHookEvent record.
func (p *AntigravityAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// 💝️KiroAgentClientProvider holds the data fields for a kiro agent-client provider record.
type KiroAgentClientProvider struct{}

func (p *KiroAgentClientProvider) Kind() string { return "kiro-cli" }

func (p *KiroAgentClientProvider) ResolveNativeEvent(nativeEvent string, toolKind model.ToolKind) (model.HookEvent, string, error) {
	return resolveKiroEvent(nativeEvent, toolKind)
}

func (p *KiroAgentClientProvider) FormatHookOutput(hookEventName string, result model.HookResult) string {
	out, _ := json.Marshal(result)
	return string(out)
}

func (p *KiroAgentClientProvider) NativeEventFromHookEvent(event model.HookEvent, parentInfo string) string {
	return ""
}

// #endregion 🎆️AgentClientProviders
