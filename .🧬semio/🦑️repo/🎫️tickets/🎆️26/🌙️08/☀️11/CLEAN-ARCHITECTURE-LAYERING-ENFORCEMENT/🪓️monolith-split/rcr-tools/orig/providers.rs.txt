//! 🧩️ Composable provider integrations for the semio repository domain: issue/milestone management
//! (GitHub through the `gh` CLI), version control (git), sandbox (devcontainer), the eight editor
//! providers and the registry that hands them out.
//!
//! Every external process goes through [`ProcessRunner`], so each provider is exercised end to end
//! without `gh` or `git` on the machine — see `🧪️tests/🐙️github-management-transcripts`.
//!
//! `#![allow(async_fn_in_trait)]` is the per-crate responsibility every `#[dyn_enum]` declaring crate
//! carries; see `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/🦀️.rs`.
#![allow(async_fn_in_trait)]

use dispatch_macros::{dyn_enum, dyn_enum_close};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cell::RefCell;

// #region 🔖️ProviderError
/// ⚠️ Every provider failure, carrying the message the Go implementation formats verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderError {
    pub message: String,
}

impl ProviderError {
    /// 🆕️ Wraps a message.
    pub fn new(message: impl Into<String>) -> ProviderError {
        ProviderError { message: message.into() }
    }
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ProviderError {}

/// 🎯️ The crate's result alias.
pub type ProviderResult<T> = Result<T, ProviderError>;
// #endregion 🔖️ProviderError

// #region 📐️ModelPending
/// 🚚️ `ToolKind`, `HookEvent` and `HookResult` are model-level shapes. `semio-framework-repo-model`
/// does not export them yet (checked at ticket time: its `🦀️.rs` carries no `Kind`, `HookEvent` or
/// `HookResult`), so they live here and MUST move down to `🔨️modules/📐️model` the moment that crate
/// grows them; this crate then re-exports them for `🪝️hooks`.
/// 🏷️ A categorized tool execution kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolKind {
    Generic,
    Plan,
    CodeSearch,
    CodeEdit,
    Test,
    Build,
    Terminal,
}

impl ToolKind {
    /// 🔤️ The wire slug.
    pub fn as_str(self) -> &'static str {
        match self {
            ToolKind::Generic => "generic",
            ToolKind::Plan => "plan",
            ToolKind::CodeSearch => "code-search",
            ToolKind::CodeEdit => "code-edit",
            ToolKind::Test => "test",
            ToolKind::Build => "build",
            ToolKind::Terminal => "terminal",
        }
    }

    /// 🧭️ Parses a wire slug, defaulting to `Generic` exactly like the Go zero value.
    pub fn parse(raw: &str) -> ToolKind {
        match raw {
            "plan" => ToolKind::Plan,
            "code-search" => ToolKind::CodeSearch,
            "code-edit" => ToolKind::CodeEdit,
            "test" => ToolKind::Test,
            "build" => ToolKind::Build,
            "terminal" => ToolKind::Terminal,
            _ => ToolKind::Generic,
        }
    }
}

/// 📡️ A lifecycle event kind for hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookEvent {
    VersionCheckpointStarting,
    VersionCheckpointEnded,
    VersionCheckinStarting,
    VersionCheckinEnded,
    VersionCheckoutStarting,
    VersionCheckoutEnded,
    AgentStarted,
    AgentEnded,
    AgentPromptSubmitting,
    AgentCompacting,
    AgentToolStarting,
    AgentToolEnded,
    AgentToolPlanUpdatingStarting,
    AgentToolPlanUpdatingEnded,
    AgentToolSearchStarting,
    AgentToolSearchEnded,
    AgentToolCodeEditStarting,
    AgentToolCodeEditEnded,
    AgentToolTestStarting,
    AgentToolTestEnded,
    AgentToolBuildStarting,
    AgentToolBuildEnded,
    AgentToolTerminalStarting,
    AgentToolTerminalEnded,
    AgentThinkingStarting,
    AgentThinkingEnded,
}

/// 📋️ Every valid hook event, in the Go declaration order.
pub const ALL_HOOK_EVENTS: [HookEvent; 26] = [
    HookEvent::VersionCheckpointStarting,
    HookEvent::VersionCheckpointEnded,
    HookEvent::VersionCheckinStarting,
    HookEvent::VersionCheckinEnded,
    HookEvent::VersionCheckoutStarting,
    HookEvent::VersionCheckoutEnded,
    HookEvent::AgentStarted,
    HookEvent::AgentEnded,
    HookEvent::AgentPromptSubmitting,
    HookEvent::AgentCompacting,
    HookEvent::AgentToolStarting,
    HookEvent::AgentToolEnded,
    HookEvent::AgentToolPlanUpdatingStarting,
    HookEvent::AgentToolPlanUpdatingEnded,
    HookEvent::AgentToolSearchStarting,
    HookEvent::AgentToolSearchEnded,
    HookEvent::AgentToolCodeEditStarting,
    HookEvent::AgentToolCodeEditEnded,
    HookEvent::AgentToolTestStarting,
    HookEvent::AgentToolTestEnded,
    HookEvent::AgentToolBuildStarting,
    HookEvent::AgentToolBuildEnded,
    HookEvent::AgentToolTerminalStarting,
    HookEvent::AgentToolTerminalEnded,
    HookEvent::AgentThinkingStarting,
    HookEvent::AgentThinkingEnded,
];

impl HookEvent {
    /// 🔤️ The dotted event slug carried on the wire.
    pub fn as_str(self) -> &'static str {
        match self {
            HookEvent::VersionCheckpointStarting => "version.checkpoint.starting",
            HookEvent::VersionCheckpointEnded => "version.checkpoint.ended",
            HookEvent::VersionCheckinStarting => "version.checkin.starting",
            HookEvent::VersionCheckinEnded => "version.checkin.ended",
            HookEvent::VersionCheckoutStarting => "version.checkout.starting",
            HookEvent::VersionCheckoutEnded => "version.checkout.ended",
            HookEvent::AgentStarted => "agent.started",
            HookEvent::AgentEnded => "agent.ended",
            HookEvent::AgentPromptSubmitting => "agent.prompt.submitting",
            HookEvent::AgentCompacting => "agent.compacting",
            HookEvent::AgentToolStarting => "agent.tool.starting",
            HookEvent::AgentToolEnded => "agent.tool.ended",
            HookEvent::AgentToolPlanUpdatingStarting => "agent.tool.plan.updating.starting",
            HookEvent::AgentToolPlanUpdatingEnded => "agent.tool.plan.updating.ended",
            HookEvent::AgentToolSearchStarting => "agent.file.read.starting",
            HookEvent::AgentToolSearchEnded => "agent.file.read.ended",
            HookEvent::AgentToolCodeEditStarting => "agent.tool.code.edit.starting",
            HookEvent::AgentToolCodeEditEnded => "agent.tool.code.edit.ended",
            HookEvent::AgentToolTestStarting => "agent.tool.test.starting",
            HookEvent::AgentToolTestEnded => "agent.tool.test.ended",
            HookEvent::AgentToolBuildStarting => "agent.tool.build.starting",
            HookEvent::AgentToolBuildEnded => "agent.tool.build.ended",
            HookEvent::AgentToolTerminalStarting => "agent.tool.terminal.starting",
            HookEvent::AgentToolTerminalEnded => "agent.tool.terminal.ended",
            HookEvent::AgentThinkingStarting => "agent.thinking.starting",
            HookEvent::AgentThinkingEnded => "agent.thinking.ended",
        }
    }

    /// 🧭️ Resolves a dotted event slug.
    pub fn parse(raw: &str) -> Option<HookEvent> {
        ALL_HOOK_EVENTS.into_iter().find(|event| event.as_str() == raw)
    }
}

