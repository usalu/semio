# SPR Value-Contract Fixture Repair

## Boundary

Source-only repair of the SPR test-fixture frontier from the coordinator's member-dialect native build. No production serde compatibility was added; no tests or assertions were disabled. Cargo remains serialized through the coordinator. This packet has no compile or runtime acceptance yet.

## Original Evidence

The retained compiler JSON is `🗑️generated/member-dialect-exact/exact-cargo-laws-pKuwf4/01/build.stdout`. Its SPR-primary diagnostics collapse to missing first-party value derives on test fixtures and obsolete serde bounds in assertions against production protocol types. A read-only census found 309 distinct primary location/message pairs; those are cascading diagnostics, not 309 independent defects. Schema laws from coordinator sessions 71740 and 79256 remain separately credited by the coordinator; this repair does not claim their execution.

## Changes

- Registry `MiniDoc`, `MiniDiff`, `MiniMutation`, and `RenameMini` now implement the current first-party `ToValue`/`FromValue` contract with fully qualified derives.
- Both counter mutation rosters, their direct/composite leaves, and structural diffs implement the same contract. Value attributes mirror the existing test-only serde wire contract: tagged representation, camel-case names, unknown-field rejection, and skipped observation state.
- `AddInference` implements the required first-party value contract. Its deterministic output is compared against its independent test-only serde encoding.
- Production `MutationApplyError`, `MutationMeta`, `Edit`, and `MutationLeafDescriptor` assertions now execute the first-party JSON codec. Existing exact neutral descriptor/error literals, enum wires, omission assertions, and roundtrips remain.
- The canonical contributed-origin byte law uses a separate test-only serde DTO. It compares byte-for-byte output, cross-decodes each implementation's bytes, and preserves the integer-array payload-hash witness without adding serde to production `MutationOrigin`.
- Both neutral mutation fixture harnesses compare real first-party encode/decode with serde. Hostile unknown, missing, and wrong-typed payloads exercise the first-party decoder as well as the independent decoder. Arithmetic and inverse assertions remain intact.

## Verification

`git diff --check -- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr` exited 0 after the source repair. The coordinator owns the next registered same-binary native rerun. No Cargo process was launched by this packet, and no pass count is claimed.

## Paused WAL Work

The inference WAL chain checkpoint is staged separately: a 14-case neutral hash-chain corpus, a Rust law using actual `SprWriter` and an independent `blake3` oracle, and page-bounded validation in the actual `WalReplayCursor`. That work has not compiled or run; its new law is not yet added to the permanent seven-law gate. The prior gate 28714 remains a bounded build timeout with zero discovered laws/assertions, not a pass. Approval integration, multi-segment runtime acceptance, and GIS model execution remain unclaimed.
