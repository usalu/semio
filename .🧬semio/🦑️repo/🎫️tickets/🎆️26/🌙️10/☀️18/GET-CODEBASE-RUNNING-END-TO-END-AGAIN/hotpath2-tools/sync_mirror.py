"""🔄 Copies every file that changed or was added in the live tree since the mirror was cloned into the mirror (M and D lines of `mirror.py diff`), never the other way: python3 -I sync_mirror.py <mirror-name> [--skip PREFIX]..."""
import os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
name = sys.argv[1]
skip = [sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--skip"]
live = HERE
while not os.path.isdir(os.path.join(live, "🧰️framework")) and live != "/": live = os.path.dirname(live)
mirror = os.path.join(os.environ.get("SEMIO_MIRROR_HOME", os.environ.get("TMPDIR", "/tmp")), "sm", name)
out = subprocess.run([sys.executable, "-I", os.path.join(os.path.dirname(HERE), "mirror-tools", "mirror.py"), "diff", name], capture_output=True, text=True).stdout.splitlines()
count = 0
for line in out:
    if len(line) < 3 or line[1] != " " or line[0] not in "MD": continue
    rel = line[2:]
    if any(rel.startswith(p) for p in skip): continue
    source = os.path.join(live, rel)
    if not os.path.isfile(source): continue
    target = os.path.join(mirror, rel)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    if os.path.lexists(target): os.remove(target)
    shutil.copyfile(source, target); count += 1
print("synced", count, "files from live into", mirror)
