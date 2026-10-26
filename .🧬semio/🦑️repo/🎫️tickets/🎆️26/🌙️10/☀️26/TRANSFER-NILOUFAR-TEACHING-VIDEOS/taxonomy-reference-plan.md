# Teaching Clip Taxonomy Reference Plan

Planning only. No product, source, taxonomy or index mutations were made for this report. The existing transfer ticket was reopened by the parent. There are no live render jobs. File inspection used filesystem reads while the parent rebuilt the required repo command interface; no shell command bypassed that interface.

## Ownership Decision

Adopt the architecture-level authoring proposal in [taxonomy-audit.md](taxonomy-audit.md). Register `🎓️teaching/🏛️architecture/🏗️authoring` with explicit, parent-aware member rosters. Keep the five authored lessons beneath the existing energy topics, institutional intro and curriculum descriptor beneath energy, and implementation mechanisms beneath authoring. The current framework module inventory contains process, tool-run, typeset, canvas and document owners, but no animation/presentation owner. Moving this Manim authoring implementation into an invented framework owner would broaden the rename into an extraction. Its curriculum loading, narration timing and Manim conventions make teaching authoring the coherent current owner.

The owner structure supports later language implementations through kind-only implementation leaves. It does not require old filename aliases or a second layout.

| Current teaching-relative file | Proposed teaching-relative target |
| --- | --- |
| `video_runtime.py` | `🏛️architecture/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py` |
| `render_videos.py` | `🏛️architecture/🏗️authoring/🎬️clip/🖌️render/🐍️.py` |
| `manim_fonts.py` | `🏛️architecture/🏗️authoring/🔤️fonts/🐍️.py` |
| `manim_visuals.py` | `🏛️architecture/🏗️authoring/🎨️visuals/🐍️.py` |
| `tts_pipeline.py` | `🏛️architecture/🏗️authoring/🔊️speech/🐍️.py` |
| `test_video_runtime.py` | `🏛️architecture/🏗️authoring/🎬️clip/🧪️tests/🐍️.py` |
| `video_runtime.tests.json` | `🏛️architecture/🏗️authoring/🎬️clip/🧪️tests/🔣️.cases.json` |
| `test_manim_fonts.py` | `🏛️architecture/🏗️authoring/🔤️fonts/🧪️tests/🐍️.py` |
| `glyph_fallback.tests.json` | `🏛️architecture/🏗️authoring/🔤️fonts/🧪️tests/🔣️.cases.json` |
| `video_manifest.schema.json` | `🏛️architecture/🏗️authoring/🎬️clip/🧬️schema/🔣️.schema.json` |
| `video_manifest.json` | `🏛️architecture/⚡️energy/🎬️clip/🔣️.json` |
| `intro/intro_scene.py` | `🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py` |
| `intro/welfenschloss (1).pdf` | `🏛️architecture/⚡️energy/🎬️clip/👋️intro/🏰️institution/📕️.pdf` |

The shared and content directories in this table are proposed registrations, not an assertion that they are already admitted. The authoritative source is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, schema version 7. The low audit covers exact kind rules, package filename exceptions and the complete content-owner mapping.

The five topic mappings are physics, ventilation, heating, cooling and demand for Physical Fundamentals, Ventilation, Heating, Cooling and Energy Balance respectively. Each topic's `🎬️clip/🐍️.py` owns its full wrapper. Each registered named chapter concern owns its scene `🐍️.py`; `🏗️composition/🐍️.py`, `🔊️speech/🐍️.py`, `⏱️timing/🔣️.json` and `📍️trace/🔣️.json` are distinct facets where those original files exist. Directory numbering does not carry playlist order; the descriptor does.

## Python Loading Contract

Kind-only leaves mean that every Python file is named `🐍️.py`. Such a filename cannot appear in a Python `import` statement. Do not create `manim_fonts`, `video_runtime`, `tts_pipeline` or other old-name aliases in `sys.modules`, ASCII shim files, symlinks or compatibility folders.

Use the full canonical taxonomy module coordinate with `importlib.import_module`, with the recognized package root on the import path. Python namespace packages allow the semantic directory hierarchy to carry module identity without adding nonconforming `__init__.py` files. Alternatively, an owned path loader can use `spec_from_file_location` with the canonical full coordinate; it must cache that exact coordinate before executing the module. Both approaches require focused validation against the actual active Python and Manim loader before the move is accepted. Direct syntax imports cannot be mechanically renamed to emoji syntax.

The full coordinate is essential: loading every file under its basename `🐍️` would collapse unrelated modules. The fonts/visuals dependency is cyclic through imports inside functions; caching canonical modules consistently prevents duplicate font setup and duplicate timing globals. The five full wrappers currently use synthetic names such as `ventilation_full_p1`; replace those with canonical source coordinates. The provenance test currently resolves source classes through `sys.modules[beat.__module__].__file__`, so canonical keys must remain registered there.

