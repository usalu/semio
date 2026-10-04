# CSV Browser Archive Reload

## Failure classification

The preview32 reload failure came from a mixed frontend/native checkpoint. The browser reported `AppChannelClient.loadDocumentArchive(s.stdio.csv@rfc4180/*#editor): no answer to seq 11 of operation 9`. That sequence is the terminal `AcknowledgeDocumentArchiveLoad`: admission uses sequence 9, the ready poll uses sequence 10, and acknowledgement uses sequence 11.

Preview32 deliberately reused preview31's native activation. `stdio-csv-preview-current-31.log` finished at 19:56:58, while the current native dispatcher source `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` was modified at 20:12:50. The later source explicitly handles `AcknowledgeDocumentArchiveLoad` and returns `Done { in_reply_to: seq }`. Preview33 rebuilt the activation at 20:57:20 with artifact digest `5fbedc953a061837377c0640d5c9dc41fd117a1f675db0c855277a0d40b1f432` and source digest `08cfe5f55cfa5eaf94d8bf4ffa1761694f628f4b3f94f98997f6046931b7de8d`. No preserved preview31 receipt records its old channel digest, so the timestamp/order and the exact missing acknowledgement are the available stale-activation evidence.

The current TypeScript and Rust protocol sources agree on channel version 20 and tags 32–36 for load/read/poll/cancel/acknowledge plus frame tag 27 for `DocumentArchiveLoad`. The current native dispatcher replies to admission, cancellation, and acknowledgement with a sequence-correlated `Done`, and replies to polling with `DocumentArchiveLoad`.

## Added acceptance laws

The language-neutral archive-host fixture now contains `a-browser-reload-admits-as-operation-nine-polls-on-sequence-ten-and-acknowledges-on-sequence-eleven`. It requires the exact command sequence and ready outcome observed by the browser reload path. Both the Rust and TypeScript archive hosts consume this corpus.

The CSV native editor suite adds `browser_archive_reload_round_trips_the_saved_csv_through_the_exact_stepped_wire`. It saves a real CSV edit, makes a later unsaved edit, encodes and decodes each sequence 9/10/11 command through the retained guest command decoder, dispatches it through the real `PluginApp` archive lifecycle, round-trips every reply through the frame codec, verifies the saved snapshot and archive are restored, and verifies acknowledgement retires operation 9.

## Validation

- `NX_DAEMON=false SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated bun nx run @semio-tech/framework-os:test-channel-oracles --skip-nx-cache`: **3/3 passed**, 242 expectations. Receipt: `🗑️generated/csv-archive-reload-channel-oracle-ts-1.log`.
- The first CSV native selection compiled current source but selected zero tests because its full bare positional filter did not match. Receipt retained as diagnostic evidence: `🗑️generated/csv-archive-reload-native-1.log`.
- The first filter-expression retry was rejected by the generated shell command before Cargo because unescaped parentheses reached `/bin/sh`. It ran no native code. Receipt: `🗑️generated/csv-archive-reload-native-2.log`.
- A shell-safe bare-filter retry compiled the complete current CSV test binary but still selected zero tests. Its retained Nextest metadata proves the binary was current; the selection route did not expose the generated fully qualified name. Receipt: `🗑️generated/csv-archive-reload-native-3.log`. This is not reported as a passing native law.

## Runtime diagnostic

The temporary archive metadata trace logged only command name, sequence, operation, returned frame kind, and `inReplyTo`; it never logged document bytes or any raw payload. It was removed immediately after preview33 supplied the exact receipt.

## Preview33 runtime receipt

The fresh coherent preview33 archive restore passed at 19:02:46 local time:

- `LoadDocumentArchive`, sequence 9, operation 9 returned `Done(inReplyTo: 9)` plus the expected ephemeral frame.
- Poll sequences 11 through 23 each returned `DocumentArchiveLoad` with matching `inReplyTo`, plus ephemeral state.
- `AcknowledgeDocumentArchiveLoad`, sequence 24, operation 9 returned `Done(inReplyTo: 24)` plus ephemeral state.

The full CSV grid and Details remounted, fresh browser warning/error logs were empty, and a post-reload edit `Receipt33` published in both surfaces. This proves current-source archive transfer and continued editability. The next ordinary reload selected the Demo example and reset the first field to `alpha`, so it is not a persisted user-file save/reopen witness.

## End-user file transfer frontier

Every app already receives schema-owned `Export Document` / `Import Document…` actions. The shell intercepts them: export reads the focused program's complete `DocumentArchivePack` and downloads `<kind>-<UTC>.semio-archive`; import picks that media type, creates a new instance of the same program, drives the cancellable archive load with Tasks progress, and focuses the new window. The repository's browser I/O matrix specifies export → import into a new window → re-export → byte-identical comparison in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚪️io-matrix/🟦️.ts`.

A preview33 user-flow check with an edited CSV archive is pending browser file-download/file-picker evidence. Until that check, current-source archive restore is accepted, while end-user exported-file reopen remains open.

## Natural-file Open/Save audit

The document archive is a Semio session carrier, not a natural artifact file. The existing
`ArtifactPack` and `ArtifactDsl` names also do not imply natural bytes: CSV and PNG
`ArtifactPack` implementations wrap their payloads in the Semio binary envelope. A natural-file
route must therefore use an explicit codec owned by the editor's declared format.

The production transport already has the required raw-byte channel shapes:

- `MediaOut` returns descriptor plus raw bytes.
- `MediaIn` accepts descriptor plus raw bytes.
- the shell already owns file picking, downloads, Tasks progress, cancellation, and new-instance
  creation for archive import.

The missing path is the explicit bridge between those facilities and the editor's natural codec:

1. a schema-owned natural-file descriptor on the app I/O declaration;
2. editor methods that encode/decode the natural representation without an `ArtifactPack`
   fallback;
3. an `artifact:native` media route;
4. browser handle ingress for `MediaIn`;
5. shell Open File / Save File actions. Open creates a fresh app instance and destroys it on
   refusal or cancellation, leaving the focused document unchanged.

The representative acceptance witnesses are RFC 4180 CSV parsed independently as rows, PNG checked
by signature/chunk parser (and Pillow where the registered oracle already uses it), and DOCX checked
as an OPC ZIP with required content-types and document parts. These witnesses distinguish actual
natural bytes from the internal pack envelope.
