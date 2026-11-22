# Descriptor Emitter Freshness Guard

## Cause
`checkDescriptorEmitterCapabilitiesV1` (driver preflight) and `ensureDescriptorEmitterV1` only tested that the published emitter
(`.cache/b/p/m/browser/<mode>/tools/descriptor-emitter/<platform>/semio-plugin-describe`) existed and answered `capabilities`.
Nothing compared it with the current sources. `.semio-artifact.json` holds only `version`, `owner` and the file list, no source
provenance. A 13 h stale emitter therefore passed, every component linked for ~25 min, and the describe stage failed
(`unsupported window layout kind 'split'`).

## Provenance used
The emitter publication keeps the Cargo compiler receipt (`compilation/receipt.json`, already read and verified by
`withDescriptorEmitterTaskV1` as `tool.observation`). Its `inputs` rows are `[stringIndex, "file", sha256]` for every file the
compile consumed (2973 files, 2055 authored workspace sources; the rest are registry sources under `.cache`). That is the exact
source closure of the emitter crate, so no new digest machinery was needed.

## Fix
- New `🛠️emitter/🌡️freshness/🟦️.ts`: `staleDescriptorEmitterInputsV1(receipt, workspace, signal)` re-hashes authored workspace inputs
  (bounded parallelism via the existing `mapBoundedV1`, cancellable) and lists missing/changed ones; `requireFreshDescriptorEmitterV1`
  throws a bilingual (en/de) error naming `repo run @semio-tech/os-plugin-describe-rs:build`.
- `🛠️emitter/🟦️.ts`: `checkDescriptorEmitterCapabilitiesV1` now runs the check right after the capabilities probe. It runs before any
  lane is admitted (driver `if(!buildsEmitter)` preflight, also reached by `components describe`), so a stale emitter fails in seconds.
  When the plan itself builds the emitter the check is skipped (unchanged).
- Chosen: refuse early rather than auto-rebuild. The preflight holds the artifact reader and the compiler store, and republishing
  there would need the writer under the reader's custody; the refusal names the exact target instead.
- Limitation: a brand-new file is only noticed when a hashed file referencing it changed (always true for Rust `mod` wiring).

## Tests
`🛠️emitter/🌡️freshness/🧪️tests/🟦️.ts` (5 tests): fresh receipt; edited and deleted sources reported sorted; `.cache` and outside-workspace
inputs ignored; receipt without inputs refused; bilingual error text and target. Oracle: hard-coded FIPS 180-4 SHA-256 known
answers (`abc`, empty), so no hashing code is shared with the implementation. Also checked against the live receipt (stale list empty).
Run: `repo test-file <path>` -> 5 pass. Existing `🛠️emitter/🧪️tests/📏️request/🟦️.ts` -> 6 pass.

## Changed files
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🌡️freshness/🟦️.ts (new)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🌡️freshness/🧪️tests/🟦️.ts (new)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts (import and one call)
