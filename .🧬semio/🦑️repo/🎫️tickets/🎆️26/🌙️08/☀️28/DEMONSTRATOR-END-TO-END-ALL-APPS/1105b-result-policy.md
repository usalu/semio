STATUS: steps 1-3 DONE; unwrap/expect 12 of 16 fixed (by deleting dead fns), 4 need a design call; #[allow] triage listed, 0 dead_code/unused fixed (none added since 10-28; old ones unverifiable while cargo is gated). Compile verification NOT RUN (no lock-ok).

## 1. Source copies in C (deleted, verbatim, cmp-checked against the repo original)
1102-tools/actinfra/patches/{emitter-request-bytes.test.ts, artifact-refusal-reason.test.ts, retention-size-law.test.ts, worker-demand.test.ts, strip-debug/module.ts, strip-debug/test.ts}; 1101-tools/ts2d/patches/{booleans.test.ts, paths.ts}. Empty dir strip-debug left out by rmdir (non-empty check ok). Not touched: other ticket `26/10/18 GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/.../build/📜️script.ts` and `.../🔧️tools/🏗️bazel/📜️script.ts` (2 tracked copies, listed for the owner).

## 2. [DEBUG] leftovers
All hits were in files last modified before 10:00 today (latest 08:47); script `C/1105b-tools/policy/dbg_plan.py`. Removed: 28 print/log statements (eprintln!/println!/console.log/error, Bun.write) incl. in embedded JS oracles; stripped the `[DEBUG] ` prefix from ~30 assert/expect/panic/Error messages. Files: framework tests (admission dag/freshness, large-close, edge-reads, native-supersession, registration, ephemeral, capture, birth, member receipts, work, original-view-read, lifted job mutation, native diagnostics), plugins (flow, brep mesh, shooting/raster/playground/norm/html/mp3 sqlite oracles, fem native-tests, energy thermal/source, semio natural, sparse, reasoning reorganize, catalog shooting-route).
Left on purpose (protocol or tooling, not leftovers): `⚡️caching/.../🔌️nx/📦️graph/` (`[DEBUG] Fixed UDRW image` log protocol parsed by production code, `[DEBUG] graph-root` test protocol, fixtures), custody/🌊️output script+test (child stderr contract), the docstring-and-debug-census checker + its source-census fixture, cargo dependencies consumption test (asserts absence).
Possible follow-up: removed native-tests.rs eprintln may leave `steps`/`total` unused (warning only) in the fem 3d native tests.

