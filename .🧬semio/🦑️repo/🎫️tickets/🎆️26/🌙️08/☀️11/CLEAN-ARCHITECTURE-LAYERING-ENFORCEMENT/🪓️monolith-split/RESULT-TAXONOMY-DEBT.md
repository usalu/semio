# Taxonomy Debt: the 1,141 older unresolved directories

Slug: TAXONOMY-DEBT, prefix `txd`. Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Edited: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (JSON only: 3 new member rows, 1 new fixed-directory contract, 5 names appended to 2 existing rows) plus 64 folder renames with their references (section 4).

## 1. Verdict

The real whole-repo inventory (`inventoryTaxonomy`, same engine as `verify taxonomy`) went from **1,141 to 28** non-dot `directory-kind-unresolved` findings. 1,113 were fixed: 1,049 by taxonomy rows and 64 by renames. The remaining 28 are one ticket folder (out of scope), 9 generated or vendored trees that the exclusion mechanism cannot take without an engine edit, and 18 cases that I list with evidence (section 6). No `directory-kind-ambiguous` and no `fixed-directory-contract-ambiguous` appeared. No non-dot finding is new.

## 2. Before and after, per class

Base: the real inventory of 2026-10-16 15:07 (`tx3-inv-before.json`, 1,141 non-dot findings). Final: real full inventory on 2026-10-16, afternoon, on the live tree, with the taxonomy as I left it (`txd-taxonomy-after.json`, SHA-256 `32eb5bf0caec1c51…`; a peer has edited the live file since, my rows are still in it).

| Class | Before | After | What |
|---|---:|---:|---|
| (a) legitimate kind the taxonomy lacked | 1,049 | 0 | 3 new member rows, 1 fixed-directory contract, 5 names appended to existing rows (section 3) |
| (b) breaks the emoji or naming rules | 64 | 0 | renamed with all references (section 4) |
| (c) vendored or generated tree | 9 | 9 | not excludable by JSON (section 5), listed in section 6 |
| (d) unclear or owner-blocked | 18 | 18 | listed with evidence in section 6 |
| ticket folder under a dot-named parent (out of scope) | 1 | 1 | `✏️s/.🧬semio/…/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` |
| **Total non-dot** | **1,141** | **28** | |

Other measures:

| Measure | Before | After |
|---|---:|---:|
| Real, `directory-kind-unresolved`, whole repo | 3,704 | 2,604 |
| Real, same, dot paths (ticket folders, out of scope) | 2,563 | 2,576 (+13 new peer ticket folders) |
| Real, `directory-kind-ambiguous` / `fixed-directory-contract-ambiguous` | 0 / 0 | 0 / 0 |
| Real, non-dot findings that did not exist before | n/a | 0 |
| Simulator (all 116,131 directories incl. dot), unresolved / ambiguous | 7,628 / 0 | 6,532 / 0 |
| `validateTaxonomy` problems, `loadTaxonomy` | 0, ok (758 kinds) | 0, ok (758 kinds), re-run on the live file after the last peer edit |

## 3. Taxonomy changes (class a)

Sources of truth I used: `matchDirectoryKind` and `semanticMemberPatternEmojiAdmitted` (so a name is only registered if the engine can match it), and the member-row rules in `_semanticDirectoryMemberKindsComment`.

1. `mit-bestand-document-children` (new pattern row, 1,029 directories). Owner `mit-bestand`, the farthest ancestor, so it can only fire where nothing nearer matched.

   ```json
   "mit-bestand-document-children": { "ownerKindIds": ["mit-bestand"], "memberNames": [], "memberSlugPattern": "^[\\p{L}\\p{N}][\\p{L}\\p{N}. -]*$", "source": "registry" }
   ```

   It covers the report asset trees (`🖼️asset/👥️akteur/<flag><CC>/<type-emoji><ID-slug>`, logos, projects, catalog), the compact and interim report folders, the presentation slide-title folders (`🌷️Einführung`, `🟣️Gedanke Schweiz`) and the demonstrator copy of the play site. Letters, digits, dot, space and hyphen only: an underscore stays unresolved on purpose, so the vendored `…_files` export is not swallowed.
