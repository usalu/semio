"""[gram2-g] staged batch `farm-listener`: the farm capacity request removes its abort listener from the long-lived invocation signal when released (fixes the ~230 MB retained per `emojiProjectJsonNodes` call). usage: batch.py apply|revert|status. Anchored edits (no whole-file replacement), idempotent, refuses when an anchor is missing or ambiguous."""
import hashlib, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 10)))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/"
SRC = L + "🟦️.ts"
LAW = L + "🧪️tests/🏊️farm/🟦️.ts"
OLD_SRC = ' const cancel=()=>{if(!admitted)controller.abort(refuseCapacityV1("cancelled","farm-wait-cancelled"));release();};\n'
NEW_SRC = ' const cancel=()=>{input.signal?.removeEventListener("abort",cancel);if(!admitted)controller.abort(refuseCapacityV1("cancelled","farm-wait-cancelled"));release();};\n'
OLD_IMPORT = 'import {tmpdir} from "node:os";'
NEW_IMPORT = 'import {getEventListeners} from "node:events";\nimport {tmpdir} from "node:os";'
MARK = '\ntest("a request leaves no listener on the long-lived signal of its invocation'


def read(path):
    return open(os.path.join(ROOT, path), encoding="utf8").read()


def write(path, text):
    open(os.path.join(ROOT, path), "w", encoding="utf8").write(text)


def swap(text, old, new, what):
    if text.count(new) == 1:
        return text
    if text.count(old) == 1:
        return text.replace(old, new)
    sys.exit(f"anchor of {what} missing or ambiguous: refuse")


def status():
    src, law = read(SRC), read(LAW)
    return {"source": NEW_SRC in src, "law": MARK in law, "sha256": {SRC: hashlib.sha256(src.encode()).hexdigest(), LAW: hashlib.sha256(law.encode()).hexdigest()}}


def apply():
    write(SRC, swap(read(SRC), OLD_SRC, NEW_SRC, "source"))
    law = swap(read(LAW), OLD_IMPORT, NEW_IMPORT, "law import")
    if MARK not in law:
        body = open(os.path.join(HERE, "law-test.ts"), encoding="utf8").read()
        law = law.rstrip("\n") + "\n" + body
    write(LAW, law)


def revert():
    write(SRC, read(SRC).replace(NEW_SRC, OLD_SRC))
    law = read(LAW).replace(NEW_IMPORT, OLD_IMPORT)
    at = law.find(MARK)
    if at >= 0:
        law = law[:at].rstrip("\n") + "\n"
    write(LAW, law)


{"apply": apply, "revert": revert, "status": lambda: print(status())}[sys.argv[1]]()