/// 🔶️ The outcome of a hook invocation: the two fields every Go `HookResult` exposes through its
/// interface, plus the event-specific payload `json.Marshal` would have emitted.
#[derive(Debug, Clone, PartialEq)]
pub struct HookResult {
    pub allowed: bool,
    pub message: String,
    /// 🧾️ The event-specific fields in the order the Go struct declares them. A `Map` would sort
    /// them alphabetically and `json.Marshal` does not, so the order is carried explicitly.
    pub extra: Vec<(String, Value)>,
    /// 🙈️ Whether the `message` member belongs to the event payload rather than to the base.
    ///
    /// Go's `HookResultAgentBase` declares `MessageID string \`json:"message,omitempty"\``, which
    /// shadows `HookResultBase.Message` on every agent result: the block reason stays readable
    /// through `GetMessage()` and never reaches the marshalled object. This flag reproduces that.
    pub message_shadowed: bool,
}

impl HookResult {
    /// 🆕️ A result carrying only the base fields.
    pub fn new(allowed: bool, message: impl Into<String>) -> HookResult {
        HookResult { allowed, message: message.into(), extra: Vec::new(), message_shadowed: false }
    }

    /// ➕️ Adds one event-specific field, replacing an earlier value in place.
    pub fn with(mut self, key: impl Into<String>, value: Value) -> HookResult {
        let key = key.into();
        match self.extra.iter_mut().find(|(name, _)| name == &key) {
            Some(slot) => slot.1 = value,
            None => self.extra.push((key, value)),
        }
        self
    }

    /// 🧾️ The fields `json.Marshal(result)` writes, in Go's declaration order: `allowed`, `message`
    /// when non-empty, then every event-specific field.
    pub fn fields(&self) -> Vec<(String, Value)> {
        let mut fields = vec![("allowed".to_string(), Value::Bool(self.allowed))];
        if !self.message.is_empty() && !self.message_shadowed {
            fields.push(("message".to_string(), Value::String(self.message.clone())));
        }
        fields.extend(self.extra.iter().cloned());
        fields
    }

    /// 🧾️ The same fields as an unordered map, for a caller that only looks values up.
    pub fn payload(&self) -> Map<String, Value> {
        self.fields().into_iter().collect()
    }

    /// 🧾️ The JSON text `json.Marshal(result)` produces, field order included.
    pub fn to_json(&self) -> String {
        write_ordered_object(&self.fields())
    }
}

/// 🧾️ Writes an object whose members keep the given order, which `serde_json::Map` cannot.
pub fn write_ordered_object(fields: &[(String, Value)]) -> String {
    let members: Vec<String> = fields.iter().map(|(key, value)| format!("{}:{}", Value::String(key.clone()), value)).collect();
    format!("{{{}}}", members.join(","))
}

impl Serialize for HookResult {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let fields = self.fields();
        let mut map = serializer.serialize_map(Some(fields.len()))?;
        for (key, value) in &fields {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for HookResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let object = Map::<String, Value>::deserialize(deserializer)?;
        let allowed = object.get("allowed").and_then(Value::as_bool).unwrap_or_default();
        let message = object.get("message").and_then(Value::as_str).unwrap_or_default().to_string();
        let extra = object.into_iter().filter(|(key, _)| key != "allowed" && key != "message").collect();
        Ok(HookResult { allowed, message, extra, message_shadowed: false })
    }
}
// #endregion 📐️ModelPending

// #region 🪪️McpClientKind
/// 🪪️ Which IDE-native MCP surface is talking. Fix 7 of `📓️go-region-dependency-graph.md`: the enum is
/// editor identity, not MCP protocol, so it sits beside the editor providers that enumerate the same
/// IDE set and `🎫️tickets`/`🪝️hooks` reach it downward instead of importing `🔌️mcp`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpClientKind {
    Generic,
    Cursor,
    Kiro,
    Copilot,
    Claude,
    Codex,
}

impl McpClientKind {
    /// 🔤️ The wire slug.
    pub fn as_str(self) -> &'static str {
        match self {
            McpClientKind::Generic => "generic",
            McpClientKind::Cursor => "cursor",
            McpClientKind::Kiro => "kiro",
            McpClientKind::Copilot => "copilot",
            McpClientKind::Claude => "claude",
            McpClientKind::Codex => "codex",
        }
    }
}

/// 🧭️ Maps a CLI/MCP profile slug to an MCP server kind.
pub fn parse_mcp_client_kind(raw: &str) -> ProviderResult<McpClientKind> {
    match raw.trim().to_lowercase().as_str() {
        "" | "generic" | "client" => Ok(McpClientKind::Generic),
        "cursor" => Ok(McpClientKind::Cursor),
        "kiro" => Ok(McpClientKind::Kiro),
        "copilot" => Ok(McpClientKind::Copilot),
        "claude" => Ok(McpClientKind::Claude),
        "codex" => Ok(McpClientKind::Codex),
        _ => Err(ProviderError::new(format!("unknown mcp kind {:?} (expected client, cursor, copilot, claude, codex, or kiro)", raw))),
    }
}

/// 🪝️ Maps an MCP entry binary to the hook client id used by hook event resolution.
pub fn hook_client_for_mcp_kind(kind: McpClientKind) -> &'static str {
    match kind {
        McpClientKind::Cursor => "cursor-chat",
        McpClientKind::Kiro => "kiro-cli",
        McpClientKind::Copilot => "copilot-chat",
        McpClientKind::Claude => "claude-code",
        McpClientKind::Codex => "codex",
        McpClientKind::Generic => "",
    }
}

/// 🧾️ Maps a validated ticket client slug to an MCP surface kind.
pub fn mcp_kind_from_resolved_client(client: &str) -> McpClientKind {
    match client.trim() {
        "cursor-chat" | "cursor" => McpClientKind::Cursor,
        "kiro-cli" => McpClientKind::Kiro,
        "copilot-chat" => McpClientKind::Copilot,
        "claude-code" => McpClientKind::Claude,
        "codex" => McpClientKind::Codex,
        _ => McpClientKind::Generic,
    }
}

/// 🏷️ The MCP server identifier for the given kind.
pub fn mcp_server_name(kind: McpClientKind) -> &'static str {
    match kind {
        McpClientKind::Cursor | McpClientKind::Kiro | McpClientKind::Copilot | McpClientKind::Claude | McpClientKind::Codex | McpClientKind::Generic => "repo",
    }
}
// #endregion 🪪️McpClientKind

// #region 🏃️ProcessRunner
/// 📨️ One external process invocation: `argv[0]` is the program, the rest are its arguments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessRequest {
    pub argv: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
}

