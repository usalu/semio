"""🛡️ Plan scanners: the law-test-like checks (kind-kebab, `nested_patch!` section order, `row_key!` presence, `crate::` across crate splits, snapshot initializers) run on the planned file set before anything is written, so a plan that would fail a repo law or the compiler on a known pitfall is refused instead of applied."""
import importlib.util, os, re, tomllib
from core import NORM, SKIP_DIRS, singular_of, snake, walk_files
import l_patch

SPLIT_REFS = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "split-refs", "scan.py"))
SEMANTICS = re.compile(r'for (\w+)[^{]*\{\s*const SEMANTICS[^}]*kind: "([^"]+)"')
KEY_USE = re.compile(r"\bby (\w+Key)\b")
KEY_STRUCT = re.compile(r"^\s*pub struct (\w+Key);", re.M)
KEY_ENTRY = re.compile(r"^\s*(\w+Key) for \w+;", re.M)
KEY_BLOCK = re.compile(r"row_key! \{\n((?:.*\n)*?)\}", re.M)
HEAD = re.compile(r"^( *)(?:///[^\n]*\n *)*(\w+) for [\w:]+ \{\n", re.M)


def to_kebab(name):
    """🔤 The dsl::Mutations derive's to_kebab (same function as the kind-kebab law test `⚖️compliance/🧪️tests/🧷️kind-kebab`)."""
    out, chars = "", list(name)
    for i, c in enumerate(chars):
        if c in "_-":
            if out and not out.endswith("-"):
                out += "-"
            continue
        if c.isupper():
            p = chars[i - 1] if i else None
            n = chars[i + 1] if i + 1 < len(chars) else None
            boundary = p is not None and (p.islower() or p.isdigit()) or (p is not None and p.isupper() and n is not None and n.islower())
            if boundary and not out.endswith("-"):
                out += "-"
            out += c.lower()
        else:
            out += c
    return out


def rust_files(plan):
    return [(p, n) for p, (o, n, _) in sorted(plan.files.items()) if p.endswith(".rs")]


def kind_kebab(plan):
    out = []
    for path, text in rust_files(plan):
        if "🧬️mutations" not in path:
            continue
        for m in SEMANTICS.finditer(text):
            if m.group(2) != to_kebab(m.group(1)):
                out.append(f"kind-kebab: {m.group(1)} has kind {m.group(2)!r}, the derive needs {to_kebab(m.group(1))!r} ({plan.rel(path)})")
    return out


def nested_patch_order(plan):
    out = []
    for path, text in rust_files(plan):
        if "nested_patch! {" not in text:
            continue
        for m in l_patch.BLOCK.finditer(text):
            body = m.group(1)
            if "macro_rules" in body[:80]:
                continue
            if l_patch.normalize_block(body) != body:
                head = HEAD.search(body)
                out.append(f"nested_patch: {head.group(2) if head else '?'} has sections out of the order set, within, optional, rows or a section twice ({plan.rel(path)})")
    return out


def diff_tree(ctx):
    root = ctx.layout.p("🧬️schema/🔺️diff")
    paths = {p for p in walk_files(root, (".rs",)) if not any(x in p[len(root):] for x in ("twin", "🧪️tests", "📦️packages"))}
    paths |= {p for p in ctx.plan.files if p.startswith(root + "/") and p.endswith(".rs") and not any(x in p[len(root):] for x in ("twin", "🧪️tests", "📦️packages"))}
    return sorted(paths)


def row_key_presence(ctx):
    texts = {p: ctx.plan.read(p) for p in diff_tree(ctx)}
    declared, used = [], {}
    for path, text in texts.items():
        declared += [(name, path) for name in KEY_STRUCT.findall(text)]
        for block in KEY_BLOCK.finditer(text):
            declared += [(name, path) for name in KEY_ENTRY.findall(block.group(1))]
        for name in KEY_USE.findall(text):
            used.setdefault(name, path)
    names = [n for n, _ in declared]
    out = [f"row_key: `by {name}` has no `{name} for <Row>;` entry in a row_key! list and no `pub struct {name};` ({ctx.plan.rel(path)})" for name, path in sorted(used.items()) if name not in names]
    out += [f"row_key: {name} is declared {names.count(name)} times ({', '.join(sorted({ctx.plan.rel(p) for n, p in declared if n == name}))})" for name in sorted(set(names)) if names.count(name) > 1]
    return out


