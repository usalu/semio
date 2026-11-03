"""🧬 Schema layer: the collection (or leaf) is written into the snapshot, artifact and diff JSON Schema documents; every twin and generated struct comes from `schema twins generate`."""
import copy, json, re
from core import SCHEMA, SNAPSHOT, DIFF, Names, Unsupported, camel, pascal
from ctx import append_item, append_member, get, json_edit, parse, put_after, span_edit


def record_def(ent, ctx, template):
    """🪲 The snapshot `$defs` record of an entity: fields in order, optional nested array property, Rust annotations copied from the template record."""
    props = {"id": {"type": "string"}}
    required = ["id"]
    for f in ent.fields:
        props[f.camel] = f.snapshot_node()
        required.append(f.camel)
    for n in ent.nested:
        props[camel(n.plural)] = {"type": "array", "items": {"$ref": "#/$defs/" + n.entity}, "x-semio-ui": {"label": dict(n.label)}, "x-semio-dsl": {"table": True}}
        required.append(camel(n.plural))
    rec = {"title": ent.entity, "type": "object", "additionalProperties": False, "required": required, "properties": props}
    if "x-semio-formats" in template:
        rec["x-semio-formats"] = list(template["x-semio-formats"])
    note = {k: copy.deepcopy(v) for k, v in template.get("x-semio-rust", {}).items() if k in ("derive", "attributes")}
    rec["x-semio-rust"] = {"doc": [f"{ent.emoji} {ent.doc}"], **note}
    return rec


def artifact_def(ent):
    props = {"id": {"type": "string"}}
    required = ["id"]
    for f in ent.fields:
        props[f.camel] = f.artifact_node()
        required.append(f.camel)
    for n in ent.nested:
        props[camel(n.plural)] = {"type": "array", "items": {"$ref": "#/$defs/" + n.entity}}
        required.append(camel(n.plural))
    return {"type": "object", "additionalProperties": False, "properties": props, "required": required}


def template_def(ctx, entity):
    rec = ctx.defs.get(entity)
    if rec is None:
        raise Unsupported("template record missing: " + entity)
    return rec


def run_documents(ctx):
    S, L = ctx.spec.collection, ctx.layout
    tmpl = template_def(ctx, ctx.ref.entity)
    ntmpl = template_def(ctx, ctx.nested_ref_names().entity) if S.nested else None

    def edit(text, builders, prop_from, required):
        root = parse(text)
        if root.get("properties").value.get(S.plural):
            return text
        defs = get(root, "$defs")
        for key, node in builders:
            if not defs.get(key):
                text = append_member(text, get(parse(text), "$defs"), key, node)
        root = parse(text)
        prop = copy.deepcopy(prop_from(text))
        prop["items"]["$ref"] = "#/$defs/" + S.entity
        text = append_member(text, get(root, "properties"), S.plural, prop)
        root = parse(text)
        if required and root.get("required") and ctx.ref.plural_camel in json.loads(text).get("required", []):
            text = append_item(text, root.get("required").value, S.plural)
        return text

    def snapshot(text):
        builders = [(f.enum_name, f.enum_def()) for f in S.all_enums] + [(n.entity, record_def(n, ctx, ntmpl)) for n in S.nested] + [(S.entity, record_def(S, ctx, tmpl))]
        return edit(text, builders, lambda t: json.loads(t)["properties"][ctx.ref.plural_camel], True)

    def artifact(text):
        builders = [(f.enum_name, f.enum_def()) for f in S.all_enums] + [(n.entity, artifact_def(n)) for n in S.nested] + [(S.entity, artifact_def(S))]
        return edit(text, builders, lambda t: json.loads(t)["properties"][ctx.ref.plural_camel], True)

    span_edit(ctx.plan, L.p(SNAPSHOT + "/🔣️.json"), snapshot)
    span_edit(ctx.plan, L.p(SCHEMA + "/🔣️.json"), artifact)
    ctx.plan.note(f"{S.plural}[] ({S.entity}" + "".join(f", {n.plural}[]" for n in S.nested) + ") in snapshot and artifact schema")


ROW_SUFFIX = r"(?:DeltaKey|Key|Patch|Rows|Delta|Inserted|Modified|Moved|Removed|Removal|Insertion|Relocation|Modification)"


def family_regex(ctx, ref=None, forms=None):
    """🪱 The names of the diff row group of a collection: `[En1997](Entity|PluralPascal)<suffix>` (`En1997LayerWatersRows`, `En1998JointDelta`)."""
    ref = ref or ctx.ref
    forms = forms or [ref.entity, ref.plural_pascal]
    return re.compile(r"^(En\d{4})?(%s)(%s)$" % ("|".join(re.escape(f) for f in sorted(set(forms), key=lambda f: -len(f))), ROW_SUFFIX))


