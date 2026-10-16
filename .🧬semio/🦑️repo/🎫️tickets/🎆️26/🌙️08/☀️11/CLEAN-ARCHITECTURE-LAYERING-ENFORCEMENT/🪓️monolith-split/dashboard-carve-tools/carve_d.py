"""🪚 Stage D of the dashboard carve (a delta on stages A to C): the labels of tabs and windows move to the labels crate, the inventory and the command tree become their own crate. Usage: python3 carve_d.py <dashboard-dir>"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import spec
import manifests
from lib import rd, wr, sub, subs, take, region

D = sys.argv[1]
J = lambda *a: os.path.join(D, *a)
RG = n.e(0x1F516)
rname = lambda name: RG + name

def extract_fn(path, marker, remove=True):
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

def residual(path):
    left = [l for l in rd(path).split("\n") if re.search(r"\bcrate::", l)]
    return left

REG_ROOT = J(n.REGISTRY, n.RS)
REG_TESTS = J(n.REGISTRY, n.TESTS, n.UNIT, n.RS)
INV = J(n.INVENTORY, n.RS)
CT = J(n.COMMAND_TREE, n.RS)
CTT = J(n.COMMAND_TREE, n.TESTS, n.UNIT, n.RS)
LAB = J(n.TERMINAL, n.LABELS, n.RS)
LABT = J(n.TERMINAL, n.LABELS, n.TESTS, n.UNIT, n.RS)

# ---------------------------------------------------------------- D1: the labels of tabs and windows belong to the labels crate
labels_region = region(REG_ROOT, rname("Labels"))
mount = f"#[cfg(test)]\n#[path = \"{n.TESTS}/{n.UNIT}/{n.RS}\"]\nmod tests;"
lab = rd(LAB)
assert lab.count(mount) == 1
lab = lab.replace(mount, labels_region + "\n" + mount)
old_use = "use ui_locale::{AppLabels, Locale, Terminology};\n"
assert lab.count(old_use) == 1
lab = lab.replace(old_use, "use dashboard_launch::TaskLabel;\n" + old_use + "use ui_tui::tui::text::{elide_end, elide_middle};\n")
wr(LAB, lab)
sub(REG_ROOT, "use ui_tui::tui::text::{elide_end, elide_middle};\n", "use ui_tui::tui::text::elide_end;\n")
subs(REG_ROOT, [("const LABEL_CELLS: usize = 72;", "const LABEL_CELLS: u16 = 72;"), ("elide_end(&ticket.title, cap(LABEL_CELLS))", "elide_end(&ticket.title, LABEL_CELLS)")])
tab_test = extract_fn(REG_TESTS, "fn tab_text_keeps_the_verb_and_elides_the_subject_in_the_middle")
wr(LABT, rd(LABT).rstrip("\n") + "\n\n" + tab_test)

T = lambda *a: J(n.TERMINAL, *a)
subs(T(n.PANES, n.RS), [("dashboard_registry::tab_text(", "dashboard_labels::tab_text(", 1)])
subs(T(n.WINDOWS, n.RS), [("dashboard_registry::window_title(", "dashboard_labels::window_title(", 1), ("dashboard_registry::tab_texts(", "dashboard_labels::tab_texts(", 1), ("dashboard_registry::TAB_CELLS", "dashboard_labels::TAB_CELLS", 1)])
subs(T(n.LAUNCHER, n.RS), [
    ("use dashboard_labels::{labels, DashboardLabels};", "use dashboard_labels::{labels, verb_text, DashboardLabels};"),
    ("use dashboard_registry::{split_arguments, verb_text, Entry,", "use dashboard_registry::{split_arguments, Entry,"),
    ("dashboard_registry::window_title", "dashboard_labels::window_title", 3),
])

# ---------------------------------------------------------------- D2: the inventory crate
subs(REG_ROOT, [
    (f"#[path = \"../{n.INVENTORY}/{n.RS}\"]\npub mod inventory;\n\n#[path = \"../{n.COMMAND_TREE}/{n.RS}\"]\npub mod command_tree;\n\n", ""),
    (f"\n// #region {rname('Tests')}\n#[cfg(test)]\n#[path = \"{n.TESTS}/{n.UNIT}/{n.RS}\"]\nmod tests;\n// #endregion {rname('Tests')}\n", ""),
])
reg = rd(REG_ROOT)
assert reg.endswith("// #endregion " + rname("Registry") + "\n"), repr(reg[-80:])

subs(INV, [
    ("use crate::{graph_file, segment_key,", "use dashboard_registry::{graph_file, segment_key,"),
    ("pub use crate::GRAPH_REUSE;", "pub use dashboard_registry::GRAPH_REUSE;"),
    (f"// #region {rname('Snapshot')}", f"#[path = \"../{n.COMMAND_TREE}/{n.RS}\"]\npub mod command_tree;\n\n// #region {rname('Snapshot')}"),
    (f"// #region {rname('Tests')}\n#[cfg(test)]\n#[path = \"{n.TESTS}/{n.UNIT}/{n.RS}\"]\nmod tests;\n// #endregion {rname('Tests')}\n",
     f"// #region {rname('Tests')}\n#[cfg(test)]\n#[path = \"{n.TESTS}/{n.UNIT}/{n.RS}\"]\nmod tests;\n\n#[cfg(test)]\n#[path = \"../{n.REGISTRY}/{n.TESTS}/{n.UNIT}/{n.RS}\"]\nmod registry_tests;\n// #endregion {rname('Tests')}\n"),
])
subs(CT, [
    ("use crate::{Entry, Kind, Registry};", "use dashboard_registry::{Entry, Kind, Registry};"),
    ("crate::verb_rank", "dashboard_registry::verb_rank", 2),
    ("crate::inventory::discover", "crate::discover", 2),
])
subs(CTT, [
    ("use crate::{Request, RunPolicy};", "use dashboard_registry::{Request, RunPolicy};"),
    ("crate::graph_file()", "dashboard_registry::graph_file()", 2),
    ("crate::inventory::discover_with_catalog", "crate::discover_with_catalog", 2),
    ("crate::inventory::discover(", "crate::discover(", 1),
])
for path in (INV, CT, CTT):
    left = [l for l in residual(path) if "crate::command_tree" not in l and "crate::discover" not in l]
    assert not left, (path, left[:2])

head = ("use dashboard_launch::{Launch, LaunchProcess, Ready, Stop, TaskLabel};\n"
        "use dashboard_repo_domain::{RepoAction, RepoImplementation};\n"
        "use dashboard_registry::{graph_file, verb_of, verb_rank, CommandId, Facts, GraphBasis, PlaygroundFacts, Problem, ProjectFacts, Registry, Request, RunPolicy, TargetFacts, FALLBACK_VERB, GRAPH_REUSE};\n"
        "use std::collections::HashSet;\n"
        "use std::path::{Path, PathBuf};\n")
text = rd(REG_TESTS)
assert text.startswith("use super::*;\n")
text = head + text[len("use super::*;\n"):]
assert text.count("crate::inventory::discover_with_catalog") == 4
text = text.replace("crate::inventory::discover_with_catalog", "crate::discover_with_catalog")
wr(REG_TESTS, text)

# consumers
for path in [J(n.CLI, n.RS), J(n.CLI, n.TESTS, n.UNIT, n.RS)] + [os.path.join(dp, f) for dp, dns, fns in os.walk(J(n.TERMINAL)) for f in fns if f.endswith(".rs") and "node_modules" not in dp]:
    text = rd(path)
    if "dashboard_registry::inventory::" in text:
        wr(path, text.replace("dashboard_registry::inventory::", "dashboard_inventory::"))
GLUE = J(n.PKG, n.RUST, n.RS)
sub(GLUE, "dashboard_registry::command_tree::run", "dashboard_inventory::command_tree::run")

# ---------------------------------------------------------------- D3: manifests, script, README
manifests.write_manifests(D)
SCRIPT = J(n.PKG, n.RUST, "📜️script.ts")
sub(SCRIPT, f"\"{spec.package('dashboard_registry')}\", \"{spec.package('dashboard_wire')}\"", f"\"{spec.package('dashboard_registry')}\", \"{spec.package('dashboard_inventory')}\", \"{spec.package('dashboard_wire')}\"")
README = J("README.md")
readme = rd(README)
start = readme.index("The crate is a family of producer crates")
end = readme.index("## 🧪️ Tests")
readme = readme[:start] + (
    "The crate is a family of producer crates, one per layer, each rooted at the folder that states what it is; every folder keeps its\n"
    "own `📦️packages/🦀️rust` package. From the bottom: `🧱️core` (the invocation, the plugin and playground catalog, the dev environment and the\n"
    "content hash), `🧾️launch` (the launch contract), `🏛️repo-domain` (the repo actions), `🎮️registry` (commands, declarations and resolution),\n"
    "`📚️inventory` (reads the sources, keeps the snapshot, runs the background discovery; it also mounts `🌳️command-tree`, the registry\n"
    "projected as launcher rows and wizard steps, and runs the laws of the registry through a real discovery), `🌀️daemon/🧵️wire` (the frames,\n"
    "the transport, readiness, replay, journal, lease and `📎️connection`), `🌀️daemon` (the views, groups, supervisor, control surface and\n"
    "lifecycle), `⚙️preferences`, `🖥️terminal/🌐️labels` (every string, the tab texts and the window titles), `🖥️terminal` (the launcher, windows,\n"
    "panes, sessions and controls) and `🧭️cli` (the verbs above). The package of `📦️packages/🦀️rust` keeps the dispatch of `semio`,\n"
    "`📇️playground-catalog`, `🔌️plugin-registry`, `⌨️usage`, the `🌀️daemon/⌨️command` verb and `🚪️entrypoint`.\n\n"
) + readme[end:]
wr(README, readme)
print("stage D done")
