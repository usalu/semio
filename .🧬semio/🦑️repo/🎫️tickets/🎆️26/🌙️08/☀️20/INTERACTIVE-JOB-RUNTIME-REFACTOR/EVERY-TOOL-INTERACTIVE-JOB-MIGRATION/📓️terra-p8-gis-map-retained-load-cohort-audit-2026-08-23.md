# Terra Audit: Phase 8 GIS Map Retained-Load Cohort — 2026-08-23

## Verdict

**ACCEPT — GIS Map retained-load source cohort only.** This accepts neither Phase 8 nor runtime, native, Wasm, browser, timing, global-close, or full-roster behavior.

## Independent Source Findings

- The live GIS catalog is `GisMapEnvelopeOwnedFieldCatalog`; it supplies snapshot, mutation, VCS, conflict, snapshot-retirement, and mutation-retirement owners to the shared `artifact_owned_spr_edit_history_decoder`. The shared decoder reserves its bounded mutation list before publication, validates operation/generation/cancellation on every token, and cursor-retires every rejected/pending mutation and edit before terminal Drop.
- `GisMapOwnedRetirement` owns all six snapshot fields in order: positions, routes, regions, drawing, optional image, and value. It retires one feature/string/value child at a time; `DslValue::{String,Array,Object}` is recursive, and each drawing/image/value child retires its exact `child_id`, `artifact_id`, dialect `artifact_kind`, `standard`, and `subset` strings. The Arc snapshot root either transfers a unique root to the cursor or retains it when shared.
- All twelve GIS mutation shapes are explicit: create/delete/reorder/replace for positions, routes, and regions. The all-variants fixture calls `close_step(0, ...)` and proves `Pending { released_items: 0, released_bytes: 0 }`, so a zero-item grant leaves ownership untouched.
- The initializer validates identity and duplicate edits, incrementally clones the six-field snapshot, seeds causal history, applies/hashes one mutation operation at a time, and creates the candidate only through `from_initialized_runtime_with_owners` after checked `generation + 1`. Stale, cancel, fault, rejected candidate, and displaced owners pass through retained close cursors; nonterminal Drop asserts terminal-empty.
- `Gis2dPlayApp` supplies the catalog, owner bundle, initializer, and document-store disposer. The Wasm ABI is only `beginEnvelopeLoad` → `admitEnvelopePage(Uint8Array)` → seal → one `maintenance_step(1, pageBytes)` plus one operation poll → generation-validated replacement acknowledgement, with cancel and one bounded close step. `Uint8Array` length is rejected before the fixed Rust page copy. A duplicate acknowledgement is explicitly idempotent.
- The old direct whole-string GIS route is absent. The only GIS `ArtifactStore::new` match is the `#[cfg(test)]` round-trip fixture at binary codec line 1243; no GIS production occurrence of `reject_whole_buffer_artifact_envelope_ingress`, `envelope_json: &str`, or the former direct Wasm store route remains.

## Fixtures and Mutation Quality

The live source includes fixtures for successful generation advancement and incremental candidate close; cancellation and stale generation to terminal-empty; nested `DslValue` and drawing/image/value retirement; all twelve variants and the zero-grant invariant; partial ingress cancellation; exact first and duplicate acknowledgement.

The interactivity self-test exercises eleven adversarial GIS mutations: fallback shared decoder, dynamic post-lift page, missing acknowledgement, initializer drop, unchecked generation, drawing deep drop, nested-value deep drop, missing mutation variant, missing zero-grant fixture, and injected whole-buffer ingress. These are semantic negative mutations of the exact route predicate, not a string-count-only check. The inherited owned-schema/ingress assertions cover malformed, truncated, duplicate, unknown, oversized, saturation/+1, false-terminal, cancellation, and close behavior. Rust fixtures were inspected but not executed under the source-only constraint.

## Census and Scope

`rg` finds exactly **15** production Rust occurrences of `reject_whole_buffer_artifact_envelope_ingress`: **one** shared fail-closed definition and **14** live non-GIS callers. GIS Map has **zero** occurrences. Therefore this cohort truthfully changes the structural placeholder census to **1 shared definition + 14 live callers**; it does not accept those remaining callers.

The staged cohort inventory is the GIS mutation codec, GIS editor, GIS Wasm bridge, shared store source, and `📜️script.ts` (2,800 additions / 142 deletions across those five files). No P1, P3, or unrelated production file is in this scoped inventory.

## Commands and Results

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true <three GIS files>` | PASS (exit 0) |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: `self-tests=163 clean` |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS: DENY clean; one declared, structurally invisible test-only blocking-bridge record |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected RED (exit 1): GIS retained-route assertion does not fail; the unchanged global census is 18 failure classes and **0/884** admitted commands |
| `cmp -s p8yt-gis-map-tool-jobs.json p8yt-gis-map-tool-jobs-repeat.json` | PASS (exit 0); both SHA-256 `05dbfee879ca84ef57eb15c932eca39c2f782dcc7261b042ba1f2a7b23a9c04b` |
| Scoped and whole `git diff --check`, `git diff --cached --check`, `git diff HEAD --check` | PASS (all exit 0) |

## Residuals Deliberately Not Accepted

No Cargo compilation, Rust fixture execution, Wasm/browser execution, network, Nx, or root lint was run. Consequently native/Wasm ABI execution, hostile-input timing, runtime scheduling, global close, and exact runtime ownership remain unverified. The full tool-job verifier remains **RED: 18 failure classes and 0/884**, including the 14 remaining live whole-buffer callers and other Phase 8-wide structural/roster failures. Phase 8 remains RED.