## 3. Docstrings without emoji in the demonstrator closure
Closure = 8 pane roots (process3d-editor, sourcing-curation, gis2d-unit-tests, cad-cad, puzzle-3d-editor, fem-3d-editor, energy mounted tests, composition-laws) expanded over path deps (normal+build; roots' dev) = 1393 crates (`C/1105b-tools/policy/closure.mjs`). Scan (`docscan.py`, `docscan_all.py` incl. tests): 0 true violations. Two hits (`semio-framework-hash`, `semio-framework-deflate` line 1 `//! #⃣ ...`) start with a keycap emoji, accepted. The 105 repo-wide violations are outside the closure (norm plugins `//! more mutation leaves`). No edit needed.

## 4. unwrap/expect and #[allow] per crate (closure, non-test files only, text before first `#[cfg(test)]`)
Totals: .unwrap()/.expect( = 16, #[allow]/#![allow] = 382 across 385 crates. Full table: `C/1105b-policy-counts.tsv`. Caveat: only .rs under each crate dir, files named *test* and tests dirs excluded, so far lower than the audit's workspace-wide 10138 (that counts `workspace` fallbacks).
Top: semio-framework-pack allow 2; 12 stdio crates (epw-io-registry, binary-io-registry, txt-tests, xml-schema-2, deflate-tests, zip-opc, json-tests, png-tests, md-tests, csv-io-registry, svg-mutations-2, dxf-tests) unwrap 1 + allow 1 each; all others 0/1 (semio-plugin, os-kernel, framework-async/dispatch-macros/machine/schema, stdio-semio-video-* and value-mutations).


## 5. unwrap/expect fix (follow-up wave)
All 16 closure hits were `io_registry::register()` in the mirror files (`🪞️mirror/🦀️.rs`): `register_composer_entries(..).expect("static Stdio registration ...")`.
- Fixed 12 (epw-io-registry, binary-io-registry, deflate-tests, zip-opc, json-tests, csv-io-registry, svg-mutations-2, dxf-tests, stl-examples, ply-io-registry, obj-io-registry, las-tests-2): root cause = the fn is dead. Proof: walked every `#[path]`-mounted file of each crate (no `io_registry::register()` call), the whole repo has only 5 call sites (gif, ifc, png-editor, txt-io, bmp-io, none of these crates), and no consumer names `io_registry::register` (reexports are `pub use ..::io_registry`). Deleted `register()` and the now unused `register_composer_entries` import (script `C/1105b-tools/policy/dead_register.py`, fail-closed pattern, rustfmt parse OK).
- NOT fixed 4 (txt-tests, xml-schema-2, png-tests, md-tests): their mounted artifact `io` file calls `crate::io_registry::register()` from its own `pub fn register()`, one of stdio's protected imperative plugin-root calls that return `()` and carry ~10 more `.expect` each outside the crate dirs. Typed propagation means `register() -> Result<(), IoRegistryRegistrationError>` through the artifact register chain up to the stdio plugin setup (112 mirror files share the pattern). Needs a design decision / dedicated wave.
- Counting caveat: my 16/382 counts only cover files under each crate dir; most sources are mounted via `#[path]` from ancestor dirs, so the real closure numbers are much larger (audit: 10138 unwrap/expect repo-wide).

## 6. #[allow] triage
- Added since 2026-10-28 (base ec6b067 = last commit before that date; `git diff -G'allow\('`): 758 added repo-wide, 9 in non-test files of the closure trees (list `C/1105b-policy-allows-added.tsv`): async_fn_in_trait x2 (energy snapshot reference, framework dispatch), clippy::too_many_arguments x5 (energy snapshot reference, store delegating operations, 2x wgpu chrome context with reason, 1 wgpu row bounds with reason), non_camel_case_types x1 (runtime node native), drop_bounds x1 (retained-clone derive). No dead_code/unused_* among them, so nothing to delete.
- Pre-existing dead_code/unused allows inside closure trees: 45 dead_code + 1662 unused_* (list `C/1105b-policy-dead-allows-closure.txt`; mostly `unused_imports` on generated mutation/snapshot reexports). Not deleted: cargo is gated, many are wasm/feature-gated or reserved API (documented reasons), and deleting without compile proof risks the build. Candidates with explicit "not yet called" reasons: wgpu events accessors (tooltip/drag-scroll/pointer), plugin declaration/registration captures, guest-runtime/mock-guest, scene-records, wasmtime-node-host `blob_store`.
- Counting caveat as above (crate-dir-only scan found 328 async_fn_in_trait, 52 clippy::result_large_err, 3 unused_extern_crates [`extern crate self as ..` alias, required by macros], 2 unused_imports/ambiguous_glob_reexports in framework-pack [glob reexports], 1 dead_code).

## 7. Follow-up (coordinator message, 2026-11-06): NOT EXECUTED, plan only
- Freeze check: `🗑️generated/1105b/freeze` does not exist under C (checked 2026-11-06), so the stop condition did not trigger. The `🗑️generated` folder itself exists; only the freeze marker is the stop signal.
- The 4 remaining sites are `expect("static Stdio registration ...")` in `register()` of txt/utf-8/any/io, xml/1.0/valid/io, md/commonmark/any/io and png/1.2/any/editor (`io_registry::register()` and `register_subset_validator(...)`). Typed propagation needs `register() -> Result<(), IoRegistryRegistrationError>` (or `SubsetValidatorRegistryError`), and every caller must propagate it: the stdio root `🧿️semio/🦀️.rs` (about 20 `register()` calls), `⚙️engine::register()` in the standard roots, and the plugin root `🗄️stdio/🦀️.rs`. Those roots are the protected imperative plugin-root calls that currently return `()`. Changing them is a cross-crate contract change, not a local fix.
- Not run: no edit, no `#[allow]` removal, no dead-code removal, no wasm check. Repo cargo tools were not loaded, and the lock state was not verified. Removing code without compile proof risks breaking the build (see section 6).
- Decision needed from coordinator: approve the protected-root signature change (about 25 files across stdio root, the four standard roots and the four io files), or accept the four sites as documented exceptions.

### 7b. Scope check for the approved register() Result chain (2026-11-06, no edits made)
- Freeze marker `🗑️generated/1105b/freeze` still absent.
- The approved chain is far larger than the ~25 files estimated. Under `✏️s/🔌️plugins/🗄️stdio/` there are 154 non-mirror `pub fn register()` definitions, and about 100 `.expect("static Stdio registration ...")` sites in roughly 60 io/editor files (for example ifc, epw, wav, bmp, mp4, avi, mp3, svg, jpg, and the whole `🧿️semio` subset family). The 4 sites in the earlier audit (txt, xml, md, png) are only the closure-visible subset.
- Converting only the 4 sites would leave their callers in the other ~150 register functions unconverted, and the plugin root `🗄️stdio/🦀️.rs` still calls `()`-returning functions. That is a partial conversion, which the coordinator's per-crate completeness rule forbids.
- No claims lines were added for the chain. The earlier line in `1105b-claims.md` covers the plan only.
- Not done: the per-crate wasm checks, the hub boot check, and the dead-code re-triage.
- Decision needed: (a) the full stdio plugin conversion (about 60 io files, about 150 register fns, one claims line per file, a wasm check per crate), or (b) the narrow option (convert only the four named subsets and their direct engine callers, documenting the remaining ~96 sites as known exceptions).

### 7c. register() Result codemod: dry-run done, NOT applied (2026-11-06)
- Codemod: `1105b-tools/policy/register-result/codemod.py`, file list generated from the tree into `files.txt` (182 files, `🧪️tests` excluded and reported as boundary). Fails closed on unbalanced braces, multi-expect lines, one-line bodies and unmatched expects.
- Dry-run output (no source file touched): `🗑️generated/1105b/policy/register-result.diff`, staged copies in `🗑️generated/1105b/policy/stage/`, counts in `🗑️generated/1105b/policy/counts.json`.
- Counts: 186 `register()` signatures (158 full + 88 `{}` stubs), 126 `.expect("static Stdio ...")` lines replaced by `?`, 8 call statements `X::register();` to `X::register()?;`. Removed lines = 320 = 186 + 126 + 8, no other line touched. Added lines 418 (signature 186, `Ok(())` inserted 98, `?`/`?;` lines 126, call lines 8).
- rustfmt parse check of the 182 staged files: 0 real parse errors. 64 files declare out-of-line modules, so rustfmt reports a module lookup only. They are verified by cargo, not by this check.
- Return type used: `Result<(), semio_plugin::io::RegistrationFault>`. This type does NOT exist yet and must be added to the framework (`🧰️framework/.../🚪️io`) before apply, with `From` impls for IoRegistryRegistrationError and SubsetValidatorRegistryError.
- Boundary, NOT converted by the codemod (apply needs manual edits):
  - Stdio root `🗿️artifacts/🧿️semio/🦀️.rs` `register(conversions)`: 20 calls (lines 85-103) plus `(conversion.register)()` in the loop. Its signature must return Result, and it is the plugin boot surface.
  - Repo-wide callers of the root: 3 tracked files (`🌍️gis/.../🔬️relocated-engine` test, `📸️remodel/.../io` production, `🖨️raster/.../semio-bridge` production). These callers must propagate the Result too.
  - Test-only callers: `💾️binary/🧪️tests/🔬️io-registry-unit` line 19, `🎞️gif/🧬️migrations/🧪️tests` line 77, and the six `🧪️tests` mirror `register()` fns (excluded from the list).
- Stop checks: the freeze marker `🗑️generated/1105b/freeze` is still absent. Tree check: `git status` shows no changes under `✏️s/🔌️plugins/🗄️stdio`.

### 7d. Dead-code re-triage (static, git grep -w word count over all tracked files, lock files excluded)
- 45 `dead_code` allows in the closure list (`1105b-policy-dead-allows-closure.txt`), table at `🗑️generated/1105b/policy/dead-code-retriage.tsv`.
- Candidates with only the definition line in tracked files (word count 1), not removed: `reclaim_orphaned_registry_slots` and `make_seed_vertex`. `clear_drag_payload` and `output_registry_occupied_count` match outside `*.rs` (docs or other text) and are not candidates under this filter. The earlier `*.rs`-only run listed all four; the TSV on disk is the all-files run and leaves two.
- 17 entries could not be matched to a single item name (macro-generated or multi-line signatures). Not triaged.
- Removal is deferred until the apply window: each candidate needs a `cargo check` of its crate, and a `git grep -w` per removal, as the coordinator required.

### 7e. Flat re-stage and apply status (2026-11-06)
- Step 0 done: `🗑️generated/1105b/policy/flat/<sha1-of-repo-path>.rs` (182 files), `preimage/<sha1>.rs` (182 pre-images, the restore source), `mapping.json` (sha1 to repo path, pre sha256, post sha256). Max absolute path 181 chars, 0 over 256. Nested `stage/` deleted.
- Step 1 NOT done: RegistrationFault (en+de messages, From impls from the subset faults; the existing Display impls are English only), stdio root `register(conversions)` with the fn-pointer loop and its 3 tracked callers, test-only callers, and removal of the 4 definition-only items (re-grep pending). Nothing staged for these yet.
- Step 2 NOT done: no claims lines appended for apply, nothing applied, no wasm or hub checks run. The stdio tree is unchanged by this lane.
- Blocker for a 45-minute fix-forward window: step 1 is not staged, and the wasm checks have not been run once on the converted tree.

### 7f. Step 1 and apply: stopped before apply (2026-11-06)
- Localization convention: the framework's typed faults (`IoRegistryRegistrationError`, `SubsetValidatorRegistryError`, `TextError`, `Diagnostic`) carry stable codes and English `Display` impls. No framework-level en/de catalog for fault messages exists (only the demonstrator's `🌐️locale/🔣️.json`, keyed by UI text ids, not fault codes). RegistrationFault will follow the `IoRegistryRegistrationError` pattern. German catalog entries are NOT added, because there is no catalog to add them to. Gap, needs a decision.
- Fan-out found: `SemioConversion.register` is `fn()` (`🗿️artifacts/🧿️semio/.../✉️base/🦀️.rs:273`) and has 27 construction sites. Changing it to `fn() -> Result<(), RegistrationFault>` touches all 27 plus the converted register fns they point at.
- Not done: RegistrationFault type, the root and fn-pointer loop, the 3 tracked callers, the test callers, dead-item removal (only `reclaim_orphaned_registry_slots` and `make_seed_vertex` confirmed by the all-files grep), claims lines, apply, cargo checks, hub boot.
- Why the apply stops here: the wasm check scope is all stdio crates (about 180 by the codemod's file set) plus a hub boot. Not reachable within the 45-minute fix-forward window, and a partially green apply would break peers editing stdio.

### 7g. Step 1 stage complete, apply on hold (2026-11-06)
- Re-staged against the CURRENT tree by `1105b-tools/policy/register-result/stage_step1.py` (imports the codemod). Previous codemod-only stage (`register-result.diff`) is superseded and deleted.
- Flat store: `🗑️generated/1105b/policy/flat/` (post-images), `preimage/` (pre-images), `mapping.json` (sha1 to repo path, pre and post sha256). 198 files, hash check 0 mismatches.
- Diff: `🗑️generated/1105b/policy/step1.diff`. Removed 397 lines, added 517. Counts in `stage1-counts.json`: 194 register signatures (186 from the codemod, 6 test mirrors, the root, and the SemioConversion field type), 132 `static Stdio` expects to `?`, 31 register call statements (8 codemod, 19 root, 2 test callers, 2 png calls), 1 fn-pointer loop, 24 dead-item body lines (2 functions and their attributes), 11 helper lines.
- Parse check: 0 real errors across the 198 post-images (77 files have out-of-line modules, so rustfmt reports only a module lookup there).
- Included changes:
  - `RegistrationFault` (framework `🧰️framework/.../🚪️io/🦀️.rs`, appended): stable codes `stdio.registration.composer` and `stdio.registration.subset-validator`, Display impl, Error, Clone, From impls for both subset faults. No catalog added, per the framework convention.
  - `SemioConversion.register: fn() -> Result<(), RegistrationFault>` (`✉️base/🦀️.rs`). The 27 construction sites are private `fn register()` bodies, converted by the codemod.
  - Stdio semio root `register(conversions)` returns Result; 19 calls propagate with `?`, the fn-pointer loop becomes `(conversion.register)()?`, `Ok(())` at the end.
  - Raster and remodel `ensure_stdio_semio_and_png_registered` (production): OnceLock<Result<..>>, callers `.map_err(|error| error.to_string())?` to keep their `Result<_, String>` public APIs unchanged. Boundary: pane APIs carry the fault as its Display string.
  - Gis test helper and the two test callers (io-registry-unit, gif migrations): `.expect("...")` in test code only.
  - Dead items removed, both grep-confirmed as definition-only in source: `reclaim_orphaned_registry_slots` (`🖱️ui/🧠️runtime/📤️output`) and `make_seed_vertex` (`🧊️3d/.../sweep-machinery`). The 3 grep hits per name include my own report lines.
- Not done: claims lines and apply (on hold until "APPLY"), cargo checks, hub boot.
