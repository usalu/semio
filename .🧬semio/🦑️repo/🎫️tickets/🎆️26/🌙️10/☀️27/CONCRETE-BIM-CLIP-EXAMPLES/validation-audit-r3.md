# Villa Savoye Validation and Clip Integration Audit

## Contracts
Canonical clip: 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip. No nested AGENTS found under teaching. Portable cases: 🧪️cases/🔣️.json. Model/schema: 🧱️model/🔣️.json and 🧱️model/🧬️schema/🔣️.schema.json.

Existing tests encode a regular 25-piloti grid, equal spacing, constant-slope ramp, concentric car/glass arc, slab geometry and wall/opening chains. Remodel source, cases, schema and bilingual narrated values together. Python oracles include NumPy/SciPy, UUID, mesh hull/volume and kd-tree sampling. AJV and independent JavaScript reproduce schema checks and GlobalIds.

Meaningful classes: BimModelStructureContract, BimGridContract, BimGeometryContract, BimQuantityContract, BimRepresentationContract, BimAnalysisContract, BimTributaryContract and BimClipContract.

Native presentation tests compare plan geometry and drawing providers against NumPy/Cairo; verify canonical window identity and 3D dock points; exercise real animation setup/interpolation/cleanup and Cairo in both languages at 480x270, 4fps. SEMIO_TEST_BIM_TOPICS narrows lifecycle to information, objects, representations, models, exchange, consistency. Source hashes before/after reject concurrent stale results.

## Exact Commands
Run from repository root. Configure SEMIO_TEACHING_OUTPUT_ROOT and SEMIO_TEACHING_CACHE_ROOT inside ticket 🗑️generated, PYTHONDONTWRITEBYTECODE=1, SEMIO_TEST_MANIM=1.

Focused Python:
```text
repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -m unittest 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️
repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -m unittest 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🎭️presentation.🐍️
repo tools exec bun -- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🟨️.cjs
repo tools exec bun -- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🟨️metadata.mjs
```

Supplemental suites: model/exchange/tests, consistency/tests/lifecycle, models/tests/footer, objects/tests/retirement, model/tests/loss-selector. The shared authoring test entry also runs optimization and energy suites:
```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
```

## Render
Exact flags read from current parser:
```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_representations --language en --audience spectator --quality l --resolution 960,540 --fps 12 --jobs 1 --no-play --output-root <ticket>/🗑️generated/render
```
Repeat for de; final coverage should include bim_information, bim_objects, bim_representations, bim_models, bim_exchange and bim_consistency. --chapter selects scene, --force bypasses cache, --log-output retains attempt logs. No still/frame flag exists; native Cairo contracts generate frames or use repo tools exec ffmpeg to extract video frames.

Preserve literal importlib provider paths for dependency closure. villa_axon contextual selection uses ghost=(tag,). STEP export requires explicit scene language. All views must derive remodeled common geometry.

## Status
Read-only discovery only. No tests, renders or dependency provisioning executed. Only this audit file created.
