# Audit: region markers, child folder names, sizes and docstrings (monolith split)

Read-only audit, 2026-10-15. No source, git state or generated folder was modified. Scratch scripts and JSON are in the session scratchpad (`scan.tsv`, `audit2.json`, `report.ts`).

## 1. Result

- Splits audited: 45 (monolith files with at least 250 lines at HEAD that now have less than 80% of their lines, or were deleted, found by diffing the working tree against HEAD over all changed `.rs`, `.ts`, `.tsx` files).
- Overall verdict: **20 PASS, 25 FAIL**.
- Per criterion (PASS / FAIL; rows 1 and 2 are n/a for N and D because they are deleted): regions 20 / 25; folder names and path length 39 / 4; sizes 41 / 4; docstrings 39 / 4.
- Region names: 132 original names are not present anywhere in the changed Rust/TS set, and 71 are present only outside their split folder (may be coincidental name reuse). Not counted: 14 new presentation modules with zero markers (see finding 1).
- Unbalanced region files: 31 (open count differs from close count), excluding the HEAD originals.

## 2. Findings

1. **Presentation dedupe lost its region markers.** The 14 new modules under `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/` (`auto-animate`, `json-tree`, `deck-chrome`, `deck`, `morph-views`, `morph-context`, `figure-frames`, `pdf-canvas-view`, `reveal-morph`, `figure-zoom`, `figure-scroll`, `arrangement-section`, `interaction-model`, `interaction`) contain 0 region markers. HEAD's `🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` had 20 region names; 18 of them are absent from the presentation react tree. The deleted animate copy (row 2) has the same 20 names. The new parent `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🟦️.tsx` (363 lines) begins with `// #region 🧲️Header`, not a doc comment.
2. **os.ts is not marker-free in the current tree.** The task said one splitter dropped all markers from `framework/🛍️products/💻️os/🟦️.ts`. The current parent keeps 16 balanced marker pairs, and all 40 HEAD region names are present across the parent and its 15 children. Remaining failures: `🔨️modules/🧵️backbone/🟦️.ts` has 8 open and 9 close markers; 4 children do not start with a doc comment (2 start with `#!/usr/bin/env bun`, 2 with `import`); sibling emoji duplicate `🔗️link/🧬️schema/` (`📦️codec` and `📦️packages`). The HEAD file itself is 40 open and 41 close. I could not reproduce "all markers dropped" for this file; if it refers to another copy, please name the path.
3. **Unbalanced region markers** are the most common failure. Affected files and counts are in section 4. Several are in HEAD originals too (for example `🛍️products/💻️os/🟦️.ts` 40/41, `🎤️presentation` C 20/19, `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` 28/26), so the splitter inherited them and did not fix them.
4. **Sizes.** No new child exceeds 2,500 lines (largest child 2,494: 🎛️selection-events/🦀️.rs under the directed/normal monolith). Four parents still exceed the limit: `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (6,569 lines, unsplit, 3 region names not found anywhere), `🌍️world/🦀️.rs` (2,582), `➡️directed/➕️normal/🦀️.rs` (2,669), `🧱️elements/🌐️World3dHost/🟦️.tsx` (2,845).
5. **Sibling emoji duplicates among folders (rows 7, 14, 21, 43).** CAD renderer tests: `🧪️hookorder` and `🧪️repluserfacingsuggestiondetail`. Actor: `🎟️retained-turn/` has `📦️batch` and `📦️packages`; `🧬️ownership`, `🧬️codec`, `🧬️typegen` are siblings; `🧵️pack`, `🧵️shard-runtime`, `🧵️turn-scheduler` are siblings. Job: `🧪️testing`, `🧪️tests`, `🧪️conformance` in `🧵️job/`; `🎟️turn` and `🎟️authority` in `👷️worker/`; `🧪️testing` and `🧪️tests` in `👷️worker/` and in `👷️worker/🎟️authority/`; `📏️extent`, `📏️capacity`, `📏️limit` in `📬️outcome/📦️builder/`. os.ts: `🔗️link/🧬️schema/` has `📦️codec` and `📦️packages`. Duplicates that already exist at HEAD (for example `🧬️mutations/🦴️create-load` in BIM) are not counted.
6. **Paths.** Longest repo-relative path is 220 UTF-8 bytes, under the 240 limit. Measured from the filesystem root (prefix `/Users/ueli/Documents/semio/`, 28 bytes), 3 file(s) would exceed 240 bytes: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🧪️tests/🔬️unit/🦀️.rs`, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/🧪️tests/🔬️unit/🦀️.rs`, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🧪️tests/🔬️unit/🦀️.rs`.
7. **Emoji names.** No new folder name failed the RGI emoji plus U+FE0F rule. Pre-existing folders (for example the ZWJ name `🧑‍🎨engine`) are not checked.

## 3. Verdict table

Columns: R = region names present in the split and every file balanced; N = new folder names are RGI emoji + U+FE0F, no sibling emoji duplicate, paths within 240 bytes; S = no file over 2,500 lines; D = every new file starts with an emoji doc comment (`//!` for Rust, `/**` for TS). "HEAD" column: lines / region open:close at HEAD. "Now" column: parent lines / open:close.

