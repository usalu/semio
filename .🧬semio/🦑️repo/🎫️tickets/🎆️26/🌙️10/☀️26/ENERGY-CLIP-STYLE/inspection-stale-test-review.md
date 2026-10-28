# Registered Test Contract Audit

Read-only checkpoint during concurrent hierarchy/browser test updates. No test edits or suite/UI runs. All paths below are relative to 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring; 📜️script.ts line 18 still registers these modules.

| Concrete stale assertion | Path and lines | Required contract |
| --- | --- | --- |
| Phase-only transition_duration(focus/phase), topic/video timeline fields, two click targets only | 🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️.py:542–604, especially 546, 552–566, 580–598 | Scene-specific duration and opened_path/revealed_path with all four navigation levels; actual expected clicks derived from selected chapter path and ancestors. |
| Native interaction frame helper and popup request still pass physical_fundamentals as video_id directly to hierarchy renderer | Same file:476–509 | Hierarchy renderer selects a chapter scene; keep clip ID only at the public clip wrapper boundary. |
| Browser native fixtures send opened_topic/opened_video/revealed_topic/revealed_video and assert root cards equal six topics | 🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🐍️.py:191–249, 267–321 | Actual recursive opened_path/revealed_path and root course, then topics, clips, chapters. Executor already owns this migration. |
| Geometry native browser packet uses old fields; nested cell test covers only topics and first videos | 🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🐍️.py:119–133 and 179–185 | First packet is incompatible with strict new browser protocol. Cell coverage should include courses and chapters while retaining independent geometry oracle. |
| Runtime composition asserted raw chapter concat then a single whole-lesson envelope, exactly once | 🎬️clip/🪟️presentation/🧪️tests/🐍️.py:172–221 | Runtime must publish individual chapter windows then concatenate those windows. Direct low-level envelope API remains valid separately. |

## Legitimate Current Assertions

No executable deleted scene_title/play_scene_title/beat_subtitle call remains in the inspected registered authoring tests. The names in fonts/🧪️tests/📑️heading/🔣️.json are intentional forbidden-API fixture data. Instructional section_heading and calculation heading assertions are legitimate and must remain.

The hierarchy test was already updated concurrently: hierarchy/🧪️tests/🐍️.py lines 56–96 now validates four path commits and scene-specific choreography. Its six topic ownership assertions (lines 16–34) remain valid curriculum metadata, because contract.topics is a projection under courses[0]; they do not assert six topics at the visible root. Likewise presentation metadata checks of topic ownership remain legitimate.

Direct presentation.render_presentation(clip_id, content, output, settings) tests are not automatically stale: this remains a public clip-wrapper API. Their content preservation, finite audio, resampling and atomic-certificate tests should stay, with new hierarchy scene context where required. The obsolete runtime test above is specifically about compose_videos behavior, not the existence of this wrapper.

Exact canonical overview comparisons remain legitimate with persistent actual-UI boundary assets. Independently painted uncached blur rasters must not be assumed deterministic. Native simple color-raster finite-frame fixture assertions are also legitimate narrow encoder controls, not full collection acceptance.

## Executor Migration Recheck

Read-only rescan after executor saved test migration: browser, hierarchy and interaction test modules contain no old opened_topic/opened_video/revealed_topic/revealed_video keys, phase-only transition_duration calls, or physical_fundamentals clip-ID frame request. The earlier findings for those three modules are resolved at this source checkpoint. Geometry/presentation ownership remains with root; this rescan does not duplicate their native suites.
