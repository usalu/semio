# P8yw Raster Sixth Remediation Independent Final Audit

## Verdict

**RED — not accepted.** The sixth remediation correctly removes the public `RasterOwnedMap<V>: serde::Serialize` implementation and its `serialize_map(Some(self.length))` / `serialize_entry` loop. The three derived serde fields are real empty-only, fail-closed seams, and the new hostile serde fixture plus its restored-former-loop mutation are faithful.

However, the requested absence of a public/mounted populated-map whole-map serialization route is still not true. Public mounted Raster exporters call `RasterSnapshot::print_dsl`; that hand-crafted codec synchronously walks every populated asset/parameter map, allocates a complete `Vec`/`String` text representation, and in the parameter case serializes each nested `DslValue`. The public `ArtifactPack` implementation has equivalent whole-map `for` loops. No retained admitted output encoder exists anywhere in the Raster production source census. Thus populated data does **not** use the claimed retained admitted encoder.

This is a separate real source blocker, not a FEM verifier-wiring problem. No production source was changed in this audit. P2a1 was not started.

## Evidence Read

Read root `AGENTS.md`; the fifth independent rejection `📓️terra-independent-p8yw-raster-fifth-remediation-final-audit-2026-08-24.md`; the latest handoff's sixth-remediation section in `📓️p8yw-raster-retained-envelope-ingress-2026-08-23.md`; the owned-map, retained codec, Raster artifact/snapshot schemas, editor/Wasm surfaces, public exporter callsites, current diff, and permanent verifier predicate/self-tests.

## Sixth Serde Repair: Accepted as Far as It Goes

`RasterOwnedMap` has no `Serialize` implementation or `V: serde::Serialize` bound. The remaining helper is:

```rust
pub(crate) fn serialize_empty_owned_map<S: serde::Serializer, V>(
    map: &RasterOwnedMap<V>, serializer: S,
) -> Result<S::Ok, S::Error>
```