def split_refs():
    spec = importlib.util.spec_from_file_location("split_refs_scan", SPLIT_REFS)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def crate_roots(ctx):
    plan = ctx.plan
    found = {p for p in plan.files if p.endswith("/Cargo.toml")}
    for path in plan.files:
        d = os.path.dirname(path)
        while len(d) > len(NORM) and path.endswith(".rs"):
            cargo = os.path.join(d, "Cargo.toml")
            if plan.exists(cargo):
                found.add(cargo)
                break
            d = os.path.dirname(d)
    return sorted(found)


def crate_refs(ctx):
    scan = split_refs()
    plan = ctx.plan
    scan.ROOT = NORM
    scan.READ = plan.read
    scan.EXISTS = plan.exists
    scan.PKG.clear()
    for dp, dn, fn in os.walk(NORM):
        dn[:] = [d for d in dn if d not in SKIP_DIRS and d != "🗑️generated"]
        if "Cargo.toml" in fn and not any(x in dp for x in ("🧪️tests",)):
            try:
                t = tomllib.load(open(os.path.join(dp, "Cargo.toml"), "rb"))
                lp = t.get("lib", {}).get("path")
                if lp:
                    scan.PKG[t["package"]["name"].replace("-", "_")] = os.path.normpath(os.path.join(dp, lp))
            except Exception:
                pass
    for path in plan.files:
        if path.endswith("/Cargo.toml"):
            try:
                t = tomllib.loads(plan.read(path))
                lp = t.get("lib", {}).get("path")
                if lp:
                    scan.PKG[t["package"]["name"].replace("-", "_")] = os.path.normpath(os.path.join(os.path.dirname(path), lp))
            except Exception:
                pass
    out = []
    for cargo in crate_roots(ctx):
        try:
            result = scan.scan_crate(cargo)
        except Exception as error:
            out.append(f"crate-refs: {plan.rel(cargo)} not scanned ({type(error).__name__}: {error})")
            continue
        if result and result[2] and not result[3] and not result[0].endswith("-io"):
            for name, files in sorted(result[2].items()):
                out.append(f"crate-refs: crate {result[0]} uses `crate::{name}` that its mounted modules do not define ({sorted(files)[0]})")
    return out


