#!/usr/bin/env python3
"""🔤️ S5-CHANNEL (design §22.19): descriptor packs are canonical by the descriptor layer.

`describe` orders the members of every object of the descriptor value by UTF-8 key bytes before it encodes, through a type
(`CanonicalDescriptorValue`) that is the only input `descriptor_pack` takes, so the emitted bytes no longer depend on a value
encoder's own order policy. Laws: the Rust emitter reproduces the bytes the TypeScript pack encoder sealed in
`🧫️fixtures/🧫️canonical-descriptor-pack` for the authored, the canonical and the reversed member order, and the emitter
source names the wire value encoder exactly once. The fixture, its schema and the TypeScript oracle are already on disk
(sealed by `🧪️s5-channel-seal-canonical-descriptor-pack.ts`).
Explicit files; every anchor must match exactly its count or nothing is written; `--verify` proves every row is on disk.
Usage: python3 🧪️s5-channel-describe-canonical.py [--apply | --verify]
"""
from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
DESCRIBE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"
EMISSION = f"{DESCRIBE}/🛂️descriptor-emission/🦀️.rs"
UNIT = f"{DESCRIBE}/🧪️tests/🔬️unit/🦀️.rs"
SCRIPT = f"{DESCRIBE}/📦️packages/🦀️rust/📜️script.ts"
PROJECT = f"{DESCRIBE}/📦️packages/🦀️rust/📋️project.json"

CANONICAL = '''/// 🔤️ A descriptor value whose every object holds its members in UTF-8 key-byte order, at every depth (design §22.19 of
/// ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the only value [`descriptor_pack`] encodes, so the emitted bytes are the
/// canonical ones every verifier re-derives, whatever member order a value encoder keeps.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDescriptorValue(semio_framework_value::DslValue);

impl CanonicalDescriptorValue {
    /// 🔃️ Orders the members of every object by key bytes; members of equal keys keep their authored order.
    pub fn new(value: semio_framework_value::DslValue) -> Self {
        Self(canonical_members(value))
    }

    /// 👁️ The canonical value itself.
    pub fn value(&self) -> &semio_framework_value::DslValue {
        &self.0
    }
}

/// 🪜️ [`CanonicalDescriptorValue::new`] through every array and object of `value`.
fn canonical_members(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(canonical_members).collect()),
        DslValue::Object(entries) => {
            let mut entries: Vec<(String, DslValue)> = entries.into_iter().map(|(key, entry)| (key, canonical_members(entry))).collect();
            entries.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            DslValue::Object(entries)
        }
        scalar => scalar,
    }
}

/// 📦️ The pack bytes of a descriptor: its canonical value through the wire value encoder.
pub fn descriptor_pack(value: &CanonicalDescriptorValue) -> Vec<u8> {
    store::pack_rt::encode_wire_value(value.value())
}

'''

LAW = '''
//#region 🔖️CanonicalDescriptorPackLaw
/// 🔁️ `value` with the members of every object in reverse order, at every depth.
fn reversed_members(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(reversed_members).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().rev().map(|(key, entry)| (key, reversed_members(entry))).collect()),
        scalar => scalar,
    }
}

/// 🔤️ LAW (design §22.19): a descriptor packs to the bytes the TypeScript pack encoder sealed in the language-neutral fixture
/// (`🧫️fixtures/🧫️canonical-descriptor-pack`), whatever order its members were authored in — the members of every object
/// in UTF-8 key-byte order at every depth, with either order policy of the value encoder underneath.
#[semio_framework_async_macros::async_test]
async fn a_descriptor_pack_is_canonical_whatever_order_its_members_were_authored_in() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("canonical descriptor pack fixture");
    let cases = fixture.get("cases").and_then(|cases| cases.as_array()).expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let id = case.get("id").and_then(|id| id.as_str()).expect("id");
        let authored = semio_framework_pack_json::to_dsl_value(case.get("authored").expect("authored"));
        let canonical = semio_framework_pack_json::to_dsl_value(case.get("canonical").expect("canonical"));
        let expected = case.get("expectedPackHex").and_then(|hex| hex.as_str()).expect("expectedPackHex");
        assert_ne!(authored, canonical, "{id}: the fixture authors its members out of canonical order");
        assert_eq!(CanonicalDescriptorValue::new(authored.clone()).value(), &canonical, "{id}: members in key-byte order at every depth");
        assert_eq!(CanonicalDescriptorValue::new(canonical.clone()).value(), &canonical, "{id}: canonical order is a fixed point");
        for (form, value) in [("authored", authored), ("canonical", canonical.clone()), ("reversed", reversed_members(canonical))] {
            let packed: String = descriptor_pack(&CanonicalDescriptorValue::new(value)).iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(packed, expected, "{id}: the {form} member order packs to the bytes the TypeScript encoder sealed");
        }
    }
}

/// 🚧️ Every descriptor byte leaves through [`descriptor_pack`]: the emitter names the wire value encoder exactly once, on the
/// canonical value.
#[semio_framework_async_macros::async_test]
async fn the_emitter_encodes_descriptors_only_through_the_canonical_pack() {
    let source = include_str!("../../🛂️descriptor-emission/🦀️.rs");
    assert_eq!(source.matches("encode_wire_value(").count(), 1, "a descriptor is encoded outside `descriptor_pack`");
    assert!(source.contains("pub fn descriptor_pack(value: &CanonicalDescriptorValue) -> Vec<u8> {\\n    store::pack_rt::encode_wire_value(value.value())\\n}"), "`descriptor_pack` encodes anything but the canonical value");
}
//#endregion 🔖️CanonicalDescriptorPackLaw
'''

