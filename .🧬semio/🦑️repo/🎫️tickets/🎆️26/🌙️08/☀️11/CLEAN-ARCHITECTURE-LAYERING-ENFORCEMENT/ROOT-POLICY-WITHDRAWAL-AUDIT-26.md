# Root Policy Withdrawal Audit 26

Read-only source audit. No source edits, runtime gates, raw40 guard reads, Git mutations, or lifecycle claims. No execution or publication certification.

## Bounded Receipt

Complete decision JSON and complete live Root source were read under 64 MiB byte ceiling, 65536 work units and 60-second deadline. Observed bytes: 10175633; work: 6; elapsed seconds: 0.193. Decision SHA-256: `a47f58cdfbd850eec772cb058201cad920835f942888764f01741c41ead10d25`. Live Root SHA-256: `ab4b05880c47427cf961ca3cd1b3dd9abcfbf80affe6be82e9959359e4c8eb3a`; matches retained current: True. The producer decision records a separate 16 GiB ceiling and 706779745 charged bytes, which cannot certify this audit's bound.

| Body | UTF-8 bytes | SHA-256 | Matches declared hash |
| --- | ---: | --- | --- |
| frozen | 1888758 | `0cacc31e73644b9655f509a7f59490c5829bc584e17d9366a57c692045d884eb` | True |
| preparedBefore | 1888758 | `0cacc31e73644b9655f509a7f59490c5829bc584e17d9366a57c692045d884eb` | True |
| current | 1885445 | `ab4b05880c47427cf961ca3cd1b3dd9abcfbf80affe6be82e9959359e4c8eb3a` | True |
| proposed | 1891711 | `896519153169de362cd512bfa95e0da71c3691e25d17c36a2fa4da06dcc70981` | True |

## Findings

The peer withdrew stdio-specific field-sweep presence checking, its empty allowlist, per-subset key helper, and its S2 aggregation call. S2 remains exported and retains VCS machinery, facet drift, grammar honesty, and DiffAlgebra checks. This is narrowing of stdio coverage, not deletion of S2. DiffAlgebra text drops `between`; executable trait-presence checking remains. Source facts prove withdrawal but do not prove broader intent or behavioral law correctness.

The retained proposed full body reintroduces all withdrawn definitions and the S2 call. Current/proposed counts: field-sweep checker 0/2, allowlist 0/5, subset-key helper 0/2. Do not join this whole proposed body. Successor41 must begin with entire current source and transplant authorized candidate changes while preserving withdrawals. A removed naming rule need not be restored.

The candidate moves structural enforcement to MutationTaxonomyStructuralSourceView and caller-owned admission, while semantic checks receive declared mutation roots. This provides a domain-neutral structural receiver direction, not certification of behavioral field coverage or import/export/tree laws. The removed field_sweep checker tested syntactic naming/presence only. Any replacement must assert actual domain-neutral behavior at its owning receiver, retain all original runtime and architecture assertions/controls, and avoid copying stdio naming coupling or shortening the census.

Proposed executable R13/R14 checks remove current ephemeral lane/type blanket exceptions: constants disappear and whole-state/restore checks are unconditional. Its function docstring still claims presence/transient exemptions and contradicts the body; correct that documentation. Retain current shared restore matcher and rule reasons. Candidate policyRestoreInverseBreaches docstring narrows the description despite unchanged shared matcher in the candidate change list.

Producer reports 120 public declarations/signatures unchanged; this audit hashes complete source bodies but does not independently run a declaration parser. Positional IfStatement/ExportDeclaration overlaps must not authorize resurrection. Retain original receiver tests and controls and bind fresh successor41 receipts to selected full source before runtime/publication claims. Stage40 cannot certify another Root identity.
