#!/usr/bin/env python3
"""F129 landing: applies the staged edges-only refresh files to the live tree atomically under the `graph` landing lock, failing closed when any target changed since it was staged (digest of the live file differs from base-digests.json, or a new file already exists). `--check` verifies the digests only and writes nothing, takes no lock. Order: new module first, then its consumers, tests last; every write is temp file plus rename in the target folder. The lock is released right after the last rename."""
import hashlib, json, os, shutil, sys, time

LIVE = "/Users/ueli/Documents/semio"
T = LIVE + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
HERE = T + "/🧰️scripts/F129"
STAGE = HERE + "/stage"
LOCK = T + "/landing.lock.graph"
base = json.load(open(HERE + "/base-digests.json", encoding="utf8"))
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


staged = []
for root, _dirs, files in os.walk(STAGE):
    for name in files:
        staged.append(os.path.relpath(os.path.join(root, name), STAGE))


def rank(rel):
    if "🪡️edge-heal" in rel:
        return 0
    if "/🧪️tests/" in rel or "🧪️tests" in rel:
        return 9
    if rel.endswith(".d.mts"):
        return 4
    if "🤝️prefetch" in rel or "🧭️discovery" in rel or "🕰️host" in rel:
        return 1
    if "🧭️plan" in rel:
        return 2
    return 5


staged.sort(key=lambda rel: (rank(rel), rel))


def verify():
    bad = []
    for rel in staged:
        live = os.path.join(LIVE, rel)
        if rel in base:
            if not os.path.exists(live) or sha(live) != base[rel]:
                bad.append("changed since staging: " + rel)
        elif os.path.exists(live):
            bad.append("new file already exists: " + rel)
    return bad


bad = verify()
if bad:
    print("\n".join(bad))
    sys.exit(1)
print("digests hold for %d files" % len(staged))
if "--check" in sys.argv:
    sys.exit(0)

try:
    os.mkdir(LOCK)
except FileExistsError:
    print("graph lock is held: " + open(LOCK + "/owner", encoding="utf8").read().strip() if os.path.exists(LOCK + "/owner") else "graph lock is held")
    sys.exit(2)
try:
    open(LOCK + "/owner", "w", encoding="utf8").write("F129 land %s\n" % time.strftime("%Y-%m-%d %H:%M:%S"))
    bad = verify()
    if bad:
        print("\n".join(bad))
        sys.exit(1)
    for rel in staged:
        live = os.path.join(LIVE, rel)
        os.makedirs(os.path.dirname(live), exist_ok=True)
        temp = os.path.join(os.path.dirname(live), ".f129-" + str(os.getpid()) + ".tmp")
        shutil.copyfile(os.path.join(STAGE, rel), temp)
        os.replace(temp, live)
        print("landed " + rel[len(G):] if rel.startswith(G) else "landed " + rel)
finally:
    try:
        os.remove(LOCK + "/owner")
    except FileNotFoundError:
        pass
    try:
        os.rmdir(LOCK)
    except OSError:
        pass