2. `energy-bestest-cases` (new exact-name row, 6 directories). Owners `assets`, `examples`, `fixtures`; names `🏛️bestest-600FF`, `🏛️bestest-900FF` (the published ASHRAE 140 case ids keep their capitals).
3. `members-of-members-of-assets` (existing row, +4 names, 4 directories): `🌉️aec.building`, `📏️aec.building`, `🗺️aec.building`, `🧊️aec.building`. A peer renamed the CAD model definitions from `aec.building.structure.*` to `aec.building`; the row still holds five stale names without directories (see section 7).
4. `pdf-standard-versions` (new exact-name row, 2 directories). Owner `standards`, names `4️⃣1.4`, `7️⃣1.7`. A pattern would also have re-kinded the 2 resolved DWG keycap standards (`4️⃣ac1018`), so I registered the two names.
5. `crate-module-children` (existing row, +1 name): `🧩️extension_sdk`.
6. Fixed directory contract `nextest-workspace-metadata` (7 directories), inserted after `nextest-metadata`: path pattern `**/.config`, scope `path-pattern`, authority cargo-nextest. The repository-root `.config` keeps winning by specificity (no wildcard token), which the real scoped runs confirm (no ambiguity). Covers `✏️s/.config`, `🌎️hub/.config`, `🎓️teaching/.config`, `🧰️framework/.config`, two `✏️s/🧑‍💻dev/**/💡️services/.config` and the STEP oracles crate `.config`.

### Safety proof

- Install: the live file's SHA-256 changed under me three times (peer edits). Every install rebuilt from the live file with `txd-tools/install.sh` (JSON round trip asserted byte-identical first), validated the candidate, simulated it over all directories, re-checked the hash, then did an atomic `mv`. Install 1 (batch A): `7f560565…` to `7c166733…`. Install 2 (batch C): `65d993ae…` to `32eb5bf0…`.
- Simulator over all 116k directories: **0 ambiguities**; previously resolved directories whose kind id changed: **5**.
  - 4 are my cascade: the `🕹️interactions` children of the four newly resolved `aec.building` directories. They used to resolve through the ancestor fallback (`members-of-members-of-assets`) and now match their siblings (`members-of-members-of-members-of-assets`).
  - 1 is a peer edit (`📇️directory/🔌️client/🧪️tests/🧶️companion`, `members-of-tests` to `crate-module-children`, because a peer added `🧶️companion` to that row before my first install; verified in `txd-live-b1.json`).
- Scoped real inventories, each before and after: `♻️mit-bestand` 1,035 to 6 `directory-kind-unresolved`, `✏️s/🔌️plugins/🔋️energy` 7 to 1, `✏️s/🔌️plugins/📐️cad` 4 to 0, `🎓️teaching` `.config` gone. **0 rows added** in all of them. The 6, 1 and 0 that remain there were treated in section 4 or listed.
- Map-adjusted whole-repo diff of all violation rows (old paths mapped to their renamed paths): 1,121 rows removed (1,113 directory-kind plus 8 file-level), 469 added, none inside my areas. The additions are peer files (`semantic-stem-unresolved` 239, `kind-only-basename` 155, `package-implementation-file` 41, 13 new ticket folders, `path-too-long` 8, `tracked-path-missing` 8 for peer-moved tracked files, `fixed-source-disposition-unresolved` 4, `package-role-unresolved` 1).

## 4. Renames (class b)

All new names are `<RGI emoji>️<kebab>` and every full path stays far below 240 bytes (largest measured 235 bytes, STEP fixtures). Every sibling emoji is unique. References were found with `git grep -F` (outside ticket folders) and replaced in the same step by `txd-tools/rename.py`, which refuses an existing target.

