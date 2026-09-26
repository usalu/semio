"""🧫️ N1 one-off: derives one production specification vector per mutation kind for every norm family.

Every vector comes from production code through the emitter (`emitter/main.rs`): a target snapshot is a rule-based
perturbation of a committed example (one scalar changed, one collection entry removed/copied/swapped), production's
editor derivation `from_snapshot(base, target)` must answer with EXACTLY ONE mutation, production dispatch must apply
it cleanly (no diagnostic), reach the target, and its own inverse must restore the base. Kinds `from_snapshot` never
emits get a payload built from the leaf's own payload schema and addresses read off the example, under the same
acceptance rule. Writes `.🧬semio/🌐hub/s13-n1-vectors/vectors-<family>.json`; `n1-materialize.py` turns those records into the tree.

    python3 n1-vectors.py <emitter-binary> [family …]
"""
import copy, json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
ARTIFACTS = f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = f"{ROOT}/.🧬semio/🌐hub/s13-n1-vectors"
FAMILIES = {"din16798": "🌬️din16798", "din18599": "⚡️din18599", "din4108": "🧱️din4108", "en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999", "iso16757": "📇️iso16757", "vdi3805": "🏭️vdi3805"}


class Emitter:
    def __init__(self, binary):
        self.process = subprocess.Popen([binary], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, encoding="utf-8")
        self.calls = 0

    def ask(self, request):
        self.calls += 1
        self.process.stdin.write(json.dumps(request, ensure_ascii=False) + "\n")
        self.process.stdin.flush()
        reply = json.loads(self.process.stdout.readline())
        if "error" in reply:
            raise ValueError(reply["error"])
        return reply["ok"]


def subset_dir(directory):
    return f"{ARTIFACTS}/{directory}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def examples(emitter, family, directory):
    assets = f"{subset_dir(directory)}/🖼️assets"
    found = []
    for dirpath, _dirs, files in os.walk(assets):
        if "🗣️.dsl.semio" in files:
            path = f"{dirpath}/🗣️.dsl.semio"
            try:
                found.append((os.path.relpath(path, assets), emitter.ask({"family": family, "op": "dsl", "text": open(path, encoding="utf-8").read()})))
            except ValueError as error:
                print(f"  [{family}] example {os.path.relpath(path, assets)} does not decode: {error}")
    return sorted(found, key=lambda item: (-richness(item[1]), item[0]))


def richness(value):
    if isinstance(value, dict):
        return sum(richness(child) for child in value.values())
    if isinstance(value, list):
        return 1 + len(value) + sum(richness(child) for child in value)
    return 0


def schema_enums(directory):
    enums = {}
    def walk(node):
        if isinstance(node, dict):
            for key, child in (node.get("properties") or {}).items():
                if isinstance(child, dict) and isinstance(child.get("enum"), list):
                    enums.setdefault(key, set()).update(value for value in child["enum"] if isinstance(value, str))
            for child in node.values():
                walk(child)
        elif isinstance(node, list):
            for child in node:
                walk(child)
    path = f"{subset_dir(directory)}/🧬️schema/📸️snapshot/🔣️.json"
    if os.path.exists(path):
        walk(json.load(open(path, encoding="utf-8")))
    return enums


def observed_strings(snapshots):
    seen = {}
    def walk(node, key):
        if isinstance(node, dict):
            for child_key, child in node.items():
                walk(child, child_key)
        elif isinstance(node, list):
            for child in node:
                walk(child, key)
        elif isinstance(node, str) and key is not None:
            seen.setdefault(key, set()).add(node)
    for snapshot in snapshots:
        walk(snapshot, None)
    return seen


def scalar_candidates(key, value, enums, strings):
    if isinstance(value, bool):
        return [not value]
    if isinstance(value, int):
        return [value + 1, value - 1 if value > 0 else value + 2]
    if isinstance(value, float):
        return [round(value * 1.25 + (0.0 if value else 0.5), 6), round(value * 0.8, 6)]
    if isinstance(value, str):
        if re.search(r"(^id$|Id$|_id$)", key):
            return []
        pool = sorted((enums.get(key) or set()) | (strings.get(key) or set()))
        options = [candidate for candidate in pool if candidate != value]
        if not options and not enums.get(key):
            options = [f"{value} (N1)" if value else "N1"]
        return options[:4]
    return []


