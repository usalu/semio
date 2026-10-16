"""🪚 Stage A of the dashboard carve: core, launch contract, registry domain, repo domain and the daemon wire. Usage: python3 carve_a.py <dashboard-dir>"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
from lib import rd, wr, sub, subs, take, region

D = sys.argv[1]
J = lambda *a: os.path.join(D, *a)
RG = n.e(0x1F516)

def rname(name):
    return RG + name

def extract_fn(path, marker, remove=True):
    """Cuts a top-level fn (with its attribute and doc lines) whose declaration line starts with `marker`; ends at the first line equal to `}`."""
    lines = rd(path).split("\n")
    at = [i for i, line in enumerate(lines) if line.startswith(marker)]
    assert len(at) == 1, f"{path}: {marker!r} found {len(at)}"
    first = at[0]
    while first > 0 and (lines[first - 1].startswith("#[") or lines[first - 1].startswith("///")):
        first -= 1
    last = next(i for i in range(at[0], len(lines)) if lines[i] == "}")
    cut = lines[first:last + 1]
    if remove:
        del lines[first:last + 1]
        if first < len(lines) and first > 0 and lines[first] == "" and lines[first - 1] == "":
            del lines[first]
        wr(path, "\n".join(lines))
    return "\n".join(cut) + "\n"

GLUE = J(n.PKG, n.RUST, n.RS)
GLUE_TESTS = J(n.TESTS, n.UNIT, n.RS)
REG_ROOT = J(n.REGISTRY, n.RS)
REG_TESTS = J(n.REGISTRY, n.TESTS, n.UNIT, n.RS)
IPC = J(n.DAEMON, n.IPC, n.RS)
SUP = J(n.DAEMON, n.SUPERVISOR, n.RS)
CTL = J(n.DAEMON, n.CONTROL, n.RS)
DAEMON_ROOT = J(n.DAEMON, n.RS)
DAEMON_UNIT = J(n.DAEMON, n.TESTS, n.UNIT, n.RS)
CORE_ROOT = J(n.CORE, n.RS)
LAUNCH_ROOT = J(n.LAUNCH, n.RS)
WIRE_ROOT = J(n.DAEMON, n.WIRE, n.RS)

# ================================================================ core
core_regions = [region(GLUE, rname(name)) for name in ["Args", "Catalog", "Options", "EnvContract"]]
core_regions[1] = core_regions[1].replace("    pub(crate) fn configured_catalog_root()", "    pub fn configured_catalog_root()")
assert "pub(crate)" not in core_regions[1]
env = core_regions[3]
assert env.count("crate::registry::RUNNER_ENV.iter()") == 1
env = env.replace("crate::registry::RUNNER_ENV.iter()", "RUNNER_ENV.iter()")
anchor = "    /// 🚪️ Resolves the dev-server port"
assert env.count(anchor) == 1
env = env.replace(anchor, "    /// 🖥️ The variables that keep terminal rendering and process lifecycle with the dashboard.\n    pub const RUNNER_ENV: &[(&str, &str)] = &[(\"SEMIO_BUILD_TERMINAL\", \"false\")];\n\n" + anchor)
core_regions[3] = env
hash_region = region(REG_ROOT, rname("Hash"))

wr(CORE_ROOT, (
    "//! 🧱️ The dashboard core: the invocation split into verb, segments and flags, the plugin and playground catalog it reads, the\n"
    "//! environment a playground session boots with and the content hash every persistent name of the dashboard derives from.\n"
    "//! Every other crate of the dashboard may depend on these; nothing here depends on any of them.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs\n"
    "\n"
    + "\n".join(core_regions) + "\n" + hash_region + "\n"
    "#[cfg(any(test, feature = \"testkit\"))]\n"
    f"#[path = \"{n.TESTS}/{n.TESTKIT}/{n.RS}\"]\n"
    "pub mod testkit;\n"
    "\n"
    f"// #region {rname('Tests')}\n"
    "#[cfg(test)]\n"
    f"#[path = \"{n.TESTS}/{n.UNIT}/{n.RS}\"]\n"
    "mod tests;\n"
    f"// #endregion {rname('Tests')}\n"
))

assert_proved = extract_fn(GLUE_TESTS, "pub(crate) fn assert_proved").replace("pub(crate) fn assert_proved", "pub fn assert_proved")
wr(J(n.CORE, n.TESTS, n.TESTKIT, n.RS), "//! 🥒️ The test kit of the dashboard crates: the proof that a feature file and the tests that name its scenarios agree.\n\n" + assert_proved)

core_tests = [extract_fn(GLUE_TESTS, "fn " + name) for name in ["args_split_verb_segments_and_flags", "env_contract_sets_locks_only_for_individual", "resolve_port_prefers_explicit_then_catalog_then_fallback"]]
consumer = take(GLUE_TESTS, "//#region " + rname("CatalogConsumer"), "//#endregion " + rname("CatalogConsumer"))
xxh3_test = extract_fn(REG_TESTS, "fn the_xxh3_hash_matches_the_reference_vectors")
wr(J(n.CORE, n.TESTS, n.UNIT, n.RS), (
    "use super::xxh3_64;\n"
    "use crate::args::parse;\n"
    "use crate::catalog::{PlaygroundEntry, Ports};\n"
    "use crate::env_contract::{build_dev_env, resolve_port, DevOptions};\n"
    "use crate::options::Lock;\n\n"
    + "\n".join(core_tests) + "\n" + xxh3_test + "\n" + consumer
))

# facade unit tests: the header and the daemon verb test
unknown = extract_fn(DAEMON_UNIT, "fn unknown_subcommand_returns_usage_without_side_effects")
unknown = unknown.replace("assert_eq!(run(Path::new(\".\"), &parsed), 1);", "assert_eq!(crate::daemon::run(Path::new(\".\"), &parsed), 1);")
assert "crate::daemon::run" in unknown
gtext = rd(GLUE_TESTS)
old_head = ("use crate::args::parse;\n"
            "use crate::catalog::{playgrounds_json_text, PlaygroundEntry, Ports};\n"
            "use crate::env_contract::{build_dev_env, resolve_port, DevOptions};\n"
            "use crate::options::Lock;\n")
assert gtext.startswith(old_head)
gtext = "use dashboard_core::args::{parse, ParsedArgs};\nuse std::collections::HashMap;\nuse std::path::Path;\n" + gtext[len(old_head):]
gtext = gtext.rstrip("\n") + "\n\n" + unknown
wr(GLUE_TESTS, gtext)

# ================================================================ launch contract
stop_block = take(REG_ROOT, "/// 🛑️ Whether stopping one member of a compound stops the others.", "pub enum Stop")
process_block = take(REG_ROOT, "/// ▶️ One process of a resolved launch.", "}")
launch_block = take(REG_ROOT, "/// 🚀️ `(id, chosen parameters, extra arguments)` resolved", "}")
label_block = take(IPC, "/// 🏷️ What a running task is called;", "}")
ready_block = take(IPC, "/// 🟢 Ready when the output shows", "}")
wr(LAUNCH_ROOT, (
    "//! 🧾️ The launch contract: what a resolved launch is made of and what travels with a running task — the processes in start order,\n"
    "//! the services that must be ready first, how a group stops, what a task is called and the address its output announces.\n"
    "//! The registry produces these, the daemon wire carries them; neither depends on the other.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs\n"
    "\n"
    "use serde::{Deserialize, Serialize};\n"
    "use std::path::PathBuf;\n\n"
    + stop_block + "\n" + label_block + "\n" + ready_block + "\n" + process_block + "\n" + launch_block
))

# ================================================================ registry domain
subs(REG_ROOT, [
    ("use crate::repo_domain::{RepoAction, RepoImplementation};\nuse serde::{Deserialize, Serialize};",
     "use dashboard_core::env_contract::RUNNER_ENV;\nuse dashboard_core::xxh3_64;\nuse dashboard_repo_domain::{RepoAction, RepoImplementation};\nuse serde::{Deserialize, Serialize};"),
    ("use ui_tui::tui::text::{elide_end, elide_middle};\n\n// #region " + rname("Wire") + "\n/// 🏷️ The wire types of a launch, which the daemon schema owns and the registry produces: what a running task is\n/// called, and the address its output announces when it is ready.\n///\n/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs\npub use crate::daemon::ipc::{Ready, TaskLabel};\n",
     "use ui_tui::tui::text::{elide_end, elide_middle};\n\n"
     f"#[path = \"../{n.INVENTORY}/{n.RS}\"]\npub mod inventory;\n\n"
     f"#[path = \"../{n.COMMAND_TREE}/{n.RS}\"]\npub mod command_tree;\n\n"
     "// #region " + rname("Wire") + "\n/// 🏷️ The launch types the registry produces and the daemon wire carries, which the launch contract owns: what a running task is\n/// called, the address its output announces when it is ready and the processes a resolved launch starts.\n///\n/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧾️launch/🦀️.rs\nuse dashboard_launch::{Launch, LaunchProcess, Ready, Stop, TaskLabel};\n"),
    ("/// 🖥️ The variables that keep terminal rendering and process lifecycle with the dashboard.\npub const RUNNER_ENV: &[(&str, &str)] = &[(\"SEMIO_BUILD_TERMINAL\", \"false\")];\n", ""),
    ("crate::repo_domain::", "dashboard_repo_domain::", 5),
    ("crate::options::Lock", "dashboard_core::options::Lock", 2),
    ("crate::catalog::PlaygroundEntry", "dashboard_core::catalog::PlaygroundEntry"),
    ("crate::catalog::Ports", "dashboard_core::catalog::Ports"),
    ("crate::env_contract::DevOptions", "dashboard_core::env_contract::DevOptions"),
    ("crate::env_contract::build_dev_env", "dashboard_core::env_contract::build_dev_env"),
])
assert "crate::" not in rd(REG_ROOT).replace("pub(crate)", ""), [l for l in rd(REG_ROOT).split("\n") if "crate::" in l]

REG_TESTS_T = rd(REG_TESTS)
assert REG_TESTS_T.count("crate::repo_domain::go_binary_path") == 1
wr(REG_TESTS, REG_TESTS_T.replace("crate::repo_domain::go_binary_path", "dashboard_repo_domain::go_binary_path"))

INV = J(n.INVENTORY, n.RS)
subs(INV, [
    ("use crate::registry::{graph_file,", "use crate::{graph_file,"),
    ("crate::catalog::playgrounds_json_text_from_catalog", "dashboard_core::catalog::playgrounds_json_text_from_catalog"),
    ("crate::repo_domain::ticket_index", "dashboard_repo_domain::ticket_index"),
    ("crate::catalog::configured_catalog_root", "dashboard_core::catalog::configured_catalog_root", 4),
    ("pub use crate::registry::GRAPH_REUSE;", "pub use crate::GRAPH_REUSE;"),
])
CT = J(n.COMMAND_TREE, n.RS)
subs(CT, [
    ("use crate::registry::{Entry, Kind, Registry};", "use crate::{Entry, Kind, Registry};"),
    ("crate::registry::verb_rank", "crate::verb_rank", 2),
    ("crate::repo_domain::action_key", "dashboard_repo_domain::action_key"),
    ("crate::args::ParsedArgs", "dashboard_core::args::ParsedArgs"),
])
CTT = J(n.COMMAND_TREE, n.TESTS, n.UNIT, n.RS)
subs(CTT, [
    ("use crate::registry::{Request, RunPolicy};", "use crate::{Request, RunPolicy};"),
    ("crate::registry::graph_file", "crate::graph_file", 2),
    ("crate::repo_domain::ANALYZE_SCOPES", "dashboard_repo_domain::ANALYZE_SCOPES"),
])

# ================================================================ repo domain
RD = J(n.REPO_DOMAIN, n.RS)
sub(RD, "crate::args::ParsedArgs", "dashboard_core::args::ParsedArgs")
RDT = J(n.REPO_DOMAIN, n.TESTS, n.UNIT, n.RS)
grammar = extract_fn(RDT, "fn every_action_has_a_distinct_key_the_registry_grammar_accepts")
grammar = grammar.replace("action_key", "dashboard_repo_domain::action_key", 0)
grammar = grammar.replace("crate::registry::CommandId", "CommandId")
reg_tests_text = rd(REG_TESTS)
grammar = grammar.replace("actions.iter().map(action_key)", "actions.iter().map(dashboard_repo_domain::action_key)")
grammar = grammar.replace("let actions = [", "let actions = [")
wr(REG_TESTS, reg_tests_text.rstrip("\n") + "\n\n" + grammar)
subs(RDT, [
    ("crate::tests::assert_proved(include_str!(\"../../../🧪️tests/🏛️repo-domain/🥒️.feature\"), &[include_str!(\"🦀️.rs\")]",
     "dashboard_core::testkit::assert_proved(include_str!(\"../../../🧪️tests/🏛️repo-domain/🥒️.feature\"), &[include_str!(\"🦀️.rs\"), include_str!(\"../../../🎮️registry/🧪️tests/🔬️unit/🦀️.rs\")]"),
])

# ================================================================ daemon wire
subs(IPC, [
    ("use crate::registry::{Launch, LaunchProcess, Stop};", "use dashboard_launch::{Launch, LaunchProcess, Ready, Stop, TaskLabel};"),
    ("crate::registry::xxh3_64(bytes)", "dashboard_core::xxh3_64(bytes)"),
    ("/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs\npub fn stable_hash", "/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧱️core/🦀️.rs\npub fn stable_hash"),
])
READY = J(n.DAEMON, n.READY, n.RS)
subs(READY, [("use super::ipc::Ready;", "use dashboard_launch::Ready;"), ("pub(super) mod http;", "pub mod http;")])
HTTP = J(n.DAEMON, n.READY, n.HTTP, n.RS)
sub(HTTP, "pub(crate) fn probe", "pub fn probe")
REPLAY = J(n.DAEMON, n.REPLAY, n.RS)
sub(REPLAY, "use super::ipc::Ready;", "use dashboard_launch::Ready;")
JOURNAL = J(n.DAEMON, n.JOURNAL, n.RS)
sub(JOURNAL, "super::supervisor::WorkspaceLease::acquire", "super::lease::WorkspaceLease::acquire")


def tail_from(path, marker):
    lines = rd(path).split("\n")
    at = [i for i, line in enumerate(lines) if line.startswith(marker)]
    assert len(at) == 1, f"{path}: {marker!r} found {len(at)}"
    first = at[0]
    while first > 0 and lines[first - 1].startswith("///"):
        first -= 1
    cut = lines[first:]
    del lines[first:]
    wr(path, "\n".join(lines).rstrip("\n") + "\n")
    return "\n".join(cut).rstrip("\n") + "\n"

lease_block = take(SUP, "/// 🔒 The operating system releases this workspace lease", "/// 🌀 Serves the workspace", include_end=False)
lifecycle_block = tail_from(SUP, "pub fn serve(")
connect_or_start = take(CTL, "/// 🔗 A connection to the workspace daemon, which is started first when none runs.", "}")
assert connect_or_start.count("super::supervisor::spawn_daemon") == 1
connect_or_start = connect_or_start.replace("super::supervisor::spawn_daemon", "spawn_daemon")
sub(SUP, "use crate::terminal::labels::DashboardLabels;\n", "")

wr(J(n.DAEMON, n.LEASE, n.RS), (
    "//! 🔐️ The lease of a workspace daemon: the lock file the operating system releases when the daemon exits and the pid file\n"
    "//! `semio daemon status` reads.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs\n"
    "\n"
    "use super::ipc;\n"
    "use std::path::{Path, PathBuf};\n\n"
    + lease_block.rstrip("\n") + "\n"
))

lifecycle_block = lifecycle_block.replace("super::client::Connection", "Connection").replace("super::client::Message", "Message").replace("super::control::tasks", "control::tasks")
assert "super::" not in lifecycle_block, [l for l in lifecycle_block.split("\n") if "super::" in l]
wr(J(n.DAEMON, n.LIFECYCLE, n.RS), (
    "//! 🚦️ The lifecycle of the workspace daemon: serving a workspace, starting the daemon detached, asking its status, stopping it, and a\n"
    "//! connection that starts it first when none runs.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs\n"
    "\n"
    "use super::client::{Connection, Message};\n"
    "use super::control;\n"
    "use super::ipc::{self, ClientMsg, ServerMsg};\n"
    "use super::lease::{write_pid, WorkspaceLease};\n"
    "use super::supervisor::Supervisor;\n"
    "use super::transport::Listener;\n"
    "use dashboard_labels::DashboardLabels;\n"
    "use std::path::Path;\n"
    "use std::sync::atomic::{AtomicBool, Ordering};\n"
    "use std::time::{Duration, Instant};\n\n"
    + connect_or_start + "\n" + lifecycle_block
))

wr(WIRE_ROOT, (
    "//! 🧵️ The daemon wire: what both ends of the workspace daemon agree on and the plumbing under the supervisor — the frames and the\n"
    "//! workspace paths, the transport, the readiness of output, its replay from log files, the journal of sessions, the lease of a\n"
    "//! workspace and the connection a view holds. The supervisor, the control surface and the terminal view build on it.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json\n"
    "\n"
    f"#[path = \"../{n.IPC}/{n.RS}\"]\npub mod ipc;\n\n"
    f"#[path = \"../{n.READY}/{n.RS}\"]\npub mod ready;\n\n"
    f"#[path = \"../{n.REPLAY}/{n.RS}\"]\npub mod replay;\n\n"
    f"#[path = \"../{n.TRANSPORT}/{n.RS}\"]\npub mod transport;\n\n"
    f"#[path = \"../{n.JOURNAL}/{n.RS}\"]\npub mod journal;\n\n"
    f"#[path = \"../{n.LEASE}/{n.RS}\"]\npub mod lease;\n\n"
    f"#[path = \"../../{n.CONNECTION}/{n.RS}\"]\npub mod client;\n"
))

command_region = region(DAEMON_ROOT, rname("Command"))
tests_region = region(DAEMON_ROOT, rname("Tests"))
for old, new, count in [("crate::preferences::cli_locale", "dashboard_preferences::cli_locale", 1), ("supervisor::start_detached", "lifecycle::start_detached", 1), ("supervisor::serve", "lifecycle::serve", 1), ("supervisor::stop", "lifecycle::stop", 1), ("supervisor::status", "lifecycle::status", 1), ("crate::terminal::run(&root)", "dashboard_terminal::run(&root)", 1)]:
    assert command_region.count(old) == count, (old, command_region.count(old))
    command_region = command_region.replace(old, new)
assert "crate::" not in command_region and "supervisor" not in command_region
wr(J(n.DAEMON, n.COMMAND, n.RS), (
    "//! ⌨️ The `semio daemon start|serve|stop|status|attach` verb: it controls the lifecycle of the workspace daemon and attaches the\n"
    "//! terminal dashboard to it.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🚦️lifecycle/🦀️.rs\n"
    "\n"
    "use dashboard_core::args::ParsedArgs;\n"
    "use dashboard_daemon::lifecycle;\n"
    "use dashboard_labels::labels;\n"
    "use dashboard_wire::ipc;\n"
    "use std::path::{Path, PathBuf};\n\n"
    + command_region
))

wr(DAEMON_ROOT, (
    "//! 🌀️ The dashboard daemon: the process that owns the developer processes of a workspace independently of the views attached to it.\n"
    "//! One folder per concern. The wire both ends agree on and the plumbing under it (the frames, the transport, the readiness and\n"
    "//! replay of output, the journal, the lease and the connection a view holds) is the `dashboard_wire` crate; this crate is what\n"
    "//! only the daemon process and its controllers need — the views it serves, the launch groups, the supervisor, the control surface\n"
    "//! without a terminal and the lifecycle — and the `semio daemon start|serve|stop|status|attach` verb is the `⌨️command` folder.\n"
    "//!\n"
    "//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json\n"
    "\n"
    "use dashboard_wire::{client, ipc, journal, lease, ready, replay, transport};\n\n"
    f"#[path = \"{n.CLIENTS}/{n.RS}\"]\npub mod clients;\n\n"
    f"#[path = \"{n.GROUPS}/{n.RS}\"]\npub mod groups;\n\n"
    f"#[path = \"{n.SUPERVISOR}/{n.RS}\"]\npub mod supervisor;\n\n"
    f"#[path = \"{n.CONTROL}/{n.RS}\"]\npub mod control;\n\n"
    f"#[path = \"{n.LIFECYCLE}/{n.RS}\"]\npub mod lifecycle;\n\n"
    + tests_region
))

# daemon tests
subs(DAEMON_UNIT, [
    ("use super::*;\nuse crate::registry::{Launch, LaunchProcess, Stop};\nuse ipc::{ClientMsg, Ready, ServerMsg, SessionInfo, SessionStatus, SpawnGroup, TaskLabel};\nuse serde_json::Value;\nuse std::collections::{BTreeSet, HashMap};\n",
     "use super::*;\nuse dashboard_launch::{Launch, LaunchProcess, Ready, Stop, TaskLabel};\nuse ipc::{ClientMsg, ServerMsg, SessionInfo, SessionStatus, SpawnGroup};\nuse serde_json::Value;\nuse std::collections::{BTreeSet, HashMap};\nuse std::path::PathBuf;\n"),
    ("crate::tests::assert_proved", "dashboard_core::testkit::assert_proved", 8),
])
DAEMON_INT = J(n.DAEMON, n.TESTS, n.INTEGRATION, n.RS)
subs(DAEMON_INT, [
    ("use ipc::{ClientMsg, Ready, ServerMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup, TaskLabel};\nuse std::collections::BTreeMap;",
     "use dashboard_launch::{Ready, TaskLabel};\nuse ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup};\nuse std::collections::BTreeMap;"),
    ("use std::io::{Read, Write};\n", "use std::io::{Read, Write};\nuse std::path::{Path, PathBuf};\n"),
    ("crate::registry::LaunchProcess", "dashboard_launch::LaunchProcess", 4),
    ("crate::registry::Launch", "dashboard_launch::Launch", 3),
    ("crate::registry::Stop", "dashboard_launch::Stop", 2),
    ("supervisor::serve(", "lifecycle::serve(", 2),
])
print("stage A done")