def diff_names(ctx, plural_pascal=None, ref=None):
    """🪰 The `$defs` of the diff row group of the reference collection (either naming family)."""
    d = json.loads(ctx.plan.read(ctx.layout.p(DIFF + "/🔣️.json")))
    rx = family_regex(ctx, ref)
    return d, [k for k in d["$defs"] if rx.match(k)]


def patch_def(entity, fields, nested_lane, base, template=None):
    """🪯 The Patch def of a row type; with a template (the reference Patch) its property node shape and extra keys (`title`) are kept."""
    first = next(iter(template["properties"].values())) if template and template.get("properties") else None
    props = {}
    for f in fields:
        if first is not None:
            blob = json.dumps(first, ensure_ascii=False)
            blob = re.sub(r"/\$defs/\w+/properties/\w+", "/$defs/%s/properties/%s" % (entity, f.camel), blob)
            props[f.camel] = json.loads(blob)
        else:
            props[f.camel] = {"anyOf": [{"$ref": f"{base}#/$defs/{entity}/properties/{f.camel}"}, {"type": "null"}]}
    required = [f.camel for f in fields]
    if nested_lane:
        props[nested_lane[0]] = {"$ref": "#/$defs/" + nested_lane[1]}
        required.append(nested_lane[0])
    out = {}
    if template and "title" in template:
        out["title"] = None
    out.update({"type": "object", "additionalProperties": False})
    if not template or "required" in template:
        out["required"] = required
    out["properties"] = props
    return out


def detect_prefix(ctx):
    d, group = diff_names(ctx)
    if not group:
        raise Unsupported("no diff row group of the reference collection " + ctx.ref.plural_pascal)
    m = family_regex(ctx).match(group[0])
    return m.group(1) or ""


def rename_def_name(ctx, key, new_forms):
    m = family_regex(ctx).match(key)
    return (m.group(1) or "") + (new_forms[0] if m.group(2) == ctx.ref.entity else new_forms[1]) + m.group(3)


def run_diff(ctx):
    S, L = ctx.spec.collection, ctx.layout
    base = ctx.snapshot["$id"]
    d, group = diff_names(ctx)
    if not group:
        raise Unsupported("no diff row group of the reference collection " + ctx.ref.plural_pascal)
    prefix = detect_prefix(ctx)
    nref = ctx.nested_ref_names()
    ngroup = []
    if S.nested:
        nrx = family_regex(ctx, ctx.nref, [prefix + ctx.nref.plural_pascal + nref.plural_pascal])
        ngroup = [k for k in d["$defs"] if k.startswith(prefix + ctx.nref.plural_pascal + nref.plural_pascal)]
    forms = (S.entity, S.names.plural_pascal)
    lane_ref = lane_of(ctx, d)

    def mutate(text):
        data = json.loads(text)
        if S.plural in data["properties"]:
            return text
        defs = data["$defs"]
        new_defs = {}
        for key in group:
            if key.endswith("Patch"):
                continue
            new = rename_def_name(ctx, key, forms)
            blob = json.dumps(defs[key], ensure_ascii=False)
            blob = blob.replace("/$defs/" + ctx.ref.entity + '"', "/$defs/" + S.entity + '"')
            for old_name in group:
                blob = blob.replace(old_name + '"', rename_def_name(ctx, old_name, forms) + '"')
            new_defs[new] = json.loads(blob)
            if "title" in new_defs[new]:
                new_defs[new]["title"] = new
        nested_lane = None
        for n in S.nested:
            npl = prefix + S.names.plural_pascal + pascal(n.plural)
            for key in ngroup:
                if key.endswith("Patch"):
                    continue
                new = key.replace(prefix + ctx.nref.plural_pascal + nref.plural_pascal, npl)
                blob = json.dumps(defs[key], ensure_ascii=False).replace("/$defs/" + nref.entity, "/$defs/" + n.entity).replace(prefix + ctx.nref.plural_pascal + nref.plural_pascal, npl)
                new_defs[new] = json.loads(blob)
            new_defs[npl + "Patch"] = patch_def(n.entity, n.fields, None, base)
            nested_lane = (camel(n.plural), npl + "Rows")
        patch_key = next(k for k in group if k.endswith("Patch"))
        patch = patch_def(S.entity, S.fields, nested_lane, base, None if S.nested else defs[patch_key])
        new_patch = rename_def_name(ctx, patch_key, forms)
        if "title" in patch:
            patch["title"] = new_patch
        new_defs[new_patch] = patch
        for key, node in new_defs.items():
            text = append_member(text, get(parse(text), "$defs"), key, node)
        blob = json.dumps(data["properties"][lane_ref], ensure_ascii=False)
        for old_name in group:
            blob = blob.replace(old_name + '"', rename_def_name(ctx, old_name, forms) + '"')
        text = append_member(text, get(parse(text), "properties"), S.plural, json.loads(blob))
        return text

    span_edit(ctx.plan, L.p(DIFF + "/🔣️.json"), mutate)
    ctx.plan.note(f"diff lane {S.plural} and row groups {', '.join(sorted({rename_def_name(ctx, k, forms) for k in group}))}")
    return prefix


