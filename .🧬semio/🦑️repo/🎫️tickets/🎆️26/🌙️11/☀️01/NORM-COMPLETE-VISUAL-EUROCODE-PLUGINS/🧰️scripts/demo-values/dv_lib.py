"""Shared reader of the five catalogue artifacts: registry entries, input vocabulary, demo document, oracle module. Run with python3 -I."""
import importlib.util, json, pathlib, re, sys
ROOT = pathlib.Path("/home/deck/semio")
ARTS = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
DIRS = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997"}
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
ENTRY = re.compile(r'(?:Entry \{|entry_of\()\s*(?:id: )?"([^"]+)",\s*(?:(?:part|standard): )?"[^"]*",\s*(?:clause: )?"[^"]*",\s*(?:en: )?"((?:[^"\\]|\\.)*)",\s*(?:de: )?"((?:[^"\\]|\\.)*)",\s*(?:inputs: )?&\[([^\]]*)\]')

def art(code):
    return ARTS / DIRS[code] / "🏅️standards/🔖️1/🪆️subsets/✳️any"

def registry_files(code):
    return [p for p in art(code).rglob("🦀️.rs") if "🧪️tests" not in str(p) and "🗂️cases" not in str(p) and "📚️inputs" not in str(p)]

def entries(code):
    out = {}
    for p in registry_files(code):
        for m in ENTRY.finditer(p.read_text(encoding="utf-8")):
            out.setdefault(m.group(1), {"id": m.group(1), "en": json.loads('"%s"' % m.group(2)), "de": json.loads('"%s"' % m.group(3)), "inputs": re.findall(r'"([^"]+)"', m.group(4)), "file": str(p)})
    return list(out.values())

def vocab_path(code):
    return next(p for p in art(code).rglob("🔣️.json") if p.parent.name == "📚️inputs")

def vocab(code):
    return json.loads(vocab_path(code).read_text(encoding="utf-8"))

def demo_doc_path(code):
    return art(code) / "🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json"

def spec(v, entry_id, key):
    rows = [r for r in v["inputs"] if r["key"] == key]
    scoped = [r for r in rows if entry_id in [s.strip() for s in r.get("scope", "").split(",") if s.strip()]]
    return (scoped or [r for r in rows if not r.get("scope")] or rows or [None])[0]

def oracle(code):
    p = art(code) / "🔮️oracles/⚖️compliance/🐍️.py"
    sys.path.insert(0, str(p.parent))
    s = importlib.util.spec_from_file_location("oracle_" + code, p)
    m = importlib.util.module_from_spec(s)
    s.loader.exec_module(m)
    return m
