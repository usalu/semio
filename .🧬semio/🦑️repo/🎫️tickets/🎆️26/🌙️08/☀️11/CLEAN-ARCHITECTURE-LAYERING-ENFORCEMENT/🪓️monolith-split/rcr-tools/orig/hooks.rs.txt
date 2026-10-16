//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 🪝️ Lifecycle hooks of the `🦑️repo` product: an IDE hands a native payload to the repo binary,
//! this module normalises it into a neutral [`HookEvent`], decides whether the tool invocation is
//! allowed, projects the event-specific record, formats it the way the calling IDE reads it, and
//! records it in the session log.
//!
//! Layering (`📓️go-region-dependency-graph.md`, fixes 5 and 7): the per-IDE native event tables and
//! the output formatting live in `🧩️providers`; this module reaches DOWN into them through
//! [`HookFormatter`] and [`EditorProvider`] and providers never reaches back. `McpClientKind` is
//! taken from `🧩️providers` too, so nothing here imports `🔌️mcp` at L5.
//!
//! Everything that would otherwise touch a clock, a process or the filesystem is a port with an
//! inert default: [`HookEnvironment`] (checkpoint identity, commit message, formatter),
//! [`TestFileResolver`] (command → test file paths, owned by `🏃️test-runner`) and [`SessionStore`]
//! (where `session.json` lives). A caller that supplies none of them still gets the full
//! normalisation, blocking, plan and formatting behaviour, deterministically.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/🧬️schema/🔣️.json

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

//#region 🔁️Reexports
/// 🔁️ The JSON reader hook payloads arrive in. Re-exported because a hook payload is JSON by
/// definition: a client of this crate — a test adapter, the CLI, the MCP server — must be able to
/// build and read one without declaring a dependency this crate already carries.
pub use serde_json;
/// 🔁️ The recorded plan shapes this module folds an incoming plan into, declared by `📐️model`.
pub use semio_framework_repo_model::{TicketAgentPlan, TicketAgentPlanStep};
/// 🔁️ The event vocabulary, the result shape, the editor providers that format it and the process
/// runner micro-commit is delegated through — all declared by `🧩️providers`, re-exported so no
/// consumer has to name a module one level down to speak about a hook. `📋️plan.md` §3: an exported
/// api never forces a client to reach past the module it is talking to.
pub use semio_framework_repo_providers::{format_plain_hook_output, get_editor_provider, resolve_claude_compatible_event, EditorProvider, EditorProviders, HookEvent, HookFormatter, HookResult, McpClientKind, ProcessExchange, ProcessOutcome, ProcessRequest, ProcessRunner, ProcessTranscript, RecordedProcessRunner, SystemProcessRunner, ToolKind, ALL_HOOK_EVENTS};
/// 🔁️ The logging switches this module obeys, declared by `🏠️workspace` which owns `📋️config.toml`.
pub use semio_framework_repo_workspace::{load_repo_config, parse_repo_config, repo_meta_dir_for_root, LoggingConfig, RepoConfig};
//#endregion 🔁️Reexports

//#region 🔖️HookError
/// ⚠️ Every hook failure, carrying the message the Go implementation formats verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookError {
    pub message: String,
}

impl HookError {
    /// 🆕️ Wraps a message.
    pub fn new(message: impl Into<String>) -> HookError {
        HookError { message: message.into() }
    }
}

impl std::fmt::Display for HookError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for HookError {}

/// 🎯️ The crate's result alias.
pub type HookOutcome<T> = Result<T, HookError>;
//#endregion 🔖️HookError

//#region 🏷️HookKind
/// 🏷️ Whether an event belongs to the version control lifecycle or to an agent session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HookKind {
    Version,
    Agent,
}

impl HookKind {
    /// 🔤️ The wire slug.
    pub fn as_str(self) -> &'static str {
        match self {
            HookKind::Version => "version",
            HookKind::Agent => "agent",
        }
    }
}

/// 🔷️ The hook kind of an event: the six `version.*` events, everything else is agent work.
pub fn hook_event_kind(event: HookEvent) -> HookKind {
    match event {
        HookEvent::VersionCheckpointStarting
        | HookEvent::VersionCheckpointEnded
        | HookEvent::VersionCheckinStarting
        | HookEvent::VersionCheckinEnded
        | HookEvent::VersionCheckoutStarting
        | HookEvent::VersionCheckoutEnded => HookKind::Version,
        _ => HookKind::Agent,
    }
}

/// 🔤️ Resolves a neutral event slug, listing every valid slug when it does not.
pub fn validate_hook_event(raw: &str) -> HookOutcome<HookEvent> {
    HookEvent::parse(raw).ok_or_else(|| HookError::new(format!("invalid hook event {raw:?}, valid events: {}", hook_event_slugs().join(", "))))
}

/// 📋️ Every valid neutral event slug, in declaration order.
pub fn hook_event_slugs() -> Vec<&'static str> {
    ALL_HOOK_EVENTS.iter().map(|event| event.as_str()).collect()
}
//#endregion 🏷️HookKind

//#region 🎯️HookContext
/// 🎯️ Everything one hook invocation knows: the resolved event, who sent it and the native payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HookContext {
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "client", default)]
    pub client: String,
    #[serde(rename = "second", default)]
    pub second: String,
    #[serde(rename = "repoRoot", default)]
    pub repo_root: String,
    #[serde(rename = "toolName", default, skip_serializing_if = "String::is_empty")]
    pub tool_name: String,
    #[serde(rename = "toolArgs", default, skip_serializing_if = "String::is_empty")]
    pub tool_args: String,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
    #[serde(rename = "parentInfo", default, skip_serializing_if = "String::is_empty")]
    pub parent_info: String,
    #[serde(rename = "extra", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
    #[serde(rename = "input", default, skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,
}

impl HookContext {
    /// 🆕️ A context for one resolved event and client.
    pub fn new(event: HookEvent, client: impl Into<String>) -> HookContext {
        HookContext { event: event.as_str().to_string(), client: client.into(), ..HookContext::default() }
    }

    /// 📡️ The resolved event, or `None` when the slug is not a neutral event.
    pub fn hook_event(&self) -> Option<HookEvent> {
        HookEvent::parse(&self.event)
    }

    /// 🕰️ Sets the RFC 3339 second the caller stamped this invocation with.
    pub fn at_second(mut self, second: impl Into<String>) -> HookContext {
        self.second = second.into();
        self
    }

    /// 🧰️ Sets the tool name and raw argument string the IDE reported.
    pub fn with_tool(mut self, tool_name: impl Into<String>, tool_args: impl Into<String>) -> HookContext {
        self.tool_name = tool_name.into();
        self.tool_args = tool_args.into();
        self
    }

    /// 👪️ Sets the parent hint (`subagent`, or a parent session id).
    pub fn with_parent(mut self, parent_info: impl Into<String>) -> HookContext {
        self.parent_info = parent_info.into();
        self
    }

    /// 📨️ Attaches the native payload the IDE wrote to stdin.
    pub fn with_input(mut self, input: Value) -> HookContext {
        self.input = Some(input);
        self
    }

    /// 🏠️ Sets the repository root the hook runs against.
    pub fn at_root(mut self, repo_root: impl Into<String>) -> HookContext {
        self.repo_root = repo_root.into();
        self
    }
}
//#endregion 🎯️HookContext

//#region 🔌️Ports
/// 🌍️ The three effects a hook needs from the machine, each with an inert default so a caller that
/// wants pure normalisation supplies nothing.
pub trait HookEnvironment {
    /// 🔖️ The current version control checkpoint id, when the payload does not carry one.
    fn current_checkpoint(&self) -> String {
        String::new()
    }
    /// 📝️ The checkpoint message being prepared under `repo_root`, when the payload carries none.
    fn checkpoint_message(&self, _repo_root: &str) -> String {
        String::new()
    }
    /// 🖌️ Formats a file that a code edit just finished writing.
    fn format_file(&self, _path: &str) {}
}

/// 🚫️ The environment that does nothing and knows nothing — the deterministic default.
#[derive(Debug, Default, Clone, Copy)]
pub struct InertEnvironment;

impl HookEnvironment for InertEnvironment {}

/// 🧪️ Resolution of a shell command into the test files it would run — owned by `🏃️test-runner`,
/// consumed here so the test events can name labs and tests without this module walking a codebase.
pub trait TestFileResolver {
    /// 📄️ The test files the command selects.
    fn test_files(&self, _command: &str, _cwd: &str) -> Vec<String> {
        Vec::new()
    }
    /// 🧫️ The lab ids of those files.
    fn lab_ids(&self, _file_paths: &[String]) -> Vec<String> {
        Vec::new()
    }
}

/// 🚫️ The resolver that resolves nothing — the deterministic default.
#[derive(Debug, Default, Clone, Copy)]
pub struct InertTestFileResolver;

impl TestFileResolver for InertTestFileResolver {}
//#endregion 🔌️Ports

//#region 🧭️PayloadReading
/// 🔍️ The payload object, when the input is a JSON object at all.
pub fn decode_hook_input_map(input: Option<&Value>) -> Option<&Map<String, Value>> {
    input.and_then(Value::as_object)
}

/// 🪆️ The first non-blank string reachable under any of `keys`, searched breadth-first at each level
/// then depth-first into every child — the Go `findNestedStringValue` traversal.
pub fn find_nested_string_value(value: Option<&Value>, keys: &[&str]) -> String {
    let Some(value) = value else { return String::new() };
    match value {
        Value::Object(object) => {
            for key in keys {
                if let Some(Value::String(raw)) = object.get(*key) {
                    if !raw.trim().is_empty() {
                        return raw.trim().to_string();
                    }
                }
            }
            for nested in object.values() {
                let found = find_nested_string_value(Some(nested), keys);
                if !found.is_empty() {
                    return found;
                }
            }
            String::new()
        }
        Value::Array(items) => {
            for nested in items {
                let found = find_nested_string_value(Some(nested), keys);
                if !found.is_empty() {
                    return found;
                }
            }
            String::new()
        }
        _ => String::new(),
    }
}

