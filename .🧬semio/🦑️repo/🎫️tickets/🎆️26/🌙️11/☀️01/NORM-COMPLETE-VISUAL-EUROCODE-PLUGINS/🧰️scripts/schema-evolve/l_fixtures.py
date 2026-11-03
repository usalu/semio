"""🧫 Fixture layer: every committed snapshot and diff JSON that carries the reference collection gets the new key (empty list in snapshots, `null` or an empty delta in diffs, by the reference lane), and every DSL asset gets an empty table in schema order. Edits are text spans, so each file keeps its layout."""
import json, os, re
import jsonspan as js
from core import SKIP_DIRS, Unsupported, camel, walk_files
from ctx import dumps


def lane_kind(ctx):
    """📤 How a diff lane of the reference is typed: `Option<..>` (null when untouched) or a plain delta value."""
    path = ctx.layout.p("🧬️schema/🔺️diff/🦀️.rs")
    text = ctx.plan.read(path)
    return "option" if re.search(r"pub %s: Option<" % ctx.ref.plural_snake, text) else "delta"


EMPTY_DELTA = {"removed": [], "inserted": [], "moved": [], "modified": []}


def fixture_value(anchor_value, lane):
    if anchor_value.kind == "array":
        return "[]"
    if anchor_value.kind == "scalar" and anchor_value.text == "null":
        return "null"
    if anchor_value.kind == "object":
        return "null" if lane == "option" else None
    return None


def run_json(ctx):
    L, S = ctx.layout, ctx.spec.collection
    ref_key, anchor, new = ctx.ref.plural_camel, ctx.anchor, S.names.plural_camel
    lane = lane_kind(ctx)
    count = 0
    needle = f'"{ref_key}"'
    for path in walk_files(L.any, (".json",)):
        rel = path[len(L.any) + 1:]
        if rel.startswith(("🧬️schema/🔣️.json", "🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔺️diff/🔣️.json")):
            continue
        try:
            text = ctx.plan.read(path)
        except UnicodeDecodeError:
            continue
        if needle not in text or f'"{new}"' in text:
            continue
        try:
            root = js.parse(text)
        except (AssertionError, IndexError, ValueError):
            continue
        hits = js.find_objects(root, lambda o: o.get(ref_key) is not None and o.get(anchor) is not None and o.get("properties") is None)
        edits = []
        for _, obj in hits:
            if obj.get(new):
                continue
            value = fixture_value(obj.get(anchor).value, lane)
            if value is None:
                continue
            member = obj.get(anchor)
            colon = text[member.key_end:member.value.start]
            separator = js._separator(text, obj, obj.members.index(member))
            edits.append((member.value.end, separator + json.dumps(new) + colon + value))
        if edits:
            ctx.plan.write(path, js.apply_edits(text, edits))
            count += 1
    ctx.plan.note(f"fixtures: {count} snapshot/diff JSON files get `{new}`")


def dsl_type(f, parent_nested=False):
    if f.ty in ("String", "enum"):
        return "TEXT"
    if f.ty in ("u32", "u8"):
        return "UINT"
    return "QTY" if f.unit in ("m", "m2", "Pa", "N", "deg") else "NUM"


def run_assets(ctx):
    L, S, r = ctx.layout, ctx.spec.collection, ctx.ref
    kebab = r.plural_kebab
    count = 0
    for path in walk_files(L.any, (".dsl.semio",)):
        text = ctx.plan.read(path)
        if not re.search(r"^%s \[" % re.escape(kebab), text, re.M) or re.search(r"^%s \[" % re.escape(S.names.plural_kebab), text, re.M):
            continue
        cols = ["id:TEXT"] + [f"{f.name.replace('_', '-')}:{dsl_type(f)}" for f in S.fields] + [f"{n.names.plural_kebab}:TABLE" for n in S.nested]
        table = f"{S.names.plural_kebab} [{' '.join(cols)}] {{\n}}\n"
        ctx.plan.write(path, text.rstrip("\n") + "\n" + table)
        count += 1
    ctx.plan.note(f"DSL assets: {count} files get the empty table `{S.names.plural_kebab}`")


def run(ctx):
    ctx.layer("fixtures")
    run_json(ctx)
    ctx.layer("assets")
    run_assets(ctx)
