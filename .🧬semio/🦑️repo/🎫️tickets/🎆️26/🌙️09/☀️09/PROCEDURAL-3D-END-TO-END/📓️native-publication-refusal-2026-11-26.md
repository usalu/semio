# Native publication refusal (2026-11-26)

## Symptom
`Exact canonical persistent Native publication required: .cache/b/{d,r}` with `filesystemBytes:[authored, policy]`, `schema: []`.

## Root cause
The only failing comparisons in `inspectPersistentReceiver` (`💾️storage/🧹️cleanup/🎛️command/🟦️.ts`) were
`authored.filesystemBytes!==x.filesystemBytes` and `authored.maximumPhysicalBytes>x.maximumPhysicalBytes`.
The image cap (2^35 bytes, macOS APFS unmap limit) and the lowered policy (`budget/🔣️policy.json`: rust 96 GiB -> 32 GiB elastic maximum, dependencies 36 -> 32 GiB) landed after the images were born
(`.cache/b/d` 36 GiB, `.cache/b/r` 96 GiB, both published Nov 11). The budget resolver derived the envelope the receiver must match with
`effectiveCapacityV1` (clamped the receipt to `rule.maximumBytes`) and `pinnedCapacityV1` (ignored a receipt above the declaration), and the candidate budget
with the receipt capacity was additionally rejected by the schema (single-image partition capacity <= cap) and silently dropped. So the receiver was handed 32 GiB for a 36/96 GiB published image.
Nothing was wrong with the image, receipt or disk; `cargo check/test` worked because they do not open that image in exact mode.

## Fix (source, no receipt/image edits)
- `budget/📐️elastic/🟦️.ts`: `effectiveCapacityV1` and `pinnedCapacityV1` return the readable receipt capacity unchanged (a published image keeps its exact envelope; a changed policy only applies to the next image born by the governed reset/regrow/relayout paths).
- `budget/🟦️.ts`: `readRepositoryCacheBudgetV1(value, existing)`; with `existing` the schema judges a single-image partition at the cap (`withoutExistingImageOversizeV1`), so the resolved budget carries the receipt envelope while the declared policy still must stay within the cap (still refused when declared).
  `repositoryCacheBudgetV1` uses it for the receipt-derived candidates and the final budget.
- Governed migration stays with the owning controls: `imageOversizeV1` + reclaim-due regrow, and the in-flight `cache-relayout-rust` (needs the home image openable).
- Tests (`budget/🧪️tests/🟦️.ts`): pinned/elastic receipt rows and Python models updated (above-declared and above-cap receipts kept), new test "an image published above the image cap keeps its exact envelope in the resolved budget" (36 GiB dependencies, 96 GiB rust, declared 36 GiB still refused); stale rust sizing assertion made derive from the policy (another agent had lowered needBytes).

## Results
- `repo test-file` storage-budget: 25 pass, 0 fail. `repo typecheck` on both changed sources: 0 errors.
- Proof: `repo cargo --store=shared --configuration=prototype -- build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown` passes the former refusal and compiles (rustc running).
