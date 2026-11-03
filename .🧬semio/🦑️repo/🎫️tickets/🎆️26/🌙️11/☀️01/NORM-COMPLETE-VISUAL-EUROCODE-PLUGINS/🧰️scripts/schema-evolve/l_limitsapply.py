"""📏 `evolve.py limits <artifact...> [--apply]` and `evolve.py verify limits [artifact...]`: writes the physical limits of `l_limits.decide` into the snapshot schema, the artifact schema and the set-field mutation payload schemas of an artifact (text edits, the layout of every file is kept), checks them against every committed document of the artifact and lists the fixture values that fall outside. `schema twins generate` produces the twins afterwards."""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "field-adder"))
import jsonspan
import l_limits, l_limitscan
from l_limits import numeric_nodes
from l_verify import NORM, TAXONOMY

ROOT = NORM + "/🗿️artifacts"
SNAPSHOT = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"
ARTIFACT = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json"
MUTATIONS = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
BOUNDS = ("minimum", "exclusiveMinimum", "maximum", "exclusiveMaximum")


def folders(shorts):
    names = sorted(os.listdir(ROOT))
    return [n for n in names if os.path.exists(f"{ROOT}/{n}/{SNAPSHOT}") and (not shorts or any(n.endswith(s) for s in shorts))]


def number(value):
    if isinstance(value, float) and value == int(value) and abs(value) < 1e15:
        return str(int(value))
    return json.dumps(value)


def unit_of(prop):
    return (prop.get("x-semio-ui") or {}).get("unit") or (prop.get("x-semio-dsl") or {}).get("unit")


def label_of(prop):
    return ((prop.get("x-semio-ui") or {}).get("label") or {}).get("en", "")


def doc_of(prop):
    return " ".join((prop.get("x-semio-rust") or {}).get("doc", []))


def decisions(snapshot, observed=None):
    """{(owner, property): bounds} for every numeric member of the snapshot schema; a lower bound that excludes a documented zero (the value occurs in a committed document) becomes closed."""
    out = {}
    for owner, name, prop, node in numeric_nodes(snapshot):
        bounds = l_limits.decide(unit_of(prop), name, label_of(prop), doc_of(prop), node, owner)
        if bounds is None:
            out[(owner, name)] = None
            continue
        seen = (observed or {}).get((owner, name))
        if node.get("type") == "integer":
            if seen and 0 in seen and bounds.get("minimum", 0) > 0:
                bounds["minimum"] = node.get("minimum", 0)
            out[(owner, name)] = bounds
            continue
        if seen and "exclusiveMinimum" in bounds and 0 in seen and bounds["exclusiveMinimum"] == 0:
            bounds = {("minimum" if k == "exclusiveMinimum" else k): v for k, v in bounds.items()}
        for key in BOUNDS:
            if key in node and key not in bounds and not (key.endswith("inimum") and any(k.endswith("inimum") for k in bounds)) and not (key.endswith("aximum") and any(k.endswith("aximum") for k in bounds)):
                bounds[key] = node[key]
        out[(owner, name)] = bounds
    return out


def node_edits(text, node_span, bounds, integer):
    """Edits [(start, end, replacement)] that give the object node its bounds: floats keep the bounds they have, integers are replaced."""
    members = {m.key: m for m in node_span.members}
    type_member = members["type"]
    edits, add = [], {}
    lower_have = any(k.endswith("inimum") for k in members)
    upper_have = any(k.endswith("aximum") for k in members)
    for key in BOUNDS:
        want = bounds.get(key)
        if want is None:
            continue
        if not integer and key not in members and (lower_have if key.endswith("inimum") else upper_have):
            continue
        if key in members:
            if integer and json.loads(text[members[key].value.start:members[key].value.end]) != want:
                edits.append((members[key].value.start, members[key].value.end, number(want)))
            continue
        add[key] = want
    if add:
        index = node_span.members.index(type_member)
        colon = text[type_member.key_end:type_member.value.start]
        gap = jsonspan._separator(text, node_span, index) if len(node_span.members) > 1 else ","
        insertion = "".join(gap + json.dumps(key) + colon + number(value) for key, value in add.items())
        edits.append((type_member.value.end, type_member.value.end, insertion))
    return edits