/// 🗺️ The `tool_input` object, at the top level or under `event` or `native.event`.
pub fn extract_tool_input_map(data: Option<&Value>) -> Option<&Map<String, Value>> {
    let object = data?.as_object()?;
    if let Some(tool_input) = object.get("tool_input").and_then(Value::as_object) {
        return Some(tool_input);
    }
    if let Some(event) = object.get("event").and_then(Value::as_object) {
        if let Some(tool_input) = event.get("tool_input").and_then(Value::as_object) {
            return Some(tool_input);
        }
    }
    if let Some(native) = object.get("native").and_then(Value::as_object) {
        if let Some(event) = native.get("event").and_then(Value::as_object) {
            if let Some(tool_input) = event.get("tool_input").and_then(Value::as_object) {
                return Some(tool_input);
            }
        }
    }
    None
}

/// 🔹️ The shell command the payload carries: a bare JSON string, `tool_input.command`, or any
/// nested `command` / `command_line` / `commandLine`.
pub fn extract_command_from_stdin(input: Option<&Value>) -> String {
    let Some(input) = input else { return String::new() };
    if let Value::String(raw) = input {
        return raw.trim().to_string();
    }
    if input.as_object().is_none() {
        return String::new();
    }
    if let Some(tool_input) = extract_tool_input_map(Some(input)) {
        if !tool_input.is_empty() {
            if let Some(Value::String(command)) = tool_input.get("command") {
                return command.trim().to_string();
            }
        }
    }
    find_nested_string_value(Some(input), &["command", "command_line", "commandLine"])
}

/// 📁️ The command and the working directory it runs in; an absent directory stays empty rather than
/// silently becoming the host's cwd, so the projection never depends on where a test ran.
pub fn extract_command_and_cwd(input: Option<&Value>) -> (String, String) {
    let command = extract_command_from_stdin(input);
    let mut cwd = String::new();
    if let Some(tool_input) = extract_tool_input_map(input) {
        if let Some(Value::String(raw)) = tool_input.get("cwd") {
            cwd = raw.trim().to_string();
        }
    }
    if cwd.is_empty() {
        cwd = find_nested_string_value(input, &["cwd", "working_directory", "workingDirectory"]);
    }
    (command, cwd)
}

/// 🔷️ The tool name the payload names.
pub fn extract_tool_name(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["tool", "tool_name", "toolName", "mcp_tool_name", "mcpToolName"])
}

/// 📡️ The native hook event name the payload names.
pub fn extract_hook_event_name(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["hookEventName", "hook_event_name", "hook_event", "hookEvent", "event"])
}

/// 🧾️ The transcript path the payload names.
pub fn extract_transcript(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["transcript", "transcript_path", "transcriptPath", "log_path", "logPath"])
}

/// 🧠️ The model the payload names.
pub fn extract_llm(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["llm", "model", "model_name", "modelName"])
}

/// 🏋️ The reasoning effort the payload names.
pub fn extract_effort(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["effort", "reasoning_effort", "reasoningEffort"])
}

/// ▪️ The message identifier of the turn.
pub fn extract_message_id(input: Option<&Value>) -> String {
    top_level_string(input, &["messageId", "message_id", "turnId", "turn_id", "requestId", "request_id"])
}

/// ▫️ The message identifier of the parent turn.
pub fn extract_parent_message_id(input: Option<&Value>) -> String {
    top_level_string(input, &["parentMessageId", "parent_message_id", "parentTurnId", "parent_turn_id"])
}

fn top_level_string(input: Option<&Value>, keys: &[&str]) -> String {
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    for key in keys {
        if let Some(Value::String(raw)) = object.get(*key) {
            if !raw.is_empty() {
                return raw.trim().to_string();
            }
        }
    }
    String::new()
}

/// 🪪️ The session identifier, falling back to the basename of the transcript path.
pub fn extract_session_id(input: Option<&Value>) -> String {
    let session = find_nested_string_value(input, &["session_id", "sessionId", "trajectory_id", "trajectoryId", "conversation_id", "conversationId", "agent_id", "agentId"]);
    if !session.is_empty() {
        return session;
    }
    let transcript = extract_transcript(input);
    if transcript.is_empty() {
        return String::new();
    }
    let base = base_name(transcript.trim());
    match base.rfind('.') {
        Some(index) if index > 0 => base[..index].trim().to_string(),
        _ => base.trim().to_string(),
    }
}

/// 🪪️ Normalises a parent hint: the literal `subagent` is a shape, not a session id.
pub fn normalize_parent_session_id(parent: &str) -> String {
    let parent = parent.trim();
    if parent.is_empty() || parent.eq_ignore_ascii_case("subagent") {
        return String::new();
    }
    parent.to_string()
}

fn parent_from_object(object: &Map<String, Value>) -> String {
    for key in ["parent", "parentInfo", "parent_info", "parentSessionId", "parent_session_id", "parentId", "parent_id"] {
        if let Some(Value::String(raw)) = object.get(key) {
            let parent = normalize_parent_session_id(raw);
            if !parent.is_empty() {
                return parent;
            }
        }
    }
    String::new()
}

/// 👪️ The parent session the payload names, at the top level, under `event` or under `native`.
pub fn extract_parent(input: Option<&Value>) -> String {
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    let parent = parent_from_object(object);
    if !parent.is_empty() {
        return parent;
    }
    for key in ["event", "native"] {
        let Some(nested) = object.get(key).and_then(Value::as_object) else { continue };
        let parent = parent_from_object(nested);
        if !parent.is_empty() {
            return parent;
        }
        if key == "native" {
            if let Some(native_event) = nested.get("event").and_then(Value::as_object) {
                let parent = parent_from_object(native_event);
                if !parent.is_empty() {
                    return parent;
                }
            }
        }
    }
    String::new()
}

/// 🔳️ The parent session: the caller's hint when it names one, otherwise the payload's.
pub fn resolve_parent_session_id(parent_info: &str, input: Option<&Value>) -> String {
    let parent = normalize_parent_session_id(parent_info);
    if !parent.is_empty() {
        return parent;
    }
    extract_parent(input)
}

/// 💬️ The conversation the compaction event is about.
pub fn extract_chat(input: Option<&Value>) -> String {
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    for key in ["chat", "conversation", "context", "messages"] {
        match object.get(key) {
            Some(Value::String(raw)) => return raw.trim().to_string(),
            Some(Value::Array(items)) => return Value::Array(items.clone()).to_string(),
            _ => {}
        }
    }
    String::new()
}

/// 📦️ The report an agent-ended payload carries, at the top level or under `event`/`input`/`native`.
pub fn extract_report(input: Option<&Value>) -> String {
    fn report_of(node: Option<&Map<String, Value>>) -> String {
        let Some(node) = node else { return String::new() };
        if let Some(Value::String(report)) = node.get("report") {
            if !report.trim().is_empty() {
                return report.trim().to_string();
            }
        }
        if let Some(tool_info) = node.get("tool_info").and_then(Value::as_object) {
            if let Some(Value::String(response)) = tool_info.get("response") {
                if !response.trim().is_empty() {
                    return response.trim().to_string();
                }
            }
        }
        String::new()
    }
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    let report = report_of(Some(object));
    if !report.is_empty() {
        return report;
    }
    for key in ["event", "input"] {
        let report = report_of(object.get(key).and_then(Value::as_object));
        if !report.is_empty() {
            return report;
        }
    }
    if let Some(native) = object.get("native").and_then(Value::as_object) {
        let report = report_of(Some(native));
        if !report.is_empty() {
            return report;
        }
        let report = report_of(native.get("event").and_then(Value::as_object));
        if !report.is_empty() {
            return report;
        }
    }
    String::new()
}

/// 🔖️ The checkpoint identifier the payload names, falling back to the environment.
pub fn extract_checkpoint_id(input: Option<&Value>, environment: &dyn HookEnvironment) -> String {
    let checkpoint = top_level_string(input, &["sha", "checkpoint_sha", "checkpointSha", "hash"]);
    if !checkpoint.is_empty() {
        return checkpoint;
    }
    environment.current_checkpoint()
}

/// 📝️ The checkpoint message the payload names, falling back to the environment.
pub fn extract_checkpoint_message(input: Option<&Value>, repo_root: &str, environment: &dyn HookEnvironment) -> String {
    let message = top_level_string(input, &["message", "commit_message", "commitMessage"]);
    if !message.is_empty() {
        return message;
    }
    environment.checkpoint_message(repo_root)
}

/// 📝️ What a code edit event carries: the path, the replaced text, the replacement, and whether the
/// edit applies to every occurrence.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeEdit {
    pub path: String,
    pub old: String,
    pub new: String,
    pub all: bool,
}

/// 📝️ Reads a code edit out of the payload, or out of the raw tool argument string.
pub fn extract_code_edit(input: Option<&Value>, tool_args: &str) -> CodeEdit {
    let fallback = parse_tool_args(tool_args);
    let source = extract_tool_input_map(input).cloned().or(fallback);
    let Some(source) = source else { return CodeEdit::default() };
    let mut edit = CodeEdit::default();
    for key in ["filePath", "path", "file_path", "file"] {
        if let Some(Value::String(raw)) = source.get(key) {
            if !raw.is_empty() {
                edit.path = raw.clone();
                break;
            }
        }
    }
    for key in ["oldString", "old", "old_string", "search"] {
        if let Some(Value::String(raw)) = source.get(key) {
            edit.old = raw.clone();
            break;
        }
    }
    for key in ["newString", "new", "new_string", "replace"] {
        if let Some(Value::String(raw)) = source.get(key) {
            edit.new = raw.clone();
            break;
        }
    }
    if let Some(Value::Bool(all)) = source.get("all") {
        edit.all = *all;
    } else if let Some(Value::Bool(all)) = source.get("replaceAll") {
        edit.all = *all;
    }
    edit
}

/// 🟫️ The terminal command: the payload's, or the raw tool argument string's `command` member.
pub fn extract_terminal_command(input: Option<&Value>, tool_args: &str) -> String {
    let command = extract_command_from_stdin(input);
    if !command.is_empty() {
        return command;
    }
    match parse_tool_args(tool_args).and_then(|args| args.get("command").cloned()) {
        Some(Value::String(command)) => command,
        _ => String::new(),
    }
}

