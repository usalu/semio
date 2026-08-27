# Generator Input Transition Recovery

## Reproduced Defect

The empty-facet integration correctly made a moved authored Markdown leaf an input of the plugin-registry generator. Injecting a failure after its move exposed an existing recovery defect: `validateResumeTuples` tried to read every generator input at its original path even when the journal already proved that the file had been staged or installed elsewhere. Recovery therefore failed with `ENOENT` before rollback could restore the source.

A separate minimal fixture reproduced the same error after staging. It contains one moved TypeScript leaf, one rewritten TypeScript consumer, and an exact three-input generator contract. The first fixture drafts were rejected for unordered inputs and noncanonical preview keys; after correcting only those fixtures, the intended red test failed at the original moved source stat (zero passing, one failure, five assertions, 1.106 seconds).

## Implementation

After validating move, relocation, removal, symlink, backup, and reference-edit tuples, recovery resolves each input to its journal-proven physical location: original source, staging leaf, installed destination, retained old symlink target, relocated embedded subtree, or edit backup. It reads that physical preimage with no-follow ancestry checks and compares the complete record, using the original logical input identity, against the frozen generator input ledger. No input comparison is skipped.

Exact operation and edit lookups are indexed once per validation, avoiding an input-count-by-move-count scan for large generator ledgers. Unaffected inputs remain checked at their actual path. Edited live content is independently validated before its original input preimage is read from backup.

## Verification

