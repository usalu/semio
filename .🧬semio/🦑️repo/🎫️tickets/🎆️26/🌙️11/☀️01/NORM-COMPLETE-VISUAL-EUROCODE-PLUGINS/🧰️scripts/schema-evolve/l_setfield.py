"""✏️ Set-field kinds by cloning: the kind of a new leaf (or of every field of a new collection) is a renamed copy of an existing set-field kind of the same artifact whose field has the same type; crates, registries, binary tags and fixtures follow the insert/remove clone. Used for every artifact that has no dedicated leaf-kinds driver."""
import copy, json, os, re
from core import Names, Unsupported, camel, kebab, pascal, plural_of, snake, walk_files
from l_mutations import MUT, Tr, apply_labels, clone_kind, clone_registries, find_dir, kind_names, next_tags, prose, protocol_clone, rename_map, tree_files
from l_codec import kind_of
import l_basedoc


def rules_text(ctx):
    return "".join(ctx.plan.read(p) for p in walk_files(ctx.layout.p(MUT + "/🧭️edit-rules"), (".rs",)))


def candidates(ctx, kind):
    """🪹 Existing set-field rules whose field has the given type kind: (plural, field, kind slug, rule line)."""
    out = []
    plurals = dict(ctx.collections)
    for m in re.finditer(r'SetFieldRule \{ path: "(\w+)\[\]\.(\w+)", kind: "(change-[a-z0-9-]+)"[^\n]*', rules_text(ctx)):
        plural, field, slug = m.group(1), m.group(2), m.group(3)
        ent = plurals.get(plural)
        node = ctx.defs.get(ent, {}).get("properties", {}).get(field) if ent else None
        if node is not None and kind_of(node, ctx.defs) == kind:
            out.append((plural, field, slug, m.group(0)))
    return out


def enum_info(ctx, plural, field):
    ent = dict(ctx.collections).get(plural)
    node = ctx.defs[ent]["properties"][field]
    if "$ref" in node:
        name = node["$ref"].rsplit("/", 1)[1]
        return name, ctx.defs[name]["enum"]
    if "enum" in node:
        return node.get("x-semio-rust", {}).get("type") or pascal(field), node["enum"]
    return None


def retoken(ctx, dst, old, new):
    """🪸 The payload schema of a cloned enum kind lists the new tokens: JSON arrays and TypeScript unions of the reference tokens are replaced."""
    for f in tree_files(dst):
        path = f
        if not ctx.plan.exists(path):
            continue
        text = ctx.plan.read(path)
        a = "[" + ", ".join(json.dumps(t) for t in old) + "]"
        arr = re.compile(r"\[\s*" + r"\s*,\s*".join(re.escape(json.dumps(t)) for t in old) + r"\s*\]")
        text2 = arr.sub(lambda m: "[" + ", ".join(json.dumps(t) for t in new) + "]", text)
        text2 = text2.replace(" | ".join(json.dumps(t) for t in old), " | ".join(json.dumps(t) for t in new)).replace("|".join(json.dumps(t) for t in old), "|".join(json.dumps(t) for t in new))
        if text2 != text:
            ctx.plan.write(path, text2)


def field_renamer(ref_field, new_field):
    """🪷 Renames a field in its snake, camel, Pascal and kebab spellings (`new_x_m`, `newXM`, `x_m`, `xM`, `x-m`); a single-word field is one string in several spellings, so the spelling follows the neighbours (quotes: camel, dashes: kebab, otherwise snake)."""
    forms = lambda f: {"s": snake(f), "c": f, "k": kebab(snake(f)), "p": pascal(f)}
    table = {}
    for prefix, pre_new in (("new_", "new_"), ("new", "new"), ("New", "New"), ("", "")):
        rf, nf = forms(ref_field), forms(new_field)
        spell = {"new_": ("s", "s"), "new": ("c", "p"), "New": ("p", "p"), "": None}[prefix]
        if prefix == "new_":
            table.setdefault(f"new_{rf['s']}", {})["s"] = f"new_{nf['s']}"
        elif prefix == "new":
            table.setdefault(f"new{rf['p']}", {})["c"] = f"new{nf['p']}"
        elif prefix == "New":
            table.setdefault(f"New{rf['p']}", {})["p"] = f"New{nf['p']}"
        else:
            for k in "sckp":
                table.setdefault(rf[k], {})[k] = nf[k]
    keys = sorted(table, key=lambda a: -len(a))
    rx = re.compile("|".join("(?<![A-Za-z0-9_])%s(?![a-z0-9_])" % re.escape(a) if a[0].islower() else "(?<![a-z0-9_])%s(?![a-z0-9])" % re.escape(a) for a in keys))

    def pick(m):
        src, options = m.group(0), table[m.group(0)]
        targets = set(options.values())
        if len(targets) == 1:
            return next(iter(targets))
        text, before, after = m.string, m.string[m.start() - 1:m.start()], m.string[m.end():m.end() + 1]
        if src[0].isupper() and "p" in options:
            return options["p"]
        if "-" in (before, after) and "k" in options:
            return options["k"]
        if (before == '"' or after == '"') and "c" in options:
            return options["c"]
        return options.get("s") or next(iter(targets))
    return lambda text: rx.sub(pick, text)


