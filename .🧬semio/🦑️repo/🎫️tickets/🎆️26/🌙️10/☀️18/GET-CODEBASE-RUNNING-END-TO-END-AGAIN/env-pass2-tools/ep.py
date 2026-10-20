import json, os, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
FM = "🧰️framework/🔨️modules/"


def stage(name, rel, pairs):
    path = os.path.join(HERE, "staged", "patches", name + ".json")
    specs = json.load(open(path, encoding="utf-8")) if os.path.exists(path) else []
    text = open(os.path.join(ROOT, rel), encoding="utf-8").read()
    edits = []
    for pair in pairs:
        old, new = pair[0], pair[1]
        want = pair[2] if len(pair) > 2 else None
        n = text.count(old)
        if n == 0 or (want is not None and n != want):
            raise SystemExit("anchor mismatch (%d) in %s: %r" % (n, rel[-60:], old[:100]))
        edits.append({"old": old, "new": new, "count": n})
    specs = [s for s in specs if s["file"] != rel] + [{"file": rel, "edits": edits}]
    json.dump(specs, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("staged", name, rel[-70:], len(edits))


def overlay():
    print(subprocess.run([HERE + "/apply.sh", "overlay", C + "/🗑️generated/ep2-overlay"], capture_output=True, text=True).stdout.strip()[-60:])


INVOCATION_TEST = FM + "🏃️process/🧪️testing/📥️invocation/🟦️.ts"
ROUTING_PROCESS = FM + "🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"
ROUTING = FM + "🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"


def rel_import(file, target):
    p = os.path.relpath(os.path.join(ROOT, target), os.path.dirname(os.path.join(ROOT, file))).replace(os.sep, "/")
    return p if p.startswith(".") else "./" + p


def text_of(rel):
    return open(os.path.join(ROOT, rel), encoding="utf-8").read()


FROZEN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"


def drain_direct():
    import apply_lib
    return apply_lib.drain_direct()