The permanent language-neutral fixture declares source/destination paths, the consumer, five injected failure stages, and three concurrent-drift cases. The TypeScript compiler parser independently confirms the fixture's reference target. Real disposable Nx generator commands execute during retry and commit.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️generator-input-transitions.test.ts'
9 pass, 0 fail, 99 assertions, 32.06 seconds
```

The five stages are after staging, moves, edits, regeneration, and before verification. Each restores the original source/consumer bytes and modes, then retries to committed state. The negative cases change an unaffected input, an installed moved input, and an applied edited input. Recovery rejects each mismatch without overwriting the concurrent bytes; restoring the exact expected test preimage allows journal resume to commit.

All fixture repositories and retained failure evidence are inside this ticket. No production move was performed by this packet. The artifact-facet lane independently confirmed its previously failing rollback and exact-reference lifecycle after this change; its final aggregate result is recorded by that lane.

## Files

- Normalizer `🧹️normalization/🟦️.ts`: indexed input authority and phase-aware preimage reader.
- Permanent fixture `🧫️fixtures/🧪️generator-input-transitions/🔣️.json`.
- Ticket test `🧪️generator-input-transitions.test.ts`.

The complete frozen transaction aggregate and timing gate remain separately required; this packet is not a claim of all-repository completion.

## Generator Failure Before Output Creation

The JCO package pilot exposed a distinct rollback failure when a generator exits before creating its output directory. The executor marked regeneration started, then rollback force-removed the already absent output and attempted to fsync its also-absent parent. That raised `ENOENT` and prevented the remaining rollback.

The language-neutral fixture now declares an absent output-parent directory. Its deliberately failing real Nx generator reproduced the exact error: zero passes, one failure, six assertions, 14.09 seconds. The narrow correction skips output removal only when its immediate parent is absent; removal and parent fsync remain unchanged whenever the parent exists. The focused test passed one case with eleven assertions in 9.02 seconds, confirming original source/consumer bytes, absence of the canonical moved source, and continued absence of the output parent. The generator command's intentional nonzero exit is expected; successful transaction rollback is the assertion.

## Input Membership During Execution

Frozen input tuple checks alone do not detect a newly added input. A wildcard input outside the scoped source inventory was added during the staging callback; the preimplementation executor incorrectly committed. The intended regression failed with zero passing tests, one failure, six assertions, 7.75 seconds. An initial fixture-only assertion expected the wildcard's literal root itself, which `root/**` does not match; that assertion was corrected before this reproduction.

The shared input selector now accepts a logical filesystem view. The transaction overlays only journal-proven moves, relocations, removals, embedded roots, symlink preimages, and edit backups. Installed destinations and newly created ancestors are hidden only through exact plan/journal authority, and an unplanned child prevents hiding its ancestor. Own generated outputs remain outside that generator's inputs. Original logical source names are reinserted into directory membership even for untracked moved inputs.

Full tuple and rederived membership checks run before each generator, before commit, and during recovery. The new four-phase addition matrix and third-party glob parity are under validation; no green result is claimed for them until the commands finish.

The four-phase added-input matrix subsequently passed all four cases with 39 assertions in 37.00 seconds. The expanded full packet passed 15 tests, zero failures, 155 assertions, in 118.85 seconds. Its isolated schema intentionally removes only the unrelated production registry selector; the synthetic generator has its own exact/wildcard input authority. A first four-case attempt lacked that fixture isolation and failed because the real registry implementation was not materialized, before any transaction assertion.

The JCO pilot then exposed an additional overlay ordering error for moved inputs that also receive reference edits: resolving a canonical destination to its retained edit backup accidentally re-admitted that destination as a logical source input. A separate permanent prospective-destination/self-reference vector reproduced the exact extra member after edits (zero passes, one failure, five assertions, 4.20 seconds). Destination shadowing now precedes backup resolution for non-source identities; exact source identities still resolve to their proven preimage. The focused corrected result is recorded after the command finishes.

The moved-and-edited input regression passed one test with nine assertions in 13.37 seconds. It verifies rollback to the original self-reference and a successful retry with the canonical self-reference.

## Empty Directory Input Parity

The previous native selector built wildcard candidates from Git file names, while the logical selector traversed the filesystem. Consequently a matching empty directory could be absent from the frozen ledger but appear during unchanged recovery. The permanent empty-directory vector reproduced that omission (zero passes, one failure, five assertions, 2.41 seconds).

Native and logical wildcard selection now share directory traversal with no-follow ancestry and opaque-first admission. Directory entries supply non-directory candidate names; nonmatching leaves are not statted or opened. Only matched leaves are validated, so ignored matching inputs remain included and matched symlinks remain rejected. The focused empty-directory parity test passed one test with nine assertions in 11.93 seconds, including independent fast-glob agreement, rollback, retry, and preservation of the empty directory.

A focused rerun of the older CAD/Draw ignored-input test was blocked before its input assertions by the newly declared JCO adapter generator: its frozen external root-script edit activated an unmaterialized producer in that isolated fixture. This is a package activation/isolation integration issue, not a passing ignored-input regression, and it was sent to the package lane. The complete expanded input packet is still running separately.

The complete expanded input-transition packet then passed 17 tests, zero failures, and 173 assertions in 124.59 seconds. It includes all prior phase/recovery cases, the moved-and-edited prospective destination, the four added-member phases, the absent generator output parent, third-party glob parity, and empty-directory preservation. An uncached filtered transaction-v2 Nx rebuild also passed one case with six assertions in 0.921 seconds, confirming current sources compile through the permanent package task.

## Nonmatching Leaf Metadata Admission

The native wildcard walk correctly used directory-entry kinds, but its logical recovery view still statted every child to reconstruct kinds. An ignored text leaf that did not match the declared JSON wildcard therefore received an unnecessary metadata probe during rollback. A permanent language-neutral wildcard/nonmatching-leaf vector and an exact `lstatSync` guard reproduced that violation: zero passes, one failure, six assertions, 4.93 seconds overall.

The logical view now preserves native directory-entry kinds for unchanged children. It resolves metadata separately only for exact journal-proven source identities and synthesized entries, after destination/output shadowing. Created-ancestor hiding and full matched-input tuple validation are unchanged. The focused regression passed one test, zero failures, nine assertions, in 12.93 seconds overall, with independent fast-glob membership comparison, rollback, real Nx retry, and preserved sentinel bytes. A complete rerun after this addition remains required separately.

The complete 18-case packet subsequently passed with zero failures and 182 assertions in 435.21 seconds. This was the source module loaded before the new global reference-closure integration; it confirms the no-follow Dirent change and all previous generator-input/recovery cases, not the later incoming-reference changes or an aggregate timing gate. Every real disposable Nx invocation ran; the deliberately failing generator-before-output case passed its rollback assertions.