/// 💠️ What a finished terminal invocation reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalOutcome {
    pub command: String,
    pub pid: String,
    pub terminated: bool,
    pub stdout: String,
    pub stderr: String,
}

/// 💠️ Reads a finished terminal invocation out of the payload.
pub fn extract_terminal_outcome(input: Option<&Value>) -> TerminalOutcome {
    let Some(object) = decode_hook_input_map(input) else { return TerminalOutcome::default() };
    let mut outcome = TerminalOutcome::default();
    let tool_input = object.get("tool_input").and_then(Value::as_object).unwrap_or(object);
    for key in ["command", "command_line"] {
        if let Some(Value::String(raw)) = tool_input.get(key) {
            if !raw.is_empty() {
                outcome.command = raw.clone();
                break;
            }
        }
    }
    if let Some(tool_info) = object.get("tool_info").and_then(Value::as_object) {
        if let Some(Value::String(raw)) = tool_info.get("command_line") {
            if !raw.is_empty() {
                outcome.command = raw.clone();
            }
        }
    }
    for key in ["pid", "process_id", "execution_id", "id"] {
        match object.get(key) {
            Some(Value::String(raw)) if !raw.is_empty() => {
                outcome.pid = raw.clone();
                break;
            }
            Some(Value::Number(number)) => {
                outcome.pid = format!("{}", number.as_f64().unwrap_or_default() as i64);
                break;
            }
            _ => {}
        }
    }
    for key in ["terminated", "has_terminated", "exited", "stopped"] {
        if let Some(Value::Bool(flag)) = object.get(key) {
            outcome.terminated = *flag;
            break;
        }
    }
    for key in ["stdout", "output", "tool_output"] {
        if let Some(Value::String(raw)) = object.get(key) {
            outcome.stdout = raw.clone();
            break;
        }
    }
    for key in ["stderr", "error_output"] {
        if let Some(Value::String(raw)) = object.get(key) {
            outcome.stderr = raw.clone();
            break;
        }
    }
    outcome
}

/// 🔎️ Web pages and file line ranges a search event touched.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchReads {
    pub pages: Vec<String>,
    pub ranges: Vec<String>,
}

/// 🔎️ Reads the search coverage out of the payload, or out of the raw tool argument string.
pub fn extract_search_reads(input: Option<&Value>, tool_args: &str) -> SearchReads {
    let Some(source) = plan_source(input, tool_args) else { return SearchReads::default() };
    let mut reads = SearchReads::default();
    if let Some(Value::String(url)) = source.get("url") {
        if !url.is_empty() {
            reads.pages.push(url.clone());
        }
    }
    if let Some(Value::Array(pages)) = source.get("pages") {
        for page in pages {
            if let Value::String(page) = page {
                if page.starts_with("http://") || page.starts_with("https://") {
                    reads.pages.push(page.clone());
                }
            }
        }
    }
    let file_path = match (source.get("file_path"), source.get("filePath")) {
        (Some(Value::String(path)), _) => path.clone(),
        (_, Some(Value::String(path))) => path.clone(),
        _ => String::new(),
    };
    if file_path.is_empty() {
        return reads;
    }
    let number = |key: &str| source.get(key).and_then(Value::as_f64).unwrap_or_default() as i64;
    let start_line = number("startLine");
    let mut end_line = number("endLine");
    let limit = number("limit");
    let offset = if source.contains_key("offset") { number("offset") } else { 1 };
    if start_line > 0 {
        if end_line <= 0 {
            end_line = start_line;
        }
        if start_line == end_line {
            reads.ranges.push(format!("{file_path}#L{start_line}"));
        } else {
            reads.ranges.push(format!("{file_path}#L{start_line}-L{end_line}"));
        }
    } else if limit > 0 {
        let start = if offset > 0 { offset } else { 1 };
        reads.ranges.push(format!("{file_path}#L{start}-L{}", start + limit - 1));
    } else {
        reads.ranges.push(file_path);
    }
    reads
}

fn parse_tool_args(tool_args: &str) -> Option<Map<String, Value>> {
    if tool_args.is_empty() {
        return None;
    }
    serde_json::from_str::<Value>(tool_args).ok().and_then(|value| value.as_object().cloned())
}
//#endregion 🧭️PayloadReading

//#region 🏛️ToolClassification
/// 🏛️ The tool kind of a named tool, across every IDE's tool vocabulary.
pub fn classify_tool(tool_name: &str) -> ToolKind {
    match tool_name {
        "manage_todo_list" | "Task" | "task" | "todo_tool" | "TodoWrite" => ToolKind::Plan,
        "read_file" | "grep_search" | "rg" | "ripgrep" | "file_search" | "semantic_search" | "list_dir" | "list_code_usages" | "get_errors" | "Read" | "fetch_webpage" | "open_simple_browser" | "Grep" | "Glob" | "fs_read" | "code" | "grep" | "glob" | "web_search" | "web_fetch" => ToolKind::CodeSearch,
        "replace_string_in_file" | "create_file" | "multi_replace_string_in_file" | "Edit" | "Write" | "editfile" | "fs_write" => ToolKind::CodeEdit,
        "run_in_terminal" | "get_terminal_output" | "Bash" | "terminal" | "execute_bash" => ToolKind::Terminal,
        "runTests" | "run_tests" => ToolKind::Test,
        "run_task" | "create_and_run_task" => ToolKind::Build,
        _ => ToolKind::Generic,
    }
}

const TEST_COMMAND_NAMES: [&str; 8] = ["pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"];

const SEARCH_COMMAND_NAMES: [&str; 68] = [
    "grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind", "locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less", "more", "head", "tail", "wc", "file", "stat", "du", "which", "whereis", "type", "command", "hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath", "basename", "dirname", "jq", "yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column", "rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env", "printenv", "set", "export", "pwd", "id", "whoami", "hostname",
];

const SEARCH_COMMAND_NAMES_TAIL: [&str; 15] = ["uname", "date", "uptime", "free", "df", "ps", "top", "htop", "lsof", "netstat", "ss", "test", "python-none", "node-none", "unused"];

const EDIT_COMMAND_NAMES: [&str; 27] = ["rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown", "chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip", "gunzip", "bzip2", "bunzip2", "xz", "unxz", "zstd", "unused"];

/// 🔤️ The tool kind a raw shell command implies.
pub fn classify_command_kind(command: &str) -> ToolKind {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return ToolKind::Terminal;
    }
    let segments = split_command_segments(trimmed);
    if segments.len() > 1 {
        let mut best = ToolKind::Terminal;
        for segment in &segments {
            let kind = classify_command_kind(segment);
            if matches!(kind, ToolKind::Test | ToolKind::Build | ToolKind::CodeEdit | ToolKind::Plan) {
                return kind;
            }
            if kind == ToolKind::CodeSearch {
                best = ToolKind::CodeSearch;
            }
        }
        if best != ToolKind::Terminal {
            return best;
        }
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let Some(first) = parts.first() else { return ToolKind::Terminal };
    let base = base_name(first);
    if base == "test" || base == "[" {
        return ToolKind::CodeSearch;
    }
    if base.starts_with("phpunit") || matches!(base.as_str(), "rspec" | "pytest" | "py.test" | "jest" | "vitest" | "mocha" | "gradle" | "mvn" | "bundle") {
        return ToolKind::Test;
    }
    if trimmed.contains("cargo nextest") {
        return ToolKind::Test;
    }
    if TEST_COMMAND_NAMES.iter().any(|name| trimmed.contains(name)) {
        return ToolKind::Test;
    }
    const TEST_PREFIXES: [&str; 24] = [
        "npm test", "npm run test", "pnpm test", "yarn test", "bun test", "go test", "cargo test", "cargo nextest", "make test", "make check", "dotnet test", "swift test", "dart test", "flutter test", "mix test", "mvn test", "mvn verify", "gradle test", "./gradlew test", "gradlew test", "cabal test", "stack test", "lein test", "sbt test",
    ];
    if TEST_PREFIXES.iter().any(|prefix| trimmed.starts_with(prefix)) {
        return ToolKind::Test;
    }
    if trimmed.contains("pytest") || trimmed.contains("unittest") || trimmed.contains("phpunit") {
        return ToolKind::Test;
    }
    if trimmed.starts_with("bundle exec rspec") || trimmed.starts_with("tox") || trimmed.starts_with("rspec ") {
        return ToolKind::Test;
    }
    if SEARCH_COMMAND_NAMES.contains(&base.as_str()) || SEARCH_COMMAND_NAMES_TAIL.contains(&base.as_str()) {
        return ToolKind::CodeSearch;
    }
    let stream_editor = base == "sed";
    let awk_family = matches!(base.as_str(), "awk" | "gawk" | "mawk" | "nawk");
    if (stream_editor || awk_family) && !trimmed.contains("-i") {
        return ToolKind::CodeSearch;
    }
    if EDIT_COMMAND_NAMES.contains(&base.as_str()) {
        return ToolKind::CodeEdit;
    }
    if (stream_editor || awk_family) && trimmed.contains("-i") {
        return ToolKind::CodeEdit;
    }
    ToolKind::Terminal
}

/// ✂️ Cuts a command at its first unquoted pipe.
pub fn trim_pipeline_tail(segment: &str) -> String {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let mut current = String::new();
    let mut quote = '\0';
    for character in trimmed.chars() {
        if quote != '\0' {
            current.push(character);
            if character == quote {
                quote = '\0';
            }
            continue;
        }
        if character == '\'' || character == '"' {
            quote = character;
            current.push(character);
            continue;
        }
        if character == '|' {
            break;
        }
        current.push(character);
    }
    current.trim().to_string()
}

fn is_test_command_segment(segment: &str, binary: &str) -> bool {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return false;
    }
    let fields: Vec<&str> = trimmed.split_whitespace().collect();
    let binary = if binary.is_empty() {
        match fields.first() {
            Some(first) => base_name(first),
            None => return false,
        }
    } else {
        binary.to_string()
    };
    if classify_command_kind(trimmed) == ToolKind::Test {
        return true;
    }
    if fields.len() < 2 {
        return false;
    }
    if matches!(binary.as_str(), "npx" | "pnpm" | "yarn" | "bun") && classify_command_kind(&fields[1..].join(" ")) == ToolKind::Test {
        return true;
    }
    if binary == "uv" {
        if fields[1] == "run" && fields.len() > 2 && classify_command_kind(&fields[2..].join(" ")) == ToolKind::Test {
            return true;
        }
        if classify_command_kind(&fields[1..].join(" ")) == ToolKind::Test {
            return true;
        }
    }
    false
}