def perturbations(snapshot, enums, strings):
    """🔀️ Every single-edit target of `snapshot`, shallow edits first."""
    def walk(node, path):
        if isinstance(node, dict):
            for key, child in node.items():
                if isinstance(child, (dict, list)):
                    yield from walk(child, path + [key])
                else:
                    for candidate in scalar_candidates(key, child, enums, strings):
                        yield path + [key], ("set", candidate), f"sets-{slug(key)}"
        elif isinstance(node, list):
            name = path[-1] if path else "items"
            if node:
                yield path, ("remove", 0), f"removes-the-first-{slug(singular(name))}"
                yield path, ("remove", len(node) - 1), f"removes-the-last-{slug(singular(name))}"
                yield path, ("append-copy", 0), f"appends-a-copy-of-the-first-{slug(singular(name))}"
                yield path, ("insert-copy", 0), f"inserts-a-copy-of-the-first-{slug(singular(name))}"
            if len(node) >= 2:
                yield path, ("swap", 0), f"swaps-the-first-two-{slug(name)}"
            for index, child in enumerate(node[:2]):
                yield from walk(child, path + [index])
    yield from walk(snapshot, [])


def singular(noun):
    noun = str(noun)
    return noun[:-3] + "y" if noun.endswith("ies") else noun[:-1] if noun.endswith("s") and not noun.endswith("ss") else noun


def slug(text):
    text = re.sub(r"([a-z0-9])([A-Z])", r"\1-\2", str(text))
    text = re.sub(r"([A-Z])([A-Z][a-z])", r"\1-\2", text).lower()
    return re.sub(r"[^a-z0-9]+", "-", text).strip("-") or "value"


def fresh_id(value, suffix):
    return f"{value}-{suffix}" if isinstance(value, str) else value


def reidentify(entry):
    entry = copy.deepcopy(entry)
    if isinstance(entry, dict) and isinstance(entry.get("id"), str):
        entry["id"] = f"{entry['id']}-n1"
    return entry


def edit(snapshot, path, operation):
    target = copy.deepcopy(snapshot)
    node = target
    for step in path[:-1] if operation[0] == "set" else path:
        node = node[step]
    verb, argument = operation
    if verb == "set":
        node[path[-1]] = argument
    elif verb == "remove":
        del node[argument]
    elif verb == "append-copy":
        node.append(reidentify(node[argument]))
    elif verb == "insert-copy":
        node.insert(0, reidentify(node[argument]))
    elif verb == "swap":
        node[0], node[1] = node[1], node[0]
    return target


def accept(emitter, family, base, mutation, target=None):
    result = emitter.ask({"family": family, "op": "apply", "base": base, "mutation": mutation})
    if not result.get("applied") or not result.get("changed") or result.get("messages") or not result.get("restores") or not result.get("inverse"):
        return None
    if target is not None and result["after"] != target:
        return None
    return result


def leaf_payload_schema(directory, owner, payload_schema):
    path = f"{ROOT}/{owner}/{payload_schema}"
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None


def all_ids(snapshot):
    ids = []
    def walk(node):
        if isinstance(node, dict):
            if isinstance(node.get("id"), str):
                ids.append(node["id"])
            for child in node.values():
                walk(child)
        elif isinstance(node, list):
            for child in node:
                walk(child)
    walk(snapshot)
    return ids


def entries_like(snapshot, properties):
    best = []
    def walk(node):
        if isinstance(node, dict):
            for child in node.values():
                walk(child)
        elif isinstance(node, list):
            for child in node:
                if isinstance(child, dict):
                    best.append((len(set(child) & set(properties or {})), child))
                walk(child)
    walk(snapshot)
    return [entry for score, entry in sorted(best, key=lambda item: -item[0]) if score > 0][:3]


def crafted_payloads(schema, snapshot):
    """🧪️ Payload candidates for one leaf from its own payload schema: ids read off the example, indices 0/1, values."""
    properties = (schema or {}).get("properties") or {}
    options = {}
    for key, spec in properties.items():
        kind = spec.get("type") if isinstance(spec, dict) else None
        kind = kind[0] if isinstance(kind, list) else kind
        if isinstance(spec, dict) and isinstance(spec.get("enum"), list):
            options[key] = spec["enum"][:4]
        elif re.search(r"(id|Id)$", key) and kind == "string":
            options[key] = all_ids(snapshot)[:12]
        elif key in ("index", "from", "to", "position") or kind == "integer":
            options[key] = [0, 1, 2]
        elif kind == "number":
            options[key] = [1.5, 0.5, 10.0, 100.0]
        elif kind == "boolean":
            options[key] = [True, False]
        elif kind == "string":
            options[key] = ["N1"]
        elif kind == "object":
            options[key] = [reidentify(entry) for entry in entries_like(snapshot, (spec.get("properties") or {}))] or [{}]
        elif kind == "array":
            options[key] = [[]]
        else:
            options[key] = [None]
    keys = list(options)
    combos = [{}]
    for key in keys:
        combos = [dict(combo, **{key: value}) for combo in combos for value in options[key]][:400]
    return combos


