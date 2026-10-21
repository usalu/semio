//! 🎯️ Hook model: the error, event kind, context and port vocabulary plus the hook path helpers.
//!
//! Holds the `🔖️HookError`, `🏷️HookKind`, `🎯️HookContext`, `🔌️Ports`, `🛤️Paths` regions of the crate root; the root re-exports its items.

use super::*;

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

    /// 🧰️ Sets the tool name and raw argument string the agent client reported.
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

    /// 📨️ Attaches the native payload the agent client wrote to stdin.
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