| # | RESULT doc (best match) | Monolith (HEAD path, from `framework/` or `✏️s/`) | HEAD | Now (parent) | Children | Missing names | Unbalanced files | R | N | S | D | Verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | PRESENTATION-DEDUPE (B) | animate copy of `markdown-html-compiler/🟦️.ts`, deleted | 367 / 7:7 | - | 6 of 7 names carried to `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts` (6:6, balanced); `🧪️Tests` (HEAD lines 361-366) not carried | 1 | 0 | FAIL | n/a | PASS | n/a | **FAIL** |
| 2 | PRESENTATION-DEDUPE (B) | animate copy of `⚛️react/🟦️.tsx`, deleted | 5560 / 21:20 | - | 18 of 20 names absent from the presentation react tree (its 14 new modules carry 0 markers) | 18 | 0 | FAIL | n/a | PASS | n/a | **FAIL** |
| 3 | PRESENTATION-DEDUPE | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx` D | 3880 / 0:0 | - / - | deleted (no children in folder) (max 0) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 4 | BIM-MODEL-REHOME | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs` M | 7818 / 2:2 | 128 / 1:1 | 221 children (max 272) | 1 (1 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 5 | ARCHITECT-REGISTERS-SPLIT | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🦀️.rs` M | 7728 / 70:70 | 108 / 1:1 | 11 children (max 1652) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 6 | ARCHITECT-REGISTERS-SPLIT | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧪️tests/🔬️unit/🦀️.rs` D | 4183 / 0:0 | - / - | deleted (no children in folder) (max 0) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 7 | CAD-RENDERER | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` M | 6860 / 22:20 | 425 / 2:2 | 11 children (max 1860) | 21 (2 relocated) | 0 | FAIL | FAIL | PASS | PASS | **FAIL** |
| 8 | not identified | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` M | 494 / 0:0 | 304 / 0:0 | 0 children (max 0) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 9 | PUZZLE-EDITORS-SPLIT | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` M | 4060 / 16:16 | 1358 / 5:5 | 4 children (max 1259) | 8 (3 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 10 | PUZZLE-EDITORS-SPLIT | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` M | 9142 / 19:19 | 1704 / 5:5 | 8 children (max 1991) | 1 | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 11 | HUB-BOOTSTRAP-SPLIT | `🌎️hub/🏗️bootstrap/🦀️.rs` M | 11964 / 18:18 | 586 / 2:2 | 22 children (max 1272) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 12 | HUB-DIRECTORY-INFERENCE-SPLIT | `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` M | 3957 / 1:1 | 136 / 0:0 | 11 children (max 927) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 13 | HUB-DIRECTORY-INFERENCE-SPLIT | `🌎️hub/📇️directory/🦀️.rs` M | 4356 / 21:21 | 241 / 3:3 | 8 children (max 862) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 14 | not identified | `framework/🔨️modules/🎭️actor/🦀️.rs` M | 5313 / 29:29 | 1797 / 11:13 | 7 children (max 1767) | 1 | 3 | FAIL | FAIL | PASS | FAIL | **FAIL** |
| 15 | not identified | `framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/🦀️.rs` M | 1869 / 18:18 | 975 / 16:16 | 0 children (max 0) | 2 (2 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 16 | UI-CYCLES (probable) | `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` M | 8749 / 128:126 | 6569 / 124:122 | 0 children (max 0) | 3 | 1 | FAIL | PASS | FAIL | PASS | **FAIL** |
| 17 | UI-CYCLES (probable) | `framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts` M | 624 / 7:7 | 465 / 7:7 | 1 children (max 168) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 18 | TILED-MAP | `framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs` M | 4188 / 13:13 | 278 / 5:5 | 13 children (max 855) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 19 | MANIFEST-SPLIT | `framework/🔨️modules/🛂️manifest/🦀️.rs` M | 8139 / 42:42 | 590 / 8:8 | 7 children (max 1986) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 20 | not identified | `framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs` M | 372 / 6:6 | 148 / 3:3 | 0 children (max 0) | 3 (3 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 21 | not identified | `framework/🔨️modules/🧵️job/🦀️.rs` M | 3879 / 16:15 | 1282 / 11:10 | 43 children (max 1285) | 0 | 3 | FAIL | FAIL | PASS | FAIL | **FAIL** |
| 22 | not identified | `framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs` M | 784 / 0:0 | 124 / 0:0 | 0 children (max 0) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 23 | PRESENTATION-DEDUPE (C) | `framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` (now 4 lines) | 5551 / 20:19 | 4 / 0:0 (new parent `presentation/🎯️targets/⚛️react/🟦️.tsx`: 363 / 1:1) | 14 modules under `presentation/🎯️targets/⚛️react/🔨️modules/`, all outside the old folder, max 1166 lines | 18 of 20 absent from the tree; the 14 modules carry 0 markers | 0 | FAIL | PASS | PASS | FAIL (new parent starts with `// #region`, not a doc) | **FAIL** |
| 24 | OS-TS-R3F (probable) | `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` M | 4602 / 20:20 | 498 / 2:2 | 12 children (max 1162) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 25 | INFINITE-SPLIT | `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` M | 17409 / 28:26 | 2582 / 6:6 | 13 children (max 2099) | 0 | 3 | FAIL | PASS | FAIL | PASS | **FAIL** |
| 26 | INFINITE-SPLIT | `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` M | 13837 / 7:7 | 2669 / 2:2 | 6 children (max 2494) | 0 | 2 | FAIL | PASS | FAIL | PASS | **FAIL** |
| 27 | INFINITE-SPLIT | `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` M | 5431 / 16:17 | 1650 / 13:14 | 5 children (max 1366) | 0 | 1 | FAIL | PASS | PASS | PASS | **FAIL** |
| 28 | OS-HOST-MCP-WORKSPACE | `framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` M | 5832 / 13:13 | 162 / 3:3 | 12 children (max 1203) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 29 | not identified | `framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs` M | 777 / 0:0 | 484 / 0:0 | 0 children (max 0) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 30 | WGPU-RENDERER-SPLIT | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` M | 19808 / 39:38 | 1010 / 18:17 | 16 children (max 2416) | 0 | 9 | FAIL | PASS | PASS | PASS | **FAIL** |
| 31 | ENGINECANVAS-INTERPRETER | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` M | 7652 / 14:14 | 206 / 0:0 | 9 children (max 1382) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 32 | SHELLHELPERS-WORLD3DHOST-SPLIT | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` M | 8552 / 22:22 | 2846 / 1:1 | 9 children (max 1534) | 21 (3 relocated) | 0 | FAIL | PASS | FAIL | PASS | **FAIL** |
| 33 | WGPU-SCENES-SPLIT | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` M | 12344 / 40:39 | 419 / 0:0 | 15 children (max 2055) | 0 | 5 | FAIL | PASS | PASS | PASS | **FAIL** |
| 34 | SHELLHOST-SPLIT | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` M | 13693 / 38:38 | 592 / 1:1 | 30 children (max 888) | 12 (5 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 35 | WGPU-SHELL-SPLIT | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` M | 35571 / 88:88 | 993 / 2:2 | 20 children (max 2418) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 36 | PLUGIN-RUNTIME-TSX | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` M | 4583 / 16:15 | 370 / 1:1 | 18 children (max 484) | 14 (4 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 37 | SOURCE-TEXT-READERS (probable) | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` M | 4006 / 27:27 | 120 / 1:1 | 17 children (max 844) | 26 (2 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 38 | ENGINECANVAS-INTERPRETER | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` M | 6861 / 23:23 | 158 / 0:0 | 10 children (max 1687) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 39 | SHELLHELPERS-WORLD3DHOST-SPLIT | `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` M | 6709 / 21:21 | 391 / 1:1 | 10 children (max 1099) | 20 (6 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |
| 40 | PLUGIN-INSTANCE-HOST-SPLIT | `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` M | 7296 / 25:25 | 839 / 8:8 | 3 children (max 2373) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 41 | PLUGIN-INSTANCE-HOST-SPLIT | `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs` M | 10928 / 0:0 | 1257 / 0:0 | 6 children (max 2344) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 42 | OS-HOST-MCP-WORKSPACE | `framework/🛍️products/💻️os/🖥️host/🦀️.rs` M | 4667 / 42:39 | 62 / 0:0 | 8 children (max 1360) | 0 | 3 | FAIL | PASS | PASS | PASS | **FAIL** |
| 43 | OS-TS-R3F | `framework/🛍️products/💻️os/🟦️.ts` M | 5776 / 40:41 | 535 / 16:16 | 15 children (max 1891) | 0 | 1 | FAIL | FAIL | PASS | FAIL | **FAIL** |
| 44 | REPO-CLI-GRAPHQL | `framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🦀️.rs` M | 4299 / 13:13 | 36 / 3:3 | 8 children (max 1469) | 0 | 0 | PASS | PASS | PASS | PASS | **PASS** |
| 45 | REPO-CLI-GRAPHQL (probable) | `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/🦀️.rs` M | 5039 / 37:37 | 63 / 3:3 | 5 children (max 799) | 23 (23 relocated) | 0 | FAIL | PASS | PASS | PASS | **FAIL** |

Rows 1, 2 and 23 are hand-annotated (their children live outside the monolith folder). Rows marked "probable" or "not identified" are heuristic matches by file path and keyword against the RESULT docs.

## 4. Per-split details

Only findings are listed. A split with no bullet under a criterion passes that criterion.

### 1. `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts` (D)
- Folder: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/`
- HEAD: 367 lines, region markers 7 open / 7 close, 7 distinct names. Current parent: deleted.
- Region names not present in the split folder (7):
  - `🔖️Schema` -> found only outside folder at `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts`
  - `🔗️UrlPolicy` -> found only outside folder at `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts`
  - `🧩️InlineParser` -> found only outside folder at `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts`
  - `🧱️BlockParser` -> found only outside folder at `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts`
  - `🧾️Serializer` -> found only outside folder at `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts`
  - `🔌️Compiler` -> found only outside folder at `framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts`
  - `🧪️Tests` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✅️analysis-check/🦀️.rs`

### 2. `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx` (D)
- Folder: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/`
- HEAD: 5560 lines, region markers 21 open / 20 close, 20 distinct names. Current parent: deleted.
- Region names not present in the split folder (20):
  - `🧲️Header` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔖️Markdown` -> NOT FOUND in any changed Rust/TS file
  - `🔖️MountOptions` -> NOT FOUND in any changed Rust/TS file
  - `🔖️RevealChrome` -> NOT FOUND in any changed Rust/TS file
  - `🔖️RevealMorph` -> NOT FOUND in any changed Rust/TS file
  - `🔖️ArrangementSettled` -> NOT FOUND in any changed Rust/TS file
  - `🔖️HiddenPreflight` -> NOT FOUND in any changed Rust/TS file
  - `🔖️MorphView` -> NOT FOUND in any changed Rust/TS file
  - `🔖️SlideEpoch` -> NOT FOUND in any changed Rust/TS file
  - `🔌️PdfCanvasPort` -> NOT FOUND in any changed Rust/TS file
  - `🔖️FigureScroll` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Interaction` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  - `🔖️ArrangementSection` -> NOT FOUND in any changed Rust/TS file
  - `🔖️PresentationInteractionProvider` -> NOT FOUND in any changed Rust/TS file
  - `🔖️PresentationDeck` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Mount` -> NOT FOUND in any changed Rust/TS file
  - `🧪️Tests` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✅️analysis-check/🦀️.rs`
  - `🔖️json` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Renderer` -> found only outside folder at `framework/🔨️modules/🖼️canvas/🦀️.rs`

### 3. `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx` (D)
- Folder: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🧪️tests/🧪️compilemarkdowntohtml/`
- HEAD: 3880 lines, region markers 0 open / 0 close, 0 distinct names. Current parent: deleted.

### 4. `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs` (M)
- Folder: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/`
- HEAD: 7818 lines, region markers 2 open / 2 close, 2 distinct names. Current parent: 128 lines.
- Region names not present in the split folder (1):
  - `🔖️Leaves` -> found only outside folder at `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`

### 5. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🦀️.rs` (M)
- Folder: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/`
- HEAD: 7728 lines, region markers 70 open / 70 close, 70 distinct names. Current parent: 108 lines.

### 6. `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧪️tests/🔬️unit/🦀️.rs` (D)
- Folder: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧪️tests/🔬️unit/`
- HEAD: 4183 lines, region markers 0 open / 0 close, 0 distinct names. Current parent: deleted.

### 7. `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` (M)
- Folder: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/`
- HEAD: 6860 lines, region markers 22 open / 20 close, 22 distinct names. Current parent: 425 lines.
- Region names not present in the split folder (21):
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔌️PortWiring` -> found only outside folder at `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`
  - `⚡️R3FPreviewKernel` -> NOT FOUND in any changed Rust/TS file
  - `🎬️WorkerClient` -> NOT FOUND in any changed Rust/TS file
  - `🪩️ArchivedFootprints` -> NOT FOUND in any changed Rust/TS file
  - `📐️Layout` -> NOT FOUND in any changed Rust/TS file
  - `🧲️GeometryTargets` -> NOT FOUND in any changed Rust/TS file
  - `🖼️DisplayPrimitives` -> NOT FOUND in any changed Rust/TS file
  - `🔵️CircleArcPreview` -> NOT FOUND in any changed Rust/TS file
  - `🎨️HostCustomization` -> NOT FOUND in any changed Rust/TS file
  - `🔖️TransformGumball` -> NOT FOUND in any changed Rust/TS file
  - `🎨️SpatialSceneColors` -> NOT FOUND in any changed Rust/TS file
  - `🖱️Interaction` -> NOT FOUND in any changed Rust/TS file
  - `🧲️GeometryInteraction` -> NOT FOUND in any changed Rust/TS file
  - `🧲️SpatialPickGeometryLayer` -> NOT FOUND in any changed Rust/TS file
  - `🎨️TypologyPatternMaterial` -> NOT FOUND in any changed Rust/TS file
  - `🧊️CommittedMesh` -> NOT FOUND in any changed Rust/TS file
  - `🪝️Hooks` -> NOT FOUND in any changed Rust/TS file
  - `🪩️Canvas` -> NOT FOUND in any changed Rust/TS file
  - `🪩️Repl` -> NOT FOUND in any changed Rust/TS file
  - `­ƒº¬Tests` -> NOT FOUND in any changed Rust/TS file
- Sibling emoji duplicates: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/: 🧪️hookorder | 🧪️repluserfacingsuggestiondetail`

### 8. `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (M)
- Folder: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/`
- HEAD: 494 lines, region markers 0 open / 0 close, 0 distinct names. Current parent: 304 lines.

### 9. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (M)
- Folder: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`
- HEAD: 4060 lines, region markers 16 open / 16 close, 16 distinct names. Current parent: 1358 lines.
- Region names not present in the split folder (8):
  - `🔖️Constants` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
  - `🔖️Scene` -> found only outside folder at `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs`, `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs`, `framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/🦀️.rs`
  - `🔖️BoardReads` -> NOT FOUND in any changed Rust/TS file
  - `🎯️TargetRegions` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🖱️pointer-gestures/🦀️.rs`, `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`
  - `📋️Clipboard` -> NOT FOUND in any changed Rust/TS file
  - `🔖️BoardHostSync` -> NOT FOUND in any changed Rust/TS file
  - `🔖️UiScopes` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Puzzle2dCommand` -> NOT FOUND in any changed Rust/TS file

### 10. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (M)
- Folder: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`
- HEAD: 9142 lines, region markers 19 open / 19 close, 19 distinct names. Current parent: 1704 lines.
- Region names not present in the split folder (1):
  - `🙈️SelectionFlags` -> NOT FOUND in any changed Rust/TS file

### 11. `🌎️hub/🏗️bootstrap/🦀️.rs` (M)
- Folder: `🌎️hub/🏗️bootstrap/`
- HEAD: 11964 lines, region markers 18 open / 18 close, 18 distinct names. Current parent: 586 lines.

### 12. `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` (M)
- Folder: `🌎️hub/💡️inference/🏃️runtime/`
- HEAD: 3957 lines, region markers 1 open / 1 close, 1 distinct names. Current parent: 136 lines.

### 13. `🌎️hub/📇️directory/🦀️.rs` (M)
- Folder: `🌎️hub/📇️directory/`
- HEAD: 4356 lines, region markers 21 open / 21 close, 21 distinct names. Current parent: 241 lines.

### 14. `framework/🔨️modules/🎭️actor/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🎭️actor/`
- HEAD: 5313 lines, region markers 29 open / 29 close, 29 distinct names. Current parent: 1797 lines.
- Region names not present in the split folder (1):
  - `🧵️Pack` -> NOT FOUND in any changed Rust/TS file
- Unbalanced files (open/close):
  - `framework/🔨️modules/🎭️actor/🦀️.rs` 11/13 (parent)
  - `framework/🔨️modules/🎭️actor/💼️job/📨️publication/🦀️.rs` 2/3
  - `framework/🔨️modules/🎭️actor/📖️vocabulary/🦀️.rs` 15/12
- Children without leading emoji doc comment:
  - `framework/🔨️modules/🎭️actor/💼️job/📨️publication/🧪️tests/🦀️.rs`: starts with: use super::*;
  - `framework/🔨️modules/🎭️actor/🧬️ownership/🧪️tests/🟦️.ts`: starts with: import{expect,test}from"bun:test";
- Sibling emoji duplicates: `framework/🔨️modules/🎭️actor/🎟️retained-turn/: 📦️batch | 📦️packages`; `framework/🔨️modules/🎭️actor/: 🧬️ownership | 🧬️codec | 🧬️typegen`; `framework/🔨️modules/🎭️actor/: 🧵️pack | 🧵️shard-runtime | 🧵️turn-scheduler`

### 15. `framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/`
- HEAD: 1869 lines, region markers 18 open / 18 close, 18 distinct names. Current parent: 975 lines.
- Region names not present in the split folder (2):
  - `🪪️JobBridge` -> found only outside folder at `framework/🔨️modules/🎭️actor/💼️job/📨️publication/🧪️tests/🦀️.rs`, `framework/🔨️modules/🎭️actor/📖️vocabulary/🦀️.rs`
  - `🔖️JobProgressOverlay` -> found only outside folder at `framework/🔨️modules/🎭️actor/💼️job/📨️publication/🧪️tests/🦀️.rs`

### 16. `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (M)
- Folder: `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/`
- HEAD: 8749 lines, region markers 128 open / 126 close, 127 distinct names. Current parent: 6569 lines.
- Region names not present in the split folder (3):
  - `🌱️TreeUnitDrag` -> NOT FOUND in any changed Rust/TS file
  - `🛟️ChromePanelSafeArea` -> NOT FOUND in any changed Rust/TS file
  - `⛅️Date` -> NOT FOUND in any changed Rust/TS file
- Unbalanced files (open/close):
  - `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` 124/122 (parent)
- Over 2,500 lines: `framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` 6568

### 17. `framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts` (M)
- Folder: `framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/`
- HEAD: 624 lines, region markers 7 open / 7 close, 7 distinct names. Current parent: 465 lines.

### 18. `framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🗺️surface/🗺️tiled-map/`
- HEAD: 4188 lines, region markers 13 open / 13 close, 13 distinct names. Current parent: 278 lines.

### 19. `framework/🔨️modules/🛂️manifest/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🛂️manifest/`
- HEAD: 8139 lines, region markers 42 open / 42 close, 42 distinct names. Current parent: 590 lines.

### 20. `framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/`
- HEAD: 372 lines, region markers 6 open / 6 close, 6 distinct names. Current parent: 148 lines.
- Region names not present in the split folder (3):
  - `🔒️CallbackQuarantine` -> found only outside folder at `framework/🔨️modules/🧵️job/👷️worker/⏱️budget/🧪️tests/🦀️.rs`
  - `📒️SustainedOverrun` -> found only outside folder at `framework/🔨️modules/🧵️job/👷️worker/🎟️authority/⏱️budget/🧪️tests/🦀️.rs`
  - `🌐️PlatformClock` -> found only outside folder at `framework/🔨️modules/🧵️job/👷️worker/🎟️authority/⏱️budget/🧪️tests/🦀️.rs`

### 21. `framework/🔨️modules/🧵️job/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🧵️job/`
- HEAD: 3879 lines, region markers 16 open / 15 close, 16 distinct names. Current parent: 1282 lines.
- Unbalanced files (open/close):
  - `framework/🔨️modules/🧵️job/🦀️.rs` 11/10 (parent)
  - `framework/🔨️modules/🧵️job/👷️worker/⏱️budget/🧪️tests/🦀️.rs` 2/1
  - `framework/🔨️modules/🧵️job/👷️worker/🎟️authority/⏱️budget/🧪️tests/🦀️.rs` 4/3
- Children without leading emoji doc comment:
  - `framework/🔨️modules/🧵️job/👶️scope/🧪️tests/👶️ownership/🦀️.rs`: starts with: use super::*;
  - `framework/🔨️modules/🧵️job/👷️worker/🎟️authority/📦️packages/🦀️rust/📜️script.ts`: starts with: #!/usr/bin/env bun
  - `framework/🔨️modules/🧵️job/👷️worker/🎟️authority/🪜️clock/🧪️tests/🦀️.rs`: starts with: use super::*;
  - `framework/🔨️modules/🧵️job/👷️worker/🧪️tests/🔬️retained-ownership/🦀️.rs`: starts with: use super::*;
  - `framework/🔨️modules/🧵️job/📬️outcome/📤️publication/⚠️fault/📦️packages/🦀️rust/📜️script.ts`: starts with: #!/usr/bin/env bun
  - `framework/🔨️modules/🧵️job/📬️outcome/📤️publication/📦️packages/🦀️rust/📜️script.ts`: starts with: #!/usr/bin/env bun
  - `framework/🔨️modules/🧵️job/🔭️preview/🎟️drive/📦️packages/🦀️rust/📜️script.ts`: starts with: #!/usr/bin/env bun
  - `framework/🔨️modules/🧵️job/🔭️preview/🎟️drive/📦️packages/🦀️rust/🦀️.rs`: starts with: #[path="../../🦀️.rs"]
- Sibling emoji duplicates: `framework/🔨️modules/🧵️job/: 🧪️testing | 🧪️tests | 🧪️conformance`; `framework/🔨️modules/🧵️job/👷️worker/: 🎟️turn | 🎟️authority`; `framework/🔨️modules/🧵️job/👷️worker/: 🧪️testing | 🧪️tests`; `framework/🔨️modules/🧵️job/👷️worker/🎟️authority/: 🧪️testing | 🧪️tests`; `framework/🔨️modules/🧵️job/📬️outcome/📦️builder/: 📏️extent | 📏️capacity | 📏️limit`

### 22. `framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs` (M)
- Folder: `framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/`
- HEAD: 784 lines, region markers 0 open / 0 close, 0 distinct names. Current parent: 124 lines.

### 23. `framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` (M)
- Folder: `framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/`
- HEAD: 5551 lines, region markers 20 open / 19 close, 20 distinct names. Current parent: 4 lines.
- Region names not present in the split folder (20):
  - `🧲️Header` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔖️Markdown` -> NOT FOUND in any changed Rust/TS file
  - `🔖️MountOptions` -> NOT FOUND in any changed Rust/TS file
  - `🔖️RevealChrome` -> NOT FOUND in any changed Rust/TS file
  - `🔖️RevealMorph` -> NOT FOUND in any changed Rust/TS file
  - `🔖️ArrangementSettled` -> NOT FOUND in any changed Rust/TS file
  - `🔖️HiddenPreflight` -> NOT FOUND in any changed Rust/TS file
  - `🔖️MorphView` -> NOT FOUND in any changed Rust/TS file
  - `🔖️SlideEpoch` -> NOT FOUND in any changed Rust/TS file
  - `🔌️PdfCanvasPort` -> NOT FOUND in any changed Rust/TS file
  - `🔖️FigureScroll` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Interaction` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  - `🔖️ArrangementSection` -> NOT FOUND in any changed Rust/TS file
  - `🔖️PresentationInteractionProvider` -> NOT FOUND in any changed Rust/TS file
  - `🔖️PresentationDeck` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Mount` -> NOT FOUND in any changed Rust/TS file
  - `🔖️json` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Renderer` -> found only outside folder at `framework/🔨️modules/🖼️canvas/🦀️.rs`
  - `🧪️Tests` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✅️analysis-check/🦀️.rs`

### 24. `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/`
- HEAD: 4602 lines, region markers 20 open / 20 close, 20 distinct names. Current parent: 498 lines.

### 25. `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/`
- HEAD: 17409 lines, region markers 28 open / 26 close, 27 distinct names. Current parent: 2582 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs` 10/8
  - `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/📇️registry/🦀️.rs` 1/0
  - `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs` 0/1
- Over 2,500 lines: `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` 2581

### 26. `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/`
- HEAD: 13837 lines, region markers 7 open / 7 close, 7 distinct names. Current parent: 2669 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🎛️selection-events/🦀️.rs` 1/2
  - `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill-job/🦀️.rs` 2/1
- Over 2,500 lines: `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` 2668

### 27. `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/`
- HEAD: 5431 lines, region markers 16 open / 17 close, 16 distinct names. Current parent: 1650 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` 13/14 (parent)

### 28. `framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/`
- HEAD: 5832 lines, region markers 13 open / 13 close, 13 distinct names. Current parent: 162 lines.

### 29. `framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/`
- HEAD: 777 lines, region markers 0 open / 0 close, 0 distinct names. Current parent: 484 lines.

### 30. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/`
- HEAD: 19808 lines, region markers 39 open / 38 close, 39 distinct names. Current parent: 1010 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` 18/17 (parent)
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` 0/1
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs` 3/4
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs` 3/2
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs` 2/1
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs` 1/2
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🗂️asset-authority/🦀️.rs` 1/0
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs` 1/0
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🦀️.rs` 0/1

### 31. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/`
- HEAD: 7652 lines, region markers 14 open / 14 close, 14 distinct names. Current parent: 206 lines.

### 32. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/`
- HEAD: 8552 lines, region markers 22 open / 22 close, 22 distinct names. Current parent: 2846 lines.
- Region names not present in the split folder (21):
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔖️World3dHost` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
  - `WorldSceneParsing` -> NOT FOUND in any changed Rust/TS file
  - `WorldMeshPaint` -> NOT FOUND in any changed Rust/TS file
  - `🎉️WorldInstanceCelebrate` -> NOT FOUND in any changed Rust/TS file
  - `WorldViewportCamera` -> NOT FOUND in any changed Rust/TS file
  - `GlbMeshBounds` -> NOT FOUND in any changed Rust/TS file
  - `GlbMeshStyling` -> NOT FOUND in any changed Rust/TS file
  - `WorldInstancesLayer` -> NOT FOUND in any changed Rust/TS file
  - `WorldPointCloudLayer` -> NOT FOUND in any changed Rust/TS file
  - `WorldVortexMarkers` -> NOT FOUND in any changed Rust/TS file
  - `CatalogueDrop` -> NOT FOUND in any changed Rust/TS file
  - `WorldRelocateGesture` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🖱️pointer/🦀️.rs`
  - `WorldVolumeBrushGesture` -> NOT FOUND in any changed Rust/TS file
  - `WorldCatalogueDropPreviewStore` -> NOT FOUND in any changed Rust/TS file
  - `WorldSelectionPreviewStore` -> NOT FOUND in any changed Rust/TS file
  - `WorldGumballTransformPreviewStore` -> NOT FOUND in any changed Rust/TS file
  - `WorldInstanceChromeStore` -> NOT FOUND in any changed Rust/TS file
  - `WorldWindowInstance` -> NOT FOUND in any changed Rust/TS file
  - `🎯️WorldInteractionDomain` -> NOT FOUND in any changed Rust/TS file
  - `World3dHost` -> NOT FOUND in any changed Rust/TS file
- Over 2,500 lines: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` 2845

### 33. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/`
- HEAD: 12344 lines, region markers 40 open / 39 close, 40 distinct names. Current parent: 419 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs` 4/3
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs` 3/1
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/📷️scene-camera-dispatch/🦀️.rs` 1/2
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖋️ink-canvas-render/🦀️.rs` 3/5
  - `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs` 1/0

### 34. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/`
- HEAD: 13693 lines, region markers 38 open / 38 close, 37 distinct names. Current parent: 592 lines.
- Region names not present in the split folder (12):
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔖️BuiltNodeReconciler` -> NOT FOUND in any changed Rust/TS file
  - `FrameworkOsShell` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
  - `🎥️TutorialOverlayHosts` -> NOT FOUND in any changed Rust/TS file
  - `🎥️TutorialRecorder` -> NOT FOUND in any changed Rust/TS file
  - `🐚️ShellMount` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Identity` -> found only outside folder at `framework/🛍️products/🖥️server/🔨️modules/🧬️contract/🦀️.rs`
  - `🏛️SpaceAdministration` -> found only outside folder at `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🏛️space-admin-and-settle/🦀️.rs`
  - `🧵️ConcurrencyHelpers` -> NOT FOUND in any changed Rust/TS file
  - `🩺️RuntimeDiagnostics` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
  - `🔁️InvokeExtensionDispatch` -> NOT FOUND in any changed Rust/TS file
  - `🔖️BuiltNodeStoreCache` -> NOT FOUND in any changed Rust/TS file

### 35. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/`
- HEAD: 35571 lines, region markers 88 open / 88 close, 87 distinct names. Current parent: 993 lines.

### 36. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/`
- HEAD: 4583 lines, region markers 16 open / 15 close, 15 distinct names. Current parent: 370 lines.
- Region names not present in the split folder (14):
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔖️plugin-runtime` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
  - `🔖️Merge` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/🧾️app-channel/🟦️.ts`
  - `🔖️ActorAdapter` -> NOT FOUND in any changed Rust/TS file
  - `📬️TypedOperationResult` -> NOT FOUND in any changed Rust/TS file
  - `🔖️RetainedUiPatch` -> NOT FOUND in any changed Rust/TS file
  - `🔖️GenericThunkQueue` -> NOT FOUND in any changed Rust/TS file
  - `🔖️PluginTurnScheduler` -> NOT FOUND in any changed Rust/TS file
  - `🪦️InstanceRetirement` -> NOT FOUND in any changed Rust/TS file
  - `🔖️ChannelAdapter` -> NOT FOUND in any changed Rust/TS file
  - `🔖️ActorIdentity` -> NOT FOUND in any changed Rust/TS file
  - `🔖️HistorySnapshotRetry` -> NOT FOUND in any changed Rust/TS file
  - `🔖️Transaction` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/🧾️app-channel/🟦️.ts`
  - `🔖️DependencyOrderedBoot` -> NOT FOUND in any changed Rust/TS file

### 37. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/`
- HEAD: 4006 lines, region markers 27 open / 27 close, 27 distinct names. Current parent: 120 lines.
- Region names not present in the split folder (26):
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `🔖️NodeGraphHost` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
  - `Types` -> NOT FOUND in any changed Rust/TS file
  - `🎯️GraphPickContract` -> NOT FOUND in any changed Rust/TS file
  - `Viewport` -> NOT FOUND in any changed Rust/TS file
  - `Parsing` -> NOT FOUND in any changed Rust/TS file
  - `FlowCatalogueSpotlight` -> NOT FOUND in any changed Rust/TS file
  - `Keyboard` -> NOT FOUND in any changed Rust/TS file
  - `DiagramNode` -> NOT FOUND in any changed Rust/TS file
  - `WasmGraphSurface` -> NOT FOUND in any changed Rust/TS file
  - `DiagramFallback` -> NOT FOUND in any changed Rust/TS file
  - `NodeGraphHost` -> NOT FOUND in any changed Rust/TS file
  - `Helpers` -> NOT FOUND in any changed Rust/TS file
  - `Component` -> NOT FOUND in any changed Rust/TS file
  - `🔖️graph-canvas-overlays` -> NOT FOUND in any changed Rust/TS file
  - `DagOverlayTypes` -> NOT FOUND in any changed Rust/TS file
  - `DagOverlayGeometry` -> NOT FOUND in any changed Rust/TS file
  - `📷️ContentFraming` -> NOT FOUND in any changed Rust/TS file
  - `DagOverlayPaint` -> NOT FOUND in any changed Rust/TS file
  - `🎚️SliderGestureLanes` -> NOT FOUND in any changed Rust/TS file
  - `DagDomOverlays` -> NOT FOUND in any changed Rust/TS file
  - `🔖️flow-graph-canvas-host` -> NOT FOUND in any changed Rust/TS file
  - `Sync` -> NOT FOUND in any changed Rust/TS file
  - `📦️SharedFlowPayloads` -> NOT FOUND in any changed Rust/TS file
  - `FlowGraphCanvasHost` -> NOT FOUND in any changed Rust/TS file
  - `🫱️InteractionPublication` -> NOT FOUND in any changed Rust/TS file

### 38. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/`
- HEAD: 6861 lines, region markers 23 open / 23 close, 23 distinct names. Current parent: 158 lines.

### 39. `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/`
- HEAD: 6709 lines, region markers 21 open / 21 close, 21 distinct names. Current parent: 391 lines.
- Region names not present in the split folder (20):
  - `🔌️Adapters` -> found only outside folder at `✏️s/🔌️plugins/✒️writer/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/📜️script.ts`, `✏️s/🔌️plugins/🌊️flow/🏭️bridge/📜️script.ts`
  - `ShellHelpers` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
  - `📥️ChunkedImport` -> NOT FOUND in any changed Rust/TS file
  - `RequestMediaFrames` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🎨️theme-file-import-and-accessibility/🦀️.rs`
  - `Bmff` -> NOT FOUND in any changed Rust/TS file
  - `Tier1` -> NOT FOUND in any changed Rust/TS file
  - `Tier2` -> NOT FOUND in any changed Rust/TS file
  - `WindowLayoutChangeClassify` -> NOT FOUND in any changed Rust/TS file
  - `🧰️UtilityRegistry` -> NOT FOUND in any changed Rust/TS file
  - `🪟️TreeWindows` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧩️shell-core-state/🟦️.ts`, `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
  - `👁️✏️SurfaceRoleLabels` -> NOT FOUND in any changed Rust/TS file
  - `📤️DocumentTransfer` -> found only outside folder at `framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🟦️.tsx`
  - `🔖️CheckInAndSyncStatus` -> NOT FOUND in any changed Rust/TS file
  - `🎥️TutorialUiBridge` -> NOT FOUND in any changed Rust/TS file
  - `🛑️ExtensionRequestCancellation` -> NOT FOUND in any changed Rust/TS file
  - `🥽️Puzzle3dBrushMeshUpload` -> NOT FOUND in any changed Rust/TS file
  - `🧰️WindowActionPane` -> NOT FOUND in any changed Rust/TS file
  - `🎛️CommandRegistry` -> NOT FOUND in any changed Rust/TS file
  - `🛠️ToolRegistry` -> NOT FOUND in any changed Rust/TS file
  - `UiRefresh` -> found only outside folder at `framework/🔨️modules/🛂️manifest/🟦️.ts`

### 40. `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/`
- HEAD: 7296 lines, region markers 25 open / 25 close, 25 distinct names. Current parent: 839 lines.

### 41. `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/`
- HEAD: 10928 lines, region markers 0 open / 0 close, 0 distinct names. Current parent: 1257 lines.

### 42. `framework/🛍️products/💻️os/🖥️host/🦀️.rs` (M)
- Folder: `framework/🛍️products/💻️os/🖥️host/`
- HEAD: 4667 lines, region markers 42 open / 39 close, 37 distinct names. Current parent: 62 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🖥️host/🔌️plugin-host/🦀️.rs` 16/15
  - `framework/🛍️products/💻️os/🖥️host/🖼️media-export-raster/🦀️.rs` 3/2
  - `framework/🛍️products/💻️os/🖥️host/🧱️instance/🦀️.rs` 6/5

### 43. `framework/🛍️products/💻️os/🟦️.ts` (M)
- Folder: `framework/🛍️products/💻️os/`
- HEAD: 5776 lines, region markers 40 open / 41 close, 40 distinct names. Current parent: 535 lines.
- Unbalanced files (open/close):
  - `framework/🛍️products/💻️os/🔨️modules/🧵️backbone/🟦️.ts` 8/9
- Children without leading emoji doc comment:
  - `framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/📦️packages/🦀️rust/📜️script.ts`: starts with: #!/usr/bin/env bun
  - `framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🧪️tests/🟦️.ts`: starts with: import {test,expect} from "bun:test";
  - `framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/📦️packages/🦀️rust/📜️script.ts`: starts with: #!/usr/bin/env bun
  - `framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🧪️tests/🟦️.ts`: starts with: import {test,expect} from "bun:test";
- Sibling emoji duplicates: `framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/: 📦️codec | 📦️packages`

### 44. `framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🦀️.rs` (M)
- Folder: `framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/`
- HEAD: 4299 lines, region markers 13 open / 13 close, 13 distinct names. Current parent: 36 lines.

### 45. `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/🦀️.rs` (M)
- Folder: `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/`
- HEAD: 5039 lines, region markers 37 open / 37 close, 37 distinct names. Current parent: 63 lines.
- Region names not present in the split folder (23):
  - `📜️Schema` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaCommon` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaRepo` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaTechnology` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaBundle` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaFolder` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaFile` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaSection` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaDefinition` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaStatute` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaBreach` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaPolicy` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaInteraction` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaGoal` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaDraft` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaTodo` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaTicket` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaCheckpoint` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaContributor` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaInputs` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🏗️SchemaRoots` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `📜️Sdl` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🏗️repo-schema/🦀️.rs`
  - `🗄️FsRepoContext` -> found only outside folder at `framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🦀️.rs`


## 5. Coverage and limits

- Not auto-detected (no monolith in HEAD shrank below 80%, so they are not in the table): RESULT-CARVE-COMPILER, RESULT-CARVE-PRINT-SERVER, RESULT-CARVE-QUIZ-GRAPHQL (except the graphql row 45), RESULT-CARVE-PETS, RESULT-FRAMEWORK-S-IDS, RESULT-HARDEN-MANIFEST-PLUGIN-HUB, RESULT-HARDEN-WAVE2, RESULT-PLUGIN-CROSS-EDGES, RESULT-TEST-EDGE-RELOCATION. Their region and folder rules were not checked.
- Rows 17, 37 and 45 are heuristic RESULT matches (UI-CYCLES, SOURCE-TEXT-READERS, REPO-CLI-GRAPHQL), not confirmed.
- "relocated" means the region name exists in some other changed file, not necessarily one created from this monolith.
- Children are the new (added or untracked) Rust and TS files under each monolith folder, assigned to the deepest monolith folder that contains them. Moved files can be missed when they land outside that folder (see rows 1, 2, 23 and 45).
- Region names are compared as trimmed text after `region`. Markers are counted by the pattern `//\s*#?\s*(end)?region`, so a marker in a string or a block comment would be counted as well.
- Working tree state was read at the time of the audit; peers may have changed these files since.