Retain existing ASCII Scene class names, playlist symbols, video IDs and logical output IDs. They are domain identifiers, not filenames. The runtime deliberately requires ASCII Python identifiers for Scene classes, and the manifest schema has corresponding patterns. The authored beat lists and order stay unchanged: 26 chapters, 112 beat segments, five intros, 117 global raw segments, 31 logical clips and five final movies. Global first ordinals remain 1, 10, 33, 69 and 91 even when selecting one video.

## Root And Path Closure

| Consumer | Existing assumption | Required change |
| --- | --- | --- |
| `video_runtime.py:24–36` | Search ancestors for literal `manim_fonts.py`; read root `video_manifest.json`. | Use the recognized teaching package boundary and canonical descriptor coordinate. A package marker plus its registered architecture child is unambiguous; `package.json` alone is unsuitable because both repository and teaching boundaries exist. No fallback to the old marker. |
| Scene and intro bootstrap | Ancestor search for `manim_fonts.py`, insert old teaching root, bare helper imports. | Resolve the same package boundary and import canonical helpers. Move only bootstrap/path expressions; retain visual construction and timing behavior. |
| Five full wrappers | Local numbered chapter folders, `intro/intro_scene.py`, root font marker, old synthetic module names. | Load the exact registered chapter and intro coordinates from the frozen map. Keep wrapper playlist class identity and order. Remove stale `_TUTORIAL_ROOT`/`_SEMIO_ROOT` assumptions rather than repurposing them ambiguously. |
| Runtime `segment_state` | Root `manim_fonts.py`, root `manim_visuals.py`, optional root `fonts`, asset recursion below script parent. | Fingerprint canonical helper and asset coordinates. Keep chapter timing/trace facets under their scene concern so dependency recursion remains local. Explicitly include institutional PDF beneath the intro concern. |
| Intro `intro_scene.py:49` | PDF is next to script under `welfenschloss (1).pdf`. | Point at `🏰️institution/📕️.pdf`; preserve bytes and vector parsing behavior. |
| Fonts helper `:40` | Default cache beneath helper file's parent. | Keep cache outside governed source owners using existing output/cache environment contracts and a recognized package cache fallback. Moving the helper must not put generated files under authoring. |
| Runtime `video_main` | `from render_videos import main`. | Import canonical render module. The render module likewise imports canonical runtime. |
| Native `📜️script.ts` | `import.meta.dir` is teaching root; relative cache `../.🧬semio/...`; root `render_videos.py`. | Retain this fixed launcher at the package root, updating only its canonical implementation paths. If its owner changes, calculate package/cache roots explicitly. |
| Section speech/composition tools | Bare `tts_pipeline`/`video_runtime`, root marker and section-local output defaults. | Import canonical implementation; use the scene concern as base when tool script moves into a facet. Do not let `Path(__file__).parent` silently become the speech/composition facet instead of chapter owner. |

Inspection of the transferred scene/tool tree found 110 textual `manim_fonts` references, 37 `manim_visuals`, 51 `tts_pipeline`, 35 `video_runtime` and 180 `sys.path` references. These are search counts, including repeated bootstrap checks and comments, not a replacement specification. The frozen per-file map and syntax-aware review must determine each edit.

The current runtime accepts teaching-relative POSIX script paths, resolves them, verifies containment and checks existence. Preserve that safety contract with Unicode paths; do not normalize away variation selectors or replace taxonomy punctuation. `Path.as_posix()` remains necessary for Windows FFmpeg concat lists.

## Timing And Trace Closure

Scene modules call `load_vo_timing` with `Path(__file__).parent / "vo_timing.json"`. Moving timing JSON to `⏱️timing/🔣️.json` requires updating every such expression, even for scenes with no committed timing file. A missing file intentionally selects estimates; accidentally losing an existing file would silently change authored duration.

`manim_visuals.py:1971` accepts a timing path explicitly, so its parser needs no format change. Its trace writer at line 2151 derives `vo_trace.json` from the loaded scene module's `__file__`; update that destination to the registered trace facet. Ensure the destination parent exists for opt-in trace generation. Trace output should honor the explicit generated output root where configured, rather than create validation artifacts in source owners. Speech tools' timing and trace constants must select the same coordinates as scene readers and trace producers. Preserve JSON content and exact clause keys.

`tts_pipeline.py:34` uses its own parent for `.env`. Moving speech implementation changes that meaning; resolve the existing package-level optional environment contract explicitly. `build_section_video` hashes its base directory relative to teaching root and defaults output beneath that base. A composition facet move must pass the chapter scene owner, not the composition directory, and explicit output routing must remain ticket-scoped for validation.

## Descriptor And Schema Closure

