#!/usr/bin/env python3
"""🆔️ F9 (T14, extends T13's `wp-t13/f9/content-id.py`): every persisted composed-child / scene id is minted by ONE
specified function, `store::content_id(prefix, bytes)` = `<prefix>-` + the first 16 lowercase hex digits of
SHA-256(bytes), instead of `std::collections::hash_map::DefaultHasher` (unspecified across Rust releases, unreproducible
by a second implementation). The bytes are exactly what each site hashes today; a site that hashed several fields joins
them with U+001F. 29 minting sites: T13's 23 plus reasoning wires, cad models, raster assets (both handles), remodeling
assets and norm din18599 climate scenes (all six reach `ArtifactChild::new` / `ArtifactRef.artifact_id` and were missed
because their `format!` sits in a helper that receives a `u64`). TypeScript: a first-party synchronous SHA-256 in the
hash module (twin of Rust `Sha256`), `contentId` beside `parseArtifactChild`, and the remodeling mutation twin drops its
SipHash-1-3 transliteration of `DefaultHasher`. Schema: the child schema states `ContentId`. Laws: Rust + TS against the
Python `hashlib` vectors (third-party oracle). Excluded: in-memory revisions (block3d `world_fit_revision`, cad/puzzle
edit-mode revisions), digests that are not ids (inference `content_digest` fields), draw element ids.

usage: content-id.py [--write] [--root <tree>]   (default: dry run on the live tree; refuses a second run)"""
import json
import os
import re
import sys
from collections import OrderedDict
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent
write = "--write" in sys.argv
problems, edits = [], {}
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
STORE_ANCHOR = "pub struct ArtifactChild<S> {"
STORE_FN = '''/// 🆔️ The content-addressed id of a composed child or persisted scene: `<prefix>-` + the first 16 lowercase hex digits
/// of SHA-256 over `bytes` (the child schema's `ContentId`). Specified, stable across processes, platforms and
/// toolchains, and reproducible by any implementation — unlike a `std` hasher, whose algorithm Rust leaves unspecified.
/// A site that addresses several fields joins them with U+001F. TypeScript twin: `contentId` (`🪆️child/🧬️schema/🟦️.ts`).
pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    format!("{prefix}-{}", semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(bytes)[..8]))
}

'''
P = ROOT / "✏️s/🔌️plugins"
SENTINELS = [
    (STORE, "pub fn content_id(prefix: &str, bytes: &[u8]) -> String {"),
    (STORE.parent / "🪆️child/🧬️schema/🟦️.ts", "export function contentId"),
    (STORE.parent / "🪆️child/🧬️schema/🔣️.json", '"ContentId"'),
    (STORE.parent / "🧪️tests/🪪️artifact-addressing/🟦️.ts", "export function testContentIdOracle"),
    (ROOT / "📜️script.ts", "testContentIdOracle();"),
]
present = [marker in path.read_text(encoding="utf-8") for path, marker in SENTINELS]
if all(present):
    print("nothing to do (applied): content_id, TS contentId, schema ContentId, TS oracle and its runner are all present")
    sys.exit(0)
if any(present):
    raise SystemExit(f"partially applied tree: {[str(path.relative_to(ROOT)) for (path, _), hit in zip(SENTINELS, present) if hit]}")


def text_of(path):
    return edits.get(path) or path.read_text(encoding="utf-8")


def site(path, pattern, replacement, count=1):
    text = text_of(path)
    found = len(re.findall(pattern, text))
    if found != count:
        problems.append(f"{path.relative_to(ROOT)}: pattern found {found} times, expected {count}: {pattern[:90]}")
        return
    edits[path] = re.sub(pattern, lambda _: replacement, text)


def exact(path, old, new):
    text = text_of(path)
    if text.count(old) != 1:
        problems.append(f"{path.relative_to(ROOT)}: anchor found {text.count(old)} times: {old.strip()[:90]!r}")
        return
    edits[path] = text.replace(old, new)


