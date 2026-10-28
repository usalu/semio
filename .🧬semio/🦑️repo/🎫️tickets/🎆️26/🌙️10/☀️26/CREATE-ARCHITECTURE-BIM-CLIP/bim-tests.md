# BIM Video Test Contract

## Final Scope

Six independently registered videos use IDs `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, and `bim_consistency`. Each video has one source scene, its own thin wrapper, a unique output name, and explicit English/German catalog titles. The shared entry is an unregistered six-video playlist.

The language-neutral cases record these identities, paths, titles, outputs, language/audience policies, geometric expectations, invalid events, and malformed model cases. A schema-first model contract defines object identity, inheritance, composition, representations, model subsets, analytical translation, and external links.

## Verification

All 15 final Python contracts pass when verified in the following two runs:

- Fourteen semantic, scene-discovery, registry, wrapper, playlist, and locale contracts passed in 7.135 seconds against the final fixtures.
- The exhaustive native caption contract passed for every bilingual clause and both audience policies. A focused final topic-four check passed in 41.760 seconds after its two additional FEM/translation clauses were saved.

The final content has 49 clauses: 98 distinct English/German captions and 196 audience-specific caption invocations. Native Manim/Pango `caption_bar` verifies that visible captions fit inside the frame and hidden captions retain only the invisible anchor. The initial exhaustive native run completed its caption contract without errors in a 178.593-second suite; its only unrelated failure was a catalog-title fixture that changed while that process was running. The updated fixture passed the subsequent fourteen-contract run.

The independent Ajv oracle accepts the authored model and rejects all five portable malformed cases. Existing NumPy independently checks rectangle area, solid volume, and reconstructed FEM shell areas. FEM checks also resolve node/material references and base supports. Event checks preserve object identity and source immutability, keep material-layer thickness coherent, derive fresh representations, and report stale external analytical links.

Locale contracts require explicit language, isolate recorded speech by locale, propagate language/audience to render children, and partition composition media/trace/settings paths. Each source is discovered by the actual Manim scene-discovery API.

## Test-Driven Evidence

The initial schema/cases/test run failed before the model helper, scenes, and curriculum registration existed. After the six-video steering, the split contract also rejected the old singular playlist entry before `VIDEO_IDS` landed. Standalone wrapper and playlist routing now pass, including selected-scene forwarding.

## Integration and Cleanup

The existing teaching test command discovers the BIM Python suite and runs the test-only Ajv oracle through a Node module boundary. No package or runtime dependency was added.

All commands run through `repo tools exec`. Final Python verification used the provisioned Python 3.12 teaching environment with explicit `UV_PROJECT_ENVIRONMENT`, ticket-local `SEMIO_TEACHING_OUTPUT_ROOT`, and `SEMIO_TEST_MANIM=1`. An accidentally created project-local UV environment was confirmed owned by this invocation and completely removed; the shared teaching cache was retained. Temporary native SVGs were inside automatically cleaned ticket-local temporary directories.

## Files Owned by This Agent

- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️cases/🔣️.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🟨️.cjs`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-BIM-CLIP/bim-tests.md`

## Final Runtime Review and Selected-Source Regression

The final focused run passed **28 tests in 25.667 seconds**: 18 BIM contracts and 10 affected shared curriculum/cache/native-media tests. The native caption test remains covered by its earlier successful exhaustive run and focused final topic-four confirmation; it was intentionally not repeated. The BIM suite therefore contains 19 tests.

The shared checks covered curriculum registration and global ordinals, canonical recorded speech, dependency invalidation, semantic excerpts, reuse without segment media, success-only cache publication, complete Manim discovery, actual ordered source rendering, and actual partial-source rendering/audio assembly. Concurrent optimization registration completed before the final run, so the expanded 17-video curriculum also passed its shared contract checks. The unrelated presentation JSON-key-order/choreography test reported by the root agent was not modified or included in this focused run.

Review found one task-local integration blocker: a newly registered unselected lesson with unfinished source files prevented rendering selected BIM lessons. Four schema-first regression methods now cover selected ready assets, metadata-only enumeration, selected missing asset rejection, strict complete-manifest loading, global containment/scene/clause validation, selected clause AST ranges, and renderer metadata routing. Their initial run failed before the new keyword existed.

`load_manifest(*, source_ids=None)` retains full source validation by default. An explicit list validates file existence and narration clause ranges for those lessons while keeping structure and containment validation global. An empty list permits metadata enumeration. Rendering passes its selected IDs, with `None` and empty render selections both retaining complete source validation. The CLI uses metadata enumeration to build choices, then selected rendering performs strict checks.

The independent Ajv oracle additionally accepts the portable selection-manifest fixture against the existing curriculum schema and rejects its four malformed global structures. Together the two Ajv checks accept two valid fixtures and reject nine malformed cases. No new package or runtime dependency was introduced.

The runtime review found no additional regression in explicit locale settings, render-child language propagation, locale-separated canonical speech, media/trace roots, or dynamic manifest registration. No success-only fingerprint guard was weakened.

## Presentation Provenance Audit

See `presentation-fingerprint-audit.md` and its reproducible ticket-local input script `presentation-audit.py`. Two sequential presentation snapshots were equal, with 2,546 authored source hashes and zero generated/cache paths. Existing media certificates were stale against the concurrently edited presentation catalog `🪟️presentation/🔣️.json`. The publication rejection correctly protected provenance. Root was advised to regenerate with current inputs rather than reuse stale certificates.

## Additional Files Owned During Final Verification

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-BIM-CLIP/presentation-audit.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-BIM-CLIP/presentation-fingerprint-audit.md`

