"""↩️ Compare the concrete EN 1990 inverse expressions with the frozen source baseline."""

from collections import Counter
from hashlib import sha256
import json
from pathlib import Path
import re
import sys


def semantic_tokens(body):
    body = re.sub(r"\breturn Vec::new\(\)", "return None", body)
    body = re.sub(r"\blet _ = (?:mutation|payload);", "", body)
    body = re.sub(r"vec!\[Op::from\((.+)\)\]", r"Some(\1)", body)
    body = re.sub(r"\.map\(\|item\| Some\((.+)\)\)\.unwrap_or_default\(\)", r".map(|item| \1)", body)
    return re.sub(r"\s+", "", body)


def logic(path, old):
    source = path.read_text(encoding="utf-8")
    adapter = source.split("pub fn inverse", 1)[1]
    adapter_header, _, adapter_body = adapter.partition(" {\n")
    adapter_destination = re.search(r"From<(crate::\w+::\w+)>", adapter_header)[1]
    if not old:
        function = source.split("fn previous", 1)[1]
        assert re.match(r"\s*Ok\(match previous\((\w+), base\) \{\s*Some\(inverse\) => vec!\[Op::from\(inverse\)\],\s*None => Vec::new\(\),\s*\}\)", adapter_body), path
    else:
        function = adapter
    header, _, body = function.partition(" {\n")
    body = body.rsplit("\n}", 1)[0].strip()
    if old:
        assert header.startswith("<Op>"), path
        closure = re.search(r"^Ok\(\(\|\| \{\n(.*)\n\s*\}\)\(\)\)$", body, re.S)
        body = closure[1] if closure else re.sub(r"(?m)^(\s*)Ok\((.+)\)$", r"\1\2", body)
    else:
        assert header.startswith("("), path
    destination = re.search(r"(?:From<|Option<)(crate::\w+::\w+)>", header)[1]
    assert adapter_destination == destination, path
    return destination, semantic_tokens(body)


def owner_map(root):
    source = (root / "🦀️.rs").read_text(encoding="utf-8")
    return {leaf: owner for owner, leaf in re.findall(r"^use (semio_s_artifact_norm_en1990_mut_\w+)::(\w+);", source, re.M)}


def audited_paths(root, capture, expected):
    if not capture:
        return [root / name for name in expected]
    paths = sorted(root.glob("*/↩️inverse/🦀️.rs"))
    return [path for path in paths if "\nfn previous(" not in path.read_text(encoding="utf-8")]


def fixture(folder, name):
    return json.loads((folder / f"🔣️.{name}.json").read_text(encoding="utf-8"))


def assert_fixture_vectors(root):
    fixtures = root.parents[1] / "🧪️tests/🧫️fixtures/🧬️mutations"
    members = fixtures / "🪚remove-member/🔬️middle-row"
    original = fixture(members, "before")["members"]
    modified = fixture(members, "after")["members"]
    mutation = fixture(members, "mutation")
    assert mutation["mutation"] == "removeMember"
    index = mutation["index"]
    restored = list(modified)
    restored.insert(index, original[index])
    assert restored == original
    effects = fixtures / "📎insert-effect/📏clamp"
    original = fixture(effects, "before")["effects"]
    modified = fixture(effects, "after")["effects"]
    mutation = fixture(effects, "mutation")
    assert mutation["mutation"] == "insertEffect"
    requested = mutation["index"]
    index = min(requested, len(original))
    assert requested > index and modified[:index] + modified[index + 1:] == original


def run(root, baseline, capture):
    expected = {} if capture else json.loads(baseline.read_text(encoding="utf-8"))
    owners = owner_map(root)
    observed = {}
    audited_owners = Counter()
    for path in audited_paths(root, capture, expected):
        destination, tokens = logic(path, capture)
        wrappers = (path.parent.parent / "🦀️.rs").read_text(encoding="utf-8")
        module = re.search(r"crate::(\w+)::inverse::inverse\(self, base\)", wrappers)[1]
        owner = owners[module]
        assert owner == owners[destination.split("::")[1]], path
        audited_owners[owner] += 1
        observed[path.relative_to(root).as_posix()] = sha256(json.dumps([owner, destination, tokens]).encode()).hexdigest()
    assert len(observed) == 31 and len(audited_owners) == 7
    if capture:
        print(json.dumps(observed, ensure_ascii=False, indent=2))
        return
    for path, digest in observed.items():
        assert digest == expected[path], path
    assert_fixture_vectors(root)
    print("31 concrete inverse computations match the baseline. Middle-row restoration and clamped insertion fixtures pass.")


if __name__ == "__main__":
    run(Path(sys.argv[1]), Path(sys.argv[2]), "--capture" in sys.argv[3:])
