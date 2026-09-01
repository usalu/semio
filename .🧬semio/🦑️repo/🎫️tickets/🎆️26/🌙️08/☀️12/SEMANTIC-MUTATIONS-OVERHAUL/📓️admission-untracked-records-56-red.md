# Git Untracked Record Grammar 56: Reviewed RED Packet

## Outcome

The final pre-splice run is genuinely RED against unchanged production source, without a harness failure: [run-KUccbG receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/🧫️run-KUccbG/🔣️receipt.json). All seven captured source endpoints match exactly. The bounded child completed in 1.116 seconds; no timeout or signal occurred.

| Independently Reported Group | Passed/Executed |
| --- | --- |
| reference | 53/53 |
| grammar | 22/33 |
| physical | 12/12 |
| marker | 1/12 |
| walk | 0/1 |
| git | 1/5 |

The independent reference gate is GREEN; the actual helper, collector, walk, and isolated-wrapper results remain separate. Total checks are 89/116. The result is not a complete workspace roster, content census, or native build.

## Authored Boundary

Only these new ticket-owned inputs were authored:

- [Controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/📜️script.ts).
- [Closed neutral schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/🧬️schema/🔣️.json).
- [Handcrafted neutral vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/🔣️vectors.json).

The roster contains 33 grammar cases, 12 unchanged strict physical-path cases, 12 marker-versus-final-observation cases, one separate buffer-name walk case, and one real isolated Git fixture specification. The two later-record BOM cases were appended after review; every preceding authored vector remains unchanged.

The schema requires private rows `{path,directoryMarker}`. Exactly one final slash is transport metadata, not part of the physical path. The physical schema still rejects empty, slash-only, dot/dot-dot, repeated-slash, drive-prefixed, backslash, control, and lone-surrogate paths. Framing cases reject missing terminal NUL, empty NUL records, invalid/overlong/truncated UTF-8, and UTF-8 surrogate encodings.

Strict Ajv validates the actual schema/vector files and five closed-schema counterexamples. jsonc-parser is checked against strict JSON parsing. The independent transport reference uses iconv-lite with BOM retention, exact byte roundtrip comparison, and the Ajv physical-path schema. It is not a second admission implementation.

## Actual Source Evidence

The controller extracts exact current declarations through the TypeScript AST. Before the source splice it selects `sourceAdmissionUntrackedPaths`; after an authored replacement it will select `sourceAdmissionUntrackedRows`. This is explicit test-subject selection: returned strings are never converted into marker objects.

All 21 malformed framing/path grammar cases reject, and the empty stream accepts. The other 11 grammar cases are RED because the current helper cannot represent marker facts and/or rejects a terminal directory marker. Ordinary file, trailing-space, literal replacement-character, and later-record BOM file outputs retain their byte spelling, but still return the old string-array shape; their failures are not mislabeled as file-spelling regressions.

The BOM distinction is observed directly:

- First record `efbbbf666f6f00` returns `"foo"`, losing U+FEFF.
- Later record `612e74787400efbbbf666f6f00` returns `["a.txt","\ufefffoo"]`; the same U+FEFF is retained after the first record.
- The actual walk closure receives `efbbbf66696c652e747874` from one injected directory-buffer read and returns `owned/file.txt`, instead of `owned/\ufefffile.txt`. Its stat responses are virtual; no real candidate traversal is credited.

The 12 marker cases execute the actual collector and actual observation function, injecting only the specified Git records, index rows, and stat boundary. Every marked case currently fails before observation and records zero probes. Those rejected cases remain RED because the law requires exactly one final observation; catching the parser error does not synthesize success. The ordinary unmarked file case observes exactly once and passes.

The cases freeze directory success, file/executable/symlink/absent/nonregular/unsafe-ancestor rejection, tracked-origin union, and both marked/unmarked duplicate orders. The expected directory fact must survive every origin merge.

## Actual Isolated Parent and Nested Repository

A fresh parent repository and a committed nested repository were created only beneath `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/🧫️run-KUccbG/🧪️fixture`. Git configuration and templates are isolated within that run; hooks/signing and inherited repository-location/configuration variables cannot redirect writes. No shared repository index, objects, config, cache, or evidence were modified or removed.

The actual independent `git ls-files --others --exclude-standard -z -- .` call returned these exact three records:

```json
[
  "café-nested/",
  "ordinary.txt",
  "taxonomy.json"
]
```

Exact 42-byte stdout:

```text
63616665cc812d6e65737465642f006f7264696e6172792e747874007461786f6e6f6d792e6a736f6e00
```

SHA-256: `2c14fa5e5bf2a8a82234c5e7281b6b29b1bc9d50c1cb52a2d39c4a8f16d90a5c`.

