# Puzzle Fill Retained Preview JSON Fifth Adversarial Audit

## Verdict

**RED.** The fourth-audit root/ghost and color blockers are remediated: both source-index schema locations cap at `9007199254740991`; the retained cursor preflights that bound before fuel or owner mutation; the shared law covers safe maximum/plus-one and ASCII/multibyte color boundaries; and the parser uses matching safe-integer and UTF-8-byte admission.

The complete declared wire contract is nevertheless still incoherent for the diagnostic integer and status-label fields. The schema admits values that the renderer rejects and that the native cursor can emit. The packet therefore cannot yet be accepted as schema-first across producer, schema, and consumer.

No production source was edited. Cargo, Nx, Bun/Vitest, Wasm, browser, and runtime commands were not run.

## Confirmed Remediations — GREEN

| Contract | Evidence |
| --- | --- |
| Root and `candidateGhost` safe-index cap | `preview-json.schema.json:9,38` both use integer minimum `0` and maximum `9007199254740991`. The law declares max/plus-one at `preview-json-law.json:11-14`. |
| Native preflight before observable work | `FillPreviewJsonSourceAuthority::field/read` enforces the named maximum at `fill/🦀️component.rs:55-71`; `FillPreviewJsonCursor::step` calls it before cancellation, deadline, fuel decrement, identity reset, allocation, or publication at `:524-550`. Root and ghost emission separately read the authority at `:277-281,331-335`. |
| Exact retained-owner preservation on index plus-one | The Rust law builds a ready page, mutates the source index to maximum-plus-one, then proves unchanged fuel, checkpoint, phase, ready pointer/text/identity, color/status owners, and no output/retirement owner at `fill/🦀️component.rs:4391-4417`. This is portable for narrow `usize` targets. |
| 128 UTF-8-byte color semantics | Schema retains `maxLength:128` and adds owned `x-semio-maxUtf8Bytes:128` at `preview-json.schema.json:13`; law cases cover ASCII 128/129 and `ü` 64/65 (128/130 bytes) at `preview-json-law.json:15-20`. Rust consumes them with the test-only serde oracle at `fill/🦀️component.rs:4422-4444`; parser byte-census is at `World3dHost/🟦️component.tsx:1163-1170,1179`; renderer hostile cases include unsafe index and 129-byte ASCII/130-byte multibyte color at `index.test.ts:2787-2807,2819-2833`. |
| Fixture/serde encoding parity | Rust asserts schema maxima/annotation and exact EN/DE fixture bytes against the owned `serde_json` oracle at `fill/🦀️component.rs:4345-4366`; the maximum-index law also compares the retained result to that oracle at `:4378-4386`. |

## Remaining Defects

### RED-1 — Diagnostic numbers lack the renderer's exact-integer wire cap

The diagnostic schema has no `maximum` for all fourteen numeric fields: `operation`, `baseRevision`, `registryGeneration`, `sequence`, `generation`, `collisionCount`, `sampleCursor`, `insideBoth`, `targetCursor`, `candidateCursor`, `acceptedCount`, `totalCount`, `searchCount`, and `rejectedCount` (`preview-json.schema.json:49-72`). It therefore admits `9007199254740992` for each appropriate nonnegative field.

`parseWorldBrushPreview` rejects those values because it applies `Number.isSafeInteger` to all identity fields (`World3dHost/🟦️component.tsx:1197-1206`) and all counters/cursors (`:1215-1223`). The native encoder prints the same unbounded `u64`/`usize` values directly (`fill/🦀️component.rs:354-372,380-427`), while `FillPreviewJsonIdentity::read` only requires nonzero values (`:42-51`). A 64-bit producer can consequently serialize schema-valid diagnostic data which the renderer fails closed. Neither the language-neutral law nor the Rust/renderer laws declare or exercise those maximum/plus-one cases.

### RED-2 — Status-label schema still uses characters where the wire uses UTF-8 bytes

`statusLabel` has `{ "type":"string", "minLength":1, "maxLength":256 }` only (`preview-json.schema.json:55`). The native cursor rejects more than 256 UTF-8 bytes before reserve (`fill/🦀️component.rs:485-515`) and the parser uses the same byte census (`World3dHost/🟦️component.tsx:1208-1210`). Thus 256 astral scalars (1,024 UTF-8 bytes) are schema-valid but rejected by both implementations. Unlike color, no `x-semio-maxUtf8Bytes:256` schema annotation or language-neutral ASCII/multibyte boundary laws exist. The existing renderer test proves its parser cap for `ü` 128/129 (`index.test.ts:2763-2766`), but not schema/producer parity.

## Required Repair

Make the JSON-schema/law wire domain authoritative for every diagnostic integer and `statusLabel`, then use the same bounds in Rust preflight and the renderer parser. Add portable maximum/plus-one laws for every native-emitted diagnostic integer family, preserving fuel/checkpoint/last valid owners on rejection; add the status-label UTF-8 annotation and ASCII/multibyte 256-byte boundary laws with the independent serde output oracle. The 4 KiB total-wire bound should also be represented consistently if the schema is intended to define full admission rather than only shape.

## Static Checks Performed

| Command | Result |
| --- | --- |
| `jq -S .` on both schema/law fixtures | Both fixtures parsed successfully. |
| `jq` inspection of root/ghost indices, color, and law boundaries | Both index maxima are `9007199254740991`; color annotation is `128`; law records max/plus-one plus all four color byte cases. |
| `jq` census of fourteen diagnostic numeric schemas | All fourteen have only type/minimum, no `maximum`. |
| `jq -e '...statusLabel["x-semio-maxUtf8Bytes"] == 256'` | `false`: status label has no byte-bound annotation. |
| Scoped `rustfmt --check fill/🦀️component.rs` | Exit 0. |
| Scoped source/test line inspection and `git diff --check` | No whitespace errors; confirmed the cited static paths. |

