"""📖 C6PLUG source-text reader census, precise: (1) every tracked non-Rust file under the artifact directory that names the split file's last two components (relative reads), (2) every tracked file anywhere that names the last five components (absolute reads), (3) every Rust file under the artifact directory using include!/include_str!/#[path] that resolves to the split file. usage: readers2.py <tag>..."""
import os, subprocess, sys, re
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
EXC = [":!.🧬semio", ":!*Cargo.lock", ":!📋️build-owners.json", ":!🔒️dependencies.json", ":!.vscode/launch.json", ":!.vscode/🧩️launch.seed.jsonc"]
def git_grep(needle, paths):
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-n", "-F", needle, "--"] + paths + EXC, cwd=R, capture_output=True, text=True).stdout.strip().split("\n")
    return [l for l in out if l and "Cargo.toml" not in l.split(":")[0]]
for tag in sys.argv[1:]:
    rel = ft[tag]
    parts = rel.split("/")
    art = None
    for i, p in enumerate(parts):
        if p.startswith("🗿️artifacts") and i + 1 < len(parts):
            art = "/".join(parts[:i + 2])
    if art is None:
        art = "/".join(parts[:-2])
    t2 = "/".join(parts[-2:])
    ai = [i for i, p in enumerate(parts) if p.startswith("🗿️artifacts")]
    t5 = "/".join(parts[ai[0] + 1:]) if ai else "/".join(parts[-5:])
    print(f"=== {tag}: artifact dir {art[-80:]!r}")
    a = git_grep(t2, [art + "/**/*.ts", art + "/**/*.tsx", art + "/**/*.json", art + "/**/*.py", art + "/**/*.mjs", art + "/**/*.sh", art + "/**/*.md"])
    print(f"  (1) relative tails {t2!r}: {len(a)} hits")
    for l in a[:40]:
        f, ln, rest = (l.split(":", 2) + ["", ""])[:3]
        print("     ", f[-120:], ln, rest.strip()[:160])
    b = git_grep(t5, ["*.ts", "*.tsx", "*.json", "*.jsonc", "*.py", "*.mjs", "*.toml", "*.sh", "*.md"])
    print(f"  (2) absolute tails {t5!r}: {len(b)} hits")
    for l in b[:20]:
        f, ln, rest = (l.split(":", 2) + ["", ""])[:3]
        print("     ", f[-120:], ln, rest.strip()[:160])
