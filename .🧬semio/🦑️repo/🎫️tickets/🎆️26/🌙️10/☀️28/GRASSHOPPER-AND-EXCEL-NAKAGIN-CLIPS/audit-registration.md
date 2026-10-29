# 🔎 Registration Audit: BIM and Optimization Video Series

Read-only audit for the new series `grasshopper_*` (`🎓️teaching/🏛️architecture/🦗️grasshopper/`) and `excel_*` (`🎓️teaching/🏛️architecture/📊️excel/`), modelled on `bim_*` and `optimization_*`. Paths below are relative to the repo root `C:/git/semio` unless marked `T/`. The teaching root is `🎓️teaching/`; `🏛️architecture/` is abbreviated `ARCH/`, `⚡️energy/🏗️authoring/🎬️clip/` is abbreviated `CLIP/`, and `⚡️energy/🎬️clip/` is abbreviated `MANIFEST-DIR/`.

Search scope: `rg` over the whole repo excluding `node_modules`, `__pycache__`, `.git` and `.🧬semio/🦑️repo/🎫️tickets`, for `bim_information`, `optimization_extrema`, `BIM_information`, `Beat1_DataToInformation`, `🏗️bim/🎬️clip`, `🎯️optimization/🎬️clip`, plus the family tokens `bim_`, `optimization_`, `BIM_0`, `Optimization_`, `🪟️presentation`, `🥞️hierarchy`, `📚️collection`.

---

## 1. Anatomy of one series (what BIM and optimization actually consist of)

| Layer | BIM example | Optimization example | Who reads it |
|---|---|---|---|
| Manifest entry (one per video) | `ARCH/⚡️energy/🎬️clip/🔣️.json` `videos[]` | same file | runtime `load_manifest`, render CLI choices, all chapter navigation |
| Per-video entry script | `ARCH/🏗️bim/📊️information/🎬️clip/🐍️.py` (`VIDEO_ID`, `compose`, `main`) | `ARCH/🎯️optimization/📉️extrema/🎬️clip/🐍️.py` | manifest `script`, tests |
| Canonical segment scene | `ARCH/🏗️bim/🎬️clip/📊️information/🐍️.py` (`Beat1_DataToInformation`) | `ARCH/🎯️optimization/🎬️clip/📉️extrema/🐍️.py` (`Beat1_LocalGlobalExtrema`) | manifest `segments[].script`, dependency walk |
| Series root playlist | `ARCH/🏗️bim/🎬️clip/🐍️.py` (`VIDEO_IDS`, 6 ids) | `ARCH/🎯️optimization/🎬️clip/🐍️.py` (`VIDEO_IDS`, 6 ids) | the two `.cjs`/tests, humans |
| Presentation topic entry | `CLIP/🪟️presentation/🔣️.json` `topics[id=bim].videos[]` | same, `topics[id=optimization]` | `_hierarchy.video()`, `lesson_window()`, render |
| Chapter title (en, de) | `CLIP/🪟️presentation/📚️collection/🔣️.json` `chapters` map | same | `collection.enrich()` (runtime, strict) |
| Series test cases | `ARCH/🏗️bim/🎬️clip/🧪️cases/🔣️.json` `chapters[]` | `ARCH/🎯️optimization/🎬️clip/🧪️cases/🔣️.json` `chapters[]` | series tests |
| Series README and transcripts | `🏗️bim/🎬️clip/README.md`, `🗣️transcript.{en,de}.md` | `🎯️optimization/🎬️clip/README.md`, transcripts | humans only (not tested) |

### 1.1 Exact manifest shape (copy source)

File: `ARCH/⚡️energy/🎬️clip/🔣️.json`. Schema: `CLIP/🧬️schema/🔣️.schema.json` (`$schema` `../🏗️authoring/🎬️clip/🧬️schema/🔣️.schema.json`).

