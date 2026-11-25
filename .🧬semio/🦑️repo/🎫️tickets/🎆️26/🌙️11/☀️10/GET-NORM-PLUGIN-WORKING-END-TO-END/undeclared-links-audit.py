#!/usr/bin/env python3
"""🔎 Audit required string fields of norm insert-mutation rows for undeclared or unmatched links.

Per artifact family, every insert mutation (🧬️schema/🧬️mutations/*insert-*) is resolved to its row schema
(the payload property that is an object or array of objects, following $ref across the family snapshot).
Every required string field of that row (top level and inside required nested objects, excluding the row's own
top-level id) is classified:
  A  declares x-semio-ui.ref.kind that no insert rule of the family matches (normalised: case and separators removed)
  B  declares no reference but looks like a link: name ends in Id/Ids, or title/description says it refers to an entity
Each entry also lists the source lines in the family (inferences, validators, diff, examples, tests) that mention the
field name, and the empty-string verdict from the schema (pattern, enum) and from the framework reference check.
Output goes to stdout only.

Links: 📚 https://json-schema.org/draft-07/schema  📚 https://www.rfc-editor.org/rfc/rfc6901
"""
import json
import os
import re
import sys

LINK_TEXT = re.compile(
    r"(refers? to|links? to|linked to|points to\b|pointer to|identif(?:y|ies) the|id of the|reference to|references? the"
    r"|verweist auf|bezieht sich auf|verknüpft|zeigt auf|bezug auf|referenz auf|\bID des\b|\bID der\b|Kennung des|Kennung der)",
    re.IGNORECASE,
)
INSERT_RE = re.compile(r"^[^a-zA-Z]*insert-(.+)$")
SCHEMA_REL = os.path.join("🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema")
MUT_NAME = "\U0001f9ec️mutations"
SNAP_NAME = "\U0001f4f8️snapshot"
SKIP_DIRS = {MUT_NAME, SNAP_NAME, "\U0001f4e6️packages"}


def norm(text):
    return re.sub(r"[^a-z0-9]", "", text.lower())


def find_root():
    here = os.path.dirname(os.path.abspath(__file__))
    cur = here
    while True:
        if os.path.isdir(os.path.join(cur, "✏️s")):
            return cur
        parent = os.path.dirname(cur)
        if parent == cur:
            raise SystemExit("repo root with the plugin tree not found")
        cur = parent


ROOT = find_root()
ARTS = os.path.join(ROOT, "✏️s", "\U0001f50c️plugins", "\U0001f4d5️norm", "\U0001f5ff️artifacts")

_json_cache = {}
_lines_cache = {}


def load(path):
    if path not in _json_cache:
        with open(path, encoding="utf-8") as fh:
            _json_cache[path] = json.load(fh)
    return _json_cache[path]


def lines_of(path):
    if path not in _lines_cache:
        with open(path, encoding="utf-8") as fh:
            _lines_cache[path] = fh.read().split("\n")
    return _lines_cache[path]


def build_id_index():
    index = {}
    for dirpath, _dirs, files in os.walk(ARTS):
        for name in files:
            if not name.endswith(".json"):
                continue
            path = os.path.join(dirpath, name)
            try:
                doc = load(path)
            except (ValueError, OSError):
                continue
            if isinstance(doc, dict) and isinstance(doc.get("$id"), str):
                index.setdefault(doc["$id"], path)
    return index


ID_INDEX = build_id_index()


def pointer(doc, frag):
    node = doc
    for token in [t for t in frag.split("/") if t]:
        token = token.replace("~1", "/").replace("~0", "~")
        node = node[token]
    return node


def resolve(node, path, line=0):
    """Follow $ref chains; return (resolved node, file path, line of the resolved node)."""
    hops = 0
    while isinstance(node, dict) and "$ref" in node:
        url, _, frag = node["$ref"].partition("#")
        target = ID_INDEX.get(url, path) if url else path
        if target is None:
            raise SystemExit(f"unresolved $ref {node['$ref']} in {path}")
        lines = lines_of(target)
        cur = 0
        node = load(target)
        for token in [t.replace("~1", "/").replace("~0", "~") for t in frag.split("/") if t]:
            node = node[token]
            cur = find_line(lines, token, cur)
        path = target
        line = cur
        hops += 1
        if hops > 50:
            raise SystemExit(f"$ref cycle at {path}")
    return node, path, line


