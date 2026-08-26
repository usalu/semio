# Puzzle Fill Retained Preview JSON Sixth Adversarial Audit

## Verdict

**RED.** The fifth-audit diagnostic and status-label repairs are now coherent across the schema, law fixture, native encoder admission, and `World3dHost` parser. The retained cursor nevertheless does **not** preflight the declared 4 KiB full-wire contract. A native oversized `stage` (or another otherwise-valid emitted string) passes `FillPreviewJsonAdmission`, consumes fuel, initializes/mutates cursor state, and only later faults in `Census`. This disproves the required source-admission-before-cancellation/deadline/fuel/cursor/owner/publication-mutation invariant.

No production sources were edited by this audit. Cargo, Nx, Bun/Vitest, Wasm, browser, and cache-writing commands were not run.

## Confirmed Contracts — GREEN

| Contract | Exact evidence |
| --- | --- |
| All fourteen diagnostic integers have schema-safe domains | `preview-json.schema.json:49-72` gives every declared diagnostic numeric property `maximum: 9007199254740991`; identity fields are minimum `1` except sequence `0`, and all nine counters/cursors are minimum `0`. The language-neutral `diagnosticNumericFields` has exactly the same fourteen names and minima at `preview-json-law.json:36-51`, with `maximumDiagnosticInteger` at `:7`. |
| Native producer agrees with those bounds | `fill/🦀️component.rs:33-40` defines the common safe wire maximum; `FillPreviewJsonIdentity::read` at `:52-59` applies the five identity minima; `FillPreviewJsonDiagnosticAuthority::read` at `:85-102` preflights all fourteen native values before cursor work. The emitter uses those same preview fields at `:400-465`. |
| Renderer agrees with those bounds | `World3dHost/🟦️component.tsx:1161` defines a safe nonnegative integer; `:1197-1228` applies safe-positive checks to identity fields and safe-nonnegative checks to the remaining nine fields. The direct renderer law exercises exact `Number.MAX_SAFE_INTEGER` and plus-one for every one of the fourteen fields at `index.test.ts:2770-2785`. |
| Portable max/plus-one and retained-state laws exist for both native integer families | `fill/🦀️component.rs:4541-4613` enumerates all seven `u64` and all seven `usize` producers, proves fixture-name equality, handles narrow-`usize` platforms, checks serde-oracle output at maximum, and calls `assert_preflight_rejection_preserves_ready` at plus-one. That helper (`:4421-4449`) proves unchanged fuel, checkpoint, phase, ready pointer/text/identity, color/status owners, and transient/retiring owners. |
| `statusLabel` byte contract is schema-first | Schema `statusLabel` has `x-semio-maxUtf8Bytes:256` at `preview-json.schema.json:55`; law fixture has exact ASCII 256/257 and multibyte 256/258 byte cases at `preview-json-law.json:24-29`; native admission uses `len() <= 256` at `fill/🦀️component.rs:107-116`; parser performs its own scalar UTF-8 census at `World3dHost/🟦️component.tsx:1164-1172,1209-1212`; Rust serde-oracle boundary law is at `fill/🦀️component.rs:4616-4636`; renderer cases are at `index.test.ts:2763-2766`. |
| Root full-wire cap and parser scope are coherent | Schema sets `x-semio-maxEncodedUtf8Bytes:4096` at `preview-json.schema.json:5`, law declares exact 4096/4097 at `preview-json-law.json:31-34`, native uses `FILL_PREVIEW_JSON_MAX_BYTES = 4 * 1024` at `fill/🦀️component.rs:30`, and parser uses the same cap at `World3dHost/🟦️component.tsx:319,1173`. The parser returns the ordinary brush record before applying the fill cap when `fillBuildPreview` is absent (`:1155-1158`), so unrelated brush pages are not subjected to this fill-only admission rule. Exact/plus-one fill pages are covered in Rust at `fill/🦀️component.rs:4665-4684` and in the renderer at `index.test.ts:2786-2792`. |
| Finite tuples remain coherent | Schema requires exact vector/quaternion lengths at `preview-json.schema.json:22-33`; native admission rejects non-finite ghost pose and `last_sample` values at `fill/🦀️component.rs:113-115`; encoder also refuses non-finite values at `:228-236`; parser applies `Number.isFinite` to root/ghost tuples and `lastSample` at `World3dHost/🟦️component.tsx:1162,1179-1180,1195-1196,1227`; hostile renderer and native laws cover non-finite cases at `index.test.ts:2823-2829,2853-2856` and `fill/🦀️component.rs:4687-4699`. |