Schema constraints: `videos` minItems 1; each video `additionalProperties: false`, required `id`, `script`, `chapters`, `output`; `id` pattern `^[a-z_]+$`; `script` pattern `^🏛️architecture/(?:[^/]+/)+🎬️clip/🐍️\.py$`; segment `script` pattern `^🏛️architecture/(?:[^/]+/)+🐍️\.py$`; `scene` pattern `^[A-Za-z_][A-Za-z_0-9]*$`; chapter requires `scene` and `segments` (minItems 1); segment optional `clauses` with `first` and `last`. Uniqueness of ids and outputs is NOT in the schema; it is enforced by tests (see 2.2).

BIM snippet (copy this shape, the file order is energy → bim_* → optimization_*):

```json
{
  "id": "bim_information",
  "script": "🏛️architecture/🏗️bim/📊️information/🎬️clip/🐍️.py",
  "chapters": [
    {
      "scene": "BIM_01_Information",
      "segments": [
        {
          "script": "🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py",
          "scene": "Beat1_DataToInformation"
        }
      ]
    }
  ],
  "output": "BIM_information"
}
```

Optimization snippet:

```json
{
  "id": "optimization_extrema",
  "script": "🏛️architecture/🎯️optimization/📉️extrema/🎬️clip/🐍️.py",
  "chapters": [
    {
      "scene": "Optimization_Extrema",
      "segments": [
        {
          "script": "🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py",
          "scene": "Beat1_LocalGlobalExtrema"
        }
      ]
    }
  ],
  "output": "Optimization_Extrema"
}
```

Current state: 17 entries (5 energy, 6 `bim_*` at lines 653-748, 6 `optimization_*` at lines 749-842). All 23 referenced script paths exist on disk (checked). Note the naming drift: BIM output is `BIM_information` (matches its cases file), optimization output is `Optimization_Extrema` while its cases file uses `optimization_extrema` (`🎯️optimization/🎬️clip/🧪️cases/🔣️.json` line 27). No test compares these strings.

### 1.2 Per-video entry script (copy source)

`ARCH/🏗️bim/📊️information/🎬️clip/🐍️.py` (31 lines; the optimization one is identical in shape):

```python
_runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
VIDEO_ID = "bim_information"

def compose(*, language: str, chapters: list[str] | None = None, **settings) -> dict[str, Path]:
    return _runtime.compose_videos([VIDEO_ID], language=language, chapter_scenes=chapters, **settings)

def main(argv...) -> ...:   # calls _runtime.video_main(VIDEO_ID, argv)
```

Required: a module-level `VIDEO_ID` equal to the manifest id, and a `compose` with the same keyword contract (both asserted by tests, see 2.2).

### 1.3 Series root playlist (copy source)

`ARCH/🏗️bim/🎬️clip/🐍️.py` (31 lines) and `ARCH/🎯️optimization/🎬️clip/🐍️.py` (31 lines) are identical apart from the tuple:

```python
VIDEO_IDS = ("bim_information", "bim_objects", "bim_representations", "bim_models", "bim_exchange", "bim_consistency")
VIDEO_IDS = ("optimization_extrema", "optimization_sampling", "optimization_experiments", "optimization_index", "optimization_pareto", "optimization_algorithms")
```

It calls `compose_videos(list(VIDEO_IDS), ...)` and `renderer.main([--video id ...])`.

### 1.4 Presentation topic entry (copy source)

`CLIP/🪟️presentation/🔣️.json`, topic `bim` (lines 178-268) and topic `optimization` (lines 296-386). Topic `grasshopper` (lines 270-280) and topic `excel` (lines 283-293) already exist with `"videos": []`, so the new videos go into those arrays. One BIM entry:

```json
{
  "id": "bim_information",
  "title": { "en": "Data and Information", "de": "Daten und Information" },
  "icon": "📊️",
  "symbol": "bim",
  "cell": { "column": 0, "row": 0 }
}
```

Topic-level constraints: the schema fixes `topics` to exactly 6 (`minItems`/`maxItems` 6, `🪟️presentation/🧬️schema.json` line ~146). The topic id enum already contains `grasshopper` and `excel`. The `symbol` enum (line ~394) is `physics, air, heat, cool, balance, bim, optimization`: any new symbol must be added there (grasshopper/excel videos can reuse none of them without a semantic lie). `symbol` is data only; no Python or TS reads it.

