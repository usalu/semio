"""🔁 convert-field-to-collection: an object field of a row entity (`beams[].studs`) becomes a top-level id-keyed collection (`studs[]`). One tool operation, not a runtime migration: the new collection goes through the add-collection layers, the object leaves the schema documents, every fixture document is rewritten (the object becomes rows by the spec's row rules, verified against declared invariants) and the references left in hand-written sources are listed; no compatibility code remains."""
import ast, json, os, re
import jsonspan as js
from core import SEMIO, Unsupported, camel, walk_files
from ctx import get, parse, span_edit

ALLOWED = (ast.Expression, ast.BinOp, ast.UnaryOp, ast.Constant, ast.Name, ast.Load, ast.Add, ast.Sub, ast.Mult, ast.Div, ast.FloorDiv, ast.Mod, ast.USub, ast.Subscript, ast.Compare, ast.Lt, ast.LtE, ast.Gt, ast.GtE, ast.Eq, ast.NotEq, ast.BoolOp, ast.And, ast.Or, ast.Call, ast.GeneratorExp, ast.comprehension, ast.Store, ast.IfExp, ast.Tuple, ast.List, ast.Slice)
CALLS = {"len": len, "all": all, "any": any, "abs": abs, "sum": sum, "min": min, "max": max, "range": range, "int": int, "float": float, "round": round, "get": lambda row, key, default=None: row.get(key, default)}


def safe_eval(expression, **names):
    """🧮 Evaluates a row rule: arithmetic, comparisons, subscripts and the calls of `CALLS` only."""
    tree = ast.parse(expression, mode="eval")
    for node in ast.walk(tree):
        if not isinstance(node, ALLOWED) or isinstance(node, ast.Name) and node.id.startswith("_"):
            raise Unsupported(f"row rule {expression!r}: {type(node).__name__} is not allowed")
        if isinstance(node, ast.Call) and not (isinstance(node.func, ast.Name) and node.func.id in CALLS):
            raise Unsupported(f"row rule {expression!r}: only {', '.join(CALLS)} may be called")
    return eval(compile(tree, "<rule>", "eval"), {"__builtins__": {}, **CALLS, **names})


class Rule:
    """📏 How one old object becomes rows."""

    def __init__(self, raw, target):
        self.parent, self.field = raw["parent"], raw["field"]
        self.target = target
        self.count = raw["rows"]["count"]
        self.values = raw["rows"]["values"]
        self.verify = raw.get("verify", [])
        self.dropped = raw.get("dropped", [])
        self.drop_new = raw.get("dropForNewParents", True)

    def rows(self, obj, parent):
        n = int(safe_eval(self.count, obj=obj, parent=parent))
        out = []
        for i in range(n):
            row = {}
            for key, rule in self.values.items():
                row[key] = safe_eval(rule[len("="):], obj=obj, parent=parent, i=i) if isinstance(rule, str) and rule.startswith("=") else (rule.format(obj=obj, parent=parent, i=i) if isinstance(rule, str) else rule)
            out.append(row)
        for check in self.verify:
            if not safe_eval(check, rows=out, obj=obj, parent=parent):
                raise Unsupported(f"invariant {check!r} fails for {parent.get('id')}: the rows do not reproduce the object {obj}")
        return out


