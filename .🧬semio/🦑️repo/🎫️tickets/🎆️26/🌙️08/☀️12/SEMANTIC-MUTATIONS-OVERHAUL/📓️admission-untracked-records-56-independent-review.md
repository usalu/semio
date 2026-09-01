# Independent Review of Mounted Untracked Record Repair 56

No defect found in the bounded repair. This is a read-only review of the exact source delta and root-executed GREEN evidence; no tests or Git commands were rerun.

Current N is `34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f`, byte-identical to the retained GREEN source snapshot and stable across this review. The reviewed [GREEN receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/🧫️run-tOiY15/🔣️receipt.json) has SHA-256 `2b412e870428edc07ba9c63977ea9e311a979eb38c175b2b8fb206eaa6d68a4a`, all seven source endpoints stable, no harness failure, and 116/116 checks: 53 reference, 33 grammar, 12 physical, 12 marker, one walk, five isolated Git. Controller, schema, and vectors are unchanged from [final RED](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-records-56/🧫️run-KUccbG/🔣️receipt.json).

## Exact Source Scope

The full diff from RED N `0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce` contains only the four approved declaration changes:

- Fatal UTF-8 Git record decoding now preserves BOM.
- The old untracked helper is replaced by private marker-bearing rows, stripping exactly one transport slash before unchanged strict path validation.
- Directory-entry buffer decoding preserves BOM.
- The collector unions the marker with Boolean OR and validates the final nofollow observation before appending a candidate.

There is no compatibility alias or public path-schema relaxation. Of 542 named declarations, the other 538 are unchanged. Replacing just those four declaration spans with stable placeholders leaves an exactly byte-identical remainder, SHA-256 `3b1ce9076a255932b9b03c88a445069395cd9fea2e3ec263a363dce2cb9b0456`. This also covers comments/imports and all parser/reference-coordinate bytes outside those declarations.

Selected retained AST slices are unchanged:

| Declaration | Identical RED/GREEN SHA-256 |
| --- | --- |
| `sourceAdmissionSafePath` | `62dd9e376e3656cee2fe2bfae51f39174d15e9bebab7fbc5f1185338d089a9fd` |
| `sourceAdmissionGitExclusions` | `57ac49ba06eb202a05278ebc85190ba9f6970c116d1ede350af8a4e85e433755` |
| `sourceAdmissionGitRows` | `ecf128f030e211bfd0ed86a535fc9bf744d771703c8e16f15bb6d330d65fb430` |
| `gitRows` | `2761f2b13372c3c810cb4735cba5cc0f3cbf5b077c7e235a9f701408487d180e` |
| `referenceCoordinateRoots` | `6c93f963c3198b2548dd7e7ee9840c4df3aa74f14ab53e0c03ff3ffb590ae8ed` |
| `markdownInlineTokens` | `e46e25c624446761390a978c401bd1e5de65b056ccc1019878fb367d0446eec7` |
| `referenceTokens` | `90482f02edfe48acdbf68d333cc0c50ff4b5bdfd8c6c8fd9560b5104b9e54da9` |
| `frozenCoordinateEvidenceCoordinates` | `2d293a4884bdad653850af0ba5ca402e83aab6ffbf26f7278c13ea6d652266a5` |
| `frozenMarkdownCoordinateEvidenceCoordinates` | `a7d1a8897bf8f645b8d5ff4ffe9f0a43621488ff0f3f4555fe56d6272274e9e3` |

All 25 specifically checked admission/parser/coordinate declarations matched, including the observation, lstat, ancestry, pathspec, JSON coordinate, Markdown coordinate, and frozen coordinate authorities/caches. Exact call-expression texts and counts are also unchanged: `sourceAdmissionSafePath`: 11 → 11; `sourceAdmissionGitExclusions`: 2 → 2; `sourceAdmissionGitRows`: 1 → 1; `gitRows`: 2 → 2. This is source/caller identity evidence, not a claim that the intentionally changed record decoder has identical behavior.

## Reviewed Actual Outcomes

Each of the 12 marker cases records exactly one actual observation-boundary probe. Marked directory and ordinary unmarked file succeed. File, executable, symlink, absent, nonregular, and unsafe-ancestor targets throw the marker-mismatch error after the probe; they do not fabricate a rejected result or bypass observation.

The tracked-plus-untracked directory retains both origins and its exact stage-zero index entry. A tracked-origin union and both marked/unmarked duplicate orders cannot erase the marker: file observations still reject. The source uses `row.directoryMarker ||= directoryMarker` before any final observation.

First and later Git records now preserve the same U+FEFF spelling for files and directories, with actual `{path,directoryMarker}` results. The separate buffer-name walk returns `["owned","owned/\ufefffile.txt"]`, with one directory-buffer read and the expected three stat-boundary calls. These virtual boundary probes remain distinct from real filesystem execution.

The real isolated Git stdout remains the same exact 42 bytes, SHA-256 `2c14fa5e5bf2a8a82234c5e7281b6b29b1bc9d50c1cb52a2d39c4a8f16d90a5c`. The unmodified public wrapper now completes with exactly `cafe\u0301-nested`, `ordinary.txt`, and `taxonomy.json`. The nested path is an explicit `040000` directory with nonignored-untracked origin; no nested descendants are admitted. Its taxonomy hash is `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce`. The existing nofollow observation/ancestry code is unchanged.

## Boundary

This review does not replace the separately running canonical IO regression and does not claim a complete real-workspace roster, parser runtime regression suite, content census, or native build. No production, test, controller, fixture, launch, or seed edits were made; only this review Markdown was authored.