### 1.5 Chapter titles (copy source)

`CLIP/🪟️presentation/📚️collection/🔣️.json` `chapters` (lines 125-150 for BIM, 151-172 for optimization), keyed by scene name:

```json
"BIM_01_Information": { "en": "Data and Information", "de": "Daten und Information" },
```

Runtime enforcement: `collection.enrich()` (`📚️collection/🐍️.py` lines 20-47) raises `ValueError("Chapter navigation metadata must match the entire authored curriculum")` unless the set of manifest chapter scenes equals the set of keys in this map exactly. Also `ValueError("Clip navigation must cover every authored lesson exactly once")` unless every manifest video appears exactly once in the presentation topics. So a new chapter without a title, or a new presentation video without a manifest entry, fails at runtime, not only in tests.

### 1.6 Series test cases (copy source)

BIM `ARCH/🏗️bim/🎬️clip/🧪️cases/🔣️.json` `chapters[]` item (line ~908):

```json
{
  "folder": "📊️information",
  "scene": "Beat1_DataToInformation",
  "concepts": ["data", "information"],
  "video_id": "bim_information",
  "entry": "🏛️architecture/🏗️bim/📊️information/🎬️clip/🐍️.py",
  "title": { "en": "Data and Information", "de": "Daten und Information" },
  "output": "BIM_information"
}
```

Optimization `ARCH/🎯️optimization/🎬️clip/🧪️cases/🔣️.json` `chapters[]` item (no `concepts`): `folder`, `scene`, `video_id`, `entry`, `title`, `output`. The file also carries top-level `languages` `["en","de"]` and `audiences` (speaker captions true, spectator false). Grasshopper and Excel series have no cases file yet, so they need a new one plus tests modelled on `🏗️bim/🎬️clip/🧪️tests/🐍️.py` (`test_curriculum_registers_six_standalone_videos`) and `🎯️optimization/🎬️clip/🧪️tests/🐍️.py` (`test_curriculum_registers_six_independent_videos`, `test_entry_points_route_only_their_own_scenes`, `test_playlist_routes_the_six_videos`).

---

## 2. Touchpoints (path, what to add, enforced by tests)

Legend: **Auto** = picked up by code without edit; **Manual** = must be edited; **T-yes** = a test fails if it is missing; **T-no** = no test; **T-silent** = nothing fails if missing.

### 2.1 Required for the new videos to exist and render

