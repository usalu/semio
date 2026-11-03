"""🧬 Mutation layer: insert/remove kinds of the new collection (and its nested rows) are cloned from the reference sibling's kinds (crates, registries, fixtures, binary protocol); set-field kinds are produced by the artifact's leaf-kinds driver after the twins exist."""
import copy, json, os, re
from core import NORM, SKIP_DIRS, Names, Unsupported, balanced_end, camel, kebab, pascal, snake, walk_files
import jsonspan as js
from ctx import append_item, dumps, parse
from spec import Entity

MUT = "🧬️schema/🧬️mutations"
SUFFIXES = (".rs", ".py", ".feature", ".semio", ".json")
SKIP_FILES = ("🟦️.ts", "🔗️.graphql", "🛰️.proto")


def slug_pair(ref_slug, new_slug):
    """🔝 The short spelling of an entity as the kinds name it (`joint`, `layer-water`), as a pair of Names."""
    from core import plural_of
    return (Names(camel(plural_of(ref_slug)), pascal(ref_slug)), Names(camel(plural_of(new_slug)), pascal(new_slug)))


def kind_names(prefix, slug):
    return {"k": f"{prefix}-{slug}", "s": f"{prefix}_{slug.replace('-', '_')}", "P": pascal(f"{prefix}-{slug}"), "c": camel(f"{prefix}-{slug}"), "t": f"{prefix.capitalize()} {slug.replace('-', ' ').title()}"}


def rename_map(prefix, ref_slug, new_slug, ent_pairs):
    """🔜 Ordered (old, new) text pairs: kind forms first, then the entity forms."""
    a, b = kind_names(prefix, ref_slug), kind_names(prefix, new_slug)
    pairs = [(a[k], b[k]) for k in a]
    pairs += ent_pairs
    return sorted(set(pairs), key=lambda kv: -len(kv[0]))


class Tr:
    """🔛 Translation of cloned mutation text: kind spellings by plain replacement, then the word-aligned entity renames (reference -> new names, also the short slug spelling `joint` of an artifact-prefixed entity)."""

    def __init__(self, kind_pairs, name_pairs, evidence=None):
        self.kind_pairs, self.name_pairs, self.evidence = sorted(set(kind_pairs), key=lambda kv: -len(kv[0])), name_pairs, evidence

    def __call__(self, text, hint=None):
        held = {}
        for a, b in self.kind_pairs:
            if a == b:
                continue
            mark = held.setdefault(b, f"\x00{len(held)}\x00")
            text = text.replace(a, mark)
        for ref, new in self.name_pairs:
            text = ref.rename(text, new, '"aB"' if hint in ("json", "path") else self.evidence, "camel" if hint == "path" else "snake")
        for b, mark in held.items():
            text = text.replace(mark, b)
        return text


def apply_pairs(text, pairs, hint=None):
    if callable(pairs):
        return pairs(text, hint)
    for a, b in pairs:
        text = text.replace(a, b)
    return text


def find_dir(ctx, kind):
    root = ctx.layout.p(MUT)
    hits = [d for d in os.listdir(root) if d.endswith(kind) and os.path.isdir(os.path.join(root, d)) and re.fullmatch(r"\W*" + re.escape(kind), d)]
    if not hits:
        raise Unsupported(f"no mutation directory for the reference kind {kind}")
    return os.path.join(root, hits[0]), hits[0]


def tree_files(path):
    for dp, ds, fs in os.walk(path):
        ds[:] = [d for d in ds if d not in SKIP_DIRS]
        for f in sorted(fs):
            yield os.path.join(dp, f)


def mention_units(text, tokens):
    """🔚 Spans (start, end) of the statements that mention a token, merged over consecutive mention lines and extended to bracket balance."""
    lines = text.splitlines(True)
    pos, starts = 0, []
    for line in lines:
        if any(t in line for t in tokens):
            starts.append((pos, pos + len(line)))
        pos += len(line)
    spans = []
    for s, e in starts:
        if spans and s < spans[-1][1]:
            continue
        end = e
        if spans and s == spans[-1][1] and end:
            spans[-1] = (spans[-1][0], max(spans[-1][1], balanced_end(text, s), e))
            continue
        spans.append((s, max(e, balanced_end(text, s))))
    return spans


def clone_registry(ctx, path, tokens, pairs, labels):
    text = ctx.plan.read(path)
    spans = mention_units(text, tokens)
    if not spans:
        return
    out, last = [], 0
    for s, e in spans:
        chunk = text[s:e]
        new = apply_pairs(chunk, pairs)
        out.append(text[last:e])
        if new not in text:
            out.append(new)
        last = e
    out.append(text[last:])
    ctx.plan.write(path, "".join(out))