| Old | New | References updated |
|---|---|---|
| 4 × `…/🖼️asset/👥️akteur/{🇨🇭CH/👨‍💻️S01-digvis, 🇳🇱NL/🐦‍⬛️U14-kraaijvanger}` in `📋️zwischenbericht` and `📑️forschungsbericht` (ZWJ emoji with a trailing U+FE0F is not RGI) | same without the trailing U+FE0F | `📋️bericht/📎️anhang/🕸️akteursnetz-figuren.tex`, `📇️akteursnetz-erweiterung-kandidaten.md` |
| `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1` | `🧑‍🏭service-v1` | `🌎️hub/Cargo.toml` (member and dependency path), `📋️build-owners.json`, 1 framework fixture JSON |
| `…/🌊️flow/🕸️wasm/🧪️tests/🧫️fixtures/🧑‍🤝‍🧑️browser-runtime` | `🧑‍🤝‍🧑browser-runtime` | 6 files (2 Rust `include_str!`, 1 JS, 3 TS) |
| `♻️mit-bestand/🧺️demonstrator/public`, `🏢️semio-tech/🎡️play/public` (no emoji) | `🌐️public` | the 2 `publicDir` lines in the two `🏗️builder/🌐️vite/🟦️.ts` |
| `…/🗓️schedule/🧪️tests/🧰️support/weekly-day-slots` | `🗓️weekly-day-slots` | none |
| `🖱️ui/🎯️targets/⚛️react/🪟️chrome/responsive-label` | `🏷️responsive-label` | `⚛️react/🟦️.tsx`, `🪟️chrome/🟦️.ts` (one export path each) |
| `🖍️draw/…/🎮️commands/⌫️delete-selection` (⌫ not RGI) | `✂️delete-selection` | `#[path]` in `🎮️commands/🦀️.rs`, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (2 + 2 occurrences) |
| `🖍️draw/…/🎯️points/🧪️tests/🧫️fixtures/▧️marquee` (▧ not RGI) | `🔲️marquee` | a Rust and a TS test |
| `✒️writer/…/☑️options/⇥️tab` (⇥ not RGI) | `🗂️tab` | `#[path]` in `✒️writer/🦀️.rs` |
| `🧩️puzzle/…/🧬️mutations/⚑️flag` in `🧪️tests/🧫️fixtures` and in `🧬️schema` (⚑ not RGI) | `🚩️flag` | 6 Rust/TS files, `.vscode/launch.json` (16) and the seed (17) |
| `🗄️stdio/…/🧿️semio/…/🌐change-run-language/🇩retags` ×2 (lone regional indicator) | `🏷️retags` | 4 files (oracles JSON, feature file, 2 Rust) |
| `📚️library/⚖️laws/indexed-generated-output` | `📇️indexed-generated-output` | the import in `🎛️commands/✅️verify/🟦️.ts` |
| stdio `🧪️tests/🧫️fixtures/🧬️history-edits/{doctype,target,attribute}` (html), `{header,field}` (csv), `{inlines,replace,insert,remove}` (md), `header` (deflate), `{boolean}` (json base), `{rename}` (i-json), `title` (mp4), `view` ×3 (ifc), `version-info` ×2 (dwg) = 18 | `📜️doctype`, `🎯️target`, `🔖️attribute`, `🧾️header`, `✏️field`, `✏️inlines`, `🔁️replace`, `➕️insert`, `➖️remove`, `🧾️header`, `🔘️boolean`, `🏷️rename`, `🏷️title`, `👁️view`, `🔢️version-info` | the 8 `🎛️history-inputs/🟦️.ts` tests (row ids and path strings) |
| stdio STEP `…/1️⃣cc1…6️⃣cc6/🧪️tests/🧫️fixtures/🎛️history-inputs/{remove-shape-representation, restore-entities, set-file-schema, set-product-identity, demote-shape-representation, set-shape-representation}` = 28 | `🗑️…`, `♻️restore-entities`, `🏷️…`, `🪪️…`, `⬇️…`, `🪜️…` | the one literal in `📐️step/…/🧪️tests/🎛️history-inputs/🟦️.ts` (the Rust tests iterate with `read_dir`, no names) |

