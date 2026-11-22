# Wave 2, item 3: tutorial video transport, record and add-chapter (opus55 fork)

Nothing compiled or run (build freeze). `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `W` = `E/🎯️targets/🧊️wgpu`.

## A. Tutorial video now travels the frame transport
Design: the media-slot path gets a second, typed owner. The tutorial `<video>` is ONE shell-owned `PresentedMediaSlot` (window `shell`, plugin `shell`, controller `shell.tutorial`, node `ui.tutorial.video`, token `shell.tutorial.video`, `props.kind = "tutorialVideo"` with src/muted/playing/rate/localTimeMs). It rides the existing canonical, retired descriptor vector, so no new output field and no new retirement type.
- Schema first: `W/🎬️media-slots/🧬️contract/🔣️.json` (`props` is now anyOf media-transport | `$defs/shellTutorialVideoProps`; an if/then pins the shell identity constants).
- Rust: new `W/🎬️media-slots/🎞️shell-tutorial-video/🦀️.rs` (`shell_tutorial_video_slot`, `push_shell_tutorial_video_slot`: refuses empty rect, empty or >32768 byte source, non-finite numbers, and never exceeds the 32 slot ceiling; paint order = last). Fixture `E/🧪️tests/🧫️fixtures/🧊️wgpu/🎬️media-slots/🎞️shell-tutorial-video/🔣️.json`, Rust law beside the module.
- Shell: `PanelsViewCtx::tutorial_video_cue` (`E/🧱️elements/🐚️Shell/.../🪀️view-panels/🖼️chrome-frame-and-main-window`, only while Playing/Paused/Deviated, viewport = `chrome.screen_w/h`, reuses `tutorial_video_overlay`). The presenter appends the slot to `cursor.media_slots` right after the shell clock deadline is computed in `W/🧊️renderer/🖥️frame-presentation/🎬️present-step-inner` (inside the existing acknowledge step, so the existing close/retire path for `cursor.media_slots` owns it; `AppPresentStep::Complete` is unchanged, so no struct literal or match needed updating).
- TS: `E/🎬️media/🌐️browser/🎞️tutorial-video/🟦️.ts` gained `splitShellMediaSlots`, `parseShellTutorialVideoSlot`, `parseTutorialVideoPlan`, `SHELL_TUTORIAL_VIDEO_SLOT`. The frame worker (`W/🎞️frame-worker/🎞️frame-and-introspection`) splits the shell slot off BEFORE the plugin `mediaRegistry.accept` (so the transport parser, registry and plugin overlay never see it), posts `tutorialVideo` on the `frame` message (optional field, absent = none; degraded or quarantined frames post null). Transport (`W/🚚️browser-frame-transport`, contract) re-admits the plan and delivers `directives.tutorialVideo`; `W/🌐️browser-host` creates `createBrowserTutorialVideoOverlay(root)` in `onReady`, calls `accept(tutorialVideo)` beside `mediaOverlay.accept`, disposes it on fault and dispose. New package export `.../engine/media/browser/tutorial-video` in `🛍️products/💻️os/📦️packages/🟦️typescript/package.json`.
- Native: no decoder; `media_slots` there is not consumed by any video host, so the cue paints nothing (as React without a src).
- TS law: `E/🧪️tests/🎞️wgpu-tutorial-video/🟦️.ts` validates the shared fixture slots with Ajv against the updated contract, splits and re-parses them, refuses a forged owner, a duplicate shell slot, an empty source.

## B. Record and add-chapter buttons (React `TutorialBar`)
- Bar (`E/🧱️elements/🐚️Shell/.../🍱️view-chrome/🧭️navbar-footer-and-overlays`): phases 22 add chapter (`shell.tutorial.addChapter`, icon `plus`), 23 record (`shell.tutorial.record`, icon `circle-dot`, pressed = recording, exposed to accessibility), 24 recording indicator text in `theme.error` while recording; laid right to left after captions, React order.
- Actions (`🪝️window-actions/🎓️tutorial-runtime`): `tutorial_toggle_recording` (recording: stop and save; otherwise end any playback and start recording the live document), `tutorial_add_chapter` (only while recording, as React's no-recorder no-op). Chapter rule `TutorialRuntime::next_chapter` (surfaces): id `chapter-N`, rounded non-negative playhead, title one locale-free string `Chapter N`, exactly React's `TutorialRecorder.addChapter`.
- Labels en+de in `W/📖️shell-vocab` (`tutorial.record` Record/Aufnehmen, `tutorial.recording` Recording/Aufnahme läuft, `tutorial.addChapter` Add Chapter/Kapitel setzen), text equal to React's guidance bundle.
- Fixture `recording` block in `E/🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎬️tutorial-presentation/🔣️.json`; Rust laws `a_recording_adds_react_numbered_chapters_at_the_rounded_playhead`, `the_record_controls_use_the_shared_en_de_labels` in `🔬️wgpu-tutorial-presentation`.

## Honest gaps
- The shell slot is appended after the paid collect phase (one small allocation per frame while a video cue is active); the existing paid-heap laws of the collect and snapshot path were not re-run and might need the slot counted.
- Record from a playing tutorial ends that playback (one runtime slot); React keeps playing and records the live session in parallel.
- React shows Record only in dev builds; wgpu shows it always (no dev flag in the shell). Narration audio and blob video sources remain unsupported (resolver returns None for blobs).
- `🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (generated) does not yet list the new package export; regenerate with the repo tool.
- `TutorialVideoOverlay.rect` must be the same `Rect` type as the media-slot crate's (`semio_ui_theme::geometry::Rect`); if not, convert at the presenter call.
- TS React-id fixture entries (`reactControls`) are not yet asserted against the React DOM.

## Run list (all NOT RUN), from `/Users/ueli/Documents/semio`
1. `repo cargo --store=shared --configuration=prototype -- check -p semio-renderer-wgpu-media-slots -p semio-renderer-wgpu-shell-view-panels -p semio-renderer-wgpu-frame-presentation -p semio-renderer-wgpu-shell-view-chrome -p semio-renderer-wgpu-shell-window-actions -p semio-renderer-wgpu-shell-surfaces -p semio-renderer-wgpu-shell-vocab`
2. `... test -p semio-renderer-wgpu-media-slots` (new `shell_tutorial_video::tests`, plus the existing paid collect laws) and `-p semio-renderer-wgpu-shell-tests-chrome-4` (filter `tutorial`).
3. Wasm check of media-slots, frame-presentation and view-panels (`--target wasm32-unknown-unknown` per the repo script).
4. Vitest: `E/🧪️tests/🎞️wgpu-tutorial-video`, `E/🧪️tests/🎬️presented-media-slots`, `E/🧪️tests/🎬️wgpu-browser-media`, `E/🧪️tests/📨️browser-frame-transport/*` (frame messages now carry optional `tutorialVideo`, directives a required one), plus the typecheck of the renderer package.
5. Visual in the browser host: a tutorial with a video cue (url) plays muted, seeks within 300 ms drift; record, add chapter (tick appears after stop and replay), indicator, en and de.
