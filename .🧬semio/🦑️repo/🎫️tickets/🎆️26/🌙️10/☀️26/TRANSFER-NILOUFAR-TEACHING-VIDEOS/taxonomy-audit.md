# Teaching Clip Taxonomy Audit

Read-only authority: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (schemaVersion 7). Read repository AGENTS.md; no nested AGENTS.md exists in the explored teaching tree. No product/index change or command process was performed.

## Authoritative Rules And Current Gap

`physicalLeafRendering.filename` requires file-kind emoji plus extension chain; `_treePurityComment` places semantic concerns in registered emoji-plus-slug directories. Thus `🐍️video_runtime.py` is still wrong: use a semantic runtime owner containing `🐍️.py`.

The registered tree is exactly `🎓️teaching/🏛️architecture/⚡️energy/{🧲️physics,🌬️ventilation,🔥️heating,❄️cooling,📊️demand}`. Each topic currently has only the registered `❓️quiz` member. `🎬️clip` is not yet a semanticDirectoryKind or member of those teaching topics; neither intro, chapter, audio, speech nor timing ownership is registered. Add precise parent-aware kinds/member rosters before claiming these new paths conform. Do not evade the domain tree by giving old numbered folders emoji prefixes.

## Canonical File Roles

| Purpose | Registered file kind | Canonical leaf |
| --- | --- | --- |
| Python implementation or Python test | python-source, source role | `🐍️.py` |
| JSON descriptor/manifest/timing/trace data | json, schema role | `🔣️.json` |
| JSON Schema | json-schema, schema role | `🔣️.schema.json` |
| Language-neutral case table | json-case-table, test role | `🔣️.cases.json` |
| Fixture/expectation | json-fixture/json-expect | `🔣️.fixture.json`, `🔣️.expect.json` |
| Authored Markdown | markdown, documentation role | `📝️.md` |
| Institutional PDF asset | pdf, documentation role | `📕️.pdf` |
| Actual audio asset | audio, asset role | `🔊️.mp3` or `🔊️.wav` |
| Actual movie asset | video, asset role | `🎬️.mp4` |
| Actual slide deck | presentation, documentation role | `📽️.pptx` |
| TOML configuration | toml, configuration role | `⚙️.toml` |

A speech generator is Python source under a speech semantic directory, not an audio asset. A Manim scene/intro is Python source, not a presentation or movie file. Existing timing and trace JSON need separate semantic owners to avoid conflicting `🔣️.json` leaves. No `.cfg` or `.ini` file kind exists in current taxonomy; `⚙️manim.cfg` is not a conforming workaround. Prefer explicit Manim configuration through the existing Python interface and eliminate redundant per-section discovery configs, or register an actual format authority if their continued existence is essential. Generated render configuration stays generated.

Exact external filename contracts remain legitimate exceptions: `README.md` and `📜️script.ts` are registered path contracts; `pyproject.toml` and `uv.lock` are package-root contracts for ecosystem `🐍️python`. Retain these exact names only at a recognized package boundary; never cosmetically prefix them and break discovery. Ordinary custom manifests/tests have no such exception.

## Recommended Domain Ownership

Use the existing five topic owners with `🎬️clip` for their authored lesson content. Map Physical Fundamentals to `🧲️physics`, Ventilation to `🌬️ventilation`, Heating to `🔥️heating`, Cooling to `❄️cooling`, Energy Balance to `📊️demand`. Remove the duplicate plain `energy/demand/1_*` through `5_*` taxonomy after moving all live references.

Within each topic clip owner, put each original chapter into a named semantic concern (English kebab-case, fitting distinct emoji), not an ordinal folder. That concern owns its scene `🐍️.py`, composition/build, optional speech generation, timing and trace facets as separately registered semantic children. Example proposed path: `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🫁️air-volume/🐍️.py`; the air-volume member must be registered explicitly. Preserve playlist ordering in schema data rather than depending on directory numbering.

Shared rendering, fonts, visual primitives, curriculum schema and institutional intro have five independent topic consumers, so their lowest common domain owner is `⚡️energy`, not one topic and not loose teaching-root Python files. Register an energy-level `🎬️clip` owner for that shared pipeline with explicit semantic members, for example `🎬️clip/⚙️runtime/🐍️.py`, `🎬️clip/🔤️fonts/🐍️.py`, `🎬️clip/🎨️visuals/🐍️.py`, `🎬️clip/👋️intro/🐍️.py`, `🎬️clip/👋️intro/🏰️institution/📕️.pdf`, and `🎬️clip/🧬️schema/🔣️.schema.json`. These are proposed registrations, not already-valid vocabulary. Keep schema/manifest language-neutral; shared implementation can gain sibling language leaves later.