class FieldTr(Tr):
    def __init__(self, kind_pairs, name_pairs, field_fn):
        Tr.__init__(self, kind_pairs, name_pairs)
        self.field_fn = field_fn

    def __call__(self, text, hint=None):
        return self.field_fn(Tr.__call__(self, text, hint))


def change_dir_name(slug):
    return slug


def delta_empty(doc, skip):
    """🪵 The empty form of a diff lane in this artifact: copied from a lane that holds no rows."""
    for k, v in doc.items():
        if k == skip:
            continue
        if v is None or (isinstance(v, dict) and all(isinstance(x, list) and not x for x in v.values())):
            return copy.deepcopy(v)
    return None


def fixture_transform(ctx, ref_plural, ref_field, new_plural, new_field, node, sample_row, new_value, pairs, new_camel_kind):
    """🪴 before/after/diff/mutation/outcome of the cloned kind: the ref documents with the target row of the new entity, the field set to the new value."""
    def build(rel, text):
        obj = json.loads(text)
        return obj
    return build


def clone_fixtures(ctx, ref_kind, new_kind, pairs, ref_plural, ref_field, new_plural, ent, new_field, new_value, row_id):
    root = ctx.layout.p("🧪️tests/🧫️fixtures/🧬️mutations")
    src = [d for d in os.listdir(root) if d.endswith(ref_kind) and re.fullmatch(r"\W*" + re.escape(ref_kind), d)]
    if not src:
        return
    base = os.path.join(root, src[0])
    dst = os.path.join(root, src[0][: len(src[0]) - len(ref_kind)] + new_kind)
    files = {os.path.relpath(f, base): json.loads(ctx.plan.read(f)) for f in tree_files(base)}
    existing = l_basedoc.sample(ctx).get(new_plural) or []
    row = copy.deepcopy(existing[0] if existing else ent.sample_row())
    if row is None:
        raise Unsupported(f"the spec of {new_plural} has no sample row to carry the new field")
    new_value = differing(row.get(camel(new_field)), new_value, ent, new_field)
    for rel, obj in files.items():
        name = os.path.basename(rel)
        if name == "🔣️.before.json":
            out = l_basedoc.with_rows(ctx, new_plural, [row])
        elif name == "🔣️.after.json":
            changed = copy.deepcopy(row)
            changed[camel(new_field)] = new_value
            out = l_basedoc.with_rows(ctx, new_plural, [changed])
        elif name == "🔣️.mutation.json":
            out = json.loads(apply_pairs_json(json.dumps(obj, ensure_ascii=False), pairs))
            for k in list(out):
                if k.startswith("new"):
                    out[k] = new_value
            for k in out:
                if k == "id" or (k.endswith("Id") and not k.startswith("new")):
                    out[k] = row.get("id", row_id)
        elif name == "🔣️.diff.json":
            lane = obj.get(ref_plural)
            if isinstance(lane, dict) and lane.get("modified"):
                item = copy.deepcopy(lane["modified"][0])
                for k in [k for k in item if k != "patch"]:
                    item[k] = row["id"] if k == "id" else item[k]
                item["patch"] = {camel(f): (new_value if f == new_field else None) for f in ent_fields(ctx, ent, new_plural)}
                new_lane = {**copy.deepcopy(lane), "modified": [item]}
            elif isinstance(lane, list) and lane and lane[0].get("op") == "Replace":
                item = copy.deepcopy(lane[0])
                item["value"] = {**copy.deepcopy(row), camel(new_field): new_value}
                new_lane = [item]
            else:
                raise Unsupported(f"diff fixture of {ref_kind} has no modified row to clone")
            out = l_basedoc.diff_of(obj, new_plural, new_lane)
        else:
            out = obj
        ctx.plan.write(os.path.join(dst, rel), json.dumps(out, indent=2, ensure_ascii=False) + "\n")