It checks `map.is_empty()` before any iterator/page/key/value access, returns an explicit error for a populated map, and only writes `serialize_map(Some(0))` for the empty shell. Exact source: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs:315-322`.

The precise three derived serde paths are guarded:

- `RasterLayerNode::Adjustment.params` at `.../🗿️artifacts/🖨️raster/🦀️component.rs:503-505`;
- `RasterArtifact.assets` at `.../🧬️schema/🦀️component.rs:23-24`; and
- `RasterSnapshot.assets` at `.../🧬️schema/📸️snapshot/🦀️component.rs:40-42`.

The static census found no production `impl Serialize for RasterOwnedMap`, no `serialize_map(Some(self.length))`, and no `serialize_entry` loop. The generic derived call therefore has no public populated-map serde bound to resurrect.

The focused test `raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact` in the retained codec (`.../🧬️mutations/💾️binary/🦀️component.rs:4752-4799`) constructs 64 nested entries, proves the +1 key allocation is returned unchanged, calls the public derived adjustment serde path within `catch_unwind`, requires `Ok(Err(_))`, verifies the retained first key remains exact, exercises a zero-item/zero-byte close, and retires to terminal-empty with standalone/page/initialization process counters all zero. Execution was intentionally not run.

The permanent predicate reads the owned-map, retained codec, artifact schema, and snapshot schema together. Its self-test restores the actual former `impl<V: serde::Serialize> serde::Serialize for RasterOwnedMap<V>` with the length-based map loop and rejects it; the 328 self-tests prove that mutation is killed. It also rejects a derived guard removal and hostile-fixture removal.

## Blocking Finding: Mounted Whole-Map Output Still Exists

`RasterSnapshot` publicly implements `store::ArtifactDsl` and `store::ArtifactPack`:

- `print_dsl` calls `print_raster_snapshot_body` at `.../🧬️schema/📸️snapshot/🦀️component.rs:519-535`.
- `encode_pack_with` calls `encode_raster_snapshot_binary` at `.../🧬️schema/📸️snapshot/🦀️component.rs:538-543`.

Both use uncredited whole-map materialization:

```rust
pub(crate) async fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String {
    format!("[{}]", map.iter().map(/* each entry */).collect::<Vec<_>>().join(","))
}
pub(crate) async fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String {
    format!("[{}]", params.iter().map(/* each entry */).collect::<Vec<_>>().join(","))
}
```

Exact source: `.../🧬️schema/📸️snapshot/🦀️component.rs:109-110` and `151-152`. `enc_params` additionally calls `serde_json::to_vec(v)` per nested value. The binary route's `write_asset_map` and `write_params` loop directly over every map entry at lines `341-346` and `391-396`.

These are mounted public routes, not inert helpers: GIF, TIFF, SVG, BMP, PDF, JPG, PNG, and DWG exporters each invoke `<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot)` (their `.../🚪️io/📤️export/.../🦀️component.rs:5`). A populated map can therefore be materialized to a complete output in one ordinary exporter call, with no fixed output pages, pre-admitted item/byte/control credit, semantic grant per key/value/page/control, cancellation/fault handback, panic containment, or resumable close. The text `"require the retained page output authority"` names an authority that the source census cannot find.

The JSON serializer and several editor/viewer/io callers now fail closed through serde for populated maps, which is good but does not close the public ArtifactDsl/ArtifactPack/export routes.

The permanent verifier does not detect this: it only rejects the former public serde bound, `serialize_map(Some(self.length))`, and `.serialize_entry(...)`. Its source predicate accepts the actual snapshot-codec loops, and its restored-loop mutation does not restore or mutate `enc_asset_map`, `enc_params`, `write_asset_map`, `write_params`, or an exporter callsite. This explains why both self-test and live static predicate are clean despite the counterexample.

### Required Repair

Replace the public populated `ArtifactDsl`/`ArtifactPack` and mounted exporter routes with the retained, fixed-page, pre-admitted output authority, or fail-close them for populated owned maps until that authority exists. The authority must own one admitted semantic key/value/page/control unit per grant and preserve exact owner/credit across zero grant, cancellation, fault, panic, and close. Add fixtures at max/+1 and nested value depth, then make the permanent verifier mutate the *actual* `enc_*`/`write_*` loops and mounted exporter reachability; the mutation must be rejected.

## Earlier Raster Invariant Recheck

| Invariant | Result | Source evidence |
|---|---|---|
| Saturated standalone and Arc retirement preserves owner / resumes | Preserved structurally | Optional `try_claim().ok()`, retryable claim, and `control_returned` witness remain required by the permanent predicate; max/+1 saturation fixture names are present. |
| Fixed-page populated map, exact remove/replacement, ordinary Drop refusal | Preserved structurally | `ManuallyDrop` pages, exact `remove_entry`, empty-only Clone, and populated Drop assertion remain present and predicate-checked. |
| Separate payload/control accounting, mounted 64 fuel, combined depth | Preserved structurally | Predicate still requires independent control counters, size proofs, mounted-64 fixture, semantic unit reservation, 403-frame margin, and deepest combined retirement fixture. |
| Zero grant/cancel/fault/panic/terminal credit closure | Preserved structurally | The new serde fixture and retained pre-existing DSL/saturation/candidate fixtures retain zero-grant and terminal counter assertions. Rust fixtures were not run by instruction. |
| Ingress preflight, fixed pages, generation/ACK, cancellation/close | Preserved structurally | Predicate still requires WASM preflight before construction/copy, fixed page, operation-generation, ACK, cancellation, and close strings. |
| No direct derive reintroduces a map serde bound | Preserved | Exactly three guarded fields and no generic map serializer were found. |

## Scoped Gates

| Gate | Result |
|---|---|
| `rustfmt --check --edition 2021` on owned-map, retained codec, artifact schema, snapshot schema | PASS |
| Scoped `git diff --check` on those files and `📜️script.ts` | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS — `self-tests=328 clean` |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected global RED — 884 remaining commands and unrelated global failure classes; the former FEM `frameworkPlugin`-undefined wiring failure did **not** recur, and no Raster predicate failure was emitted. This does not invalidate the independent Raster counterexample above. |
| Cargo / Nx / Wasm / browser / runtime / network / broad build | Not run by instruction |

