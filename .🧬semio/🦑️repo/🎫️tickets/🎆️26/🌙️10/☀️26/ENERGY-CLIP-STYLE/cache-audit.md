# Energy Clip Cache and Font Audit

## Cache Dependency Closure

Owner: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py`.

`clip_state` (line 254) SHA-256 fingerprints every existing supplied file. `segment_state` (line 281) is the single insertion point: it currently supplies scene script, shared fonts Python module, shared visuals Python module, nearby scene assets, and TTF/OTF files recursively under the teaching fonts module only. Framework-owned canonical tokens and font files are outside both existing scans, so edits to them currently leave the clip cache reusable.

Add the theme Python module, canonical UI tokens JSON, font catalog if theme resolves it at runtime, and every exact vendored font outline actually registered by the shared font interface. Prefer a theme-owned dependency-path function consumed by segment_state rather than duplicating catalog/token/path knowledge inside runtime. That function should be pure to avoid importing Manim or registering fonts merely to fingerprint a clip. All shared appearance dependencies affect every scene and should be included even for scenes without a local asset.

The same state automatically propagates to chapter `_logical_state` and final video state. `render_segment` compares segment_state before and after rendering, so the closure also prevents publication if a token/font/theme changes during render. No separate chapter/video invalidation change is necessary.

## Existing Test Anchors

Tests: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🧪️tests/🐍️.py`; language-neutral cases: sibling `🔣️.cases.json`.

- line 202: test_whole_chapters_reuse_without_retaining_segment_media; fixture currently creates only shared fonts/visuals modules.
- line 366: test_changed_intro_asset_invalidates_clip exercises raw clip_state with changed PDF bytes.
- line 403: test_render_cache_publication_requires_completed_unchanged_sources exercises source changes during render and successful provenance publication; common helper fixture at line 411 currently contains only fonts/visuals.
- line 440: test_non_reusable_segment_disables_manim_partial_cache ensures stale runs bypass Manim partial caching.

Add language-neutral cases for theme code, tokens, font catalog and registered TTF byte changes; assert segment fingerprint changes, existing sidecar becomes non-reusable, and changing a dependency during render prevents successful sidecar publication. Existing temp-root tests patch teaching_root, so a new helper must permit deterministic fixture paths instead of inadvertently hashing only real workspace dependencies.

## Scene Font Literal Audit

Audited all Python under energy clip scene trees. No literal font family bypasses BODY_FONT in authored scenes. The only scene-level explicit family keyword is `font=BODY_FONT` in `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` at lines 172, 177, 655, 823 and 1078. The certificate scene uses a parameter called font for numeric sizes (lines 139, 205 and call sites), not a family override. A conduction narration string says Times as ordinary English text.

All remaining Georgia/serif/DejaVu family references are centralized in `🏗️authoring/🔤️fonts/🐍️.py` and its tests; the owner is already replacing that shared policy. The intro assigns apply_body_font from the shared module. No per-scene family cleanup is needed.