| # | Path | Add | Enforced? |
|---|---|---|---|
| 1 | `ARCH/⚡️energy/🎬️clip/🔣️.json` | one `videos[]` entry per new id, appended in curriculum order (BIM after energy, then grasshopper, excel or after bim/opt; the test wants manifest order = `curriculum.ids` order) | T-yes: `CLIP/🧪️tests/🐍️.py::test_curriculum_contract` (ids, chapter and segment counts, and `VIDEO_ID`/`compose` per video). No test validates the live file against the schema; `🏗️bim/🎬️clip/🧪️tests/🟨️.cjs` (lines 64-66) validates only a fixture copy |
| 2 | `CLIP/🧬️schema/🔣️.schema.json` | nothing (video and segment patterns already accept `🏛️architecture/🦗️grasshopper/…/🎬️clip/🐍️.py`). Patterns require `🎬️clip/🐍️.py` at the end of a video script and `🐍️.py` for segments | schema only; the BIM `.cjs` loads it (fixture) |
| 3 | `ARCH/🦗️grasshopper/<topic>/🎬️clip/🐍️.py` and `ARCH/📊️excel/<topic>/🎬️clip/🐍️.py` (one per id) | new entry script, copied from 1.2 with `VIDEO_ID` changed | T-yes: energy `test_curriculum_contract` (`module.VIDEO_ID == video["id"]`, `compose` call); series test (to be written) |
| 4 | `ARCH/🦗️grasshopper/🎬️clip/<topic>/🐍️.py` and `ARCH/📊️excel/🎬️clip/<topic>/🐍️.py` (canonical scenes) | bilingual scene class (`Beat…(Scene)` with `NARRATION` en/de), using literal `importlib.import_module("🎓️teaching….")` for every shared module | T-yes for scene discovery: energy `test_manifest_matches_manim_discovery` (gated by `SEMIO_TEST_MANIM=1`, which `script.ts test` sets). The dependency graph is discovered by parsing literal `import_module` strings (`⚙️runtime/🐍️.py` lines ~385-411), so non-literal imports silently drop out of the fingerprint |
| 5 | `CLIP/🪟️presentation/🔣️.json` | fill `topics[id=grasshopper].videos` (currently `[]`, lines 270-280) and `topics[id=excel].videos` (lines 283-293) with entries as in 1.4; add the new symbols to `symbol` enum | Runtime: `_hierarchy.video()` raises `Unknown teaching video`; `lesson_window()` in `🪟️presentation/🐍️.py` line 258 raises for an unplaced video. T-yes: `CLIP/🪟️presentation/🧪️tests/🐍️.py::test_portable_order_locales_chrome_crop_and_timing` (topic ids, video ids and titles versus `🧪️tests/🔣️.cases.json`) |
| 6 | `CLIP/🪟️presentation/🧬️schema.json` | add new values to the `symbol` enum (line ~394) if new symbols are used | T-no: no test found that validates `🪟️presentation/🔣️.json` against this schema |
| 7 | `CLIP/🪟️presentation/📚️collection/🔣️.json` | `chapters` map: one `{en,de}` title per new chapter scene | Runtime strict (`collection.enrich`). T-yes: `🥞️hierarchy/🧪️tests/🐍️.py::test_portable_coverage…` (`set(metadata["chapters"]) == expected`) |
| 8 | `CLIP/🪟️presentation/📚️collection/🐍️.py` | nothing | - |
| 9 | `CLIP/🪟️presentation/🥞️hierarchy/🖌️render/…`, `🪟️presentation/🐍️.py`, `⚙️runtime/🐍️.py` | nothing: all per-video data is read from the manifest, the presentation topics and the collection map | - |

### 2.2 Test fixtures that hard-code the curriculum (must be edited by hand)

| # | Path | Add | Enforced? |
|---|---|---|---|
| 10 | `CLIP/🧪️tests/🔣️.cases.json` | `curriculum.ids` (append the new ids in manifest order), `chapter_counts` (1 per new video, as for bim/opt), `segment_counts` (`[1]` per new video) | T-yes, loud: `test_curriculum_contract` (`assertEqual` on ids, chapter counts, segment counts, unique ids, no `intro`) |
| 11 | `CLIP/🪟️presentation/🧪️tests/🔣️.cases.json` | `topics[]` entries for grasshopper and excel: `videos: [{id, title:{en,de}}]` (topics already present, `videos: []`) | T-yes: `test_portable_order_locales_chrome_crop_and_timing` (zip over topics and videos, title equality). Also `test_energy_curriculum_has_five_topic_owned_videos…` (energy only) |
| 12 | `CLIP/🪟️presentation/🥞️hierarchy/🧪️tests/🔣️.cases.json` | `topics[]` `videos` as id strings for grasshopper and excel (currently `[]`, cells 0,1 and 1,1) | T-yes: `🥞️hierarchy/🧪️tests/🐍️.py` lines 20-32 (topic ids, cells, locale keys of titles, video titles) |
| 13 | `CLIP/🪟️presentation/🥞️hierarchy/🧪️tests/🧩️chapters/🔣️.cases.json` | (a) `counts` (line 8): `clips` 17→+N, `chapters` 38→+N, `canonical_sources` 117→+N (unique script/scene pairs), `segment_occurrences` 125→+N, `clause_selections` 8→+0 unless clauses are used; (b) `clips[]` entry per video (`topic`, `id`, `chapters` identical to the manifest, same order); (c) `boundaries` (line 842 onward, 39 entries = chapters + 1) with `path`, `left`, `right`, `level`; (d) `course` is `architecture` (unchanged) | T-yes: `🧩️chapters/🐍️.py` `test_portable_coverage_matches_authored_sources_and_locales` (manifest equals `clips` golden, counts, titles) and `test_paths_neighbors_and_boundaries_match_independent_numpy_ancestors` (ordering, depths, boundary levels) |
| 14 | `ARCH/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json` | BIM-only (narration keys per chapter). Grasshopper and excel need an equivalent only if they add a presentation test | T-yes for BIM only |

