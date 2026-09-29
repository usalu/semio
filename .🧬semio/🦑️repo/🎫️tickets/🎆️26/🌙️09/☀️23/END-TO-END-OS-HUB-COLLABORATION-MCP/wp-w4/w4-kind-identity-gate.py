#!/usr/bin/env python3
"""🛂️ W4 (14c → window 3, landed by L1 in T3, AFTER w4-kind-spec-identity.py): the describe-time kind-identity gate.

`semio-framework-plugin-describe describe` refuses to write a descriptor when
  (1) static law — an app's `io` presents a document artifact (`ArtifactPresentation.id` non-empty) that no ArtifactKindSpec
      (app- or plugin-level) declares, or declares with a schema other than `io.artifact_schema`;
  (2) codec law — the package declares ≥ 1 artifact kind and the compiled component owns a codec for none of them (the
      trusted-catalog publish would refuse it: "trusted <plugin> closure carries no artifact codec").
The codec probe reuses the component the describe step already compiled (no second compile) and stops at the first owned
kind (a full census of stdio's bundle costs minutes); a refusal lists every kind probed. Language-neutral fixture `🧫️fixtures/🪪️kind-identity/🔣️.json`
drives the static-law test.

usage: python3 w4-kind-identity-gate.py --dry-run | --write | --revert [--root <tree>]
"""
import json, os, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
ROOT = f"{TREE}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"
EMITTER = "🛂️descriptor-emission/🦀️.rs"
TESTS = "🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = "🧫️fixtures/🪪️kind-identity/🔣️.json"

FIXTURE_DOC = {
    "schema": "semio.plugin.describe.kind-identity-fixture/v1",
    "cases": [
        {"name": "presented kind declared with the io schema", "apps": [{"appId": "s.note.note@1/*#editor", "presentedKind": "s.note.note", "presentedSchema": "note.document", "declared": [["s.note.note", "note.document"]]}], "faults": 0},
        {"name": "an app presenting no artifact is not judged", "apps": [{"appId": "s.note.note@1/*#viewer", "presentedKind": "", "presentedSchema": "note.document", "declared": []}], "faults": 0},
        {"name": "media schema distinct from the store schema", "apps": [{"appId": "s.imperative.procedure@1/*#editor", "presentedKind": "computation.procedure", "presentedSchema": "procedure.document/v1", "declared": [["computation.procedure", "procedure.document"]]}], "faults": 1},
        {"name": "presented kind declared by no spec", "apps": [{"appId": "s.fem.fem2d@1/*#editor", "presentedKind": "2d.fem", "presentedSchema": "fem.2d", "declared": [["computation.fem2d", "computation.fem2d"]]}], "faults": 1},
        {"name": "a plugin-level spec declares the presented kind", "apps": [{"appId": "s.gis.gismap@1/*#editor", "presentedKind": "s.gis.gismap", "presentedSchema": "gis.map", "declared": [["2d.image", "raster.document"], ["s.gis.gismap", "gis.map"]]}], "faults": 0},
    ],
}

