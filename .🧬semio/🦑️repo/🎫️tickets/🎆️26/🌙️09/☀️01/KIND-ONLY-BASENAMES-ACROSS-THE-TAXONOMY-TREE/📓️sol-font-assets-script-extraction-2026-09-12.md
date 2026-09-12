# Font and Asset Script Extraction

## Result

The Infinite packed-font contract, styling font/catalog/download implementation, styling projection and verification, icon/metabolism projection, asset publication, and logo animation implementation now live in anonymous TypeScript leaves under their semantic owners. The three mandatory `📜️script.ts` entries retain command routing, task sequencing, progress/error output, cancellation wiring, and native process staging, and export no reusable API.

The assets and styling byte plans did not change. The registered native font tool produced and staged the packed font artifact through the moved validator. All nine repaired style roots exist. Parent/nested and repeated scan roots are now traversed once, so both collectors return unique diagnostics.

## Source Ownership Map

| Former implementation host | Anonymous semantic owner | Responsibility |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/🟦️.ts` | `FONT_ASSET` and little-endian packed-font validation |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️fonts/🟦️.ts` | catalog parser, exact font identities, provider CSS parser, cancellable acquisition and progress |
| same | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts` | theme resolution, TS/CSS/Rust/Python/.NET rendering, output manifest validation, preview/check/write |
| same | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛡️verification/🟦️.ts` | px/color scan authority, predicates, current coordinates, duplicate traversal prevention |
| `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts` | icon path identities, SVG normalization, language renderers and catalog output plan |
| same | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🏗️builder/📽️projection/🟦️.ts` | metabolism icon catalog and TS/Rust output plan |
| same | `🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts` | declared output membership, write/prune/check/preview/publication |
| same | `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts` | owned jsdom boundary, matrix/SVG parsing, keyframes, animated SVG and MP4 export |

The styling command entry decreased from 1,046 to 73 lines and the assets entry from 962 to 68 lines. The Infinite entry is 43 lines and keeps the platform-native build/stage composition needed by its two registered commands.

## Consumer and Producer Closure

The following support files were changed in this slice:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts`: imports the packed-font owner and preserves atomic temporary staging, the 60-second native process budget, signal cancellation, output caps, validation and cleanup.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`: the existing font staging/receipt consumer imports the same owner directly.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts`: routes generation, preview, check, font acquisition, verification, and tests to the extracted owners.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts`: imports `fetchElementsFonts` from the semantic font owner.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/vitest.config.ts`: selects the projection leaf that contains the inline tests. The browser facade is no longer treated as a test file.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`: imports the font catalog/provider APIs directly and retains Ajv and provider-oracle cases.
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts`: delegates catalog, metabolism, logo, publication and preview commands.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`: TypeScript/Bun compiler oracles now read the logo and icon projection owners; the Rust include oracle uses the actual `../` relation from the generated binding.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: registers `asset-builder-projection`, `asset-builder-publication`, and `ui-styling-verification`; adds the semantic owners to `assets-build` and `styling-tokens`; replaces the absent font schema input with `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧬️schema/🔣️.json`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️framework-source-topology/🔣️.json`: declares the eight extractions, contexts, producer inputs, packed-font vectors, nine root moves, hostile style cases and repeated/overlapping-root control.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️framework-source-topology/🔣️.json`: validates that portable contract.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts`: checks anonymous leaves, zero script exports, semantic contexts, generator inputs, Buffer/Ajv/compiler oracles, current root reachability, hostile findings, overlap deduplication and live logo coordinates.

The existing producer registrations remain the executable authority:

- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json`: build/preview/check/generate-logo/logo routes and declared outputs.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json`: generate/preview/check/fonts/verification/test routes.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📋️project.json`: font-tool/fonts routes and dist outputs.
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`: existing generator/logo and portable topology launch entries were exercised through their Nx commands. This slice did not change either launch file.

A repository search found no remaining consumer import of the extracted APIs from any of the three command scripts. References to command paths are executable routes, generator contracts, or the portable assertion that command leaves export no implementation API.

## Styling Coverage

The nine repaired px roots are:

