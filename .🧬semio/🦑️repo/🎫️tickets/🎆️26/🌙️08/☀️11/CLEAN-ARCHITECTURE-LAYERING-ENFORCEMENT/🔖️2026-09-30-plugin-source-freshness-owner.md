# Plugin Source Freshness Owner

Actual source freshness production moved from development activation into the neutral plugin build owner. The new owner directly defines all source-scan bounds/concurrency, bounded asynchronous mapping, file/stat/content-hash structures, aggregate hashing, staged metadata reading/writing and boot digest reuse. Every real consumer imports that owner directly. Development activation retains its activation receipt and policy; it has no forwarding source-freshness exports.

The existing SourceStatIndexV1 schema declaration moved to the same neutral owner. Its single current wire identifier remains semio.dev.source-stat-index/v1, so the active source-index input remains valid without any alias or legacy parser. Descriptor output admission now derives both exact active metadata filenames from STAGED_SOURCE_FRESHNESS_FILES. Unknown children remain preserved and rejected. No active staged metadata was deleted or rewritten by this worker.

## Exact Created Manifest

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🔍️source-freshness/🔣️.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️source-freshness/🟦️.ts
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔖️2026-09-30-plugin-source-freshness-owner.md

## Exact Updated Manifest

- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🏃️execution/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts

## Exact Removed Manifest

None. Production definitions and one schema definition moved in place; no input or active source-stat index was removed. Root owns fresh schema/catalog/launcher generation.

## Executed Validation

The red trigger was the parent's actual Sequence guest materialization failure: assertPluginOutputChildren rejected an actively written .source-stat-index.json carrying the valid existing schema and source digests. This worker preserved that input and repaired ownership and closed admission.

Owned Nx framework-plugin canonical-architecture --oracle-only passed the new seventeen-case AJV/WebCrypto/file oracle plus the existing eight-case extension retirement oracle. The new law validates the fixture and actual SourceStatIndexV1 with third-party AJV; independently recomputes each source hash and the aggregate through WebCrypto; invokes the actual descriptor closed-admission function; verifies both metadata byte strings survive unchanged; confirms two warm source digests are reused without rehashing; confirms an unexpected input is preserved and rejected; and exercises actual source-walk cancellation. Runtime console observation recorded [DEBUG] admission of two exact metadata files, reuse of two source digests and preservation of the rejected input. That temporary log has been removed from source.

Owned Nx framework-os-dev:test with the unchanged staging-root suite passed 56/56 tests (1 file, 1.28 seconds). Existing laws cover the source tree, metadata exclusion, digest/stat reuse, changed/add/remove files, bounded concurrency, cancellation and event-loop yielding. TMPDIR/TEMP/TMP routed its generated filesystem inputs into this ticket's generated folder.

Final clean-source oracle and complete neutral canonical native rerun passed: both portable oracles and all nine exact native laws, binary SHA-256 0421b8795d9a5dda8f34635e0044cd3c574f7a3c0c78a2fd72c6fbdf7116334e. No native feature is introduced by this move; root owns the actual Sequence component pipeline retry and catalogue refresh. No pass is claimed for a pipeline that this worker did not execute.

Final unrelated extension Drop-guard source rerun also passed the unchanged freshness17 oracle, expanded retirement9 oracle and ten exact native canonical laws. Binary SHA-256 1ed7a26e3e95d5c198a3f093508e9990278e104f620ad4d1861944a39681eef7.

Worker-owned generated oracle/test logs and temporary filesystem inputs were removed after these results were recorded. Inputs, reports, active metadata and other workers’ output remain intact.
## Later Unchanged Oracle Receipt

The allocation follow-up neutral canonical target executed the unchanged seventeen-case source-freshness oracle again. It passed alongside retirement thirteen on the final actor fixture source. Native aggregate progress remains reported in the extension retirement report; this unchanged oracle receipt does not imply a native pass for concurrent UI edits.

Final clean neutral eleven-law canonical run passed after the shared UI owner settled its source, along with the unchanged freshness17 oracle and retirement13. Binary SHA-256 3f3c8a1c219a8f5f6fe96ac170c5924796655bae4fa3cb119b0b6a3595a9dbdf. Whole-corpus and actual guest producer runs remain parent-owned.