ANCHOR = "/// 🪪️ `descriptor_sha256` self-hashes the descriptor's own encoded pack MINUS this very field\n"
TAIL = '''/// 🪪️ The emitter classifies a declared-but-unowned kind by the plugin crate's own fault text; the law keeps the two in step.
#[semio_framework_async_macros::async_test]
async fn unowned_codec_schema_fault_is_the_plugin_crates_own_text() {
    assert!(PLUGIN_SDK_SOURCE.contains(&format!("plugin_internal_fault(\\"{UNOWNED_ARTIFACT_CODEC_SCHEMA}\\")")), "the plugin crate no longer faults with UNOWNED_ARTIFACT_CODEC_SCHEMA");
}
'''

ROWS: dict[str, list[tuple[str, str, int]]] = {
    EMISSION: [
        (ANCHOR, CANONICAL + ANCHOR, 1),
        ("    let prehash_value = semio_framework_value::ToValue::to_value(&descriptor);\n    let prehash_bytes = store::pack_rt::encode_wire_value(&prehash_value);\n    descriptor.hashes.descriptor_sha256 = semio_framework_hash::sha256_hex(&prehash_bytes);\n\n    let final_value = semio_framework_value::ToValue::to_value(&descriptor);\n    let final_bytes = store::pack_rt::encode_wire_value(&final_value);\n    let final_json = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&final_value));\n",
         "    let prehash_bytes = descriptor_pack(&CanonicalDescriptorValue::new(semio_framework_value::ToValue::to_value(&descriptor)));\n    descriptor.hashes.descriptor_sha256 = semio_framework_hash::sha256_hex(&prehash_bytes);\n\n    let final_value = semio_framework_value::ToValue::to_value(&descriptor);\n    let final_json = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&final_value));\n    let final_bytes = descriptor_pack(&CanonicalDescriptorValue::new(final_value));\n", 1),
    ],
    UNIT: [
        (TAIL, TAIL + LAW, 1),
    ],
    SCRIPT: [
        ('import { DescribeComponentScript, testFreshComponentSourceEpochV1, testFreshComponentStagingV1, testFreshComponentProcessV1 } from "../../🏭️fresh-component/🟦️.ts";\n',
         'import { DescribeComponentScript, testFreshComponentSourceEpochV1, testFreshComponentStagingV1, testFreshComponentProcessV1 } from "../../🏭️fresh-component/🟦️.ts";\nimport { canonicalDescriptorPackOracle } from "../../🧪️tests/🧪️canonical-descriptor-pack/🟦️.ts";\n', 1),
        ("if (import.meta.main) await runScriptMain(",
         "/** 🔤️ Runs the TypeScript oracle of the canonical descriptor pack fixture (design §22.19). */\nclass CanonicalDescriptorPackCheckScript extends BundleScript {\n  run(segments: string[]): void {\n    if (segments.length) throw Error(\"test-canonical-descriptor-pack accepts no arguments\");\n    console.log(`canonical-descriptor-pack-oracle cases=${canonicalDescriptorPackOracle(this.repoRoot)}`);\n  }\n}\n\nif (import.meta.main) await runScriptMain(", 1),
        ('.register("test-fresh-component", FreshComponentCheckScript));', '.register("test-fresh-component", FreshComponentCheckScript).register("test-canonical-descriptor-pack", CanonicalDescriptorPackCheckScript));', 1),
    ],
    PROJECT: [
        ('        "workspaceCommand": [\n          "test-fresh-component"\n        ]\n      }\n    }\n  },\n',
         '        "workspaceCommand": [\n          "test-fresh-component"\n        ]\n      }\n    },\n    "test-canonical-descriptor-pack": {\n      "executor": "nx:run-commands",\n      "cache": false,\n      "dependsOn": [],\n      "options": {\n        "command": "bun ./📜️script.ts test-canonical-descriptor-pack",\n        "cwd": "{projectRoot}"\n      },\n      "metadata": {\n        "workspaceCommand": [\n          "test-canonical-descriptor-pack"\n        ]\n      }\n    }\n  },\n', 1),
    ],
}


def main() -> None:
    apply, verify = "--apply" in sys.argv, "--verify" in sys.argv
    only = [flag.split("=", 1)[1] for flag in sys.argv if flag.startswith("--only=")]
    if not (ROOT / ".git").exists() or not ROWS:
        raise SystemExit("[DEBUG] not the repo root or no rows: refusing")
    selected = {path: rows for path, rows in ROWS.items() if not only or any(path.endswith(suffix) for suffix in only)}
    if not selected:
        raise SystemExit("[DEBUG] --only selects no file: refusing")
    results: dict[str, str] = {}
    failed = False
    for path, rows in selected.items():
        target = ROOT / path
        if not target.is_file():
            raise SystemExit(f"[DEBUG] missing file: {path}")
        text = target.read_text()
        print(f"== {path}")
        for index, (old, new, count) in enumerate(rows, 1):
            found = text.count(new) if verify else text.count(old)
            wrong = found != count
            failed |= wrong
            print(f"   {index} {'MISMATCH' if wrong else 'ok'} x{found}/{count}: {old.strip().splitlines()[0][:100]}")
            text = text.replace(old, new)
        results[path] = text
    if failed:
        raise SystemExit("[DEBUG] anchors do not match: nothing written")
    if apply:
        for path, text in results.items():
            (ROOT / path).write_text(text)
    print(f"[DEBUG] {len(results)} files {'written' if apply else ('verified' if verify else 'would change (dry run)')}")


if __name__ == "__main__":
    main()
