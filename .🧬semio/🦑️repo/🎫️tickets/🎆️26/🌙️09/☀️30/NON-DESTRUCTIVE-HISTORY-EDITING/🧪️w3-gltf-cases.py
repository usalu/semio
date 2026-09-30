#!/usr/bin/env python3
"""🪢️ W3-GLTF: restores the canonical mutation case pair for every glTF leaf.

Each committed fixture bundle `♾️any/🧫️fixtures/🧬️mutations/<entity>/<verb>/<case>/` gets its implementation case
`♾️any/🧬️schema/🧬️mutations/<entity>/<verb>/🧪️tests/<case>/🦀️.rs`, mounted by the leaf in place of the former
`🧪️tests/🔬️direct-leaf` (whose semantic-identity assertion moves into the case) and, for the two material leaves, of
`🧪️tests/🔬️unit` (whose apply/reject test moves into the case). The case calls the shared corpus law `assert_case`.
`--check` reports what would change and writes nothing."""
import re
import shutil
import sys
from pathlib import Path

SUBSET = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any")
SCHEMA = SUBSET / "🧬️schema/🧬️mutations"
CORPUS = SUBSET / "🧫️fixtures/🧬️mutations"
CHECK = "--check" in sys.argv
EMOJI = re.compile(r"^(?:[\U0001F000-\U0001FAFF☀-➿⬀-⯿←-⇿⌀-⏿㊐-㋿]️?(?:‍)?)+")
MOUNT = '#[cfg(test)]\n#[path = "🧪️tests/{path}/🦀️.rs"]\nmod {name};'

changes = []


def slug(name: str) -> str:
    rest = EMOJI.sub("", name)
    assert re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", rest), name
    return rest


def write(path: Path, text: str) -> None:
    if path.is_file() and path.read_text(encoding="utf-8") == text:
        return
    changes.append(f"write {path.relative_to(SUBSET)}")
    if not CHECK:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")


def remove(path: Path) -> None:
    if not path.exists():
        return
    changes.append(f"remove {path.relative_to(SUBSET)}")
    if not CHECK:
        shutil.rmtree(path)


for fixture in sorted(CORPUS.glob("*/*/*")):
    if not (fixture / "🦠️mutation/🔣️.json").is_file():
        continue
    entity, verb, case = fixture.relative_to(CORPUS).parts
    leaf = SCHEMA / entity / verb
    source = (leaf / "🦀️.rs").read_text(encoding="utf-8")
    direct = leaf / "🧪️tests/🔬️direct-leaf/🦀️.rs"
    unit = leaf / "🧪️tests/🔬️unit/🦀️.rs"
    kind = re.search(r'kind: "([a-z0-9-]+)"', source).group(1)
    implemented = leaf / "🧪️tests" / case / "🦀️.rs"
    witness = direct if direct.is_file() else implemented if implemented.is_file() else None
    if witness:
        identity = next(line.strip() for line in witness.read_text(encoding="utf-8").splitlines() if "SEMANTICS.kind" in line)
    else:
        record = re.search(r"impl protocol::MutationKind<GltfSnapshot, GltfMutation> for (\w+)", source).group(1)
        identity = f'assert_eq!(<{record} as protocol::MutationKind<GltfSnapshot, GltfMutation>>::SEMANTICS.kind, "{kind}");'
    extra = ""
    if unit.is_file():
        body = unit.read_text(encoding="utf-8").split("use super::*;\n", 1)[1].strip("\n")
        extra = f"\n\n{body}"
    elif not direct.is_file() and implemented.is_file():
        tail = implemented.read_text(encoding="utf-8").split("\n}\n", 1)[1].strip("\n")
        extra = f"\n\n{tail}" if tail else ""
    name = f"case_{slug(case).replace('-', '_')}"
    emoji = EMOJI.match(case).group(0)
    relative = f"{entity}/{verb}/{case}"
    write(
        leaf / "🧪️tests" / case / "🦀️.rs",
        f"""//! {emoji} `{kind}` implementation case `{case}`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/{relative}/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {{
    {identity}
    super::super::component::fixture_corpus_tests::assert_case("{relative}");
}}{extra}
""",
    )
    mount = MOUNT.format(path=case, name=name)
    updated = source
    if direct.is_file():
        old = MOUNT.format(path="🔬️direct-leaf", name="direct_leaf_tests")
        assert updated.count(old) == 1, leaf
        updated = updated.replace(old, mount)
    elif mount not in updated:
        updated = f"{updated.rstrip()}\n\n//#region 🧪️Tests\n{mount}\n//#endregion 🧪️Tests\n"
    if unit.is_file():
        old = MOUNT.format(path="🔬️unit", name="tests") + "\n"
        assert updated.count(old) == 1, leaf
        updated = updated.replace(old, "")
    write(leaf / "🦀️.rs", updated)
    remove(direct.parent)
    remove(unit.parent)

print(f"[w3-gltf-cases] {'would change' if CHECK else 'changed'}={len(changes)}")
for change in changes:
    print(f"  {change}")
if CHECK and changes:
    sys.exit(1)
