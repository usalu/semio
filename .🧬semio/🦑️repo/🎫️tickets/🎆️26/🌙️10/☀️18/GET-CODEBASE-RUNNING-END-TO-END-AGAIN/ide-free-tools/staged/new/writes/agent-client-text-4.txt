//! 🎆️ Agent-client providers: native event tables, hook output formatting and the eight agent-client providers.
//!
//! Holds the `🎆️AgentClientProviders` region of the crate root; the root re-exports its items.

use super::*;

// #region 🎆️AgentClientProviders
/// 🏷️ The pre-tool-use event for a tool kind.
pub fn resolve_pre_tool_use(kind: ToolKind) -> HookEvent {
    match kind {
        ToolKind::Plan => HookEvent::AgentToolPlanUpdatingStarting,
        ToolKind::CodeSearch => HookEvent::AgentToolSearchStarting,
        ToolKind::CodeEdit => HookEvent::AgentToolCodeEditStarting,
        ToolKind::Test => HookEvent::AgentToolTestStarting,
        ToolKind::Build => HookEvent::AgentToolBuildStarting,
        ToolKind::Terminal => HookEvent::AgentToolTerminalStarting,
        ToolKind::Generic => HookEvent::AgentToolStarting,
    }
}

/// 🔸️ The post-tool-use event for a tool kind.
pub fn resolve_post_tool_use(kind: ToolKind) -> HookEvent {
    match kind {
        ToolKind::Plan => HookEvent::AgentToolPlanUpdatingEnded,
        ToolKind::CodeSearch => HookEvent::AgentToolSearchEnded,
        ToolKind::CodeEdit => HookEvent::AgentToolCodeEditEnded,
        ToolKind::Test => HookEvent::AgentToolTestEnded,
        ToolKind::Build => HookEvent::AgentToolBuildEnded,
        ToolKind::Terminal => HookEvent::AgentToolTerminalEnded,
        ToolKind::Generic => HookEvent::AgentToolEnded,
    }
}

/// 🖥️ A shell invocation whose kind was not classified is terminal work.
pub fn resolve_shell_pre_tool_use(kind: ToolKind) -> HookEvent {
    match kind {
        ToolKind::Generic => HookEvent::AgentToolTerminalStarting,
        _ => resolve_pre_tool_use(kind),
    }
}

/// 🖥️ A finished shell invocation whose kind was not classified is terminal work.
pub fn resolve_shell_post_tool_use(kind: ToolKind) -> HookEvent {
    match kind {
        ToolKind::Generic => HookEvent::AgentToolTerminalEnded,
        _ => resolve_post_tool_use(kind),
    }
}

fn unknown_event(native_event: &str, client: &str) -> ProviderError {
    ProviderError::new(format!("unknown native event {native_event:?} for {client}"))
}

/// 📡️ Maps a VS Code / Copilot Chat native event to a neutral hook event.
pub fn resolve_copilot_event(native_event: &str, kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
    let resolved = match native_event {
        "SessionStart" => (HookEvent::AgentStarted, ""),
        "Stop" => (HookEvent::AgentEnded, ""),
        "SubagentStart" => (HookEvent::AgentStarted, "subagent"),
        "SubagentStop" => (HookEvent::AgentEnded, "subagent"),
        "UserPromptSubmit" => (HookEvent::AgentPromptSubmitting, ""),
        "PreCompact" => (HookEvent::AgentCompacting, ""),
        "PreToolUse" => (resolve_pre_tool_use(kind), ""),
        "PostToolUse" | "PostToolUseFailure" => (resolve_post_tool_use(kind), ""),
        "beforeMCPExecution" => (HookEvent::AgentToolStarting, ""),
        "afterMCPExecution" => (HookEvent::AgentToolEnded, ""),
        "beforeReadFile" | "beforeTabFileRead" => (HookEvent::AgentToolSearchStarting, ""),
        "afterFileEdit" | "afterTabFileEdit" => (HookEvent::AgentToolCodeEditEnded, ""),
        "beforeShellExecution" => (resolve_shell_pre_tool_use(kind), ""),
        "afterShellExecution" => (resolve_shell_post_tool_use(kind), ""),
        "afterAgentResponse" => (HookEvent::AgentEnded, ""),
        "afterAgentThought" => (HookEvent::AgentThinkingEnded, ""),
        _ => return Err(unknown_event(native_event, "copilot-chat")),
    };
    Ok((resolved.0, resolved.1.to_string()))
}

