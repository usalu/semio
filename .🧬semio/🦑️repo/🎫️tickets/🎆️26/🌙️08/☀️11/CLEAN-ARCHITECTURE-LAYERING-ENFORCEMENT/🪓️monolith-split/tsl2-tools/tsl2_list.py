import subprocess, sys, os
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
roots = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules/🏃️process"]
def run(args):
    return subprocess.run(["git", "-c", "core.quotepath=false"] + args, cwd=R, capture_output=True, text=True).stdout
files = set(run(["grep", "-l", "-F", "-e", "bun:test", "--"] + roots).split("\n"))
for u in run(["ls-files", "--others", "--exclude-standard", "--"] + roots).split("\n"):
    if u.endswith((".ts", ".tsx")):
        try:
            if "bun:test" in open(os.path.join(R, u), encoding="utf8", errors="replace").read():
                files.add(u)
        except OSError:
            pass
files = sorted(f for f in files if f and os.path.exists(os.path.join(R, f)))
open(G + "/tsl2-candidates.txt", "w", encoding="utf8").write("\n".join(files) + "\n")
print(len(files))