LAW = '''/// 🪪️ One app's kind-identity projection: its surface id, the artifact its `io` presents (`ArtifactPresentation.id` and
/// `io.artifact_schema`), and the `(id, schema)` of every ArtifactKindSpec it or its plugin declares.
pub struct KindIdentityApp<'a> {
    pub app_id: &'a str,
    pub presented_kind: &'a str,
    pub presented_schema: &'a str,
    pub declared: Vec<(&'a str, &'a str)>,
}

/// 🪪️ The static kind-identity law: an app whose `io` presents a document artifact declares that kind with the ONE schema
/// its `io.artifact_schema` names — the identity the hub's codec rows, document-open targets and genesis, the MCP
/// workspace store and host-media contributions all key on (ticket 26/09/23 W4: nine packages carried a distinct
/// "media schema", owned no codec, and the trusted catalog refused them). An app presenting no artifact is not judged.
pub fn kind_identity_faults(apps: &[KindIdentityApp<'_>]) -> Vec<String> {
    apps.iter()
        .filter(|app| !app.presented_kind.is_empty())
        .filter_map(|app| match app.declared.iter().find(|(kind, _)| *kind == app.presented_kind) {
            None => Some(format!("{}: io presents artifact kind {:?} that no ArtifactKindSpec declares", app.app_id, app.presented_kind)),
            Some((_, schema)) if *schema != app.presented_schema => Some(format!("{}: artifact kind {:?} declares schema {schema:?}, its io names {:?}", app.app_id, app.presented_kind, app.presented_schema)),
            Some(_) => None,
        })
        .collect()
}

/// 🪪️ [`KindIdentityApp`] rows of one described package — each app's own specs followed by the plugin-level ones.
pub fn descriptor_kind_identity_apps(descriptor: &PackageDescriptor) -> Vec<KindIdentityApp<'_>> {
    let manifest = &descriptor.manifest;
    manifest
        .apps
        .iter()
        .map(|app| KindIdentityApp {
            app_id: &app.id,
            presented_kind: &app.io.artifact.id,
            presented_schema: &app.io.artifact_schema,
            declared: app.artifact_kinds.iter().chain(manifest.artifact_kinds.iter()).map(|kind| (kind.id.as_str(), kind.schema.as_str())).collect(),
        })
        .collect()
}

/// 🗂️ Every `(kind, schema)` one described package declares — plugin-level specs first, then each app's, deduplicated
/// by kind id in declaration order (the same union the trusted-catalog publish asks the component about).
pub fn declared_artifact_kind_pairs(descriptor: &PackageDescriptor) -> Vec<(String, String)> {
    let manifest = &descriptor.manifest;
    let mut pairs: Vec<(String, String)> = Vec::new();
    for kind in manifest.artifact_kinds.iter().chain(manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())) {
        if !pairs.iter().any(|(id, _)| *id == kind.id) {
            pairs.push((kind.id.clone(), kind.schema.clone()));
        }
    }
    pairs
}

/// 🧬️ Whether the compiled component owns a codec for at least one of `pairs` — the `codec.pack-schema-hash` probe
/// [`component_codec_rows`] asks, on the instance `describe` already compiled, stopping at the first owned kind (one
/// throwaway instance for almost every package; a full census of a large bundle would cost minutes per describe).
/// Unowned kinds are expected (inputs, companions); the faults of every kind probed before the first owned one are
/// returned so a refusal names them.
async fn first_owned_codec(runtime: &OwnedRuntime, compiled: &CompiledHandle, pairs: &[(String, String)]) -> Result<(), Vec<String>> {
    let budget = semio_framework::kernel::Budget { fuel: DESCRIBE_FUEL_BUDGET, deadline_ms: DESCRIBE_DEADLINE_MS, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };
    let mut faults = Vec::new();
    for (kind, schema) in pairs {
        match runtime.codec_pack_schema_hash(compiled, schema, budget).await {
            Ok(hash) if hash != [0; 32] => return Ok(()),
            Ok(_) => faults.push(format!("{kind}={schema}: no structural record specification")),
            Err(TurnFault::Guest(fault)) if fault.message == UNOWNED_ARTIFACT_CODEC_SCHEMA => faults.push(format!("{kind}={schema}: unowned")),
            Err(error) => faults.push(format!("{kind}={schema}: {error}")),
        }
    }
    Err(faults)
}

'''

GATE = '''    let identity_faults = kind_identity_faults(&descriptor_kind_identity_apps(&descriptor));
    if !identity_faults.is_empty() {
        return Err(DescribeError(format!("refusing to write the descriptor of {}: {}", wasm_path.display(), identity_faults.join("; "))));
    }
    let pairs = declared_artifact_kind_pairs(&descriptor);
    if !pairs.is_empty() {
        if let Err(faults) = first_owned_codec(&runtime, &compiled, &pairs).await {
            return Err(DescribeError(format!("refusing to write the descriptor of {}: none of its {} declared artifact kinds is owned by an app of the bundle, so no hub can create or open its documents ({})", wasm_path.display(), pairs.len(), faults.join("; "))));
        }
    }
    write_descriptor_pair_atomic(out_dir, &final_bytes, format!("{final_json}\\n").as_bytes())?;
'''

TEST = '''
/// 🪪️ The static kind-identity law over the language-neutral fixture (`🧫️fixtures/🪪️kind-identity`).
#[semio_framework_async_macros::async_test]
async fn kind_identity_law_matches_the_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️kind-identity/🔣️.json")).expect("kind-identity fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let apps: Vec<KindIdentityApp<'_>> = case["apps"]
            .as_array()
            .expect("apps")
            .iter()
            .map(|app| KindIdentityApp {
                app_id: app["appId"].as_str().expect("appId"),
                presented_kind: app["presentedKind"].as_str().expect("presentedKind"),
                presented_schema: app["presentedSchema"].as_str().expect("presentedSchema"),
                declared: app["declared"].as_array().expect("declared").iter().map(|pair| (pair[0].as_str().expect("kind"), pair[1].as_str().expect("schema"))).collect(),
            })
            .collect();
        assert_eq!(kind_identity_faults(&apps).len() as u64, case["faults"].as_u64().expect("faults"), "{}", case["name"]);
    }
}
'''

