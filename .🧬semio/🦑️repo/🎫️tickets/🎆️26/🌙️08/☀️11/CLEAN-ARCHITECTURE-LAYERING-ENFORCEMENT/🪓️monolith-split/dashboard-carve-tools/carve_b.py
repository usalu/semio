"""🪚 Stage B of the dashboard carve: labels, preferences, terminal, cli and the facade. Usage: python3 carve_b.py <dashboard-dir>"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
from lib import rd, wr, sub, subs, take, region

D = sys.argv[1]
J = lambda *a: os.path.join(D, *a)
RG = n.e(0x1F516)

def rname(name):
    return RG + name

def residual(path, allow=()):
    text = rd(path)
    left = [line for line in text.split("\n") if re.search(r"\bcrate::", line) and not any(token in line for token in allow)]
    assert not left, (path, left[:3])

def rewrite(path, pairs, check=True, allow=()):
    text = rd(path)
    for old, new, *rest in pairs:
        count = rest[0] if rest else None
        found = text.count(old)
        assert found >= 1 and (count is None or found == count), f"{path}: {old[:80]!r} found {found}, expected {count}"
        text = text.replace(old, new)
    wr(path, text)
    if check:
        residual(path, allow)

# ---------------------------------------------------------------- labels
LAB = J(n.TERMINAL, n.LABELS, n.RS)
rewrite(LAB, [("crate::ipc::ErrorCode", "dashboard_wire::ipc::ErrorCode", 2), ("crate::daemon::client::Skew", "dashboard_wire::client::Skew", 1)])
rewrite(J(n.TERMINAL, n.LABELS, n.TESTS, n.UNIT, n.RS), [
    ("crate::preferences::", "dashboard_preferences::", 2),
    ("crate::args::parse", "dashboard_core::args::parse", 1),
    ("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 1),
])

# ---------------------------------------------------------------- preferences
PREF = J(n.PREFS, n.RS)
rewrite(PREF, [("crate::args::ParsedArgs", "dashboard_core::args::ParsedArgs", 6), ("crate::registry::Parameter", "dashboard_registry::Parameter", 1)])
rewrite(J(n.PREFS, n.TESTS, n.UNIT, n.RS), [
    ("crate::args::parse", "dashboard_core::args::parse", 5),
    ("crate::registry::", "dashboard_registry::", 2),
    ("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 1),
])
rewrite(J(n.PREFS, n.KEYMAP, n.TESTS, n.UNIT, n.RS), [("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 1)])

# ---------------------------------------------------------------- terminal
T = lambda *a: J(n.TERMINAL, *a)
TERM_ROOT = T(n.RS)
rewrite(TERM_ROOT, [
    (f"#[path = \"{n.LABELS}/{n.RS}\"]\npub mod labels;\n\n", ""),
    ("labels::labels(", "dashboard_labels::labels(", 2),
    ("crate::args::ParsedArgs", "dashboard_core::args::ParsedArgs", 2),
    ("crate::preferences::", "dashboard_preferences::", None),
    ("crate::ipc::", "dashboard_wire::ipc::", 2),
    ("crate::inventory::", "dashboard_registry::inventory::", 1),
])
rewrite(T(n.CONTROLS, n.RS), [
    ("crate::preferences::", "dashboard_preferences::", None),
    ("crate::inventory::", "dashboard_registry::inventory::", None),
])
rewrite(T(n.PANES, n.RS), [
    ("use crate::ipc::SessionStatus;", "use dashboard_wire::ipc::SessionStatus;"),
    ("use crate::registry;\n", ""),
    ("registry::tab_text(", "dashboard_registry::tab_text(", 1),
    ("crate::ipc::SessionInfo", "dashboard_wire::ipc::SessionInfo", 1),
    ("crate::preferences::", "dashboard_preferences::", None),
])
rewrite(T(n.SESSIONS, n.RS), [
    ("use crate::daemon::client::{Connection, Message};", "use dashboard_wire::client::{Connection, Message};"),
    ("use crate::inventory::Update;", "use dashboard_registry::inventory::Update;"),
    ("use crate::ipc::{ClientMsg,", "use dashboard_wire::ipc::{ClientMsg,"),
    ("crate::registry::GRAPH_REUSE", "dashboard_registry::GRAPH_REUSE", 1),
    ("crate::registry::Registry", "dashboard_registry::Registry", 1),
    ("crate::preferences::", "dashboard_preferences::", None),
    ("crate::ipc::ErrorCode", "dashboard_wire::ipc::ErrorCode", 2),
    ("crate::daemon::supervisor::spawn_daemon", "dashboard_daemon::lifecycle::spawn_daemon", 1),
    ("crate::daemon::client::Skew", "dashboard_wire::client::Skew", 1),
    ("crate::inventory::", "dashboard_registry::inventory::", None),
])
rewrite(T(n.LAUNCHER, n.RS), [
    ("use super::labels::{labels, DashboardLabels};", "use dashboard_labels::{labels, DashboardLabels};"),
    ("use crate::preferences::Preferences;\nuse crate::registry::{split_arguments, verb_text, Entry, Launch, Parameter, ParameterKind, Registry, Request};\n",
     "use dashboard_launch::Launch;\nuse dashboard_preferences::Preferences;\nuse dashboard_registry::{split_arguments, verb_text, Entry, Parameter, ParameterKind, Registry, Request};\n"),
    ("crate::registry::window_title", "dashboard_registry::window_title", 3),
])
rewrite(T(n.WINDOWS, n.RS), [
    ("use super::labels::{labels, DashboardLabels};\nuse super::launcher::LauncherState;", "use super::launcher::LauncherState;"),
    ("use crate::inventory::{Job, Phase};\nuse crate::ipc::{ClientMsg, SessionInfo, SessionStatus};\nuse crate::preferences::keymap::Keymap;\nuse crate::preferences::{Change, Preferences};\nuse crate::registry::{self, Registry, TaskLabel};\n",
     "use dashboard_labels::{labels, DashboardLabels};\nuse dashboard_launch::TaskLabel;\nuse dashboard_preferences::keymap::Keymap;\nuse dashboard_preferences::{Change, Preferences};\nuse dashboard_registry::inventory::{Job, Phase};\nuse dashboard_registry::Registry;\nuse dashboard_wire::ipc::{ClientMsg, SessionInfo, SessionStatus};\n"),
    ("registry::window_title(", "dashboard_registry::window_title(", 1),
    ("registry::tab_texts(", "dashboard_registry::tab_texts(", 1),
    ("registry::TAB_CELLS", "dashboard_registry::TAB_CELLS", 1),
])
TU = T(n.TESTS, n.UNIT, n.RS)
rewrite(TU, [
    ("use crate::daemon::client::Message;\nuse crate::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};\nuse crate::preferences::{Change, Preferences};\nuse crate::registry::{Facts, Registry, TaskLabel};\n",
     "use dashboard_launch::TaskLabel;\nuse dashboard_preferences::{Change, Preferences};\nuse dashboard_registry::{Facts, Registry};\nuse dashboard_wire::client::Message;\nuse dashboard_wire::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};\n"),
    ("crate::preferences::", "dashboard_preferences::", None),
    ("crate::ipc::", "dashboard_wire::ipc::", None),
    ("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 3),
])
rewrite(T(n.LAUNCHER, n.TESTS, n.UNIT, n.RS), [
    ("use crate::registry::Facts;", "use dashboard_registry::Facts;"),
    ("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 1),
])

# ---------------------------------------------------------------- cli
CLI = J(n.CLI, n.RS)
rewrite(CLI, [
    ("use crate::daemon::client::Connection;\nuse crate::daemon::control::{self as daemon_control, Action, Followed, RunError, Wait};\nuse crate::daemon::ipc::{self, ClientMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup};\n",
     "use dashboard_daemon::control::{self as daemon_control, Action, Followed, RunError, Wait};\nuse dashboard_wire::client::Connection;\nuse dashboard_wire::ipc::{self, ClientMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup};\n"),
    ("use crate::registry::{ParameterKind, Registry, Request, RunPolicy};\nuse crate::repo_domain::RepoImplementation;\n",
     "use dashboard_registry::{ParameterKind, Registry, Request, RunPolicy};\nuse dashboard_repo_domain::RepoImplementation;\n"),
    ("crate::inventory::", "dashboard_registry::inventory::", 3),
    ("daemon_control::connect_or_start(", "dashboard_daemon::lifecycle::connect_or_start(", 1),
])
rewrite(J(n.CLI, n.TESTS, n.UNIT, n.RS), [
    ("crate::registry::Ready", "dashboard_launch::Ready", 2),
    ("crate::registry::LaunchProcess", "dashboard_launch::LaunchProcess", 1),
    ("crate::registry::Launch", "dashboard_launch::Launch", 2),
    ("crate::registry::Stop", "dashboard_launch::Stop", 2),
    ("ipc::Ready", "dashboard_launch::Ready", 1),
    ("ipc::TaskLabel", "dashboard_launch::TaskLabel", 1),
    ("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 1),
])

# ---------------------------------------------------------------- facade modules
rewrite(J(n.PLUGIN_REGISTRY, n.RS), [("use crate::catalog::configured_catalog_root;", "use dashboard_core::catalog::configured_catalog_root;")], allow=("crate::proc",))
rewrite(J(n.PLAYGROUND, n.RS), [("use crate::args::ParsedArgs;\nuse crate::catalog::{", "use dashboard_core::args::ParsedArgs;\nuse dashboard_core::catalog::{")])
rewrite(J(n.PLAYGROUND, n.TESTS, n.UNIT, n.RS), [("use crate::catalog::{PlaygroundEntry, Ports};", "use dashboard_core::catalog::{PlaygroundEntry, Ports};")])

# ---------------------------------------------------------------- facade glue
GLUE = J(n.PKG, n.RUST, n.RS)
g = rd(GLUE)
mounts_old = [
    ("../../" + n.PLUGIN_REGISTRY, "plugin_registry", True),
    ("../../" + n.DAEMON, "daemon", False),
    ("../../" + n.USAGE, "usage", True),
    ("../../" + n.PLAYGROUND, "playground_catalog", True),
    ("../../" + n.COMMAND_TREE, "command_tree", False),
    ("../../" + n.PREFS, "preferences", False),
    ("../../" + n.INVENTORY, "inventory", False),
    ("../../" + n.REGISTRY, "registry", False),
    ("../../" + n.REPO_DOMAIN, "repo_domain", False),
    ("../../" + n.CLI, "cli", False),
    ("../../" + n.TERMINAL, "terminal", False),
]
for path, module, keep in mounts_old:
    block = f"#[path = \"{path}/{n.RS}\"]\npub mod {module};\n\n"
    assert g.count(block) == 1, (module, g.count(block))
    if not keep:
        g = g.replace(block, "")
reexport = "/// ✉️ The framed dashboard transport, owned by the daemon sub-folder.\npub use daemon::ipc;\n\n"
assert g.count(reexport) == 1
g = g.replace(reexport, "")
daemon_mount = f"#[path = \"../../{n.DAEMON}/{n.COMMAND}/{n.RS}\"]\npub mod daemon;\n\n"
anchor = f"#[path = \"../../{n.PLUGIN_REGISTRY}/{n.RS}\"]\npub mod plugin_registry;\n\n"
assert g.count(anchor) == 1
g = g.replace(anchor, anchor + daemon_mount)
header_old = g[:g.index("#[path = ")]
header_new = (
    "//! 🎛️ The semio terminal dashboard: the wizard command tree, the pseudo-terminal windows it runs\n"
    "//! commands in, the session daemon behind them, and the repo-domain leaves the Rust domain crates\n"
    "//! answer in process.\n"
    "//!\n"
    "//! The dashboard is a family of producer crates, one per layer, each rooted at a domain sub-folder of\n"
    "//! `🔨️modules/🎛️dashboard` so the taxonomy tree states what the dashboard is made of: the core, the launch contract, the repo\n"
    "//! domain, the command registry with its inventory and command tree, the daemon wire, the daemon, the labels, the preferences,\n"
    "//! the terminal view and the command line. This crate owns the `semio` binary: `🚪️entrypoint` hands its argv to [`run`], which\n"
    "//! dispatches each verb to its crate or command module.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔣️.json\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🚪️entrypoint/🦀️.rs\n"
    "\n"
    "use dashboard_core::args::{parse, ParsedArgs};\n\n"
)
g = g.replace(header_old, header_new)
g = re.sub(r"\n{3,}", "\n\n", g)
wr(GLUE, g)
rewrite(GLUE, [
    ('"dashboard" => terminal::run_with(&root, &parsed),', '"dashboard" => dashboard_terminal::run_with(&root, &parsed),', 1),
    ('"preferences" => preferences::run(&root, &parsed),', '"preferences" => dashboard_preferences::run(&root, &parsed),', 1),
    ('"repo-view" => repo_domain::run_action(&root, &parsed),', '"repo-view" => dashboard_repo_domain::run_action(&root, &parsed),', 1),
    ('"command-tree" => command_tree::run(&root, &parsed),', '"command-tree" => dashboard_registry::command_tree::run(&root, &parsed),', 1),
    ('"commands" => cli::commands(&root, rest),', '"commands" => dashboard_cli::commands(&root, rest),', 1),
    ('"run" => cli::run(&root, rest),', '"run" => dashboard_cli::run(&root, rest),', 1),
    ('"tasks" => cli::tasks(&root, rest),', '"tasks" => dashboard_cli::tasks(&root, rest),', 1),
    ('"logs" => cli::logs(&root, rest),', '"logs" => dashboard_cli::logs(&root, rest),', 1),
    ('"stop" | "restart" | "kill" => cli::act(&root, &parsed.verb, rest),', '"stop" | "restart" | "kill" => dashboard_cli::act(&root, &parsed.verb, rest),', 1),
    ('"open" => cli::open(&root, rest),', '"open" => dashboard_cli::open(&root, rest),', 1),
    ("fn invocation(argv: &[String]) -> args::ParsedArgs {", "fn invocation(argv: &[String]) -> ParsedArgs {", 1),
    ("return args::parse(&std::iter::once", "return parse(&std::iter::once", 1),
    ("    args::parse(argv)\n", "    parse(argv)\n", 1),
    ("let text = crate::terminal::labels::labels(crate::preferences::cli_locale(cwd, &crate::args::ParsedArgs::default()));", "let text = dashboard_labels::labels(dashboard_preferences::cli_locale(cwd, &dashboard_core::args::ParsedArgs::default()));", 1),
], check=True)
print("stage B done")
