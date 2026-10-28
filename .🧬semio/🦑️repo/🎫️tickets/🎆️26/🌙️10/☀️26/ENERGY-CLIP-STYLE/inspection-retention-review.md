# Inspection Retention Review

Read-only review of inspection-retain.mjs against inspection-chapter-verify.py and the current canonical manifest. No publisher, verifier, or retention process was started.

## Concrete Finding

The 93 copied sidecars (55 render certificates and 38 chapter clocks) lack destination SHA-256 verification. Movies verify source and destination against independently recorded media hashes; images verify source against destination. Sidecars use copyFileSync only. The before/after provenance proof rechecks original sidecars, not retained copies. Therefore the claimed exact-hash retention guarantee is incomplete for sidecars. Use the same source/destination checked copy for all artifacts before the staging rename. Root notified.

## Contract Checks

The scalar verifier emits the fields used by retention: chapters use id=scene and clip_id; lessons use id, ordered chapters, and chapter_sha256 map; all have path, sha256, frames, duration, and certificate_sha256. Profile contains width, height, fps, language and audience. No key/profile join mismatch found.

Current manifest contains 17 lessons and 38 unique scene IDs, all accepted by the script ID expression. Record uniqueness, manifest ordering, clip ownership and lesson-to-chapter digest joins are checked. Provenance covers nested source bindings, publication certificates, chapter clocks, manifest and completion gates; rehashing before and after copies checks current originals. Completed and verifier timestamps are ordered. Retention loops produce exactly 55 movies, 93 sidecars and 114 images from 38 chapter records.

The longest current staged chapter image path is 165 UTF-16 units, safely below 256; retained movie and sidecar paths are shorter. Gallery follows catalog course, topic, clip and chapter hierarchy and links each chapter coverage/opening/closing image. Adjacent raster equality and all-frame ordered lesson equality remain independently gated by verifier flags, rather than inferred from the gallery.

Acceptance is pending actual final 55-media verification and current source provenance. This review does not accept any old publication.

Final gallery audit follow-up: current cumulative SHA-based chapter visual report is now linked directly. Opening prose uses readable17/38 counts. This changes only the retained gallery input, not production owners or active render sources.

## Final Source Re-Review While Publication Continues

The checkedCopy correction now covers every retained movie, render certificate, chapter clock and evidence image; it hashes source before and after copying and the destination. The93-sidecar gap is resolved. Exact55 record uniqueness, manifest chapter ownership/order/digest joins, complete source/chapter gates, independent joins/order/provenance flags, and before/after current proof-file hashes remain enforced. Current path inventory remains below256 units. No retainer execution or publication acceptance was performed here.

Concrete gallery correction needed: the Visual audit link targets historical inspection-visual-audit.md. The current exact-SHA chapter decisions and known repaired-but-not-yet-refreshed models footer hold are in inspection-current-chapter-visual-audit.md. The final gallery should link that current report (or an explicit redirect) so inspection evidence is not confused with historical whole-envelope previews. Root notified. Introductory generated text also still concatenates All17 and and38; restore normal spaces.

The final gallery otherwise supplies one course/topic/clip/chapter hierarchy,17 playable full clip embeds,38 individual chapter links and114 coverage/opening/closing image links. Playback and final before/after retained identity audit remain pending actual complete verification, including legitimate refresh of the models source.

Resolution: root corrected the final gallery Visual audit target to inspection-current-chapter-visual-audit.md and restored normal All 17 / and 38 spacing. Readback confirms the current cumulative audit is linked. No unresolved retainer finding remains from this review; actual execution still waits for complete current verification.

Final evidence review resolved: gallery now links the exact-SHA final-epoch visual ledger directly. Supplemental PNGs are constrained by final closure membership, owning clip/chapter, bound hashes, real generated/iv paths, exact nine stage names and six BIM sources; every copy checks source before/after and destination bytes. Independent evidence producer validates native tiles/stages and sheet counts before the postcomplete proof. No retention executed at this checkpoint.
