#!/usr/bin/env python3
"""🆔️ F9 (prepared for the next landing window): every persisted composed-child / scene id is minted by ONE specified
function, `store::content_id(prefix, bytes)` = `<prefix>-` + the first 16 lowercase hex digits of SHA-256(bytes),
instead of `std::collections::hash_map::DefaultHasher` (unspecified across Rust releases, unreproducible by a second
implementation). The bytes are exactly what each site hashes today; a site that hashed several fields joins them with
U+001F. Excluded: in-memory revisions (block3d `world_fit_revision`), norm din18599 (slice N1). `--write` applies; the
default is a dry run that prints every hunk. Refuses a second run."""
import json
import re
import sys
from collections import OrderedDict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
STORE_ANCHOR = "pub struct ArtifactChild<S> {"
STORE_FN = '''/// 🆔️ The content-addressed id of a composed child or persisted scene: `<prefix>-` + the first 16 lowercase hex digits
/// of SHA-256 over `bytes`. Specified (the composition schema's `childId`), stable across processes, platforms and
/// toolchains, and reproducible by any implementation — unlike a `std` hasher, whose algorithm Rust leaves unspecified.
pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    format!("{prefix}-{}", &semio_framework_hash::sha256_hex(bytes)[..16])
}

'''
P = "✏️s/🔌️plugins/"
# (file, exact old text, new text)
SIMPLE = [
    ("🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs", "content_json", "jack-content", True),
    ("🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs", "content_json", "dag-content", True),
    ("🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs", "content_json", "sequence-content", False),
    ("✒️writer/🗿️artifacts/✒️writer/🦀️.rs", "content_json", "document", True),
    ("🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️.rs", "content_key", "gisterrain-mesh", True),
    ("📏️layout/🗿️artifacts/📏️layout/🦀️.rs", "content_json", "background-drawing", True),
    ("💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs", "mesh_json", "mesh", True),
    ("🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs", "canonical", "kind-catalogs", False),
    ("🧱️block/🗿️artifacts/🧊️3d/🦀️.rs", "canonical", "catalog", False),
    ("🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs", "canonical", "catalog", False),
]
write = "--write" in sys.argv
problems, edits = [], {}


def text_of(path):
    return edits.get(path) or path.read_text(encoding="utf-8")


def site(path, pattern, replacement, count=1):
    text = text_of(path)
    found = len(re.findall(pattern, text))
    if found != count:
        problems.append(f"{path.relative_to(ROOT)}: pattern found {found} times, expected {count}: {pattern[:90]}")
        return
    edits[path] = re.sub(pattern, lambda _: replacement, text)


H = r"    let mut hasher = std::collections::hash_map::DefaultHasher::new\(\);\n"
for rel, var, prefix, named in SIMPLE:
    path = ROOT / P / rel
    if named:
        site(path, H + rf"    {var}\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"{prefix}-\{{content_hash:016x\}}\"\);\n", f'    let child_id = store::content_id("{prefix}", {var}.as_bytes());\n')
    else:
        site(path, H + rf"    {var}\.hash\(&mut hasher\);\n    let child_id = format!\(\"{prefix}-\{{:016x\}}\", hasher\.finish\(\)\);\n", f'    let child_id = store::content_id("{prefix}", {var}.as_bytes());\n')
animate = ROOT / P / "🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs"
for prefix in ("presentation", "animation"):
    site(animate, H + rf"    content_json\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"{prefix}-\{{content_hash:016x\}}\"\);\n", f'    let child_id = store::content_id("{prefix}", content_json.as_bytes());\n')
imperative = ROOT / P / "📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs"
for prefix in ("imperative-flow", "imperative-text"):
    site(imperative, H + rf"    content_json\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"{prefix}-\{{content_hash:016x\}}\"\);\n", f'    let child_id = store::content_id("{prefix}", content_json.as_bytes());\n')
playbook = ROOT / P / "📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs"
for prefix in ("playbook-flow", "playbook-document"):
    site(playbook, H + rf"    content_json\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"{prefix}-\{{content_hash:016x\}}\"\);\n", f'    let child_id = store::content_id("{prefix}", content_json.as_bytes());\n')
architect = ROOT / P / "🏛️architect/🗿️artifacts/🏛️program/🦀️.rs"
for prefix in ("architect-benchmarks", "architect-knowledge"):
    site(architect, H + rf"    content_json\.hash\(&mut hasher\);\n    format!\(\"{prefix}-\{{:016x\}}\", hasher\.finish\(\)\)\n", f'    store::content_id("{prefix}", content_json.as_bytes())\n')
