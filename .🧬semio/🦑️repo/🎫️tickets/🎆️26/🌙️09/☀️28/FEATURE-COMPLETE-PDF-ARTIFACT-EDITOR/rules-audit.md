# 🛡️ Rules Audit: FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR (2026-11-22)

Read-only audit against `/Users/ueli/Documents/semio/CLAUDE.md`. Scope: changed or new non-ticket Rust, TypeScript and Python sources (about 504 files) plus manifests and the 14 named reports. Paths are repo-relative. Emoji path segments are abbreviated with `…` where long.

## Method and limits
- Regex and Perl heuristics over the working tree. Docstring and comment checks are approximate, and continuation lines may be false positives.
- Reports were searched for keywords to assign owners. Hits with no owner are marked `unassigned`.
- Only `git status`, `git diff` and `git grep` were run. No git write commands were used.

## Findings grouped by owning report

### authoring-features.md
1. **HIGH, hand-edited generated asset.** Line 68 says `hub-composition-sync.py` added seven actions to 49 app entries of the hub composition JSON, bypassing the wasm descriptor emitter. Rule: generated assets must not be hand-written. Fix: regenerate with the emitter once the component builds, or remove the hand edit.
2. **LOW, comment inside a definition.** `//#region ✍️Work` in the authoring crate (report line 75). Rule: no comments inside definitions. Fix: drop the region marker or move it outside the impl.
3. **MEDIUM, incomplete work.** G11 is deliberately unchanged (line 69). Rule: get everything working. Fix: track as an open item or implement.

### g5-g7-g10-segmented-io.md
4. **HIGH, raised size caps.** The archive cap went from 4 MiB to 16 MiB. Confirmed in the tree:
   - `🧰️framework/🔨️modules/🪢️wire-contracts/🟦️.ts:25` `DOCUMENT_ARCHIVE_MAXIMUM_BYTES = 16 * 1024 * 1024`
   - `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/📦️archive/🧬️schema/🦀️.rs:3`
   - `🧰️framework/🔨️modules/🎠️kernel/📥️cold-pair/🦀️.rs:3` `COLD_PAIR_MAXIMUM_BYTES`
   
   The report also raises history allocation, cold-pair pages (64 to 256), and local catalog base64 (8 to 24 MiB). Rule: no raised caps or budgets. Fix: keep the caps and make the paths stream, or get explicit user approval.
5. **MEDIUM, not compiled.** Line 119 says none of the changes has compiled, and the >4 MiB reopen law is unproven. Rule: get everything working. Fix: compile and run the law.

### compile-green.md
6. **HIGH, red build.** 48 test-only errors remain in two PDF test files (report line 14). Rule: get everything working. Fix: fix the tests or report them as blocking.

### carve-idempotence.md
7. **MEDIUM, over budget and no test.** The facade crate exceeds the hard budget (line 67: 177 lib and 234 test files). Also, no regression fixture was added for the PDF ring shape (line 30). Rules: get everything working, and add a language-agnostic test for each feature. Fix: add the fixture and resolve the budget breach without raising the budget.

### g4-g6-natural-source-integration.md
8. **MEDIUM, shared-worktree data loss.** Line 22 says a concurrent writer replaced `🧰️framework/…/plugin/👷️job/🏛️reserved/🦀️.rs` with an older copy, dropping the ordinal 22 natural factory. Rule: work with others on the same files. Fix: re-read before edit and verify the factory is present now.

### g14-raw-picker.md and paint-fidelity.md
9. **INFO.** The 24,546-byte picker cap (`MAX_FILE_BYTES`) is pre-existing and untouched. Not a violation. Both reports ship language-agnostic fixtures (`paint-fidelity.json`), which satisfies the test rule.

### Unassigned (no named report owns these)
10. **MEDIUM, leftover debug log.** `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/…/✏️editor/🧪️tests/🔬️unit/🧩️contributions-and-examples/🦀️.rs:136`: `eprintln!("[DEBUG] archive {operation} settled after {turns} turns")`. The prefix is right but the log must be removed before close. Fix: delete the line.
11. **HIGH, deleted language-agnostic test.** Teaching deletions include `🎓️teaching/…/🧪️tests/💨️transport-view/🥒️.feature` (plus its `🟦️.ts` test) and `…/💨️transport/〰️streamlines/🟦️.ts`, which held `MAX_POINTS_PER_SIDE = 120`. Rule: every feature needs a language-agnostic test, and tests must not be weakened. Fix: restore the `.feature` or add an equivalent law.
12. **MEDIUM, ignored tests.** `🧰️framework/…/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/` has `#[ignore = "needs a live hub…"]` on several tests (for example `creation-door-and-cross-shell-meeting/🦀️.rs:15`, `projection-fixtures-and-live-workspace/🦀️.rs:157`, `cross-shell-collaboration-and-sign-in-task/🦀️.rs:72`). Rule: no weakened tests. Fix: gate them behind a fixture or run them in the live lane.
13. **MEDIUM, env-gated skips.** `🧰️framework/…/🦑️repo/…/📦️dependencies/💾️storage/🧪️tests/🟦️.ts:143` and `…/👓️reading/🟦️.ts:132-137` use `test.skipIf(process.env.SEMIO_DEPENDENCY_…)` on producer diagnostic laws. Fix: run them unconditionally or move them to a named lane.
14. **MEDIUM, net assertion loss.** `✏️s/…/4️⃣1.4/…/🖼️page/🧪️tests/🔬️unit/🦀️.rs` loses 12 assertions (`ResizePage`, `remove-page`, `set-page-size` checks) with none added. These may have moved into the authoring crate. Verify, and restore the assertions if they were dropped.
15. **MEDIUM, dependency test net loss.** `…/📦️dependencies/💾️storage/🧪️tests/🟦️.ts`: assertions 15 to 12 with the skip change above. Verify.
16. **LOW, timeout raised.** A new test adds `{ timeout: 240_000 }` in the teaching tree. Verify this is not a weakened budget.
17. **LOW, docstring rule.** Docstrings starting without an emoji (file:line):
    - `🔨️modules/🎠️kernel…`: none
    - `…/🧮️table-and-paint2d/🦀️.rs:77853, 77864` and `…/🧮️table-model/🦀️.rs:48948, 48956`, `…/🗃️scene-state/…/🦀️.rs:77405`, `…/💬…/🧬️component/🦀️.rs:87413`: docstrings starting with `⋯`, which is not an emoji
    - `…/🗃️registries/🦀️.rs:65571`: starts with plain text ("Vertices without…")
    - `🔨️modules/🖼️assets/🌐️published-page/🟦️.ts:60731`: `/// <reference types=…/>` directive
    - `🔨️modules/🧵️job/⏱️context/🦀️.rs:62555, 62824, 62941`: possible continuation lines, verify
    
    Fix: prefix each with a fitting emoji. Keycap-led lines (`#️⃣`) pass.
