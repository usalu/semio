# WAV Natural Sample And Selection Editing Audit

## Current capability

The WAV editor already has a bounded scalar storage primitive. A generic Details edit at `/data/value/<sample>` is lowered to `WavMutation::PatchData`, and the retained publication path prepares forward, inverse, and post state without cloning the entire sample vector in the first step. The editor also exposes the generic snapshot metadata tree and the `MediaWindowKit` body.

This does not provide an audio editing experience. The main window currently renders a media placeholder with `duration_ms = 0` and `position_ms = 0`. It has no sample rows, frame or channel coordinates, selection state, direct sample value controls, structural frame actions, audio format controls, or keyboard actions. The mode declares no commands. The only natural mutation path is a single flat sample addressed through the generic Details JSON pointer.

`WavData` is interleaved while channel count lives in `WavFmt`. The existing scalar edit path does not guard the relationship between them. In particular, a generic insertion or removal can leave a typed sample vector whose length is not divisible by `fmt.channels`, and a channel-count edit can leave `block_align`, `byte_rate`, and the interleaving interpretation inconsistent. `Raw` data cannot be safely interpreted as typed sample frames and must remain available through Details while the natural frame editor reports that limitation explicitly.

## Required behavior

The scoped implementation will add a schema-native audio edit command surface that:

- addresses typed samples as `(frame, channel)` rather than a flat array offset;
- edits a selected sample value with numeric range validation for PCM8, PCM16, and Float32;
- inserts and removes the selected frame atomically by lowering them to bounded `PatchData` mutations;
- exposes guarded format edits for sample rate and channel layout, preserving coherent `channels`, `block_align`, and `byte_rate` values;
- provides a windowed, editable frame table with localized English and German labels;
- publishes keyboard-reachable add/remove actions and revision guards;
- preserves undo/redo through exact inverse mutations and retained publication preparation;
- rejects natural frame editing for `Raw` data instead of assigning it an invented sample interpretation.

Channel insertion/removal needs a strided interleaved transformation and coordinated format update. It will use an artifact-owned schema mutation and bounded retained preparation rather than a whole-snapshot generic edit. If the currently stable framework cannot express this without changing shared APIs, the implementation will keep the shared APIs stable and record the exact remaining bound rather than silently clone large sample buffers.

## Owned files

This slice owns only WAV-local files and this ticket report:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/**`
- WAV-local editor tests and neutral fixtures under the same subset
- WAV-local mutation schema/facets/tests only if channel editing requires a new persistent event
- `📓️wav-serialization-state-and-zip-preparation-2026-09-27.md` and this audit

The slice will not edit the shared Table, Details, retained-copy/retained-clone foundation, OPC implementation/tests, ZIP implementation, or global framework APIs. It will reuse their current public contracts.

## Immediate fidelity checkpoint

No newer WAV or PNG source defect was found ahead of this UI slice. The previously observed WAV native failure was a contradictory neutral fixture (`audio_format = 0xfffe` with `WavData::Pcm8`); that fixture has been corrected to PCM format `1`, but the native WAV suite has not been rerun after the correction. The PNG DCI 4K ceiling source correction likewise remains native-unverified. These facts must stay explicit until their focused native gates actually run.

## Implemented source checkpoint

The WAV main surface now uses two complete bounded projections. Documents with up to eight channels render a windowed frame matrix. Wider documents render a windowed coordinate list with explicit frame, channel, and value columns, so later channels remain reachable without admitting every header or cell. Separate windowed frame and channel control lists keep insert/remove actions addressable for the complete document. Every editable value or structural action carries its true frame/channel coordinate and the current canonical document revision.

A WAV-owned English/German toolbar exposes append-frame and append-channel controls; a rendered format table exposes sample-rate editing; addressed row/channel buttons expose insert/remove. Every sample and sample-rate editor is wired as a native input, and every structural action as a native button, using the host control types that define focus and input-commit or button Enter/Space semantics; their authored bindings carry the current document revision. Browser/native runtime dispatch has not been observed, so this is a source-wiring result rather than an interaction pass. Every revision-bearing action is excluded from the raw command palette. The six provisional app-global chords were removed after source audit: the current host opens a staged action form for any chord whose action has arguments, and that form would expose the internal revision token and would not inherit the focused row/channel binding. A direct selected-frame/channel shortcut remains unimplemented until the shared host can dispatch the focused control's bound arguments; no shortcut pass is claimed. Raw payloads remain visible as bounded read-only bytes with an explicit localized explanation that their frame layout is unknown.

The schema-native `EditAudio` command covers selected sample replacement, append/insert/remove frame, append/insert/remove channel, and sample-rate edits. Its retained worker validates revision, sample type/range, interleaving divisibility, format/data agreement, positive channel count, and checked `blockAlign`/`byteRate` arithmetic before emitting. Frame edits are divided into at most 16,384-byte `PatchData` pages; a two-page wide-frame removal test proves every page removes at the same logical index and that forward and inverse states are exact. Channel edits rewrite interleaved frames from the end in the same payload-bounded pages, then publish a coherent `SetFmt`; the inverse sequence restores the exact source. A channel transformation is refused when even one frame exceeds the retained patch envelope. The retained plan admits at most 128 mutations, so channel transformations that cannot fit 127 payload pages plus the coherent `SetFmt` are explicitly refused as too large. Format extension bytes, whose schema ceiling is 65,535 bytes, are copied in 16,384-byte pages with `RetainedBytesCopy` before mutation construction, with localized progress and byte-grant-aware cancellation retirement; the planning state stores only scalar derived fields.

A neutral JSON fixture is shared by Rust and TypeScript tests. The Rust oracle opens the edited native WAVE through the independent `riff` crate and checks channel count, block alignment, byte rate, and interleaved data bytes. A separate retained-work test covers localized progress, stale-revision refusal, incremental close, and source immutability, and a command-codec test covers revision/selection preservation.

The action roster audit covers all eight natural commands: set sample; append, insert, and remove frame; append, insert, and remove channel; and set sample rate. Each occurs exactly once in the window definition, command roster, retained factory, publication contract, dispatcher, and rendered revision-bound native control. An authored Rust law enforces the definition/control/revision join and also requires the window definition to have no global chord that could expose a revision form. It has not run because of the shared native compile stop.

`NX_DAEMON=false bun nx test @semio-tech/stdio-wav --skip-nx-cache` passed after the final TypeScript source changes with 9 tests, 0 failures, and 31 assertions. Rust tests now also cover exact action-address decoding, late-channel window bindings, absence of raw-revision global chords, one-to-one rendered control coverage, a too-wide-frame refusal, a multi-page wide-frame removal, large `fmt.ext` paged completion/cancellation, and an odd extension with nonzero pad through edit/save/reopen/undo. `NX_DAEMON=false bun nx test @semio-tech/stdio-wav-rs --skip-nx-cache` was attempted before the final chord-removal test change, but it stopped before compiling WAV or running tests because the current shared `semio-framework-os-kernel` dependency has 30 `RetainedClone` trait/close compile errors. The exact terminal result is captured in `🗑️generated/wav-native-current-8.log`; no native result is claimed for the new command surface.
