"""🔤 Rebuilds staged batch `umlaut` from the CURRENT live files: German catalog texts are written with ä ö ü ß instead of ae oe ue ss transliterations (word map `umlaut_words.json`, whole words only, `de` leaves only)."""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stagelib import build
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
WORDS = json.load(open(os.path.join(HERE, "umlaut_words.json"), encoding="utf8"))
TOKEN = re.compile(r"[A-Za-zÄÖÜäöüß]+(?:-[A-Za-zÄÖÜäöüß]+)*")
def fix(text):
    def word(m):
        w = m.group(0)
        if w in WORDS: return WORDS[w]
        parts = w.split("-")
        return "-".join(WORDS.get(p, p) for p in parts) if len(parts) > 1 else w
    return TOKEN.sub(word, text)
def leaves(o):
    if isinstance(o, str): yield o
    elif isinstance(o, dict):
        for v in o.values(): yield from leaves(v)
    elif isinstance(o, list):
        for v in o: yield from leaves(v)
def de_strings(o):
    if isinstance(o, dict):
        if "de" in o and not ({"type", "properties"} & set(o)): yield from leaves(o["de"])
        for v in o.values(): yield from de_strings(v)
    elif isinstance(o, list):
        for v in o: yield from de_strings(v)
def plan():
    files = subprocess.run(["git", "-C", ROOT, "ls-files", "-z"], capture_output=True).stdout.decode().split("\0")
    if not files[0]: files = [os.path.relpath(os.path.join(d, n), ROOT) for top in ("🧰️framework", "🚀️launcher", "🎓️teaching") for d, _s, ns in os.walk(os.path.join(ROOT, top)) if "node_modules" not in d for n in ns]
    files = [f for f in files if f.endswith(".json") and f.startswith(("🧰️framework", "🚀️launcher", "🎓️teaching")) and not re.search("/tests/|/🧪|fixtures|🧫", f)]
    edits = []
    for rel in files:
        try: doc = json.load(open(os.path.join(ROOT, rel), encoding="utf8"))
        except Exception: continue
        pairs = {s: fix(s) for s in de_strings(doc) if fix(s) != s}
        if pairs: edits.append((rel, pairs))
    return edits
SKIP = re.compile(r"node_modules|/🔌️mcp/|/mcp/mcp|\.md$|\.json$")
WORD = re.compile(r"(?<![A-Za-z0-9_äöüÄÖÜß])(" + "|".join(sorted((re.escape(k) for k in WORDS), key=len, reverse=True)) + r")(?![A-Za-z0-9_äöüÄÖÜß])")
def plan_text():
    live = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", ".."))
    words = os.path.join(HERE, "umlaut_words.txt")
    open(words, "w", encoding="utf8").write("\n".join(WORDS) + "\n")
    found = subprocess.run(["git", "-C", live, "grep", "-l", "-w", "-F", "-f", words, "--", "🧰️framework", "🚀️launcher", "🎓️teaching"], capture_output=True).stdout.decode().split("\n")
    os.remove(words)
    return [rel for rel in found if rel and not SKIP.search(rel) and rel.endswith((".ts", ".mjs", ".rs", ".go", ".py")) and os.path.exists(os.path.join(ROOT, rel))]
def text_editor(text):
    return WORD.sub(lambda m: WORDS[m.group(1)], text)
def editor(pairs):
    def run(text):
        for old, new in pairs.items():
            lit = json.dumps(old, ensure_ascii=False)
            if lit not in text: raise SystemExit(f"literal not found verbatim (escaped differently?): {old[:80]!r}")
            text = text.replace(lit, json.dumps(new, ensure_ascii=False))
        return text
    return run
VERIFY = r'''PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT"
"$PY" -I "$HERE/law_umlaut.py" "$ROOT" "$HERE/manifest.json"
mkdir -p "${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}"
while IFS= read -r d; do [ -n "$d" ] && (cd "$ROOT/$d" && echo "### $d" && bun test --timeout 60000 ./🟦️.ts 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' || true); done < "$HERE/tests.txt"
echo "umlaut batch: law GREEN; compare the (fail) lines above with the baseline in C/result-progress.md"
'''
DOC = "🔤 Staged batch `umlaut` (lane progress): German catalog texts use ä ö ü ß instead of the ae oe ue ss transliterations (whole-word map, `de` leaves only)."
if __name__ == "__main__":
    plan_rows = plan()
    edits = [(rel, editor(pairs)) for rel, pairs in plan_rows] + [(rel, text_editor) for rel in plan_text()]
    name = sys.argv[2] if len(sys.argv) > 2 else "umlaut"
    build(name, ROOT, edits, VERIFY, DOC)
    out = os.path.join(HERE, "staged", name)
    open(os.path.join(out, "law_umlaut.py"), "w", encoding="utf8").write(open(os.path.join(HERE, "law_umlaut.py"), encoding="utf8").read())
    open(os.path.join(out, "tests.txt"), "w", encoding="utf8").write(open(os.path.join(HERE, "umlaut_tests.txt"), encoding="utf8").read())
    print(sum(len(p) for _r, p in plan_rows), "strings in", len(plan_rows), "files")
