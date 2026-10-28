# Optimization Video Authoring Exploration

Read root AGENTS.md. No nested AGENTS.md exists under 🎓️teaching. All operations used repo; rg is not registered, so inspection used Node. No implementation files modified or tests/renders executed.

## Authoring Convention

Closest precedent: 🎓️teaching/🏛️architecture/🏗️bim implements six standalone videos. Canonical scenes live under <domain>/🎬️clip/<topic>/🐍️.py; public wrappers under <domain>/<topic>/🎬️clip/🐍️.py. Root clip wrapper supplies a playlist.

Scenes use a unique Beat class, TITLE={en,de}, ordered NARRATION rows [key, English, German], and a domain visual interface. Shared interfaces provide theme, font, caption clock, narration holds and animation. source_files() must include semantic model, schema, fixture and theme inputs for cache provenance.

Wrappers expose VIDEO_ID, compose(*,language,chapters=None,**settings) delegating to runtime.compose_videos, and main(argv=None) delegating to runtime.video_main. Require explicit en/de. Speaker captions and spectator animation share timing. Authored narration does not itself synthesize audio.

## Registration

Curriculum: 🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json. Add six independent video objects with id, script, chapters, output only. IDs accept lowercase letters and underscores. Paths are relative to 🎓️teaching. Each chapter can select one canonical source segment, matching BIM. Outputs and scene names must be unique.

Presentation: 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json. Existing optimization topic has empty videos. Add id, localized title, icon, symbol, cell per video; six entries fit columns 0..2 and rows 0..1. Presentation schema currently restricts symbols to physics,air,heat,cool,balance,bim. New optimization symbol requires schema and renderer implementation.

## Tests and Contract

Use portable semantic fixture, JSON schema, and language-agnostic cases beside domain implementation, matching BIM 🧱️model/🧬️schema/🔣️.schema.json and 🧪️cases/🔣️.json. Validate own stdlib computations against existing NumPy or other provisioned third-party oracle. Existing BIM CJS uses Ajv 2020 schema validation.

Cover calculations and edge cases, bilingual narration AST, Manim class discovery, caption bounds for both languages/audiences, curriculum registration, presentation registration and standalone entry delegation.

Test orchestration is explicit in ⚡️energy/🏗️authoring/📜️script.ts: add optimization Python unittest module to test command and CJS oracle require alongside existing BIM require. Python authoring project already supplies Manim, NumPy and imageio-ffmpeg under Python 3.12.

Normal validation: repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test

Normal render: repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video <id> --language en --audience speaker

Repeat --video for playlist. Explicit de/spectator tests the other path. --output-root selects location; SEMIO_TEACHING_CACHE_ROOT and SEMIO_TEACHING_OUTPUT_ROOT relocate all generated work into ticket 🗑️generated, which must be removed when finished.

Existing optimization quiz distinguishes feasibility constraints from scalar objectives, counts a weighted scalar as one objective, and teaches strict dominance including equal vectors. Preserve that mathematical precision.
