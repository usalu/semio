# 🧭️ Current Admission-Only Roster 54 — Proposed Execution Boundary

## Status

Preparation only. No real-workspace roster has been executed by this packet. IO verification is still completing its pre-mount/after-mount sequence; its first package attempt failed on root S syntax before it could isolate the missing IO import. This queued roster work begins only after that handoff is coherent and the exact execution plan has been communicated to root.

## Actual authority inspected

The current N exports `inventoryTaxonomySources(options: TaxonomyInventoryOptions): TaxonomySourceInventory` at line 2855 in the inspected source. It prepares nofollow options, loads the actual taxonomy, and calls the existing admission collector. Its intrinsic result extends the closed admission fields with `repoRoot`, `taxonomyPath`, `taxonomyContentHash`, and `membershipDigest`. The collector computes the membership digest from its canonical admission representation.

The existing options expose `repoRoot`, optional `scope`, `ticketDir`, `cancelFile`, `progress`, and `taxonomyPath`. The existing cancellation check throws `TaxonomyCancellationError` when the supplied regular cancellation path exists. Progress carries operation, phase, current, total, and optional exact path. These are actual source capabilities, not a proposed parallel API.

The inspected N fingerprint recorded by the preceding IO attempt is `0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce`. Any execution will recapture current bytes; this plan does not pin or restore that source.

## Proposed new ticket-only owner and schema-first receipt

Use a new `📓️admission-current-roster-54/📜️script.ts` controller, with a ticket-local closed receipt schema authored before execution. Keep it separate from the newly authored verification controller and from all lost owners.

The receipt records a current-only observation pair: start/end times; actual invocation options; source fingerprints; every progress/cancellation event; each complete wrapper result or exact exception; independent canonical-schema validation; membership digests; stable/changed/rejected/cancelled/timed-out outcome; and any membership difference. It must never substitute a prior saved roster, source-census seed, baseline commit, or earlier fixture result for the current invocation.

Validate the admission portion of each actual inventory result against the existing canonical `$defs/sourceAdmission` using Ajv2020. The additional inventory metadata and receipt envelope get their own explicit schema; do not pass an extended object to the closed base schema or weaken that schema. Preserve each raw wrapper result as evidence.

## Actual invocation

Use the existing exported wrapper twice sequentially in one isolated Bun child. Supply only the actual workspace root, exact current taxonomy path, one controller-owned cancellation path, and a progress callback. Omit `scope`, `ticketDir`, `baselineCommit`, `workers`, and `excludedTreeDigests`.

This is an unscoped admission roster, not a content or mutation census. Do not invoke `inventoryTaxonomy`, inspect admitted leaf contents, hash admitted leaves, classify semantic mutations, enumerate a private filesystem roots list, or add another skip-list authority. Git tracked/untracked population and declared generator outputs remain the existing wrapper's responsibility. Do not explicitly admit this ticket or any ticket input. Report any naturally admitted artifact through the wrapper's actual provenance, without deleting it from the result.

No git write, source edit, cleanup, or access to a real excluded path is authorized. Lexical any-case excluded-segment checks precede controller artifact/source traversal; the wrapper retains its own reviewed exclusion/no-follow authority for candidate admission.

## Bounded child, progress, and cancellation

One child owns both observations and has a hard maximum lifetime of 60 seconds. At 55 seconds, the parent creates the previously absent controller-owned cancellation flag exclusively, letting the existing callback checkpoints cancel cooperatively. At 60 seconds, the parent kills a still-running child and records a timeout rather than completion. A cancellation exception and a forced termination are different outcomes.

Retain progress events with their exact phase/path/current/total and timestamps. Do not claim a synchronous Git command is interruptible by the cooperative flag; the hard child timeout bounds that remaining seam. Preserve all complete observations received before cancellation, but never label a one-observation or interrupted result as a stable pair.

Create evidence paths before observing membership and defer final receipt creation until both observations finish or the child terminates. This avoids introducing a newly named result artifact between the two observations. Do not assume ticket paths are ignored: document which artifacts pre-existed and preserve the actual returned membership.

## Source and membership coherence

Capture actual N, D, taxonomy, controller, and schemas with the reviewed full-ancestry/nofollow/descriptor checks. Preserve first hashes; check after module import, between observations, and at the final endpoint. Do not restore source or retry until a preferred result appears. Report whole-source drift separately from membership drift.

Retain both full admission observations and both actual membership digests. Compare exact raw physical path identities and complete observation tuples, including origins, all index stages, and generator identities. Raw NFC/NFD distinctions remain separate. Record added/removed/changed observations and diagnostic changes without interpreting them as mutation semantics.

A successful stable receipt requires two complete results, the actual schema validations, identical source endpoints, and identical membership observations/digests. A rejected admission remains a rejected roster even if the two observations agree. Stable observation does not prove the workspace was globally immutable between endpoints.

## Retention and handoff

Retain full JSON/progress/output plus a complete Markdown receipt inside the fresh run and an identical uniquely named Markdown sibling directly under the active ticket. This is loss mitigation, not a guarantee. Never recreate the missing earlier result directories or claim their logs survived.

Communicate this exact plan before executing. Production and launch changes remain out of scope. Root's separately executed physical-integration33 and fixture verification are supporting context only; they are not credited as this real-workspace roster execution.