/// 🗺️ Maps a Cursor native event to a neutral hook event.
pub fn resolve_cursor_event(native_event: &str, kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
    let resolved = match native_event {
        "sessionStart" => (HookEvent::AgentStarted, ""),
        "sessionEnd" => (HookEvent::AgentEnded, ""),
        "subagentStart" => (HookEvent::AgentStarted, "subagent"),
        "subagentStop" => (HookEvent::AgentEnded, "subagent"),
        "stop" => (HookEvent::AgentEnded, ""),
        "userPromptSubmit" | "beforeSubmitPrompt" => (HookEvent::AgentPromptSubmitting, ""),
        "preCompact" => (HookEvent::AgentCompacting, ""),
        "preToolUse" => (resolve_pre_tool_use(kind), ""),
        "postToolUse" | "postToolUseFailure" => (resolve_post_tool_use(kind), ""),
        "beforeMCPExecution" => (HookEvent::AgentToolStarting, ""),
        "afterMCPExecution" => (HookEvent::AgentToolEnded, ""),
        "beforeReadFile" | "beforeTabFileRead" => (HookEvent::AgentToolSearchStarting, ""),
        "afterFileEdit" | "afterTabFileEdit" => (HookEvent::AgentToolCodeEditEnded, ""),
        "beforeShellExecution" => (resolve_shell_pre_tool_use(kind), ""),
        "afterShellExecution" => (resolve_shell_post_tool_use(kind), ""),
        "afterAgentResponse" => (HookEvent::AgentEnded, ""),
        "afterAgentThought" => (HookEvent::AgentThinkingEnded, ""),
        _ => return Err(unknown_event(native_event, "cursor-chat")),
    };
    Ok((resolved.0, resolved.1.to_string()))
}

/// 🔷️ Maps a Windsurf native event to a neutral hook event.
pub fn resolve_windsurf_event(native_event: &str, kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
    let resolved = match native_event {
        "pre_user_prompt" => (HookEvent::AgentPromptSubmitting, ""),
        "post_cascade_response" => (HookEvent::AgentEnded, ""),
        "post_setup_worktree" => (HookEvent::AgentStarted, ""),
        "subagentStart" => (HookEvent::AgentStarted, "subagent"),
        "subagentStop" => (HookEvent::AgentEnded, "subagent"),
        "stop" => (HookEvent::AgentEnded, ""),
        "preCompact" => (HookEvent::AgentCompacting, ""),
        "preToolUse" | "pre_mcp_tool_use" => (resolve_pre_tool_use(kind), ""),
        "postToolUse" | "postToolUseFailure" | "post_mcp_tool_use" => (resolve_post_tool_use(kind), ""),
        "pre_read_code" => (HookEvent::AgentToolSearchStarting, ""),
        "post_read_code" => (HookEvent::AgentToolSearchEnded, ""),
        "pre_write_code" => (HookEvent::AgentToolCodeEditStarting, ""),
        "post_write_code" => (HookEvent::AgentToolCodeEditEnded, ""),
        "pre_run_command" => (resolve_shell_pre_tool_use(kind), ""),
        "post_run_command" => (resolve_shell_post_tool_use(kind), ""),
        _ => return Err(unknown_event(native_event, "windsurf-chat")),
    };
    Ok((resolved.0, resolved.1.to_string()))
}

/// 🔶️ Maps a Claude-compatible native event to a neutral hook event.
pub fn resolve_claude_compatible_event(native_event: &str, kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
    let resolved = match native_event {
        "start" | "SessionStart" => (HookEvent::AgentStarted, ""),
        "stop" | "Stop" | "SessionEnd" => (HookEvent::AgentEnded, ""),
        "subagentStart" | "SubagentStart" => (HookEvent::AgentStarted, "subagent"),
        "subagentStop" | "SubagentStop" => (HookEvent::AgentEnded, "subagent"),
        "userPromptSubmit" | "UserPromptSubmit" => (HookEvent::AgentPromptSubmitting, ""),
        "preCompact" | "PreCompact" => (HookEvent::AgentCompacting, ""),
        "preToolUse" | "PreToolUse" | "PermissionRequest" | "TeammateIdle" | "Notification" => (resolve_pre_tool_use(kind), ""),
        "postToolUse" | "postToolUseFailure" | "PostToolUse" | "PostToolUseFailure" => (resolve_post_tool_use(kind), ""),
        "TaskCompleted" => (HookEvent::AgentToolPlanUpdatingEnded, ""),
        _ => return Err(unknown_event(native_event, "claude-compatible")),
    };
    Ok((resolved.0, resolved.1.to_string()))
}

/// 🔹️ Maps a Kiro native event to a neutral hook event.
pub fn resolve_kiro_event(native_event: &str, kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
    let resolved = match native_event {
        "agentSpawn" => (HookEvent::AgentStarted, ""),
        "userPromptSubmit" => (HookEvent::AgentPromptSubmitting, ""),
        "preToolUse" => (resolve_pre_tool_use(kind), ""),
        "postToolUse" => (resolve_post_tool_use(kind), ""),
        "stop" => (HookEvent::AgentEnded, ""),
        _ => return Err(unknown_event(native_event, "kiro-cli")),
    };
    Ok((resolved.0, resolved.1.to_string()))
}

