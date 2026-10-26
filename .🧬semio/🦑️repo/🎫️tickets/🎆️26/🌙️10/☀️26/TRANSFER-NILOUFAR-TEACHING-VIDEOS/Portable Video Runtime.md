# Portable Video Runtime

The authored full-video wrappers retain their scene classes, playlists, beat order, German captions, styling, and narration timing. Composition uses `video_runtime.py` and the language-independent `video_manifest.json` contract: 112 individually rendered authored beats and five intros form 26 ordered chapters, 31 logical clips, and five movies.

The five curriculum videos contain 1, 6, 6, 6, and 7 chapters respectively. Each final video starts with its own authored series intro. The manifest is checked both against Python source classes and against Manim's actual scene discovery and imported playlists.

Manim is invoked as `sys.executable -m manim`, so the active provisioned Python environment works on Windows, macOS, Linux, and devcontainers. FFmpeg is resolved behind one interface from an explicitly supplied executable, PATH, or the provisioned `imageio-ffmpeg` binary. Concatenation writes resolved POSIX-style paths with FFmpeg-compatible apostrophe escaping and atomically publishes the finished MP4.

`render_videos.py --output-root <directory> --quality l --jobs <limit> --no-play` renders complete videos with one global concurrency limit. Repeated `--video <id>` options select lessons without changing global segment ordinals. Full validation uses the authored durations at 480p15; optional resolution and frame-rate arguments support previews without changing scene logic. Ctrl+C cancels queued work and active process trees while retaining successful segment provenance.

Private `s/<ordinal>` cache folders keep each segment's configuration, partials, SVG cache, and completed MP4 under the selected output root. Logical outputs remain in numeric per-video directories 1–5. Font discovery also writes to the selected root instead of the teaching source directory. Python child processes receive UTF-8 settings so emoji paths work with Windows' native default code page.

Initial test run failed because the runtime did not exist, as expected for the test-first change. After implementation, seven tests passed with `SEMIO_TEST_MANIM=1`, including actual Manim 0.22 scene discovery of every full-video wrapper. The first native Windows render exposed Manim's default-code-page config decoding; setting UTF-8 on the shared tool interface corrected it. Full rendering and final decode checks are tracked in the main ticket report.

Subsequent focused cache and process-cancellation regressions bring the validated runtime suite to eleven tests. Cancellation terminates the complete owned Windows redirector/child tree or POSIX process group. Actual child and grandchild processes are used in the regression, and five distinct series intros are explicitly required by the contract.

The final segment-plan implementation passes twenty runtime tests, including real Manim segment rendering and decoded frame/color ordering, global worker bounds, failure isolation, actual batch process-tree cancellation, queued-work cancellation, and reuse of complete logical chapters after segment media is removed. Source ordering also passes an independent comparison against every original playlist. See `Segment Rendering.md` for the final contract and corrected validation run.

Each completed clip records its settings and SHA256 fingerprints of authored dependencies. Intro dependencies include its PDF and other assets. A changed asset invalidates its clip even if file timestamps are restored. Settings are persisted before rendering and per-clip fingerprints are written only after success, so cancellation during a quality switch preserves completed matching clips and does not admit old clips at another quality. `--concat-only` rejects missing or stale clips. Tests cover changed intro assets and a partially completed quality switch.