/// 🧪️ The test-running segment of a composite command, with the directory a leading `cd` selected.
pub fn extract_test_segment(command: &str) -> (String, String) {
    let command = command.trim();
    if command.is_empty() {
        return (String::new(), String::new());
    }
    let mut segments = split_command_segments(command);
    if segments.is_empty() {
        segments = vec![command.to_string()];
    }
    let composite = segments.len() > 1;
    let mut cwd = String::new();
    for segment in &segments {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }
        if let Some(rest) = segment.strip_prefix("cd ") {
            if let Some(target) = rest.split_whitespace().next() {
                cwd = target.to_string();
            }
            continue;
        }
        if segment.starts_with("export ") || segment.starts_with("set ") {
            continue;
        }
        let Some(first) = segment.split_whitespace().next() else { continue };
        let binary = base_name(first);
        if is_test_command_segment(segment, &binary) {
            return (trim_pipeline_tail(segment), cwd);
        }
        if matches!(binary.as_str(), "npx" | "pnpm" | "yarn" | "bun" | "uv") && is_test_command_segment(segment, "") {
            return (trim_pipeline_tail(segment), cwd);
        }
    }
    if composite {
        return (String::new(), cwd);
    }
    (String::new(), String::new())
}

/// 🔶️ Normalises a native or neutral event name into a neutral [`HookEvent`] plus a parent hint.
///
/// A name that is already a neutral slug wins outright. Otherwise the tool name and the payload's
/// command decide the tool kind, and the calling IDE's own table (from `🧩️providers`) resolves the
/// native name — an unknown name is refused, never silently defaulted.
pub fn resolve_hook_event(name: &str, client: &str, tool_name: &str, input: Option<&Value>) -> HookOutcome<(HookEvent, String)> {
    if let Ok(event) = validate_hook_event(name) {
        return Ok((event, String::new()));
    }
    let mut kind = ToolKind::Generic;
    if !tool_name.trim().is_empty() {
        kind = classify_tool(tool_name);
    }
    let command = extract_command_from_stdin(input);
    if !command.is_empty() {
        let (segment, _) = extract_test_segment(&command);
        let candidate = if segment.trim().is_empty() { command.as_str() } else { segment.as_str() };
        let command_kind = classify_command_kind(candidate);
        if matches!(kind, ToolKind::Generic | ToolKind::Terminal) || matches!(command_kind, ToolKind::Test | ToolKind::Build | ToolKind::CodeSearch | ToolKind::CodeEdit) {
            kind = command_kind;
        }
    }
    if tool_name == "Glob" && input.is_some() {
        kind = ToolKind::Generic;
    }
    match get_editor_provider(client) {
        Some(provider) => provider.resolve_native_event(name, kind).map_err(|error| HookError::new(error.message)),
        None => resolve_claude_compatible_event(name, kind).map_err(|error| HookError::new(error.message)),
    }
}
//#endregion 🏛️ToolClassification

//#region 🛡️BlockingPolicy
/// 🧩️ Tool names that are refused outright, whatever their arguments.
pub const BLOCKED_TOOL_PATTERNS: [&str; 27] = [
    "rm", "rmdir", "mv", "cp", "git add", "git branch", "git checkout", "git cherry-pick", "git clean", "git clone", "git commit", "git config", "git fetch", "git init", "git merge", "git mv", "git pull", "git push", "git rebase", "git remote", "git reset", "git restore", "git revert", "git rm", "git stash", "git switch", "git tag",
];

/// ⌨️ The git subcommands that modify repository state and are therefore refused.
pub const BLOCKED_GIT_VERBS: [&str; 23] = ["add", "branch", "checkout", "cherry-pick", "clone", "commit", "config", "fetch", "init", "merge", "mv", "pull", "push", "rebase", "remote", "reset", "restore", "revert", "rm", "stash", "switch", "tag", "clean"];

const CONCURRENCY_NOTE: &str = "; other developers and agents may be editing the same files concurrently";

/// 🔷️ Splits a command at every unquoted `;`, `&&`, `|` and `||`.
pub fn split_command_segments(command: &str) -> Vec<String> {
    let command = command.trim();
    if command.is_empty() {
        return Vec::new();
    }
    let characters: Vec<char> = command.chars().collect();
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut quote = '\0';
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        if quote != '\0' {
            current.push(character);
            if character == quote {
                quote = '\0';
            }
            index += 1;
            continue;
        }
        if character == '\'' || character == '"' {
            quote = character;
            current.push(character);
            index += 1;
            continue;
        }
        if character == ';' {
            push_segment(&mut segments, &mut current);
            index += 1;
            continue;
        }
        if character == '&' && index + 1 < characters.len() && characters[index + 1] == '&' {
            push_segment(&mut segments, &mut current);
            index += 2;
            continue;
        }
        if character == '|' {
            push_segment(&mut segments, &mut current);
            index += if index + 1 < characters.len() && characters[index + 1] == '|' { 2 } else { 1 };
            continue;
        }
        current.push(character);
        index += 1;
    }
    push_segment(&mut segments, &mut current);
    segments
}

fn push_segment(segments: &mut Vec<String>, current: &mut String) {
    let segment = current.trim().to_string();
    if !segment.is_empty() {
        segments.push(segment);
    }
    current.clear();
}

fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn at_word_boundary(characters: &[char], start: usize, end: usize) -> bool {
    let before = start == 0 || !is_word_character(characters[start - 1]);
    let after = end >= characters.len() || !is_word_character(characters[end]);
    before && after
}

/// 🌿️ The first blocked `git <verb>` invocation in arbitrary inline code, matching the Go
/// `\bgit\s+(verb)\b` pattern case-insensitively and returning the matched text.
fn find_inline_git_verb(code: &str) -> Option<String> {
    let characters: Vec<char> = code.chars().collect();
    for start in 0..characters.len() {
        if !(characters[start].eq_ignore_ascii_case(&'g') && start + 3 <= characters.len()) {
            continue;
        }
        let word: String = characters[start..(start + 3).min(characters.len())].iter().collect();
        if !word.eq_ignore_ascii_case("git") {
            continue;
        }
        if start > 0 && is_word_character(characters[start - 1]) {
            continue;
        }
        let mut cursor = start + 3;
        let space_start = cursor;
        while cursor < characters.len() && characters[cursor].is_whitespace() {
            cursor += 1;
        }
        if cursor == space_start {
            continue;
        }
        for verb in BLOCKED_GIT_VERBS {
            let verb_characters: Vec<char> = verb.chars().collect();
            let end = cursor + verb_characters.len();
            if end > characters.len() {
                continue;
            }
            let candidate: String = characters[cursor..end].iter().collect();
            if !candidate.eq_ignore_ascii_case(verb) {
                continue;
            }
            if end < characters.len() && is_word_character(characters[end]) {
                continue;
            }
            return Some(characters[start..end].iter().collect());
        }
    }
    None
}

/// 📚️ Whether inline code carries a list-form git invocation such as `['git', 'stash']`.
fn contains_list_form_git(code: &str) -> bool {
    let characters: Vec<char> = code.chars().collect();
    let is_quote = |index: usize| index < characters.len() && (characters[index] == '\'' || characters[index] == '"');
    let skip_spaces = |mut index: usize| {
        while index < characters.len() && characters[index].is_whitespace() {
            index += 1;
        }
        index
    };
    for start in 0..characters.len() {
        if !is_quote(start) {
            continue;
        }
        let mut cursor = skip_spaces(start + 1);
        if cursor + 3 > characters.len() {
            continue;
        }
        let word: String = characters[cursor..cursor + 3].iter().collect();
        if !word.eq_ignore_ascii_case("git") {
            continue;
        }
        cursor = skip_spaces(cursor + 3);
        if !is_quote(cursor) {
            continue;
        }
        cursor = skip_spaces(cursor + 1);
        if cursor >= characters.len() || characters[cursor] != ',' {
            continue;
        }
        cursor = skip_spaces(cursor + 1);
        if !is_quote(cursor) {
            continue;
        }
        cursor = skip_spaces(cursor + 1);
        for verb in BLOCKED_GIT_VERBS {
            let verb_characters: Vec<char> = verb.chars().collect();
            let end = cursor + verb_characters.len();
            if end > characters.len() {
                continue;
            }
            let candidate: String = characters[cursor..end].iter().collect();
            if !candidate.eq_ignore_ascii_case(verb) {
                continue;
            }
            let closing = skip_spaces(end);
            if is_quote(closing) {
                return true;
            }
        }
    }
    false
}

/// 🌿️ Scans arbitrary inline code (`python -c`, `node -e`, …) for a blocked git invocation.
pub fn contains_blocked_git_in_code(code: &str) -> Option<String> {
    if let Some(matched) = find_inline_git_verb(code) {
        return Some(format!("blocked: {}", matched.trim().to_lowercase()));
    }
    if contains_list_form_git(code) {
        return Some("blocked: git (list form) in inline code".to_string());
    }
    None
}

