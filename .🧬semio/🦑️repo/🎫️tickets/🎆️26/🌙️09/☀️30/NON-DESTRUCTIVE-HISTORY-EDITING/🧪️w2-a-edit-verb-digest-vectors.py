"""🔗️ Derives the `verb` edit-digest vectors (a third implementation beside the Rust store and the TS oracle) and appends
them to the language-neutral `🔗️edit-digest-chains.json` fixture after re-deriving every existing vector."""
import hashlib, json, pathlib, struct, copy

FIXTURE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json")


def u64(value):
    return struct.pack(">Q", value)


def record(domain, parts):
    h = hashlib.sha256(b"semio.artifact.cursor.v2" + u64(len(domain.encode())) + domain.encode())
    for part in parts:
        h.update(u64(len(part)) + part)
    return h.digest()


def stringify(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode()


def chain(domain, items):
    state = bytes(32)
    for item in items:
        state = record(domain, [state, stringify(item)])
    return state


def text(edit, key):
    return [bytes([1 if key in edit else 0]), str(edit.get(key, "")).encode()]


def edit_digest(edit):
    forwards, inverse, meta = edit["forwards"], edit["inverse"], edit.get("mutationMeta", [])
    if len(forwards) <= 1:
        return record("edit", [edit["id"].encode(), stringify(edit)])
    chained = record("edit-chained", [edit["id"].encode(), *text(edit, "actor"), *text(edit, "description"), *text(edit, "coalesceKey"), struct.pack(">i", edit["sequenceNumber"]), edit["startedAt"].encode(), *text(edit, "finishedAt"), u64(len(forwards)), chain("edit-forward", forwards), u64(len(inverse)), chain("edit-inverse", inverse), u64(len(meta)), chain("edit-meta", meta)])
    return record("edit-verb", [chained, edit["verb"].encode()]) if "verb" in edit else chained


def expand(case):
    if "edit" in case:
        return case["edit"]
    edit = dict(case["header"])
    n = case["generatedOperations"]
    edit["forwards"] = [{"SetN": {"n": i + 1}} for i in range(n)]
    edit["inverse"] = [{"SetN": {"n": i}} for i in range(n)]
    return edit


def with_verb(edit, verb):
    out = {}
    for key, value in edit.items():
        out[key] = copy.deepcopy(value)
        if key == "description":
            out["verb"] = verb
    return out


fixture = json.loads(FIXTURE.read_text())
for case in fixture["cases"]:
    assert edit_digest(expand(case)).hex() == case["expectedDigest"], case["name"]
fixture["cases"] = [case for case in fixture["cases"] if not case["name"].endswith("-with-verb")]
base = {case["name"]: case for case in fixture["cases"]}
for name, source in [("single-operation-with-verb", "single-operation"), ("grown-to-two-with-verb", "grown-to-two")]:
    edit = with_verb(base[source]["edit"], "typeText")
    fixture["cases"].append({"name": name, "edit": edit, "expectedDigest": edit_digest(edit).hex()})
FIXTURE.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
print([(case["name"], case["expectedDigest"]) for case in fixture["cases"]])
