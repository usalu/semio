# P8yw Raster Seventh Remediation Independent Final Audit

## Verdict

**RED — one fixture-evidence blocker.** The seventh remediation closes the sixth audit's actual public whole-output route. It does not yet prove the required exact-owner identity for both `+1` values in its new hostile output fixture. No production source was edited in this audit. P2a1 was not started.

## Accepted Output Closure

`RasterSnapshot::require_empty_output_shell` is O(1): it checks only `layers.is_empty()` and `assets.is_empty()` at `🧬️schema/📸️snapshot/🦀️component.rs:47-55`. A nonempty layer list or asset map faults before any legacy print/pack allocation.

The public `ArtifactDsl::print_dsl` guard is before body/envelope construction (`:537-541`); `ArtifactPack::encode_pack_with` returns `PackError::Schema` before binary encoding (`:545-551`). The private text and binary entry points redundantly guard before `format!`/`vec!` allocation (`:262-265`, `:496-505`). The public text/binary wrapper modules delegate only to those trait implementations.

All four former map writers are empty-only: `enc_asset_map` (`:122-125`), `enc_params` (`:163-166`), `write_asset_map` (`:356-359`), and `write_params` (`:403-406`). Both layer-list writers are likewise empty-only (`:252-255`, `:487-490`). The scoped Raster census found no `collect::<Vec<_>>().join`, `serde_json::to_vec(v)`, or direct `for (k, v) in map/params` output loop.

The exact mounted caller census has eight and only eight GIF/TIFF/SVG/BMP/PDF/JPG/PNG/DWG serializers. Each has the same `require_empty_output_shell().map_err(str::to_owned)?` immediately before its sole `ArtifactDsl::print_dsl(...).into_bytes()` call. No alternate caller of the snapshot's private text/binary helpers exists. The separate JSON serializer is outside the requested exact-eight legacy codec scope; its populated owned-map paths remain fail-closed by the accepted sixth serde guards.

Empty snapshots retain their existing schema/id/title rendering plus constant empty `layers`/`assets` shells. This was inspected structurally only; Rust runtime tests were intentionally not run.

## Sixth Repair and Earlier Invariants

The sixth serde repair remains intact: no `RasterOwnedMap: Serialize`, no length-based `serialize_map`, and no entry serializer loop. The exact three derived empty-only guards remain on `RasterLayerNode::Adjustment.params`, `RasterSnapshot.assets`, and `RasterArtifact.assets`.

The permanent predicate still requires the retained page map, explicit populated-map Drop refusal, exact replacement handback, standalone/Arc saturation retry and return witnesses, separate payload/control credits, mounted-64 admission, bounded combined depth, WASM preflight-before-copy, generation/ACK, cancellation, and retained close fixtures. Its 328 self-tests include the earlier regression mutations and passed.

## Blocking Fixture Defect

The added `raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact` fixture does establish both maps at capacity, an exact maximum-depth parameter owner, both `+1` rejections, zero-grant close, shared-preflight fault, panic containment, retained close, terminal-empty, and all three process counters at zero (`🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:4799-4885`).

Its exact-owner claim is incomplete:

- For the parameter `+1`, it captures and compares only `plus_one_param_key.as_ptr()` (`:4815-4819`). The rejected `DslValue::String` allocation is constructed inline, never captured, and never compared before retirement (`:4817-4820`).
- For the asset `+1`, it captures and compares only `plus_one_asset_key.as_ptr()` (`:4845-4857`). The rejected `ArtifactChild`'s `child_id` (or another stable owned allocation) is constructed inline, never captured, and never compared before retirement (`:4849-4858`).

Moving `rejected_*.value` into retained retirement proves it is consumed, but it does not independently prove that `insert` returned the exact incoming value/child allocation. The pre-existing general asset-map fixture does make that asset-child pointer assertion (`:4486-4500`), but it is not the new max-depth output fixture and provides no corresponding `DslValue` value-identity assertion. The seventh report's assertion that this fixture proves both complete `key/value` and `key/child` allocation identity is therefore unsupported.

Required repair: retain named `+1` parameter-value and asset-child allocations before insertion, assert their stable inner pointers against `rejected_param.value` and `rejected_asset.value`, then preserve those owners through the existing zero-grant retirement. Add permanent predicate requirements/mutations for both new assertions.

## Verifier Mutation Audit

The predicate reads the snapshot source plus exactly the eight named mounted serializer sources. Its self-test faithfully appends each prior implementation of `enc_asset_map`, `enc_params`, `write_asset_map`, and `write_params`; each is rejected through the actual join/per-value-JSON/direct-loop bans. It also kills removal of one exporter preflight, the public DSL preflight, the private text preflight, the public pack preflight, and the hostile-fixture name. `bun ./📜️script.ts verify interactivity tool-jobs --self-test` passed with `self-tests=328 clean`.

The verifier has no requirement or mutation for either missing `+1` value/child identity assertion, so the current 328 self-tests cannot detect this fixture regression.

## Scoped Gates

| Gate | Result |
|---|---|
| `rustfmt --check --edition 2021` on snapshot codec, hostile fixture, and eight exporters | PASS |
| Scoped `git diff --check` on those files and `📜️script.ts` | PASS |
| Tool-job verifier self-test | PASS — 328 clean |
| Live tool-job predicate | Expected global RED — 884 live registrations and unrelated global categories; no Raster predicate failure |
| Cargo / Nx / Wasm / browser / runtime / network / broad build | Not run by instruction |