/// 🎯️ Whether a lowercased command kills a process selected by `lsof -t -i:PORT`, which in a
/// container can match PID 1 and terminate the whole devcontainer.
fn matches_kill_lsof_port(lower: &str) -> bool {
    let characters: Vec<char> = lower.chars().collect();
    let find_word = |needle: &str, from: usize| -> Option<usize> {
        let needle_characters: Vec<char> = needle.chars().collect();
        (from..characters.len()).find(|&start| {
            let end = start + needle_characters.len();
            end <= characters.len() && characters[start..end] == needle_characters[..] && at_word_boundary(&characters, start, end)
        })
    };
    let find_literal = |needle: &str, from: usize| -> Option<usize> {
        let needle_characters: Vec<char> = needle.chars().collect();
        (from..characters.len()).find(|&start| {
            let end = start + needle_characters.len();
            end <= characters.len() && characters[start..end] == needle_characters[..]
        })
    };
    let Some(kill) = find_word("kill", 0) else { return false };
    let Some(open) = find_literal("$(", kill + 4) else { return false };
    let mut cursor = open + 2;
    while cursor < characters.len() && characters[cursor].is_whitespace() {
        cursor += 1;
    }
    let Some(lsof) = find_word("lsof", cursor) else { return false };
    if lsof != cursor {
        return false;
    }
    let Some(dash_t) = find_literal("-t", lsof + 4) else { return false };
    let Some(dash_i) = find_literal("-i", dash_t + 2) else { return false };
    let mut cursor = dash_i + 2;
    while cursor < characters.len() && characters[cursor].is_whitespace() {
        cursor += 1;
    }
    if cursor >= characters.len() || characters[cursor] != ':' {
        return false;
    }
    cursor += 1;
    while cursor < characters.len() && characters[cursor].is_whitespace() {
        cursor += 1;
    }
    let digits_start = cursor;
    while cursor < characters.len() && characters[cursor].is_ascii_digit() {
        cursor += 1;
    }
    if cursor == digits_start {
        return false;
    }
    find_literal(")", cursor).is_some()
}

const SHELL_INTERPRETERS: [&str; 8] = ["bash", "sh", "zsh", "fish", "ksh", "csh", "tcsh", "dash"];
const SCRIPT_INTERPRETERS: [&str; 13] = ["python", "python3", "python2", "node", "nodejs", "perl", "ruby", "php", "lua", "tclsh", "groovy", "scala", "unused"];
const ALLOWED_SEGMENT_PREFIXES: [&str; 10] = ["grep ", "rg ", "ripgrep ", "echo ", "printf ", "ls ", "pwd", "cat ", "sed ", "awk "];

/// 🐙️ The reason one command segment is refused, or `None` when it is allowed.
pub fn is_command_segment_blocked(segment: &str) -> Option<String> {
    let segment = segment.trim();
    if segment.is_empty() {
        return None;
    }
    let lower = segment.to_lowercase();
    if matches_kill_lsof_port(&lower) {
        return Some("blocked: kill $(lsof -t -i:PORT); this can match PID 1 in containers and terminate the devcontainer, stopping all running work".to_string());
    }
    if ALLOWED_SEGMENT_PREFIXES.iter().any(|prefix| lower.starts_with(prefix)) {
        return None;
    }
    let mut tokens: Vec<String> = lower.split_whitespace().map(str::to_string).collect();
    while let Some(first) = tokens.first().cloned() {
        if first.contains('=') || matches!(first.as_str(), "env" | "command" | "sudo") {
            tokens.remove(0);
            continue;
        }
        if SHELL_INTERPRETERS.contains(&first.as_str()) || first == "xargs" {
            let joined = tokens.join(" ");
            return match joined.find("git ") {
                Some(index) => is_command_segment_blocked(&joined[index..]),
                None => None,
            };
        }
        if SCRIPT_INTERPRETERS.contains(&first.as_str()) {
            return contains_blocked_git_in_code(&tokens[1..].join(" "));
        }
        break;
    }
    let first = tokens.first_mut()?;
    if first.ends_with("/git") {
        *first = "git".to_string();
    }
    if tokens[0] != "git" {
        return None;
    }
    let mut verb_index = 1;
    while verb_index < tokens.len() && tokens[verb_index].starts_with('-') {
        verb_index += 1;
        if verb_index < tokens.len() && !tokens[verb_index].starts_with('-') && (tokens[verb_index - 1] == "-c" || tokens[verb_index - 1] == "-C") {
            verb_index += 1;
        }
    }
    if verb_index >= tokens.len() {
        return None;
    }
    let verb = tokens[verb_index].trim_matches(|character| character == '"' || character == '\'');
    if !BLOCKED_GIT_VERBS.contains(&verb) {
        return None;
    }
    if verb == "clean" && !lower.contains("-fd") && !lower.contains("-df") {
        return None;
    }
    Some(format!("blocked: git {verb}"))
}

/// ✔️ Whether an invocation is refused, and why. The arguments are examined segment by segment
/// first, so a blocked git call hidden behind `&&`, a pipe, a shell or an inline script is caught
/// before the tool name is even considered.
pub fn is_tool_blocked(tool: &str, args: &str) -> Option<String> {
    for segment in split_command_segments(args) {
        if let Some(reason) = is_command_segment_blocked(&segment) {
            return Some(format!("{reason}{CONCURRENCY_NOTE}"));
        }
    }
    let tool = tool.trim().to_lowercase();
    if BLOCKED_TOOL_PATTERNS.contains(&tool.as_str()) {
        return Some(format!("blocked: {tool}{CONCURRENCY_NOTE}"));
    }
    None
}
//#endregion 🛡️BlockingPolicy

//#region 🗺️PlanSteps
/// 🔁️ One step of a plan / task list update event.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookPlanStep {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", default)]
    pub status: String,
}

fn plan_source(input: Option<&Value>, tool_args: &str) -> Option<Map<String, Value>> {
    let data = match input {
        Some(value) => value.as_object().cloned().or_else(|| parse_tool_args(tool_args)),
        None => parse_tool_args(tool_args),
    }?;
    match data.get("tool_input") {
        Some(Value::Object(tool_input)) => Some(tool_input.clone()),
        Some(_) => None,
        None => Some(data),
    }
}

/// 🗺️ Reads the plan steps out of a payload: VS Code's `todoList` entries (`title` + `status`) and
/// the neutral `steps` entries (`name` + `status`), in that order, from `tool_input` when present.
pub fn extract_plan_steps(input: Option<&Value>, tool_args: &str) -> Vec<HookPlanStep> {
    let Some(source) = plan_source(input, tool_args) else { return Vec::new() };
    let mut steps = Vec::new();
    if let Some(Value::Array(entries)) = source.get("todoList") {
        for entry in entries {
            let Some(entry) = entry.as_object() else { continue };
            let name = entry.get("title").and_then(Value::as_str).unwrap_or_default();
            let status = entry.get("status").and_then(Value::as_str).unwrap_or_default();
            if !name.is_empty() {
                steps.push(HookPlanStep { name: name.to_string(), status: status.to_string() });
            }
        }
    }
    if let Some(Value::Array(entries)) = source.get("steps") {
        for entry in entries {
            let Some(entry) = entry.as_object() else { continue };
            let name = entry.get("name").and_then(Value::as_str).unwrap_or_default();
            let status = entry.get("status").and_then(Value::as_str).unwrap_or_default();
            if !name.is_empty() {
                steps.push(HookPlanStep { name: name.to_string(), status: status.to_string() });
            }
        }
    }
    steps
}

/// 🔀️ Folds an incoming plan into the recorded one: an incoming step keeps whatever timestamps it
/// already had, gains `ideated` the first time it is seen, `started` when it goes in progress and
/// `completed` when it finishes after having started; a recorded step the incoming plan dropped is
/// abandoned unless it already carries a lifecycle timestamp.
pub fn merge_plan_steps(existing: &[TicketAgentPlanStep], incoming: &[HookPlanStep], second: &str) -> Vec<TicketAgentPlanStep> {
    let mut by_name: BTreeMap<&str, &TicketAgentPlanStep> = BTreeMap::new();
    for step in existing {
        by_name.insert(step.name.as_str(), step);
    }
    let mut active: BTreeSet<String> = BTreeSet::new();
    let mut merged: Vec<TicketAgentPlanStep> = Vec::with_capacity(existing.len() + incoming.len());
    for step in incoming {
        let name = step.name.trim();
        if name.is_empty() {
            continue;
        }
        let mut resolved = by_name.get(name).map_or_else(|| TicketAgentPlanStep {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            status: String::new(),
            ideated: String::new(),
            started: String::new(),
            completed: String::new(),
            abandoned: String::new(),
        }, |found| (*found).clone());
        resolved.name = name.to_string();
        resolved.status = step.status.clone();
        if resolved.ideated.is_empty() {
            resolved.ideated = second.to_string();
        }
        match step.status.trim().to_lowercase().as_str() {
            "in-progress" | "in_progress" | "started" => {
                if resolved.started.is_empty() {
                    resolved.started = second.to_string();
                }
            }
            "completed" | "done" if !resolved.started.is_empty() && resolved.completed.is_empty() => {
                resolved.completed = second.to_string();
            }
            _ => {}
        }
        active.insert(name.to_string());
        merged.push(resolved);
    }
    for step in existing {
        if active.contains(&step.name) || step.name.is_empty() {
            continue;
        }
        let mut resolved = step.clone();
        if !resolved.abandoned.is_empty() || !resolved.completed.is_empty() || !resolved.started.is_empty() {
            merged.push(resolved);
            continue;
        }
        if resolved.ideated.is_empty() {
            resolved.ideated = second.to_string();
        }
        resolved.abandoned = second.to_string();
        merged.push(resolved);
    }
    merged
}
//#endregion 🗺️PlanSteps

//#region 🔶️Dispatch
fn insert_when_present(result: HookResult, key: &str, value: &str) -> HookResult {
    if value.is_empty() {
        result
    } else {
        result.with(key, Value::String(value.to_string()))
    }
}

fn string_array(values: &[String]) -> Value {
    Value::Array(values.iter().map(|value| Value::String(value.clone())).collect())
}

fn agent_base(context: &HookContext, environment: &dyn HookEnvironment) -> HookResult {
    let input = context.input.as_ref();
    let mut session = extract_session_id(input);
    let second = first_non_empty(&[find_nested_string_value(input, &["second"]), context.second.clone()]);
    let mut parent = resolve_parent_session_id("", input);
    if !context.parent_info.is_empty() {
        parent = resolve_parent_session_id(&context.parent_info, input);
        if context.parent_info == "subagent" {
            let agent_id = find_nested_string_value(input, &["agent_id", "agentId"]);
            if !agent_id.is_empty() {
                let parent_session = first_non_empty(&[extract_parent(input), extract_session_id(input)]);
                session = agent_id;
                parent = normalize_parent_session_id(&parent_session);
            } else if parent.is_empty() {
                parent = session.clone();
            }
        }
    }
    let mut result = HookResult { message_shadowed: true, ..HookResult::new(true, "") };
    result = insert_when_present(result, "checkpoint", &extract_checkpoint_id(input, environment));
    result = insert_when_present(result, "session", &session);
    result = insert_when_present(result, "second", &second);
    result = insert_when_present(result, "client", &context.client);
    result = insert_when_present(result, "llm", &extract_llm(input));
    result = insert_when_present(result, "effort", &extract_effort(input));
    result = insert_when_present(result, "transcript", &extract_transcript(input));
    result = insert_when_present(result, "message", &extract_message_id(input));
    result.with("parent", Value::String(parent))
}