def rows_of_document(doc, rule, stats, keep_new=None, drop_inserted=False):
    """🧱 Rewrites one document in place; returns True when it changed. Snapshot documents: the parent rows lose the object and the rows land in the top-level collection; diff documents: inserted parent rows move their rows into the inserted lane of the new collection."""
    changed = False
    lane = doc.get(rule.parent)
    base_len = stats.get("base_len", 0)
    if isinstance(lane, list):
        extra = list(doc.get(rule.target) or [])
        for parent in lane:
            if isinstance(parent, dict) and isinstance(parent.get(rule.field), dict):
                obj = parent.pop(rule.field)
                if keep_new is not None and parent.get("id") in keep_new:
                    stats["dropped_for_new"] += 1
                else:
                    made = rule.rows(obj, parent)
                    extra += made
                    stats["rows"] += len(made)
                stats["objects"] += 1
                changed = True
        if changed:
            doc[rule.target] = extra
    elif isinstance(lane, dict) and isinstance(lane.get("inserted"), list):
        target = doc.get(rule.target)
        moved = []
        for item in lane["inserted"]:
            row = item.get("row") if isinstance(item, dict) else None
            if isinstance(row, dict) and isinstance(row.get(rule.field), dict):
                obj = row.pop(rule.field)
                if drop_inserted:
                    stats["dropped_for_new"] += 1
                else:
                    for sub in rule.rows(obj, row):
                        moved.append({**{k: v for k, v in item.items() if k != "row"}, "index": base_len + len(moved), "row": sub})
                        stats["rows"] += 1
                stats["objects"] += 1
                changed = True
        if changed and moved:
            if isinstance(target, dict):
                target["inserted"] = (target.get("inserted") or []) + moved
            else:
                doc[rule.target] = {**{k: [] for k in lane}, "inserted": moved}
    for key, value in doc.items():
        if key != rule.parent and isinstance(value, dict):
            changed |= rows_of_document(value, rule, stats, keep_new, drop_inserted)
        elif isinstance(value, list) and key != rule.parent and key != rule.target:
            for item in value:
                if isinstance(item, dict):
                    changed |= rows_of_document(item, rule, stats, keep_new, drop_inserted)
    return changed


def payload_rewrite(doc, rule, stats):
    """✂️ A mutation payload that embeds a parent row (`{"beam": {.., "studs": {..}}}`) loses the object: the new collection has its own insert kind."""
    changed = False
    if isinstance(doc, dict):
        for key, value in doc.items():
            if isinstance(value, dict) and isinstance(value.get(rule.field), dict) and "id" in value and key not in (rule.parent,):
                value.pop(rule.field)
                stats["payload_objects"] += 1
                changed = True
            else:
                changed |= payload_rewrite(value, rule, stats)
    elif isinstance(doc, list):
        for item in doc:
            changed |= payload_rewrite(item, rule, stats)
    return changed


def new_parent_ids(before, after, rule):
    """🆕 Parent ids an `after` document has and the sibling `before` document lacks (rows inserted by the mutation under test)."""
    have = {p.get("id") for p in (before or {}).get(rule.parent, []) if isinstance(p, dict)}
    return {p.get("id") for p in after.get(rule.parent, []) if isinstance(p, dict) and p.get("id") not in have}


def schema_layer(ctx):
    """🗑️ The object leaves the snapshot and artifact schema (property and required entry) and the diff Patch of its parent."""
    rule = ctx.rule
    ent = dict(ctx.collections).get(rule.parent)
    if not ent:
        raise Unsupported(f"{rule.parent} is not a collection of {ctx.short}")
    for rel in ("🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔣️.json"):
        path = ctx.layout.p(rel)
        data = json.loads(ctx.plan.read(path))
        rec = data["$defs"][ent]
        if rule.field in rec["properties"]:
            from schema_twins import dump_compact, dump_indent, layout_of
            text = ctx.plan.read(path)
            del rec["properties"][rule.field]
            if rule.field in rec.get("required", []):
                rec["required"].remove(rule.field)
            sub = next((v["$ref"].rsplit("/", 1)[1] for v in [ctx.defs[ent]["properties"][rule.field]] if "$ref" in v), None)
            ctx.plan.write(path, dump_compact(data) if layout_of(text) == "compact" else dump_indent(data))
            ctx.obsolete_type = sub
    ctx.plan.note(f"schema: `{rule.field}` leaves {ent} in the snapshot and artifact documents (its definition {getattr(ctx, 'obsolete_type', None)} stays until the Rust users are gone)")