The unmodified public `inventoryTaxonomySources` wrapper then consumed the copied actual current taxonomy and the isolated repository. It threw `Git untracked output has an invalid source path` at N2766 before observation. Consequently no claim is made that the current wrapper already observes the directory, excludes nested content successfully, or produces an inventory/hash. Those four assertions are correctly RED. They will require a complete strict-schema result, exactly the three authored paths, the final nofollow directory identity, no nested descendant candidates, and the actual taxonomy content hash after the source repair.

The controller records seven explicit fixture Git setup/reference calls. The unmodified wrapper performs its own internal Git calls; they are not counted as seven or represented as separately intercepted raw output.

## Minimal Parent-Owned Source Proposal

The source boundaries below refer to the captured N fingerprint, not a guessed future line layout.

1. N2739–2745, `sourceAdmissionGitRecords`: retain strict NUL and fatal UTF-8 validation; preserve leading U+FEFF with `ignoreBOM:true`. No generic trim, normalization, or lossy replacement.
2. N2761–2768, replace `sourceAdmissionUntrackedPaths` with `sourceAdmissionUntrackedRows`: after strict framing, derive one terminal-marker fact, remove exactly that one transport slash, and validate the resulting path using the unchanged `sourceAdmissionSafePath`. Return internal `{path,directoryMarker}` rows, retaining raw Unicode spelling and byte ordering. No compatibility alias.
3. N2813–2852, `collectTaxonomySourceAdmission`: retain the marker in its private candidate union using Boolean OR across all additions. Observe the final candidate once through the existing nofollow observation path. Before admitting a marked candidate, reject any non-directory or unsafe-ancestor observation. Do not follow symlinks, recursively admit nested repository contents, or convert missing marked directories into successful absent/file admission.
4. N2770–2796, `sourceAdmissionWalk`: preserve the BOM of each decoded directory-entry byte buffer with fatal decoding plus `ignoreBOM:true`; the independent virtual walk RED establishes this exact change.
5. Root owns the two existing canonical IO helper-name joins. Preserve their case counts and semantics while removing the old name; no public source-admission schema change is required.

The strict path predicate is frozen at SHA-256 `62dd9e376e3656cee2fe2bfae51f39174d15e9bebab7fbc5f1185338d089a9fd`. N, D, S, P, canonical IO files, and launch/seed were not edited by this packet.

## Preserved Execution History

- `🧫️reference-QFS0Gu`: original schema/reference GREEN51/51.
- `🧫️run-VW4niB`: actual partial RED; reference51/51, grammar22/31, physical12/12, then a ticket-controller null-result assertion bug. Marker, walk, and Git groups were not executed. First controller fingerprint `9df406549febf3f1eab7721118e064ece4422c28d36a0dc1cc7b23171f0f9582` is retained in its original snapshot.
- `🧫️run-y6ppvv`: null guard corrected; complete actual RED against the original 31 grammar cases. All endpoints stable. This run had already launched when the later-record refinement arrived.
- `🧫️reference-JwmPH5`: reviewed two later-record vectors appended; independent reference GREEN53/53.
- `🧫️run-KUccbG`: final full pre-splice RED described above, with no harness failure and all endpoints stable.

Every run retains complete JSON, source snapshots, raw child output, and a unique complete Markdown sibling directly under the ticket. The earlier failures were not overwritten or reconstructed. The final complete sibling is [retained receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56-actual-2026-08-27T23-11-57-714Z-🧫️run-KUccbG.md).

Final receipt SHA-256: `c95624232cf7ec8ad55002a5fbebafb44887cf8ce5de41df1469d77c9a02e41a`.

## Frozen First/Final Source Hashes

Every listed hash matches both endpoints of the final run.

| Input | SHA-256 |
| --- | --- |
| controller | `ee0f7f66334320359e1ba854a8ace3ae0831ca2f12743c4576ae9f4fc6830705` |
| schema | `f9491c20a895d6c2902e7588fff6c11d653b15d2ca86bb5472db607f2ff21296` |
| vectors | `7a0d24032fab720bc00f0880954ff7931fd88ed3b614608c32c25e79fbe302f3` |
| N | `0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce` |
| D | `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423` |
| taxonomy | `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce` |
| admissionSchema | `ed117e588c2aa2e1a0622455ab4710cec623725dce829af6ddb4a5f6328bb1a5` |

The final controller differs from the original captured controller only by the reviewed null-result guard. The vector extension changes original vector SHA-256 `933605f7460c91fe8a41ec10820a129f83d0262166bac4348a43caaccf394f51` to the final hash above; original snapshots and receipts remain intact.

The bounded RED packet is complete and frozen for root review/source implementation. No full real-workspace roster rerun is authorized or performed here.

