#!/usr/bin/env python3
import json
import re

R = "/Users/ueli/Documents/semio/"
G = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/launch-dependents/"
REG = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
DASH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"

P = re.compile(
    r"launch\.json|launch\.seed|🧩️launch|launchSeed|LAUNCH_OUTPUT_REL_PATH|SEED_REL_PATH|devLaunchers|projectLaunchers|serverReadyAction"
    r"|launch(?:Name|Command|Order|Group|Path|Paths|Catalogs|Seed|SeedPath|Contribution|Playgrounds)|derivedLaunch|forwardedLaunch|previewLaunchCommand"
    r"|launch-name-prefix|launchNamePrefix|launch (?:row|rows|entry|entries|configuration|configurations|catalog|catalogs|seed|surface|surfaces|registration|route|surfaces)"
    r"|launch-(?:seed|name|placement)|reconcile-launch|test-launch-|🚀️launch|\.claude/launch|launcher row|launch-row|\"launch\": ?[\[{]|\"node-terminal\"|LaunchSeed|LaunchJson|LaunchNameContract|INTERACTIVITY_ALL_APP_(?:LAUNCH|REQUIRED|BROWSER|NATIVE)|interactivityAllApp(?:Launch|PlaygroundLaunch)|LAUNCH_CONFIGURATIONS|collect_launch|CompoundLeaf|Compound\(|launch configuration"
)

scan = json.load(open(G + "scan.json", encoding="utf-8"))["results"]
strongk = ["launch.json", "launch.seed", "claude-launch", "launchSeed", "LAUNCH_OUTPUT_REL_PATH", "mod:🚀️launch", "devLaunchers", "serverReadyAction", "node-terminal", "launch-wording", "launch-fixtures"]
files = set()
for f, h in scan.items():
    if any(k in h for k in strongk):
        files.add(f)
for line in open(G + "launch-idents.txt", encoding="utf-8"):
    files.add(line.rsplit(":", 1)[0].replace("LIB/", LIB + "/"))
files |= {
    ".claude/launch.json",
    ".gitignore",
    "AGENTS.md",
    "README.md",
    "🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml",
    LIB + "/🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json",
    LIB + "/🧫️fixtures/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json",
    "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json",
    "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json",
}
files |= {
    REG + "/🔎️discovery/🧪️tests/🟦️.ts",
    REG + "/🔎️discovery/🧪️tests/🚀️source-examples.feature",
    REG + "/🔎️discovery/🧪️tests/🎮️session-catalog.feature",
}
files |= {REG + "/📋️project.json"}
ANCHORLESS = {
    DASH + "/🧫️fixtures/🚀️launch-configurations/🔣️.json",
    REG + "/🧫️fixtures/🚀️launch/🔣️.json",
    REG + "/🧫️fixtures/🚀️launch/🏷️name-prefix/🔣️.json",
    REG + "/🧬️schema/🚀️launch/🔣️.json",
    REG + "/🚀️launch/🧬️schema/🧱️placement/🔣️.json",
}
files |= ANCHORLESS
OVERRIDE = {
    ".🧬semio/🦑️repo/💬️prompts/🐙️ueli.md": [2876, 4969, 7980, 9737, 12794],
    LIB + "/🔣️taxonomy.json": [9213, 14758, 26969, 27016],
    LIB + "/🖼️assets/📽️nested-cargo-package-projection/🔣️.json": [727, 732, 738],
}
FP = ("⏯️preview-eval-run.json", "🐾️pets/🧫️fixtures", "📖️pdf", "tool-run")
data = {}
for f in sorted(files):
    if f.startswith(".vscode/") or any(s in f for s in FP):
        continue
    try:
        lines = open(R + f, encoding="utf-8", errors="replace").read().split("\n")
    except OSError:
        continue
    hits = [i + 1 for i, l in enumerate(lines) if P.search(l[:4000])]
    if f == "README.md":
        hits = [n for n in hits if n in (660, 661, 663, 863)]
    if f == "AGENTS.md":
        hits = [50]
    if f == ".gitignore":
        hits = [591]
    if f in ANCHORLESS:
        hits = hits or [1]
    if f in OVERRIDE:
        hits = OVERRIDE[f]
    if not hits:
        continue
    data[f] = hits


COMMENT_ONLY = ("🏛️bestest/🧪️tests/🔬️unit/🦀️.rs", "⚡️epjson/🔖️25.2", "🏛️export-epjson-runs-in-energyplus/🐍️.py", "🤖️live-agent-loop/🟦️.ts", "🧮️program-matrix/🟦️.ts")


def category(f):
    if any(k in f for k in COMMENT_ONLY):
        return "C"
    if f in (LIB + "/🔣️taxonomy.json", LIB + "/🖼️assets/📽️nested-cargo-package-projection/🔣️.json"):
        return "L"
    if f.startswith(REG + "/🚀️launch/") or f.startswith(REG + "/🧪️tests/🚀️launch/") or f.startswith(REG + "/🧫️fixtures/🚀️launch/") or f.startswith(REG + "/🧬️schema/🚀️launch/"):
        return "G"
    if f.startswith(REG + "/"):
        return "W"
    if f == "📜️script.ts":
        return "S"
    if f.startswith(DASH + "/"):
        return "D"
    if f == REG + "/📋️project.json":
        return "W"
    if f.endswith("project.json"):
        return "N"
    if f.startswith(".claude/") or f.startswith(".devcontainer/") or f in (".gitignore", "AGENTS.md"):
        return "E"
    if f.endswith(".md") or f.endswith(".feature"):
        return "H"
    if "🧪️tests" in f or f.endswith(("🔬️_test.go",)) :
        return "T"
    if "🧫️fixtures" in f or "🧬️schema" in f or "🧬️contract" in f or f.endswith("Cargo.toml") or f.endswith("watch-policy.json"):
        return "F"
    if f.endswith("📜️script.ts"):
        return "T2"
    return "C"


ACTION = {
    "L": "taxonomy / authority ledger: delete entries (see 3.2, risk R3)",
    "G": "delete file",
    "W": "delete region(s) / rename (see 3.2)",
    "S": "delete regions (see 3.3)",
    "D": "rewrite: stop reading launch.json (see 3.4)",
    "N": "delete the launch-file input lines",
    "E": "see section 4",
    "H": "reword docs",
    "T": "delete launch block (tables A/B) or re-express",
    "T2": "delete launch read / re-express (table B)",
    "F": "delete launch keys / rows",
    "C": "reword comment/message or delete read",
}
rows = []
for f, hits in data.items():
    c = category(f)
    shown = ", ".join(str(n) for n in hits[:16]) + (f" (+{len(hits) - 16} more)" if len(hits) > 16 else "")
    rows.append((c, f, shown, len(hits)))
order = ["G", "W", "L", "S", "D", "N", "T", "T2", "F", "H", "C", "E"]
rows.sort(key=lambda r: (order.index(r[0]), r[1]))
out = ["| cat | file | anchor lines | action |", "|---|---|---|---|"]
for c, f, shown, n in rows:
    short = f.replace(LIB + "/", "📚️library/").replace(REG + "/", "R/").replace(DASH + "/", "DASH/").replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "OS/")
    out.append(f"| {c} | `{short}` | {shown} | {ACTION[c]} |")
open(G + "index.md", "w", encoding="utf-8").write("\n".join(out) + "\n")
counts = {}
for c, *_ in rows:
    counts[c] = counts.get(c, 0) + 1
print(len(rows), counts)