def differing(current, wanted, ent, field_name):
    """🎚️ The value of the set-field vector always differs from the one the row holds before the mutation (a mutation that changes nothing proves nothing)."""
    if wanted != current:
        return wanted
    if isinstance(wanted, bool):
        return not wanted
    if isinstance(wanted, (int, float)):
        return wanted + 1 if wanted else 1.5 if isinstance(wanted, float) else 1
    field = next((f for f in getattr(ent, "fields", []) if f.name == field_name), None)
    if field is not None and getattr(field, "tokens", None) and len(field.tokens) > 1:
        return next(t for t in field.tokens if t != wanted)
    return f"{wanted}-changed"


def apply_pairs_json(text, pairs):
    return pairs(text, "json") if callable(pairs) else text


def ent_fields(ctx, ent, plural):
    entity = dict(ctx.collections).get(plural)
    if entity:
        return [snake(k) for k, v in ctx.defs[entity]["properties"].items() if k != "id" and v.get("type") != "array"]
    return [f.name for f in ent.fields]


def own_semantics(ctx, dst, new_kind, field):
    """🪪 A cloned set-field kind describes itself: the semantic descriptor (`entity`, `record`) names the new kind and the payload schema's value widget takes the unit, display unit, precision and step of the spec's field, never those of the donor field."""
    slug = new_kind[len("change-"):]
    for f in sorted(p for p in ctx.plan.files if p.startswith(dst + "/")):
        if f.endswith(".rs"):
            text = ctx.plan.read(f)
            new = re.sub(r'(entity: )"[a-z0-9-]+"', lambda m: m.group(1) + '"' + slug + '"', text)
            new = re.sub(r'(record: )"Changed\w+"', lambda m: m.group(1) + '"Changed' + pascal(slug) + '"', new)
            new = re.sub(r'^(\s+)[a-z_]+(: "' + re.escape(new_kind) + r'",)$', r'\1kind\2', new, flags=re.M)
            if new != text:
                ctx.plan.write(f, new)
        elif f.endswith("🧬️schema/🔣️.json"):
            doc = json.loads(ctx.plan.read(f))
            changed = False
            for prop in doc.get("properties", {}).values():
                ui = prop.get("x-semio-ui", {})
                if ui.get("role") == "value" and prop.get("type") == "number" and field.ty == "f64":
                    fresh = {k: v for k, v in ui.items() if k not in ("unit", "displayUnit", "displayFactor", "precision", "step")}
                    fresh.update({k: v for k, v in field.ui().items() if k in ("unit", "displayUnit", "displayFactor", "precision", "step")})
                    if fresh != ui:
                        prop["x-semio-ui"] = fresh
                        changed = True
            if changed:
                ctx.plan.write(f, json.dumps(doc, indent=2, ensure_ascii=False) + "\n")


