"""[lane diskproof] builds every staged batch below `staged/` from the CURRENT live files by anchored edits (`edits.py`): `python3 -I make.py [batch...]`; exit 2 when an anchor is gone (a peer edited that spot; rebase by reading the new text, never guess)."""
import hashlib, json, os, shutil, stat, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and os.path.dirname(ROOT) != ROOT: ROOT = os.path.dirname(ROOT)
L = edits.LIB; G = edits.GC; M = edits.MEMBERS; LE = edits.LEDGER; T = edits.TICKET
SUPERSEDE = M + "🧪️tests/🧹️supersede/🟦️.ts"
LAWS = {
 "closurefix": {"tests": [SUPERSEDE], "knownRed": ["the build removes superseded blocks after, never inside, its builder custody, and the collector removes them first"], "laws": [
   {"kind": "bun", "file": SUPERSEDE}, *[{"kind": "bun", "file": M + "🧪️tests/" + n + "/🟦️.ts"} for n in ("🔎️scan", "🧱️resolve", "👷️child", "♻️lru")], {"kind": "bun", "file": M + "🧪️tests/🟦️.ts"}]},
 "futurestamp": {"tests": [G + "🧪️tests/⏳️future/🟦️.ts", LE + "🟦️.ts", LE + "🧬️schema/🔣️.json"], "knownRed": [], "laws": [
   {"kind": "bun", "file": G + "🧪️tests/⏳️future/🟦️.ts"}, *[{"kind": "bun", "file": f} for f in (G + "🧪️tests/🟦️.ts", G + "🧪️tests/💰️budget/🟦️.ts", G + "🧪️tests/📁️directory/🟦️.ts", G + "🧪️tests/🔬️verify/🟦️.ts", G + "🧪️tests/🚀️launch/🟦️.ts", G + "🧪️tests/🧶️units/🟦️.ts", G + "🧪️tests/♻️quota/🟦️.ts", G + "👯️dedupe/🧪️tests/🟦️.ts", G + "🧮️sweep/🧪️tests/🟦️.ts", LE + "🧪️tests/🟦️.ts", LE + "🧪️tests/📏️needs/🟦️.ts", LE + "🧪️tests/📐️model/🟦️.ts", LE + "🧪️tests/🔮️sqlite/🟦️.ts")]]},
 "lawregex": {"tests": [], "knownRed": ["every writer named by the ledger exists", "no cache root exists below a module directory: partitions and their images belong to the checkout root"], "laws": [
   {"kind": "bun", "file": LE + "🧪️tests/⚖️law/🟦️.ts"}, {"kind": "bun", "file": LE + "🧪️tests/⚖️law/🌐️native/🟦️.ts", "filter": "source writes"}]},
 "acceptjournal": {"tests": [], "knownRed": ["every writer named by the ledger exists", "the nightly workflow runs the acceptance per product on a pinned macOS runner by schedule and by hand, in place, fails the blocking row on a breach, and keeps the reports"], "laws": [
   {"kind": "bun", "file": LE + "🧪️tests/⚖️law/🌐️native/🟦️.ts", "filter": "source writes"}, {"kind": "bun", "file": LE + "🧪️tests/⚖️law/🟦️.ts"}, {"kind": "bun", "file": LE + "🧪️tests/🟦️.ts"}, {"kind": "bun", "file": LE + "🧪️tests/📐️model/🟦️.ts"}, {"kind": "bun", "file": edits.ACCEPT + "🧪️tests/🟦️.ts"}]},
 "mirrordrop": {"tests": [], "knownRed": [], "laws": [{"kind": "python", "script": T + "diskproof-tools/tests/test_mirror_drop.py", "args": [T + "mirror-tools/mirror.py"]}]},
}
def digest(data): return hashlib.sha256(data).hexdigest()
def build(batch):
    new = edits.build(batch, ROOT)
    out = os.path.join(HERE, "staged", batch)
    shutil.rmtree(out, ignore_errors=True)
    for sub in ("files", "base"): os.makedirs(os.path.join(out, sub))
    manifest = {}
    for number, (rel, text) in enumerate(new.items()):
        data = text.encode("utf8"); live = os.path.join(ROOT, rel); name = "f%03d" % number
        base = None
        if os.path.exists(live):
            base_data = open(live, "rb").read(); base = digest(base_data)
            open(os.path.join(out, "base", name), "wb").write(base_data)
        open(os.path.join(out, "files", name), "wb").write(data)
        manifest[rel] = {"file": name, "base": base, "new": digest(data)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(ROOT, edits.TICKET + "xplat5-tools/stage.template.py"), encoding="utf8").read()
    first = template.split("\n", 1)
    doc = '"""🧹 Staged batch `%s` (lane diskproof, 10-27/28): see C/result-diskproof.md.\nusage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)\ncheck : exit 0 when every file is `base` or `applied`, 1 otherwise.\napply : every file must hold its base hash (absent for new files) or already the new one, else exit 2 (drift); atomic replace that keeps the mode.\nrevert: every file must hold the new hash (or already base), restores base bytes, removes new files and the directories they emptied."""' % batch
    body = template.split('"""', 2)[2]
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(first[0] + "\n" + doc + body)
    os.chmod(os.path.join(out, "stage.py"), 0o755)
    laws = dict(LAWS[batch]); json.dump(laws, open(os.path.join(out, "laws.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(os.path.join(out, "verify.sh"), "w", encoding="utf8").write('#!/bin/sh\n# usage: sh verify.sh <root> [before]   (laws of the batch %s on a mirror root; `before` = laws on base, must be RED)\nHERE=$(cd "$(dirname "$0")" && pwd)\nexec python3 -I "$HERE/../../verify_batch.py" "$HERE" "$@"\n' % batch)
    longest = max(len(rel) for rel in manifest)
    print("%-14s %d files, longest rel path %d chars" % (batch, len(manifest), longest))
if __name__ == "__main__":
    names = sys.argv[1:] or list(LAWS)
    for name in names:
        try: build(name)
        except edits.Drift as error:
            print("DRIFT", name, error); sys.exit(2)
