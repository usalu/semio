# Physical Video Cache Findings

Read-only bounded inspection of known teaching cache and the neighboring same-day architecture/transfer tickets. Default `.🧬semio/🎓️teaching/⚡️cache/videos` is absent. The transfer ticket has no retained generated media.

The architecture ticket retains `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ARCHITECTURAL-ENERGY-CLIPS/🗑️generated/full`. Physical sources occupy `s/1` through `s/9`, rendered at 640×360, 6 fps, quality -ql. `full/1` has settings only and no assembled physical lesson exists at `full/rendered`.

Recomputed every source SHA-256 recorded by available physical sidecars. Intro at `s/1/v/Demo_Intro_PhysikalischeGrundlagen.mp4` and Beat8 at `s/9/v/Beat8_Zusammenfassung.mp4` match all currently recorded dependencies, including shared theme, fonts, visuals and framework style assets. Beats1–3 at s/2–4 are stale only for the authored physics/fundamentals Python source; shared appearance dependencies match. Beat4 at s/5 has partial movies only and failed because the German heater/fuel caption exceeded two lines. Beats5–7 at s/6–8 contain MP4s without successful sidecars, so provenance is incomplete and they cannot be accepted as current complete cached segments.

The architecture output therefore offers partial current-style reuse but no immediate complete physical lesson. A managed render can reuse valid intro/Beat8 and rerender stale/missing segments. Coordinate with any architecture render owner before sharing its cache. No source, media, cache metadata or process was modified by this investigation.

## Preview And Lightweight Media Checks

Available app tool metadata has no dedicated local-video preview action. The existing runtime play option uses os.startfile on Windows, open on macOS, and xdg-open on Linux. The app also supports Markdown media with an absolute local MP4 path. No browser UI or IDE is necessary.

Using the existing teaching Python environment and PyAV headers only: current intro is H.264, 640×360, 6 fps, 81 frames, 13.5 seconds. Current Beat8 is H.264, 640×360, 487 frames, 81.165365 seconds (average frame rate approximately 6). No full decoding was performed.

The final pipeline stream-copies ordered inputs through FFmpeg, publishes the output atomically, and writes provenance after successful assembly. It does not separately validate final duration/aspect; after completion, inspect final metadata and a few decoded samples. This audit did not launch the player or modify media.
