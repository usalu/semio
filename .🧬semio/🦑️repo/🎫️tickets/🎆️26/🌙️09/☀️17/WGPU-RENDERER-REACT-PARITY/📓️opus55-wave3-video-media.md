# Wave 3: VideoRenderHost (native) and MediaTransportHost native decoders

Closes items 1 and 3 of `📓️opus55-reaudit-os-elements.md`. Paths are relative to `/Users/ueli/Documents/semio/🧰️framework` (`F`); `E` = `F/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

Status key: WRITTEN = source in the tree. VERIFIED = compiled and the stated laws ran green. See section 5 for what has and has not been verified.

## 1. VideoRenderHost on the wgpu renderer

### What React does (the contract that was ported)
`E/🧱️elements/🎥️VideoRenderHost/🟦️.ts` answers `Effect::VideoRenderExport { filename, program }`:
- a plugin that declared the `media.video-render` capability hands the host a `VideoRenderProgram` (width, height, fps, background, path table, pictures, distinct scenes, run-length timeline); any other plugin is refused (`refusedCapability`),
- the kernel's program rule (`videoRenderProgramProblem`) admits or refuses it (`refused`),
- every distinct scene is painted once, each timeline run encodes one picture and one repeat per further frame,
- encoder: platform WebCodecs when the page supports the configuration, else the raster tier's first-party `AvcPcmEncoder`; container: MP4 (`writeAvcMp4`, `video/mp4`),
- the export is an event-sourced job (`VideoRenderJobLedger`, ids `export:video#N`): `started`, throttled `progressed` (100 ms), `cancelRequested`, `finished`. The UI is Task Manager rows (lane `export`, frames of total, Cancel), transient notices (started, done, cancelled, refused, failed, en+de) and the browser download. There is no settings dialog: width, height, fps and background are part of the program the plugin sends.