Temporary native-media/speech fixtures beneath this agent's ticket-local `🗑️generated/final-tests` subtree were removed after verification. Root's render output subtree was preserved.

The final selected-source tests also isolate a missing selected segment while its wrapper exists. All four selected-source contracts pass on the registered standard-library Python 3.14 interpreter (0.446 seconds), in addition to the prior Python 3.12 environment checks. The duplicate queued UV rerun was canceled after the independent standard-library run completed, preserving render capacity. No runtime/renderer change followed the empty-render-selection fix.

## Browser Source-Query Performance Regression

The browser source-query path previously imported the bundled browser-tool adapter and awaited `loadPlaywrightV1(root)` before reaching its `--sources` branch. Inspection found that `loadPlaywrightV1` configures the repository browser-cache environment and imports Playwright; it contains no explicit provisioning or dependency synchronization itself. Loading that adapter also evaluates its registry/tool-run dependency graph.

The narrow fix moves the bundled-adapter require and Playwright library load into the actual-render branch, after its cancellation hooks. Source discovery continues to bundle the adapter and collect its complete esbuild input closure. No success-only fingerprint or browser provisioning guard changed.

The language-neutral `browser_metadata` case describes the expected metadata-only calls. The new behavioral regression executes the actual renderer body with opaque filesystem/build boundaries and rejects any attempt to load a browser runtime or launch a browser. The existing Babel parser identifies import declarations for this test harness; this is a test-only use of an existing library.

The regression failed before the fix with “Metadata source discovery loaded a browser runtime.” After the fix it passed both as a direct Node test and through its registered Python test wrapper. The Python wrapper passed in 0.801 seconds. It also verifies that the returned dependency closure still contains the browser adapter source. All filesystem writes in this harness are in memory, so it creates no generated workspace artifacts.

The BIM suite now contains **20 tests**. The earlier exhaustive caption and 28-test focused verification remain applicable; the new browser metadata regression was verified separately. Runtime, browser rendering, and source assets were declared frozen before root restarted the final 24 previews.

Additional changed/created paths for ticket closure:

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🟨️metadata.mjs`

The existing case JSON and Python test module listed above were also updated for this regression. No additional runtime package was introduced.

## Encoded Audience and Language Audit

The preserved ticket input `audience-audit.py` independently decodes the published previews with PyAV and analyzes mapped caption regions with Pillow/NumPy. It derives each semantic sample from that source variant's trace and measured presentation padding, rather than assuming English/German timing equality. Quantitative results and pending coverage are recorded in `media-audience.md`; generated contact sheets and JSON stay under `🗑️generated/audience`.

The first two complete four-variant topics (Information and Objects) passed encoded audience/language checks. Visual readback confirmed readable English/German clauses and blank Spectator caption regions. Objects uses different actual sample timestamps (EN 44 seconds, DE 42 seconds) for the same `methods` clause. No production files were modified for this audit.

Final encoded-output audit: all six topics / 24 published previews passed. All 12 Speaker crops visibly contain correctly localized captions; all 12 Spectator crops have zero bright caption glyph pixels. Manual readback of all six English/German contact sheets confirmed the expected clauses. Exact metrics, trace-selected timestamps, and limitations are preserved in media-audience.md. Additional closure files: `audience-audit.py` and `media-audience.md`. No production source changed during this audit. Generated evidence may now be cleaned after root review.