impl ProcessRequest {
    /// 🆕️ Builds a request from a program and its arguments.
    pub fn new(program: &str, args: &[String], cwd: &str) -> ProcessRequest {
        let mut argv = Vec::with_capacity(args.len() + 1);
        argv.push(program.to_string());
        argv.extend(args.iter().cloned());
        ProcessRequest { argv, cwd: if cwd.is_empty() { None } else { Some(cwd.to_string()) }, stdin: None }
    }
}

/// 📬️ What a process left behind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessOutcome {
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    pub status: i32,
}

impl ProcessOutcome {
    /// 🆕️ A successful outcome carrying only standard output.
    pub fn ok(stdout: impl Into<String>) -> ProcessOutcome {
        ProcessOutcome { stdout: stdout.into(), stderr: String::new(), status: 0 }
    }
}

/// 🎞️ One recorded exchange of a process transcript fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessExchange {
    pub argv: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    pub status: i32,
}

/// 🎬️ A whole transcript: the exchanges a recorded runner replays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessTranscript {
    pub exchanges: Vec<ProcessExchange>,
}

/// 🏃️ The single seam through which a provider touches the operating system. Everything a provider
/// does to `gh` or `git` is one call on this port, so a transcript replaces the machine.
#[dyn_enum]
pub trait ProcessRunner {
    /// ▶️ Runs one process and reports what it left behind.
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome;
    /// 📜️ Every argv issued so far, in order.
    fn issued(&self) -> Vec<Vec<String>>;
}

/// 🚫️ Go `exec.Error`'s wording for a program `PATH` does not answer to.
fn not_found_message(program: &str) -> String {
    let variable = if cfg!(windows) { "%PATH%" } else { "$PATH" };
    format!("exec: {:?}: executable file not found in {variable}", program)
}

/// 💻️ Runs the real thing through `std::process`.
#[derive(Debug, Default)]
pub struct SystemProcessRunner {
    issued: RefCell<Vec<Vec<String>>>,
}

impl SystemProcessRunner {
    /// 🆕️ A fresh runner with an empty log.
    pub fn new() -> SystemProcessRunner {
        SystemProcessRunner { issued: RefCell::new(Vec::new()) }
    }
}

impl ProcessRunner for SystemProcessRunner {
    /// ▶️ Runs the request and answers what it left behind. A program `PATH` does not answer to is
    /// reported the way Go's `exec.Error` words it, so a provider's refusal reads the same in both
    /// implementations.
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome {
        self.issued.borrow_mut().push(request.argv.clone());
        let Some((program, args)) = request.argv.split_first() else {
            return ProcessOutcome { stdout: String::new(), stderr: "empty argv".to_string(), status: 127 };
        };
        if semio_framework_repo_workspace::look_path(program).is_none() {
            return ProcessOutcome { stdout: String::new(), stderr: not_found_message(program), status: 127 };
        }
        let mut command = semio_framework_repo_workspace::spawn_command(program);
        command.args(args);
        if let Some(cwd) = &request.cwd {
            command.current_dir(cwd);
        }
        match command.output() {
            Ok(output) => ProcessOutcome {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                status: output.status.code().unwrap_or(-1),
            },
            Err(error) => ProcessOutcome { stdout: String::new(), stderr: error.to_string(), status: 127 },
        }
    }

    fn issued(&self) -> Vec<Vec<String>> {
        self.issued.borrow().clone()
    }
}

/// 🎞️ Replays a recorded transcript: the first not-yet-consumed exchange whose argv matches wins, so
/// a provider's optional calls may be omitted from the fixture without reordering it.
#[derive(Debug)]
pub struct RecordedProcessRunner {
    transcript: ProcessTranscript,
    consumed: RefCell<Vec<bool>>,
    issued: RefCell<Vec<Vec<String>>>,
}

impl RecordedProcessRunner {
    /// 🆕️ Wraps a transcript.
    pub fn new(transcript: ProcessTranscript) -> RecordedProcessRunner {
        let consumed = vec![false; transcript.exchanges.len()];
        RecordedProcessRunner { transcript, consumed: RefCell::new(consumed), issued: RefCell::new(Vec::new()) }
    }

    /// 📥️ Parses a transcript fixture.
    pub fn from_json(source: &str) -> ProviderResult<RecordedProcessRunner> {
        let transcript: ProcessTranscript = serde_json::from_str(source).map_err(|error| ProviderError::new(format!("failed to parse process transcript: {error}")))?;
        Ok(RecordedProcessRunner::new(transcript))
    }
}

impl ProcessRunner for RecordedProcessRunner {
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome {
        self.issued.borrow_mut().push(request.argv.clone());
        let mut consumed = self.consumed.borrow_mut();
        for (index, exchange) in self.transcript.exchanges.iter().enumerate() {
            if consumed[index] || exchange.argv != request.argv {
                continue;
            }
            consumed[index] = true;
            return ProcessOutcome { stdout: exchange.stdout.clone(), stderr: exchange.stderr.clone(), status: exchange.status };
        }
        ProcessOutcome { stdout: String::new(), stderr: format!("no recorded exchange for {:?}", request.argv), status: 127 }
    }

    fn issued(&self) -> Vec<Vec<String>> {
        self.issued.borrow().clone()
    }
}

dyn_enum_close! {
    pub enum ProcessRunners: ProcessRunner {
        System(SystemProcessRunner),
        Recorded(RecordedProcessRunner),
    }
}
// #endregion 🏃️ProcessRunner

// #region 🔭️ProviderInterfaces
/// 💿️ A management issue record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementIssue {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub milestone: Option<ManagementIssueMilestone>,
    #[serde(default)]
    pub labels: Vec<ManagementLabel>,
}

/// 🎯️ The milestone an issue carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementIssueMilestone {
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
}

/// 🎯️ A management milestone record. Field names mirror the GitHub REST payload the `gh` CLI relays.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementMilestone {
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, rename = "due_on")]
    pub due_on: String,
    #[serde(default)]
    pub state: String,
}

/// 🏷️ A management label record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementLabel {
    #[serde(default)]
    pub name: String,
}

/// 🐙️ Issue and milestone management (GitHub, Jira, Trello, Linear, …).
#[dyn_enum]
pub trait ManagementProvider {
    fn kind(&self) -> &'static str;
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String>;
    fn close_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn delete_issue(&self, issue_url_or_number: &str) -> ProviderResult<()>;
    fn update_issue_title(&self, issue_url: &str, title: &str) -> ProviderResult<()>;
    fn update_issue_body(&self, issue_url: &str, body: &str) -> ProviderResult<()>;
    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>>;
    fn get_issue_node_id(&self, issue_url: &str) -> ProviderResult<String>;
    fn get_issue_parent_url(&self, issue_url: &str) -> ProviderResult<String>;
    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()>;
    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn add_issue_to_project(&self, issue_url: &str);
    fn assign_issue_to_current_user(&self, issue_url: &str);
    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> ProviderResult<()>;
    fn update_issue_milestone(&self, issue_url: &str, milestone_title: &str) -> ProviderResult<()>;
    fn clear_issue_milestone(&self, issue_url: &str) -> ProviderResult<()>;
    fn create_milestone(&self, title: &str, description: &str) -> ProviderResult<i64>;
    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> ProviderResult<()>;
    fn delete_milestone(&self, number: i64) -> ProviderResult<()>;
    fn get_milestone(&self, number: i64) -> ProviderResult<Option<ManagementMilestone>>;
    fn get_milestone_title(&self, number: i64) -> ProviderResult<String>;
    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>>;
    fn list_issues_for_label_sync(&self) -> ProviderResult<Vec<ManagementIssue>>;
    fn list_open_issues_with_label(&self, label: &str) -> ProviderResult<Vec<String>>;
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>>;
    fn create_repo_label(&self, name: &str) -> ProviderResult<()>;
    fn delete_repo_label(&self, name: &str) -> ProviderResult<()>;
    fn sync_repo_label_catalog(&self, valid_labels: &[String]) -> ProviderResult<()>;
    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> ProviderResult<String>;
    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> ProviderResult<()>;
    fn get_current_user(&self) -> String;
}