### What the wgpu renderer now does
| React piece | wgpu piece |
|---|---|
| `VideoRenderHost` job functions | new crate `semio-renderer-wgpu-video-render-host` (`E/🎯️targets/🧊️wgpu/🎥️video-render-host`, sources in `E/🧱️elements/🎥️VideoRenderHost/🎯️targets/🧊️wgpu/`): `🧵️jobs` (host over the kernel's own `VideoRenderJobLedger`), `🎞️render` (render loop), `🖼️program` (program to scenes, picture decode), `📖️labels` (en+de, identical to `VIDEO_RENDER_EXPORT_TEXT_V1`) |
| 2D canvas painting | the framework's wgpu rasterizer `semio_framework_raster::SceneRasterizer` (vello over wgpu, headless offscreen, RGBA readback). It had no image op, so `DrawOp::Image(ImageOp)` was added (crop, affine placement, opacity), `F/🔨️modules/🖌️raster/🦀️.rs` |
| `AvcPcmEncoder` + `writeAvcMp4` | the same Rust twin in `F/🔨️modules/🖌️raster/🎥️video/🦀️.rs` (`AvcPcmEncoder`, `write_avc_mp4`), now with its first production caller; no new encoder, no external codec |
| `createImageBitmap` of a picture URL | the framework's PNG and JPEG codecs (`semio_framework_pixels::image_decoding::decode_encoded_blocking`), `data:image/...` URLs (base64 or percent) decoded in place |
| `applyVideoRenderExportEffect` | `Effect::VideoRenderExport` arm in `E/🧱️elements/🐚️Shell/…/📥️staging/🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs`: owner = the document session's plugin, capabilities = the installed catalog row's `capabilities` (`host_base::plugin_catalog::plugin_catalog_v1`), then `OpeningState::enqueue_video_render` |
| notices | `ChromeState::show_transient_notice` with React's codes `video-render-export.{started,done,cancelled,refused,failed}` |
| Task Manager rows + cancel | `OpeningState::sync_runtime_export_tasks` (called where the install tasks are synced), lane `Export`, id `export:video#N`; the Cancel press is drained by `take_video_render_cancel_requests` in the Task Manager dispatch |
| progress / cancel affordance | the existing export band (`render_icon_export_step`) shows `Frame x of y · file` and a Cancel control when no icon export is running; accessibility node `shell.video-export.progress` (progressbar, busy) and the Activate event cancel; the settle pump stays awake while an export runs; shell close cancels and waits (`close_video_render_step`) |
| browser download | the save door added in wave 2: `present_media_export_bytes_cancellable(filename, "video/mp4", bytes, cancel)` (native save dialog); a dismissed dialog shows the `cancelled` notice, a failed save the `failed` notice |

New shell staging file: `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎥️video-export/🦀️.rs` (mounted in `E/🎯️targets/🧊️wgpu/📂️shell-doc-open/🦀️.rs`; the crate stays under its caps: 17 of 32 files, about 125 KB of 256 KB).

### Progress and cancellation
- The render loop stores frames done in `VideoRenderShared` after every frame and reads the cancel flag before every picture decode, scene paint and frame, and yields the executor after every frame. The shell folds the counter into `Progressed` ledger events (throttled, last frame always) from the frame pump.
- Cancel appends `CancelRequested` (idempotent: a second press, an unknown id and a foreign id answer `false`) and flips the shared flag; the loop ends with `Cancelled { completed }`, the job finishes `cancelled`, no file is written.
- Every failure is a finished job with a stated outcome (`refused`, `failed` with a reason such as `no wgpu rasterizer: …`), never a panic or a silent drop.

### Limits that remain (not hidden)
- Native only. The wasm32 renderer has no blocking GPU readback; there `enqueue_video_render` shows a `failed` notice. In a browser the TypeScript `VideoRenderHost` stays the owner.
- Pictures: PNG and JPEG data URLs are decoded. A same-origin path URL (`/…`) needs an asset door the native renderer does not have; it fails that export with `image N: the native renderer has no same-origin asset door for …`. SVG pictures are not decoded (the in-repo image decoders are PNG and JPEG).
- Encoder tier: first-party only (`I_PCM` pictures, `P_Skip` repeats, `VideoRenderEncoderTier::FirstParty`). A platform hardware encoder (VideoToolbox, Media Foundation, VAAPI) would be an external dependency and is not used. Files are large for moving content (exact PCM per distinct picture) and tiny for still slides.
- The program background is assumed opaque (React's canvas keeps straight alpha, vello's readback is not un-premultiplied here).

## 2. MediaTransportHost native decoders

### Survey of in-repo codecs (searched `🖌️raster`, `🔲️pixels`, `🎬️media`, `🔌️plugin/🪟️window-kits/🎬️media`, whole framework by name and by signature)
| Format | In-repo decoder before this wave | Now |
|---|---|---|
| WAV / PCM (8, 16, 24, 32 bit), IEEE float 32 and 64, A-law, µ-law, WAVE_FORMAT_EXTENSIBLE | none | decoded, `…/🔊️wav` |
| MP4 with the AVC stream the framework's own encoder writes (`I_PCM` IDR, all-`P_Skip` P) | encoder and muxer only (`🖌️raster/🎥️video`) | decoded, `F/🔨️modules/🖌️raster/🎥️video/🔓️decode` |
| general H.264 (transform, intra and inter prediction, CABAC, B pictures) | none | typed refusal `unsupported` naming the feature |
| AAC, MP3, Opus, Vorbis, FLAC, ALAC | none | browser only |
| HEVC, VP8, VP9, AV1; WebM, Matroska, Ogg, fragmented MP4 | none | browser only |
| audio output device (CoreAudio, WASAPI, ALSA) | none (no in-repo sink; the only candidates are external crates) | none: `pull_audio` hands decoded blocks to whatever sink the host owns |

The browser overlay (`E/🎬️media/🌐️browser/🎛️host/🟦️.ts`) hands the bytes to an `HTMLMediaElement`; that remains the only decoder for the "browser only" rows above. They are listed in code as `NATIVE_MEDIA_BROWSER_ONLY` and `native_media_support(media_type)` answers `Unsupported { reason }` for them, which is the capability status a native host reports.

### What was written
- `F/🔨️modules/🖌️raster/🎥️video/🔓️decode/🦀️.rs`: `decode_avc_mp4(bytes, progress, cancelled) -> DecodedVideo` (MP4 box walk over `stsd/avcC`, `stts`, `stsz`, `stsc`, `stco`/`co64`; SPS, PPS and slice parsing for the encoder's subset; distinct pictures stored once, repeats share their reference; BT.601 limited-range inverse). Typed refusals `truncated`, `notMp4`, `noVideoTrack`, `malformed`, `unsupported`, `cancelled`; sample counts are bounded by the file size.
- New crate `semio-renderer-media-native` (`E/🎯️targets/🧊️wgpu/🎬️media-native`, sources in `E/🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/`):
  - `🔊️wav`: `decode_wav` (progress about every 64 Ki frames, cancel), `encode_wav_pcm16`, `DecodedAudio`.
  - `⏯️transport`: the TypeScript window kit's rules ported rule for rule (`media_key_command`, `seeked_media_position`, `trim_value_for_selection`, `format_media_time`) and `MediaTransport` (play, pause, toggle, seek, selection, clock advance, `Ended`).
  - `🎞️player`: `sniff_native_media`, `native_media_support`, `load_native_media` (progress, cancel), `NativeMediaPlayer` (video by clock and `video_frame()`, audio by `pull_audio(frames)` which advances the position by exactly the frames handed out and ends the transport at the last frame).

### Not wired (said plainly)
There is no native media-slot presenter in the winit app: `RenderSnapshot::with_media_slots` is consumed only by the browser host, and there is no audio sink to play into. So the native player is a library with laws, not yet a window the user sees. A presenter that composites `NativeVideoFrame` into the slot rectangle and feeds `pull_audio` to an OS sink is the next step; it needs a decision on the audio output API.

## 3. Laws (language-agnostic fixtures, round trips, progress and cancel)
| Law | Where | Third-party or pinned oracle |
|---|---|---|
| every pinned fixture MP4 (bytes the encoder is separately pinned to) decodes to its source pictures, frame count, size and duration | `🖌️raster/🎥️video/🔓️decode/🧪️tests` | the existing encoder fixture `🧪️tests/🧫️fixtures/🔣️.json` (`mp4Hex`, `yuv`) |
| encode, decode keeps a smooth picture within the 4:2:0 bound, shares repeats, keeps timing, reports progress per sample | same | pinned fixture |
| decode cancel, bad magic, truncation, hostile sample count refuse with typed codes | same | none needed |
| every text exists in en and de with equal placeholders; filled messages match React's | `🎥️video-render-host/🧪️tests` | `VIDEO_RENDER_EXPORT_TEXT_V1` |
| path verbs and `data:` URLs (base64 and percent) decode to the path and picture they hold | same | the in-repo PNG encoder round trip |
| the host admits only a plugin with the capability and a valid program, throttles progress, cancels once, settles every outcome | same | the kernel ledger fixtures `🎠️kernel/🧪️tests/🧫️fixtures/🧵️video-render-job` |
| render a program through wgpu, mux, decode: frames, shared pictures, duration, pixel colours of a fill, a half fill and a 2x2 picture; progress reaches the last frame; cancel stops; a missing picture fails with its index. A machine with no wgpu adapter answers `Gpu` and skips the pixel checks | same | decoder of the pinned fixture |
| WAV fixtures (Python `wave` writer, 8/16/24/32 bit PCM, float32/64, extensible, extra chunks) decode to pinned samples; A-law and µ-law match all 256 codes | `🎬️media-native/🧪️tests` | Python `wave` and `audioop` (fixture generator `🐍️wave3-media-fixture.py`) |
| PCM16 encode, decode is the identity; progress is monotonic and ends at the total; cancel stops | same | none needed |
| transport keys, seeks, cut ranges, clock text answer the window kit's own fixture | same | `🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/⏱️time-and-keys/🧫️fixture.json`, which the TypeScript twin also reads |
| the player plays WAV by blocks and first-party MP4 by clock, refuses everything else | same | none needed |

## 4. Files
Created: `F/🔨️modules/🖌️raster/🎥️video/🔓️decode/🦀️.rs`, `…/🔓️decode/🧪️tests/🦀️.rs`; crate `E/🎯️targets/🧊️wgpu/🎥️video-render-host/**` and `E/🧱️elements/🎥️VideoRenderHost/🎯️targets/🧊️wgpu/**`; crate `E/🎯️targets/🧊️wgpu/🎬️media-native/**` and `E/🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/**` (including `🧫️fixtures/🔣️.json`); `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎥️video-export/🦀️.rs`; `🐍️wave3-media-fixture.py` in this ticket.
Edited: `F/🔨️modules/🖌️raster/🦀️.rs` (image op), `F/🔨️modules/🖌️raster/🎥️video/🦀️.rs` (mounts `decode`), root `Cargo.toml` (two members), `E/🎯️targets/🧊️wgpu/📂️shell-doc-open/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml}`, shell staging: `📂️doc-open/🧬️component`, `🐙️state/🧩️session-and-topology`, `🪝️window-actions/⚙️settings-and-dock-persistence`, `🪀️view-panels/{🖼️chrome-frame-and-main-window,🎨️theme-file-import-and-accessibility,📤️icon-export}`, `🖐️pointer/{🖱️pointer-input/♿️access-and-routing,⌨️keyboard-and-palette/🔁️deferred-menus}`, `🔀️actions/🎬️shell-actions/🎯️dispatch-action`, `🚶️session/🧰️window-actions-and-commands`, and `E/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🎞️present-state-and-engine-closers`.

## 5. Verification state
NOTHING WAS COMPILED OR RUN. My only cargo run (`repo cargo ... test -p semio-framework-raster --lib`) sat in the shared queue for about 40 minutes at the manifest-synchronize phase and was cancelled on the coordinator's build-policy change. Everything above is WRITTEN, read back for type and borrow errors by hand, and unbuilt. Integration lane E must compile and run:

Crates to build (all new or touched):
- `semio-framework-raster` (image op, decode module): tests `video::decode::tests::*` (3 laws), existing `video::tests::*`, `tests::*`
- `semio-renderer-wgpu-video-render-host` (new): tests `every_text_has_both_languages_with_equal_placeholders`, `paths_and_picture_urls_decode_to_their_content`, `jobs_are_admitted_progressed_cancelled_and_settled_through_the_ledger`, `rendered_programs_round_trip_through_the_framework_encoder_and_decoder` (needs a wgpu adapter, else it returns early)
- `semio-renderer-media-native` (new): tests `fixture_wav_files_decode_to_their_pinned_samples`, `g711_tables_match_the_third_party_oracle_and_bad_files_are_refused`, `pcm16_round_trips_with_progress_and_cancellation`, `transport_rules_match_the_window_kit_fixture`, `transport_state_follows_the_clock`, `native_players_play_audio_and_first_party_video_and_refuse_the_rest`
- `semio-renderer-wgpu-shell-doc-open` (native and wasm32 check: new `🎥️video-export` file, new field), then its dependants `shell-window-actions`, `shell-view-panels`, `shell-session`, `shell-pointer`, `frame-presentation`

Likeliest compile risks to look at first: vello `ImageBrush`/`BrushRef` conversion in `🖌️raster/🦀️.rs`; `Send` of the render future in `ShellDetached::spawn` (SceneRasterizer, vello renderer); the G.711 table law (audioop scale) and bilinear exactness at texel centres in the picture-scene pixel check.