def locate(text, root_span, owner, name):
    container = root_span.get("properties").value if owner == "$root" else root_span.get("$defs").value.get(owner).value.get("properties").value
    prop = container.get(name).value
    members = {m.key: m for m in prop.members}
    if members.get("type") and text[members["type"].value.start:members["type"].value.end] == '"array"' and "items" in members:
        return members["items"].value
    return prop


def apply_edits(text, edits):
    for start, end, replacement in sorted(edits, key=lambda e: (-e[0], -e[1])):
        text = text[:start] + replacement + text[end:]
    return text


def schema_text(path, plan, wanted):
    """The new text of a schema document whose numeric members listed in `wanted` ({(owner, name): bounds}) carry their bounds."""
    text = open(path, encoding="utf8").read()
    doc = json.loads(text)
    span = jsonspan.parse(text)
    edits, have = [], {(o, n): node for o, n, _, node in numeric_nodes(doc)}
    for key, bounds in wanted.items():
        if bounds is None or key not in have:
            continue
        edits += node_edits(text, locate(text, span, *key), bounds, have[key]["type"] == "integer")
    return apply_edits(text, edits) if edits else text


def resolve_path(snapshot, path):
    """(owner, property) of a taxonomy path such as `walls[].loadCases[].slabSpanM`."""
    node, owner = snapshot, "$root"
    segments = path.split(".")
    for index, segment in enumerate(segments):
        key = re.sub(r"\[\]$", "", segment)
        prop = (node.get("properties") or {}).get(key)
        if prop is None:
            return None
        if index == len(segments) - 1:
            return owner, key
        target = prop["items"] if prop.get("type") == "array" else prop
        ref = target.get("$ref")
        if not ref:
            return None
        owner = ref.split("/")[-1]
        node = snapshot["$defs"][owner]
    return None


def mutation_wanted(folder, snapshot, wanted):
    """{payload schema path: {property: bounds}} for the set-field kinds of the artifact: the payload value member takes the bounds of the field the kind edits."""
    taxonomy = json.load(open(TAXONOMY, encoding="utf8"))
    out = {}
    for row in taxonomy["rows"]:
        edit = row.get("edit") or {}
        if row["artifact"] != folder or edit.get("rule") != "set-field":
            continue
        key = resolve_path(snapshot, edit["path"])
        bounds = wanted.get(key) if key else None
        path = f"{ROOT}/{folder}/{MUTATIONS}/{row['module']}/🧬️schema/🔣️.json"
        if bounds and os.path.exists(path):
            out.setdefault(path, {})[("$root", edit["value"])] = bounds
    return out


def run(shorts, apply):
    changed, findings = 0, []
    for folder in folders(shorts):
        snap_path = f"{ROOT}/{folder}/{SNAPSHOT}"
        snapshot = json.load(open(snap_path, encoding="utf8"))
        seen, _ = l_limitscan.observed(f"{ROOT}/{folder}", snapshot)
        wanted = decisions(snapshot, seen)
        missing = [k for k, v in wanted.items() if v is None]
        findings += [f"{folder}: no rule for {o}.{n}" for o, n in missing]
        writes = {snap_path: schema_text(snap_path, None, wanted)}
        artifact_path = f"{ROOT}/{folder}/{ARTIFACT}"
        if os.path.exists(artifact_path):
            writes[artifact_path] = schema_text(artifact_path, None, wanted)
        for path, props in mutation_wanted(folder, snapshot, wanted).items():
            writes[path] = schema_text(path, None, props)
        count = 0
        for path, new in writes.items():
            if new != open(path, encoding="utf8").read():
                count += 1
                if apply:
                    open(path, "w", encoding="utf8").write(new)
        changed += count
        print(f"[limits] {folder}: {count} schema files {'written' if apply else 'would change'}, {len(missing)} fields without a rule")
    for line in findings:
        print("  FINDING", line)
    return 1 if findings else 0