/// 🔌️ Version control operations (git, …).
#[dyn_enum]
pub trait VersionControlProvider {
    fn kind(&self) -> &'static str;
    fn repo_url(&self) -> ProviderResult<String>;
    fn checkpoint(&self, repo_root: &str, description: &str) -> ProviderResult<String>;
    fn current_checkpoint(&self, repo_root: &str) -> ProviderResult<String>;
    fn checkin(&self, repo_root: &str, contributor: &str) -> ProviderResult<()>;
    fn checkout(&self, repo_root: &str, contributor: &str, description: &str) -> ProviderResult<String>;
    fn current_branch(&self, repo_root: &str) -> ProviderResult<String>;
    fn staged_files(&self, repo_root: &str) -> ProviderResult<Vec<String>>;
    fn stage_all(&self, repo_root: &str) -> ProviderResult<()>;
}

/// 🐳️ Sandbox and container operations (devcontainer, podman, …).
#[dyn_enum]
pub trait SandboxProvider {
    fn kind(&self) -> &'static str;
}

/// 📝️ Editor and agent integrations.
#[dyn_enum]
pub trait EditorProvider {
    fn kind(&self) -> &'static str;
    fn resolve_native_event(&self, native_event: &str, tool_kind: ToolKind) -> ProviderResult<(HookEvent, String)>;
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String;
    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String;
}
// #endregion 🔭️ProviderInterfaces

// #region 🔌️Ports
/// 🪝️ The formatting half of an editor, declared here so `🔨️modules/🪝️hooks` depends downward on
/// `🧩️providers` instead of the other way round — fix 5 of `📓️go-region-dependency-graph.md`.
pub trait HookFormatter {
    /// 🧾️ Renders a hook result for one native hook event name.
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String;
    /// 🔤️ The native event name an IDE expects for a neutral hook event.
    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String;
}

impl HookFormatter for EditorProviders {
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String {
        EditorProvider::format_hook_output(self, hook_event_name, result)
    }

    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String {
        EditorProvider::native_event_from_hook_event(self, event, parent_info)
    }
}

/// 🎫️ The narrow issue surface `🔨️modules/🎫️tickets` needs. Declaring it here breaks the
/// `providers → tickets` edge: the GitHub DTOs live with the provider that parses them, and tickets
/// consumes this port — fix 5 of `📓️go-region-dependency-graph.md`.
pub trait IssueTracker {
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String>;
    fn close_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>>;
    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()>;
    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>>;
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>>;
}

impl IssueTracker for ManagementProviders {
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String> {
        ManagementProvider::create_issue(self, title, body, milestone)
    }
    fn close_issue(&self, issue_url: &str) -> ProviderResult<()> {
        ManagementProvider::close_issue(self, issue_url)
    }
    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()> {
        ManagementProvider::reopen_issue(self, issue_url)
    }
    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        ManagementProvider::get_issue_details(self, issue_url)
    }
    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()> {
        ManagementProvider::add_comment(self, issue_url, comment)
    }
    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        ManagementProvider::add_labels(self, issue_url, labels)
    }
    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        ManagementProvider::remove_labels(self, issue_url, labels)
    }
    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        ManagementProvider::find_milestone_by_title(self, title)
    }
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        ManagementProvider::list_repo_labels(self)
    }
}
// #endregion 🔌️Ports

// #region 🐙️GitHubManagementProvider
/// 🔗️ Parses a GitHub issue URL out of `gh issue create` output.
pub fn extract_issue_url(output: &str) -> String {
    let output = output.trim();
    if output.is_empty() {
        return String::new();
    }
    if let Some(first) = output.split_whitespace().next() {
        if first.starts_with("https://") {
            return first.to_string();
        }
    }
    const PREFIX: &str = "https://github.com/";
    if let Some(index) = output.find(PREFIX) {
        let mut rest = &output[index..];
        if let Some(end) = rest.find([' ', '\t', '\r', '\n']) {
            if end > 0 {
                rest = &rest[..end];
            }
        }
        if rest.contains("/issues/") {
            return rest.to_string();
        }
    }
    String::new()
}

/// 🐙️ GitHub management through the `gh` CLI.
pub struct GitHubManagementProvider {
    runner: ProcessRunners,
}

impl GitHubManagementProvider {
    /// 🆕️ Binds the provider to a process runner.
    pub fn new(runner: ProcessRunners) -> GitHubManagementProvider {
        GitHubManagementProvider { runner }
    }

    /// 📜️ Every argv this provider issued, in order.
    pub fn issued(&self) -> Vec<Vec<String>> {
        self.runner.issued()
    }

    fn gh(&self, args: &[&str]) -> ProcessOutcome {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
        self.runner.run(&ProcessRequest::new("gh", &owned, ""))
    }

    fn gh_owned(&self, args: &[String]) -> ProcessOutcome {
        self.runner.run(&ProcessRequest::new("gh", args, ""))
    }

    fn checked(&self, args: &[&str], failure: &str) -> ProviderResult<String> {
        let outcome = self.gh(args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("{failure}: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout)
    }

    fn checked_owned(&self, args: &[String], failure: &str) -> ProviderResult<String> {
        let outcome = self.gh_owned(args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("{failure}: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout)
    }
}

impl ManagementProvider for GitHubManagementProvider {
    fn kind(&self) -> &'static str {
        "github"
    }

    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String> {
        let mut args = vec!["issue".to_string(), "create".to_string(), "--title".to_string(), title.to_string(), "--body".to_string(), body.to_string(), "--label".to_string(), "ticket".to_string()];
        if let Some(number) = milestone {
            if let Ok(milestone_title) = self.get_milestone_title(number) {
                args.push("--milestone".to_string());
                args.push(milestone_title);
            }
        }
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue create failed: {}", outcome.stderr.trim())));
        }
        let mut issue_url = extract_issue_url(&outcome.stdout);
        if issue_url.is_empty() {
            issue_url = extract_issue_url(&outcome.stderr);
        }
        if issue_url.is_empty() {
            return Err(ProviderError::new(format!("gh issue create succeeded but no issue url in output: stdout={:?} stderr={:?}", outcome.stdout.trim(), outcome.stderr.trim())));
        }
        self.add_issue_to_project(&issue_url);
        self.assign_issue_to_current_user(&issue_url);
        Ok(issue_url)
    }

    fn close_issue(&self, issue_url: &str) -> ProviderResult<()> {
        self.checked(&["issue", "close", issue_url], "gh issue close failed").map(|_| ())
    }

    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()> {
        self.checked(&["issue", "reopen", issue_url], "gh issue reopen failed").map(|_| ())
    }

    fn delete_issue(&self, issue_url_or_number: &str) -> ProviderResult<()> {
        let outcome = self.gh(&["issue", "delete", issue_url_or_number, "--yes"]);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue delete failed: {}", outcome.stderr)));
        }
        Ok(())
    }

    fn update_issue_title(&self, issue_url: &str, title: &str) -> ProviderResult<()> {
        self.checked(&["issue", "edit", issue_url, "--title", title], "gh issue edit failed").map(|_| ())
    }

    fn update_issue_body(&self, issue_url: &str, body: &str) -> ProviderResult<()> {
        self.checked(&["issue", "edit", issue_url, "--body", body], "gh issue edit body failed").map(|_| ())
    }

    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        let stdout = self.checked(&["issue", "view", issue_url, "--json", "url,state,milestone,labels,title,body"], "gh issue view failed")?;
        let issue: ManagementIssue = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh issue view output: {error}")))?;
        Ok(Some(issue))
    }