| Former absent coordinate | Current reachable coordinate |
| --- | --- |
| `framework/module/ui/js/react` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react` |
| `framework/module/ui/styling/js` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling` |
| `framework/product/os` | `🧰️framework/🛍️products/💻️os` |
| `framework/product/os/module/dev` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev` |
| `s/plugin/flow` | `✏️s/🔌️plugins/🌊️flow` |
| `s/plugin/cad/renderer` | `✏️s/🔌️plugins/📐️cad` |
| `s/plugin/puzzle` | `✏️s/🔌️plugins/🧩️puzzle` |
| `framework/os/kernel/infinite/world` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world` |
| `s/plugin/gis/2d` | `✏️s/🔌️plugins/🌍️gis` |

The color roots use those current coordinates except the styling source-of-truth root and add `.storybook`. The ten existing allowance paths were rebased to their current story, CAD, OS host, and Infinite world referents.

The original current-root run exposed duplicate traversal because OS contains its dev and Infinite world descendants. The independent retained probe in `📓️styling-overlapping-root-probe-2026-09-12.md` produced two identical rows for one hostile source. Both collectors now keep a visited-directory set per invocation. A portable parent + nested + repeated-root case produces exactly one px and one color result.

The live gates intentionally remain red on actual style debt:

- `check-no-px`: 6 unique diagnostics across 4 files, all `tailwind-arbitrary-px`.
  - `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:468`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📡️EventFeedHost/🟦️.tsx:133`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:537`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:543`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:539`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:757`
- `check-no-raw-colors`: 174 unique diagnostics across 52 files: 147 `raw-hex-color`, 15 `raw-rgb-hsl-color`, and 12 `tailwind-palette-color-class`.

No exemptions or unrelated UI styles were changed to force these gates green.

## Source and Output Identity

The pre-extraction source hashes retained for provenance were:

| Source | SHA-256 before |
| --- | --- |
| Infinite `📜️script.ts` | `eff33619ac1fd274a80cdaf1e5e372767debc79031a138b42b34b3f902c774d3` |
| styling `📜️script.ts` | `eea3fe303d537c0d496d48eb6d06e5235feb75634c194c64b2169c89ca0a7209` |
| assets `📜️script.ts` | `168db60a4aff07b905bc9d9e6659b51a082c755bd2ecd9893c9bcaf9c26b348d` |

These hashes are report evidence only; no permanent live-source hash assertion was added.

The final anonymous-owner hashes are:

| Owner | SHA-256 |
| --- | --- |
| Infinite fonts | `b55f049246d4f5590fa5a34195977bc245cdb2bb014b673e6d9188e5b339938c` |
| styling fonts | `4319d6a2b5a98246adffd50dc51c3a62cfb7d5c3df587df06fb95ad6fe37ca90` |
| styling projection | `1b2228f86508bf7082ec13ebd52fe844d5e59dcf4338501e97276b00ca63f6b9` |
| styling verification | `18b475d2926781c9de3650091d49742d5cff324cf8ef3cc7a1b08b6c9226b520` |
| icon projection | `67ec2965509ccefceae3aa9943168f636580429e4d22f3cc7e384e6e0feaf524` |
| metabolism projection | `71fc1f300af0db57f8947b035cd433a7e52602a1e0b0a6e6495bc944abe9c668` |
| asset publication | `7670f36cb243239533849bbebe7779751c07923d9bcd3e2b3d67a7f17f47de28` |
| logo animation | `6076692e213bf79cf1aeaedecffad1cd4174a28c064a7d89ada0c581dc34e4a4` |

Pre-extraction, post-extraction, and post-build previews were byte-identical:

| Contract | Bytes | SHA-256 |
| --- | ---: | --- |
| `assets-build` | 2,632,301 | `9508f622b50bf2cfeb696ea41f22154d0f473d1f04901a6417be1266676ea812` |
| `styling-tokens` | 171,988 | `e0e953712617411358f640e03dcad3ef68e66a8affb9c894c41106005c45e41b` |
| animated logo SVG | unchanged | `87767f0a403ef99eb863e6bc1721cfb63b8c0053159c3d83d01d006a635c998b` |

The registered Infinite route staged 17 font faces into an 8,757,072-byte `🔤️guestslim-typst-fonts.bin` with SHA-256 `05c4bbb7d07a3ee0c77274d546a3a4c5942366ccbc82eedad18b4885aa21fc5a`.

## TDD and Validation

The first portable run was red with 4 passes and 1 failure because `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/🟦️.ts` did not exist. The independent overlap probe was red with 2 rows for 1 unique hostile source. The final portable contract is green with the owners, consumers, producer inputs, native/third-party oracles, current root coverage and duplicate traversal control.

| Command | Result |
| --- | --- |
| `bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts'` | 5 passed, 0 failed, 452 expectations; includes Ajv Draft-07 validation, Node Buffer packed-font oracle, installed TypeScript AST export oracle, hostile scans and overlap control |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/repo-lib:test-framework-source-topology --skip-nx-cache` | passed; same 5 tests and 452 expectations through the registered route |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/assets:preview-generated --skip-nx-cache` | passed; 2,632,301-byte canonical plan |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/assets:build --skip-nx-cache` | passed; wrote 286 deterministic outputs |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/assets:check-generated --skip-nx-cache` | passed; all 286 outputs fresh |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/assets:generate-logo --skip-nx-cache` | passed; parsed 6 keyframes, generated 27 frames, output hash unchanged |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-styling-tokens:preview-generated --skip-nx-cache` | passed; 171,988-byte canonical plan |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-styling-tokens:generate --skip-nx-cache` | passed; wrote CSS/TS/C#/Rust/Python outputs |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-styling-tokens:check-generated --skip-nx-cache` | passed; outputs fresh |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-styling-tokens:fonts --skip-nx-cache` | passed; 0 downloaded, all 21 referenced faces present |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-styling-tokens:test --skip-nx-cache` | passed; 2 files and 16 tests after selecting the actual projection test owner |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-styling:test --skip-nx-cache` | passed; 45 tests and 1,440 expectations |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run semio-framework-os-infinite:font-tool --skip-nx-cache` | passed; native Cargo binary compiled and staged |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run semio-framework-os-infinite:fonts --skip-nx-cache` | passed; native tool ran under the router and the moved validator admitted/staged 17 fonts |
| targeted `path-emoji-statutes` tests for logo, asset documentation, SVG identities and Rust bindings | 4 passed, 0 failed, 127 expectations through Bun plus installed-TypeScript compiler oracles |
| direct Bun bundles of all eight owners and all three extracted command entries | passed |
| strict taxonomy validation | 0 problems |
| scoped inventory: assets | 1,843 entries; 0 findings on the new owners; 6 other current findings |
| scoped inventory: styling | 107 entries; 0 findings on the new owners; 11 other current findings |
| scoped inventory: Infinite font owner | 9 entries; 0 findings |

