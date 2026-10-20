# Result: emoji-docs (leading emoji on docstrings)

Input: `audit-agents-rules-1019.md` item 6 (1,425 of 40,912 blocks in the touched set). I scanned the whole working tree (51,674 `.rs`/`.ts`/`.tsx`/`.mjs`/`.js` files, 727 `.go` files, `.d.ts` excluded): 177,590 doc blocks, 5,365 without a leading emoji before the work.

## Outcome

- **5,027 docstrings now start with an emoji** in 2,139 files (TS/JS/Rust 4,842 in 2,018 files, Go 185 in 121 files). Fresh tree scan afterwards: 177,531 TS/Rust/JS blocks, 177,052 with a leading emoji, 479 left, all intentional (below). Zero plannable edits remain.
- Existing text is unchanged; only the emoji (plus one space, written `<emoji>\uFE0F `, the repo's own style) is inserted after `///`, `//!`, `/**`, ` * ` or `// ` (Go).

| tree | Rust | TS/JS | Go |
|---|---|---|---|
| `✏️s/🔌️plugins` | 1,717 | 2,007 | 0 |
| `🧰️framework/🔨️modules` | 142 | 675 | 0 |
| `🧰️framework/🛍️products` | 87 | 162 | 185 |
| `🌎️hub` | 5 | 21 | 0 |
| `✏️s/🔨️modules`, `✏️s/🧑‍💻dev` | 15 | 0 | 0 |
| `.storybook` | 0 | 11 | 0 |

By kind of fix: 4,091 dictionary picks, 481 + 8 `@emoji <e> text` tags rewritten to `<e> text` (the old tag form; 8 had no emoji and got a pick), 174 mojibake emoji repaired (`ð§·ï¸` back to `🧷️`), 38 escaped emoji (`\\u{1f4b0}\\uFE0F`) turned into real characters, 32 lone/broken variation selectors replaced by a pick, 185 Go docs (emoji placed before the name, matching the existing `// 🌊️stream carries ...` Go convention).

## Mapping (deterministic, no randomness)

Tools: `C/emoji-docs-tools/` (`map-nouns.ts` about 200 BIM/schema/UI nouns with 4 alternates each, `map.ts` + `map-extra.ts` about 330 concept groups, `pick.ts` selection, `docs.ts` Rust lexer and TS-AST comment finder, `plan.ts`, `apply.ts`, `apply-go.ts` + `godocs/main.go` go/ast).

1. Tokens (camel/snake split, light stemming) from the declared name first, then the first sentence, then code spans, then the directory names; module-level docs (`//!`, file headers) read prose and directory names first. Specific words are tried before generic ones (`type`, `value`, `snapshot`, ...).
2. Candidate list = concatenated emoji lists of the matched concepts. Pick the first candidate unused in the file; for the first 3 candidates also unused in the sibling files of the directory; otherwise the first file-unused candidate; otherwise a fixed neutral pool indexed by an FNV hash of name+head (18 picks, all in files with more than 60 blocks).
3. Never touched: blocks whose first line starts with a code fence, heading, table, list or link definition (none occurred), JSDoc tag-first blocks other than `@state`/`@derived`/`@module`, generated files, `.d.ts`.

Sample review: two rounds of 90 and 100 samples led to the nouns table, the stdio/binary/inverse role words and the tier ordering. Final random sample of 80 (every Nth pick of the final run): about 58 good, 12 acceptable (neutral but not wrong), 10 weak (e.g. `sourceForm`, `cementType`, `ContainerRole`, `id` in diff schemas). About 87 % fit or acceptable.

<details><summary>Final sample of 80</summary>

| emoji | name | docstring head | file |
|---|---|---|---|
| 🎞 | `assets` | Static-dir / tile-proxy / mesh-collection assets served via the existi | 🧭️coordination/🟦️.ts |
| 🎲 | `random_symmetric_model` | A random model whose one relation is genuinely symmetric ('allowed(r,a | 🔬️unit/🦀️.rs |
| 👉 | `selectedGenerationId` | @state presence | 🧬️schema/🟦️.ts |
| 🗺 | `-` | gis TypeScript package | 🟦️typescript/📜️script.ts |
| 🌿 | `register` | vcs import json | ✳️any/🟦️.ts |
| 🧱 | `COMPONENT_PROTOCOL_SEMIO` | binary rep for stdio.json 🔺️diff | 🔺️diff/🦀️.rs |
| 🌳 | `-` | the artifact tree). | 🗺️mesh-preview/🦀️.rs |
| 🔘 | `nodes` | @state artifact | 🔺️diff/🟦️.ts |
| 🏗 | `-` | structure (nodes, bar/frame members, meshed solids, supports, loads) — | 🧱️model/🦀️.rs |
| 📶 | `-` | artifact-level 'mutate-fem3d-1' case in ticket | 🛡️mutate-fem3d-1-boundary/🦀️.rs |
| 🛤 | `railings` | @state artifact | 📸️snapshot/🟦️.ts |
| ⬜ | `slabs` | @state artifact | 🧬️schema/🟦️.ts |
| 🗓 | `-` | program <- json | ✳️any/🦀️.rs |
| 🤔 | `assumptions` | @state artifact | 🧾️program-diff/🟦️.ts |
| 🤸 | `flexibility` | @state artifact | 🧬️schema/🟦️.ts |
| 📸 | `cameraFov` | @state presence | 🧬️schema/🟦️.ts |
| ✨ | `hoveredTargetMode` | @state presence | 🧬️schema/🟦️.ts |
| 📄 | `content` | @state artifact @child kind=s.stdio.semio | 🧬️schema/🟦️.ts |
| ⚙ | `evaluate_expr` | Evaluates an {@link Expr} against 'env' and an outer 'let'-binding sco | 🎬️interaction-spec/🦀️.rs |
| ⚪ | `primitive_vertex_positions` | Collects the world-space vertex positions reachable from a solid/shell | 📐️geometry/🦀️.rs |
| 🧬 | `-` | 'insert-variable' mutation for EN 1990. | 📥insert-variable/🦀️.rs |
| ↩ | `-` | Inverse for 'insert-wind-faces'. | ↩️inverse/🦀️.rs |
| ↩ | `-` | Inverse for 'change-assumed-qf-d'. | ↩️inverse/🦀️.rs |
| ➕ | `-` | 'introduce-product-series' — insert into catalogue.product_series. | 🦠️mutation/🦀️.rs |
| 🪝 | `support` | simply_supported / continuous_2_span | 📸️snapshot/🦀️.rs |
| ↩ | `-` | Inverse for 'change-beam-construction'. | ↩️inverse/🦀️.rs |
| 📜 | `-` | Norm command — 'insert-item'. | ➕insert-item/🦀️.rs |
| 🔺 | `delta_sigma_c` | Detail category Δσ_C [Pa] — derived from detailCategory (one source of | 📸️snapshot/🦀️.rs |
| ↩ | `-` | Inverse for 'insert-building'. | ↩️inverse/🦀️.rs |
| 🏃 | `runOutputJson` | @state config | 🧬️schema/🟦️.ts |
| 🧮 | `Fenestration` | See ANSI/ASHRAE 140 §5.2 cases 610/630/910/930, whose overhang and fin | 🧱️spatial-material-and-gain-entities/🦀️.rs |
| 📄 | `content` | @state artifact @child kind=s.stdio.semio | 🔺️diff/🟦️.ts |
| 🔺 | `id` | @state artifact | 🔺️diff/🟦️.ts |
| 📂 | `deserialize_bytes` | Decode deflate pack then parse. | ✳️any/🦀️.rs |
| 📦 | `bounds` | @derived | 💡️inferences/🟦️.ts |
| 🗃 | `Base` | A surviving item that was at this BASE index. | 🔺️diff/🦀️.rs |
| 💾 | `TAG` | Direct binary identity for 'set-info-title'. | 🏷️set-info-title/🦀️.rs |
| 📥 | `-` | Deserialize stdio.pdf (1.7) from stdio.deflate (raw file bytes in defl | ✳️any/🦀️.rs |
| 📥 | `-` | Deserialize stdio.jpg from stdio.binary. | ✳️any/🦀️.rs |
| 🎬 | `ID` | demo stl | 🎬️demo/🟦️.ts |
| 🖼 | `TiffTagsDiff` | Tag-id-keyed 'entries' triple for one IFD. | 🔺️diff/🟦️.ts |
| ⌨ | `sourceForm` | @state artifact | 📸️snapshot/🟦️.ts |
| 🚦 | `kind` | @derived | 💡️inferences/🟦️.ts |
| 🔳 | `columns` | @state artifact | 📸️snapshot/🟦️.ts |
| 🧬 | `census` | @derived | 💡️inferences/🟦️.ts |
| 📏 | `dimensions` | @derived | 💡️inferences/🟦️.ts |
| 📷 | `cameraY` | @state presence | 🧬️schema/🟦️.ts |
| 📝 | `-` | edit, one history row stamped with its 'TransactionRef', one parametri | 🧪️select-tool-transactions/🦀️.rs |
| 📐 | `-` | rotation turns (exact, never a negated angle that would accumulate flo | ↩️inverse/🦀️.rs |
| 🤝 | `-` | touching one of this node's handles is severed too, re-'connect-handle | 🗑️delete-node/🦀️.rs |
| 🔭 | `-` | projection (part nodes with their rim grips, fastener edges), built di | ◻️2d/🦀️.rs |
| 🧩 | `-` | Puzzle5d mutation — 'ChangePart2dIcon': changes a part's 2D-projection | 🎨change-part2d-icon/🦀️.rs |
| 🌐 | `Complete` | 'depth': how far (m) one body's surface dives into the other solid, me | 💥️overlap-state-machine/🦀️.rs |
| 🧱 | `COMPONENT_PROTOCOL_SEMIO` | 'Puzzle3dMutation''s binary wire form, 'encode_engine_command'/'decode | 🧬️mutations/🦀️.rs |
| ➕ | `-` | exists in 'base'. | 🔺️diff/🦀️.rs |
| 🧩 | `-` | Puzzle3d mutation — 'MoveObject': absolute reposition of an object's o | 📍move-object/🦀️.rs |
| 📐 | `schema` | @state artifact | 📸️snapshot/🟦️.ts |
| 🧊 | `-` | shapes), linear buckling and nodal-averaged stress recovery for contou | 🧮️analyses/🦀️.rs |
| 📏 | `ray_origin` | World-space ray origin for 3D presence (Orbit); absent for canvas/geo. | 🪟️view-records-and-codec/🦀️.rs |
| 🐳 | `ContainerRole` | counterpart, added per the packet brief for the layouts those two coll | 🎛️container-roles-and-row-actions/🦀️.rs |
| 🎬 | `SceneGeometry` | Interface for a geometry entry in a 3D scene. | 🗿️objects/🟦️.tsx |
| 🏷 | `TextareaProps` | TextareaProps holds the data fields for a TextareaProps record. | 🔤️Textarea/🟦️.tsx |
| 📃 | `page` | Initial page when no ephemeral override exists; must be in {@link PdfE | 🎭️embodiments/🟦️.ts |
| 👂 | `watch` | Runs 'run' as one watched turn; rejects with {@link CommandCancelledEr | ⏱️command-stall/🟦️.ts |
| 🧠 | `InMemoryFilesystemAdapter` | Explicit ephemeral storage adapter for browser applications and tests. | 🪞️preview2-shim/in-memory-filesystem.js |
| 🍰 | `SectionsTreeDataProvider` | Provides the tree data for the sections sidebar view of the active fil | 🌳️tree-providers/🟦️.ts |
| ➖ | `cleanDedupePreferDeepest` | Ticket size hits keep the deepest path so 'target/' wins over year/mon | 🔍️candidate-discovery/🟦️.ts |
| 📸 | `onSessionReady` | The owner's session handle ({@link frameDemandingSessionV1}): calls th | 🎨️react-renderer/🟦️.tsx |
| 🌉 | `bridgeSls` | @state artifact | 🔺️diff/🟦️.ts |
| 🏗 | `structureId` | @state artifact | 🔺️diff/🟦️.ts |
| 🔨 | `constructionActivity` | @state artifact | 📸️snapshot/🟦️.ts |
| 🌡 | `thermalBridgeType` | @state artifact | 🔺️diff/🟦️.ts |
| ↕ | `height` | @state artifact | 🧬️schema/🟦️.ts |
| 📐 | `cementType` | @state artifact | 🧬️schema/🟦️.ts |
| 🔁 | `exchangeProcess` | @state artifact | 🧬️schema/🟦️.ts |
| 📏 | `beams` | @state artifact | 📸️snapshot/🟦️.ts |
| ⚡ | `fireScenarios` | @state artifact | 📸️snapshot/🟦️.ts |
| 📨 | `Call` | '<name> = <keyword>(arg1=val1 arg2=val2)' — a named call assignment (t | 📐️model/🦀️.rs |
| 🗄 | `cacheSchemaVersion` | Local event index under .🦑️repo/⚡️cache for keyword search. Uses a co | 🎊️query-cache/🐹️.go |
| 🌉 | `Adapter` | Adapter is the registration entry point the generated host calls. | 🧬️definition-kind-derivation/🐹️.go |

</details>

## Uniqueness

- Picks: 0 collisions inside a file (dictionary picks are always file-unused; the 18 pool picks too).
- 55 emojis in 29 files appear more than once, all because the repaired corrupted emojis (174 mojibake, 38 escaped) restored an emoji the author had already used twice in that file; I kept the author's emoji.
- Module level: only the sibling files of the same directory are consulted (leaf directories hold one `🦀️.rs` and one `🟦️.ts`), repeated role docs across sibling modules (`Inverse for ...` always ↩, `Diff for ...` always 🔺) are intentional.

## Not changed, decisions needed

- **Generated files, 258 blocks:** `✏️s/🔌️plugins/🏙️bim/.../🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` and the BIM value-types file (213, header says edit the model in `.🧬semio/.../BIM-PLUGIN/r3-f1-gen-model.ts`), and 45 blocks generated by `bun ./📜️script.ts surface-schema` ("do not edit"). Fix at the generators. The norm TS twins (510 `@state artifact` docs, header "Generated from ./🔣️.json by 🧪️s2-norm-ts-twins.ts", a one-off ticket script nothing runs) were edited directly.
- **Symbol-led docs, 207:** docs that start with a deliberate glyph such as `⇥️`, `⏎️`, `⎋`, `▭️`, `Σ`, `∫`, `√` (keyboard keys and math). They fail `\p{Extended_Pictographic}` but are clearly icons; left alone. Replace them if the law must be strict.
- **JSDoc tag-first, 14:** `@param`, `@type`, `@see`, `@default`, `@description` blocks in `.mjs`/`.ts` (type tags would stop working with text before them).
- **`@emoji` tag form removed (489):** the tag was a legacy form of the rule; nothing reads it (checked), so the emoji now leads directly.
- **HARD FREEZE slip (my mistake):** my first pass did not exclude the frozen library path. It edited 4 files under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` (6 docstring lines): `⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts`, `⚡️caching/🌐️vite/🌐️service/🟦️.ts`, `🚨️policy/🔖️breach/🟦️.ts`, `🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts`. Comment-only changes (verified), but they changed mtimes while the dependency sync may be walking. I did not revert (another edit would restart the walk again); the tools now exclude `📚️library/`, root `📜️script.ts`, `package.json`, `bun.lock`, `🔣️taxonomy.json`. No other frozen file still needs a docstring edit.
- Files edited by others in the last 15 minutes were deferred and re-checked later; all deferred files already had zero remaining edits.

## Verification

- Per file before writing: line count unchanged, only comment lines differ, TS/JS `ts.transpileModule(removeComments)` output identical before and after, doc block count unchanged, mtime unchanged since read. Failed: 0.
- `bun build --no-bundle <file>` on all 661 changed TS/JS files (590 + 71): 0 failures.
- `rustfmt --edition 2021 --emit stdout < file` on all 1,357 changed Rust files: 13 report `error[internal]: left behind trailing whitespace` (all norm `✏️editor/🦀️.rs`); the HEAD version of each fails identically, so pre-existing. 0 parse errors.
- Go: `gofmt -e` clean on every changed file; 7 changed files are not gofmt-formatted but were not at HEAD either (import order). No `go build` run (comment-only).
- Field discovery: `policyExtractTypescriptSchemaFields` over 309 changed `@state` schema files (718 types), HEAD vs now: 0 differences, so the `@state` reader is unaffected by the emoji prefix.
- No docstring/emoji law exists (searched `Extended_Pictographic`, `docstring`): nothing to run. No proc macro reads `#[doc]` (searched). No test runner was started (brief: no `bun repo run`, no directory test runs).
- Not run: `cargo check`, any `bun repo` command, Go tests.

## Open items

- Decide: strict handling of the 207 symbol-led docs and the 14 tag-first JSDoc blocks.
- Generators for BIM entities and surface-schema must emit the emoji (258 blocks).
- Re-run `C/emoji-docs-tools/apply.ts` (arguments: output dir, `apply`, targets list) after the freeze lifts if new docs appear in `📚️library/`.
- Rust mojibake remains in doc *text* after the emoji in a few files (`â` for an em dash), not touched.