H = r"    let mut hasher = std::collections::hash_map::DefaultHasher::new\(\);\n"
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
for rel, var, prefix, named in SIMPLE:
    path = P / rel
    if named:
        site(path, H + rf"    {var}\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"{prefix}-\{{content_hash:016x\}}\"\);\n", f'    let child_id = store::content_id("{prefix}", {var}.as_bytes());\n')
    else:
        site(path, H + rf"    {var}\.hash\(&mut hasher\);\n    let child_id = format!\(\"{prefix}-\{{:016x\}}\", hasher\.finish\(\)\);\n", f'    let child_id = store::content_id("{prefix}", {var}.as_bytes());\n')
for rel, prefixes in (("🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs", ("presentation", "animation")),
                      ("📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs", ("imperative-flow", "imperative-text")),
                      ("📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs", ("playbook-flow", "playbook-document"))):
    for prefix in prefixes:
        site(P / rel, H + rf"    content_json\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"{prefix}-\{{content_hash:016x\}}\"\);\n", f'    let child_id = store::content_id("{prefix}", content_json.as_bytes());\n')
for prefix in ("architect-benchmarks", "architect-knowledge"):
    site(P / "🏛️architect/🗿️artifacts/🏛️program/🦀️.rs", H + rf"    content_json\.hash\(&mut hasher\);\n    format!\(\"{prefix}-\{{:016x\}}\", hasher\.finish\(\)\)\n", f'    store::content_id("{prefix}", content_json.as_bytes())\n')
