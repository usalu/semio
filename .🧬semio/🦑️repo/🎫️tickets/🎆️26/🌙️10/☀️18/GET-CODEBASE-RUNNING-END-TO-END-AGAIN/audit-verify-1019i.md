# Audit verify-1019i: comment strip, Vite loader, tsc staging, rg-test2, docs

Auditor `aud-verify9`, read-only. Scratch under the session scratchpad (`aud/`). One stray scratch file was written into the repo root by mistake and deleted again (`lineb-sites.txt`). No source edited.

## Verdicts

| Item | Verdict | Concrete fix |
|---|---|---|
| comments-a removals (sample 60 + full log of 9,329 rows) | PASS with one defect class | Restore the 35 `/** */` blocks removed from inside component bodies as docstrings of their enclosing declaration (AGENTS allows docstrings, not body comments). One of them is an anchor of a live test (pair A below). |
| comments-a: `// 🚫️async:` notes (1,674 rows in comments-a log, about 1,710 more in comments-b) | No reader found, no breakage | Decide whether the exemption register is wanted. If yes, move it to a schema field or docstring. Nothing in the repo parses the marker (see below). |
| comments-a: directives, `#region`, SAFETY, license, Rust doc | PASS: 0 rows removed | none |
| comments-b anchor sweep (all 10,232 rows of `comments-b/all-removed.jsonl` plus comments-a) | 3 broken test pairs (A, B, C) | See "Anchor pairs" below |
| comments-b `line!()` seed file | No shift, no repin needed | Optional: replace the seed with an explicit key (see below) |
| e2e-recheck `configLoader: "native"` | PASS for play and demonstrator; 2 gaps | Add the variant configs to the permanent loader law; os-dev runner result inconclusive |
| tsc-stage patches (10 spot-checked, 42 ops total) | PASS on `any`; symptom-level widenings in part | Type the fixture rows so `expect<string>` widenings are not needed (16 ops) |
| rg-test2 `pub` widenings | Partly justified | Put the test-only items behind a `test-support` feature instead of permanent `pub` |
| docs-products (10 statements vs project.json) | PASS 10/10 | none |

## 1. comments-a removals

- Sample: 60 rows drawn with seed 1019 from all 9,329 rows of `🗑️generated/comments-a/ap-*/apply-removals.jsonl`. Classified by reading each text. 58 are plain rationale comments inside bodies. Item 32 is a `/** */` block inside a body (`⏪️ Each change of the focused program's session reveals the History panel...`). The rest are continuation lines of multi-line `//` blocks.
- Full-log scan with strict patterns: `#region`/`#endregion` 0, TS directives (`@ts-*`, `eslint-*`, `biome-ignore`, `istanbul`, `@vite-ignore`, `@license`, `sourceMappingURL`, `/// <reference`) 0, Rust `rustfmt::`/`clippy::`/`#[`-attributes 0, SAFETY 0, license headers 0, Rust `///`/`//!` 0.
- `/** */` blocks inside bodies: 35 (all TS/TSX, all in `comments-a`). These are not attached to a declaration, so the rule removed them. Several document behaviour, for example `ShellHost/🛫️spawned-session/🟦️.ts:301` and `ShellHost/📥️document-load-effects` (see pair C). Fix: restore each as the docstring of its enclosing named function or component, and keep the `it/test/describe` docstrings that were kept already.
- `🚫️async:` notes: `git grep` over tracked TS, TSX, MJS, JS, PY, TOML, GO and JSON files finds the marker only in one law test as a fixture string (`🧬️mutation-leaf-generics/🟦️.ts:433`, which builds its own sample source and does not read the repo), and in one Rust doc comment (`🚪️io/🦀️.rs:52`). No gate, law or tool reads it. The removal therefore broke nothing, but the R9/E1/E4 register it carried is gone. Decide and document.

## 2. Anchor pairs (coordinator request)

Method: every string literal of at least 10 characters and 3 words in 16,797 tracked test and law files (paths with `test`, `law`, `spec` or `fixture`) was matched against the text of all 9,329 comments-a rows and 10,232 comments-b rows. Hits were kept when the line is an indexOf, includes, toContain, slice or split call and the file reads source text. Shorter anchors (1 to 2 words) are not covered.

Broken pairs (the test reads source text, searches a phrase that was a comment, and the comment was removed):