EDITS = [
    (EMITTER, "use semio_framework_plugin_host::{GuestRuntime, OwnedRuntime, PackageHash, PackageId, PackageRef, TurnFault};",
     "use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, OwnedRuntime, PackageHash, PackageId, PackageRef, TurnFault};"),
    (EMITTER, "async fn execute_describe_owned(wasm_bytes: &[u8], source: &Path) -> Result<Vec<u8>, DescribeError> {",
     "async fn execute_describe_owned(wasm_bytes: &[u8], source: &Path) -> Result<(Vec<u8>, OwnedRuntime, CompiledHandle), DescribeError> {"),
    (EMITTER, '''    runtime
        .describe_observed(
            &compiled,
            semio_framework::kernel::Budget { fuel: DESCRIBE_FUEL_BUDGET, deadline_ms: DESCRIBE_DEADLINE_MS, max_effects: 0, max_patch_bytes: 0, max_frames: 0 },
            |fuel, elapsed| eprintln!("[describe] owned phase=execute fuel={fuel} elapsed_ms={}", elapsed.as_millis()),
        )
        .await
        .map_err(|error| DescribeError(format!("calling owned describe() on {}: {error}", source.display())))
}
''', '''    let descriptor = runtime
        .describe_observed(
            &compiled,
            semio_framework::kernel::Budget { fuel: DESCRIBE_FUEL_BUDGET, deadline_ms: DESCRIBE_DEADLINE_MS, max_effects: 0, max_patch_bytes: 0, max_frames: 0 },
            |fuel, elapsed| eprintln!("[describe] owned phase=execute fuel={fuel} elapsed_ms={}", elapsed.as_millis()),
        )
        .await
        .map_err(|error| DescribeError(format!("calling owned describe() on {}: {error}", source.display())))?;
    Ok((descriptor, runtime, compiled))
}
'''),
    (EMITTER, "/// 🛂️ Instantiates `wasm_path` once (fuel-capped, `pure`-only imports), calls its `describe()`\n/// export, patches independent raw-component and extracted-core `hashes` in, and writes\n/// both output files under `out_dir`. Returns the patched descriptor for the caller to print/verify.\n",
     LAW + "/// 🛂️ Instantiates `wasm_path` once (fuel-capped, `pure`-only imports), calls its `describe()`\n/// export, patches independent raw-component and extracted-core `hashes` in, and writes\n/// both output files under `out_dir` — only after the kind-identity law and the codec census pass on the same\n/// compiled component ([`kind_identity_faults`], [`first_owned_codec`]). Returns the patched descriptor for the caller to print/verify.\n"),
    (EMITTER, "    let descriptor_bytes = execute_describe_owned(&wasm_bytes, wasm_path).await?;\n",
     "    let (descriptor_bytes, runtime, compiled) = execute_describe_owned(&wasm_bytes, wasm_path).await?;\n"),
    (EMITTER, "    write_descriptor_pair_atomic(out_dir, &final_bytes, format!(\"{final_json}\\n\").as_bytes())?;\n", GATE),
    (TESTS, "\n#[semio_framework_async_macros::async_test]\nasync fn run_with_no_args_returns_usage_exit_code() {\n",
     TEST + "\n#[semio_framework_async_macros::async_test]\nasync fn run_with_no_args_returns_usage_exit_code() {\n"),
]


def state(text, old, new):
    if old in new and new in text:
        return "applied" if text.count(new) == 1 else "AMBIGUOUS"
    if new in text and old not in text:
        return "applied"
    count = text.count(old)
    return "pending" if count == 1 else ("MISSING" if count == 0 else "AMBIGUOUS")


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--dry-run"
    if mode not in ("--dry-run", "--write", "--revert"):
        sys.exit(__doc__)
    texts, plan, bad = {}, [], 0
    for rel, old, new in EDITS:
        texts.setdefault(rel, open(os.path.join(ROOT, rel), encoding="utf-8").read())
    for rel, old, new in EDITS:
        s = state(texts[rel], old, new)
        plan.append((rel, old, new, s))
        bad += s in ("MISSING", "AMBIGUOUS")
        print(f"{s:9} {rel} :: {old.strip().splitlines()[0][:90]}")
    fixture_path = os.path.join(ROOT, FIXTURE)
    fixture_text = json.dumps(FIXTURE_DOC, indent=2, ensure_ascii=False) + "\n"
    fixture_state = "applied" if os.path.exists(fixture_path) and open(fixture_path, encoding="utf-8").read() == fixture_text else ("CONFLICT" if os.path.exists(fixture_path) else "pending")
    bad += fixture_state == "CONFLICT"
    print(f"{fixture_state:9} {FIXTURE} (new fixture)")
    if mode == "--dry-run" or bad:
        print(f"{len(EDITS) + 1} edits, {sum(p[3] == 'pending' for p in plan) + (fixture_state == 'pending')} pending, {bad} bad")
        sys.exit(1 if bad else 0)
    for rel, old, new, s in plan:
        if mode == "--write" and s == "pending":
            texts[rel] = texts[rel].replace(old, new, 1)
        elif mode == "--revert" and s == "applied":
            texts[rel] = texts[rel].replace(new, old, 1)
    for rel, text in texts.items():
        path = os.path.join(ROOT, rel)
        if open(path, encoding="utf-8").read() != text:
            open(path, "w", encoding="utf-8").write(text)
            print(f"wrote {rel}")
    if mode == "--write" and fixture_state == "pending":
        os.makedirs(os.path.dirname(fixture_path), exist_ok=True)
        open(fixture_path, "w", encoding="utf-8").write(fixture_text)
        print(f"wrote {FIXTURE}")
    if mode == "--revert" and fixture_state == "applied":
        os.remove(fixture_path)
        os.rmdir(os.path.dirname(fixture_path))
        print(f"removed {FIXTURE}")


if __name__ == "__main__":
    main()
