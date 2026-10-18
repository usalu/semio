"""🔎 C5B needle census: python3 grepn.py <needle>... prints file (last 4 components), line and the snippet around the needle for every tracked non-Rust hit (ticket tree, locks, owners, launch files excluded)."""
import subprocess, sys
R = "/Users/ueli/Documents/semio"
for n in sys.argv[1:]:
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-n", "-F", n, "--", "*.ts", "*.tsx", "*.py", "*.mjs", "*.json", "*.jsonc", "*.sh", ":!.🧬semio", ":!*Cargo.lock", ":!📋️build-owners.json", ":!🔒️dependencies.json", ":!.vscode"], cwd=R, capture_output=True, text=True).stdout.strip().split("\n")
    print(f"=== {n}: {len([o for o in out if o])} hits")
    for l in out:
        if not l: continue
        f, ln, rest = l.split(":", 2)
        i = rest.find(n)
        print("  ", "/".join(f.split("/")[-4:]), ln, rest[max(0, i - 60):i + len(n) + 70].replace("\n", " "))