/// 🔤️ The first non-blank value.
pub fn first_non_empty(values: &[String]) -> String {
    values.iter().map(|value| value.trim()).find(|value| !value.is_empty()).unwrap_or_default().to_string()
}

/// 🔶️ Builds the event-specific hook result for one context.
///
/// Blocking is applied here and nowhere else: `agent.tool.starting` and
/// `agent.tool.terminal.starting` are the two events that can refuse an invocation, and a refusal is
/// carried as `allowed = false` plus the reason, exactly as every IDE reads it back.
pub fn dispatch_hook(context: &HookContext, environment: &dyn HookEnvironment, tests: &dyn TestFileResolver) -> HookResult {
    let Some(event) = context.hook_event() else {
        return HookResult::new(false, format!("unknown event: {}", context.event));
    };
    let input = context.input.as_ref();
    if hook_event_kind(event) == HookKind::Version {
        let mut result = HookResult::new(true, if event == HookEvent::VersionCheckpointStarting { "checkpoint starting hook passed" } else { "" });
        result = insert_when_present(result, "checkpoint", &extract_checkpoint_id(input, environment));
        if matches!(event, HookEvent::VersionCheckpointStarting | HookEvent::VersionCheckpointEnded) {
            result = insert_when_present(result, "description", &extract_checkpoint_message(input, &context.repo_root, environment));
        }
        return result;
    }
    let base = agent_base(context, environment);
    match event {
        HookEvent::AgentStarted => base,
        HookEvent::AgentEnded => insert_when_present(base, "report", &extract_report(input)),
        HookEvent::AgentPromptSubmitting => insert_when_present(base, "prompt", &find_nested_string_value(input, &["prompt", "text"])),
        HookEvent::AgentCompacting => insert_when_present(base, "chat", &extract_chat(input)),
        HookEvent::AgentToolStarting => {
            let tool_name = first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]);
            let args = first_non_empty(&[context.tool_args.clone(), extract_terminal_command(input, &context.tool_args), extract_command_from_stdin(input)]);
            let reason = is_tool_blocked(&tool_name, &args);
            let mut result = HookResult { allowed: reason.is_none(), message: reason.unwrap_or_default(), extra: base.extra, message_shadowed: base.message_shadowed };
            result = insert_when_present(result, "name", &tool_name);
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            result
        }
        HookEvent::AgentToolEnded => {
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]));
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            if let Some(response) = extract_tool_response(input) {
                result = result.with("response", response);
            }
            result
        }
        HookEvent::AgentToolPlanUpdatingStarting | HookEvent::AgentToolPlanUpdatingEnded => {
            let steps = extract_plan_steps(input, &context.tool_args);
            if steps.is_empty() {
                base
            } else {
                base.with("steps", serde_json::to_value(&steps).unwrap_or(Value::Null))
            }
        }
        HookEvent::AgentToolSearchStarting | HookEvent::AgentToolSearchEnded => {
            let reads = extract_search_reads(input, &context.tool_args);
            let mut result = base;
            if !reads.pages.is_empty() {
                result = result.with("pages", string_array(&reads.pages));
            }
            if !reads.ranges.is_empty() {
                result = result.with("ranges", string_array(&reads.ranges));
            }
            result
        }
        HookEvent::AgentToolCodeEditStarting => {
            let edit = extract_code_edit(input, &context.tool_args);
            let mut result = insert_when_present(base, "path", &edit.path);
            result = insert_when_present(result, "old", &edit.old);
            result = insert_when_present(result, "new", &edit.new);
            if edit.all {
                result = result.with("all", Value::Bool(true));
            }
            result
        }
        HookEvent::AgentToolCodeEditEnded => {
            let edit = extract_code_edit(input, &context.tool_args);
            if !edit.path.is_empty() {
                environment.format_file(&edit.path);
            }
            let mut result = insert_when_present(base, "path", &edit.path);
            result = insert_when_present(result, "old", &edit.old);
            insert_when_present(result, "new", &edit.new)
        }
        HookEvent::AgentToolTestStarting => {
            let selection = extract_test_selection(input, &context.tool_args, tests);
            let mut result = base;
            if !selection.labs.is_empty() {
                result = result.with("labs", string_array(&selection.labs));
            }
            if !selection.tests.is_empty() {
                result = result.with("tests", string_array(&selection.tests));
            }
            insert_when_present(result, "timeout", &selection.timeout)
        }
        HookEvent::AgentToolTestEnded => {
            let outcome = extract_test_outcome(input, tests);
            let mut result = base;
            if !outcome.files.is_empty() {
                result = result.with("files", string_array(&outcome.files));
            }
            if !outcome.succeeded.is_empty() {
                result = result.with("succeeded", string_array(&outcome.succeeded));
            }
            if !outcome.failed.is_empty() {
                result = result.with("failed", string_array(&outcome.failed));
            }
            result
        }
        HookEvent::AgentToolBuildStarting => {
            let bundles = extract_build_bundles(input, &context.tool_args);
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), "build".to_string()]));
            result = result.with("input", serde_json::json!({ "bundles": bundles }));
            result
        }
        HookEvent::AgentToolBuildEnded => {
            let outcome = extract_build_outcome(input);
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), "build".to_string()]));
            result = result.with("response", serde_json::json!({ "succeeded": outcome.succeeded, "failed": outcome.failed }));
            result
        }
        HookEvent::AgentToolTerminalStarting => {
            let command = extract_terminal_command(input, &context.tool_args);
            let tool = first_non_empty(&[context.tool_name.clone(), "run_in_terminal".to_string()]);
            let reason = is_tool_blocked(&tool, &first_non_empty(&[command.clone(), context.tool_args.clone()]));
            let mut result = HookResult { allowed: reason.is_none(), message: reason.unwrap_or_default(), extra: base.extra, message_shadowed: base.message_shadowed };
            result = insert_when_present(result, "name", &first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]));
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            insert_when_present(result, "command", &command)
        }
        HookEvent::AgentToolTerminalEnded => {
            let outcome = extract_terminal_outcome(input);
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]));
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            result = insert_when_present(result, "command", &outcome.command);
            result = result.with("pid", Value::Number(outcome.pid.parse::<i64>().unwrap_or_default().into()));
            if outcome.terminated {
                result = result.with("terminated", Value::Bool(true));
            }
            result = result.with("stdout", Value::String(outcome.stdout));
            result.with("stderr", Value::String(outcome.stderr))
        }
        HookEvent::AgentThinkingStarting | HookEvent::AgentThinkingEnded => {
            let text = find_nested_string_value(input, &["text"]);
            HookResult { allowed: base.allowed, message: text, extra: base.extra, message_shadowed: base.message_shadowed }
        }
        _ => base,
    }
}

fn extract_tool_response(input: Option<&Value>) -> Option<Value> {
    let object = decode_hook_input_map(input)?;
    if let Some(response) = object.get("tool_response") {
        return Some(response.clone());
    }
    if let Some(output) = object.get("tool_output") {
        return Some(output.clone());
    }
    let output = find_nested_string_value(input, &["tool_output", "toolOutput", "output", "response"]);
    if output.is_empty() {
        None
    } else {
        Some(Value::String(output))
    }
}

/// 🧪️ What a starting test event selected.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSelection {
    pub labs: Vec<String>,
    pub tests: Vec<String>,
    pub timeout: String,
}

/// 🧪️ What a finished test event reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestOutcome {
    pub files: Vec<String>,
    pub succeeded: Vec<String>,
    pub failed: Vec<String>,
}

fn looks_like_file_path(value: &str) -> bool {
    value.contains('/') || value.contains('\\') || (value.contains('.') && !value.starts_with('.'))
}

fn string_members(source: &Map<String, Value>, keys: &[&str]) -> Vec<String> {
    let mut collected = Vec::new();
    for key in keys {
        match source.get(*key) {
            Some(Value::Array(items)) => {
                for item in items {
                    if let Value::String(item) = item {
                        if !item.is_empty() {
                            collected.push(item.clone());
                        }
                    }
                }
            }
            Some(Value::String(item)) if !item.is_empty() => collected.push(item.clone()),
            _ => {}
        }
    }
    collected
}

/// 🧪️ Reads a starting test event: the files and test selectors it names, and its timeout.
pub fn extract_test_selection(input: Option<&Value>, tool_args: &str, tests: &dyn TestFileResolver) -> TestSelection {
    let source = extract_tool_input_map(input).cloned().or_else(|| parse_tool_args(tool_args)).unwrap_or_default();
    let mut selection = TestSelection::default();
    let mut target_files = string_members(&source, &["files"]);
    for named in string_members(&source, &["testNames", "test_names", "tests"]) {
        if looks_like_file_path(&named) {
            target_files.push(named);
        } else {
            selection.tests.push(named);
        }
    }
    for key in ["timeout", "timeoutMs"] {
        match source.get(key) {
            Some(Value::String(raw)) if !raw.is_empty() => {
                selection.timeout = raw.clone();
                break;
            }
            Some(Value::Number(number)) => {
                selection.timeout = format!("{}", number.as_f64().unwrap_or_default() as i64);
                break;
            }
            _ => {}
        }
    }
    let mut command = extract_command_from_stdin(input);
    if command.is_empty() {
        if let Some(Value::String(raw)) = source.get("command") {
            command = raw.clone();
        }
    }
    if !command.is_empty() && target_files.is_empty() {
        let (_, cwd) = extract_command_and_cwd(input);
        let resolved = tests.test_files(&command, &cwd);
        if !resolved.is_empty() {
            target_files = resolved;
        }
    }
    if !target_files.is_empty() && selection.tests.iter().all(String::is_empty) {
        selection.labs = tests.lab_ids(&target_files);
        if !selection.labs.is_empty() {
            selection.tests.clear();
            return selection;
        }
    }
    selection
}

