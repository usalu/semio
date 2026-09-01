# SourceIndex Pair — Terminal and Root Review78

## Outcome

The single authorized pair exceeded its fixed120000ms cooperative budget. The controller correctly returned **Nx exit1** even though its child completed both metadata snapshots during the15000ms terminal grace. This is a **late-complete, changed-membership pair**, not a passing performance gate, stable census, semantic-coverage proof, or completed goal.

Retained run: [run-qeYC5r](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-before-after-65/🧫️run-qeYC5r).
The [manifest](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-before-after-65/🧫️run-qeYC5r/manifest.json) is72489bytes, SHA25692f279bf22eec5084f7f3714ffa143f1ec16312656734e69b3ab4de5da4afc39.
The [receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-before-after-65/🧫️run-qeYC5r/receipt.json) is57356bytes, SHA25658018dcaee57ec4c5100c17090dd1290e4aa22652e594112111b666bcbc679de.

## Exact Terminal Boundary

- Six critical inputs captured at2026-08-28T04:27:36.931Z: controller f99a4322, rootS b505485c, N01fd62f3, D5ef65775, taxonomy6d06daee, descriptor db1c30ab.
- Controller created cancellation file at04:29:36.945Z; original fixed budget unchanged.
- Child44030 exited0; terminal observed04:29:44.319Z, within grace. No signal was used.
- Parent/controller returned1 at its existing line304 because terminal was cooperative-cancelled. The exact error was: `source-index capture did not complete terminal=cooperative-cancelled exit=0 cancelled=true`.
- Root independently observed PIDs44020/44029/44030 absent. It explicitly released the one-attempt source hold to taxonomy. No retry or further live SourceIndex invocation followed.
- Child stdout is exactly354bytes with one complete two-call marker; stderr is0bytes. Both are retained, as are every artifact and the cancellation file.

Later taxonomy N00a0c985 is a separately announced/released callback change after this terminal boundary; it does not rewrite the historical N01fd capture.

## Retained Data And Exact Drift

| Metadata | Before | After |
| --- | ---: | ---: |
| Admission observations | 74845 | 74849 |
| Selected source paths | 62702 | 62706 |
| Role/path/hash source records | 62704 | 62708 |
| Mutation roots | 198 | 198 |

The exact [drift JSONL](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-before-after-65/🧫️run-qeYC5r/before-after-drift.jsonl) contains12rows: four additions in each of admission observations, source paths, and source records. There are no removed or changed rows in the independently compared arrays. All four admitted paths are under the runtime ticket INTERACTIVE-JOB-RUNTIME-REFACTOR:

- 📓️coordinator-cut1-r4-review-2026-08-28.md
- 🧪️coordinator-cut1-r4-review-2026-08-28.json
- 🧪️resident-primary/🔣️.json
- 🧪️resident-primary/🧬️schema/🔣️.json

Their recorded origin is nonignored-untracked. These are observed additions between snapshots, not attribution of a write or a request to remove, restore, ignore, or exempt them. Root did not read their contents during this review. The corrected controller emitted no snapshot files between its two calls; independently admitted ambient files remain legitimate evidence.

## Independent Checks Actually Performed

Root read the complete pair-controller relevant capture/subject/serialization/terminal regions, full drift, complete projected manifest except the unabridged198-root display, complete review77/controller, and the retained child marker. The independent [artifact review77](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️source-index-pair-artifact-review-77.md) parsed every row of all sevenJSONLs, rejected duplicate keys before map comparison, compared full row arrays, and recomputed observation/path/record digests and artifact byte/hash/row receipts. Its new retained-artifact command exited0; this does not change the original pair's exit1.

Root separately reread all six large observation/path/record arrays, the manifest, receipt and terminal metadata. The complete new [root proof](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-pair-root-review-78/🔣️.json) records alltrue checks for:

- exact six critical captured leaf identities/hashes/ancestor identities, rather than trusting the stored stableInputs flag;
- unique observation/path/role-plus-path key spaces;
- manifest count joins and observation-kind/origin/boundary/role totals;
- all selected paths joining admitted observations and exactly one source-role row;
- exactly two schema-role rows, their source-role duplicate hashes, and both schema path/hash/byte joins to the corresponding critical capture;
- manifest bytes/hash joining the child's retained marker;
- both sourceTreeDigest values recomputed from retained roots/records/schema hashes and the recorded membershipDigest;
- unchanged hashes and leaf identities across the root reread.

The first inline diagnostic command was rejected by Bun before evaluation because shell quoting preserved newline escapes. It made no changes and ran no test; the corrected literal-quoted read-only command exited0. The proof retains its actual successful command source and input/output tuples.

## Important Limits

The original admission diagnostics array was omitted by the pair snapshot format. N's membershipDigest covers that original admission object; it cannot be independently reconstructed here without inventing omitted diagnostics. Root did **not** recompute that digest. SourceTreeDigest recomputation treats the recorded membershipDigest as an input, not independently established filesystem truth.

Raw source bytes and raw schema bytes were not persisted by the pair. The schema-role joins authenticate recorded hashes/byte counts, not raw schema-content retention. No Rust/TypeScript binding check, descriptor coverage, full mutation inventory, native suite, generated publication, or semantic completeness is established. The198roots include independently admitted ticket fixture/proposal paths; no production-only interpretation is silently applied.

Local filesystem birth times place the first snapshot artifact at04:29:32.246Z, before cancellation. Since the unchanged subject invokes both APIs before any snapshot write, this is supporting evidence that both API returns preceded that first write; it is not direct per-phase timing. The after-snapshot's first artifact was created at04:29:39.960Z and final manifest at04:29:43.302Z. A ticket-only bounded postprocessing investigation is now preparing to distinguish repeated kind classification from serialization and other costs. No cache, predicate, deadline, collector or production source was changed by that investigation yet.

All old timeout evidence and this late-complete result remain preserved. The exhaustive goal stays active and incomplete.

