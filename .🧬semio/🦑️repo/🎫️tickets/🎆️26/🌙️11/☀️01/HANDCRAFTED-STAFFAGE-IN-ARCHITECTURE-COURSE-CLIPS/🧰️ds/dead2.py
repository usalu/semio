import subprocess, sys, os

rows = [l.split("\t") for l in open(sys.argv[1], encoding="utf-8").read().splitlines() if l]
n = 0
for p, nm, ln in rows:
    r = subprocess.run(["rg", "-w", "-n", "--no-heading", "-I", "--glob", "!**/.venv/**", "--glob", "!**/site-packages/**", "--glob", "!**/node_modules/**", "--glob", "!**/.🧬semio/**", "--glob", "!**/*.md", "--glob", "!**/target/**", "-e", nm, "🎓️teaching", "🧰️", "📚️library"],
                       capture_output=True, text=True, encoding="utf-8", errors="replace")
    hits = [h for h in r.stdout.splitlines() if h]
    own = os.path.normpath(p)
    other = [h for h in hits if not (os.path.normpath(h.split(":")[0]) == own and h.split(":")[1] == ln)]
    non_self = [h for h in other if os.path.normpath(h.split(":")[0]) != own]
    if not other:
        n += 1
        print("DEAD", p.split("architecture/")[-1], nm, ln, flush=True)
print(n)