def lane_of(ctx, d):
    """🪮 The diff root property that carries the reference collection (named like the collection)."""
    if ctx.ref.plural_camel not in d["properties"]:
        raise Unsupported(f"diff lane {ctx.ref.plural_camel} not found in the diff schema")
    return ctx.ref.plural_camel


def rust_lane_block(ctx, snake_name):
    """🧱 The `pub name: Type,` lines (with their attribute lines) that surround the new lane field of the Rust diff root, as [(snake name, type)]."""
    import l_rust
    L = ctx.layout
    paths = [L.p(DIFF + "/🦀️.rs")] + list(l_rust.diff_tree_files(ctx))
    field = re.compile(r"^\s+pub (\w+):\s*([^,\n]+),\s*$")
    for path in paths:
        if not ctx.plan.exists(path):
            continue
        lines = ctx.plan.read(path).splitlines()
        for i, line in enumerate(lines):
            m = field.match(line)
            if not m or m.group(1) != snake_name:
                continue
            block, ok = [], lambda s: field.match(s) or s.lstrip().startswith(("#[", "///"))
            a = i
            while a > 0 and ok(lines[a - 1]):
                a -= 1
            b = i
            while b + 1 < len(lines) and ok(lines[b + 1]):
                b += 1
            return [(field.match(s).group(1), field.match(s).group(2)) for s in lines[a:b + 1] if field.match(s)]
    return None


def sync_diff_lanes(ctx):
    """🧭 The diff JSON Schema lists the new lane where the Rust diff root declares it and in `required` when the Rust lane is not an `Option` (schema-sync compares order and requiredness)."""
    S = ctx.spec.collection
    block = rust_lane_block(ctx, S.names.plural_snake)
    if not block:
        return
    order = [(camel(n), not t.startswith("Option<")) for n, t in block]
    key = S.plural
    path = ctx.layout.p(DIFF + "/🔣️.json")

    def move(text, node, key, after, name):
        names = [m.key for m in node.members] if node.kind == "object" else [json.loads(text[i.start:i.end]) for i in node.items]
        if key not in names:
            return text
        at = names.index(key)
        if after is None or (after in names and names.index(after) + 1 == at):
            return text
        if node.kind == "object":
            member = node.members[at]
            rendered, prev_end, end = text[member.value.start:member.value.end], node.members[at - 1].value.end, member.value.end
        else:
            rendered, prev_end, end = text[node.items[at].start:node.items[at].end], node.items[at - 1].end, node.items[at].end
        text = text[:prev_end] + text[end:]
        again = parse(text)
        node = get_node(again, name)
        if node.kind == "object":
            return jsonspan_insert_member(text, node, after, key, rendered)
        return jsonspan_insert_item(text, node, [json.loads(text[i.start:i.end]) for i in node.items].index(after), rendered)

    def mutate(text):
        root = parse(text)
        props = root.get("properties").value
        if props.get(key) is None:
            return text
        i = [n for n, _ in order].index(key)
        pred = next((n for n, _ in reversed(order[:i]) if props.get(n)), None)
        text = move(text, props, key, pred, "properties")
        root = parse(text)
        req = root.get("required")
        if req is not None and req.value.kind == "array":
            have = [json.loads(text[x.start:x.end]) for x in req.value.items]
            if order[i][1] and ctx.ref.plural_camel in have and key not in have:
                text = jsonspan_insert_item(text, req.value, have.index(ctx.ref.plural_camel), json.dumps(key))
                root = parse(text)
                req = root.get("required")
                have = [json.loads(text[x.start:x.end]) for x in req.value.items]
            rpred = next((n for n, r in reversed(order[:i]) if r and n in have), None)
            text = move(text, req.value, key, rpred, "required")
        return text

    span_edit(ctx.plan, path, mutate)


def get_node(root, name):
    return root.get(name).value


def jsonspan_insert_member(text, node, after, key, rendered):
    import jsonspan
    return jsonspan.insert_member(text, node, after, key, rendered)


def jsonspan_insert_item(text, node, index, rendered):
    import jsonspan
    return jsonspan.insert_item(text, node, index, rendered)