def find_line(lines, key, start):
    pat = re.compile(r'"' + re.escape(key) + r'"\s*:')
    for i in range(max(start, 0), len(lines)):
        if pat.search(lines[i]):
            return i
    return max(start, 0)


def ui_of(*nodes):
    merged = {}
    for node in nodes:
        if isinstance(node, dict) and isinstance(node.get("x-semio-ui"), dict):
            merged.update(node["x-semio-ui"])
    return merged


def texts_of(*nodes):
    out = []
    for node in nodes:
        if not isinstance(node, dict):
            continue
        if isinstance(node.get("title"), str):
            out.append(node["title"])
        if isinstance(node.get("description"), str):
            out.append(node["description"])
        ui = node.get("x-semio-ui") if isinstance(node.get("x-semio-ui"), dict) else {}
        for key in ("label", "description"):
            val = ui.get(key)
            if isinstance(val, dict):
                out.extend(str(v) for v in val.values())
            elif isinstance(val, str):
                out.append(val)
    return out


def ref_kinds(ui):
    ref = ui.get("ref")
    if not isinstance(ref, dict):
        return None
    kind = ref.get("kind", ref.get("kinds"))
    if isinstance(kind, str):
        return [kind]
    if isinstance(kind, list):
        return [str(k) for k in kind]
    return []


def walk(node, path, line, prefix, depth, fields, row_file):
    if depth > 8:
        return
    sch, sp, sline = resolve(node, path, line)
    if not isinstance(sch, dict):
        return
    props = sch.get("properties", {})
    required = set(sch.get("required", []))
    lines = lines_of(sp)
    cursor = sline
    for name in props:
        if name not in required:
            continue
        raw = props[name]
        res, rp, rline = resolve(raw, sp, cursor)
        if not isinstance(res, dict):
            continue
        decl_line = find_line(lines, name, cursor)
        cursor = decl_line + 1
        typ = res.get("type")
        full = prefix + name
        if typ == "string":
            ui = ui_of(res, raw)
            fields.append({
                "path": full,
                "name": name,
                "ui": ui,
                "kinds": ref_kinds(ui),
                "widget": ui.get("widget"),
                "texts": texts_of(raw, res),
                "pattern": res.get("pattern", raw.get("pattern") if isinstance(raw, dict) else None),
                "enum": res.get("enum"),
                "schema_file": sp,
                "line": decl_line + 1,
                "row_file": row_file,
            })
        elif typ == "object" and "properties" in res:
            walk(res, rp, rline, full + ".", depth + 1, fields, row_file)
        elif typ == "array":
            items = res.get("items")
            if isinstance(items, dict):
                ri, ip, iline = resolve(items, rp, decl_line)
                if isinstance(ri, dict) and ri.get("type") == "object" and "properties" in ri:
                    walk(ri, ip, iline, full + "[].", depth + 1, fields, row_file)


def row_of_payload(mut_schema_path):
    doc = load(mut_schema_path)
    props = doc.get("properties", {})
    rows = []
    for name, raw in props.items():
        res, rp, _rl = resolve(raw, mut_schema_path)
        if not isinstance(res, dict):
            continue
        if res.get("type") == "object":
            rows.append((name, res, rp))
        elif res.get("type") == "array" and isinstance(res.get("items"), dict):
            ri, ip, _rl = resolve(res["items"], rp)
            if isinstance(ri, dict) and ri.get("type") == "object":
                rows.append((name, ri, ip))
    return rows


def families():
    for fam in sorted(os.listdir(ARTS)):
        fam_dir = os.path.join(ARTS, fam)
        if not os.path.isdir(fam_dir):
            continue
        muts = os.path.join(fam_dir, SCHEMA_REL, MUT_NAME)
        if not os.path.isdir(muts):
            yield fam, fam_dir, []
            continue
        inserts = []
        for d in sorted(os.listdir(muts)):
            m = INSERT_RE.match(d)
            if m and os.path.isdir(os.path.join(muts, d)):
                inserts.append((d, m.group(1)))
        yield fam, fam_dir, inserts


USAGE_ORDER = ["\U0001f4a1️inferences", "⚖️", "\U0001f53a️diff", "\U0001f4da️examples", "\U0001f52e️oracles"]