def snapshot_initializers(ctx):
    """🧮 Every `<Snapshot> { .. }` literal without `..` of the artifact names all root collections (a new collection missing in one initializer is a compile error)."""
    if not ctx.collections:
        return []
    names = [snake(c) for c, _ in ctx.collections]
    if ctx.spec.collection and snake(ctx.spec.collection.plural) not in names:
        names.append(snake(ctx.spec.collection.plural))
    snap = ctx.layout.snapshot_name
    out = []
    root = ctx.layout.dir
    paths = [p for p in walk_files(root, ("🦀️.rs", ".rs")) if "twin" not in p and "/target/" not in p]
    paths += [p for p in ctx.plan.files if p.endswith(".rs") and p.startswith(root + "/") and p not in paths]
    import l_rust
    for path in sorted(set(paths)):
        if not ctx.plan.exists(path):
            continue
        text = ctx.plan.read(path)
        if snap not in text and "Self {" not in text and "Self{" not in text:
            continue
        for start, end in l_rust.snapshot_literal_spans(text, snap):
            items = l_patch.split_top(text[start:end])
            if any(item.startswith("..") for item in items):
                continue
            keys = {re.match(r"\s*(\w+)", item).group(1) for item in items if re.match(r"\s*\w+\s*(?::[^:]|$)", item)}
            have = keys & set(names)
            if len(have) >= max(3, len(names) * 6 // 10) and have != set(names):
                out.append(f"snapshot-initializer: a `{snap}`/`Self` literal misses {', '.join(n for n in names if n not in keys)[:120]} ({ctx.plan.rel(path)})")
    return out


TYPE_DECL = re.compile(r"((?:^[ \t]*(?:#\[[^\n]*\]|///[^\n]*)\n)*)^pub (?:struct|enum) (\w+)[^{;]*\{", re.M)


def canonical_json(ctx):
    """🧾 Types that insert/change mutation leaves (derive `CanonicalJsonTree`) carry as fields, and the field types of those types, must derive `CanonicalJsonTree` and name the owner."""
    from core import walk_files, SNAPSHOT
    model_path = ctx.layout.p(SNAPSHOT + "/🦀️.rs")
    model = ctx.plan.read(model_path)
    decls = {}
    for m in TYPE_DECL.finditer(model):
        depth, i = 0, m.end() - 1
        while i < len(model):
            depth += model[i] == "{"
            depth -= model[i] == "}"
            if depth == 0:
                break
            i += 1
        decls[m.group(2)] = (m.group(1), model[m.end():i])
    paths = {p for p in walk_files(ctx.layout.p("🧬️schema/🧬️mutations"), (".rs",))} | {p for p in ctx.plan.files if "🧬️mutations" in p and p.endswith(".rs")}
    wanted = set()
    for path in paths:
        text = ctx.plan.read(path)
        if "CanonicalJsonTree" not in text:
            continue
        wanted |= {w for w in re.findall(r"\b[A-Z]\w+\b", " ".join(re.findall(r"^\s*pub \w+\s*:\s*([^,\n]+)", text, re.M))) if w in decls}
    seen, queue = set(), list(wanted)
    while queue:
        name = queue.pop()
        if name in seen:
            continue
        seen.add(name)
        queue += [w for w in re.findall(r"\b[A-Z]\w+\b", decls[name][1]) if w in decls]
    return [f"canonical-json: model type {name} is used by a CanonicalJsonTree mutation leaf but lacks the CanonicalJsonTree derive or #[canonical_json(owner = ..)] ({ctx.plan.rel(model_path)})" for name in sorted(seen) if "CanonicalJsonTree" not in decls[name][0] or "canonical_json(owner" not in decls[name][0]]


TAXONOMY = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"


def schema_leaf_paths(snapshot):
    """🍂 Every settable leaf path of the collections of a snapshot schema (`layers[].cptNkt`, `members[].punching.asw`, `members[].longitudinal[].count`): scalar and enum members of the row types, flattened plain objects, arrays of row types; `id` excluded."""
    defs, out = snapshot["$defs"], []

    def value(node):
        for key in ("oneOf", "anyOf"):
            parts = node.get(key)
            if parts:
                return next((x for x in parts if x != {"type": "null"}), node)
        return node

    def walk(name, prefix, depth=0):
        for member, raw in defs[name].get("properties", {}).items():
            if member == "id":
                continue
            node = value(raw)
            ref = node.get("$ref", "").rsplit("/", 1)[-1]
            if node.get("type") == "array":
                item = value(node.get("items", {})).get("$ref", "").rsplit("/", 1)[-1]
                if item in defs and "properties" in defs[item] and depth < 4:
                    walk(item, f"{prefix}{member}[].", depth + 1)
            elif ref in defs and "properties" in defs[ref]:
                walk(ref, f"{prefix}{member}.", depth + 1)
            else:
                out.append(prefix + member)
    for member, raw in snapshot["properties"].items():
        item = raw.get("items", {}).get("$ref", "").rsplit("/", 1)[-1] if raw.get("type") == "array" else ""
        if item in defs and "properties" in defs[item]:
            walk(item, f"{member}[].")
    return out


def set_field_gaps(snapshot, taxonomy, artifact_dir):
    """🕳️ Settable leaf paths of the schema that no set-field row of the mutation taxonomy edits; rows without an `edit` object (older entries) count by their kind name ending in the kebab field name."""
    covered = {row["edit"]["path"] for row in taxonomy["rows"] if row["artifact"] == artifact_dir and (row.get("edit") or {}).get("rule") == "set-field"}
    bare = [row["kind"] for row in taxonomy["rows"] if row["artifact"] == artifact_dir and not row.get("edit") and row["kind"].startswith("change-")]
    kebab = lambda name: snake(name).replace("_", "-")
    return [p for p in schema_leaf_paths(snapshot) if p not in covered and not any(k.startswith("change-" + kebab(singular_of(p.split("[", 1)[0]))) and k.endswith("-" + kebab(p.rsplit(".", 1)[-1])) for k in bare)]


def set_field_kinds(ctx):
    """🔘 A plan must not add a settable schema field without a set-field mutation kind: gaps that exist only after the plan are findings (fields the leaf-kinds driver covers after the twins exist, named by `ctx.kinds["set"]`, count as covered). Whole-repo listing: `evolve.py verify set-fields`."""
    import json
    from core import SNAPSHOT
    schema_path = ctx.layout.p(SNAPSHOT + "/🔣️.json")
    tax_path = TAXONOMY
    if not ctx.plan.exists(tax_path):
        return []
    folder = os.path.basename(ctx.layout.dir.split("/🏅️standards")[0].rstrip("/"))
    after = set_field_gaps(json.loads(ctx.plan.read(schema_path)), json.loads(ctx.plan.read(tax_path)), folder)
    before = set_field_gaps(json.loads(open(schema_path, encoding="utf8").read()), json.loads(open(tax_path, encoding="utf8").read()), folder) if os.path.exists(schema_path) else []
    planned = set(getattr(ctx, "kinds", {}).get("set", []))
    new = [p for p in after if p not in before]
    if planned:
        leaf = getattr(getattr(ctx.spec, "leaf", None), "camel", None)
        own = getattr(ctx.spec.collection, "plural", None) if getattr(ctx.spec, "collection", None) else None
        new = [p for p in new if not (leaf and p.rsplit(".", 1)[-1] == leaf or own and p.startswith(own + "[]."))]
    return [f"set-field-kinds: schema field {p} has no set-field mutation kind in the taxonomy of {folder}" for p in new]


IMPLAUSIBLE_STEP = {"mm": 100, "m": 10, "cm": 100, "°": 45, "deg": 45, "s": 3600, "m²": 100, "mm²": 10000, "°C": 50}


def ui_findings(snapshot):
    """🎚️ Suspicious `x-semio-ui` of the row types of a snapshot schema: a label that repeats the label of a root collection (a copied index field: "Wall" on `cPe`), two members of one row type with the same EN label, the "Zero-based position" description on a member that is no index, and steppers whose step cannot belong to the field (step above the maximum, or a dimensionless coefficient/fraction/factor stepping by whole numbers)."""
    defs, out = snapshot["$defs"], []
    roots = set()
    for name, node in snapshot["properties"].items():
        label = ((node.get("x-semio-ui") or {}).get("label") or {}).get("en")
        if label:
            roots |= {label.lower(), label.lower().rstrip("s")}
    for entity, definition in defs.items():
        seen = {}
        for member, raw in definition.get("properties", {}).items():
            node = raw
            ui = node.get("x-semio-ui") or {}
            label = (ui.get("label") or {}).get("en")
            where = f"{entity}.{member}"
            if member == "id" or not label:
                continue
            if label.lower() in roots:
                out.append(f"ui: {where} is labelled {label!r}, the name of a collection (copied index field label)")
            if label in seen:
                out.append(f"ui: {where} and {entity}.{seen[label]} share the label {label!r}")
            seen.setdefault(label, member)
            if "zero-based position" in ((ui.get("description") or {}).get("en") or "").lower() and member not in ("index", "position"):
                out.append(f"ui: {where} carries the 'Zero-based position' description of an index field")
            step = ui.get("step")
            if ui.get("widget") == "stepper" and step is not None:
                top = node.get("maximum", ui.get("softMax"))
                if top is not None and step > abs(top):
                    out.append(f"ui: {where} steps by {step}, above its maximum {top}")
                shown = step * (ui.get("displayFactor") or 1)
                unit = ui.get("displayUnit") or ui.get("unit")
                if unit in IMPLAUSIBLE_STEP and shown > IMPLAUSIBLE_STEP[unit]:
                    out.append(f"ui: {where} steps by {shown:g} {unit}, implausible for the unit (limit {IMPLAUSIBLE_STEP[unit]:g})")
                doc = " ".join((node.get("x-semio-rust") or {}).get("doc", [])).lower()
                if node.get("type") == "number" and not ui.get("unit") and step >= 1 and re.search(r"\b(coefficient|fraction|factor|ratio of)\b", doc + " " + label.lower()):
                    out.append(f"ui: {where} is a dimensionless coefficient but steps by {step}")
    return out


def ui_steppers(ctx):
    """🎛️ A plan must not introduce suspicious steppers or labels (findings that exist only after the plan); whole-repo listing: `evolve.py verify steppers`."""
    import json
    from core import SNAPSHOT
    path = ctx.layout.p(SNAPSHOT + "/🔣️.json")
    after = ui_findings(json.loads(ctx.plan.read(path)))
    before = ui_findings(json.loads(open(path, encoding="utf8").read())) if os.path.exists(path) else []
    return [f for f in after if f not in before]


def repo_ui_findings(shorts=None):
    """🗺️ {artifact folder: [findings]} over every artifact with a snapshot schema."""
    import glob, json
    root = NORM + "/🗿️artifacts"
    out = {}
    for folder in sorted(os.listdir(root)):
        if shorts and not any(folder.endswith(s) for s in shorts):
            continue
        schema = glob.glob(f"{root}/{folder}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json")
        if schema:
            out[folder] = ui_findings(json.load(open(schema[0], encoding="utf8")))
    return out


def schema_limits(ctx):
    """📏 A plan must not add a numeric snapshot field without physical limits (`minimum`/`exclusiveMinimum` and `maximum`/`exclusiveMaximum`); integers carry their storage width. Whole-repo listing: `evolve.py verify limits`; the bounds come from `evolve.py limits <artifact> --apply`."""
    import json
    from core import SNAPSHOT
    import l_limits
    path = ctx.layout.p(SNAPSHOT + "/🔣️.json")
    after = l_limits.unbounded_fields(json.loads(ctx.plan.read(path)))
    before = l_limits.unbounded_fields(json.loads(open(path, encoding="utf8").read())) if os.path.exists(path) else []
    return [f"limits: {field} has no lower and upper bound; add them to the schema (evolve.py limits)" for field in after if field not in before]


CHECKS = (
    ("kind-kebab", lambda ctx: kind_kebab(ctx.plan)),
    ("nested-patch-order", lambda ctx: nested_patch_order(ctx.plan)),
    ("row-key", row_key_presence),
    ("crate-refs", crate_refs),
    ("snapshot-initializers", snapshot_initializers),
    ("canonical-json", canonical_json),
    ("set-field-kinds", set_field_kinds),
    ("ui-steppers", ui_steppers),
    ("limits", schema_limits),
)


def scan(ctx, only=None):
    """🔎 (check name, finding) for every check of `CHECKS` (or the names in `only`) over the planned files."""
    return [(name, finding) for name, fn in CHECKS if not only or name in only for finding in fn(ctx)]


def repo_set_field_gaps(shorts=None):
    """🗺️ {artifact folder: [gap paths]} over every artifact that has a snapshot schema and rows in the taxonomy (`evolve.py verify set-fields [artifact ...]`)."""
    import glob, json
    root = NORM + "/🗿️artifacts"
    taxonomy = json.load(open(TAXONOMY, encoding="utf8"))
    seen = {row["artifact"] for row in taxonomy["rows"]}
    out = {}
    for folder in sorted(os.listdir(root)):
        if folder not in seen or (shorts and not any(folder.endswith(s) for s in shorts)):
            continue
        schema = glob.glob(f"{root}/{folder}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json")
        if schema:
            out[folder] = set_field_gaps(json.load(open(schema[0], encoding="utf8")), taxonomy, folder)
    return out
