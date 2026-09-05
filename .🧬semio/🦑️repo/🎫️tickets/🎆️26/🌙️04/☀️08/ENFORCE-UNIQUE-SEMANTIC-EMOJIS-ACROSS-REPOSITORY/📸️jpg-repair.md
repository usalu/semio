# JPEG/JFIF Emoji Repair

## Scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg` plus the six exact Raster, Semio-image, and Shooting JPEG import/export carrier roots.

## Baseline

- Scoped audit: 409 files, 284 directories, 686 governed nodes.
- Findings: 75 missing identities, 2 generic identities, 2 presentation errors, 19 sibling duplicates.
- The scoped audit does not include the artifact root itself; manual sibling review found `📷️jpg` colliding with `📷️png` in the main artifact collection and the carrier collections.
- A pre-existing Rust `target` tree exists under the JPEG generator. It is excluded from source naming authority and was not modified or used as a destination.

## Handpicked identities

- `📷️jpg` → `📸️jpg`: photographic JPEG image data, distinct from sibling PNG.
- `✳️baseline` → `🧱️baseline`; `✳️document` → `🧾️document`.
- `🧪️oracle` → `🔮️oracle`; window `🎚️options` → `☑️options` while `🎚️config` remains configuration.
- Baseline operations: `🧩️insert-frame-component`, `✂️remove-frame-component`, `🌳️insert-huffman-table`, `🪓️remove-huffman-table`, `➗️set-arithmetic`, `🎨️set-component-sampling`, `🎯️set-sample-precision`, `📸️set-snapshot`, and `🏁️set-sof-marker`.
- Document operations retain fitting unique identities except `📐️change-jfif-header` → `🪪️change-jfif-header`, `📤️remove-quant-table` → `🧹️remove-quant-table`, and generic `🟪️replace-pixels` → `🔲️replace-pixels`.
- JSON Schema sidecars use `🧬️.schema.json`, distinct from canonical JSON `🔣️.json`.
- Generic direct-behavior test groups use `🎯️direct-behavior`; the two document suite identities are distinguished as `📜️direct-mutation-contract` and `🛡️mutation-regressions`.
- Fixture directories use individual semantic identities recorded in the move log below. Paired JPEG leaves use `⬅️before.jpg` and `➡️after.jpg`.
- Generator scan files use number-specific `3️⃣three-tables.scan` and `4️⃣four-tables.scan` identities.

## Move log

- Main and carrier artifact directories: seven `📷️jpg` roots became `📸️jpg`; each of the six carrier `✳️any` leaves became `♾️any`.
- Main subsets: `✳️baseline` became `🧱️baseline`; `✳️document` became `🧾️document`.
- Baseline operation directories: `📐insert-frame-component→🧩️insert-frame-component`, `📏remove-frame-component→✂️remove-frame-component`, `🔖insert-huffman-table→🌳️insert-huffman-table`, `🏷remove-huffman-table→🪓️remove-huffman-table`, `🧩set-arithmetic→➗️set-arithmetic`, `🧮set-component-sampling→🎨️set-component-sampling`, `⚙set-sample-precision→🎯️set-sample-precision`, `🔧set-snapshot→📸️set-snapshot`, and `🔩set-sof-marker→🏁️set-sof-marker`.
- Document operation directories: `📐️change-jfif-header→🪪️change-jfif-header`, `📤️remove-quant-table→🧹️remove-quant-table`, and `🟪️replace-pixels→🔲️replace-pixels`.
- Baseline fixture directories use the matching operation identities, and `🧪️abbau-aufbau-masterarbeit-grundriss` became `🏘️abbau-aufbau-masterarbeit-grundriss`.
- Document fixture directories: `🔎️change-jfif-header`, `🪪️change-jfif-header-applied`, `🎚️change-re-encode-quality-applied`, `⏱️change-restart-interval`, `🔁️change-restart-interval-applied`, `📥️insert-other-segment-applied`, `🌲️remove-huffman-table`, `🪓️remove-huffman-table-applied`, `🗑️remove-other-segment-applied`, `📉️remove-quant-table`, `🧹️remove-quant-table-applied`, `🔀️replace-huffman-table`, `🌳️replace-huffman-table-applied`, `🔲️replace-pixels-applied`, `📈️replace-quant-table`, `📊️replace-quant-table-applied`, and `🏘️abbau-aufbau-masterarbeit-grundriss`.
- All 25 fixture pairs now use `⬅️before.jpg` and `➡️after.jpg`. Ten document JSON Schema sidecars became `🧬️.schema.json`; ten direct test directories became `🎯️direct-behavior`.
- Four editor window option directories became `☑️options`; both oracle directories became `🔮️oracle`; the document suite directories became `📜️direct-mutation-contract` and `🛡️mutation-regressions`; the baseline mutation case became `🛡️mutate-jpg-jfif-1-01-baseline`.
- The scan scripts became `3️⃣three-tables.scan` and `4️⃣four-tables.scan`.

## Reference repairs

- Updated both oracle manifests, all 19 mutation descriptors, Rust/TypeScript/GraphQL/JSON mutation aggregates, direct-behavior includes, editor mounts, artifact registries, native codec factories, Stdio Rust and TypeScript barrels, the Stdio oracle barrel, and all six Raster/Semio/Shooting carrier mounts.
- The document generator now carries an explicit reviewed semantic-id-to-directory-name table. Its Rust codec writes the selected directory and the `⬅️before.jpg` / `➡️after.jpg` leaves directly; marker and libjpeg modes have their own explicit mappings. No emoji is derived from a hash, palette, ordering, or slug.
- Repaired three corrupted Rust expressions where an earlier path replacement had changed the local `jpg` slice into an illegal `📸️jpg` identifier.
- Updated current documentation paths, the main artifact definition, generator commands, probe commands, scan references, and the direct mutation regression's real example-asset coordinate.
- Shared policy, dependency, root-script, MIME, OS, and taxonomy references were delegated to the parent. The parent added exact `🔮️oracle` overrides for both baseline and document owners; central taxonomy validation remained clean.

## Verification

- Strict scoped audit: 409 files, 284 directories, 686 governed nodes; every missing, generic, presentation, spacing, duplicate, multiple, reserved-name, and oracle count is zero.
- All 94 JSON files outside the pre-existing generator `target` parse.
- Canonical fixture verifier: `s.stdio.jpg` reports 24 registered fixtures and zero file problems.
- Standalone Rust JPEG/JFIF codec: 5 tests passed.
- Generator `generate`: 10/10 recipes succeeded; `markers` and `libjpeg` modes succeeded. The generated 16 semantic fixture pairs are byte-identical to the committed pairs; the only source-only directory is the expected architectural reference asset.
- Focused Stdio native JPEG filter reached the Stdio crate and exited 1. Its first diagnostic was an emoji-bearing `📸️jpg` identifier from the earlier unsafe replacement snapshot, followed by the independently known `SemioBrepEditCommand::decode_op` and broad generated `Serialize`/`Deserialize` failures across MP4, MP3, HTML, TSV, Binary, AVI, EPW, Kit, and other unrelated artifacts. A fresh current-source scan finds no `📸` occurrence outside strings, comments, or path attributes and no emoji-bearing Rust identifier; the three damaged local JPEG variables are repaired. The native aggregate remains blocked by the unrelated generated-code failures, while the isolated JPEG codec and canonical artifact verifier are green.

## Remaining issues

- The baseline mutation manifest retains its pre-existing `🔣️.schema.json` declarations even though that subset has no JSON Schema sidecars. The canonical artifact verifier accepts the current contract. No schema files were invented and no requirement was weakened during an emoji-only repair.