### 2.3 Series-level code and tests (new series must have its own)

| # | Path | Add | Enforced? |
|---|---|---|---|
| 15 | `ARCH/🦗️grasshopper/🎬️clip/🐍️.py` and `ARCH/📊️excel/🎬️clip/🐍️.py` | series root playlist as in 1.3 with `VIDEO_IDS` for the 5 and 4 ids (in manifest order) | T-yes if a test is written; the BIM equivalent is `test_playlist…` (opt: `test_playlist_routes_the_six_videos`). Nothing enforces its existence |
| 16 | `ARCH/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json` and `ARCH/📊️excel/🎬️clip/🧪️cases/🔣️.json` | `chapters[]` as in 1.6, `languages`, `audiences` | T-no until a series test reads it |
| 17 | `ARCH/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py` and `ARCH/📊️excel/🎬️clip/🧪️tests/🐍️.py` | `test_curriculum_registers_*`, `test_entry_points_route_only_their_own_scenes`, `test_playlist_routes_*` (copy from `🎯️optimization/🎬️clip/🧪️tests/🐍️.py` lines ~1240-1290) | T-silent: they only run if listed in 18 |
| 18 | `ARCH/⚡️energy/🏗️authoring/📜️script.ts` line 18 (the `commands.test` array) | add `"🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🐍️"` and `"…📊️excel.🎬️clip.🧪️tests.🐍️"` (same format as the existing bim/opt entries, all with emoji `.` separators) | **T-silent**: a module absent from the list is never run, and nothing reports it |
| 19 | `ARCH/⚡️energy/🏗️authoring/📜️script.ts` lines 21-24 | only if a series adds a `🟨️.cjs` test: `require(join(…, "🦗️grasshopper", "🎬️clip/🧪️tests/🟨️.cjs"))` (bim and opt are required in the loop on line 23; note `🏗️bim` and `🎯️optimization` only) | **T-silent** (same reason) |
| 20 | `ARCH/🏗️bim/🎬️clip/🧱️model/…`, `🎯️optimization/🎬️clip/🧮️model/…` | not needed for registration. The BIM and optimization domain models are series-specific; grasshopper and excel need their own `🧪️cases` if they test numbers | - |

### 2.4 Documentation and listings

| # | Path | Add | Enforced? |
|---|---|---|---|
| 21 | `ARCH/README.md` (line 12, Part table) | BIM row exists ("BIM videos: six independent lessons…", link to `🏗️bim/🎬️clip/README.md`). **There is no optimization video row at all**, a gap in the precedent. Add one row per new series, e.g. "Grasshopper videos: …" with link `🦗️grasshopper/🎬️clip/README.md`, and "Excel videos: …" with `📊️excel/🎬️clip/README.md` | T-no |
| 22 | `ARCH/README.md` (line 13, Quizzes row; quiz tables lines 24-30) | nothing (quizzes already listed for grasshopper and excel) | - |
| 23 | `🎓️teaching/README.md` | The "Topic tree" lists `🎬️clip` only under `⚡️energy` (lines 26-33). Optional: add `🦗️grasshopper/🎬️clip` and `📊️excel/🎬️clip` lines, since BIM and optimization are not listed either. No video list exists in this file | T-no |
| 24 | `ARCH/🏗️bim/🎬️clip/README.md` and `ARCH/🎯️optimization/🎬️clip/README.md` (the model to copy: table of ids, render commands, test command, EN+DE sections) | create `🦗️grasshopper/🎬️clip/README.md` and `📊️excel/🎬️clip/README.md` | T-no |
| 25 | `…/🗣️transcript.en.md` and `🗣️transcript.de.md` per series folder (BIM: `ARCH/🏗️bim/🎬️clip/`, opt: `ARCH/🎯️optimization/🎬️clip/`) | one transcript pair per new series (`Video: \`<id>\`` headings, as in BIM line 16) | T-no |
| 26 | `ARCH/⚡️energy/🏗️authoring/🎬️clip/README.md` | nothing (generic, does not enumerate series) | - |
| 27 | `ARCH/📚️course/…` (course `🔣️.json`, `📚️course/README.md`, `📦️packages/🟦️typescript/📋️project.json`, `📚️course/🟦️.ts`) | **nothing for videos**: the course site links no videos (grep found no clip or video URL). It loads quizzes only (`🔣️.json` `quizzes` lines 41-51 already include grasshopper and excel; `🟦️.ts` lines 12-35). The project input glob (line ~335) covers quizzes only | - |

