#!/usr/bin/env python3
"""F130 landing: applies the staged over-grant / live host pressure admission files to the live tree atomically (temp file + rename in the target folder), under the area landing locks `graph`, `memory` and `source` (taken together, in that order, never waited for: a held lock ends the run before any write), failing closed when any existing target changed since it was staged (digest of the live file differs from base-digests.json) or a new file already exists. `--check` verifies the digests and the locks only and writes nothing. Order: footprint, new module, scheduling port, governor, project wave, tests; the locks are released right after the last rename (checks run afterwards)."""
import hashlib, json, os, shutil, sys, time

LIVE = "/Users/ueli/Documents/semio"
T = LIVE + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
HERE = T + "/🧰️scripts/F130"
STAGE = HERE + "/stage"
AREAS = ["graph", "memory", "source"]
base = json.load(open(HERE + "/base-digests.json", encoding="utf8"))


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


staged = sorted(os.path.relpath(os.path.join(d, n), STAGE) for d, _s, f in os.walk(STAGE) for n in f)


def rank(rel):
    if "🧪️tests" in rel:
        return 9
    if "🦶️footprint" in rel:
        return 1
    if "🚧️wave-admission" in rel:
        return 2
    if "🗓️scheduling" in rel:
        return 3
    if "📦️project-nodes" in rel:
        return 5
    return 4


staged.sort(key=lambda rel: (rank(rel), rel.endswith(".mjs"), rel))


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


def lock(area):
    return T + "/landing.lock." + area


held = [a for a in AREAS if os.path.exists(lock(a))]
bad = verify()
if bad or held:
    print("\n".join(bad + ["lock held: " + a for a in held]))
    sys.exit(1)
print("digests hold for %d files, locks free" % len(staged))
if "--check" in sys.argv:
    sys.exit(0)

taken = []
try:
    for area in AREAS:
        try:
            os.mkdir(lock(area))
        except FileExistsError:
            print("lock %s is held, nothing written" % area)
            sys.exit(2)
        taken.append(area)
        open(lock(area) + "/owner", "w", encoding="utf8").write("F130 land %s\n" % time.strftime("%Y-%m-%d %H:%M:%S"))
    bad = verify()
    if bad:
        print("\n".join(bad))
        sys.exit(1)
    for rel in staged:
        live = os.path.join(LIVE, rel)
        os.makedirs(os.path.dirname(live), exist_ok=True)
        temp = os.path.join(os.path.dirname(live), ".f130-" + str(os.getpid()) + ".tmp")
        shutil.copyfile(os.path.join(STAGE, rel), temp)
        os.replace(temp, live)
        print("landed " + rel)
finally:
    for area in reversed(taken):
        try:
            os.remove(lock(area) + "/owner")
        except FileNotFoundError:
            pass
        try:
            os.rmdir(lock(area))
        except OSError:
            pass