The descriptor's `$schema` relative reference must point from the energy clip descriptor to the authoring clip schema. Update every video wrapper path, intro script and 112 ordered segment script values together. The schema's current video-script pattern `^energy/demand/[^.]+\\.py$` rejects the proposed architecture paths and must be replaced by the canonical path contract. Segment scripts currently allow Unicode but must now constrain the canonical `🐍️.py` leaf and normalized forward slashes. Retain identifier constraints for Scene/playlist IDs and strict object keys.

There is no need for a legacy descriptor branch or an old-path resolver. The current language-neutral case table remains the source for cardinalities, segment ordering, first ordinals, cancellation, provenance, encoding and composition checks; rename its path, not those expected behaviors. A rename changes absolute source keys in `{settings,sources,segment}` provenance, so old render metadata is not reusable automatically. Do not rewrite historical evidence to pretend a visual render used new source paths. If byte-identical artifacts are reused for rename-only validation, record that limited proof separately with a checked one-to-one content/path map; any changed visual helper, scene timing or PDF invalidates that shortcut.

## Configuration Filenames

Keep `pyproject.toml`, `uv.lock`, `README.md` and root `📜️script.ts` only under their admitted exact external/package contracts. They are not ordinary semantic basenames. In particular, cosmetically renaming `pyproject.toml` breaks `uv run --project <root>` discovery. The Python package boundary must be recognized in taxonomy before claiming this exception.

There are 27 source `manim.cfg` files: intro plus 26 chapter owners. They contain only preview/quality/media defaults (intro medium quality, chapters low quality). Product render commands already provide an explicit generated config and explicit requested quality, dimensions and fps. Remove these redundant source discovery configs in the frozen move rather than inventing an unregistered `.cfg` kind. Preserve user-facing quality defaults in the owned CLI contract. Generated `render.cfg` stays an internal ephemeral interface artifact, explicitly passed with `--config_file`; it is not an authored source path to rename. Its short `v/p/i/t/x` cache paths keep Windows generated paths below 256 code units.

## Test And Command Closure

`test_video_runtime.py` and `test_manim_fonts.py` currently assume their parent is teaching root, prepend it to `sys.path`, import old helper names and load adjacent old-named cases. Update to canonical package/root resolution and sibling `🔣️.cases.json`. Runtime tests also hardcode intro path, heating wrapper path, render CLI path, temporary old root helpers and source fixture paths. Update fixtures to the single new contract; do not retain old path cases as accepted inputs.

The native launcher currently uses `python -m unittest discover -s <root> -p test_*.py`. It cannot discover renamed kind-only leaves. Enumerate the canonical test module coordinates explicitly through unittest, or use an owned suite that loads those coordinates; do not restore test-prefixed filenames. Keep its test dependency group, UTF-8 environment, no-bytecode behavior, external tool interface and process-tree cancellation intact.

Meaningful language-neutral cases to add before implementation:

1. A canonical Unicode module coordinate resolves two different `🐍️.py` leaves without collisions, preserves `__module__` and caches one fonts/visuals instance across wrapper imports.
2. A moved heating wrapper resolves the recognized teaching package root, not a nearer script/React manifest boundary, and starts at global segment ordinal 33.
3. Canonical descriptor paths and `$schema` resolve inside teaching; old plain `energy/demand` and old helper paths are rejected or absent.
4. Existing moved timing JSON is loaded with identical clause budgets; intro resolves a byte-identical PDF at its canonical facet.
5. The native test entry runs both canonical test modules and the renderer entry runs `--help`/invalid Unicode input successfully under cp1252 as before.
6. A short real Manim render from a Unicode kind-only scene path produces the same ordered frame colors/count as the existing tiny segment integration oracle. This validates our loader/path logic using the actual third-party framework.
7. Manifest AST comparison against the original authored playlists still gives all 112 source/class pairs in the same order, and actual Manim discovery finds all 117 requested classes.
8. The entire frozen move map, generated source/cache paths and native command arguments stay within 256 UTF-16 code units on Windows, including longest Scene name and private partial/cache descendants.

Retain existing tests for full process-tree cancellation, scheduling failure isolation, queued-work cancellation, private text-cache cleanup, source-change invalidation, stale metadata rejection and segment/chapter/movie assembly. A rename must not remove those behavior checks merely because discovery changes.

## Execution Boundary

Parent owns the frozen complete move map, taxonomy registration, source/index mutation orchestration, documentation and final ticket lifecycle. This report supplies reference closure for runtime, wrappers, root helpers, schema and tests. Pipeline tooling and native setup owners should use the same map. Ticket evidence is explicitly exempt from product taxonomy; historical reports and retained input scripts stay where they are. Active acceptance scripts that reference old product paths need deliberate updates, while historical read-only evidence should retain its recorded paths.

Before acceptance, run the repo-backed taxonomy/reference checks, explicit canonical test suite, actual Manim discovery and tiny path/render oracle. Preserve authored scene construction, playlist order, timing content, PDF bytes, frozen glyph fallback and full-render settings. Do not begin another full render merely to discover a missing import; complete this bounded closure first.
