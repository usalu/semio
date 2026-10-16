"""Split specifications of the six repo crate roots (ranges are 1-based inclusive lines of the HEAD file in orig/)."""

def C(folder, emoji, mod, title, doc, ranges, macro_use=False, fixes=(), imports=()):
    return dict(folder=folder, emoji=emoji, mod=mod, title=title, doc=doc, ranges=ranges, macro_use=macro_use, fixes=list(fixes), imports=list(imports))


SPEC = {
    "tree": dict(anchor=41, children=[
        C("🧱️tree-model", "🧱️", "tree_model", "Tree model", "vocabulary constants, the four ports, the loaded records and the node helpers every builder shares.", [(43, 809)]),
        C("🩻️monorepo-tree", "🩻️", "monorepo_tree", "Monorepo tree", "assembly of the technology, bundle, folder, file, section and definition tree plus the goal, draft, policy, contributor, checkpoint and session branches.", [(811, 1258)]),
        C("🎯️goal-tree", "🎯️", "goal_tree", "Goal tree", "the goal and ticket tree with its open-work counters and text rendering.", [(1260, 1509)]),
        C("⚖️statute-tree", "⚖️", "statute_tree", "Statute tree", "the statute and territory trees and the declared statute catalog.", [(1511, 1727)]),
        C("🧹️sort-filter-search", "🧹️", "sort_filter_search", "Sorting, filtering and search", "deterministic child order, kind filtering and the in-memory fuzzy search.", [(1729, 1981)]),
        C("🎨️rendering", "🎨️", "rendering", "Rendering", "text and markdown tree output, the Mermaid treemap and the lines-of-code Mermaid views.", [(1983, 2316)]),
        C("📌️cache", "📌️", "cache", "Cache", "the content-digest tree cache envelope and its canonical encoding.", [(2318, 2382)]),
    ]),
    "test-runner": dict(anchor=24, children=[
        C("🧮️paths-and-identity", "🧮️", "paths_and_identity", "Paths and identity", "the pending identity ports and the pure path arithmetic every other layer uses.", [(26, 202)]),
        C("🌲️snapshot-and-scope", "🌲️", "snapshot_and_scope", "Snapshot and scope", "the in-memory filesystem snapshot and the resolution of a scope id to a test scope.", [(204, 534)]),
        C("🗺️planning", "🗺️", "planning", "Planning", "the pure translation of a test scope into an ordered invocation plan.", [(536, 979)]),
        C("⚙️execution-and-parsing", "⚙️", "execution_and_parsing", "Execution and parsing", "the process port with its replay runner, plan execution and the per-runner transcript parsers.", [(981, 1492)]),
        C("🔎️resolution", "🔎️", "resolution", "Resolution", "command classification and the resolution of a shell command to the test files it would run.", [(1494, 1967)]),
        C("💽️real-world", "💽️", "real_world", "Real world", "the only layer that touches the machine: the filesystem walk and the spawning process runner.", [(1995, 2153)]),
    ]),
    "hooks": dict(anchor=46, children=[
        C("🎯️hook-model", "🎯️", "hook_model", "Hook model", "the error, event kind, context and port vocabulary plus the hook path helpers.", [(48, 227), (2020, 2043)]),
        C("🧭️payload-reading", "🧭️", "payload_reading", "Payload reading", "normalisation of native IDE hook payloads into one neutral shape.", [(229, 700)]),
        C("🛡️tool-policy", "🛡️", "tool_policy", "Tool policy", "tool classification, command segmentation and the tool blocking policy.", [(702, 1241)]),
        C("🗺️plan-steps", "🗺️", "plan_steps", "Plan steps", "extraction and merging of agent plan steps.", [(1243, 1355)]),
        C("🔶️dispatch-and-formatting", "🔶️", "dispatch_and_formatting", "Dispatch and formatting", "the per-event hook dispatch and the hook result formatting.", [(1357, 1753)]),
        C("📓️session-logging", "📓️", "session_logging", "Session logging", "the session event log, its stores and the hook recorder.", [(1755, 1975)]),
        C("💽️system-ports", "💽️", "system_ports", "System ports", "micro-commit delegation and the system-backed environment and test file resolver.", [(1977, 2018), (2045, 2136)]),
    ]),
    "mcp": dict(anchor=92, children=[
        C("🔏️digest-and-json", "🔏️", "digest_and_json", "Digest and JSON", "the hand-rolled SHA-256 and the ordered JSON value with Go-compatible encoding.", [(15, 88), (93, 236)]),
        C("📨️protocol", "📨️", "protocol", "Protocol", "the bounded JSON-RPC contract: codes, identifiers, errors, capabilities and wire shapes.", [(238, 645)]),
        C("📡️event-log", "📡️", "event_log", "Event log", "the hash-chained event log and its replay.", [(647, 909)]),
        C("🔐️session", "🔐️", "session", "Session", "limits, the server, the session state machine and its handshake.", [(911, 1293)]),
        C("🚦️routing", "🚦️", "routing", "Routing", "request decoding, method routing and the notification handlers.", [(1295, 1644)]),
        C("🗄️repository", "🗄️", "repository", "Repository", "the per-client profile, the repository port and the tool, resource and prompt surface.", [(1646, 2000)], fixes=[('include_str!("../../', 'include_str!("../../../')]),
        C("🚀️entrypoint", "🚀️", "entrypoint", "Entrypoint", "the stdio transport loop and the public run entry points.", [(2002, 2126)]),
    ]),
    "providers": dict(anchor=15, children=[
        C("📐️provider-model", "📐️", "provider_model", "Provider model", "the provider error, the pending model vocabulary and the MCP client kind table.", [(17, 346)]),
        C("🏃️process-runner", "🏃️", "process_runner", "Process runner", "the process port with its system and recorded runners.", [(348, 514)]),
        C("🔭️provider-interfaces", "🔭️", "provider_interfaces", "Provider interfaces", "the management, version control, sandbox and editor provider traits plus the downward ports.", [(516, 699)], macro_use=True),
        C("🐙️github-management", "🐙️", "github_management", "GitHub management", "the gh-backed and null management providers.", [(701, 1190)]),
        C("🌿️version-control-and-sandbox", "🌿️", "version_control_and_sandbox", "Version control and sandbox", "the git version control provider and the devcontainer sandbox provider.", [(1192, 1368)]),
        C("🎆️editor-providers", "🎆️", "editor_providers", "Editor providers", "native event tables, hook output formatting and the eight editor providers.", [(1370, 1642)]),
        C("🎖️registry-and-git-hooks", "🎖️", "registry_and_git_hooks", "Registry and git hooks", "the provider registry and the repository-managed git hook files.", [(1644, 1748)]),
    ]),
    "languages": dict(anchor=10, children=[
        C("🪪️identity-pending", "🪪️", "identity_pending", "Identity pending", "the emoji rune table and entity emoji extraction awaiting the identity crate.", [(12, 90)]),
        C("🗂️language-table", "🗂️", "language_table", "Language table", "the embedded language table and its lookups.", [(111, 366)], fixes=[('include_str!("../../', 'include_str!("../../../')]),
        C("🧩️pattern-matcher", "🧩️", "pattern_matcher", "Pattern matcher", "the backtracking matcher for the declarative token patterns.", [(368, 667)]),
        C("📑️section-parsing", "📑️", "section_parsing", "Section parsing", "marker, markdown, JSON and code section parsing.", [(669, 1036)]),
        C("🧷️definition-parsing", "🧷️", "definition_parsing", "Definition parsing", "definition keyword extraction, range detection and orphan definitions.", [(1038, 1344)]),
        C("🧭️scopes-and-headers", "🧭️", "scopes_and_headers", "Scopes and headers", "scope identifiers, line-based parsing and the file header format.", [(1346, 1702)]),
    ]),
}