def usage_hits(fam_dir, field, limit=6):
    snake = re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), field)
    pats = [re.compile(r"\b" + re.escape(field) + r"\b"), re.compile(r"\b" + re.escape(snake) + r"\b")]
    hits = []
    for dirpath, dirs, files in os.walk(fam_dir):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS and d != "\U0001f9ec️mutations"]
        for name in files:
            if not name.endswith((".rs", ".ts", ".json", ".md", ".feature", ".proto", ".graphql", ".py", ".tsx")):
                continue
            path = os.path.join(dirpath, name)
            if SNAP_NAME in path:
                continue
            try:
                lines = lines_of(path)
            except (OSError, UnicodeDecodeError):
                continue
            for i, text in enumerate(lines):
                if any(p.search(text) for p in pats):
                    rel = os.path.relpath(path, ROOT)
                    hits.append((rel, i + 1, text.strip()[:220]))
    hits.sort(key=lambda h: (min([idx for idx, key in enumerate(USAGE_ORDER) if key in h[0]] or [99]), h[0], h[1]))
    return hits[:limit], len(hits)


def empty_verdict(field):
    if field["enum"] is not None and "" not in field["enum"]:
        return "refused by enum"
    if field["pattern"]:
        try:
            if re.search(field["pattern"], "") is None:
                return f"refused by pattern {field['pattern']}"
        except re.error:
            return "pattern not compilable"
    return "accepted by schema"


def main():
    out_rows = []
    totals = {}
    for fam, fam_dir, inserts in families():
        if not inserts:
            continue
        item_kinds = {norm(item) for _d, item in inserts}
        fam_rows = []
        for mut_dir, item in inserts:
            schema_path = os.path.join(fam_dir, SCHEMA_REL, MUT_NAME, mut_dir, "🧬️schema", "🔣️.json")
            for payload_name, row, row_path in row_of_payload(schema_path):
                fields = []
                walk(row, row_path, 0, "", 0, fields, row_path)
                for f in fields:
                    if f["path"] == "id":
                        continue
                    nk = [norm(k) for k in f["kinds"]] if f["kinds"] else []
                    unmatched = [k for k in (f["kinds"] or []) if norm(k) not in item_kinds]
                    declared = f["kinds"] is not None
                    link_name = f["name"].endswith("Id") or f["name"].endswith("Ids")
                    link_text = any(LINK_TEXT.search(t) for t in f["texts"])
                    if declared and unmatched:
                        cls = "A"
                    elif not declared and (link_name or link_text):
                        cls = "B"
                    else:
                        continue
                    fam_rows.append({
                        "family": fam,
                        "mutation": mut_dir,
                        "payload": payload_name,
                        "path": f["path"],
                        "cls": cls,
                        "name_link": link_name,
                        "text_link": link_text,
                        "kinds": f["kinds"],
                        "unmatched": unmatched,
                        "widget": f["widget"],
                        "texts": [t for t in f["texts"] if LINK_TEXT.search(t)] or f["texts"][:2],
                        "decl": os.path.relpath(f["schema_file"], ROOT) + ":" + str(f["line"]),
                        "empty": empty_verdict(f),
                        "field": f["name"],
                    })
        out_rows.append((fam, fam_rows, fam_dir, len(inserts)))

    for fam, fam_rows, fam_dir, n_inserts in out_rows:
        a = sum(1 for r in fam_rows if r["cls"] == "A")
        b = sum(1 for r in fam_rows if r["cls"] == "B")
        totals[fam] = (n_inserts, a, b)
        print(f"## {fam}  inserts={n_inserts}  A={a}  B={b}")
        grouped = {}
        for r in fam_rows:
            grouped.setdefault((r["cls"], r["field"]), []).append(r)
        for (cls, field), rows in sorted(grouped.items()):
            first = rows[0]
            hits, total = usage_hits(fam_dir, field)
            muts = sorted({r["mutation"] for r in rows})
            print(f"- [{cls}] {field} rows={len(rows)} mutations={len(muts)} path={first['path']}")
            print(f"    kinds={first['kinds']} unmatched={first['unmatched']} widget={first['widget']}")
            print(f"    decl={first['decl']} empty={first['empty']}")
            print(f"    texts={first['texts'][:2]}")
            print(f"    mutations={', '.join(muts)}")
            print(f"    usage_total={total}")
            for rel, ln, text in hits:
                print(f"      {rel}:{ln}: {text}")
    print("## TOTALS")
    for fam, (n, a, b) in totals.items():
        print(f"{fam}\tinserts={n}\tA={a}\tB={b}")


if __name__ == "__main__":
    sys.exit(main())