18. **LOW, comments inside impl bodies.** About 55 indented `//#region` markers and inline comments sit inside impl or fn bodies, for example `…/🚀️native-winit-app/🦀️.rs:72980-73602` and `…/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs:63532-63533`. Rule: no comments inside definitions. Fix: remove the markers or move them out.
19. **LOW, legacy wording.** Doc comments name "legacy" behaviour (`host-base/🦀️.rs:246`, `table-and-paint2d/🦀️.rs:234`, `retained-ui-engine/…/🦀️.rs:381`). These are descriptive, not shims. Optional: reword to present tense.

## Checked and clean
- **Path length:** no changed path exceeds 256 characters (chars, UTF-16 units, or absolute). Passes.
- **AGENTS.md:** no edits.
- **Generated-marker overlap:** the only hit is a test string that quotes Cargo's own header. Passes.
- **i18n:** no English-only literals found in changed TSX or Rust label calls. Regex coverage is limited, so this is a partial pass.
- **Compat or shim layers:** none added. The `shim` hits are build-script shims, a feature rather than a compatibility layer.
- **External runtime dependencies:** no new runtime deps in `Cargo.toml` or `package.json`. The new `derive_arbitrary` entry in `Cargo.lock` has no direct manifest declaration I could find. Verify it is test-only, and check the `windows-sys` bumps 0.52 and 0.60 to 0.61.2.
- **Other:** `console.log` in `🏢️semio-tech/🎡️play` is script output, which is acceptable.

## Housekeeping
- Two stray files I created during the scan were deleted: `$HOME/.cache/_dummy_list` and `/tmp/x`. Confirmed absent afterwards. No other file was written except this report.

## Not run
- Test execution, compilation, and the `repo` MCP checks.

## Style fixes applied
Scope: the changed in-scope sources (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`, `🧰️framework`, `🌎️hub`) listed by `git status`, items 17 and 18 above. No builds were run, and no git write commands were used.

- Comments: removed 120 indented `//` lines inside definition bodies (`//#region`/`//#endregion` markers, `// 🚫️async` notes, and prose comments) across 34 files. Two `// eslint-disable-next-line` directives in `🔁️contributions-refresh/🟦️.ts` were kept because removing them changes lint behaviour. Rationale text from removed prose comments was dropped, not moved into docstrings, where the enclosing item had no docstring. Column-0 `//#region` markers were not in scope and were kept.
- Docstrings: prefixed a unique emoji to 17 docstrings in 10 files (`⋯`, `▏`, plain-text and escape-literal starts; the `\u{1f4cd}`/`\u{1f5c2}` escapes in `🚦️ready/🟦️.ts` became literal emoji). Two `⇥️` docstrings in `🎛️media-presenter/🦀️.rs` already start with an emoji and were left alone. The `/// <reference types>` directive in `published-page/🟦️.ts` was not touched; it is a directive, not a docstring.
- Skipped: none in the end. Files modified within the last 3 minutes (for example `📊️table-model/🦀️.rs`, `🛬️native/🦀️.rs`, `🎞️…/🧮️table-and-paint2d/🦀️.rs`) were retried after the window had passed.
- Count: the audit's "about 55" in-body comments is an undercount for this scope. The scan found 122 indented comment lines in 34 changed files. The whole in-scope tree has about 2,558 such lines in about 405 files, which are unchanged by this work and were not touched.
- Temporary scope list `style-scope.txt` in this ticket folder was deleted after use.

Files changed:
- `🧰️framework/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/🪪️document-identity-and-composite-rows/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/📣️dispatch-and-overlay-api/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/⏱️tooltip-clock-and-overlay-api/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/✍️drag-scroll-and-edit-api/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🏗️router-construction-and-intents/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🖱️accessibility-and-pointer-dispatch/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🫳️drag-drop/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📊️table-model/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/✍️retained-edit-text-steps/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🔄️interactive-state-sync/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🗂️registries/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/📋️contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🚀️native-winit-app/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🧩️extension-splice/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🖱️canvas-gestures-input-kit-and-chrome/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔁️contributions-refresh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧩️session-and-topology/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧰️window-actions-and-commands/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/📨️command-dispatch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/🎬️shell-actions/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/⌨️keyboard-and-palette/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🧬️component/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/⌨️keyboard-and-palette/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/⚙️settings-and-dock-persistence/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/📨️command-dispatch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/📦️typed-pack-load/🛬️native/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts`