/// 💻️ Formats a hook result the way VS Code and Copilot Chat read it: the marshalled result plus a
/// `hookSpecificOutput` member carrying the permission decision or the additional context.
pub fn format_copilot_hook_output(hook_event_name: &str, result: &HookResult) -> String {
    let mut payload = result.fields();
    let mut hook_specific = Map::new();
    if !hook_event_name.is_empty() {
        hook_specific.insert("hookEventName".to_string(), Value::String(hook_event_name.to_string()));
    }
    if hook_event_name == "PreToolUse" {
        if result.allowed {
            hook_specific.insert("permissionDecision".to_string(), Value::String("allow".to_string()));
        } else {
            hook_specific.insert("permissionDecision".to_string(), Value::String("deny".to_string()));
            let reason = result.message.trim();
            if !reason.is_empty() {
                hook_specific.insert("permissionDecisionReason".to_string(), Value::String(reason.to_string()));
            }
        }
    } else {
        let message = result.message.trim();
        if !message.is_empty() {
            hook_specific.insert("additionalContext".to_string(), Value::String(message.to_string()));
        }
    }
    if !hook_specific.is_empty() {
        payload.push(("hookSpecificOutput".to_string(), Value::Object(hook_specific)));
    }
    Value::Object(payload.into_iter().collect()).to_string()
}

/// 🔤️ Converts a neutral hook event to the VS Code event string.
pub fn vscode_event_from_hook_event(event: HookEvent, parent_info: &str) -> String {
    match event {
        HookEvent::AgentStarted => {
            if parent_info == "subagent" {
                "SubagentStart"
            } else {
                "SessionStart"
            }
        }
        HookEvent::AgentEnded => {
            if parent_info == "subagent" {
                "SubagentStop"
            } else {
                "Stop"
            }
        }
        HookEvent::AgentPromptSubmitting => "UserPromptSubmit",
        HookEvent::AgentCompacting => "PreCompact",
        HookEvent::AgentToolStarting | HookEvent::AgentToolPlanUpdatingStarting | HookEvent::AgentToolSearchStarting | HookEvent::AgentToolCodeEditStarting | HookEvent::AgentToolTestStarting | HookEvent::AgentToolBuildStarting | HookEvent::AgentToolTerminalStarting => "PreToolUse",
        HookEvent::AgentToolEnded | HookEvent::AgentToolPlanUpdatingEnded | HookEvent::AgentToolSearchEnded | HookEvent::AgentToolCodeEditEnded | HookEvent::AgentToolTestEnded | HookEvent::AgentToolBuildEnded | HookEvent::AgentToolTerminalEnded => "PostToolUse",
        _ => "",
    }
    .to_string()
}

/// 🧾️ The plain formatting every non-VS-Code agent client uses: the marshalled result, nothing added.
pub fn format_plain_hook_output(result: &HookResult) -> String {
    result.to_json()
}

macro_rules! plain_agent_client_provider {
    ($name:ident, $kind:literal, $resolve:path) => {
        /// 📝️ Agent-client provider whose hook output is the marshalled result verbatim.
        #[derive(Debug, Default)]
        pub struct $name;

        impl AgentClientProvider for $name {
            fn kind(&self) -> &'static str {
                $kind
            }
            fn resolve_native_event(&self, native_event: &str, tool_kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
                $resolve(native_event, tool_kind)
            }
            fn format_hook_output(&self, _hook_event_name: &str, result: &HookResult) -> String {
                format_plain_hook_output(result)
            }
            fn native_event_from_hook_event(&self, _event: HookEvent, _parent_info: &str) -> String {
                String::new()
            }
        }
    };
}

/// 🔌️ VS Code / Copilot Chat — the only provider that wraps its output for the IDE.
#[derive(Debug, Default)]
pub struct CopilotAgentClientProvider;

impl AgentClientProvider for CopilotAgentClientProvider {
    fn kind(&self) -> &'static str {
        "copilot-chat"
    }
    fn resolve_native_event(&self, native_event: &str, tool_kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
        resolve_copilot_event(native_event, tool_kind)
    }
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String {
        format_copilot_hook_output(hook_event_name, result)
    }
    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String {
        vscode_event_from_hook_event(event, parent_info)
    }
}

plain_agent_client_provider!(CursorAgentClientProvider, "cursor-chat", resolve_cursor_event);
plain_agent_client_provider!(WindsurfAgentClientProvider, "windsurf-chat", resolve_windsurf_event);
plain_agent_client_provider!(ClaudeCodeAgentClientProvider, "claude-code", resolve_claude_compatible_event);
plain_agent_client_provider!(DroidAgentClientProvider, "droid", resolve_claude_compatible_event);
plain_agent_client_provider!(CodexAgentClientProvider, "codex", resolve_claude_compatible_event);
plain_agent_client_provider!(AntigravityAgentClientProvider, "antigravity-chat", resolve_claude_compatible_event);
plain_agent_client_provider!(KiroAgentClientProvider, "kiro-cli", resolve_kiro_event);

dyn_enum_close! {
    pub enum AgentClientProviders: AgentClientProvider {
        Copilot(CopilotAgentClientProvider),
        Cursor(CursorAgentClientProvider),
        Windsurf(WindsurfAgentClientProvider),
        ClaudeCode(ClaudeCodeAgentClientProvider),
        Droid(DroidAgentClientProvider),
        Codex(CodexAgentClientProvider),
        Antigravity(AntigravityAgentClientProvider),
        Kiro(KiroAgentClientProvider),
    }
}
// #endregion 🎆️AgentClientProviders
