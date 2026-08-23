# P8yw Raster Fifth Remediation Independent Final Audit

## Verdict

**RED — not accepted.** The fifth patch repairs the fourth audit's two literal `expect`-after-`ManuallyDrop` saturation defects and removes the ordinary populated `DslField::to_value` loop. It still leaves a public populated `RasterOwnedMap` **serde serialization** loop, which is an uncredited whole-map materialization route. That directly violates the requested no-hidden-`serde` fallback and the preserved fail-closed serde property. The permanent verifier accepts this live counterexample, so its fixture/mutation evidence is not faithful.

No production source or verifier was edited in this audit. Cargo, Nx, Wasm, browser, runtime, network, and broad builds were not run.

## Scope and Evidence Read

Read root `AGENTS.md`; the prior independent final audits, including `📓️terra-independent-p8yw-raster-third-remediation-final-audit-2026-08-23.md` and `📓️terra-independent-p8yw-raster-fourth-remediation-final-audit-2026-08-24.md`; the implementation handback, including its fifth-remediation section at lines 403–469; the current Raster source and working-tree diff; and the permanent tool-job verifier.

## Blocking Finding: Public Serde Output Still Materializes Every Populated Map

`RasterOwnedMap<V>` still implements ordinary public `serde::Serialize`. Its live body calls `serialize_map(Some(self.length))`, loops over `self`, and invokes `map.serialize_entry(key, value)` once for every entry:

```rust
let mut map = serializer.serialize_map(Some(self.length))?;
for (key, value) in self {
    map.serialize_entry(key, value)?;
}
map.end()
```

Exact source: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs:315-324`.

This is not the now-refused `DslField::to_value` route at lines 352–355. A populated 64-entry `RasterOwnedMap` can instead be serialized in one ordinary call, with no retained output cursor, pre-admitted fixed/page/item/byte/control authority, semantic grant per key/value/page/control, cancellation/fault handback, or resumable close. Each serialized value is recursively delegated to `V: Serialize`, so nested `DslValue` contents are also traversed through a non-interactive fallback.

The route is reachable from public Raster schema types rather than being an inert generic implementation:

- `RasterLayerNode::Adjustment.params` has type `RasterOwnedMap<dsl::DslValue>` and `RasterLayerNode` derives `serde::Serialize` (`.../🗿️artifacts/🖨️raster/🦀️component.rs:447-508`).
- `RasterSnapshot.assets` has type `RasterOwnedMap<RasterAssetChild>` and `RasterSnapshot` derives `Serialize` (`.../📸️snapshot/🦀️component.rs:20-41`). Its `skip_serializing_if` only excludes empty maps; populated assets invoke the loop.

Thus the fifth handback's statement that there is no hidden whole `collect`/`clone`/`serde`/DSL fallback is false. The previous `to_value` repair is real but insufficient.

### Required Repair

Make populated `RasterOwnedMap` serialization fail closed, or replace the public serializer with a retained, fixed-page output authority that owns one admitted semantic key/value/page/control unit per grant and has exact cancellation/fault/panic/drop/close handback. The same authority must be the only populated output route; the derived snapshot/layer serde surface must not bypass it.

Add a hostile populated serde-output fixture (max and max+1, nested values, zero grant, fault/cancellation, panic containment, exact owner/page/control/process terminal counters) and a verifier mutation that restores or retains `impl serde::Serialize`, `serialize_map`, `for (key, value) in self`, or `serialize_entry` for populated maps. A string check for a particular `serde_json::to_value` call is not sufficient.

## Requested-Property Disposition

| Requested property | Result | Evidence |
|---|---|---|
| Saturated standalone/Arc owner admission and resume | Source-level improvement observed; not an acceptance proof | `RasterOwnedRetirement::new` and `RasterSnapshotRetirementFactory::retire` use optional `try_claim().ok()` before storing the exact owner (retained codec lines 203–215 and 836–845). Close retries a full claim without advancing the owner (218–229, 621–628; root 761–826). Focused max/+1 fixtures exist at 4611–4700. Rust execution was intentionally not run. |
| Full retirement/page slots and terminal witnesses | Source-level structural evidence only | Retirement stack CAS claim/return is at 232–258 and 639–683; `terminal_is_empty` includes root, pages, pending slots, and control at 727–736. No prohibited runtime gate was run. |
| Mounted 64-fuel, payload/control ledgers, generation/process checks | No new source regression found in this audit | Semantic fuel is one unit and the existing fixture marker remains required by the verifier. This does not cure the independent serializer escape. |
| Owned map Drop/removal/Clone/decode/DSL | Partially preserved, but overall **RED** | Populated Drop, pair removal, Clone, decode, and ordinary DSL output are guarded at owned-map lines 212–260 and 327–363. Serialize at 315–324 remains unguarded and defeats the requested serde/output boundary. |
| Faithful permanent verifier fixtures and mutations | **RED** | `toolJobRasterEnvelopeCallerRetainedExact` positively requires the decode refusal and DSL refusal (`📜️script.ts:1844-1853`) but never rejects the live `serialize_map`/`serialize_entry` loop. Its alleged serde mutation only appends a literal `serde_json::to_value(&source.assets)` string (`4055`, `4086`); it cannot discriminate this existing generic serializer. |

## Scoped Gates Run

| Gate | Result |
|---|---|
| `rustfmt --check --edition 2021` on the owned-map and retained-codec files | PASS |
| Scoped `git diff --check` on those files and `📜️script.ts` | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS — `self-tests=328 clean` |
| Live `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected global RED — 884 remaining commands and unrelated global failures; it emitted no Raster predicate failure, which confirms the serializer gap is not covered by that predicate |
| Cargo / Nx / Wasm / browser / runtime / network / broad builds | Not run by instruction |

## Re-audit Conditions

1. Close or retain-cursorize populated `RasterOwnedMap` serde serialization, including all derived Raster schema output routes.
2. Add executable hostile serde-output and exact-terminal ledger fixtures.
3. Make the permanent verifier reject the actual serializer loop and prove that mutation; then rerun the scoped formatter, diff, self-test, and live source check.

P2a1 was not started.