Count per row, in table order: 4 + 1 + 1 + 2 + 1 + 1 + 1 + 1 + 1 + 2 + 2 + 1 + 18 + 28 = 64 directories (the two `⚑️flag` and the two `🇩retags` directories are counted as 2 each).

Test path repair I had to include: seven of the eight stdio `history-inputs` tests (html, csv, md, deflate, json, ifc, dwg) read `🧫️fixtures/…` relative to `✳️any` although the fixtures already sit under `🧪️tests/🧫️fixtures`. Baseline proof: the html test failed at `existsSync` before my change and passed with only the `🧪️tests/` fix (probe in section 8). I added `🧪️tests/` to the fixture path in those lines so the renames could be verified.

## 5. Exclusion mechanism (class c)

`pathExclusions` plus `areaEnforcement.opaquePathExclusionIds` exist, but both loaders pin them to exactly three ids (`compose`, `temp-compose`, `mit-bestand-recherche`: `OPAQUE_PATH_EXCLUSIONS` in `🔍️discovery/🟦️.ts` ~l.5230, `TAXONOMY_OPAQUE_PATH_EXCLUSIONS` in `🧹️normalization/🔣️taxonomy/🟦️.ts` ~l.1147). A fourth entry makes `validateTaxonomy` fail. Excluding generated or vendored trees therefore needs an engine edit, which was outside this JSON-only task. I did not touch it.

## 6. What remains (28)

Class (c), generated or vendored, all tracked in git (checked with `git ls-files`):

