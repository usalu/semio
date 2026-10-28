"""🧰 [rusttests] shared constants of the lane: live root, ticket folder C, the file list, the stack of unlanded staged batches this lane builds on, and relative-import computation."""
import hashlib, json, os
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
FW = "🧰️framework/"; PROC = FW + "🔨️modules/🏃️process/"; LIB = FW + "🛍️products/🦑️repo/🔨️modules/📚️library/"
HELPER = PROC + "🧪️testing/📥️invocation/🟦️.ts"; HELPER_SCHEMA = PROC + "🧪️testing/📥️invocation/🧬️schema/🪪️envelope.json"
HELPER_TESTS = PROC + "🧪️testing/📥️invocation/🧪️tests/🟦️.ts"; HELPER_FIXTURE = PROC + "🧪️testing/📥️invocation/🧪️tests/🧫️declared/🟦️.ts"
NATIVE = LIB + "🧪️testing/🪪️native-birth/🟦️.ts"
ROUTES = LIB + "🏗️build/🗂️suites/🧪️tests/🟦️.ts"
SUITES = [l.strip() for l in open(os.path.join(HERE, "suites.tsv"), encoding="utf8") if l.strip() and not l.startswith("#")]
ROWS = [l.split("\t") for l in SUITES]
STACK = ["td-leases-tools/staged/envelope", "td-misc-tools/staged/windows", "td-misc-tools/staged/consumption"]
def sha(data): return hashlib.sha256(data).hexdigest()
def read(path): return open(path, "rb").read() if os.path.exists(path) else None
def stacked(rel):
    live = read(os.path.join(LIVE, rel))
    for s in STACK:
        m = json.load(open(os.path.join(C, s, "manifest.json"), encoding="utf8"))
        if rel in m:
            if live is not None and sha(live) == m[rel]["new"]: return live, None
            if (live is None and m[rel]["base"] is None) or (live is not None and sha(live) == m[rel]["base"]):
                return open(os.path.join(C, s, "files", m[rel]["file"]), "rb").read(), s
    return live, None
def relative_import(from_file, to_file):
    r = os.path.relpath(os.path.join(LIVE, to_file), os.path.dirname(os.path.join(LIVE, from_file)))
    return r if r.startswith(".") else "./" + r
