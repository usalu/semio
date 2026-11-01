STATUS: DONE (source verification only; nothing compiled or run, UNVERIFIED by execution). 3 conflicts resolved, 1 debug leftover removed, 1 silent-fallback defect fixed.

Paths: FW = `🧰️framework`, OS = `FW/🛍️products/💻️os`. Greps scoped to subtrees.

## The three named questions
1. Guest job-output page cap: jobpage is right, tsfix is stale. `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs:68-81` `read_job_output_step_with` has no `.min(64)` (0 hits in the call subtree); page = `(length-offset).min(grant.maximum_copy_bytes).min(grant.maximum_capacity_bytes)` at line 80. The bound now comes from the host: `FW/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs:21` `OUTPUT_PAGE_BYTES=16384`, `:23` `for_output_page()`, schema `…/💼️job/📄️output-page/🧬️schema/🔣️.json:8` (const 16384), TS `…/🎟️retained-turn/🟦️.ts:71,99` `outputCrossing()`, native host user `OS/🖥️host/🎭️guest-runtime/💼️job-crossing/🦀️.rs` (uses `for_output_page`). Parity tests: `…/💼️job/📥️source/🧪️tests/🦀️.rs:35,39`, `…/🧪️tests/💼️job-funds/🟦️.ts:147`. RESOLVED-IN-CODE.
2. `set-field-drop-safe.py --apply`: it ran. `FW/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🎛️set-field/🦀️.rs:143` `terminal_is_empty` no longer includes `closing&&`, and `:147-151` `Drop` uses the script's `empty` + `debug_assert!(empty||panicking)` + conditional `ManuallyDrop::drop`. RESOLVED-IN-CODE. (Compile still unproven; owner lane mutpolicy.)
3. Holder-reentrant writer starvation: bounded by construction, accepted. `FW/🔨️modules/🏃️process/🔒️leases/⚖️fairness/🟦️.ts:20-25` `queueMayProceedV1`: a holder tree passes a waiting writer at any age (needed, otherwise a wait-for cycle); every other requester queues behind it for `native-fixed-image:` (`writerPreferenceForResourceV1`, `:14-17`, unbounded when the prefix row has no duration). So no new tree can be admitted once a grow writer waits, and the writer waits at most for the already-admitted trees to finish; each tree is finite and only descendants of a holder pass. Pinned by `…/⚖️fairness/🧪️tests/🟦️.ts:281-310` (strangers never overtake an unbounded writer; 2000 random queues: a holder is never blocked). `reentrantPassCeilingMilliseconds` is absent (0 hits in FW), consistent with the removed 6h ceiling. Residual: non-image resources use the general 120 s preference, so a stream of new trees can still overtake a writer there; not a demonstrator-runtime item, left as is.

## Fixed now
- `[DEBUG]` leftover: `FW/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🧪️tests/🟦️.ts` (was line 12, `console.log('[DEBUG] Original-slot … passed')`). Removed; assertions still fail loudly.
- Silent missing placeholder (1030-review-today #9, affects the footer pane chrome): `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🟦️.ts:38-44` `demonstratorFormat` replaced `params[name] ?? ""` by a thrown defect; docstring updated. Law added: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorlocale/🟦️.ts` "refuses a placeholder without a value instead of printing nothing". Only callers are `⚛️footer.tsx:120,151,161`, all pass `name`. UNVERIFIED by execution (vitest not run; files had other lanes' edits at 01:41-01:44, mine are additive).

## Verified RESOLVED-IN-CODE (file:line)
- Text-splice `OriginalTextSpliceAdvance{Unfunded,Progress,Ready}`: `FW/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🦀️.rs:9,58`.
- Document actor `original-turn.stock-exhausted`: `FW/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts:116`, test `…/🧪️tests/💼️job-funds/🟦️.ts:140`.
- `hostTreeWaitsV1`: `FW/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts:159,348`; `startCacheMilliseconds`: same file `:99`, schema `🧬️schema/🔣️.json:99`.
- `cargoLockMissingPathManifestV1`: `FW/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/🟦️.ts:46,98,108`.
- `impl Clone for SealedShared`: `FW/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs:115`.
- `ColdReplayProjection::bounded` + metered clone demand (H2): `OS/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs:257-272,438`.
- `abandon_trapped` (H1): `OS/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/📨️event/🦀️.rs:23`, caller `…/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs:103`.
- Producer `return_held` in Drop (H5/today #4): `OS/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs:56,62,98,121`.
- `BackboneDocument.transitions` plain `Vec` + `retire(self)` (H4), no Drop impl: `OS/🖥️host/🔌️plugin-host/🗃️backbone-document/🦀️.rs:29,101`.
- `admit_record_rows` (stdio #1): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🦀️.rs:140`.
- Record codec `maxOutputLength` (ts #6): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts`.
- Forced-unmount allowlist `-R=anchor apple` + full paths (infra #6): `FW/🛍️products/🦑️repo/…/📏️grow/🧬️schema/🔣️.json:48`; `forcedUnmountDissenters` `:26`; test `…/📏️grow/🧪️tests/🟦️.ts:122`.
- Energy `ProjectionFault` latch (plugins #2): `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/👁️read/🦀️.rs:111`.
- process3d dead call `process3d_release_publication_authority`: 0 hits in `…/🧊️process3d/…/🔬️mounted-registry/🦀️.rs` and in `OS/…/🏇️mounted-owner`.
- `unfunded`/`settle` (stepctx): `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🦀️.rs:55,60`.
- `debug_size_probe`: 0 hits in FW (F8/A2).

## Still OPEN (not touched, with reason)
- hub-gis inference deadline `u64::MAX`, `run_controlled` first turn (H8, plugins #6): owned by running lane gis (`1101-result-gis.md`).
- energy per-edit triple projection, mounted bestest test (energy lane); fem/fem2d/gen compile and mounted tests (lanes fem, gen, p2d, rtpanes, runtime, mutpolicy).
- H9 `member_stores` partial drop (design item), record_binary ignoring `PackEncodeOptions` (stdio 6b), catch_unwind over StepContext (stdio 4), oracle memory bound across segments (test-only).
- Document-child job crossings fund themselves per request (ts #4, by design); jobs run inside `reconcileUiPatches` (ts #5).
- Doc drift: mutderive text says 48-byte tail (F3/today #8); not edited (result files are history).
- Not re-checked, no pane impact: infra #1-3,#5,#8-13,#16 (repo tooling), repo review items, i18n sweep of `ArtifactPresentation.name`/`MediaPortSpec.label`, pubtoken checks for gen2d/gen3d/process3d/raster/playground/playbook, fault position 0.93 (A8).
- Not run by me (no builds started per brief scope): all tests above; the foreign blocker for native mounted runs is `semio-framework-os-store-replay-operation` (lane replay), see `1101-result-runtime.md`.
