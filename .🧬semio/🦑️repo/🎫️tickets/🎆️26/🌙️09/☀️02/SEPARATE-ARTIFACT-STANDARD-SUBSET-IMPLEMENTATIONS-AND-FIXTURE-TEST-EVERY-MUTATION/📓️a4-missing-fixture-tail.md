# 🐚 Shard A4 — the `missing-fixture` long tail

Territory: every `missing-fixture` breach EXCEPT `🗄️stdio/🗿️artifacts/🧿️semio` (A1), `🏗️fem/*` and
`🎬️sequence` (A3), `🗒️note`/`➗️mathematical` (A2). No `🪆️subsets/🔣️.json` edited.

## Counts

| when | repo-wide `missing-fixture` | my territory |
| --- | --- | --- |
| before | 366 | 93 |
| after | 273 | **0** |

Measured with `bun ./📜️script.ts test contract` (foreground), breaches read from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`. No `orphan-fixture` or `fixture-digest-mismatch`
breach anywhere in the current cache touches any of my territory's artifact paths (checked by
substring match against every `🗿️artifacts/<kind>` I own). Repo-wide `orphan-fixture` actually
fell 245 → 235 (files I repointed references onto are no longer orphaned) and
`fixture-digest-mismatch` 70 → 0 (other shards' concurrent work, not mine — I never touched fixture
bytes, only reference text).

The remaining 273 repo-wide `missing-fixture` breaches belong entirely to other shards'
territory (mostly `🗄️stdio/🗿️artifacts/🧿️semio`, shard A1).

## Root cause (one, single, systematic)

**Every one of the 93 breaches in my territory was REFERENCE DRIFT from the same repo-wide event:
the kind-only-basename fixture migration.** Fixture files were renamed from their old flat,
descriptively-named form to a kind-only basename, and — when a location holds more than one fixture
— nested one level deeper into a `🧪️<slug>/` folder so the sibling files don't collide on the same
kind-only name:

- single-fixture location: `<dir>/<slug>.<ext>` → `<dir>/<kind-emoji>.<ext>` (flat rename)
- multi-fixture location: `<dir>/<slug>.<ext>` → `<dir>/🧪️<slug>/<kind-emoji>.<ext>` (renamed + nested)

The `.feature` files (and in ~51 cases the `🦀️.rs`/`🟦️.ts` adapters sitting beside them) were never
updated to follow the rename, so every `asset://`, `shared://` and `local://` URI they declared kept
pointing at a path that no longer exists. Zero genuinely-missing fixtures and zero bogus scenarios
were found in this territory — every single breach was resolvable by finding where the file actually
lives now and repointing the reference.

Two dominant sub-patterns account for most of the volume:

1. **The shared demo DSL** — ~29 breaches across unrelated plugins (`🎬️present`, `🕸️dag`,
   `🎪️playground`, `🖍️draw`, `🔋️model`, `🌊️flow`, `📋️forms`, `🗺️gismap`, `🏔️gisterrain`,
   `📜️imperative`, `📏️layout`, most of `📕️norm/*`, `📖️playbook`, `🧊️process3d`, `🖨️raster`,
   `🔌️wires`, `📸️remodel`, `🎥️shooting`, `🗂️curate`, `🏠️home`, `🪐️space`, `🔌️jack`, `♻️rewrite`,
   `🌿️vcs`, `✒️writer`) each carry their own private copy of
   `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`. Every copy was
   renamed to `🗣️.dsl.semio` in place; every `.feature` still said `🗣️example.dsl.semio`.
2. **The stdio real-world binary fixtures** — `📐️step` (7), `🏗️ifc` (7), `📰xml` (4), `🎞️pptx` (3),
   `🖼️tiff` (3), `🎞️gif` (3), `🧊️gltf` (2), `📷️jpg` (2), plus one each for `📼️avi`, `💾️binary`,
   `🖼️bmp`, `📊️csv`, `🌦️epw`, `🌐️html`, `☁️las`, `🎵️mp3`, `🎥️mp4`, `🧊️obj`, `☁️ply`, `📷️png`,
   `🟪️stl`, `📄txt`, `🔊️wav` — all real-world captures (German building/site photos, an IFC export,
   a `.stp`/`.stl` mesh, a conference recording) that genuinely exist on disk under
   `🧫️fixtures/🧪️<slug>/<kind-emoji>.<ext>`, just not at the flat name the feature still declared.

A special case inside pattern 1: the ten `📕️norm/📘️en19XX` Eurocode artifacts (`en1990`–`en1999`
minus `en1997`, which uses the generic demo) each reference a *named* example
(`📕️high-consequence-office`, `📕️retail-hydrocarbon-fire`, …) whose `🖼️assets/` folder holds
**four** sibling kind-only files (`🗣️.dsl.semio`, `📡️.spr.semio`, `🔧️.op.semio`,
`🎒️.pack.semio`) — so the DSL one specifically also got nested into its own `🧪️<slug>/` folder to
disambiguate from its `.semio`-suffixed siblings. `♻️rewrite`'s `mutate-rewrite-1` case is the other
special case: it holds two named snapshot fixtures, and only the second-referenced one
(`nakagin-ground-floor`) got the nested-folder treatment while the first
(`nakagin-capsule-tower`) stayed flat as the case's sole root-level fixture.

## Method

1. Pulled the live `missing-fixture` breach list from
   `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`, filtered to my territory (93 entries) —
   `🔍️a4-dump-territory.py`.
2. For every breach, parsed the `Fixture <scheme>://<uri> does not resolve` summary and computed
   the absolute path the URI resolves to per the brief's resolution rules (asset:// against the
   case owner, shared:// against `<owner>/🧫️fixtures`, local:// against `<case>/🧫️fixtures`) —
   `🔍️a4-classify.py`.