    fn get_issue_node_id(&self, issue_url: &str) -> ProviderResult<String> {
        Ok(self.checked(&["issue", "view", issue_url, "--json", "id", "--jq", ".id"], "gh issue view failed")?.trim().to_string())
    }

    fn get_issue_parent_url(&self, issue_url: &str) -> ProviderResult<String> {
        let query = "query($url: URI!) {\n\t\tresource(url: $url) {\n\t\t\t... on Issue {\n\t\t\t\tparent {\n\t\t\t\t\turl\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}";
        let stdout = self.checked_owned(&["api".to_string(), "graphql".to_string(), "-f".to_string(), format!("url={issue_url}"), "-f".to_string(), format!("query={query}")], "gh api issue parent query failed")?;
        let parsed: Value = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse issue parent response: {error}")))?;
        Ok(parsed.get("data").and_then(|data| data.get("resource")).and_then(|resource| resource.get("parent")).and_then(|parent| parent.get("url")).and_then(Value::as_str).unwrap_or_default().to_string())
    }

    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()> {
        let path = std::env::temp_dir().join(format!("gh-comment-{}.md", std::process::id()));
        std::fs::write(&path, comment).map_err(|error| ProviderError::new(format!("write comment temp file: {error}")))?;
        let body_file = path.to_string_lossy().to_string();
        let outcome = self.gh_owned(&["issue".to_string(), "comment".to_string(), issue_url.to_string(), "--body-file".to_string(), body_file]);
        let _ = std::fs::remove_file(&path);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue comment failed: {}", outcome.stderr.trim())));
        }
        Ok(())
    }

    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        if labels.is_empty() {
            return Ok(());
        }
        let mut args = vec!["issue".to_string(), "edit".to_string(), issue_url.to_string()];
        for label in labels {
            args.push("--add-label".to_string());
            args.push(label.clone());
        }
        self.checked_owned(&args, "gh issue edit failed").map(|_| ())
    }

    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        if labels.is_empty() {
            return Ok(());
        }
        let mut args = vec!["issue".to_string(), "edit".to_string(), issue_url.to_string()];
        for label in labels {
            args.push("--remove-label".to_string());
            args.push(label.clone());
        }
        self.checked_owned(&args, "gh issue edit remove-label failed").map(|_| ())
    }

    fn add_issue_to_project(&self, issue_url: &str) {
        if issue_url.is_empty() {
            return;
        }
        let _ = self.gh(&["project", "item-add", "2", "--owner", "usalu", "--url", issue_url]);
    }

    fn assign_issue_to_current_user(&self, issue_url: &str) {
        if issue_url.is_empty() {
            return;
        }
        let user = self.get_current_user();
        if user.is_empty() {
            return;
        }
        let _ = self.gh(&["issue", "edit", issue_url, "--add-assignee", &user]);
    }

    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> ProviderResult<()> {
        let parent_node_id = self.get_issue_node_id(parent_issue_url).map_err(|error| ProviderError::new(format!("failed to get parent node ID: {error}")))?;
        let child_node_id = self.get_issue_node_id(child_issue_url).map_err(|error| ProviderError::new(format!("failed to get child node ID: {error}")))?;
        let query = "mutation($parentId: ID!, $childId: ID!) {\n\t\taddSubIssue(input: { issueId: $parentId, subIssueId: $childId }) {\n\t\t\tsubIssue { url }\n\t\t}\n\t}";
        let args = vec![
            "api".to_string(),
            "graphql".to_string(),
            "-H".to_string(),
            "GraphQL-Features:sub_issues".to_string(),
            "-f".to_string(),
            format!("parentId={parent_node_id}"),
            "-f".to_string(),
            format!("childId={child_node_id}"),
            "-f".to_string(),
            format!("query={query}"),
        ];
        self.checked_owned(&args, "gh api addSubIssue failed").map(|_| ())
    }

    fn update_issue_milestone(&self, issue_url: &str, milestone_title: &str) -> ProviderResult<()> {
        if milestone_title.trim().is_empty() {
            return Err(ProviderError::new("milestone title is required"));
        }
        self.checked(&["issue", "edit", issue_url, "--milestone", milestone_title], "gh issue edit milestone failed").map(|_| ())
    }

    fn clear_issue_milestone(&self, issue_url: &str) -> ProviderResult<()> {
        let issue_node_id = self.get_issue_node_id(issue_url)?;
        let query = "mutation($issueId: ID!) {\n\t\tupdateIssue(input: { id: $issueId, milestoneId: null }) {\n\t\t\tissue { id }\n\t\t}\n\t}";
        let args = vec!["api".to_string(), "graphql".to_string(), "-f".to_string(), format!("issueId={issue_node_id}"), "-f".to_string(), format!("query={query}")];
        self.checked_owned(&args, "gh api updateIssue clear milestone failed").map(|_| ())
    }

    fn create_milestone(&self, title: &str, description: &str) -> ProviderResult<i64> {
        let args = vec!["api".to_string(), "repos/:owner/:repo/milestones".to_string(), "-f".to_string(), format!("title={title}"), "-f".to_string(), format!("description={description}"), "--jq".to_string(), ".number".to_string()];
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh api milestone create failed: {}", outcome.stderr)));
        }
        Ok(outcome.stdout.trim().parse::<i64>().unwrap_or(0))
    }

    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> ProviderResult<()> {
        let mut args = vec!["api".to_string(), format!("repos/:owner/:repo/milestones/{number}"), "-X".to_string(), "PATCH".to_string()];
        if !title.is_empty() {
            args.push("-f".to_string());
            args.push(format!("title={title}"));
        }
        if !description.is_empty() {
            args.push("-f".to_string());
            args.push(format!("description={description}"));
        }
        if !state.is_empty() {
            args.push("-f".to_string());
            args.push(format!("state={state}"));
        }
        if !due_on.is_empty() {
            let normalized = if due_on.contains('T') { due_on.to_string() } else { format!("{due_on}T00:00:00Z") };
            args.push("-f".to_string());
            args.push(format!("due_on={normalized}"));
        }
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh api milestone update failed: {}", outcome.stderr)));
        }
        Ok(())
    }

    fn delete_milestone(&self, number: i64) -> ProviderResult<()> {
        let args = vec!["api".to_string(), format!("repos/:owner/:repo/milestones/{number}"), "-X".to_string(), "DELETE".to_string()];
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh api milestone delete failed: {}", outcome.stderr)));
        }
        Ok(())
    }

    fn get_milestone(&self, number: i64) -> ProviderResult<Option<ManagementMilestone>> {
        let args = vec!["api".to_string(), format!("repos/:owner/:repo/milestones/{number}")];
        let stdout = self.checked_owned(&args, "gh api milestone get failed")?;
        let milestone: ManagementMilestone = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh milestone output: {error}")))?;
        Ok(Some(milestone))
    }

    fn get_milestone_title(&self, number: i64) -> ProviderResult<String> {
        let args = vec!["api".to_string(), format!("repos/{{owner}}/{{repo}}/milestones/{number}"), "--jq".to_string(), ".title".to_string()];
        Ok(self.checked_owned(&args, "gh api milestone failed")?.trim().to_string())
    }

    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        if title.trim().is_empty() {
            return Ok(None);
        }
        let stdout = self.checked(&["api", "repos/:owner/:repo/milestones", "-X", "GET", "-F", "state=all", "-F", "per_page=100", "--paginate", "--jq", ".[]"], "gh api milestone list failed")?;
        for line in stdout.lines() {
            let Ok(milestone) = serde_json::from_str::<ManagementMilestone>(line) else { continue };
            if milestone.title == title {
                return Ok(Some(milestone));
            }
        }
        Ok(None)
    }

    fn list_issues_for_label_sync(&self) -> ProviderResult<Vec<ManagementIssue>> {
        let stdout = self.checked(&["issue", "list", "--state", "all", "--json", "url,labels", "--limit", "1000"], "gh issue list for label sync failed")?;
        serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh issue list output: {error}")))
    }

    fn list_open_issues_with_label(&self, label: &str) -> ProviderResult<Vec<String>> {
        let stdout = self.checked(&["issue", "list", "--label", label, "--state", "open", "--json", "url", "--limit", "1000"], "gh issue list failed")?;
        let issues: Vec<ManagementIssue> = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh issue list output: {error}")))?;
        Ok(issues.into_iter().map(|issue| issue.url).collect())
    }

    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        let stdout = self.checked(&["label", "list", "--json", "name", "--limit", "1000"], "gh label list failed")?;
        serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh label list output: {error}")))
    }

    fn create_repo_label(&self, name: &str) -> ProviderResult<()> {
        if name.trim().is_empty() {
            return Err(ProviderError::new("label name is required"));
        }
        self.checked(&["label", "create", name, "--color", "1d76db", "--description", "Compose technology or bundle"], "gh label create failed").map(|_| ())
    }

    fn delete_repo_label(&self, name: &str) -> ProviderResult<()> {
        if name.trim().is_empty() {
            return Err(ProviderError::new("label name is required"));
        }
        self.checked(&["label", "delete", name, "--yes"], "gh label delete failed").map(|_| ())
    }

    fn sync_repo_label_catalog(&self, valid_labels: &[String]) -> ProviderResult<()> {
        let existing: Vec<String> = self.list_repo_labels()?.into_iter().map(|label| label.name).collect();
        let mut missing: Vec<&String> = valid_labels.iter().filter(|label| label.starts_with('@') && !existing.contains(label)).collect();
        missing.sort();
        for label in missing {
            let _ = self.create_repo_label(label);
        }
        let mut stale: Vec<&String> = existing.iter().filter(|label| label.starts_with('@') && !valid_labels.contains(label)).collect();
        stale.sort();
        for label in stale {
            let _ = self.delete_repo_label(label);
        }
        Ok(())
    }

    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> ProviderResult<String> {
        let mut args = vec!["issue".to_string(), "create".to_string(), "--title".to_string(), title.to_string(), "--body".to_string(), description.to_string(), "--label".to_string(), "goal".to_string()];
        if let Some(number) = milestone {
            let milestone_title = self.get_milestone_title(number).map_err(|error| ProviderError::new(format!("could not resolve milestone {number}: {error}")))?;
            args.push("--milestone".to_string());
            args.push(milestone_title);
        }
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue create failed: {}", outcome.stderr.trim())));
        }
        let issue_url = outcome.stdout.trim().to_string();
        if !issue_url.is_empty() {
            self.add_issue_to_project(&issue_url);
        }
        Ok(issue_url)
    }

    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> ProviderResult<()> {
        let mut args = vec!["issue".to_string(), "edit".to_string(), issue_url.to_string()];
        if !title.is_empty() {
            args.push("--title".to_string());
            args.push(title.to_string());
        }
        if !description.is_empty() {
            args.push("--body".to_string());
            args.push(description.to_string());
        }
        self.checked_owned(&args, "gh issue edit failed").map(|_| ())
    }

    fn get_current_user(&self) -> String {
        let outcome = self.gh(&["api", "user", "--jq", ".login"]);
        if outcome.status != 0 {
            return String::new();
        }
        outcome.stdout.trim().to_string()
    }
}