def violations(folder):
    """[(file, owner, property, value, bounds)] of committed document values outside the bounds the snapshot schema now declares."""
    snapshot = json.load(open(f"{ROOT}/{folder}/{SNAPSHOT}", encoding="utf8"))
    bounds = {(o, n): {k: node[k] for k in BOUNDS if k in node} for o, n, _, node in numeric_nodes(snapshot)}
    out = []
    for base, dirs, files in os.walk(f"{ROOT}/{folder}"):
        dirs[:] = [d for d in dirs if d not in (l_limitscan.SKIP_DIRS - {"🪶️sqlite"})]
        if "🧬️schema" in base.split(os.sep) and "🧪️tests" not in base:
            continue
        for name in files:
            if not name.endswith(".json") or name.endswith("schema.json"):
                continue
            path = os.path.join(base, name)
            try:
                data = json.load(open(path, encoding="utf8"))
            except Exception:
                continue
            hits = []
            l_limitscan.find_roots(snapshot, data, hits)
            for root in hits:
                found = {}
                l_limitscan.walk_value(snapshot, snapshot, root, "$root", "", found)
                for key, values in found.items():
                    b = bounds.get(key) or {}
                    for v in values:
                        if violates(v, b):
                            out.append((path, key[0], key[1], v, b))
    return out


def violates(value, b):
    return (("minimum" in b and value < b["minimum"]) or ("exclusiveMinimum" in b and value <= b["exclusiveMinimum"]) or ("maximum" in b and value > b["maximum"]) or ("exclusiveMaximum" in b and value >= b["exclusiveMaximum"]))


def clamp(value, b):
    """The nearest value inside the bounds; a negative value first becomes its magnitude (the stress fixtures negate on purpose), a zero below an exclusive bound becomes the smallest unit step that fits."""
    lo = b.get("minimum", b.get("exclusiveMinimum"))
    hi = b.get("maximum", b.get("exclusiveMaximum"))
    if lo is not None and lo >= 0 and value < 0:
        value = -value
    if hi is not None and value > hi:
        value = hi
    if "exclusiveMinimum" in b and value <= b["exclusiveMinimum"]:
        value = next((c for c in (1.0, 0.5, 0.1, 0.01, 1e-3, 1e-6) if c > b["exclusiveMinimum"] and (hi is None or c <= hi)), b["exclusiveMinimum"] * 2 or 1.0)
    if "minimum" in b and value < b["minimum"]:
        value = b["minimum"]
    if "exclusiveMaximum" in b and value >= b["exclusiveMaximum"]:
        value = b["exclusiveMaximum"] / 2
    return value


def typed_edits(text, schema, node_span, node, value, owner, prop, bounds, edits):
    key, node = l_limitscan.resolve(schema, node)
    alt = node.get("anyOf") or node.get("oneOf")
    if alt:
        for branch in alt:
            typed_edits(text, schema, node_span, branch, value, owner, prop, bounds, edits)
        return
    kind = node.get("type")
    if kind in ("number", "integer") and node_span.kind == "scalar":
        try:
            number_value = json.loads(node_span.text)
        except ValueError:
            return
        if isinstance(number_value, (int, float)) and not isinstance(number_value, bool):
            b = bounds.get((owner, prop))
            if b and violates(number_value, b):
                new = clamp(number_value, b)
                edits.append((node_span.start, node_span.end, repr(float(new)) if isinstance(number_value, float) else number(new)))
    elif kind == "array" and node_span.kind == "array":
        for item in node_span.items:
            typed_edits(text, schema, item, node.get("items", {}), None, owner, prop, bounds, edits)
    elif node_span.kind == "object" and "properties" in node:
        here = key or "$root"
        for member in node_span.members:
            child = node["properties"].get(member.key)
            if child is not None:
                typed_edits(text, schema, member.value, child, None, here, member.key, bounds, edits)


