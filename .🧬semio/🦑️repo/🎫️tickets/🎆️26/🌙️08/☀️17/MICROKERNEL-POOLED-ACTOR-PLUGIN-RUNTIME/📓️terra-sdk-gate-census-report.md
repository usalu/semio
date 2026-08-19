# 📓️ terra — sdk-gate-census (independent verification)

MEASUREMENT ONLY. No source files edited. All commands run FOREGROUND with
`CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-gate-census`.

## 1. Compile state — four required exit codes

| command | exit | detail |
|---|---:|---|
| `cargo check -p semio-framework-os-kernel --lib` | **0** | 0 errors, 417 pre-existing warnings — **UNCHANGED from the last verified baseline, did NOT regress.** Log: `check-os-kernel.txt` |
| `cargo check -p semio-framework-ui --lib --features wgpu` | **101** | exactly **3** errors (`E0308` ×3), ALL in one file: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs` (lines 1525, 1532, 1573) — `IconName::as_str`/`IconName::from_str` are still `async fn` but consumed by `impl Display::fmt` (E1, sync) and a sync `if let Some(icon) = Self::from_str(...)` call. **Confirms gate:ui's report exactly** (they reported this as the sole residue, 141→3). File is registrar-only/gitignored `🤖️generated/**`; no packet can fix it directly, matches gate:ui's filed lease-request. Log: `check-ui-wgpu.txt` |
| `cargo check -p semio-framework-schema --lib` | **0** | 0 errors — confirms gate:schema's reported 27→0. Log: `check-schema.txt` |
| `cargo check -p semio-framework-plugin --lib` | **101** | **NOT GREEN.** Exactly the same 3 `icon_name.rs` errors propagate through — `semio-framework-plugin` itself has **zero errors of its own**. Confirms gate:ui's own propagation claim precisely. Log: `check-plugin.txt` |

**Headline: the guest SDK is NOT green.** The entire fleet remains blocked behind one gitignored, registrar-only generated file (`icon_name.rs`) whose `IconName::as_str`/`from_str` need to go sync (both are E1-transitive per R9: consumed by `impl Display` and a plain `if let` in sync code, zero I/O). Per gate:ui's lease-request this needs either (a) sol/a registrar edit to the generator template, or (b) an owner decision to hand-patch the generated file once and re-ratchet.

Because the SDK is not green, per the brief I did **not** run `semio-s-plugin-note` or `cargo test -p semio-framework-plugin --lib` (both conditioned on a green gate).

## 2. Repo-wide first-party `dyn` census

Python, absolute paths, comment/string-literal-stripped (see §5 for a tooling correction made mid-measurement). Scanned `🧰️framework` + `✏️s` + `🌎️hub`, excluding `.🧬semio`, `target`, `node_modules`, any `🎯️target*` — **10,533 `.rs` files**, **229 distinct first-party trait declarations**.

**First-party `dyn` total: 84** (framework 79, fleet `✏️s` 5, hub 0) — down from the previous measurement of 173 (framework 148, fleet 25). The six completed sibling packets this wave collectively closed ~90 sites, consistent with their reports.

### By trait (all 27 families with any remaining first-party `dyn`)

| trait | count | area | status |
|---|---:|---|---|
| `Emit` | 15 | framework (`🛢️db/**`) | untouched by design — os-backbone flagged as a scoping conflict, still unresolved |
| `HostAsyncRuntime` | 10 | framework (`🔌️plugin/🖥️host/⚡️effects/component.rs`) | flagged out-of-scope by dyn-os-misc — **see discrepancy §4** |
| `HttpBody` | 7 | framework (`🛎️services/component.rs`) | deliberately left `dyn` (dyn-os-misc, cfg(test)-only second impl) |
| `HttpTransport` | 5 | framework (`📇️directory/🔌️client` + `🛎️services`) | deliberately left `dyn` (dyn-os-misc) |
| `RouterEffectHandler` | 5 | framework (`⚡️effects/component.rs` ×2 + `⏳️imports.rs` ×3) | dyn-os-misc reported final count as **2** — **see discrepancy §4** |
| `Operator` | 4 | framework (`🌊️flow/📐️brep-geometry` ×3, `🧠️neural/⚙️engine` ×1) | not mentioned by any sibling report — unflagged |
| `OsBackbonePort` | 4 | fleet (`✏️s/🔌️plugins/🪐️space/component.rs`) | documented residue (dyn-os-backbone / space-hub), matches exactly |
| `SpaceBackbonePort` | 1 | fleet (same file) | documented residue, matches exactly |
| `EnvelopeInjector`, `BackboneTransport`, `CapabilityChecker`, `StorageBackend`, `EffectMetricsRecorder`, `ThreadSpawner`, `QuerySource`, `BlobStore`, `AsyncHttpTransport`, `ToolRegistry`, `ResourceRegistry`, `PromptRegistry` | 2 each | framework | `BlobStore`'s 2 sites are new/unflagged — **see discrepancy §4**; the rest not covered by any of the six sibling reports (outside their family lists) |
| `MediaCache`, `ConflictOracle`, `Signer`, `SignatureVerifier`, `CompletionSink`, `DynEngine`, `Backbone`, `MeshExporter`, `MeshImporter` | 1 each | framework | not covered by any sibling report |

Full by-area/by-trait table dumped to `dyn_census_result.json` in this ticket-adjacent scratchpad (path in §6).

## 3. std/lang `dyn` — R1-legal baseline, reported separately

**Total: 134** (`Fn` 50, `Future` 27, `FnMut` 23, `Any` 14, `FnOnce` 12, `Error` 8) — vs previous measurement of 138. Close to baseline; the ~4 difference is within the noise of ongoing edits (dyn-enum closures sometimes touch adjacent `dyn Fn`/`dyn Future` argument-position plumbing that R1 explicitly permits). This is **not a regression signal** — R1 permits this family outright.

Also found **6 more `dyn` sites that are neither first-party nor in the R1-enumerated six**: `Iterator` ×4, `ResourceLimiter` ×2 (a wasmtime external trait). These are legal std/external `dyn` too (not first-party), just not literally named in R1's list — flagging for completeness, not as a defect. One further site, `dyn for<'a> Fn(...)` in `🚪️io/component.rs:1127`, was mis-bucketed by my first tool pass as trait name `"for"` (an HRTB-prefix parsing gap) — it is a legitimate `dyn Fn`, so the true `Fn` count is 51, not 50; noted, not re-run given the 1-site materiality.

## 4. Discrepancies against sibling reports (the requested deliverable)

1. **`RouterEffectHandler`: sibling said 2, true residue is 5.** dyn-os-misc's "deliberately left dyn" table lists `RouterEffectHandler | 2 | sync trait; cfg(test)-only-implementor blocker`, counting only the 2 sites in `⚡️effects/🦀️component.rs` (lines 600, 607). A companion file in the **same directory**, `🔌️plugin/🖥️host/⏳️imports.rs`, carries **3 more** sites at lines 73, 95, 262 — all written as the module-qualified `Arc<dyn crate::effects::RouterEffectHandler>`, which is why a bare-name grep for `dyn RouterEffectHandler` (no path prefix) misses them. Independently confirmed by direct file read. This is a real undercount, not a difference in methodology — the sibling's own reasoning (cfg(test)-only second impl blocks enum-closure) may still be correct, but the census of *how much* dyn survives on that reasoning is off by 3.

2. **`HostAsyncRuntime`: sibling said "7 more", true count is 10.** dyn-os-misc's report says: "found 7 more dyn HostAsyncRuntime uses in `🔌️plugin/🖥️host/⚡️effects/component.rs` — out of scope for this packet". Direct `grep -n "dyn.*HostAsyncRuntime"` on that exact file returns **10** lines (86, 559, 985, 1022, 1034, 1048, 1099, 1221, 1235, 1247), independently matching my python census's total of 10 for that file. This family remains flagged out-of-scope either way (no packet owns it), but the true remaining surface is larger than recorded — worth correcting before any future packet sizes the work.

3. **`Emit`: os-backbone reported 21, current true count is 15.** os-backbone's dyn-census (run before dyn-os-misc's edits) found 21 `dyn Emit` sites, all inside `🛢️db/**`, and explicitly did not touch them. dyn-os-misc's own packet *did* edit several of those same `🛢️db/**` files (`⚙️engine`, `📄️artifact`, `🕸️version-graph`, `🎭️actor`) while closing `VersionGraph`/`ArtifactChannel`/`JoinHandleLike`/`AuthzHook`. The Emit count dropping from 21→15 is very likely incidental fallout of that unrelated restructuring, not a discrepancy in either report — both were accurate as of when each was measured. Flagging for the coordinator since **nobody has explicitly claimed credit for or verified this reduction**, and 15 `dyn Emit` sites remain, unowned, in `🛢️db/**` (marked "completed packet" per os-backbone's brief, so still nobody's to close).

4. **`BlobStore`: sibling reported 7→0 (closed), 2 sites remain in an unscoped file.** dyn-os-misc's table claims `BlobStore (7→0) | generic param B on SpaceRunner/WasmtimeNodeHost + generic fns media_to_artifact/media_from_artifact (all in 🏃️run)`. That work is real and verified for `🏃️run/**`. But `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs` (a **different** module from `✏️s/🔌️plugins/🪐️space` — same name, different tree, not the OsBackbonePort/SpaceBackbonePort file) still has 2 sites: `real_blob_reader(blob_store: &dyn store::BlobStore)` at line 1675 and `import_blob(blob_store: &dyn store::BlobStore, ...)` at line 1693. Not mentioned by dyn-os-misc (whose family list was scoped to `🏃️run`) or by any other sibling. Net effect: `BlobStore` is not fully closed — 2 unflagged residual sites exist.

5. **Tooling note, not a sibling discrepancy:** my first census pass had two self-inflicted bugs (an unanchored raw-string-prefix regex that treated the "r" at the end of ordinary English words like "timer" inside doc comments as a raw-string opener, and a backslash-newline escape handler that ate a literal newline per string-literal line-continuation) that together corrupted parsing for a handful of files enough to produce spurious/missing counts (e.g. an initial run showed `HostAsyncRuntime` at 0 because its trait declaration got silently swallowed). Both are fixed in `dyn_census.py` (fix comments inline); the final numbers above are post-fix and cross-verified against independent `grep`/manual reads for every family that showed >1 in the top list. A repo-wide validation pass (`validate_strip.py`) confirms zero remaining line-count mismatches across all 10,533 files.

## 5. Async-literal census

- **`async fn`: 67,069** · **plain `fn`: 9,646** · **ratio: 87.43%** — up from the previous measurement of 86.8%, moving in the right direction (framework-os-kernel and schema both went green this wave, converting real residue).
- Cross-validated with an independent, uncleaned (comment/string-inclusive) grep pass: 67,838 async / 9,874 plain / 87.29% — within ~1% of the cleaned figures, as expected from doc-comment code examples.
- **`// 🚫️async: E<n>` tags: 301** (E1 140, E4 86, E5 15, E3 60, E2 **0**) — up slightly from the previous 337... **correction: down from 337.** This is itself worth a flag: the previous measurement recorded 337 tags; this measurement finds 301. Either tags were removed somewhere (e.g. as part of an R9 reversion being un-reverted, or a file rewrite that dropped comments) or the previous count double-counted. Not chased further within this packet's budget — recommend a targeted `git log` diff on tag-bearing files if the coordinator wants the delta explained.

### R2 tag-compliance spot check

Sampled **60 random plain-`fn` declarations** (from 9,650 heuristically-matched candidates) and checked for a `// 🚫️async: E<n>` tag within 6 lines above each. **Result: 0 of 60 had a tag.** Manually inspected several: confirmed genuine, uncontroversial violations, e.g.
- `✏️s/🔌️plugins/🗄️stdio/…/🧬️schema/🦀️component.rs:35` — `impl Default for SemioBrepArtifact { fn default() -> Self { ... } }`, zero I/O, plainly E1, completely untagged (checked 15 lines of surrounding context, no tag anywhere).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️component.rs:852` — `fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> { poll_once(fut) }`, a hand-rolled poll-to-completion bridge (textbook E5 shape, `poll_once` right above it manually drives a `Waker::noop()`), inside a test module, with a doc comment explaining *why* it's sync but no `// 🚫️async: E5` tag string anywhere nearby.

**Given the scale (9,646 plain fns vs 301 total tags repo-wide, and 0/60 in an unbiased sample), untagged sync fns are not an edge case — they are the overwhelming majority.** Most of the sample were test functions and small pure helpers/`Default`/`Debug`/`Iterator` impls that plausibly ARE legitimate E1/E5 cases by content, just never received the R2 tag comment. This is a real, large, and previously unquantified compliance gap — worth a dedicated tagging sweep before "zero dyn / universal async" is called done, since R2 explicitly states an untagged exception is itself a defect (indistinguishable from a fn that was simply never converted).

## Files

All saved into the ticket folder (`.../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/`), per R10 ("save recovery/repair tools into the ticket folder") extended here to every tool built during this measurement packet:

- Report (this file): `📓️terra-sdk-gate-census-report.md`
- Raw check logs: `terra-sdk-gate-census-check-os-kernel.txt`, `terra-sdk-gate-census-check-ui-wgpu.txt`, `terra-sdk-gate-census-check-schema.txt`, `terra-sdk-gate-census-check-plugin.txt`
- Census tool (fixed — see §4.5 for the two bugs found and repaired mid-measurement): `terra-sdk-gate-census-dyn_census.py`
- Validation tool (confirms the fix, zero line-count mismatches across all 10,533 files): `terra-sdk-gate-census-validate_strip.py`
- Tag spot-check tool: `terra-sdk-gate-census-tag_spotcheck.py`
- Raw JSON census result: `terra-sdk-gate-census-dyn_census_result.json`

Build target dir (cargo, not carried into the ticket folder per the "cargo target dirs live in scratchpad" rule): `/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-gate-census/`.