site(ROOT / P / "📋️forms/🗿️artifacts/📋️forms/🦀️.rs", H + r"    content_json\.hash\(&mut hasher\);\n    format!\(\"forms-scene-\{:016x\}\", hasher\.finish\(\)\)\n", '    store::content_id("forms-scene", content_json.as_bytes())\n')
site(ROOT / P / "🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs", H + r"    dsl::json::to_json_string\(content\)\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"shooting-emblem-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id("shooting-emblem", dsl::json::to_json_string(content).as_bytes());\n')
process = ROOT / P / "🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs"
site(process, H + r"    content\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"\{slug\}-brep-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id(&format!("{slug}-brep"), content.as_bytes());\n')
site(process, H + r"    serde_json::to_string\(&semio_framework_os_kernel::ToValue::to_value\(content\)\)\.unwrap_or_default\(\)\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"steps-flow-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id("steps-flow", dsl::json::to_json_string(content).as_bytes());\n')
site(ROOT / P / "🗒️note/🗿️artifacts/🗒️note/🦀️.rs", H + r"    block_id\.hash\(&mut hasher\);\n    content_json\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"note-text-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id("note-text", format!("{block_id}\\u{1f}{content_json}").as_bytes());\n')
site(ROOT / P / "📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs", H + r"    for chunk in chunks \{\n        chunk\.hash\(&mut hasher\);\n    \}\n    format!\(\"remodeling-mesh-io-\{:016x\}-\{:016x\}\", hasher\.finish\(\), chunks\.len\(\)\)\n", '    format!("{}-{:016x}", store::content_id("remodeling-mesh-io", chunks.join("\\u{1f}").as_bytes()), chunks.len())\n')

USE = "    use std::hash::{Hash, Hasher};\n"
for path in list(edits):
    text, out, cursor = edits[path], [], 0
    while (start := text.find(USE, cursor)) >= 0:
        end = text.find("\n}\n", start)
        body = text[start + len(USE):end if end >= 0 else len(text)]
        out.append(text[cursor:start])
        if ".hash(" in body or "Hasher" in body:
            out.append(USE)
        cursor = start + len(USE)
    out.append(text[cursor:])
    edits[path] = "".join(out)

store = STORE.read_text(encoding="utf-8")
if "pub fn content_id(" in store:
    problems.append("store: content_id already present")
elif store.count(STORE_ANCHOR) != 1:
    problems.append("store: ArtifactChild anchor not unique")
else:
    edits[STORE] = store.replace(STORE_ANCHOR, STORE_FN + STORE_ANCHOR)

LAW = STORE.parent / "🧪️tests/🪪️artifact-addressing/🦀️.rs"
LAW_FIXTURE = STORE.parent / "🧫️fixtures/🪪️artifact-addressing/🔣️.json"
law = LAW.read_text(encoding="utf-8")
fixture = json.loads(LAW_FIXTURE.read_text(encoding="utf-8"), object_pairs_hook=OrderedDict)
if "content_id_is_the_specified_sha256_prefix" in law or "contentIds" in fixture:
    problems.append("store: content_id law or fixture already present")
else:
    edits[LAW] = law.rstrip("\n") + """

/// 🆔️ `content_id` is `<prefix>-` + the first 16 hex digits of SHA-256: equal to Python `hashlib`'s answer for every
/// committed vector (`contentIds`), and independent of the process that computes it.
#[test]
fn content_id_is_the_specified_sha256_prefix() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-addressing/🔣️.json")).unwrap();
    for row in fixture["contentIds"].as_array().unwrap() {
        let id = content_id(row["prefix"].as_str().unwrap(), row["text"].as_str().unwrap().as_bytes());
        assert_eq!(id, row["id"].as_str().unwrap());
        assert_eq!(id, content_id(row["prefix"].as_str().unwrap(), row["text"].as_str().unwrap().as_bytes()));
    }
}
"""
    fixture["contentIds"] = json.loads((Path(__file__).parent / "content-id-vectors.json").read_text(encoding="utf-8"))["vectors"]
    edits[LAW_FIXTURE] = json.dumps(fixture, ensure_ascii=False, indent=2) + "\n"

for path in sorted(edits, key=str):
    print(("write " if write else "dry-run ") + str(path.relative_to(ROOT)))
for problem in problems:
    print("problem:", problem)
print(f"{len(edits)} files, {len(problems)} problems")
if problems:
    sys.exit(1)
if write:
    for path, text in edits.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
