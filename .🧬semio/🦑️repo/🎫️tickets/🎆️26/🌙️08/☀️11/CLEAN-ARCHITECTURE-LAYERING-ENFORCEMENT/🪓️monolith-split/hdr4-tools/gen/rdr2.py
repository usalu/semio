import subprocess, sys
R = "/Users/ueli/Documents/semio"
# rdr2.py <needle>... : one git grep for all needles (-F -e ...), reports hits per needle
needles = sys.argv[1:]
args = ["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-n", "-F"]
for n in needles: args += ["-e", n]
args += ["--", "*.ts", "*.tsx", "*.py", "*.mjs", "*.json", "*.jsonc", "*.sh", ":!.🧬semio", ":!*Cargo.lock", ":!📋️build-owners.json", ":!.vscode/launch.json", ":!.vscode/🧩️launch.seed.jsonc", ":!*package.json", ":!*bun.lock"]
out = subprocess.run(args, cwd=R, capture_output=True, text=True).stdout.strip().split("\n")
out = [l for l in out if l]
print(len(out), "hits")
for n in needles:
    hits = [l for l in out if n in l]
    print("===", n, len(hits))
    for l in hits[:6]: print("   ", l[:260])