/// 🧪️ Reads a finished test event: the files it ran and which cases passed or failed.
pub fn extract_test_outcome(input: Option<&Value>, tests: &dyn TestFileResolver) -> TestOutcome {
    let Some(object) = decode_hook_input_map(input) else { return TestOutcome::default() };
    let mut outcome = TestOutcome::default();
    if let Some(tool_input) = extract_tool_input_map(input) {
        outcome.files = string_members(tool_input, &["files"]);
    }
    if outcome.files.is_empty() {
        let (command, cwd) = extract_command_and_cwd(input);
        if !command.is_empty() {
            outcome.files = tests.test_files(&command, &cwd);
        }
    }
    let tool_output = object.get("tool_output").and_then(Value::as_object).unwrap_or(object);
    outcome.succeeded = string_members(tool_output, &["succeeded", "passed", "passing"]);
    outcome.failed = string_members(tool_output, &["failed", "failing", "errors"]);
    outcome
}

/// 🏗️ The bundles a starting build event names.
pub fn extract_build_bundles(input: Option<&Value>, tool_args: &str) -> Vec<String> {
    let source = extract_tool_input_map(input).cloned().or_else(|| parse_tool_args(tool_args));
    let Some(source) = source else { return Vec::new() };
    string_members(&source, &["bundles", "targets", "technologies", "label"])
}

/// 🏗️ What a finished build event reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildOutcome {
    pub succeeded: Vec<String>,
    pub failed: Vec<String>,
}

/// 🏗️ Reads a finished build event.
pub fn extract_build_outcome(input: Option<&Value>) -> BuildOutcome {
    let Some(object) = decode_hook_input_map(input) else { return BuildOutcome::default() };
    let tool_output = object.get("tool_output").and_then(Value::as_object).unwrap_or(object);
    BuildOutcome { succeeded: string_members(tool_output, &["succeeded", "passed", "built"]), failed: string_members(tool_output, &["failed", "errors"]) }
}
//#endregion 🔶️Dispatch

//#region 🖨️Formatting
/// 🖨️ What the repo binary writes after a hook, and with which exit code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookOutput {
    #[serde(rename = "stdout", default)]
    pub stdout: String,
    #[serde(rename = "stderr", default)]
    pub stderr: String,
    #[serde(rename = "exitCode", default)]
    pub exit_code: i32,
}

/// 🖨️ Renders a hook result the way the calling client reads it.
///
/// Copilot Chat is the only client whose result is wrapped (`hookSpecificOutput`), and it is written
/// to stdout whether or not the invocation was allowed — the permission decision lives inside the
/// record. `json_mode` prints the plain record for every other client. Otherwise a refusal goes to
/// stderr with exit code 2 and an allowed result prints only its message, if it has one.
pub fn render_hook_output(client: &str, event: HookEvent, parent_info: &str, native_event_name: &str, result: &HookResult, json_mode: bool) -> HookOutput {
    if let Some(provider) = get_editor_provider(client) {
        if provider.kind() == "copilot-chat" {
            let hook_event_name = if native_event_name.is_empty() { HookFormatter::native_event_from_hook_event(&provider, event, parent_info) } else { native_event_name.to_string() };
            return HookOutput { stdout: HookFormatter::format_hook_output(&provider, &hook_event_name, result), stderr: String::new(), exit_code: 0 };
        }
    }
    if json_mode {
        return HookOutput { stdout: format_plain_hook_output(result), stderr: String::new(), exit_code: 0 };
    }
    if !result.allowed {
        return HookOutput { stdout: String::new(), stderr: result.message.clone(), exit_code: 2 };
    }
    HookOutput { stdout: result.message.clone(), stderr: String::new(), exit_code: 0 }
}

/// 🔤️ The native event name the client would use for a neutral event, or the payload's own name.
pub fn resolve_native_event_name(client: &str, event: HookEvent, parent_info: &str, input: Option<&Value>) -> String {
    let named = extract_hook_event_name(input);
    if !named.is_empty() {
        return named;
    }
    match get_editor_provider(client) {
        Some(provider) => HookFormatter::native_event_from_hook_event(&provider, event, parent_info),
        None => String::new(),
    }
}

/// 🎆️ The editor provider for a client slug, as the closed provider enum.
pub fn editor_for_client(client: &str) -> Option<EditorProviders> {
    get_editor_provider(client)
}
//#endregion 🖨️Formatting

//#region 📓️SessionLogging
/// 📍️ One recorded hook invocation in a session log.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionEventEntry {
    #[serde(rename = "event")]
    pub event: Value,
    #[serde(rename = "native", default, skip_serializing_if = "Option::is_none")]
    pub native: Option<SessionNativeEnvelope>,
    #[serde(rename = "response", default, skip_serializing_if = "Option::is_none")]
    pub response: Option<SessionResponse>,
}

/// 🧾️ The native payload, kept only at `full` detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionNativeEnvelope {
    #[serde(rename = "event")]
    pub event: Value,
}

/// 📤️ What the hook answered, kept at `standard` and `full` detail.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde(rename = "blocked", default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "message", default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(rename = "reason", default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

/// 🔸️ The `session.json` record of one agent session.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionMeta {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "uri", default, skip_serializing_if = "String::is_empty")]
    pub uri: String,
    #[serde(rename = "client", default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    #[serde(rename = "second", default, skip_serializing_if = "String::is_empty")]
    pub second: String,
    #[serde(rename = "checkpoint", default, skip_serializing_if = "String::is_empty")]
    pub checkpoint: String,
    #[serde(rename = "contributor", default, skip_serializing_if = "String::is_empty")]
    pub contributor: String,
    #[serde(rename = "transcript", default, skip_serializing_if = "String::is_empty")]
    pub transcript: String,
    #[serde(rename = "events", default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<SessionEventEntry>,
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TicketAgentPlan>,
}

/// 💾️ Where a session log lives. The filesystem implementation is one of several: a caller under
/// test hands in a memory store and never touches a disk.
pub trait SessionStore {
    /// 📥️ The recorded session, when one exists.
    fn read(&self, session_id: &str) -> Option<SessionMeta>;
    /// 📤️ Replaces the recorded session.
    fn write(&mut self, session_id: &str, meta: &SessionMeta) -> HookOutcome<()>;
}

/// 🧠️ An in-memory session store — the deterministic default for tests and dry runs.
#[derive(Debug, Default, Clone)]
pub struct MemorySessionStore {
    sessions: BTreeMap<String, SessionMeta>,
}

impl MemorySessionStore {
    /// 🆕️ An empty store.
    pub fn new() -> MemorySessionStore {
        MemorySessionStore::default()
    }

    /// 📋️ Every recorded session, by id.
    pub fn sessions(&self) -> &BTreeMap<String, SessionMeta> {
        &self.sessions
    }
}

impl SessionStore for MemorySessionStore {
    fn read(&self, session_id: &str) -> Option<SessionMeta> {
        self.sessions.get(session_id).cloned()
    }

    fn write(&mut self, session_id: &str, meta: &SessionMeta) -> HookOutcome<()> {
        self.sessions.insert(session_id.to_string(), meta.clone());
        Ok(())
    }
}

/// 💽️ A session store rooted at one directory, writing `<root>/<session id>/session.json`.
#[derive(Debug, Clone)]
pub struct DirectorySessionStore {
    root: PathBuf,
}

impl DirectorySessionStore {
    /// 🆕️ A store under `root`.
    pub fn new(root: impl Into<PathBuf>) -> DirectorySessionStore {
        DirectorySessionStore { root: root.into() }
    }

    /// 📄️ The file one session is recorded in.
    pub fn path(&self, session_id: &str) -> PathBuf {
        self.root.join(session_id).join(SESSION_FILE_NAME)
    }
}

impl SessionStore for DirectorySessionStore {
    fn read(&self, session_id: &str) -> Option<SessionMeta> {
        let raw = std::fs::read_to_string(self.path(session_id)).ok()?;
        serde_json::from_str(&raw).ok()
    }

    fn write(&mut self, session_id: &str, meta: &SessionMeta) -> HookOutcome<()> {
        let path = self.path(session_id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| HookError::new(format!("cannot create {}: {error}", parent.display())))?;
        }
        let document = serde_json::to_string_pretty(meta).map_err(|error| HookError::new(error.to_string()))?;
        std::fs::write(&path, document).map_err(|error| HookError::new(format!("cannot write {}: {error}", path.display())))
    }
}

/// 📄️ The file name a session log is recorded in.
pub const SESSION_FILE_NAME: &str = "session.json";

/// 📁️ The directory a session log lives in: `.🧬semio/🦑️repo/⚡️cache/🤖️generated/YY/MM/DD/<session>`.
pub fn session_log_dir(repo_root: &Path, year: i32, month: u32, day: u32, session_id: &str) -> PathBuf {
    repo_meta_dir_for_root(repo_root).join("⚡️cache").join("🤖️generated").join(format!("🎆️{:02}", year % 100)).join(format!("🌙️{month:02}")).join(format!("☀️{day:02}")).join(session_id)
}

/// 🪪️ The session id a log is filed under: the payload's, `kiro-<pid>` for the Kiro CLI, otherwise
/// the literal `unknown` — a hook is never dropped just because its client forgot to identify itself.
pub fn resolve_log_session_id(context: &HookContext, kiro_parent_pid: Option<u32>) -> String {
    let session = extract_session_id(context.input.as_ref());
    if !session.is_empty() {
        return session;
    }
    if context.client == "kiro-cli" {
        if let Some(pid) = kiro_parent_pid {
            return format!("kiro-{pid}");
        }
    }
    "unknown".to_string()
}