The initial registered styling-token test ran 16 tests but failed the suite because its configuration also selected the browser facade `🟦️.ts`, which now contains no inline tests. The config now selects only `📽️projection/🟦️.ts`; the rerun passed.

## Bounded Limits

A full `path-emoji-statutes` run made 31 tests pass and 6 fail before the asset binding expectation was corrected. The owned asset failure expected `include_str!(\"🖼️icon_svgs/…\")` while the generated source correctly contained `include_str!(\"../🖼️icon_svgs/…\")`; the focused four-test rerun is green. The other five observed failures were: a missing mutation-vector AST definition, normalization returning no `create-camera`/`create-node` vectors, missing current glTF oracle JSON, missing current TSV oracle JSON, and `🔌️disconnect` yielding a test-case identity problem where the fixture expected none. This slice made no baseline claim for those live failures and did not change their owners.

The network acquisition branch was not exercised because all 21 styling-referenced faces were already present. Its `AbortSignal`, exact provider subset mapping, WOFF2 signature check, exclusive write, and progress callback remain in the extracted owner. The MP4 export was not run; it remains behind the registered `logo` target with the existing Playwright/FFmpeg prerequisite, frame progress, and repository tool-cache environment. The deterministic animated-SVG producer was run and verified.

The 6 assets and 11 styling inventory findings are the exact live scoped counts outside this slice’s new owners; no baseline or global-clean claim is made.