### 2.5 Taxonomy, build and ignore files

| # | Path | Add | Enforced? |
|---|---|---|---|
| 28 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | Kinds exist for `teaching-grasshopper` (line 6753), `teaching-excel` (6761), `teaching-bim` (6769), `teaching-optimization` (6777), all with `parentKindIds: [teaching-architecture]`. **No clip kind exists for bim or optimization.** Only `teaching-energy-clip` (line 8763, `🎬️`, `^clip$`, parent `teaching-energy`) and `teaching-authoring-clip` (8755) exist, plus ~140 `teaching-clip-chapter-*` kinds for energy. For new `🎬️clip` directories, follow `teaching-energy-clip`: add `teaching-grasshopper-clip` and `teaching-excel-clip` (emoji `🎬️`, slugPattern `^clip$`, parent the series kind). The course README rule "every new directory is registered in the repository taxonomy" (`ARCH/📚️course/README.md` / `🎓️teaching/README.md` step 1) is therefore **not** followed by the bim/opt precedent | T-no found: no test references these kinds by name. Repo taxonomy is consumed by laws and walkers (for example `🧼️workspace-cleanup/🔍️marker-only-folders`, `🕸️dependencies/🧭️direction`); whether they fail on unregistered teaching folders was not verified |
| 29 | `.gitignore` lines 95-96 | nothing unless new series write `📍️trace/` or `⏱️timing/` inside the source tree; the current patterns are `🎓️teaching/🏛️architecture/⚡️energy/**/…` only, so they would not cover grasshopper or excel. Runtime writes traces and speech to the cache (`⚙️runtime/🐍️.py`, `.🧬semio/🎓️teaching/⚡️cache`), so probably nothing to add | T-no |
| 30 | `🎓️teaching/package.json`, `🎓️teaching/CMakeLists.txt`, `Cargo.toml`, `nx.json`, `📋️project.json` (course), `🔒️dependencies.json`, `📋️build-owners.json`, `ARCH/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml` | nothing: no per-video or per-clip registration (pyproject lists only dependencies and test deps) | - |
| 31 | `🎓️teaching/🏛️architecture/🎬️clip/` (no such dir), `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json` | the BIM model schema is domain-specific and not a registry for videos | - |

---

## 3. How `script.ts test` decides what to run

File `ARCH/⚡️energy/🏗️authoring/📜️script.ts`.

