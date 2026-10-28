# Clip Authoring Exploration

Read-only audit of existing architecture clip sources. No source changes or media generated.

## Existing contract

- Curriculum: 🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json. Videos contain id, script, chapters, output. Chapters contain ASCII scene identity and segments. Each segment points to a canonical Python script and ASCII scene class; optional clauses select inclusive first/last NARRATION keys. Paths resolve from 🎓️teaching.
- Wrapper example: 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🐍️.py. VIDEO_ID identifies its manifest entry; compose delegates to shared compose_videos; main delegates video_main.
- Source example: 🌬️ventilation/🎬️clip/❓️need/🐍️.py. Ordered NARRATION tuples contain stable clause key, English text, German text. Shared fonts and visual primitives supply captions/style.
- Shared runtime: ⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py. Reuses canonical scenes, preserves clause clocks, handles process groups, progress and cancellation.

## BIM registration requirements

- BIM currently has only ❓️quiz; new clip belongs under 🏗️bim/🎬️clip.
- Runtime load_manifest hard-codes exactly five distinct videos. Curriculum schema at authoring/🎬️clip/🧬️schema/🔣️.schema.json has minItems=maxItems=5 and energy-only wrapper/segment path regexes. Generalize these before adding BIM.
- Runtime tests and language-neutral cases at authoring/🎬️clip/🧪️tests/{🐍️.py,🔣️.cases.json} fix five identities, chapter/segment totals, global ordinals and unique-source count; update intentional expectations.
- Presentation registration is separate: authoring/🎬️clip/🪟️presentation/🔣️.json already contains topic id bim at grid cell column 2,row 0 with empty videos. Insert a video object containing id, localized title {en,de}, icon, symbol, cell {column,row}. 🥞️hierarchy/🐍️.py video() and video_topic() resolve this catalog; unknown video raises. Extend symbol schema enum and thumbnail drawing if introducing BIM symbol.
- Clean ownership recommendation: architecture-level shared authoring and curriculum since BIM and energy now share consumers. Avoid energy-specific fixed cardinality in the generalized contract. Source schema should admit architecture topic paths while retaining traversal rejection and explicit localized titles.

## Commands and verification

- Documented wrapper: repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test.
- Render: repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language en --video bim --audience spectator --quality l --jobs 2 --no-play. BIM becomes valid after registration. Repeat explicit language de and speaker audience. Use ticket generated output root while verifying.
- Runner provisions uv Python 3.12 and pinned Manim 0.22.0, numpy, imageio-ffmpeg. Tests use unittest, fonttools and Manim/media decoding as external reference implementations.
- Add language-neutral test cases first for BIM identity, chapter/scene order, narration coverage and semantic transitions. Verify schema with existing third-party validator and render against Manim/media decoder reference as appropriate.
- Existing output root and audience interfaces provide separate speaker/subtitle and spectator outputs, source fingerprints and resumable completed segments.
