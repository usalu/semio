"""🛠️ Staged batch `lease-live-language` (proof2-g): the German yield law pins the holder's language itself.

`python3 lease-live-language.py check|apply|revert`. Base: `🔒️leases/📈️standing/🧪️tests/🚦️live/🟦️.ts` (sha256 below).
Defect: the holder is the test process; its yield notice is written in the language chosen by `SEMIO_LANGUAGE` / `LC_ALL` / `LANG`
(`leaseLanguagesV1`), so with any English language chosen (the official run sets SEMIO_LANGUAGE=en) the law sees English only and fails;
with no language chosen or `de` it passes. Fix: the test removes the chosen language for its duration and restores it."""
import hashlib, os, sys

ROOT = os.environ.get("PATCH_ROOT") or os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 9)))
FILE = os.path.join(ROOT, "🧰️framework", "🔨️modules", "🏃️process", "🔒️leases", "📈️standing", "🧪️tests", "🚦️live", "🟦️.ts")
BASE = None
OPEN_OLD = 'test("a waiter on newer code asks a stale holder to yield, in German, and the holder yields where its result would be discarded",async()=>{\n'
OPEN_NEW = OPEN_OLD + ' const chosen=["SEMIO_LANGUAGE","LC_ALL","LANG"].map(name=>[name,process.env[name]] as const);for(const [name] of chosen)delete process.env[name];\n try{\n'
CLOSE_OLD = 'Prozess ${process.pid} gebeten, ${resource} freizugeben (stale)`);\n},60000);\n'
CLOSE_NEW = 'Prozess ${process.pid} gebeten, ${resource} freizugeben (stale)`);\n }finally{for(const [name,value] of chosen){if(value===undefined)delete process.env[name];else process.env[name]=value;}}\n},60000);\n'


def sha(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    text = open(FILE, encoding="utf8").read()
    applied = OPEN_NEW in text and CLOSE_NEW in text
    fresh = text.count(OPEN_OLD) == 1 and text.count(CLOSE_OLD) == 1
    if mode == "check":
        print("applied" if applied else "fresh" if fresh else "ANCHORS MOVED")
        return
    if mode == "apply":
        if not fresh:
            sys.exit("anchors moved or already applied")
        text = text.replace(OPEN_OLD, OPEN_NEW).replace(CLOSE_OLD, CLOSE_NEW)
    elif mode == "revert":
        if not applied:
            sys.exit("not applied")
        text = text.replace(OPEN_NEW, OPEN_OLD).replace(CLOSE_NEW, CLOSE_OLD)
    else:
        sys.exit("check|apply|revert")
    open(FILE, "w", encoding="utf8").write(text)
    print(mode, "ok", sha(text)[:12])


main()