## Blocking Defect

### RED-1 — The declared 4 KiB full-wire cap is late `Census` rejection, not preflight admission

`FillPreviewJsonCursor::step` invokes `FillPreviewJsonAdmission::read` before cancellation/deadline/fuel at `fill/🦀️component.rs:570-590`, but that admission only verifies source indices, diagnostic integers, color/status byte lengths, and finite tuple/sample values (`:107-116`). It does not calculate or compare the encoded wire length, and it does not inspect `preview.stage`, optional IDs, mesh URLs, candidate-page strings, or rejection text against the aggregate limit.

The 4 KiB condition occurs only after one-fuel advancement in the `Census` branch: a unit is generated and `self.exact_bytes.checked_add(unit.len)` is checked at `fill/🦀️component.rs:613-626`. An otherwise valid native `preview.stage = "x".repeat(...)` that would produce 4097 bytes therefore follows this observable sequence:

1. `FillPreviewJsonAdmission::read` succeeds.
2. Cancellation/deadline are passed, `fuel.checked_sub(1)` consumes the grant (`:577-590`).
3. An idle or stale cursor calls `begin`, changing identity/progress/pass/owner state (`:531-568,595-601`), then later advances its `Census` cursor.
4. Only after incremental census reaches the overflowing unit does it set `Rejected` (`:613-626`).

The only native 4 KiB plus-one law constructs a fresh cursor with no last-valid page and proves merely that no output is reserved/published (`fill/🦀️component.rs:4665-4684`). It does not prove full-wire plus-one preserves an existing ready page, fuel, checkpoint, phase, or owners. The `oracle_json_admits` guard called by max-boundary tests likewise has no serialized-byte check (`:4318-4344`), so it cannot supply that missing preflight proof.

This is materially different from the repaired diagnostic and label plus-one paths, which are rejected by `FillPreviewJsonAdmission` before the cancellation/deadline/fuel checks and are covered by retained-owner assertions.

## Required Repair

Make aggregate encoded UTF-8 bytes an owned, non-allocating `FillPreviewJsonAdmission` preflight calculation over exactly the same fields and JSON escaping rules as the incremental pass. It must reject `4097` before cancellation/deadline/fuel and before any cursor/owner/publication mutation, while preserving an already-ready page and all retained owners. Keep the existing incremental census as a defensive consistency check. Add language-neutral and native laws that drive a retained ready page through 4096 and 4097 aggregate source cases, compare the admitted 4096 page with the `serde_json` oracle, and prove exact preservation on 4097. Add an explicit renderer regression test that a greater-than-4096 non-fill brush page still returns normally, alongside the existing fill exact/plus-one tests.

## Static Checks Performed

| Command | Result |
| --- | --- |
| `jq -S .` on schema and law fixtures | Both fixtures parsed successfully. |
| `jq` census for all fourteen schema maxima/minima | Exactly fourteen named fields; all maximum values are `9007199254740991` and minima are `0` or `1` as declared. |
| `jq` checks for `statusLabel` 256-byte annotation, root 4096 annotation, and law cardinalities | All present; status-label law has four cases, full-wire law has two, numeric list has fourteen. |
| `rustfmt --check fill/🦀️component.rs` | Exit 0. |
| Scoped `git diff --check` across the four audited source/fixture paths | No whitespace error. |