| Path | Evidence |
|---|---|
| `extension-host-build-7a5OFB` (repo root) | 3 tracked files plus `out/`; an esbuild oracle temp tree (`oracle.cjs` names itself), created 2026-10-13 17:32 |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/same` | 2 tracked files `CACHEDIR.TAG`, `.rustc_info.json` and `debug/`: a Cargo target directory named `same` |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/{command-text-transfer, command-transport-neutral, retained-spr-command-context}` | one 3-byte `input` file each; the Rust tests join those names onto `SEMIO_TEST_ARTIFACT_DIR`, so the env var pointed at the package directory once |
| `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/stdio-composition-test` | a 3 MB built `🟦️.js` and its `.d.ts`; 0 references |
| `🌎️hub/📦️packages/🦀️rust/.semio/hub/db` and `…/db/catalog` | `root.bin` (10 bytes, 2026-08-17) beneath the `**/.semio` fixed contract; runtime database state |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🧰️zukunft-bau-entwerfen-mit-bestand_files` | the vendored presentation export; the existing row still lists the stale name `📁️zukunft-bau-entwerfen-mit-bestand_files`. The underscore keeps it out of the mit-bestand pattern on purpose |

Recommendation, not done: `git rm` the debris and add ignore rules, or add exclusion ids together with the engine change.

Class (d), with evidence:

| Path | Evidence and reason |
|---|---|
| `✏️s/🔌️plugins/🏙️bim/…/📏️authored/▭️profile` | ▭ is not RGI; `#[path = "▭️profile/🦀️.rs"]` is the only reference. Area owned by `pfa`, not renamed |
| `✏️s/🔌️plugins/📕️norm/…/🦀️rust/bins` | no emoji; one file `regenerate_example_assets.rs`, 2 referencing files. Area owned by `pfa`, not renamed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/{1standard, 2standard}` | 14 and 7 files, 9 and 7 referencing files (Cargo, `include_str!`, fixtures). `🔌️plugin/**` is owned by `pi1` |
| `…/💻️os/🔨️modules/🏪️store/🧷️assembly/⏳️lifetime/🧪️tests/🧫️fixtures/🧑️client/{escaping-return, explicit-release, implicit-scope, valid-order}` | case id equals the folder name and the id sits in a fixture JSON; the os store belongs to the kernel carve (`okwp8`). Folder and id need decoupling first |
| `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🧫️fixtures/🗺️vector-tiles/{0-0-0, 1-1-0, 2-2-1, 3-4-2, 4-8-5, 5-17-11}` | z-x-y tile keys: the corpus JSON (`"key"`, `"file": "0-0-0/🛰️.pbf"`), the Rust keys array and 3 `include_bytes!` in the GIS plugin all address the folder by its key |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/import-edges/{a, a/deep, b}` | module names of the import-edge fixture graph; `project.json` globs and 10 files reference `import-edges/**` and the context JSON describes `a`/`b` by name |
| `✏️s/🔌️plugins/🗄️stdio/🗟️artifacts` (🗟 not RGI) | a stale partial duplicate of `🗿️artifacts/📰️xml`: one `🧭️document-boundaries/🔣️.json` (1,316 bytes) that is an older, smaller version of the file in `🗿️artifacts/📰️xml/…/🧱️base/🧪️tests/🧫️fixtures/🧭️document-boundaries/🔣️.json` (2,145 bytes, the one the Rust tests `include_str!`). 0 references to the stray path. Looks safe to delete, but it is a content deletion, so I left it |

Out of scope: `✏️s/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` (ticket folder).

## 7. Open points

- Stale registry names, not removed because a peer owns that move: `members-of-members-of-assets` still lists `🌉️aec.building.structure.classic`, `🏛️aec.building.structure.classic`, `📏️aec.building.structure.fem.line`, `🗺️aec.building.structure.fem.surface`, `🧊️aec.building.structure.fem.solid` (no directories); `members-of-members-of-members-of-members-of-mit-bestand` lists `📁️zukunft-bau-entwerfen-mit-bestand_files` (the folder is `🧰️…`).
- `pathEmojiStatuteFindings` reports nothing for non-RGI base emoji (⌫ ▧ ⇥ ⚑ 🗟 🇩) or for ZWJ sequences with a trailing U+FE0F; it only reports "missing" (83 directories). I found the 12 cases through `semanticMemberPatternEmojiAdmitted`. A statute check against the same admission function would catch them in `verify taxonomy`.
- `📐️step/🧪️tests/🎛️history-inputs/🟦️.ts` and the six `…/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` still read `🧫️fixtures/🎛️history-inputs` without `🧪️tests/` (a peer edited the TS today at 07:20). I only changed the renamed literal there.
- The 15 puzzle snapshot-test failures and the JSON and I-JSON test failure at its leaf-schema line are not caused by my renames (section 8).
- The 8 `tracked-path-missing` rows in the final inventory are peer-moved tracked files, none inside my areas.
- `responsive-label` sits in `🖱️ui/🎯️targets/⚛️react`, next to the renderer-UI work (`rhu2`); I edited only the two export path tokens.

## 8. Commands run and results

- Directory lists: `git ls-files --cached --others --exclude-standard` filtered to existing paths (116,100 to 116,131 directories).
- `bun txd-tools/sim.ts <taxonomy> <dirs> <out>`: baseline 7,628 unresolved / 0 ambiguous; candidate 1 6,593; candidate 2 6,578 (live at that time 6,581); final 6,532.
- `bun txd-tools/validate.ts` on each candidate and on the live file: 0 problems, `loadTaxonomy` ok, 758 kinds.
- `bun txd-tools/inv2.ts` (real `inventoryTaxonomy`, optional scope and taxonomy path): scoped `🎓️teaching` (30 s), `♻️mit-bestand`, `🔋️energy`, `📐️cad` before and after; full repo once after all changes (`entries=281564 violations=63162`).
- `python3 txd-tools/rename.py <spec> [dry]` (dry run first for each batch), `txd-tools/stdio-edits.py [dry]`, plus the 28 STEP moves and the two `publicDir` edits inline.
- Path checks: hub workspace member path exists and the old name is gone from `🌎️hub/Cargo.toml`; 10 `#[path]`, `include_str!`, `include_bytes!` and Cargo `path` tokens containing a new name resolve (0 missing); the `include_str!` targets of the browser-runtime fixture exist; every TS/JSON path token that carries a new name resolves except the unrelated pre-existing `🚩️flags` font paths (those match the substring of `🚩️flag`, they are not mine); `.vscode/launch.json` still parses as JSON and neither launch file contains an old name.
- **Cargo: not run.** `bun repo cargo --store=shared --configuration=prototype -- metadata --no-deps --offline --format-version 1 --manifest-path 🌎️hub/Cargo.toml` stops in the launcher with `Exact canonical persistent Native publication required` (`inspectPersistentReceiver`), the infrastructure fault that `RESULT-INFRA-UNBLOCK-3.md` documents ("LAUNCHER REACHES CARGO: no"). So no `cargo check` or `cargo test` ran for the crates touched by the hub, flow, draw, writer, puzzle and stdio renames; only the static path checks above did. Run `check -p semio-hub-artifact-creation-service`, the flow `semio-…-wasm` tests, and the draw, writer, puzzle and stdio-semio Rust tests once the launcher works.
- `bun test` (per file, cwd = test dir): stdio html, csv, md, mp4, ifc, dwg and deflate `history-inputs`: **1 pass / 0 fail each**. JSON/I-JSON `history-inputs`: 1 fail at its leaf-schema assertion (line 18, before any fixture path), unrelated to my change; a probe copy with only that assertion disabled passes, which shows the renamed fixtures resolve. html baseline proof: before the path fix `expect(existsSync(path))` failed at line 31; the probe with only the `🧪️tests/` fix passed. Puzzle sqlite snapshot test: 48 pass / 15 fail, 11 of the failures are `ENOENT …🧬️mutations/🪢️connect-handles/🎮️prepare/🔣️.json` and none mention `flag`; I did not baseline this file before editing. Draw interaction unit test: 8 pass / 1 fail (`Select All`, a source-text check), the marquee fixture import test passes; also not baselined. `🧷️member-slug-pattern` 5 pass / 0 fail. `🔤️taxonomy-leading-grapheme` 9 pass / 1 fail (`ENOENT` for a schema file under `📚️library/🧬️schema`, unrelated).
- I added no `[DEBUG]` logs and left no probe or temp files in the repo.

## 9. Artifacts (`🗑️generated/`, not deleted)

- Tools in `txd-tools/`: `sim.ts`, `inv2.ts`, `validate.ts`, `emoji2.ts`, `zwj.ts`, `build1.py`, `build2.py`, `install.sh`, `rename.py`, `stdio-edits.py`, `cmp.py`, `vdiff.py` (the rest are the copied tx3 tools).
- Data: `txd-taxonomy-before.json`, `txd-taxonomy-after.json`, `txd-live-b1.json`, `txd-live-b2.json`, `txd-real-nd.json` (the 1,141), `txd-final-remaining.json` (the 28), `txd-sim-*.json`, `txd-inv-after.json`, `txd-scope-*.json`, `txd-b1-spec.json`, `txd-b3-spec.json`, `txd-b4-spec.json`, the directory and file lists `txd-dirs.txt`, `txd-files-*.txt`, and the logs `txd-inv-after.log`, `txd-scope-*.log`, `txd-hub-metadata.err`.
- To repeat the check after more peer folders appear: refresh `txd-dirs.txt`, run `txd-tools/sim.ts` on the live taxonomy (about 10 s), and use `txd-tools/inv2.ts <out> <scope> <taxonomy>` for a real run (30 s scoped, about 12 minutes whole repo).