site(P / "📋️forms/🗿️artifacts/📋️forms/🦀️.rs", H + r"    content_json\.hash\(&mut hasher\);\n    format!\(\"forms-scene-\{:016x\}\", hasher\.finish\(\)\)\n", '    store::content_id("forms-scene", content_json.as_bytes())\n')
site(P / "🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs", H + r"    dsl::json::to_json_string\(content\)\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"shooting-emblem-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id("shooting-emblem", dsl::json::to_json_string(content).as_bytes());\n')
process = P / "🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs"
site(process, H + r"    content\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"\{slug\}-brep-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id(&format!("{slug}-brep"), content.as_bytes());\n')
site(process, H + r"    serde_json::to_string\(&semio_framework_os_kernel::ToValue::to_value\(content\)\)\.unwrap_or_default\(\)\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"steps-flow-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id("steps-flow", dsl::json::to_json_string(content).as_bytes());\n')
site(P / "🗒️note/🗿️artifacts/🗒️note/🦀️.rs", H + r"    block_id\.hash\(&mut hasher\);\n    content_json\.hash\(&mut hasher\);\n    let content_hash = hasher\.finish\(\);\n    let child_id = format!\(\"note-text-\{content_hash:016x\}\"\);\n", '    let child_id = store::content_id("note-text", format!("{block_id}\\u{1f}{content_json}").as_bytes());\n')
site(P / "📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs", H + r"    for chunk in chunks \{\n        chunk\.hash\(&mut hasher\);\n    \}\n    format!\(\"remodeling-mesh-io-\{:016x\}-\{:016x\}\", hasher\.finish\(\), chunks\.len\(\)\)\n", '    format!("{}-{:016x}", store::content_id("remodeling-mesh-io", chunks.join("\\u{1f}").as_bytes()), chunks.len())\n')

wires = P / "💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs"
exact(wires, """    let content_json = dsl::os_pack::json::to_json_string(&snapshot);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    wires_content_child_from_hash(content_hash)
}

/// 🔏️ Mints the composed-child identity from a hash produced by the bounded neutral graph encoder.
pub fn wires_content_child_from_hash(content_hash: u64) -> WiresContentChild {
    let child_id = format!("wires-content-{content_hash:016x}");
""", """    let content_json = dsl::os_pack::json::to_json_string(&snapshot);
    let child_id = store::content_id("wires-content", content_json.as_bytes());
""")
cad = P / "📐️cad/🗿️artifacts/📐️cad/🦀️.rs"
exact(cad, """    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let slug = cad_model_child_pane_slug(pane);
    let child_id = format!("{slug}-model-{content_hash:016x}");
""", """    let child_id = store::content_id(&format!("{}-model", cad_model_child_pane_slug(pane)), content_json.as_bytes());
""")
raster = P / "🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs"
exact(raster, """fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RasterAssetChild {
    let child_id = format!("raster-asset-{content_hash:016x}");
""", """fn mint_asset_child_handle(asset_id: &str, content: &[u8]) -> RasterAssetChild {
    let child_id = store::content_id("raster-asset", content);
""")
exact(raster, """    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset.mime.hash(&mut hasher);
    asset.data.hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
""", """    mint_asset_child_handle(asset_id, &[asset.mime.as_bytes(), b"\\x1f", &asset.data].concat())
""")
exact(raster, """    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    <SemioImageSnapshot as store::ArtifactPack>::encode_pack(image).hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
""", """    mint_asset_child_handle(asset_id, &<SemioImageSnapshot as store::ArtifactPack>::encode_pack(image))
""")
remodeling = P / "📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs"
exact(remodeling, """fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RemodelingAssetChild {
    let child_id = format!("remodeling-asset-{content_hash:016x}");
""", """fn mint_asset_child_handle(asset_id: &str, child_id: String) -> RemodelingAssetChild {
""")
exact(remodeling, """    let mut handle = mint_asset_child_handle(asset_id, 0);
    handle.child_id = content_id.into();
    handle
""", """    mint_asset_child_handle(asset_id, content_id.into())
""")
exact(remodeling, """    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset.mime.hash(&mut hasher);
    asset.data.hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
""", """    mint_asset_child_handle(asset_id, store::content_id("remodeling-asset", format!("{}\\u{1f}{}", asset.mime, asset.data).as_bytes()))
""")
din18599 = P / "📕️norm/🗿️artifacts/⚡️din18599/🦀️.rs"
exact(din18599, """    let content_json = pack::json::to_json_string(climate);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("din18599-climate-{:016x}", hasher.finish())
""", """    store::content_id("din18599-climate", pack::json::to_json_string(climate).as_bytes())
""")

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
for path in list(edits):
    if "DefaultHasher" in edits[path] and path.name == "🦀️.rs":
        rest = [line.strip() for line in edits[path].splitlines() if "DefaultHasher" in line]
        print(f"note: {path.relative_to(ROOT)} still uses DefaultHasher (not an id): {rest[:2]}")

store = STORE.read_text(encoding="utf-8")
if "pub fn content_id(" in store:
    problems.append("store: content_id already present")
elif store.count(STORE_ANCHOR) != 1:
    problems.append("store: ArtifactChild anchor not unique")
else:
    edits[STORE] = store.replace(STORE_ANCHOR, STORE_FN + STORE_ANCHOR)

VECTORS = json.loads((HERE / "content-id-vectors.json").read_text(encoding="utf-8"))["vectors"]
LAW = STORE.parent / "🧪️tests/🪪️artifact-addressing/🦀️.rs"
LAW_TS = STORE.parent / "🧪️tests/🪪️artifact-addressing/🟦️.ts"
LAW_FIXTURE = STORE.parent / "🧫️fixtures/🪪️artifact-addressing/🔣️.json"
law = LAW.read_text(encoding="utf-8")
fixture = json.loads(LAW_FIXTURE.read_text(encoding="utf-8"), object_pairs_hook=OrderedDict)
if "content_id_is_the_specified_sha256_prefix" in law or "contentIds" in fixture:
    problems.append("store: content_id law or fixture already present")
else:
    edits[LAW] = law.rstrip("\n") + """

/// 🆔️ `content_id` is `<prefix>-` + the first 16 hex digits of SHA-256: equal to Python `hashlib`'s answer for every
/// committed vector (`contentIds`), admitted by the child schema's `ContentId`, and independent of the process that
/// computes it.
#[test]
fn content_id_is_the_specified_sha256_prefix() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-addressing/🔣️.json")).unwrap();
    for row in fixture["contentIds"].as_array().unwrap() {
        let (prefix, text) = (row["prefix"].as_str().unwrap(), row["text"].as_str().unwrap());
        let id = content_id(prefix, text.as_bytes());
        assert_eq!(id, row["id"].as_str().unwrap());
        assert_eq!(id, content_id(prefix, text.as_bytes()));
    }
}
"""
    fixture["contentIds"] = VECTORS
    edits[LAW_FIXTURE] = json.dumps(fixture, ensure_ascii=False, indent=2) + "\n"
law_ts = LAW_TS.read_text(encoding="utf-8")
if "testContentIdOracle" in law_ts:
    problems.append("store TS: content id oracle already present")
else:
    law_ts = law_ts.replace('import assert from "node:assert/strict";\n', 'import assert from "node:assert/strict";\nimport { createHash } from "node:crypto";\n', 1)
    law_ts = law_ts.replace('import { parseArtifactChild } from "../../🪆️child/🧬️schema/🟦️.ts";\n', 'import { contentId, parseArtifactChild } from "../../🪆️child/🧬️schema/🟦️.ts";\nimport { sha256Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";\n', 1)
    edits[LAW_TS] = law_ts.rstrip("\n") + """

/** 🆔️ `contentId` (TS twin of `store::content_id`) equals Python `hashlib`'s committed answers, every committed id is a
 * schema `ContentId`, and the first-party synchronous SHA-256 equals Node's `crypto` (third-party) on every vector and on
 * inputs that cross the 55/56/64-byte padding boundaries. */
export function testContentIdOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(ioSchema).addSchema(childSchema);
  const validateContentId = ajv.compile({ $ref: `${childSchema.$id}#/$defs/ContentId` });
  const encoder = new TextEncoder();
  for (const row of fixture.contentIds) {
    const bytes = encoder.encode(row.text);
    assert.equal(contentId(row.prefix, bytes), row.id);
    assert.equal(validateContentId(row.id), true, JSON.stringify(validateContentId.errors));
    assert.equal(sha256Hex(bytes), createHash("sha256").update(bytes).digest("hex"));
  }
  for (const length of [0, 1, 55, 56, 63, 64, 65, 119, 120, 1000]) {
    const bytes = Uint8Array.from({ length }, (_, index) => (index * 131 + 7) & 0xff);
    assert.equal(sha256Hex(bytes), createHash("sha256").update(bytes).digest("hex"), `length ${length}`);
  }
  for (const invalid of ["catalog", "catalog-4F53CDA18C2BAA0C", "catalog-4f53cda18c2baa0", "-4f53cda18c2baa0c"]) assert.equal(validateContentId(invalid), false, invalid);
}
"""

CHILD_SCHEMA = STORE.parent / "🪆️child/🧬️schema/🔣️.json"
schema = json.loads(CHILD_SCHEMA.read_text(encoding="utf-8"), object_pairs_hook=OrderedDict)
if "$defs" in schema and "ContentId" in schema["$defs"]:
    problems.append("child schema: ContentId already present")
else:
    schema["properties"]["childId"]["description"] = "🆔️ The child's member identity. A content-addressed child (one whose identity follows its content) carries a `ContentId`."
    schema["$defs"] = OrderedDict([("ContentId", OrderedDict([
        ("description", "🆔️ `<prefix>-` + the first 16 lowercase hex digits of SHA-256 over the child's canonical content bytes (several fields joined with U+001F). Rust `store::content_id`, TypeScript `contentId`."),
        ("type", "string"),
        ("pattern", "^[a-z0-9][a-z0-9.-]*-[0-9a-f]{16}$"),
    ]))])
    edits[CHILD_SCHEMA] = json.dumps(schema, ensure_ascii=False, indent=2) + "\n"

CHILD_TS = STORE.parent / "🪆️child/🧬️schema/🟦️.ts"
child_ts = CHILD_TS.read_text(encoding="utf-8")
if "export function contentId" in child_ts:
    problems.append("child TS: contentId already present")
else:
    first_import = child_ts.index("import ")
    child_ts = child_ts[:first_import] + 'import { hexLower, sha256 } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";\n' + child_ts[first_import:]
    edits[CHILD_TS] = child_ts.rstrip("\n") + """

/** 🆔️ TypeScript twin of Rust `store::content_id`: `<prefix>-` + the first 16 lowercase hex digits of SHA-256 over
 * `bytes` (the child schema's `ContentId`). */
export function contentId(prefix: string, bytes: Uint8Array): string {
  return `${prefix}-${hexLower(sha256(bytes).subarray(0, 8))}`;
}
"""

HASH_TS = ROOT / "🧰️framework/🔨️modules/🔏️hash/🟦️.ts"
hash_ts = HASH_TS.read_text(encoding="utf-8")
if "export function sha256(" in hash_ts:
    problems.append("hash TS: sha256 already present")
else:
    edits[HASH_TS] = hash_ts.rstrip("\n") + """

//#region 🔐️Sha256
const SHA256_ROUNDS = new Uint32Array([
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]);

function sha256Compress(state: Uint32Array, block: Uint8Array, offset: number, words: Uint32Array): void {
  for (let index = 0; index < 16; index += 1) words[index] = ((block[offset + index * 4]! << 24) | (block[offset + index * 4 + 1]! << 16) | (block[offset + index * 4 + 2]! << 8) | block[offset + index * 4 + 3]!) >>> 0;
  for (let index = 16; index < 64; index += 1) {
    const w15 = words[index - 15]!;
    const w2 = words[index - 2]!;
    const s0 = ((w15 >>> 7) | (w15 << 25)) ^ ((w15 >>> 18) | (w15 << 14)) ^ (w15 >>> 3);
    const s1 = ((w2 >>> 17) | (w2 << 15)) ^ ((w2 >>> 19) | (w2 << 13)) ^ (w2 >>> 10);
    words[index] = (words[index - 16]! + s0 + words[index - 7]! + s1) >>> 0;
  }
  let [a, b, c, d, e, f, g, h] = state as unknown as [number, number, number, number, number, number, number, number];
  for (let index = 0; index < 64; index += 1) {
    const sum1 = ((e >>> 6) | (e << 26)) ^ ((e >>> 11) | (e << 21)) ^ ((e >>> 25) | (e << 7));
    const choice = (e & f) ^ (~e & g);
    const temporary1 = (h + sum1 + choice + SHA256_ROUNDS[index]! + words[index]!) >>> 0;
    const sum0 = ((a >>> 2) | (a << 30)) ^ ((a >>> 13) | (a << 19)) ^ ((a >>> 22) | (a << 10));
    const majority = (a & b) ^ (a & c) ^ (b & c);
    const temporary2 = (sum0 + majority) >>> 0;
    h = g;
    g = f;
    f = e;
    e = (d + temporary1) >>> 0;
    d = c;
    c = b;
    b = a;
    a = (temporary1 + temporary2) >>> 0;
  }
  const next = [a, b, c, d, e, f, g, h];
  for (let index = 0; index < 8; index += 1) state[index] = (state[index]! + next[index]!) >>> 0;
}

/** 🔐️ First-party synchronous SHA-256 (FIPS 180-4) — the TypeScript twin of this module's `🦀️.rs` `Sha256::digest`, for
 * callers that cannot await Web Crypto (content ids minted inside a synchronous mutation twin). */
export function sha256(bytes: Uint8Array): Uint8Array {
  const state = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
  const words = new Uint32Array(64);
  const whole = bytes.length - (bytes.length % 64);
  for (let offset = 0; offset < whole; offset += 64) sha256Compress(state, bytes, offset, words);
  const tail = new Uint8Array(bytes.length % 64 < 56 ? 64 : 128);
  tail.set(bytes.subarray(whole));
  tail[bytes.length - whole] = 0x80;
  const bits = BigInt(bytes.length) * 8n;
  for (let index = 0; index < 8; index += 1) tail[tail.length - 1 - index] = Number((bits >> BigInt(index * 8)) & 0xffn);
  for (let offset = 0; offset < tail.length; offset += 64) sha256Compress(state, tail, offset, words);
  const digest = new Uint8Array(32);
  for (let index = 0; index < 8; index += 1) {
    digest[index * 4] = state[index]! >>> 24;
    digest[index * 4 + 1] = (state[index]! >>> 16) & 0xff;
    digest[index * 4 + 2] = (state[index]! >>> 8) & 0xff;
    digest[index * 4 + 3] = state[index]! & 0xff;
  }
  return digest;
}

/** 🔡 Lowercase hexadecimal of `bytes` — the TypeScript twin of this module's `🦀️.rs` `hex_lower`. */
export function hexLower(bytes: Uint8Array): string {
  let output = "";
  for (const byte of bytes) output += byte.toString(16).padStart(2, "0");
  return output;
}

/** #️⃣ Canonical lowercase SHA-256 hex of a complete byte slice — twin of `🦀️.rs` `sha256_hex`. */
export function sha256Hex(bytes: Uint8Array): string {
  return hexLower(sha256(bytes));
}
//#endregion 🔐️Sha256
"""

REMODELING_TS = P / "📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts"
remodeling_ts = REMODELING_TS.read_text(encoding="utf-8") if REMODELING_TS.exists() else ""
start = remodeling_ts.find("const SIP_MASK = (1n << 64n) - 1n;\n")
end = remodeling_ts.find("/** 🕸️ `image_asset_child_handle`")
old_mint = """  const hasher = new DefaultHasher();
  hasher.writeStr(asset.mime);
  hasher.writeStr(asset.data);
  const childId = `remodeling-asset-${hasher.finish().toString(16).padStart(16, "0")}`;
"""
if start < 0 or end < start or remodeling_ts.count(old_mint) != 1:
    problems.append(f"remodeling TS twin: DefaultHasher block {start}..{end} / mint anchor {remodeling_ts.count(old_mint)}")
else:
    between = remodeling_ts[start:end]
    class_at = between.find("class DefaultHasher")
    class_end = between.find("\n}\n\n", class_at)
    if between.count("class DefaultHasher") != 1 or class_end < 0:
        problems.append("remodeling TS twin: DefaultHasher class not isolated")
    else:
        kept = between[class_end + len("\n}\n\n"):]
        remodeling_ts = remodeling_ts[:start] + kept + remodeling_ts[end:]
        for gone in ("SIP_MASK", "rotl(", "DefaultHasher"):
            if gone in remodeling_ts.replace(old_mint, ""):
                problems.append(f"remodeling TS twin: {gone} still referenced after removing the SipHash class")
        remodeling_ts = remodeling_ts.replace(old_mint, """  const childId = contentId("remodeling-asset", new TextEncoder().encode(`${asset.mime}\\u001f${asset.data}`));
""")
        child_ts_path = STORE.parent / "🪆️child/🧬️schema/🟦️.ts"
        relative = os.path.relpath(child_ts_path, REMODELING_TS.parent)
        anchor = 'import { applyRemodelingDiff'
        remodeling_ts = remodeling_ts.replace(anchor, f'import {{ contentId }} from "{relative}";\n' + anchor, 1)
        edits[REMODELING_TS] = remodeling_ts

SCRIPT = ROOT / "📜️script.ts"
script = SCRIPT.read_text(encoding="utf-8")
old_runner = """      const { testSharedArtifactAddressingOracle } = await import("./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️artifact-addressing/🟦️.ts");
      testSharedArtifactAddressingOracle();
"""
if script.count(old_runner) != 1:
    problems.append(f"root script: shared-artifact-addressing runner anchor found {script.count(old_runner)} times")
else:
    edits[SCRIPT] = script.replace(old_runner, """      const { testContentIdOracle, testSharedArtifactAddressingOracle } = await import("./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️artifact-addressing/🟦️.ts");
      testSharedArtifactAddressingOracle();
      testContentIdOracle();
""").replace('"--lib", "shared_artifact_addressing", "--", "--nocapture"], this.root);', '"--lib", "artifact_addressing", "--", "--nocapture"], this.root);', 1)

for path in sorted(edits, key=str):
    print(("write " if write else "dry-run ") + str(path.relative_to(ROOT)))
for problem in problems:
    print("problem:", problem)
print(f"{len(edits)} files, {len(problems)} problems")
if problems:
    sys.exit(1)
if write:
    for path, text in edits.items():
        path.write_text(text, encoding="utf-8")