def registry_files(ctx, tokens):
    L = ctx.layout
    for path in walk_files(L.any, SUFFIXES):
        rel = path[len(L.any) + 1:]
        if "🧫️fixtures" in rel or rel.startswith(MUT + "/") and rel.count("/") > MUT.count("/") + 1 and not rel.endswith("🦀️.rs") or os.path.basename(path) in SKIP_FILES or "🧪️tests/🔬️unit" in rel or "/🧪️tests/" in rel and rel.startswith("🚪️io"):
            continue
        if rel.startswith("🧬️schema/🧬️mutations/") and os.path.basename(os.path.dirname(path)) != "🧬️mutations" and not path.endswith(("🦀️.rs",)):
            continue
        try:
            text = open(path, encoding="utf8").read()
        except (UnicodeDecodeError, OSError):
            continue
        if any(t in text for t in tokens):
            yield path


def json_registry(ctx, path, tokens, pairs):
    text = ctx.plan.read(path)
    root = parse(text)

    def walk(node):
        found = []
        if node.kind == "array":
            for i, item in enumerate(node.items):
                if any(t in text[item.start:item.end] for t in tokens):
                    deeper = walk(item)
                    found += deeper if deeper else [(node, i)]
        elif node.kind == "object":
            for m in node.members:
                found += walk(m.value)
        return found
    hits = walk(root)
    new_text = text
    for node, i in reversed(hits):
        chunk = text[node.items[i].start:node.items[i].end]
        new = apply_pairs(chunk, pairs, "json")
        if new in text:
            continue
        sep = js._separator(text, node, i)
        new_text = new_text[: node.items[i].end] + sep + new + new_text[node.items[i].end:]
    ctx.plan.write(path, new_text)


def ui_nodes(doc):
    """🏷️ The `x-semio-ui` objects of a schema document with their property key."""
    out = []

    def walk(o, key=None):
        if isinstance(o, dict):
            if isinstance(o.get("x-semio-ui"), dict):
                out.append((key, o["x-semio-ui"]))
            for k, v in o.items():
                walk(v, k)
        elif isinstance(o, list):
            for v in o:
                walk(v, key)
    walk(doc)
    return out


def reference_labels(src_doc):
    """🔖 The EN + DE labels the reference kind carries for its entity (the payload record of insert kinds, the `id` selector of the other kinds) and for the changed field (`new*` value)."""
    entity, field, index = None, None, None
    props = src_doc.get("properties", {})
    for key, node in props.items():
        label = (node.get("x-semio-ui") or {}).get("label")
        if not label:
            continue
        pair = (label["en"], label["de"])
        if "$ref" in node:
            entity = pair
        elif key == "id" and entity is None:
            entity = pair
        elif key == "index" and pair[0] != "Position":
            index = pair
        elif key.startswith("new"):
            field = pair
    entity = entity or index
    return entity, field


def swap_text(text, old, new, lower=True):
    """🔁 Replaces `old` (any case of its first letter) by `new`: a lower-case first letter stays lower inside a sentence."""
    def fix(m):
        if m.start() == 0 or not lower or not m.group(0)[:1].islower():
            return new[:1].upper() + new[1:] if m.group(0)[:1].isupper() else new
        return new[:1].lower() + new[1:]
    return re.sub(re.escape(old), fix, text, flags=re.I)


GENERIC_LABELS = {"position"}


def relabel_doc(doc, pairs):
    """🏷️ Every label and description string of the kind schema takes the spec's EN + DE entity and field labels; strings are matched by the reference label."""
    for key, ui in ui_nodes(doc):
        if key == "index" and (ui.get("label") or {}).get("en", "").lower() in GENERIC_LABELS:
            continue
        for part in ("label", "description"):
            node = ui.get(part)
            if not isinstance(node, dict):
                continue
            for lang in ("en", "de"):
                text = node.get(lang)
                if not isinstance(text, str):
                    continue
                for (old_en, old_de), (new_en, new_de) in pairs:
                    old, new = (old_en, new_en) if lang == "en" else (old_de, new_de)
                    if old and old.lower() in text.lower():
                        text = swap_text(text, old, new, lower=lang == "en")
                node[lang] = text


def prose_pairs(ctx, src, ent, field):
    """🔖 Reference label -> spec label pairs of a kind directory, from its payload schema."""
    path = os.path.join(src, "🧬️schema/🔣️.json")
    if ent is None or not os.path.exists(path):
        return []
    ref_entity, ref_field = reference_labels(json.loads(ctx.plan.read(path)))
    pairs = []
    if ref_entity:
        pairs.append((ref_entity, (ent.singular_en, ent.singular_de)))
    if ref_field and field is not None:
        pairs.append((ref_field, (field.en, field.de)))
    return pairs


def relabel_prose(text, pairs):
    """📝 Reference labels inside comments and string literals of a cloned Rust file take the spec's labels."""
    def fix(m):
        body = m.group(0)
        if re.fullmatch(r'"[a-z0-9._:-]+"', body):
            return body
        for (old_en, old_de), (new_en, new_de) in pairs:
            for old, new, lower in ((old_en, new_en, True), (old_de, new_de, False)):
                if old and old.lower() not in GENERIC_LABELS and old.lower() in body.lower():
                    body = swap_text(body, old, new, lower)
        return body
    return re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', fix, text)