def taxonomy_row(ctx, ref_kind, new_kind, dst, plural, field):
    """🗂️ The `mutation-leaf-taxonomy-v1` row of a cloned set-field kind (derived from the donor kind's row, with the edit path and payload field of the new field); idempotent."""
    from l_mutations import TAXONOMY
    taxonomy = json.loads(ctx.plan.read(TAXONOMY))
    rows = taxonomy["rows"]
    artifact = next(r["artifact"] for r in rows if r["artifact"].endswith(ctx.short))
    if any(r["artifact"] == artifact and r["kind"] == new_kind for r in rows):
        return
    ref = next((r for r in rows if r["artifact"] == artifact and r["kind"] == ref_kind), None)
    if ref is None:
        ref = next((r for r in reversed(rows) if r["artifact"] == artifact and r["verb"] == "change" and r.get("edit", {}).get("rule") == "set-field"), None)
        if ref is None:
            raise Unsupported(f"the mutation taxonomy has no set-field row of {artifact} to derive the row of {new_kind} from")
    slug = new_kind[len("change-"):]
    row = copy.deepcopy(ref)
    edit = dict(row.get("edit") or {"rule": "set-field", "selectors": [{"kind": "id", "key": "id"}]})
    edit.update({"rule": "set-field", "path": f"{plural}[].{camel(field.name)}", "value": "new" + pascal(field.name)})
    row.update({"kind": new_kind, "module": dst, "entity": slug, "aggregateVariant": pascal(new_kind), "type": pascal(new_kind), "record": "Changed" + pascal(slug), "source": ref["source"].replace(ref["module"], dst), "edit": edit})
    row = dict(sorted(row.items()))
    last = max(i for i, r in enumerate(rows) if r["artifact"] == artifact)
    rows.insert(last + 1, row)
    ctx.plan.write(TAXONOMY, json.dumps(taxonomy, indent=2, ensure_ascii=False) + "\n")


def clone_setfield(ctx, ent, plural, field, kind_slug, entity_def=None):
    """🪳 One set-field kind for `field` (a spec Field) of the entity `ent` behind the collection `plural`."""
    kind = {"f64": "f64", "String": "String", "u8": "u8", "u32": "u32", "enum": "enum"}[field.ty]
    new_kind = f"change-{kind_slug}-{field.name.replace('_', '-')}"
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', ctx.plan.read(ctx.layout.p(MUT + "/🦀️.rs")), re.M))
    if new_kind in existing or os.path.isdir(os.path.join(ctx.layout.p(MUT), "")) and any(d.endswith(new_kind) for d in os.listdir(ctx.layout.p(MUT))):
        return None
    cands = candidates(ctx, kind)
    if kind == "enum":
        cands = [c for c in cands if enum_info(ctx, c[0], c[1])]
    if not cands:
        raise Unsupported(f"no existing set-field kind with a {kind} field in {ctx.short} to clone")
    donor = ctx.spec.raw.get("leaf", {}).get("donor")
    pick = next((c for c in cands if donor and c[2] == donor), None) or next((c for c in reversed(cands) if c[0] == plural), cands[-1])
    ref_plural, ref_field, ref_kind, _ = pick
    ref_entity = dict(ctx.collections)[ref_plural]
    ref_ent_names = Names(ref_plural, ref_entity)
    ref_prefix_slug = ref_kind[len("change-"):]
    ref_field_kebab = kebab(snake(ref_field))
    ref_ent_slug = ref_prefix_slug[: -(len(ref_field_kebab) + 1)] if ref_prefix_slug.endswith("-" + ref_field_kebab) else ref_prefix_slug.rsplit("-", 1)[0]
    kp = rename_map("change", ref_prefix_slug, f"{kind_slug}-{field.name.replace('_', '-')}", [])
    name_pairs = [(Names(ref_plural, ref_entity), Names(plural, ent.entity if hasattr(ent, "entity") else ref_entity)), (Names(plural_of(ref_ent_slug) if False else camel(plural_of(ref_ent_slug)), pascal(ref_ent_slug)), Names(camel(plural_of(kind_slug)), pascal(kind_slug)))]
    extra = []
    if kind == "enum":
        ref_enum, ref_tokens = enum_info(ctx, ref_plural, ref_field)
        extra = [(ref_enum, field.enum_name)]
    pairs = FieldTr(kp + extra, name_pairs, field_renamer(ref_field, camel(field.name)))
    labels = []
    src_dir, name = find_dir(ctx, ref_kind)
    old = re.findall(r'native\("([^"]+)", "([^"]+)"\)', "".join(open(f, encoding="utf8").read() for f in tree_files(src_dir) if f.endswith(".rs")))[:1]
    if old:
        labels = [(old[0], (f"Change {ent.singular_en}: {field.en}", f"{ent.singular_de}: {field.de} ändern"))]
    tag = next_tags(ctx, 1)[0]
    dst = clone_kind(ctx, ref_kind, new_kind, pairs, labels, tag, ent, field)
    own_semantics(ctx, os.path.join(os.path.dirname(src_dir), dst), new_kind, field)
    taxonomy_row(ctx, ref_kind, new_kind, dst, plural, field)
    if kind == "enum":
        retoken(ctx, os.path.join(os.path.dirname(src_dir), dst), ref_tokens, field.tokens)
    clone_registries(ctx, [ref_kind, ref_kind.replace("-", "_"), pascal(ref_kind), camel(ref_kind)], pairs)
    protocol_clone(ctx, [ref_kind], [new_kind], [tag])
    ctx.__dict__.setdefault("text_pairs", []).append((ref_kind, new_kind, pairs))
    value = {"f64": 1.5, "String": "probe", "u8": 1, "u32": 1}.get(kind)
    if kind == "enum":
        value = field.tokens[1] if len(field.tokens) > 1 else field.tokens[0]
    clone_fixtures(ctx, ref_kind, new_kind, pairs, ref_plural, ref_field, plural, ent, field.name, value if kind == "enum" else (field.sample if field.sample not in (None, 0.0, "") else value), None)
    return new_kind