| # | Test (reads) | Anchor literal | Removed from | Status | Fix |
|---|---|---|---|---|---|
| A | `🧪️tests/🪟️spawned-program-session/🟦️.tsx:302` (`shellHostSource` = `ShellHost/🟦️.tsx` module tree) | `A program the user just opened takes keyboard focus` | `ShellHost/🛫️spawned-session/🟦️.ts:301` (comments-a, `/** ⌨️ ...`) | BROKEN (confirmed by fnsplit3) | Restore the docstring on the enclosing declaration of that effect, and change the test to anchor on the code (the focus call), not on prose. |
| B | `🧪️tests/♻️hot-swap-session-handover/🟦️.ts:108` (`flow-session-attach/🟦️.ts`) | `A retained surface host unmounts exactly once` | `NodeGraph/🔌️flow-session-attach/🟦️.ts` (comments-a, `/** 🪪️ ...`) | BROKEN: `indexOf` returns -1, `expect(-1).toBeGreaterThan(0)` fails | Restore the docstring above the enclosing declaration. Preferred: anchor the test on `law.sourceScan.nodeGraphSurfaceEffectKey` and the unmount call, so no prose is needed. |
| C | `🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/📡️reprojection-status/🟦️.tsx:108-109` (reads `ShellHost/🟦️.tsx`) | `const task = core.documentTransferRef...trackDocumentTransfer(file, baseSession.pluginId);` and `` `Effect::LoadDocument` is pack+spr bytes only `` | the code moved to `ShellHost/⚡️host-effects/📥️document-load-effects/🟦️.ts` by the split; the prose note was removed there (comments-a, line 42) | BROKEN, two causes: file split (both strings absent from `ShellHost/🟦️.tsx`, so `slice(-1, -1)` returns empty) and comment removal | Retarget the read to `⚡️host-effects/📥️document-load-effects/🟦️.ts`, anchor on the code only, restore the `Effect::LoadDocument` note as docstring of `applyLoadDocumentEffect`. |

Cleared (matched, but not broken):
- `🧪️tests/🔬️unit` Rust "at least one object", "panic in a destructor" and similar: runtime error strings, not read from source.
- `🧬️schema/🧪️tests/🧾️record-list/🦀️.rs` "not a record": runtime value.
- `⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts:289` `source.includes("registration.contract == row.contract")`: the string is code in the production file (verified by `git grep`), not only in a comment.
- `🧪️tests/🧬️...` `expect(...).not` and negative bans (`Box<dyn Future`, `"ui.chrome.appearance"`): removing a banned token from a comment cannot break a ban check.
- `micro-commit-staged-index` "nothing is staged": git stderr, not source.

Fix note: the fnsplit3 result also lists `Paint2dHost/…/selection-focus` and `window-fault`. Those are not comment anchors and are not in this audit.

## 3. comments-b `line!()` seed

- The file is `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️support/🦀️.rs`, line 101: `framework_hash::hash_bytes(concat!(file!(), line!()))` in `new_tile_id`.
- comments-b removed 5 rows from this file, all at lines 139 to 143 (below the seed). Line 101 is unchanged; `git show` of `7880cc36f78`, `bd9e4e02bd0` and `fd14da55f18` shows `line!()` at 101 in each. The seed value is therefore unchanged.
- No fixture repin is needed for this file. Callers: `add_tile` and `engagement_submit` (`🎮️commands/🀄️add-tile`, `⌨️engagement`).
- Fragility, not a current defect: any future edit above line 101 changes every tile id generated by `new_tile_id`. Recommend an explicit counter or a content-derived key in a follow-up.
- The other three `line!()` files (`⏱️rate`, `🎬️scene/🧩️sobjects`, `native-build/📥️resources`, `🧮️compiler`, and the 37 sites in `interactive-state-sync`) have zero removal rows in both logs. Their line numbers are unchanged. Restoring them is not needed.

## 4. e2e-recheck `configLoader: "native"`

Checked with `e2e-tools/resolve-vite-config.mjs <config> build runner` (Node-default loader):