def leftover_label(text, pairs, own):
    for (old_en, old_de), _ in pairs:
        for old in (old_en, old_de):
            if old and old.lower() not in own and old.lower() not in GENERIC_LABELS and re.search(r"(?<!\w)%s(?!\w)" % re.escape(old), text, re.I):
                return old
    return None


def clone_kind(ctx, ref_kind, new_kind, pairs, labels, tag, ctx_names=None, field=None):
    src, name = find_dir(ctx, ref_kind)
    ref_pairs = prose_pairs(ctx, src, ctx_names, field)
    own = {s.lower() for _, new in ref_pairs for s in new}
    emoji = name[: len(name) - len(ref_kind)]
    dst = os.path.join(os.path.dirname(src), emoji + new_kind)
    for f in tree_files(src):
        rel = os.path.relpath(f, src)
        text = open(f, encoding="utf8").read()
        new = apply_pairs(apply_labels(text, labels), pairs, "json" if f.endswith(".json") else None)
        if f.endswith(".rs"):
            new = prose(new, ctx_names)
        if rel == "🔣️.json":
            d = json.loads(new)
            d["binaryTag"] = tag
            new = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
        if f.endswith("🧬️schema/🔣️.json") and ctx_names is not None:
            new = relabel_kind(ctx, text, new, ctx_names, field, new_kind)
        elif f.endswith(".rs") and ref_pairs:
            new = relabel_prose(new, ref_pairs)
        if ref_pairs and (f.endswith(".rs") or f.endswith(".feature")):
            left = leftover_label(" ".join(t for t in re.findall(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', new) if not re.fullmatch(r'"[a-z0-9._:-]+"', t)), ref_pairs, own)
            if left:
                raise Unsupported(f"{new_kind}: the label {left!r} of the reference kind remains in {rel} after relabelling")
        ctx.plan.write(os.path.join(dst, rel), new)
    return emoji + new_kind


def relabel_kind(ctx, source_text, new_text, ent, field, new_kind):
    """🏷️ The cloned payload schema: reference entity and field labels become the spec's labels; any reference label left over aborts the layer."""
    ref_entity, ref_field = reference_labels(json.loads(source_text))
    pairs = []
    if ref_entity:
        pairs.append((ref_entity, (ent.singular_en, ent.singular_de)))
    if ref_field and field is not None:
        pairs.append((ref_field, (field.en, field.de)))
    doc = json.loads(new_text)
    relabel_doc(doc, pairs)
    out = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
    own = {s.lower() for _, new in pairs for s in new}
    shown = [text for key, ui in ui_nodes(doc) if not (key == "index" and (ui.get("label") or {}).get("en", "").lower() in GENERIC_LABELS) for part in ("label", "description") if isinstance(ui.get(part), dict) for text in ui[part].values() if isinstance(text, str)]
    for (old_en, old_de), _ in pairs:
        for old in (old_en, old_de):
            if old and old.lower() not in own and old.lower() not in GENERIC_LABELS and any(re.search(r"(?<!\w)%s(?!\w)" % re.escape(old), text, re.I) for text in shown):
                raise Unsupported(f"{new_kind}: the label {old!r} of the reference kind remains in the cloned schema after relabelling")
    return out


PROSE = re.compile(r'"((?:[^"\\]|\\.)*(?:already exists|past the end of|does not exist|not found|duplicate|missing)(?:[^"\\]|\\.)*)"')


def prose(text, entity):
    """🔙 Error prose of a cloned kind names the new entity by its singular EN label (`probe_row {key} already exists.` becomes `Tie beam {key} already exists.`)."""
    names = entity.names
    forms = sorted({names.entity, names.camel, names.snake, names.kebab.replace("-", " ")} - {""}, key=lambda x: -len(x))

    def fix(m):
        body = m.group(1)
        for form in forms:
            body = re.sub(r"(?<![\w{])%s(?![\w])" % re.escape(form), lambda mm: entity.singular_en[:1].upper() + entity.singular_en[1:] if mm.start() == 0 else entity.singular_en, body)
        return '"' + body + '"'
    return PROSE.sub(fix, text)


def apply_labels(text, labels):
    for (old_en, old_de), (en, de) in labels:
        text = text.replace(old_en, en).replace(old_de, de)
    return text


def labels_of(ctx, src_dir):
    out = []
    for f in tree_files(src_dir):
        out += re.findall(r'native\("([^"]+)", "([^"]+)"\)', open(f, encoding="utf8").read())
    return out[:1]


def next_tags(ctx, count):
    top = -1
    root = ctx.layout.p(MUT)
    for d in os.listdir(root):
        p = os.path.join(root, d, "🔣️.json")
        if os.path.isfile(p):
            try:
                top = max(top, json.load(open(p, encoding="utf8")).get("binaryTag", -1))
            except json.JSONDecodeError:
                pass
    for path, (old, new, _) in ctx.plan.files.items():
        if path.endswith("🔣️.json") and "/🧬️mutations/" in path and old is None:
            try:
                top = max(top, json.loads(new).get("binaryTag", -1))
            except (json.JSONDecodeError, AttributeError):
                pass
    return list(range(top + 1, top + 1 + count))


def protocol_clone(ctx, ref_kinds, new_kinds, tags):
    for path in walk_files(ctx.layout.p("🚪️io/💾️binary/🧬️mutations"), (".protocol.semio",)):
        text = ctx.plan.read(path)
        if new_kinds[0] in text:
            continue
        add = ""
        for ref, new, tag in zip(ref_kinds, new_kinds, tags):
            m = re.search(r"^record %s tag=\d+\n(?:(?!record )[^\n]*\n)*" % re.escape(ref), text, re.M)
            if not m:
                raise Unsupported(f"binary protocol record of {ref} not found")
            add += re.sub(r"tag=\d+", f"tag={tag}", m.group(0).replace(ref, new))
        ctx.plan.write(path, text.rstrip("\n") + "\n" + add)


def fixtures_clone(ctx, ref_kind, new_kind, new_dir_name, pairs, row_fn):
    root = ctx.layout.p("🧪️tests/🧫️fixtures/🧬️mutations")
    src = [d for d in os.listdir(root) if d.endswith(ref_kind)]
    if not src:
        return
    base = os.path.join(root, src[0])
    emoji = src[0][: len(src[0]) - len(ref_kind)]
    for f in tree_files(base):
        rel = os.path.relpath(f, base)
        text = open(f, encoding="utf8").read()
        ctx.plan.write(os.path.join(root, emoji + new_kind, rel), row_fn(rel, text))


def rule_kinds(ctx, path_camel):
    """🔘 Insert and remove kind of a collection path, read from the edit rules (`InsertItemRule { path: "layerWaters", kind: .. }`, `RemoveItemRule` or `IdRemoval`)."""
    rules = "".join(ctx.plan.read(p) for p in walk_files(ctx.layout.p(MUT + "/🧭️edit-rules"), (".rs",)))
    ins = re.search(r'\w*Insert\w* \{ path: "%s", kind: "(insert-[a-z0-9-]+)"' % re.escape(path_camel), rules)
    rem = re.search(r'\w*Remov\w* \{ path: "%s", kind: "(remove-[a-z0-9-]+)"' % re.escape(path_camel), rules)
    if not ins or not rem:
        raise Unsupported(f"edit rules of {path_camel} have no insert/remove kinds to clone")
    return ins.group(1), rem.group(1)


def registry_candidates(ctx):
    L = ctx.layout
    for path in walk_files(L.any, SUFFIXES):
        rel = path[len(L.any) + 1:]
        if os.path.basename(path) in SKIP_FILES or "🧫️fixtures" in rel or "/🧪️tests/🔬️unit" in rel or "🧪️tests/🔬️unit" in rel:
            continue
        top = rel.startswith((MUT + "/🦀️.rs", MUT + "/🔣️.json", MUT + "/🧭️edit-rules/")) or re.match(r"🧪️tests/[^/]*mutate-%s-\d+/" % ctx.short, rel) or rel == "🔮️oracles/🔣️.json" or rel == "🚪️io/📝️text/🧬️mutations/🦀️.rs"
        group = rel.startswith(MUT + "/") and rel.count("/") == MUT.count("/") + 2 and rel.endswith("/🦀️.rs") and not os.path.exists(os.path.join(os.path.dirname(path), "🔣️.json"))
        if top or group:
            yield path


def list_item_clone(text, tokens, pairs):
    """🔖 `use path::{a, b, c};` lists: the renamed copy of every item that mentions a token is appended inside the same braces."""
    def fix(m):
        items = [x.strip() for x in m.group(2).split(",") if x.strip()]
        add = [apply_pairs(x, pairs) for x in items if any(t in x for t in tokens)]
        add = [x for x in add if x not in items]
        return m.group(1) + ", ".join(items + add) + m.group(3) if add else m.group(0)
    return re.sub(r"(^\s*(?:pub )?use [^{\n]*\{)([^}\n]*)(\};)", fix, text, flags=re.M)


def clone_registries(ctx, tokens, pairs):
    for path in registry_candidates(ctx):
        text = ctx.plan.read(path)
        if not any(t in text for t in tokens):
            continue
        if path.endswith(".json"):
            json_registry(ctx, path, tokens, pairs)
            continue
        text = list_item_clone(text, tokens, pairs)
        ctx.plan.write(path, text)
        lines_only = [l for l in text.splitlines(True)]
        spans = []
        for s, e in mention_units(text, tokens):
            line = text[s:text.find("\n", s) + 1]
            if re.match(r"\s*(?:pub )?use [^{\n]*\{[^}\n]*\};", line):
                continue
            spans.append((s, e))
        out, last = [], 0
        for s, e in spans:
            chunk = text[s:e]
            new = apply_pairs(chunk, pairs, "path" if "/🧭️edit-rules/" in path else None)
            if "/🧭️edit-rules/" in path:
                new = re.sub(r'\b(path|item): "([^"]*)"', lambda m: f'{m.group(1)}: "' + re.sub(r"_([a-z0-9])", lambda g: g.group(1).upper(), m.group(2)) + '"', new)
            out.append(text[last:e])
            if new not in text:
                out.append(new)
            last = e
        out.append(text[last:])
        ctx.plan.write(path, "".join(out))


def sample_transformer(ctx, matchers, kind_pairs):
    """📯 JSON transformer for committed fixtures: a ref row (matched by key set) becomes the new sample row (nested rows transformed recursively), ids are mapped in order of appearance, keys and strings are renamed."""
    ids = {}

    def new_id(old, prefix):
        return ids.setdefault(old, f"{prefix}-{sum(1 for v in ids.values() if v.startswith(prefix + '-')) + 1}")

    def go(node):
        if isinstance(node, dict):
            keys = frozenset(node)
            for ks, ent, nested_keys in matchers:
                if keys == ks:
                    row = copy.deepcopy(ent.sample_row())
                    row["id"] = new_id(node["id"], ent.names.kebab)
                    for n, old in zip(ent.nested, nested_keys):
                        row[camel(n.plural)] = [go(x) for x in node[old]]
                    return {k: row[k] for k in ["id"] + [f.camel for f in ent.fields] + [camel(n.plural) for n in ent.nested]}
            return {apply_pairs(k, kind_pairs, "json"): go(v) for k, v in node.items()}
        if isinstance(node, list):
            return [go(v) for v in node]
        if isinstance(node, str):
            return ids.get(node) or apply_pairs(node, kind_pairs, "json")
        return node
    return go


def fixture_family(ctx, ref_kind, new_kind, matchers, pairs, ref_lane, new_lane, lane_null):
    """🧪 Vectors of a cloned insert/remove kind: before/after are the artifact's sample snapshot with the transformed rows of the donor vector in the new lane, the diff touches the new lane only, mutation and outcome are the transformed donor documents."""
    import l_basedoc
    root = ctx.layout.p("🧪️tests/🧫️fixtures/🧬️mutations")
    src = [d for d in os.listdir(root) if d.endswith(ref_kind) and re.fullmatch(r"\W*" + re.escape(ref_kind), d)]
    if not src:
        return
    base = os.path.join(root, src[0])
    dst = os.path.join(root, src[0][: len(src[0]) - len(ref_kind)] + new_kind)
    tf = sample_transformer(ctx, matchers, pairs)
    for f in sorted(tree_files(base), key=lambda p: (0 if p.endswith("🔣️.before.json") else 1, p)):
        rel = os.path.relpath(f, base)
        obj = json.loads(open(f, encoding="utf8").read())
        name = os.path.basename(f)
        if name in ("🔣️.before.json", "🔣️.after.json") and isinstance(obj, dict) and ref_lane in obj:
            obj = l_basedoc.with_rows(ctx, new_lane, tf(obj[ref_lane]))
        elif name == "🔣️.diff.json" and isinstance(obj, dict) and ref_lane in obj:
            obj = l_basedoc.diff_of(obj, new_lane, tf(obj[ref_lane]))
        else:
            obj = tf(obj)
        ctx.plan.write(os.path.join(dst, rel), json.dumps(obj, indent=2, ensure_ascii=False) + "\n")


def ref_entity_info(ctx, names, entity_name):
    d = ctx.defs[entity_name]
    nested = [n for n, p in d["properties"].items() if p.get("type") == "array"]
    fields = [n for n, p in d["properties"].items() if n != "id" and n not in nested]
    return {"fields": fields, "nested": nested}


TAXONOMY = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"


def taxonomy_clone(ctx, rows):
    """🗂️ Taxonomy rows (with the `edit` metadata the fixture checker and the oracle engine read) of the cloned insert/remove kinds, derived from the reference kinds' rows; idempotent."""
    taxonomy = json.loads(ctx.plan.read(TAXONOMY))
    artifact = next(r["artifact"] for r in taxonomy["rows"] if r["artifact"].endswith(ctx.short))
    have = {r["kind"] for r in taxonomy["rows"] if r["artifact"] == artifact}
    new = []
    for ref_kind, new_kind, dst, pairs, path in rows:
        if new_kind in have:
            continue
        ref = next((r for r in taxonomy["rows"] if r["artifact"] == artifact and r["kind"] == ref_kind), None)
        fallback = ref is None
        if fallback:
            verb = ref_kind.split("-", 1)[0]
            depth = path.count("[]")
            same = [r for r in taxonomy["rows"] if r["artifact"] == artifact and r["kind"].startswith(verb + "-") and r.get("edit", {}).get("path", "").count("[]") == depth]
            if not same:
                raise Unsupported(f"the mutation taxonomy has no {verb} row of nesting depth {depth} in {artifact} to derive the rows of {new_kind} from")
            ref = same[-1]
            ctx.plan.note(f"taxonomy: no row for the reference kind {ref_kind}; the row of {new_kind} derives from {ref['kind']}")
        row = json.loads(pairs(json.dumps(ref, ensure_ascii=False), "json")) if not fallback else copy.deepcopy(ref)
        row["edit"] = {**row.get("edit", {}), "path": path, "item": new_kind.split("-", 1)[1]}
        prefix, slug = new_kind.split("-", 1)
        row.update({"kind": new_kind, "module": dst, "entity": slug, "aggregateVariant": pascal(new_kind), "type": pascal(new_kind), "record": ("Inserted" if prefix == "insert" else "Removed") + pascal(slug), "source": ref["source"].replace(ref["module"], dst)})
        new.append(row)
    if not new:
        return
    last = max(i for i, r in enumerate(taxonomy["rows"]) if r["artifact"] == artifact)
    new.sort(key=lambda r: r["module"])
    taxonomy["rows"][last + 1: last + 1] = new
    ctx.plan.write(TAXONOMY, json.dumps(taxonomy, indent=2, ensure_ascii=False) + "\n")


def clone_family(ctx):
    S, L = ctx.spec.collection, ctx.layout
    nested = bool(S.nested) and not getattr(ctx, "skip_nested_kinds", False)
    families = [(ctx.ref, S, ctx.ref.plural_camel, None)]
    if nested:
        nref = ctx.nested_ref_names()
        families.append((nref, S.nested[0], f"{ctx.nref.plural_camel}[].{nref.plural_camel}", (ctx.nref, S)))
    tags = next_tags(ctx, 2 * len(families))
    ti = 0
    all_ref_kinds, all_new_kinds, all_tags, taxonomy_rows = [], [], [], []
    name_pairs = [(ctx.ref, S.names)]
    extra = []
    if nested:
        name_pairs += [(ctx.nref, S.names), (nref, S.nested[0].names)]
        extra += [(f"{ctx.nref.snake}_id", f"{S.names.snake}_id"), (f"{ctx.nref.camel}Id", f"{S.names.camel}Id")]
    for ref_names, ent, path, parent in families:
        ins_ref, rem_ref = rule_kinds(ctx, path)
        slug_new = ent.raw.get("kindSlug", ent.names.kebab)
        for ref_kind in (ins_ref, rem_ref):
            prefix, ref_slug = ref_kind.split("-", 1)
            new_kind = f"{prefix}-{slug_new}"
            kp = rename_map("insert", ref_slug, slug_new, []) + rename_map("remove", ref_slug, slug_new, [])
            pairs = Tr(kp + extra, name_pairs + [slug_pair(ref_slug, slug_new)])
            src_dir, name = find_dir(ctx, ref_kind)
            labels = [(lab, (f"Insert {ent.singular_en}" if prefix == "insert" else f"Remove {ent.singular_en}", f"{ent.singular_de} einfügen" if prefix == "insert" else f"{ent.singular_de} entfernen")) for lab in labels_of(ctx, src_dir)]
            dst = clone_kind(ctx, ref_kind, new_kind, pairs, labels, tags[ti], ent)
            ctx.__dict__.setdefault("text_pairs", []).append((ref_kind, new_kind, pairs))
            taxonomy_rows.append((ref_kind, new_kind, dst, pairs, S.names.plural_camel if not parent else f"{S.names.plural_camel}[].{ent.names.plural_camel}"))
            ref_row = ref_entity_info(ctx, ref_names, ctx.nref.entity if parent else ctx.ref.entity)
            matchers = [(frozenset(["id"] + [camel(x) for x in ref_row["fields"]] + ref_row["nested"]), S if parent else S, ref_row["nested"])]
            if parent:
                nrow = ref_entity_info(ctx, ref_names, nref.entity)
                matchers.append((frozenset(["id"] + [camel(x) for x in nrow["fields"]] + nrow["nested"]), ent, nrow["nested"]))
            lane_null = None
            fixture_family(ctx, ref_kind, new_kind, matchers, pairs, (ctx.nref if parent else ctx.ref).plural_camel, S.names.plural_camel, lane_null)
            all_ref_kinds.append(ref_kind)
            all_new_kinds.append(new_kind)
            all_tags.append(tags[ti])
            ti += 1
    tokens = []
    for k in all_ref_kinds:
        prefix, slug = k.split("-", 1)
        tokens += [k, k.replace("-", "_"), pascal(k), camel(k)]
    full_pairs = []
    for k, nk in zip(all_ref_kinds, all_new_kinds):
        prefix, ref_slug = k.split("-", 1)
        full_pairs += rename_map(prefix, ref_slug, nk.split("-", 1)[1], [])
    ref_slug0 = all_ref_kinds[0].split("-", 1)[1]
    new_slug0 = all_new_kinds[0].split("-", 1)[1]
    clone_registries(ctx, tokens, Tr(full_pairs + extra, name_pairs + [slug_pair(ref_slug0, new_slug0)]))
    for path in ctx.layout.find("🚪️io/📝️text/🧬️mutations/🦀️.rs"):
        text = ctx.plan.read(path)
        names = [S.entity] + [n.entity for n in S.nested]
        text = re.sub(r"(use %s::\{)([^}]*)(\};)" % re.escape(ctx.layout.model_crate), lambda m: m.group(1) + m.group(2) + "".join(", " + n for n in names if not re.search(r"\b%s\b" % n, m.group(2))) + m.group(3), text, count=1)
        ctx.plan.write(path, text)
    protocol_clone(ctx, all_ref_kinds, all_new_kinds, all_tags)
    taxonomy_clone(ctx, taxonomy_rows)
    ctx.plan.note(f"mutation clone: {', '.join(all_new_kinds)} with binary tags {all_tags}")


def run(ctx):
    S = ctx.spec.collection
    ref_info = ctx.ref
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', ctx.plan.read(ctx.layout.p(MUT + "/🦀️.rs")), re.M))
    slugs = [(S, S.raw.get("kindSlug", S.names.kebab))] + [(n, n.raw.get("kindSlug", n.names.kebab)) for n in S.nested]
    if S.nested and ctx.nref:
        try:
            rule_kinds(ctx, f"{ctx.nref.plural_camel}[].{ctx.nested_ref_names().plural_camel}")
        except Unsupported:
            ctx.skip_nested_kinds = True
            slugs = slugs[:1]
            ctx.plan.note(f"mutations: the reference parent {ctx.nref.plural_camel} edits its nested rows as a whole array (no insert/remove kinds of its own), so {S.nested[0].plural} get none either")
    planned = [f"{p}-{slug}" for _, slug in slugs for p in ("insert", "remove")]
    set_kinds = [f"{ctx.spec.kind_prefix}-{slug}-{f.name.replace('_', '-')}" for ent, slug in slugs for f in ent.fields if f.settable]
    present = [k for k in planned if k in existing]
    if present and len(present) != len(planned):
        raise Unsupported("only some insert/remove kinds exist already: " + ", ".join(present))
    set_kinds = [k for k in set_kinds if k not in existing]
    ctx.kinds = {"clone": planned, "set": set_kinds}
    if not present:
        clone_family(ctx)
    import l_setfield
    made = []
    if ctx.short in DRIVERS:
        ctx.plan.note(f"set-field kinds for the leaf-kinds driver {driver_of(ctx.short)} after the twins exist: {len(set_kinds)}" + (f" ({set_kinds[0]} ...)" if set_kinds else ""))
    else:
        for ent, slug in slugs:
            if ent is not S:
                continue
            for f in ent.fields:
                if f.settable:
                    new = l_setfield.clone_setfield(ctx, ent, S.names.plural_camel, f, slug)
                    if new:
                        made.append(new)
        ctx.kinds["set"] = []
    crate = l_setfield.isolate_crate(ctx, ([] if present else planned) + made)
    if not crate and (made or not present):
        ctx.plan.note("mutation crates: the kinds stay in the group crates of their reference kinds (this artifact mounts its kinds without group crates)")
    import l_text
    l_text.run(ctx, crate)
    if ctx.short in DRIVERS:
        return
    bump_mirror_counts(ctx, (0 if present else len(planned)) + len(made))
    ctx.plan.note(f"set-field kinds by the clone driver {driver_of(ctx.short)} from existing kinds of the artifact: {', '.join(made) or 'none new'}")


def bump_mirror_counts(ctx, added):
    """🔢 The prose of the exhaustive mutation mirror names the number of kinds (`631-kind`, `The 631 ✅apply vectors`); it grows with every cloned kind."""
    if not added:
        return
    for path in walk_files(ctx.layout.p("🧪️tests"), (".rs", ".feature", ".py")):
        if not re.search(r"mutate-%s-\d+/" % ctx.short, path):
            continue
        text = ctx.plan.read(path)
        found = set(re.findall(r"\b(\d+)-kind\b|\bThe (\d+) `✅apply`", text))
        nums = {n for pair in found for n in pair if n}
        if len(nums) != 1:
            continue
        old = int(next(iter(nums)))
        new = re.sub(r"\b%d(-kind\b| `✅apply`)" % old, lambda m: f"{old + added}{m.group(1)}", text)
        if new != text:
            ctx.plan.write(path, new)


DRIVERS = {"en1997": "en1997.py"}
CLONE_DRIVER = "l_setfield.py"
ARTIFACTS = ("en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999")


def driver_of(short):
    """📮 The leaf-kinds driver of an artifact: the profile driver in `../leaf-kinds` where one can register new entities (en1997), otherwise the clone driver that copies a set-field kind of the same field type."""
    return DRIVERS.get(short) or CLONE_DRIVER


def register_entities(ctx, driver_path):
    """📭 Adds the new entities to the `ENTS` list of the artifact's leaf-kinds driver (idempotent text edit)."""
    text = open(driver_path, encoding="utf8").read()
    S = ctx.spec.collection
    if S is None:
        return False
    slug = S.raw.get("kindSlug", S.names.kebab)
    if f'E("{slug}"' in text:
        return False
    parent = f'("{S.names.plural_snake}", "{S.entity}", "{S.names.snake}_id")'
    lines = f'    E("{slug}", "{S.emoji}", [("{S.names.plural_snake}", "{S.entity}", "id")], "{S.singular_en.capitalize()}", "{S.singular_de}"),\n'
    for n in S.nested:
        nslug = n.raw.get("kindSlug", n.names.kebab)
        lines += f'    E("{nslug}", "{n.emoji}", [{parent}, ("{n.names.plural_snake}", "{n.entity}", "id")], "{n.singular_en.capitalize()}", "{n.singular_de}"),\n'
    end = text.index("\n]\n", text.index("ENTS = ["))
    text = text[: end + 1] + lines + text[end + 1:]
    text = re.sub(r"(L\.TARGET_DE\.update\(\{)", lambda m: m.group(1) + f'"{S.names.snake}_id": "{S.singular_de}", ', text, count=1)
    open(driver_path, "w", encoding="utf8").write(text)
    seeds = os.path.join(os.path.dirname(driver_path), "seeds-" + ctx.short + ".json")
    data = json.load(open(seeds, encoding="utf8")) if os.path.exists(seeds) else {}
    data[S.names.plural_camel] = S.sample_row()
    open(seeds, "w", encoding="utf8").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    return True


def retag(ctx, kinds):
    """📬 Gives the new set-field kinds binary tags above every other tag (catalog and protocol line), so they never collide with the cloned insert/remove kinds."""
    root = ctx.layout.p(MUT)
    catalogs = {}
    for d in os.listdir(root):
        p = os.path.join(root, d, "🔣️.json")
        if os.path.isfile(p):
            try:
                c = json.load(open(p, encoding="utf8"))
            except json.JSONDecodeError:
                continue
            if "binaryTag" in c:
                catalogs[c["semanticKind"]] = (p, c)
    fixed = {k for k in kinds if k in catalogs}
    top = max(c["binaryTag"] for k, (p, c) in catalogs.items() if k not in fixed)
    proto = [p for p in walk_files(ctx.layout.p("🚪️io/💾️binary/🧬️mutations"), (".protocol.semio",))][0]
    ptext = open(proto, encoding="utf8").read()
    for k in kinds:
        if k not in catalogs:
            continue
        top += 1
        path, c = catalogs[k]
        c["binaryTag"] = top
        open(path, "w", encoding="utf8").write(json.dumps(c, indent=2, ensure_ascii=False) + "\n")
        ptext = re.sub(r"(record %s tag=)\d+" % re.escape(k), lambda m: m.group(1) + str(top), ptext)
    open(proto, "w", encoding="utf8").write(ptext)


def dedupe_patches(ctx):
    """📫 The leaf-kinds driver writes its own `Patch` line for every registered entity; a duplicate declaration left by the clone keeps the driver's line (the last one)."""
    path = ctx.layout.p("🧬️schema/🔺️diff/🦀️.rs")
    text = open(path, encoding="utf8").read()
    seen, out = {}, text.split("\n")
    for i, line in enumerate(out):
        m = re.match(r"\s+pub (\w+Patch) of ", line)
        if m:
            seen.setdefault(m.group(1), []).append(i)
    drop = {i for idx in seen.values() if len(idx) > 1 for i in idx[:-1]}
    if drop:
        open(path, "w", encoding="utf8").write("\n".join(l for i, l in enumerate(out) if i not in drop))
        print(f"[schema-evolve] removed {len(drop)} duplicate Patch declaration(s) in the diff Rust")


def apply(ctx):
    """📪 Runs after the twins exist: set-field kinds through the artifact's leaf-kinds driver."""
    import subprocess, sys
    kinds = ctx.kinds["set"] if hasattr(ctx, "kinds") else []
    driver = DRIVERS.get(ctx.short)
    if not kinds:
        return
    if not driver:
        print(f"[schema-evolve] no leaf-kinds driver registered for {ctx.short}: set-field kinds {kinds} are not generated")
        return
    leaf_dir = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "leaf-kinds"))
    if register_entities(ctx, os.path.join(leaf_dir, driver)):
        print("[schema-evolve] registered the new entities in leaf-kinds/" + driver)
    env = dict(os.environ, LEAF_ONLY=",".join(kinds))
    done = subprocess.run([sys.executable, "-I", driver, "--apply"], cwd=leaf_dir, env=env, stdin=subprocess.DEVNULL)
    if done.returncode:
        raise SystemExit(f"[schema-evolve] leaf-kinds driver failed ({done.returncode})")
    retag(ctx, kinds)
    dedupe_patches(ctx)