Use established `🧪️tests` and `🧫️fixtures` ownership: Python tests `🧪️tests/🐍️.py`, neutral case tables under their specific test concern, and a schema under `🧬️schema`. Do not keep custom root `test_*.py` and `*.tests.json` names. Update discovery to enumerate canonical leaves explicitly.

## Existing Exact Examples

- Teaching data already follows concern + kind-only leaf: `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/❓️quiz/🔣️.json`.
- Python test example: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗂️collection-catalog/🧪️tests/🐍️.py`.
- Python implementation example: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔮️oracles/📦️package-boundary-classification/🐍️.py`.

Final acceptance requires registered ownership and valid kind leaves for every transferred path, live incoming/outgoing reference closure, retained immutable authored scene/PDF bytes where possible, explicit Unicode-path loading (Python emoji basenames are not identifiers), and the 256-code-unit path bound. Taxonomy root comments explicitly require incoming and moved outgoing reference closure with preimage drift rejection. A semantic path move alone is not complete without updating runtime imports, manifest descriptors, PDF/font root lookups, tooling/test entrypoints and docs.

## Ticket Evidence Scope And Final Ownership Revision

Exact area authority resolves ticket evidence: `areas[".🧬semio/🦑️repo/🎫️tickets"] = "exempt"`, while `areas["🎓️teaching"] = "clean"`. Existing retained ticket report/input names are outside clean enforcement. Preserve them; do not rename 113 audit inputs/reports merely to imitate governed product leaves. The ticket manifest `🎫️ticket.json` is also a fixed filename contract. The generic ticket Markdown projection contracts do not override the explicit exempt area for this task.

For the final product map, prefer architecture-level authoring over the earlier energy-level shared-runtime suggestion: `🎓️teaching/🏛️architecture/🏗️authoring` owns framework-neutral rendering/font/visual/speech mechanisms. This reuses the existing teaching product rather than introducing another product. Register this parent-aware teaching authoring kind; existing generic authoring vocabulary scoped under other owners is not evidence of admission here.

Recommended exact owner map for shared transferred files (proposed registrations):

| Original teaching-relative input | Final teaching-relative owner/leaf |
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
| `video_manifest.json` | `🏛️architecture/⚡️energy/🎬️clip/🔣️.json` |
| `video_manifest.schema.json` | `🏛️architecture/🏗️authoring/🎬️clip/🧬️schema/🔣️.schema.json` |
| `intro/intro_scene.py` | `🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py` |
| `intro/generate_audio.py` | `🏛️architecture/⚡️energy/🎬️clip/👋️intro/🔊️speech/🐍️.py` |
| `intro/welfenschloss (1).pdf` | `🏛️architecture/⚡️energy/🎬️clip/👋️intro/🏰️institution/📕️.pdf` |
| `energy/demand/KONZEPTE_UND_FORMELN.md` | `🏛️architecture/⚡️energy/📖️concepts/📝️.md` |

Keep institutional intro/data at the energy content owner; do not put authored institutional content into generic authoring. For each topic, its full wrapper becomes topic `🎬️clip/🐍️.py`; every chapter concern owns `🐍️.py`, `🏗️composition/🐍️.py` for its builder, `🔊️speech/🐍️.py` for synthesis, `⏱️timing/🔣️.json`, and `📍️trace/🔣️.json` where the old files actually exist. Register exact concern rosters, not wildcard emojis. Topic README moves may use the existing fixed README contract or an explicit `📃️readme/📝️.md` owner rule, preserving unrelated existing root README content. Fixed launcher/package metadata paths remain exact contract names after package-boundary admission is confirmed.

The governed implementation scope is 141 explicit transferred/custom source/test/tool paths plus the owned README contribution, along with required authoritative taxonomy updates. Existing native shared files are already fixed external setup contracts. All generated output stays disposable and does not enter this product map. Parent coordination owns the complete explicit per-file move plan and final reference closure; this report supplies authority and ownership recommendations only.