def fixtures_layer(ctx):
    """♻️ Every fixture document of the artifact on disk or planned by an earlier layer is rewritten; it runs before and after the mutation layer (kind fixtures cloned from disk documents)."""
    rule = ctx.rule
    stats = {"objects": 0, "rows": 0, "dropped_for_new": 0, "payload_objects": 0, "files": 0}
    root = ctx.layout.any
    skip = ("/🧬️schema/📸️snapshot/", "/🧬️schema/🔺️diff/", "/🧬️schema/🔣️.json")
    docs = {}
    late = ctx.late if hasattr(ctx, "late") else False
    for path in sorted(set(walk_files(root, (".json",))) | {p for p in ctx.plan.files if p.startswith(root) and p.endswith(".json")}):
        rel = path[len(root):]
        if rel.startswith("/🧬️schema/") and "/🧬️mutations/" not in rel or "/📦️packages/" in rel:
            continue
        text = ctx.plan.read(path)
        if f'"{rule.field}"' not in text:
            continue
        try:
            docs[path] = json.loads(text)
        except ValueError:
            continue
    for path, doc in sorted(docs.items()):
        stats["base_len"] = len(docs.get(os.path.join(os.path.dirname(path), "🔣️.before.json"), {}).get(rule.target) or [])
        before = docs.get(os.path.join(os.path.dirname(path), "🔣️.before.json")) if path.endswith(("🔣️.after.json",)) else None
        keep = new_parent_ids(before, doc, rule) if before is not None and rule.drop_new else None
        changed = rows_of_document(doc, rule, stats, keep, os.path.exists(os.path.join(os.path.dirname(path), "🔣️.mutation.json")))
        changed |= payload_rewrite(doc, rule, stats)
        if changed:
            ctx.plan.write(path, json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
            stats["files"] += 1
    ctx.plan.note(f"fixtures: {stats['files']} JSON files rewritten, {stats['objects']} `{rule.field}` objects became {stats['rows']} rows, {stats['dropped_for_new']} objects of parents inserted by the mutation under test dropped (the insert payload cannot carry rows), {stats['payload_objects']} mutation payloads lost the object; dropped by spec: {', '.join(rule.dropped) or 'nothing'}")


def residual_layer(ctx):
    """🔎 Hand-written sources that still name the removed object: listed, never edited."""
    rule = ctx.rule
    words = re.compile(r"\b%s\b|\b%s\b" % (re.escape(rule.field), re.escape(camel(rule.field))))
    hits = {}
    for path in walk_files(ctx.layout.dir, (".rs", ".ts", ".sql", ".py", ".feature")):
        if "/🧬️schema/" in path and path.endswith(("🟦️.ts", "🛰️.proto", "🔗️.graphql")) or "/target/" in path:
            continue
        text = ctx.plan.read(path)
        count = len(words.findall(text))
        if count:
            hits[ctx.plan.rel(path)[len(ctx.plan.rel(ctx.layout.dir)) + 1:]] = count
    for rel, count in sorted(hits.items()):
        ctx.plan.note(f"MANUAL {count:>3}x `{rule.field}` in {rel}")
    ctx.plan.note(f"residual: {len(hits)} hand-written files still name `{rule.field}` (model field, SQLite sub-table, codec cells, checks, scene, mutation kinds on the old leaves)")


def layers(collection_layers):
    """🪜 The add-collection layers; the schema removal and the fixture rewrite run before the mutation layer so that the cloned kind fixtures start from rewritten documents."""
    def need(fn):
        return lambda ctx: fn(ctx)
    base = list(collection_layers)
    cut = [name for name, _ in base].index("mutations")
    return base[:cut] + [("convert-schema", need(schema_layer)), ("convert-fixtures", need(fixtures_layer))] + base[cut:] + [("convert-fixtures-late", need(fixtures_layer)), ("residual", need(residual_layer))]