3. Walked the resolution root for every file sharing the URI's extension, scored candidates by
   whether the containing folder's name (minus its `🧪️` prefix) matches the URI's own slug (minus
   its leading kind-emoji), and took the best match — `🔍️a4-resolve.py`. Every one of the 93 had
   exactly one real file on disk; the only two categories the automatic scorer got wrong were the
   `.semio`-suffixed norm siblings (compound extension, four candidates share the trailing
   `.semio`) and `♻️rewrite`'s flat-vs-nested pair — both fixed with an explicit override checked
   by hand against file content and feature prose.
4. Built the old-URI → new-URI mapping per breach — `🩹️a4-repair-plan.py` — and applied it as a
   literal string replace (both the `scheme://` form and any bare backtick-quoted mention of the
   same path in prose) across every affected `.feature` file and any `🦀️.rs`/`🐍️.py`/`🟦️.ts`
   adapter in the same case directory — `🩹️a4-apply.py`. 81 feature files and 51 adapters were
   touched; 240 literal occurrences repointed.
5. Re-ran `bun ./📜️script.ts test contract` in the foreground and re-measured.

## Per-artifact classification

All 93 = **REFERENCE DRIFT**, repointed. None were GENUINELY ABSENT or BOGUS SCENARIO.

| artifact(s) | breaches | old → new (representative) |
| --- | --- | --- |
| `🎬️present`, `🕸️dag`, `🎪️playground`, `🖍️draw`, `🔋️model`, `🌊️flow`, `📋️forms`, `🗺️gismap`, `🏔️gisterrain`, `📜️imperative`, `📏️layout`, `📗️din16798`, `📙️din18599`, `📕️din4108`, `📘️en1997`, `📓️iso16757`, `📔️vdi3805`, `📖️playbook`, `🧊️process3d`, `🖨️raster`, `🔌️wires`, `📸️remodel`, `🎥️shooting`, `🗂️curate`, `🏠️home`, `🪐️space`, `🔌️jack`, `♻️rewrite`, `🌿️vcs`, `✒️writer` | 1 each (asset demo DSL) | `🖼️assets/🗣️example.dsl.semio` → `🖼️assets/🗣️.dsl.semio` |
| `📘️en1990`…`📘️en1999` (minus en1997) | 1 each | `🖼️assets/🗣️<slug>.dsl.semio` → `🖼️assets/🧪️<slug>/🗣️.dsl.semio` |
| `🎬️present`, `🌊️flow`, `🏔️gisterrain`, `🖨️raster`, `🗂️curate`, `🧱️block/◻2d`, `♻️rewrite` (×2) | 1 local each | `local://<slug>.snapshot.json` → `local://🔣️.snapshot.json` (or nested for the second rewrite fixture) |
| `📐️step` | 7 | `shared://📐️hexagonal-cut-concrete-forest-left-ap214.stp` → `shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp` |
| `🏗️ifc` | 7 (3 distinct files) | `shared://🏗️wellness-center-sama-street-level.ifc` → `shared://🧪️wellness-center-sama-street-level/🏗️.ifc` (+2 more) |
| `📰xml` | 4 | 3 nested (`ooxml-readme-document`, `macos-uttype-plist`, `reuse-marketplaces-plist`), 1 flat (`ooxml-word-document.xml` → root `🏷️.xml`, content-verified) |
| `🎞️pptx` | 3 (1 file) | `shared://🎞️semio-talk.pptx` → `shared://📽️.pptx` |
| `🖼️tiff` | 3 (2 shared + 1 local) | `shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff` → nested; `local://🔄️flipped-scan.rgba` → flat `🖼️.rgba` |
| `🎞️gif` | 3 | 2 nested shared (`dancing-87a`, `dancing-87a-large`) + 1 asset (`💃️dancing` example) |
| `🧊️gltf` | 2 | asset (`🌱️metabolism` example) + local (`base-with-nested-node`), both nested |
| `📷️jpg` | 2 (1 file) | `shared://🖼️abbau-aufbau-masterarbeit-grundriss.jpg` → nested |
| `📼️avi`, `💾️binary`, `🖼️bmp`, `📊️csv`, `🌦️epw`, `🌐️html`, `☁️las`, `🎵️mp3`, `🎥️mp4`, `🧊️obj`, `☁️ply`, `📷️png`, `🟪️stl`, `📄txt`, `🔊️wav` | 1 each | flat or nested rename, content-verified where ambiguous (`📄txt`'s `hub-boot-log.txt` confirmed by reading the renamed file's content) |

## What was produced vs repointed

Nothing was produced — every fixture this territory's scenarios need already exists on disk under
its post-migration kind-only name. All 93 breaches were closed by repointing references, never by
authoring new fixtures.

## Files touched

- `🔍️a4-dump-territory.py`, `🔍️a4-classify.py`, `🔍️a4-resolve.py`, `🩹️a4-repair-plan.py`,
  `🩹️a4-apply.py` — the scripts, kept in the ticket root per house rules.
- 81 `🥒️.feature` files across the artifacts listed above.
- 51 sibling `🦀️.rs`/`🟦️.ts` adapter files in the same case directories that repeated a fixture
  reference.
- No `🪆️subsets/🔣️.json` file was touched (out of scope per the shard brief).

## Left open

Nothing in this territory. The 273 remaining repo-wide `missing-fixture` breaches all belong to
other shards (chiefly A1's `🗄️stdio/🗿️artifacts/🧿️semio`).