MOUNT = re.compile(r'#\[path = "\.\./([^"/]+)/🦀️\.rs"\]\npub mod (\w+);\n')


def isolate_crate(ctx, kinds):
    """📦 The cloned kinds of a new collection (insert, remove and set-field, nested ones included) live in a crate of their own (`mut-<plural>`), never in the group crates of the reference kinds they were cloned from: the mounts move out of those crates, the new crate gets manifest, workspace member and dependency, `script.ts` package, dependents and `use` paths. A bare `use` of a moved module keeps working through a `use` of the new crate; `pub use` re-exports of the old group crates are dropped (their consumers are re-pointed)."""
    S = ctx.spec.collection
    mods = {snake(k): k for k in kinds}
    root = ctx.layout.p(MUT)
    groups = []
    for d in sorted(os.listdir(root)):
        path = f"{root}/{d}/🦀️.rs"
        if not os.path.isfile(path) or os.path.exists(f"{root}/{d}/🔣️.json") or not os.path.isfile(f"{root}/{d}/📦️packages/🦀️rust/Cargo.toml"):
            continue
        text = ctx.plan.read(path)
        mounts = list(MOUNT.finditer(text))
        moved = [m for m in mounts if m.group(2) in mods]
        if moved and len(moved) < len(mounts):
            groups.append((d, text, mounts, moved))
    if not groups:
        return None
    slug = kebab(S.plural)
    ndir = f"{S.emoji}{slug}"
    if os.path.isdir(f"{root}/{ndir}") or ctx.plan.exists(f"{root}/{ndir}/🦀️.rs"):
        return None
    first_dir, first_text, first_mounts, _ = groups[0]
    manifest = ctx.plan.read(f"{root}/{first_dir}/📦️packages/🦀️rust/Cargo.toml")
    gname = re.search(r'^name = "([^"]+)"', manifest, re.M).group(1)
    nname = re.sub(r"-mut-.*$", "-mut-" + slug, gname)
    nsnake = nname.replace("-", "_")
    renames = []
    block = ""
    for d, text, mounts, moved in groups:
        kept = text
        for m in reversed(moved):
            kept = kept[:m.start()] + kept[m.end():]
        ctx.plan.write(f"{root}/{d}/🦀️.rs", kept)
        block += "".join(f'#[path = "../{m.group(1)}/🦀️.rs"]\npub mod {m.group(2)};\n' for m in moved)
        gn = re.search(r'^name = "([^"]+)"', ctx.plan.read(f"{root}/{d}/📦️packages/🦀️rust/Cargo.toml"), re.M).group(1)
        renames.append((gn.replace("-", "_"), {m.group(2) for m in moved}))
    doc = f"//! {S.emoji} {S.entity} mutation leaves: insert, remove and one set-field kind per settable field."
    head = re.sub(r"^//!.*$", lambda _: doc, first_text[:first_mounts[0].start()], count=1, flags=re.M)
    ctx.plan.write(f"{root}/{ndir}/🦀️.rs", head + "//#region 🪢️Mounts\n" * (0 if "//#region" in head else 1) + block + first_text[first_mounts[-1].end():])
    nmanifest = re.sub(r'^description = ".*"$', f'description = "{S.entity} mutation leaves: insert, remove and one set-field kind per settable field"', manifest.replace(gname, nname), flags=re.M)
    ctx.plan.write(f"{root}/{ndir}/📦️packages/🦀️rust/Cargo.toml", nmanifest)
    art_root = ctx.layout.dir
    gnames = {re.search(r'^name = "([^"]+)"', ctx.plan.read(f"{root}/{d}/📦️packages/🦀️rust/Cargo.toml"), re.M).group(1): d for d, *_ in groups}
    moved_mods = {m for _, ms in renames for m in ms}
    alt = "|".join(re.escape(m) for m in sorted(moved_mods))
    path_rx = re.compile(r"\b(semio_\w+_mut_\w+)::(%s)\b" % alt)
    list_rx = re.compile(r"(use semio_\w+_mut_\w+::\{)([^}]*)(\})")

    def drop(m, whole=""):
        items = [x.strip() for x in m.group(2).split(",") if x.strip()]
        kept = [x for x in items if x not in moved_mods]
        gone = [x for x in items if x in moved_mods]
        if not gone:
            return m.group(0)
        head = m.group(1) + ", ".join(kept) + m.group(3) if kept else ""
        rest = whole.replace(m.group(0), "")
        needed = [x for x in gone if not m.group(1).lstrip().startswith("pub ") and re.search(r"(?<![\w:])%s::" % x, rest)]
        add = f"use {nsnake}::{{{', '.join(needed)}}};" if needed else ""
        return head + ("\n" if head and add else "") + add
    roots = [os.path.dirname(os.path.dirname(os.path.dirname(p))) for p in walk_files(art_root, ("Cargo.toml",)) if "/target/" not in p and "📦️packages/🦀️rust/Cargo.toml" in p]
    users = set()
    for path in walk_files(art_root, (".rs", ".toml", ".ts")):
        if "/target/" in path or path.startswith(f"{root}/{ndir}/"):
            continue
        text = ctx.plan.read(path)
        new = text
        if path.endswith(".rs") and "_mut_" in new:
            new = path_rx.sub(lambda m: m.group(0) if m.group(1) == nsnake else f"{nsnake}::{m.group(2)}", new)
            whole = new
            new = list_rx.sub(lambda m: drop(m, whole), new)
            if new != text and nsnake in new:
                users.add(path)
        if new != text:
            ctx.plan.write(path, new)
    owners = set()
    for path in users:
        deepest = max((r for r in roots if path.startswith(r + "/")), key=len, default=None)
        if deepest:
            owners.add(deepest + "/📦️packages/🦀️rust/Cargo.toml")
    gn, d = next(iter(gnames.items()))
    for path in walk_files(art_root, ("Cargo.toml", "script.ts")):
        if "/target/" in path or path.startswith(f"{root}/{ndir}/"):
            continue
        text = ctx.plan.read(path)
        new = text
        if path in owners and f"{nname} =" not in new:
            dep = re.search(r"^semio-s-artifact-norm-%s-mut-[\w-]+ = \{ workspace = true \}\n" % ctx.short, new, re.M)
            if dep:
                new = new[:dep.end()] + f"{nname} = {{ workspace = true }}\n" + new[dep.end():]
        if path == art_root + "/Cargo.toml":
            rel = f"{ctx.layout.any[len(art_root) + 1:]}/{MUT}/{d}/📦️packages/🦀️rust"
            member = f'    "{rel}",\n'
            if member in new and nname not in new:
                new = new.replace(member, member + member.replace(d, ndir))
            dep_block = re.search(r"\[workspace\.dependencies\.%s\]\npath = \"[^\"]+\"\n\n" % re.escape(gn), new)
            if dep_block and f"[workspace.dependencies.{nname}]" not in new:
                new = new[:dep_block.end()] + dep_block.group(0).replace(gn, nname).replace(d, ndir) + new[dep_block.end():]
        elif path.endswith("script.ts") and f'"{gn}"' in new and f'"{nname}"' not in new:
            new = new.replace(f'"{gn}"', f'"{gn}","{nname}"', 1)
        if new != text:
            ctx.plan.write(path, new)
    ctx.plan.note(f"set-field kinds mounted in the new crate {nname} ({ndir}), not in the crates of the reference kinds ({', '.join(gnames)})")
    return nname