def wire(template, variant, payload):
    if isinstance(template, dict) and "mutation" in template:
        return dict({"mutation": variant[0].lower() + variant[1:]}, **payload)
    return {variant: payload}


def derive_family(emitter, family, directory):
    kinds = emitter.ask({"family": family, "op": "kinds"})
    wanted = {descriptor["semanticKind"]: descriptor for descriptor in kinds}
    bases = examples(emitter, family, directory)
    if not bases:
        return {"family": family, "vectors": {}, "missing": sorted(wanted), "examples": []}
    enums = schema_enums(directory)
    strings = observed_strings([snapshot for _name, snapshot in bases])
    vectors, template = {}, None
    for asset, base in bases:
        for path, operation, label in perturbations(base, enums, strings):
            if len(vectors) == len(wanted):
                break
            target = edit(base, path, operation)
            try:
                derived = emitter.ask({"family": family, "op": "derive", "base": base, "target": target})
            except ValueError:
                continue
            if len(derived["kinds"]) != 1 or derived["kinds"][0] in vectors:
                continue
            template = template or derived["mutations"][0]
            result = accept(emitter, family, base, derived["mutations"][0], derived["target"])
            if result is None:
                continue
            kind = derived["kinds"][0]
            vectors[kind] = {"kind": kind, "example": asset, "rule": f"{operation[0]} {'/'.join(map(str, path))}", "scenario": scenario_name(kind, label, path), "before": base, "mutation": derived["mutations"][0], "after": result["after"], "diff": result["diff"], "outcome": {"status": "applied"}, "derivation": "from_snapshot"}
    for kind, descriptor in wanted.items():
        if kind in vectors:
            continue
        schema = leaf_payload_schema(directory, descriptor["owner"], descriptor["payloadSchema"])
        for asset, base in bases:
            found = None
            for payload in crafted_payloads(schema, base):
                try:
                    result = accept(emitter, family, base, wire(template, descriptor["aggregateVariant"], payload))
                except ValueError:
                    continue
                if result is not None and result.get("kind") == kind:
                    found = (payload, result)
                    break
            if found:
                payload, result = found
                vectors[kind] = {"kind": kind, "example": asset, "rule": "payload " + ",".join(sorted(payload)), "scenario": scenario_name(kind, "applies-" + kind, []), "before": base, "mutation": wire(template, descriptor["aggregateVariant"], payload), "after": result["after"], "diff": result["diff"], "outcome": {"status": "applied"}, "derivation": "payload-schema"}
                break
    missing = sorted(set(wanted) - set(vectors))
    return {"family": family, "examples": [asset for asset, _ in bases], "vectors": vectors, "missing": missing, "descriptors": wanted}


def scenario_name(kind, label, path):
    where = "-".join(slug(step) for step in path[:-1] if isinstance(step, str))[-40:].strip("-")
    text = label if not where or where in label else f"{label}-of-the-{where}"
    return re.sub(r"-+", "-", text)[:72].strip("-")


def main():
    binary, selected = sys.argv[1], sys.argv[2:] or list(FAMILIES)
    os.makedirs(OUT, exist_ok=True)
    emitter = Emitter(binary)
    for family in selected:
        record = derive_family(emitter, family, FAMILIES[family])
        json.dump(record, open(f"{OUT}/vectors-{family}.json", "w", encoding="utf-8"), ensure_ascii=False)
        by = {}
        for vector in record["vectors"].values():
            by[vector["derivation"]] = by.get(vector["derivation"], 0) + 1
        print(f"{family}: {len(record['vectors'])}/{len(record.get('descriptors', {})) or '?'} kinds vectored {by}; missing {record['missing']}; emitter calls {emitter.calls}", flush=True)


if __name__ == "__main__":
    main()