/// 💻️ A management provider that does nothing and reports nothing.
#[derive(Debug, Default)]
pub struct NullManagementProvider;

impl ManagementProvider for NullManagementProvider {
    fn kind(&self) -> &'static str {
        "none"
    }
    fn create_issue(&self, _title: &str, _body: &str, _milestone: Option<i64>) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn close_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn reopen_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn delete_issue(&self, _issue_url_or_number: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn update_issue_title(&self, _issue_url: &str, _title: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn update_issue_body(&self, _issue_url: &str, _body: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn get_issue_details(&self, _issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        Ok(None)
    }
    fn get_issue_node_id(&self, _issue_url: &str) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn get_issue_parent_url(&self, _issue_url: &str) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn add_comment(&self, _issue_url: &str, _comment: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn add_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }
    fn remove_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }
    fn add_issue_to_project(&self, _issue_url: &str) {}
    fn assign_issue_to_current_user(&self, _issue_url: &str) {}
    fn add_sub_issue(&self, _parent_issue_url: &str, _child_issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn update_issue_milestone(&self, _issue_url: &str, _milestone_title: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn clear_issue_milestone(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn create_milestone(&self, _title: &str, _description: &str) -> ProviderResult<i64> {
        Ok(0)
    }
    fn update_milestone(&self, _number: i64, _title: &str, _description: &str, _state: &str, _due_on: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn delete_milestone(&self, _number: i64) -> ProviderResult<()> {
        Ok(())
    }
    fn get_milestone(&self, _number: i64) -> ProviderResult<Option<ManagementMilestone>> {
        Ok(None)
    }
    fn get_milestone_title(&self, _number: i64) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn find_milestone_by_title(&self, _title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        Ok(None)
    }
    fn list_issues_for_label_sync(&self) -> ProviderResult<Vec<ManagementIssue>> {
        Ok(Vec::new())
    }
    fn list_open_issues_with_label(&self, _label: &str) -> ProviderResult<Vec<String>> {
        Ok(Vec::new())
    }
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        Ok(Vec::new())
    }
    fn create_repo_label(&self, _name: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn delete_repo_label(&self, _name: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn sync_repo_label_catalog(&self, _valid_labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }
    fn create_goal_issue(&self, _title: &str, _description: &str, _milestone: Option<i64>) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn update_goal_issue(&self, _issue_url: &str, _title: &str, _description: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn get_current_user(&self) -> String {
        String::new()
    }
}

dyn_enum_close! {
    pub enum ManagementProviders: ManagementProvider {
        GitHub(GitHubManagementProvider),
        Null(NullManagementProvider),
    }
}
// #endregion 🐙️GitHubManagementProvider

// #region 🌿️GitVersionControlProvider
/// 🌿️ Version control through the `git` CLI.
pub struct GitVersionControlProvider {
    runner: ProcessRunners,
}

impl GitVersionControlProvider {
    /// 🆕️ Binds the provider to a process runner.
    pub fn new(runner: ProcessRunners) -> GitVersionControlProvider {
        GitVersionControlProvider { runner }
    }

    /// 📜️ Every argv this provider issued, in order.
    pub fn issued(&self) -> Vec<Vec<String>> {
        self.runner.issued()
    }

    fn git(&self, args: &[&str], repo_root: &str) -> ProcessOutcome {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
        self.runner.run(&ProcessRequest::new("git", &owned, repo_root))
    }
}

impl VersionControlProvider for GitVersionControlProvider {
    fn kind(&self) -> &'static str {
        "git"
    }

    fn repo_url(&self) -> ProviderResult<String> {
        let outcome = self.runner.run(&ProcessRequest::new("gh", &["repo".to_string(), "view".to_string(), "--json".to_string(), "url".to_string(), "--jq".to_string(), ".url".to_string()], ""));
        if outcome.status != 0 {
            return Err(ProviderError::new(outcome.stderr.trim().to_string()));
        }
        Ok(outcome.stdout.trim().to_string())
    }

    fn checkpoint(&self, repo_root: &str, description: &str) -> ProviderResult<String> {
        if self.git(&["diff", "--cached", "--quiet"], repo_root).status == 0 {
            self.stage_all(repo_root).map_err(|error| ProviderError::new(format!("stage all failed: {error}")))?;
        }
        let outcome = self.git(&["commit", "-m", description], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git commit failed (exit {}): {}", outcome.status, outcome.stderr.trim())));
        }
        self.current_checkpoint(repo_root).map_err(|error| ProviderError::new(format!("failed to get checkpoint sha after checkpoint: {error}")))
    }

    fn current_checkpoint(&self, repo_root: &str) -> ProviderResult<String> {
        let outcome = self.git(&["rev-parse", "HEAD"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git rev-parse HEAD failed: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout.trim().to_string())
    }

    fn checkin(&self, repo_root: &str, contributor: &str) -> ProviderResult<()> {
        let contributor_branch = format!("{contributor}/latest");
        let fetched = self.git(&["fetch", "origin", "main"], repo_root);
        if fetched.status != 0 {
            return Err(ProviderError::new(format!("git fetch origin main failed: {}", fetched.stderr.trim())));
        }
        let current_branch = self.current_branch(repo_root)?;
        if current_branch != contributor_branch {
            let switched = self.git(&["switch", &contributor_branch], repo_root);
            if switched.status != 0 {
                let created = self.git(&["switch", "-c", &contributor_branch], repo_root);
                if created.status != 0 {
                    return Err(ProviderError::new(format!("git switch to {contributor_branch} failed: {}", created.stderr.trim())));
                }
            }
        }
        let merged = self.git(&["merge", "--ff-only", "origin/main"], repo_root);
        if merged.status != 0 {
            return Err(ProviderError::new(format!("git fast-forward merge to main failed: {}", merged.stderr.trim())));
        }
        Ok(())
    }

    fn checkout(&self, repo_root: &str, contributor: &str, description: &str) -> ProviderResult<String> {
        let contributor_branch = format!("{contributor}/latest");
        let archive_branch = archive_branch_name(contributor, utc_year_month_day());
        if self.git(&["branch", &archive_branch, &contributor_branch], repo_root).status != 0 {
            for suffix in 2..=99 {
                let candidate = format!("{archive_branch}-{suffix}");
                if self.git(&["branch", &candidate, &contributor_branch], repo_root).status == 0 {
                    break;
                }
            }
        }
        let switched = self.git(&["switch", "main"], repo_root);
        if switched.status != 0 {
            return Err(ProviderError::new(format!("git switch to main failed: {}", switched.stderr.trim())));
        }
        let squashed = self.git(&["merge", "--squash", &contributor_branch], repo_root);
        if squashed.status != 0 {
            return Err(ProviderError::new(format!("git squash merge failed: {}", squashed.stderr.trim())));
        }
        let committed = self.git(&["commit", "-m", description], repo_root);
        if committed.status != 0 {
            return Err(ProviderError::new(format!("git commit squash merge failed: {}", committed.stderr.trim())));
        }
        self.current_checkpoint(repo_root)
    }

    fn current_branch(&self, repo_root: &str) -> ProviderResult<String> {
        let outcome = self.git(&["rev-parse", "--abbrev-ref", "HEAD"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git rev-parse --abbrev-ref HEAD failed: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout.trim().to_string())
    }

    fn staged_files(&self, repo_root: &str) -> ProviderResult<Vec<String>> {
        let outcome = self.git(&["diff", "--cached", "--name-only"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git diff --cached --name-only failed: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout.replace("\r\n", "\n").split('\n').filter(|line| !line.is_empty()).map(str::to_string).collect())
    }

    fn stage_all(&self, repo_root: &str) -> ProviderResult<()> {
        let outcome = self.git(&["add", "-A"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git add -A failed: {}", outcome.stderr.trim())));
        }
        Ok(())
    }
}

/// 🗓️ The `contributor/YYYY/MM/DD` archive branch name a checkout parks the previous work on.
pub fn archive_branch_name(contributor: &str, (year, month, day): (i64, i64, i64)) -> String {
    format!("{contributor}/{year:04}/{month:02}/{day:02}")
}

/// 🕰️ Today in UTC as `(year, month, day)`, from the civil-from-days algorithm — no time crate.
pub fn utc_year_month_day() -> (i64, i64, i64) {
    let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |delta| delta.as_secs() as i64);
    civil_from_days(seconds.div_euclid(86_400))
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

dyn_enum_close! {
    pub enum VersionControlProviders: VersionControlProvider {
        Git(GitVersionControlProvider),
    }
}
// #endregion 🌿️GitVersionControlProvider

// #region ⛑️DevcontainerSandboxProvider
/// 🐳️ Devcontainer sandbox.
#[derive(Debug, Default)]
pub struct DevcontainerSandboxProvider;

impl SandboxProvider for DevcontainerSandboxProvider {
    fn kind(&self) -> &'static str {
        "devcontainer"
    }
}

dyn_enum_close! {
    pub enum SandboxProviders: SandboxProvider {
        Devcontainer(DevcontainerSandboxProvider),
    }
}
// #endregion ⛑️DevcontainerSandboxProvider

// #region 🎆️EditorProviders
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
pub fn format_vscode_hook_output(hook_event_name: &str, result: &HookResult) -> String {
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

/// 🧾️ The plain formatting every non-VS-Code editor uses: the marshalled result, nothing added.
pub fn format_plain_hook_output(result: &HookResult) -> String {
    result.to_json()
}

macro_rules! plain_editor_provider {
    ($name:ident, $kind:literal, $resolve:path) => {
        /// 📝️ Editor provider whose hook output is the marshalled result verbatim.
        #[derive(Debug, Default)]
        pub struct $name;

        impl EditorProvider for $name {
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
pub struct CopilotEditorProvider;

impl EditorProvider for CopilotEditorProvider {
    fn kind(&self) -> &'static str {
        "copilot-chat"
    }
    fn resolve_native_event(&self, native_event: &str, tool_kind: ToolKind) -> ProviderResult<(HookEvent, String)> {
        resolve_copilot_event(native_event, tool_kind)
    }
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String {
        format_vscode_hook_output(hook_event_name, result)
    }
    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String {
        vscode_event_from_hook_event(event, parent_info)
    }
}

plain_editor_provider!(CursorEditorProvider, "cursor-chat", resolve_cursor_event);
plain_editor_provider!(WindsurfEditorProvider, "windsurf-chat", resolve_windsurf_event);
plain_editor_provider!(ClaudeCodeEditorProvider, "claude-code", resolve_claude_compatible_event);
plain_editor_provider!(DroidEditorProvider, "droid", resolve_claude_compatible_event);
plain_editor_provider!(CodexEditorProvider, "codex", resolve_claude_compatible_event);
plain_editor_provider!(AntigravityEditorProvider, "antigravity-chat", resolve_claude_compatible_event);
plain_editor_provider!(KiroEditorProvider, "kiro-cli", resolve_kiro_event);

dyn_enum_close! {
    pub enum EditorProviders: EditorProvider {
        Copilot(CopilotEditorProvider),
        Cursor(CursorEditorProvider),
        Windsurf(WindsurfEditorProvider),
        ClaudeCode(ClaudeCodeEditorProvider),
        Droid(DroidEditorProvider),
        Codex(CodexEditorProvider),
        Antigravity(AntigravityEditorProvider),
        Kiro(KiroEditorProvider),
    }
}
// #endregion 🎆️EditorProviders

// #region 🎖️ProviderRegistry
/// 🔌️ Every registered editor provider, in registration order.
pub fn all_editor_providers() -> Vec<EditorProviders> {
    vec![
        EditorProviders::from(CopilotEditorProvider),
        EditorProviders::from(CursorEditorProvider),
        EditorProviders::from(WindsurfEditorProvider),
        EditorProviders::from(ClaudeCodeEditorProvider),
        EditorProviders::from(DroidEditorProvider),
        EditorProviders::from(CodexEditorProvider),
        EditorProviders::from(AntigravityEditorProvider),
        EditorProviders::from(KiroEditorProvider),
    ]
}

/// 💻️ The editor provider for a client slug.
pub fn get_editor_provider(client: &str) -> Option<EditorProviders> {
    all_editor_providers().into_iter().find(|provider| provider.kind() == client)
}

/// 🐙️ The default management provider (GitHub) bound to a runner.
pub fn default_management_provider(runner: ProcessRunners) -> ManagementProviders {
    ManagementProviders::from(GitHubManagementProvider::new(runner))
}

/// 📌️ The default version control provider (git) bound to a runner.
pub fn default_version_control_provider(runner: ProcessRunners) -> VersionControlProviders {
    VersionControlProviders::from(GitVersionControlProvider::new(runner))
}

/// 🐳️ The default sandbox provider (devcontainer).
pub fn default_sandbox_provider() -> SandboxProviders {
    SandboxProviders::from(DevcontainerSandboxProvider)
}

/// 💻️ A management provider bound to the real machine.
pub fn system_management_provider() -> ManagementProviders {
    default_management_provider(ProcessRunners::from(SystemProcessRunner::new()))
}

/// 💻️ A version control provider bound to the real machine.
pub fn system_version_control_provider() -> VersionControlProviders {
    default_version_control_provider(ProcessRunners::from(SystemProcessRunner::new()))
}
// #endregion 🎖️ProviderRegistry

// #region 🪝️GitHookFiles

/// 🪝️ Every git hook filename this repository may install and must never leave enabled.
pub const REPO_MANAGED_GIT_HOOKS: &[&str] = &["pre-commit", "post-commit", "prepare-commit-msg", "commit-msg", "pre-push", "pre-rebase", "post-merge", "post-checkout", "post-rewrite"];

/// 🔄️ The hooks the micro-commit workflow installs, in installation order.
pub const MICRO_COMMIT_GIT_HOOKS: &[&str] = &["prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"];

/// 📁️ The checked-in hook scripts the micro-commit workflow copies from.
pub fn micro_commit_hook_source_dir(repo_root: &std::path::Path) -> std::path::PathBuf {
    repo_root.join("🧰️framework").join("🛍️products").join("🦑️repo").join("🪝️hooks")
}

/// 🧹️ Deletes the repo-managed git hooks so commits, rebases and squashes stay unblocked, and
/// answers the names it actually removed in [`REPO_MANAGED_GIT_HOOKS`] order.
///
/// Twin of `RemoveGitHooks` in `🪝️hooks/📦️packages/🐹️go/🐹️.go`: an absent hook is skipped, and an
/// empty root removes nothing.
pub fn remove_git_hooks(repo_root: &std::path::Path) -> ProviderResult<Vec<String>> {
    if repo_root.as_os_str().is_empty() {
        return Ok(Vec::new());
    }
    let hooks_dir = repo_root.join(".git").join("hooks");
    let mut removed = Vec::new();
    for name in REPO_MANAGED_GIT_HOOKS {
        let path = hooks_dir.join(name);
        match std::fs::remove_file(&path) {
            Ok(()) => removed.push((*name).to_string()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(ProviderError::new(format!("remove git hook {name}: {error}"))),
        }
    }
    Ok(removed)
}

/// 🔄️ Copies the checked-in micro-commit hook scripts into `.git/hooks`.
///
/// Twin of `InstallMicroCommitHooks`: an empty root installs nothing, and a missing source script
/// is the refusal `read micro-commit hook <name>: <error>`.
pub fn install_micro_commit_hooks(repo_root: &std::path::Path) -> ProviderResult<()> {
    if repo_root.as_os_str().is_empty() {
        return Ok(());
    }
    let source_dir = micro_commit_hook_source_dir(repo_root);
    let hooks_dir = repo_root.join(".git").join("hooks");
    for name in MICRO_COMMIT_GIT_HOOKS {
        let body = std::fs::read(source_dir.join(name)).map_err(|error| ProviderError::new(format!("read micro-commit hook {name}: {error}")))?;
        let target = hooks_dir.join(name);
        std::fs::write(&target, &body).map_err(|error| ProviderError::new(format!("install micro-commit hook {name}: {error}")))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o755)).map_err(|error| ProviderError::new(format!("install micro-commit hook {name}: {error}")))?;
        }
    }
    Ok(())
}

// #endregion 🪝️GitHookFiles

// #region 🧪️Tests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