| Config | runner | Verdict |
|---|---|---|
| play `🏗️builder/🌐️vite/🟦️.ts` | FAILED: Captured inventory Source module load failed | Needs native (fixed at 4 call sites, verified by grep: `runtime/📜️script.ts` serve and serve-test, `site/🟦️.ts`) |
| demonstrator `🏗️builder/🌐️vite/🟦️.ts` | FAILED, same | Needs native (fixed at the same 4 kinds of site) |
| course `🏗️builder/🌐️vite/🟦️.ts` | RESOLVED | no change |
| hub admin `🛡️admin/🏗️builder/🌐️vite/🟦️.ts` | RESOLVED | no change |
| presentation 33 `📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts` | RESOLVED | no change |
| pets react `🎯️targets/⚛️react/🏗️builder/🌐️vite/🟦️.ts` | RESOLVED | no change |
| variants `🌍️gis` and `🧩️puzzle` `🏗️builder/🌐️vite/🟦️.ts` | FAILED, same inventory error | Same closure. Under native they pass the inventory step and stop at a missing activation artifact (`registry/dist/sessions/.../🟦️.ts`), which is expected before activation. Served through the os-dev builder, which passes native. Add to the law. |
| os-dev `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` | FAILED: "Vite module runner has been closed" after 64 s | Inconclusive. The native rerun in the background produced no output and I did not retry it. Re-run this one. |

Not a Vite config (exclude from any law): the "config must export" entries for `styling/🏗️builder`, `styling/📦️packages`, `pets/📦️packages` and `os-dev/📦️packages` are helper modules.

Open: no permanent law yet resolves each product config through its loader. The `resolve-vite-config.mjs` tool is the oracle.

Limitation: the play and demonstrator native results are taken from `result-e2e-recheck.md`, not re-run here.

## 5. tsc-stage staged patches

- No `any` cast, `@ts-ignore` or `@ts-expect-error` added in any of the 42 ops.
- Spot-checked 10 patches (`patches.json`, entries 1, 4, 6, 8, 10, 12, 14, 16, 18, 20): path budget (`pathBudgetCodes`), `Object.freeze` contextual type, `BazelTaskV1` import from its owner, `CacheProbeSpecV1` typed once, the `Fixture` type gaining `nativePins`/`windowsLongPaths`, the `dotenv` oracle replaced by `node:util` `parseEnv` (system library, acceptable), the capture contexts trimmed to the fields the implementation reads, and the DEBUG `console.error` removal. These are root-cause.
- Symptom-level: 16 of 42 ops widen the type argument of `expect<T>` (`bun:test` expect). That silences the checker without changing runtime. Fix: type the fixture rows (or `classify`'s return union) and remove the widenings.

## 6. rg-test2 `pub` widenings

- Commit `fd14da55f18` changed five private functions in `📬️paged-one-item/🦀️.rs` to `pub fn` (`retain_seal_returns`, `returned_edit_demands`, `close_returned_edit`, `returned_post_demands`, `close_returned_post`). It also made `enum Stage` in `🧬️mutation-apply/🦀️.rs` `pub` and re-exported it at the crate root (`pub use mutation_apply::{..., Stage}`). So five functions plus one enum and its re-export.
- Justified only by the move of the test into an external `[[test]]` target (`mutation_apply`, verified in `Cargo.toml`). Not acceptable as permanent public API of a library crate.
- Fix: gate them as `#[cfg(any(test, feature = "test-support"))] #[doc(hidden)] pub mod test_support { pub use ... }`, enable the feature via the crate's own dev-dependency, and keep the rest `pub(crate)`.
- The `#[cfg(test)] #[global_allocator]` in `📬️one-item-owners/🦀️.rs:20-22` is acceptable: it is test-only, it applies only to that crate's unit-test binary, and a global allocator must sit at a binary's crate root.
- Compile evidence: none, as the result states. `cargo check` was blocked by `Cargo.lock`.

## 7. docs-products: 10 statements vs project.json

Read from `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`, `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`, `🏢️semio-tech/🎡️play/📋️project.json`, `♻️mit-bestand/🧺️demonstrator/📋️project.json`.

1. Course `test` cached: true. 2. Course `typecheck` cached: true. 3. Course `test-e2e` has no `workspace:deps-browsers` dependency: true. 4. Proctor `build` cached: true. 5. Proctor `test` cached: true. 6. Play `prepare-prototype` cached: true. 7. Play `activate-prototype` cached: true. 8. Demonstrator `prepare-release` cached: true. 9. Play and demonstrator `prepare-e2e` depend on `workspace:deps-browsers`: true for both. 10. Ports: course dev 6061, proctor 8791, play 6033 (in the files): true. Also checked: play `test-release` and `release-verify` have no `build` dependency: true.

## Open items

1. Restore the 35 body-level `/** */` blocks as docstrings, and re-anchor tests A, B, C as above.
2. Decide the `🚫️async:` register.
3. Re-run os-dev `resolve-vite-config.mjs` under native; add a permanent loader law.
4. Replace the `pub` widenings with a `test-support` feature.
5. Type the fixture rows to drop the 16 `expect<T>` widenings.
