"""🏷️ Shared reader of the field-meta choice tables and the schema enums they edit (language-agnostic, standard library only)."""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
NORM = os.path.normpath(os.path.join(HERE, "..", ".."))
ARTIFACTS_ROOT = os.path.join(NORM, "🗿️artifacts")
SKIP_PARTS = {"node_modules", "🧫️fixtures", "🧪️tests", "target", "🗑️generated"}
CONST = re.compile(r"^(const\s+([A-Z0-9_]+)\s*:\s*&\[NormFieldChoice\]\s*=\s*&\[)(.*?)(\];)[ \t]*$", re.S | re.M)
ROW = re.compile(r'(?:choice\(|value:)\s*"((?:[^"\\]|\\.)*)"\s*,?\s*(?:label_en:\s*)?"((?:[^"\\]|\\.)*)"\s*,\s*(?:label_de:\s*)?"((?:[^"\\]|\\.)*)"')
ENTRY = re.compile(r'^\s*\(\s*"([^"]+)"\s*,(.*)$', re.M)
NAME = re.compile(r"\b([A-Z][A-Z0-9_]*)\b")


def walk(root):
    for base, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in SKIP_PARTS]
        yield base, files


def artifact_dirs(only):
    for name in sorted(os.listdir(ARTIFACTS_ROOT)):
        key = re.sub(r"^[^a-z]+", "", name)
        if not only or key in only:
            yield key, os.path.join(ARTIFACTS_ROOT, name)


def meta_files(root):
    return [os.path.join(b, "🦀️.rs") for b, f in walk(root) if os.path.basename(b) == "🏷️field-meta" and "🦀️.rs" in f]


def load_schemas(root):
    out = []
    for base, files in walk(root):
        if os.path.basename(base) in ("🧬️schema", "📸️snapshot") and "🔣️.json" in files:
            doc = json.load(open(os.path.join(base, "🔣️.json"), encoding="utf-8"))
            if "properties" in doc:
                out.append(doc)
    return out


def variants(doc, node):
    while isinstance(node, dict) and "$ref" in node:
        node = doc.get("$defs", {}).get(node["$ref"].split("/")[-1], {})
    if not isinstance(node, dict):
        return []
    subs = node.get("anyOf") or node.get("oneOf")
    if subs:
        return [v for s in subs for v in variants(doc, s)]
    return [node]


def enum_node(doc, path):
    nodes = [doc]
    for seg in [s for s in path.split(".") if s]:
        arrays = seg.endswith("[]")
        key = re.sub(r"\[[^\]]*\]$", "", seg)
        nxt = []
        for n in nodes:
            for v in variants(doc, n):
                prop = (v.get("properties") or {}).get(key)
                if prop is None and isinstance(v.get("additionalProperties"), dict):
                    prop = v["additionalProperties"]
                for pv in variants(doc, prop or {}):
                    if arrays and pv.get("type") == "array":
                        nxt += variants(doc, pv.get("items", {}))
                    elif not arrays:
                        nxt.append(pv)
        nodes = nxt
        if not nodes:
            return None
    return next((n for n in nodes if "enum" in n), None)


def norm_path(path):
    return re.sub(r"\[[^\]]*\]", "[]", path)


def unquote(s):
    return s.replace('\\"', '"').replace("\\\\", "\\")


def quote(s):
    return s.replace("\\", "\\\\").replace('"', '\\"')


def parse_meta(text):
    consts = {}
    for m in CONST.finditer(text):
        consts[m.group(2)] = [tuple(unquote(x) for x in r) for r in ROW.findall(m.group(3))]
    uses = {}
    for m in ENTRY.finditer(text):
        for n in NAME.findall(m.group(2)):
            if n in consts:
                uses.setdefault(n, []).append(m.group(1))
    return consts, uses


def key_of(token):
    return re.sub(r"[^a-z0-9]", "", token.lower())


def expected_rows(enum, old_rows):
    options = (enum.get("x-semio-ui") or {}).get("options") or {}
    old = {key_of(t): (en, de) for t, en, de in old_rows}
    rows, fallback = [], []
    for token in enum["enum"]:
        opt = options.get(token)
        if opt and opt.get("en") and opt.get("de"):
            rows.append((token, opt["en"], opt["de"]))
        elif key_of(token) in old:
            rows.append((token, *old[key_of(token)]))
        else:
            fallback.append(token)
            rows.append((token, token, token))
    return rows, fallback


def render(rows, literal):
    if literal:
        fmt = '    NormFieldChoice {{ value: "{}", label_en: "{}", label_de: "{}" }},'
    else:
        fmt = '    choice("{}", "{}", "{}"),'
    return "\n".join(fmt.format(*(quote(x) for x in r)) for r in rows)
