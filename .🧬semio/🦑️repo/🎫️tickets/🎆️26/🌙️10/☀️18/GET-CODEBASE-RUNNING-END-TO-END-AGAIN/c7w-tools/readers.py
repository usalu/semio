"""📖 C5B source-text reader census: for each split file, every tracked non-Rust file (ts, tsx, py, mjs, json, jsonc, toml, sh, md; ticket tree and lock files excluded) that names its path (last three components), plus untracked ones. usage: readers.py <tag>..."""
import os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
def need(tag):
    parts = ft[tag].split("/")
    k = 4 if parts[-2] in ("🔬️unit",) or parts[-3] == "🧪️tests" and parts[-2].startswith("🔬️") and False else 3
    k = 5 if parts[-2] == "🔬️unit" else 3
    return "/".join(parts[-k:])
for tag in sys.argv[1:]:
    n = need(tag)
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-n", "-F", n, "--", "*.ts", "*.tsx", "*.py", "*.mjs", "*.json", "*.jsonc", "*.toml", "*.sh", "*.md", ":!.🧬semio", ":!*Cargo.lock", ":!📋️build-owners.json", ":!🔒️dependencies.json", ":!.vscode/launch.json", ":!.vscode/🧩️launch.seed.jsonc"], cwd=R, capture_output=True, text=True).stdout.strip().split("\n")
    out = [l for l in out if l and "Cargo.toml" not in l.split(":")[0]]
    print(f"=== {tag}: needle {n!r}: {len(out)} hits")
    for l in out[:20]:
        f, ln, rest = (l.split(":", 2) + ["", ""])[:3]
        print("   ", f[-110:], ln, rest.strip()[:170])
