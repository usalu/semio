# 🧭️ Current Admission Pair After Gitlink Adoption

## Actual Terminal Outcome

One unchanged-budget real-workspace admission-only pair completed on 2026-08-28, from 00:22:22.543Z to 00:23:00.932Z. Both inventories returned `complete` and passed the actual schema. The child exited 0 without stderr, cancellation, signals, or timeout in 37,205.225 ms. The controller/Nx correctly exited 1 with `status: changed`: membership was not stable across the pair. No retry was launched.

The first observation ran 00:22:22.721Z–00:22:41.761Z and returned 73,152 records. The second ran 00:22:42.063Z–00:22:59.484Z and returned 73,156. All seven captured implementation/schema/controller input identities and hashes stayed equal at their recorded endpoints. Source stability is not workspace-membership stability.

| Property | First | Second |
| --- | ---: | ---: |
| Regular files | 72,455 | 72,459 |
| Symlinks | 48 | 48 |
| Directories | 50 | 50 |
| Absent tracked paths | 599 | 599 |
| Tracked origin | 70,879 | 70,879 |
| Nonignored untracked origin | 1,932 | 1,936 |
| Ignored generator origin | 341 | 341 |
| Explicit ticket origin | 0 | 0 |

Both observations retained 70,879 stage-zero index tuples, no nonzero-stage tuples, and 362 generator-output tuples. All 599 diagnostics are `tracked-path-absent`; no blocking admission diagnostic was returned. Absent index identities are observations, not inferred deletion intent.

## Exact Membership Difference

Four new nonignored-untracked regular-file records appeared in the second observation; no path was removed and no existing record changed:

- `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️fixture.json`
- `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️schema.json`
- `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️contract.json`
- `🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json`

This is a membership observation, not attribution of authorship or a request to pause, restore, or exclude those files. Their contents were not read by this admission run. Diagnostics were identical. Returned and independently ordered reference digests differed consistently:

- First: `dd55e057d958be1206d8311da4670d57279406d28fed3fa703b3f7e9c3f80cbc`
- Second: `6eb2a52804f9b7c9ca9f010445689e9af9c2da3fc4e2b916cf4ed9a07b09af08`

## Gitlink Observation

Both inventories returned the same single tagged repository boundary: raw path `♻️mit-bestand/recherche`, physical `directory` / `040000` / explicit directory, tracked origin, `repositoryBoundary: gitlink`, index mode `160000`, object `92036c7ca0149b43ddea28db8c8e516f983fe718`, stage 0. Neither inventory contained any descendant of that path. The earlier `nonregular-node` admission rejection is not reproduced at this source endpoint.

This verifies the returned admission record and absence of returned descendants, not an independent operating-system I/O trace. The separate synthetic no-descent and full-inventory-refusal tests are recorded in the source release. No full taxonomy classification, plan, move, apply, recovery, mutation-content census, or scaffolding ran here. Existing stale-plan apply/recovery concerns remain open. The complete mutation goal remains open.

## Source Endpoints And Retained Evidence

- N: `970b240e43810044e1d497c9319abe5561a8ae02c8db0fa2efac57fb2b4767cb`
- D: `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423`
- Taxonomy: `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce`
- Canonical admission schema: `1b88f7dfd1cd8f4809e690225af22251c798f7fac4526d993301eedca04afbc4`
- Receipt schema: `aee39eb5d69d7d41576bb53c1c1fb75401a09741082b00912471c1c8826c5b16`
- Receipt vectors: `3c39bad7f12e0daacf3c4fc36133c209c2642b879db50c8593e1f518f9f44278`
- Controller: `4032fe54943c5d295ee6d82c010d38a26050f0c07fd28dc551bae0b71925923f`

The unchanged controller was run through Bun/Nx with `current`, no scope or explicit ticket, 55-second cooperative / 60-second hard limits. The exact schema self-checks remain 15/15; all 26 receipt checks passed, while the independent comparison explicitly reports changed membership.

Complete evidence remains at `📓️admission-current-roster-54/🧫️run-k7yzCQ/🔣️receipt.json` and its raw stdout/stderr, input capture, and Markdown copy. JSON SHA256 recorded by the controller: `7da1899e902e2244d990fb831a15367daa7141c74ea91fc152d8bf5390548174`. The ticket-level complete copy is `📓️admission-current-roster-54-current-2026-08-28T00-22-22-543Z-🧫️run-k7yzCQ.md`. No evidence was removed or rewritten.
