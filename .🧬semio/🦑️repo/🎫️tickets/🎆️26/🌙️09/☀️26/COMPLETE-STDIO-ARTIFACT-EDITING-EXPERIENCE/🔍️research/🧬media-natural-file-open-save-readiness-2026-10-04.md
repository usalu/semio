# Media Natural File Open/Save Readiness — 2026-10-04

## Scope

This cut covers the six editor surfaces requested across five media families: WAV RIFF PCM, JPEG JFIF document, JPEG baseline, MP3 MPEG-1 Layer III, MP4 ISO-BMFF, and AVI 1.0. The shared natural-file transport already carries raw `DslValue::Bytes`; the shared `ArtifactEditor::import_media` correction is owned by the bounded-ownership lane.

## Readiness matrix

| Surface | App id | Natural kind / MIME | Real subject codec | Exact admission gate | Independent oracle | Missing before this cut |
|---|---|---|---|---|---|---|
| WAV | `s.stdio.wav@riff-pcm/*#editor` | `s.stdio.wav@riff-pcm` / `audio/wav` | `decode_wav` + fallible `try_encode_wav` | RIFF/WAVE parse, `validate_wav_serialization`, exact source re-encode | `riff` 2.0 | editor codec declaration, whole-document event, registered fresh-owner lifecycle |
| JPEG document | `s.stdio.jpg@jfif-1.01/*#editor` | `s.stdio.jpg@jfif-1.01` / `image/jpeg` | `decode_jpg` + `encode_jpg` | complete JFIF document parser/encoder and terminal EOI ownership | `image` 0.25 JPEG | editor codec declaration, whole-document event, registered fresh-owner lifecycle |
| JPEG baseline | `s.stdio.jpg@jfif-1.01/baseline#editor` | `s.stdio.jpg@jfif-1.01` / `image/jpeg` | shared JFIF codec | terminal EOI ownership plus hard-error-free `check_baseline_conformance` after decode and before encode | `image` 0.25 JPEG | paired declaration and enforced baseline boundary on both directions |
| MP3 | `s.stdio.mp3@mpeg1-layer3/*#editor` | `s.stdio.mp3@mpeg1-layer3` / `audio/mpeg` | `decode_mp3` + `encode_mp3` | ID3/MPEG frame parse, non-empty frame set, exact source re-encode | `id3` 1.17 plus independent MPEG frame walker | editor codec declaration, whole-document event, registered fresh-owner lifecycle |
| MP4 | `s.stdio.mp4@isobmff/*#editor` | `s.stdio.mp4@isobmff` / `video/mp4` | `decode_mp4` + `encode_mp4` | mandatory `ftyp`/`moov`, typed tracks/sample tables, exact canonical source re-encode | `mp4` 0.14 | editor codec declaration, whole-document event, registered fresh-owner lifecycle |
| AVI | `s.stdio.avi@1.0/*#editor` | `s.stdio.avi@1.0` / `video/x-msvideo` | `decode_avi` + `encode_avi` | RIFF/AVI parse, mandatory `avih`, stream/header ownership, exact source re-encode | `riff` 2.0 plus independent AVI codec | editor codec declaration, whole-document event, registered fresh-owner lifecycle |

## Implementation contract

The shared neutral matrix grows from 22 to 28 rows. Every editor must expose a paired codec and one `SetSnapshot` whole-document operation. Tests must enter through `PluginApp::consume_media`, settle the import, save through `PluginApp::produce_media`, validate the saved bytes with the independent oracle, reopen them in a separately bound app owner, and prove Undo/Redo isolation between the source and reopened histories. Direct codec round trips remain supporting evidence only.

All fallible validation happens before publication or output. JPEG baseline refuses any hard conformance diagnostic in both directions. The container formats reparse their freshly encoded bytes so a structurally invalid typed snapshot cannot be published as a successful Save.

## Exact admission finding

The WAV walker reads complete chunks until fewer than eight bytes remain, the MP3 walker sync-scans forward, and the AVI walker trusts the declared RIFF extent. Their raw decoders are intentionally useful for format analysis, but a natural editor route cannot accept bytes the snapshot does not own because a later Save would silently discard them. MP4 similarly rebuilds a canonical ISO-BMFF layout and only models declared boxes and metadata. WAV, MP3, MP4, and AVI natural imports therefore require `encode(decode(source)) == source`. JPEG remains semantic because decoding and re-encoding compressed pixels is intentionally lossy, so its natural routes instead require the EOI marker to own the physical end of the source; the baseline editor additionally applies the hard baseline conformance gate. Every registered route law appends an otherwise ignored octet, accepts refusal either during dispatch or bounded settlement, and proves the owner snapshot is unchanged before accepting the exact fixture.

## Validation receipts

- `🗑️generated/media-natural-file-schema-green-1.log`: the shared 28-row schema, exact codec declarations, file naming, raw binary descriptor matching, fresh owner creation/retirement, progress, cancellation, and read bounds pass in the independent Ajv/browser law (1 selected passed, 699 skipped).
- Native per-family registered route and independent decoder receipts remain queued behind the coordinated PDF native job.