/// 📓️ Appends one hook invocation to a session log, creating the record on first sight.
///
/// Nothing is written for a `version.*` event or when `[logging] session` is off. `detail` decides
/// how much of each entry survives: `minimal` keeps only the neutral event, `standard` adds the
/// response, `full` adds the native payload as well. `[logging] plan` folds a finished plan update
/// into the session's plan.
pub fn record_session_hook(store: &mut dyn SessionStore, context: &HookContext, result: &HookResult, session_id: &str, logging: &LoggingConfig, environment: &dyn HookEnvironment) -> HookOutcome<Option<SessionMeta>> {
    let Some(event) = context.hook_event() else { return Ok(None) };
    if hook_event_kind(event) == HookKind::Version || !logging.session {
        return Ok(None);
    }
    let input = context.input.as_ref();
    let mut meta = store.read(session_id).unwrap_or_default();
    let second = first_non_empty(&[context.second.clone(), find_nested_string_value(input, &["second"])]);
    let checkpoint = extract_checkpoint_id(input, environment);
    let transcript = extract_transcript(input);
    if meta.id.is_empty() {
        meta.id = session_id.to_string();
        meta.uri = format!("repo://session/{session_id}");
        meta.client = context.client.clone();
        meta.second = second.clone();
        meta.checkpoint = checkpoint.clone();
        meta.contributor = "unknown".to_string();
        meta.transcript = transcript.clone();
    } else {
        if meta.client.is_empty() {
            meta.client = context.client.clone();
        }
        if meta.second.is_empty() {
            meta.second = second.clone();
        }
        if meta.checkpoint.is_empty() {
            meta.checkpoint = checkpoint.clone();
        }
        if meta.transcript.is_empty() {
            meta.transcript = transcript.clone();
        }
    }
    let mut neutral = Map::new();
    neutral.insert("kind".to_string(), Value::String(event.as_str().to_string()));
    neutral.insert("client".to_string(), Value::String(context.client.clone()));
    neutral.insert("session".to_string(), Value::String(session_id.to_string()));
    neutral.insert("second".to_string(), Value::String(second.clone()));
    neutral.insert("checkpoint".to_string(), Value::String(checkpoint));
    neutral.insert("contributor".to_string(), Value::String("unknown".to_string()));
    if !transcript.is_empty() {
        neutral.insert("transcript".to_string(), Value::String(transcript));
    }
    let mut entry = SessionEventEntry { event: Value::Object(neutral), native: None, response: None };
    if logging.include_native() {
        if let Some(input) = input {
            entry.native = Some(SessionNativeEnvelope { event: input.clone() });
        }
    }
    if logging.include_response() {
        entry.response = Some(if result.allowed {
            SessionResponse::default()
        } else {
            SessionResponse { blocked: Some(true), message: result.message.clone(), reason: result.message.clone() }
        });
    }
    meta.events.push(entry);
    if logging.plan && event == HookEvent::AgentToolPlanUpdatingEnded {
        let steps = extract_plan_steps(input, &context.tool_args);
        if !steps.is_empty() {
            let existing = meta.plan.as_ref().map(|plan| plan.steps.clone()).unwrap_or_default();
            meta.plan = Some(TicketAgentPlan { steps: merge_plan_steps(&existing, &steps, &second) });
        }
    }
    store.write(session_id, &meta)?;
    Ok(Some(meta))
}
//#endregion 📓️SessionLogging

//#region 🔖️MicroCommit
/// 📜️ The argv the repo binary hands to bun for a micro-commit subcommand.
pub fn micro_commit_argv(bun: &str, args: &[String]) -> Vec<String> {
    let mut argv = vec![bun.to_string(), "./📜️script.ts".to_string(), "micro-commit".to_string()];
    argv.extend(args.iter().cloned());
    argv
}

/// 🕯️ Every place a bun binary is looked for, in order: the pinned override, the repo's own pin
/// file, a `BUN_INSTALL` tree, the workspace's `node_modules/.bin`, then `PATH`.
pub fn bun_candidates(repo_root: &Path, pinned: Option<&str>, bun_install: Option<&str>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(pinned) = pinned {
        if !pinned.trim().is_empty() {
            candidates.push(PathBuf::from(pinned.trim()));
        }
    }
    candidates.push(repo_meta_dir_for_root(repo_root).join("🐹️compose-micro-commit-bun"));
    if let Some(bun_install) = bun_install {
        if !bun_install.trim().is_empty() {
            for name in ["bun", "bun.exe"] {
                candidates.push(Path::new(bun_install.trim()).join("bin").join(name));
            }
        }
    }
    for relative in ["node_modules/.bin/bun", "node_modules/.bin/bun.exe"] {
        candidates.push(repo_root.join(relative));
    }
    candidates
}

/// 🔖️ Delegates a micro-commit subcommand to the monorepo script through the process runner, so the
/// version control hooks (`post-commit`, `post-checkout`, …) reach exactly one implementation.
pub fn run_micro_commit<R: ProcessRunner>(runner: &R, repo_root: &str, bun: &str, args: &[String]) -> ProcessOutcome {
    runner.run(&ProcessRequest { argv: micro_commit_argv(bun, args), cwd: Some(repo_root.to_string()), stdin: None })
}

/// ♻️ The `micro-commit reset` delegation every post-checkpoint git hook performs.
pub fn run_micro_commit_reset<R: ProcessRunner>(runner: &R, repo_root: &str, bun: &str) -> ProcessOutcome {
    run_micro_commit(runner, repo_root, bun, &["reset".to_string()])
}
//#endregion 🔖️MicroCommit

//#region 🛤️Paths
/// 📄️ The last path element, splitting on both separators the way Go's `filepath.Base` does on
/// Windows; a path that is all separators answers the separator itself, as Go does.
pub fn base_name(path: &str) -> String {
    let trimmed = path.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() {
        return if path.is_empty() { ".".to_string() } else { "/".to_string() };
    }
    match trimmed.rfind(['/', '\\']) {
        Some(index) => trimmed[index + 1..].to_string(),
        None => trimmed.to_string(),
    }
}

/// 🛤️ Normalises a path the way hook payloads carry it: forward separators, no `./` prefix.
pub fn normalize_hook_path(path: &str) -> String {
    let path = path.trim();
    if path.is_empty() {
        return String::new();
    }
    let normalized = path.replace('\\', "/");
    normalized.strip_prefix("./").unwrap_or(&normalized).to_string()
}
//#endregion 🛤️Paths

//#region 💽️SystemPorts
// 💽️ The two ports of `🔌️Ports` bound to the real machine. Everything above stays pure: these are
// the only definitions in this crate that read a checkpoint, walk a codebase or spawn anything.

/// 💻️ The environment that reads the checkpoint identity and message off the real machine.
///
/// Twin of `SystemEnvironment` in `📦️packages/🐹️go/🐹️.go`. `format_file` is deliberately inert:
/// the Go twin delegates to `📜️statutes`' formatter registry, which this crate may not reach
/// without inverting the dependency — the CLI passes a formatting environment when it wants one.
#[derive(Debug, Clone)]
pub struct SystemHookEnvironment {
    repo_root: PathBuf,
}

impl SystemHookEnvironment {
    /// 🆕️ Binds the environment to one repository root.
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self { repo_root: repo_root.into() }
    }
}

impl HookEnvironment for SystemHookEnvironment {
    fn current_checkpoint(&self) -> String {
        let Ok(output) = std::process::Command::new("git").args(["rev-parse", "HEAD"]).current_dir(&self.repo_root).output() else { return String::new() };
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn checkpoint_message(&self, repo_root: &str) -> String {
        let root = if repo_root.is_empty() { self.repo_root.clone() } else { PathBuf::from(repo_root) };
        std::fs::read_to_string(root.join(".git").join("COMMIT_EDITMSG")).map(|body| body.trim().to_string()).unwrap_or_default()
    }
}

/// 🏃️ The resolver that answers through the repository's own test runner and codebase.
///
/// Twin of `WorkspaceTestFileResolver`: the command is narrowed to its test segment, the segment
/// resolved into the files it selects, and each file mapped to the lab id `🗂️codebase` gives it.
pub struct WorkspaceTestFileResolver {
    repo_root: PathBuf,
    snapshot: std::cell::OnceCell<semio_framework_repo_test_runner::FilesystemSnapshot>,
}

impl WorkspaceTestFileResolver {
    /// 🆕️ Binds the resolver to one repository root. The walk is deferred until a hook actually
    /// asks which test files a command selects — most hook events never do.
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self { repo_root: repo_root.into(), snapshot: std::cell::OnceCell::new() }
    }

    /// 🌍️ The repository snapshot, walked once on first use.
    fn snapshot(&self) -> &semio_framework_repo_test_runner::FilesystemSnapshot {
        self.snapshot.get_or_init(|| {
            let ignore = semio_framework_repo_workspace::GitIgnore::compile_file(&self.repo_root.join(".gitignore")).ok();
            semio_framework_repo_test_runner::FilesystemSnapshot::from_root(&self.repo_root, ignore.as_ref())
        })
    }
}

impl TestFileResolver for WorkspaceTestFileResolver {
    fn test_files(&self, command: &str, cwd: &str) -> Vec<String> {
        let command = command.trim();
        if command.is_empty() {
            return Vec::new();
        }
        let root = self.repo_root.to_string_lossy().replace('\\', "/");
        let mut working = if cwd.trim().is_empty() { root } else { cwd.replace('\\', "/") };
        let (segment, extracted) = semio_framework_repo_test_runner::extract_test_segment_from_command(command);
        let mut command = command.to_string();
        if !segment.is_empty() && segment != command {
            command = segment;
            if !extracted.is_empty() {
                working = if semio_framework_repo_test_runner::is_absolute_path(&extracted) { extracted } else { semio_framework_repo_test_runner::join_path(&working, &extracted) };
            }
        }
        semio_framework_repo_test_runner::resolve_test_files_from_command(self.snapshot(), &command, &working)
    }

    fn lab_ids(&self, file_paths: &[String]) -> Vec<String> {
        let codebase = semio_framework_repo_codebase::Codebase::new(self.repo_root.clone());
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut found = Vec::new();
        for path in file_paths {
            let id = codebase.build_file_id(path);
            if id.is_empty() || !seen.insert(id.clone()) {
                continue;
            }
            found.push(id);
        }
        found
    }
}
//#endregion 💽️SystemPorts

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