1. Lines 21-25 (only when `command === "test"`): synchronously `require`s three TypeScript or CJS files, which execute their own assertions at load time: `🏗️bim/🎬️clip/🧪️tests/🟨️.cjs`, `🎯️optimization/🎬️clip/🧪️tests/🟨️.cjs`, and `🏗️bim/🎬️clip/🧱️model/🧪️tests/🧬️loss-selector/🟦️.ts`. The loop is over the literal array `["🏗️bim", "🎯️optimization"]`.
2. Line 18: `commands.test` is a literal argument array for `python -m unittest <module …>`. It contains 37 hard-coded dotted module names, of which the bim/optimization ones are 9 (bim: `🎬️clip.🧪️tests.🐍️`, `.🎭️presentation.🐍️`, `🔗️consistency.🧪️tests.🔄️lifecycle`, `🧩️models.🧪️tests.📏️footer`, `🧱️model.🔌️exchange.🧪️tests`; optimization: `🎬️clip.🧬️algorithms.🧪️tests.🧩️widgets`, `.🧬️algorithms.🧪️tests.📈️comparison`, `🎬️clip.🧪️tests`, `🎬️clip.🧪️experiments.🧪️tests.🧊️consumed`). I verified every listed module file exists.
3. Execution (lines 27-33): `uv run --project 📦️packages/🐍️python --python 3.12 --group test python <args>`, `cwd` = workspace root, env `SEMIO_TEST_MANIM=1` (enables the Manim-discovery test), output root under `.🧬semio/🎓️teaching/⚡️cache/tests`.

Conclusion: `test` is an explicit list, not discovery. Nothing in the bim or optimization test modules is found by walking the tree. A new series is **not** picked up automatically; its test modules must be appended to line 18, and its `.cjs` files to lines 21-24.

Automatic vs manual summary:

- Auto (no edit): the render CLI choices (`🖌️render/🐍️.py` lines 30-35, from `load_manifest`), `--chapter` choices, `load_manifest`, chapter navigation, source-dependency fingerprints, the presentation lesson window (once the topic entry exists).
- Loud failure (tests or runtime fail when you forget): manifest without cases ids (`test_curriculum_contract`), manifest without chapter titles (`collection.enrich` raises), presentation video without manifest entry (`collection.enrich` raises), chapter counts and boundaries (`test_paths_neighbors…`), topic titles (presentation test).
- Silent (nothing fails): the new series' own test modules missing from line 18, `.cjs` missing from lines 21-24, the series root playlist, the README and transcripts, the taxonomy kinds.

---

## 4. Speaker and spectator presentation: how it is derived per video

- Audience variants are global, not per video: `CLIP/👥️audience/🔣️.json` has exactly `speaker` (subtitles overlay) and `spectator` (hidden). No per-video entry.
- Frame geometry, timing, locales, chrome, crop and labels are global: `CLIP/🪟️presentation/🔣️.json` top-level keys (`viewport`, `locales`, `identity`, `chrome`, `focus`, `timing`, `labels`) are validated against `🪟️presentation/🧬️schema.json`. Adding a video does not touch them.
- Per-video data the presentation reads: (a) placement in `topics[].videos[]` (`id`, `title`, `icon`, `symbol`, `cell`), resolved by `🥞️hierarchy/🐍️.py::video()`; (b) chapters from the manifest, each given `title` from the collection map and `cell` as `index % 3, index // 3` (`📚️collection/🐍️.py` lines 31-35); (c) the segment sources and narration via `NARRATION` in the scene classes and the literal import graph.
- No separate slides, hierarchy or chapters JSON per video exists. The per-video "chapter" is simply the manifest chapter with the same scene name. The only extra per-video artifacts are the optional clause recordings and the timing/trace files, both generated into the cache at render time.
- Therefore a new video needs exactly: one manifest entry, one entry script (`VIDEO_ID`, `compose`), its canonical scene(s) with bilingual `NARRATION`, one presentation topic entry, one chapter title per scene, and (for tests) the curriculum and hierarchy case entries listed above.

---

## 5. Concrete checklist for the grasshopper and excel series

