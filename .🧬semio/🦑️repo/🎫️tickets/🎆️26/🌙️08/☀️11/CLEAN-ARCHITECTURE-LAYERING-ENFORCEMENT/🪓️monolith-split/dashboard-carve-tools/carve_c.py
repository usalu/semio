"""🪚 Stage C of the dashboard carve: manifests, Nx projects, facade manifest and script, README and source readers. Usage: python3 carve_c.py <dashboard-dir>"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import spec
from lib import rd, wr, sub, subs

D = sys.argv[1]
J = lambda *a: os.path.join(D, *a)

import manifests
manifests.write_manifests(D)

# facade script: the unit suites of all crates, bottom up
SCRIPT = J(n.PKG, n.RUST, "📜️script.ts")
order = ["dashboard_core", "dashboard_launch", "dashboard_repo_domain", "dashboard_registry", "dashboard_wire", "dashboard_preferences", "dashboard_labels", "dashboard_daemon", "dashboard_terminal", "dashboard_cli"]
names_list = ", ".join(f"\"{spec.package(key)}\"" for key in order)
script = rd(SCRIPT)
old_const = 'const crate = "semio-framework-repo-dashboard";\n'
assert script.count(old_const) == 1
script = script.replace(old_const, old_const + f"/** 🦀️ The dashboard crates, bottom up: the producers carved along the layers of the dashboard, then the facade that owns the `semio` binary. */\nconst crates = [{names_list}, crate];\n")
assert script.count("runRepositoryCargoTests([crate],") == 2
script = script.replace("runRepositoryCargoTests([crate],", "runRepositoryCargoTests(crates,")
wr(SCRIPT, script)

# README: the packages paragraph
README = J("README.md")
readme = rd(README)
start = readme.index("Modules of the crate, one folder each:")
end = readme.index("## 🧪️ Tests")
new_paragraph = (
    "The crate is a family of producer crates, one per layer, each rooted at the folder that states what it is; every folder keeps its\n"
    "own `📦️packages/🦀️rust` package. From the bottom: `🧱️core` (the invocation, the plugin and playground catalog, the dev environment and the\n"
    "content hash), `🧾️launch` (the launch contract), `🏛️repo-domain` (the repo actions), `🎮️registry` (commands, declarations, resolution, labels;\n"
    "it also mounts `📚️inventory`, which reads the sources, keeps the snapshot and runs the background discovery, and `🌳️command-tree`, the\n"
    "registry projected as launcher rows and wizard steps), `🌀️daemon/🧵️wire` (the frames, the transport, readiness, replay, journal,\n"
    "lease and `📎️connection`), `🌀️daemon` (the views, groups, supervisor, control surface and lifecycle), `⚙️preferences`, `🖥️terminal/🌐️labels`,\n"
    "`🖥️terminal` (the launcher, windows, panes, sessions and controls) and `🧭️cli` (the verbs above). The package of\n"
    "`📦️packages/🦀️rust` keeps the dispatch of `semio`, `📇️playground-catalog`, `🔌️plugin-registry`, `⌨️usage`, the `🌀️daemon/⌨️command` verb and\n"
    "`🚪️entrypoint`.\n\n"
)
readme = readme[:start] + new_paragraph + readme[end:]
wr(README, readme)

# source readers
TS = J(n.TESTS, n.REGISTRY, "🟦️.ts")
sub(TS, 'readFileSync(join(dashboard, "🎮️registry/🧪️tests/🔬️unit/🦀️.rs"), "utf8");\n  for (const input of ["", "a", "abc"])', 'readFileSync(join(dashboard, "🧱️core/🧪️tests/🔬️unit/🦀️.rs"), "utf8");\n  for (const input of ["", "a", "abc"])')
print("stage C done")