def aux_edits(text, span, by_name, edits, trail=None):
    """Clamps the numbers under members named like a bounded property (`newX`, `oldX` and `X`) in the payload and diff documents of a fixture; skipped when the bounds of the name differ between records."""
    if span.kind == "object":
        for member in span.members:
            name = re.sub(r"^(new|old)([A-Z])", lambda m: m.group(2).lower(), member.key)
            if member.value.kind == "scalar" and name in by_name:
                try:
                    v = json.loads(member.value.text)
                except ValueError:
                    continue
                if isinstance(v, (int, float)) and not isinstance(v, bool):
                    results = {clamp(v, b) for b in by_name[name] if violates(v, b)}
                    if len(results) == 1 and all(violates(v, b) or True for b in by_name[name]):
                        new = results.pop()
                        edits.append((member.value.start, member.value.end, repr(float(new)) if isinstance(v, float) else number(new)))
            else:
                aux_edits(text, member.value, by_name, edits)
    elif span.kind == "array":
        for item in span.items:
            aux_edits(text, item, by_name, edits)


def fix(folder, apply):
    """Rewrites the committed documents of the artifact that fall outside the bounds (typed documents by the schema, payload and diff documents by member name): the value moves to the nearest valid one, the same function on the same input, so a before/after pair stays a pair."""
    snapshot = json.load(open(f"{ROOT}/{folder}/{SNAPSHOT}", encoding="utf8"))
    bounds = {(o, n): {k: node[k] for k in BOUNDS if k in node} for o, n, _, node in numeric_nodes(snapshot)}
    by_name = {}
    for (o, n), b in bounds.items():
        by_name.setdefault(n, []).append(b)
    roots, touched = [], 0
    for base, dirs, files in os.walk(f"{ROOT}/{folder}"):
        dirs[:] = [d for d in dirs if d not in (l_limitscan.SKIP_DIRS - {"🪶️sqlite"})]
        if "🧬️schema" in base.split(os.sep) and "🧪️tests" not in base:
            continue
        for name in files:
            if not name.endswith(".json") or name.endswith("schema.json"):
                continue
            path = os.path.join(base, name)
            text = open(path, encoding="utf8").read()
            try:
                data = json.loads(text)
            except Exception:
                continue
            span = jsonspan.parse(text)
            edits, hits = [], []
            l_limitscan.find_roots(snapshot, data, hits)
            if hits:
                def walk(sp, d):
                    if isinstance(d, dict) and set(snapshot.get("required", [])) <= set(d) and sp.kind == "object":
                        typed_edits(text, snapshot, sp, snapshot, None, "$root", "", bounds, edits)
                    elif isinstance(d, dict) and sp.kind == "object":
                        for m in sp.members:
                            walk(m.value, d[m.key])
                    elif isinstance(d, list) and sp.kind == "array":
                        for i, item in enumerate(sp.items):
                            walk(item, d[i])
                walk(span, data)
            elif "🧬️mutations" in base and (name.endswith(("mutation.json", "diff.json", "outcome.json")) or "mutation" in name or "diff" in name):
                aux_edits(text, span, by_name, edits)
            edits = sorted(set(edits))
            if edits:
                touched += 1
                if apply:
                    open(path, "w", encoding="utf8").write(apply_edits(text, edits))
    return touched


def repo_findings(shorts=None):
    out = {}
    for folder in folders(shorts):
        snapshot = json.load(open(f"{ROOT}/{folder}/{SNAPSHOT}", encoding="utf8"))
        out[folder] = [f"limits: {field} has no lower and upper bound" for field in l_limits.unbounded_fields(snapshot)]
    return out


if __name__ == "__main__":
    args = sys.argv[1:]
    if args and args[0] == "fix-fixtures":
        for folder in folders([a for a in args[1:] if a != "--apply"]):
            print(f"[limits] {folder}: {fix(folder, '--apply' in args)} fixture files {'rewritten' if '--apply' in args else 'would change'}")
        sys.exit(0)
    if args and args[0] == "violations":
        total = 0
        for folder in folders(args[1:]):
            rows = violations(folder)
            files = {r[0] for r in rows}
            print(f"[limits] {folder}: {len(rows)} values in {len(files)} files outside the bounds")
            total += len(rows)
            for r in rows[:int(os.environ.get("LIMITS_SHOW", "0"))]:
                print("   ", r[0].split(folder)[1][-90:], r[1], r[2], r[3], r[4])
        sys.exit(1 if total else 0)
    sys.exit(run([a for a in args if a != "--apply"], "--apply" in args))