1. Create entry scripts, canonical scenes, series root playlists, README and transcripts (see 1.2, 1.3, 2.4) under `ARCH/🦗️grasshopper/` and `ARCH/📊️excel/`, following the BIM folder pattern (`<topic>/🎬️clip/🐍️.py` entry and `🎬️clip/<topic>/🐍️.py` scene).
2. Append manifest entries to `ARCH/⚡️energy/🎬️clip/🔣️.json` in an order you also use in `curriculum.ids`.
3. Add chapter titles to `📚️collection/🔣️.json`, topic entries to `🪟️presentation/🔣️.json` (grasshopper and excel topics already exist), new symbols to `🧬️schema.json` `symbol` enum.
4. Update `🪟️presentation/🧪️tests/🔣️.cases.json`, `🥞️hierarchy/🧪️tests/🔣️.cases.json`, `🥞️hierarchy/🧪️tests/🧩️chapters/🔣️.cases.json` (counts, clips, boundaries) and energy `🧪️tests/🔣️.cases.json` (curriculum).
5. Write series test modules and their `🧪️cases`, then add them to `📜️script.ts` line 18 (and `.cjs` to lines 21-24 if any). Without this, nothing runs.
6. Register taxonomy kinds for `🎬️clip` (`teaching-grasshopper-clip`, `teaching-excel-clip`) in `🔣️taxonomy.json`, or document that the precedent does not do this.
7. Add architecture README rows (grasshopper, excel videos; and consider the missing optimization row).

---

## 6. Git status of the touchpoints (snapshot, read-only, `git -c core.quotepath=false status --short --`)

Taken during the audit; other agents edit concurrently.

Clean (no output): `ARCH/⚡️energy/🎬️clip/🔣️.json`; `CLIP/🪟️presentation/` (all of it, including `🧪️tests/🔣️.cases.json`, `📚️collection/`, `🧬️schema.json`, `🥞️hierarchy/` tests and cases); `CLIP/🧪️tests/`; `CLIP/🖌️render/`; `ARCH/⚡️energy/🏗️authoring/📜️script.ts`; `ARCH/README.md`; `🎓️teaching/README.md`; `ARCH/🎯️optimization/🎬️clip/` (entire tree, incl. `🐍️.py`, README and transcripts); `ARCH/📚️course/`; `.gitignore`; `🧰️…/📚️library/🔣️taxonomy.json`; `ARCH/🦗️grasshopper/` and `ARCH/📊️excel/` (only the committed quiz directories exist).

Modified (other agents' work in progress, BIM series):

```
 M ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/✂️section/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/📋️plan/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/📏️sheet/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🔩️detail/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧱️core/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🧱️model/🐍️.py
 M ARCH/🏗️bim/🎬️clip/🧱️model/🔣️.json
 M ARCH/🏗️bim/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json
?? ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🧪️cases.json
?? ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🧪️tests.py
?? ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧪️ortho-remodel/cases.json
?? ARCH/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_cells.py
?? ARCH/🏗️bim/🎬️clip/🧱️model/🧪️tests/🏛️remodel.json
?? ARCH/🏗️bim/🎬️clip/🧱️model/🧪️tests/🏛️remodel.py
```

(`ARCH` = `🎓️teaching/🏛️architecture`, `CLIP` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip`. Snapshot at audit time; the `??` entries of the BIM tree are not part of the registration.)

---

## 7. Findings to act on

1. Silent failure risk: `📜️script.ts` line 18 is a hand-maintained list; a new series' tests are never run unless added. This is the biggest trap.
2. The architecture README has no optimization video row; the BIM precedent is incomplete.
3. The taxonomy has no `🎬️clip` kinds for BIM or optimization, contrary to the repo rule that every new directory is registered. Either follow the rule for the new series only (and flag the precedent), or follow the precedent. Decision needed.
4. Titles for one chapter are copied into up to five files (manifest-collection map, presentation topics, presentation tests, series cases, BIM presentation cases). Edit them together.
5. The `symbol` enum blocks new symbols until `🪟️presentation/🧬️schema.json` is extended.
6. Output naming drift: `Optimization_Extrema` (manifest) versus `optimization_extrema` (cases). Choose one convention for the new series.
7. The bim `.cjs` test validates a fixture, not the live manifest; the live manifest has no schema test. An ajv check of `ARCH/⚡️energy/🎬️clip/🔣️.json` against `CLIP/🧬️schema/🔣️.schema.json` would close this gap (not run here).
