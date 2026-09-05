# Font Review and Hand Repair

The active asset font tree has 219 files: Anta 12, Kelly Slab 9, Share Tech Mono 3, and Noto Emoji 195. Each binary must be preserved. Font family names, CSS font stacks and public behavior must not be changed merely to repair paths.

The installed independent `opentype.js` parser read the actual regular-weight Noto TTF character maps. This is local evidence, not an inference from arbitrary subset numbers.

| Existing subset | Mapped code points | Observed content | Handpicked group proposal |
| --- | ---: | --- | --- |
| 0 | 29 | Regional indicator letters | 🌍️regions |
| 1 | 23 | Flags, rainbow, gender and tag sequences | 🚩️flags |
| 2 | 266 | Arrows, controls, keycaps and symbols | 🔣️symbols |
| 3 | 265 | Tools, clothing, clocks and other objects | 🧰️objects |
| 4 | 119 | Sports, celebrations, games and arts | 🎯️activities |
| 5 | 126 | Buildings, vehicles, travel and places | 🧳️travel |
| 6 | 133 | Food, drinks and tableware | 🍽️food |
| 7 | 225 | Weather, celestial bodies, plants and animals | 🌿️nature |
| 8 | 131 | People, professions and human activity | 🧑️people |
| 9 | 247 | Faces, hands, body parts and expressions | 😀️faces |
| 10 | 34 | Joined directional people and supplementary composite forms | 🔗️joined-forms |
| 11 | 10 | Harp, shovel, bare tree, fingerprint, root vegetable, splatter and tired face | 🪉️supplement |
| emoji | 1,447 | Combined older glyph coverage | 🌐️complete |

Counts include control, variation and combining characters. No release-version claim is made. Proposed weight owners are `🪶️light` (300), `📖️regular` (400), `⚖️medium` (500), `🖋️semibold` (600), and `🏋️bold` (700). Within each exact subset/weight owner, the three encodings can have distinct descriptive names: `🔤️outline.ttf`, `🌐️web.woff`, and `🗜️compressed.woff2`. Family proposals: `🚀️anta`, `🧱️kelly-slab`, `⌨️share-tech-mono`; existing `😀️noto-emoji` remains fitting. These are proposals pending implementation and verification, not completed moves.

Incoming authored font paths are in the UI styling token JSON, generated palette CSS, styling asset-server test, UI WGPU text module, UI render text module, and OS WGPU renderer. The 21 web font-face records explicitly preserve CSS family names. Native TTF paths must be updated together.

The existing font downloader is already inconsistent: its `GOOGLE_FONT_QUERIES` keys begin with `font/...`, while all current committed `fontFaces.src` values begin with `🔤️fonts/...`. Its URL resolver also derives subset IDs from old filenames and contains arbitrary fallback subset choices. Do not run this downloader or download replacements over vendored font bytes. A source-first explicit family/subset mapping with neutral parser tests is needed when paths are repaired; do not infer remote identity from new emojis or basename position.

The canonical styling generator is `🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts`; the TypeScript script delegates to it. Its ordinary artifact generation is distinct from downloading fonts. Canonical asset owner root was already corrected to framework assets by the kernel agent. Detailed cmap output is `🗑️generated/assets-noto-subsets.jsonl`.

## Applied repair

The asset AGENTS instructions have been read. Before any binary move, all 219 font SHA-256 values were captured under `🗑️generated/fonts/before-sha256.json`.

New source-first `🔤️fonts/📇️catalog.json` and its schema explicitly declare the four handpicked families, 21 subsets, five weights, and three encoding filenames. Non-Noto subsets use `🏛️latin`, `➕️latin-ext`, `🧮️math`, `🔣️symbols`, and `🪆️cyrillic`; their only weight is `📖️regular`. All weights have case-owned encoding leaves `🔤️outline.ttf`, `🌐️web.woff`, and `🗜️compressed.woff2`.

The combined older Noto font explicitly declares no remote subset identity. It must remain vendored; neither missing-subset fallback nor substitution with a partial modern subset is permitted. A read-only request to the [Google Fonts Noto Emoji CSS endpoint](https://fonts.googleapis.com/css2?family=Noto+Emoji:wght@400&display=swap) confirmed the provider exposes numbered subset URLs. No font binary was downloaded.

The neutral source-resolution fixture compares our parser with the existing independent PostCSS parser and asserts exact named and numbered subset selection, failure for absent subset 11, no remote replacement for the combined vendored subset, and no WOFF2 replacement for a TTF request. Initial Nx invocation was blocked by a concurrent temporary central taxonomy validation error before test execution; it is not counted as the expected regression failure.

The owned Nx test then failed against the old basename-based resolver (`src.split` cannot admit explicit source metadata). After the resolver change, that neutral test passed and the catalog-completeness test correctly failed before the physical moves. After all manual moves, all 32 styling tests pass with 580 assertions, including independent Ajv validation, all 219 exact binary paths, and PostCSS agreement.

All 219 old/new binary identities are SHA-256 identical, verified from the pre-move evidence. Three family directories and all 219 encoding files were moved individually; no rename script or automatic emoji choice was used. Each old Noto `<subset>-<weight>.<format>` is now the exact catalog-declared subset/weight/encoding path. Each old text `<subset>.<format>` is now the exact subset/`📖️regular`/encoding path. CSS family names, font stacks, glyph data, and weights are unchanged.

All 21 token font-face paths and the asset-server test use the current paths. The two UI Rust text modules and the OS renderer resolve all 31 native font includes; the OS web font fetch URL now points to the same actual Anta asset. Concurrent updates to the two UI text files were preserved after verifying their targets. Styling generation and its canonical freshness check pass. The schema-owned catalog is now an explicit styling generator input, and rendering rejects any font-face path absent from it. The downloader no longer contains old `font/...` path guessing or arbitrary fallback subsets, skips existing vendored files, and supports cancellation/progress for missing-file acquisition. No font downloader command or binary download was executed.

The final physical audit covers 321 entries with zero emoji-statute findings and zero unknown directory roles. Exact family/subset/weight members are registered under their font owners; obsolete generic family member entries were removed. The only old font URLs found outside generated evidence are in the existing stdio plain-text log specimen, which remains byte-preserved as test data.

Canonical and direct WGPU generation/check targets also exited successfully after the neighboring Flow bindings-output repair. Their temporary renderer log directory, then the active font evidence directory and Cargo cache, were removed by an unrelated concurrent workspace process. No such cleanup was performed by this agent or the parent. All completed audit/hash/test outcomes above were observed before that removal; the per-file temporary SHA evidence is no longer present. The native UI target subsequently returned exit 0. The render target returned exit 1, but its diagnostic log had already been removed, so no cause or success is claimed for that rerun. Further verification should preserve active logs and compiler caches until completion.
