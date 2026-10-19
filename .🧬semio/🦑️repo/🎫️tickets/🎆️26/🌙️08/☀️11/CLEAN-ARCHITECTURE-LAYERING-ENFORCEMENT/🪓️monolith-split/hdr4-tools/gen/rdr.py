import subprocess, sys
R = "/Users/ueli/Documents/semio"
# rdr.py <repo-rel file>... : non-Rust files naming the file path (last 3 components) or its crate dir
for f in sys.argv[1:]:
    parts = f.split("/")
    for k in (3,):
        n = "/".join(parts[-k:])
        out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-n", "-F", n, "--", "*.ts", "*.tsx", "*.py", "*.mjs", "*.json", "*.jsonc", "*.toml", "*.sh", "*.md", ":!.🧬semio", ":!*Cargo.lock", ":!📋️build-owners.json", ":!.vscode/launch.json", ":!.vscode/🧩️launch.seed.jsonc"], cwd=R, capture_output=True, text=True).stdout.strip().split("\n")
        out = [l for l in out if l and "Cargo.toml" not in l.split(":")[0]]
        print(f"=== {n}: {len(out)} hits")
        for l in out[:12]: print("   ", l[:230])
