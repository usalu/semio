import copy, json, sys
fixtures = sys.argv[1]
def load(path):
    text = open(path, encoding="utf-8").read()
    document = json.loads(text)
    assert json.dumps(document, indent=2, ensure_ascii=False) + "\n" == text, path
    return document
def save(path, document):
    open(path, "w", encoding="utf-8", newline="\n").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
def rows(node):
    if isinstance(node, list):
        for row in node:
            yield from rows(row)
    elif isinstance(node, dict):
        yield node
        for value in node.values():
            yield from rows(value)
def row(document, identifier):
    return next(r for r in rows(document) if r.get("id") == identifier)
for leaf, identifier, field, other in (
    ("🌉️change-bridge-method", "br-1", "method", "SafeLife"),
    ("📏️change-member-warping-support", "member-b1", "warpingSupport", "Fork"),
    ("🕸️change-frame-bar-release-end", "bar-1", "releaseEnd", "Pinned"),
    ("🧱️change-plated-stiffener-shape", "pp-1", "stiffenerShape", "None"),
    ("🧷️change-restraint-end-support", "restraint-1", "endSupport", "Fixed"),
):
    path = f"{fixtures}/{leaf}/✅apply/🔣️.before.json"
    document = load(path)
    row(document, identifier)[field] = other
    save(path, document)
for leaf, old, new, field in (
    ("🔩️change-joint-weld-force-longitudinal", "joint-j1", "joint", "weldForceLongitudinal"),
    ("🔩️change-joint-weld-force-transverse", "joint-j1", "joint", "weldForceTransverse"),
    ("🧱️change-panel-design-temperature", "pp-1", "plate", "designTemperature"),
):
    base = f"{fixtures}/{leaf}/✅apply/🔣️"
    mutation = load(f"{base}.mutation.json")
    (name, payload), = mutation.items()
    payload["id"] = new
    save(f"{base}.mutation.json", mutation)
    before = load(f"{base}.before.json")
    after = copy.deepcopy(before)
    row(after, new)[field] = next(v for k, v in payload.items() if k != "id")
    save(f"{base}.after.json", after)
print("ok")
