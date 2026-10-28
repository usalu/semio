# Alignment Schema And Fixture Independent Checkpoint

Read-only review of new definition/types/schema/cases/fixture. Endpoint is under implementation; no production semantics approval or new native execution claim. No confirmed P1/P2 production defect at this checkpoint.

Definition matches pinned released URI/hash and exact H/V/C declaration/length roster. Request explicitly selects layout/kind/applicability, report separately marks geometry/schema/view not-evaluated. Schema closed objects and exact canonicalu64 ID boundary checked directly for max/max+1 and several near-limit prefixes; sampled values agreed with expected bounds. This is a small regex sanity probe, not exhaustive schema proof.

Fixture correctly uses catalog allAttributes for simple entities and declaration attributes for every complex partial. It keeps actual ordered RelatedObjects array referenced by relation. Complex conversion exposes inherited-owner positions rather than only renaming a simple entity. Competing-list and duplicate-attribute rows create distinct ambiguity routes. Wrong-design/wrong-target rows rename declarations without remapping args; they test early domain refusal only and should not be cited as correct attribute-position fixtures for the replacement declaration.

## Neutral fixture contract gap

Cases schema permits lengths:[] combined with variants requiring a terminal parameter/member (signedzero, typed measure, integer/tiny/negative/string, wrong-member, dangling-design, etc). Fixture then treats nesting as last parameter or dereferences undefined terminal, causing construction errors rather than a concept report. Existing authored rows avoid those combinations. Constrain variant prerequisites in schema or have fixture explicitly reject unsupported combinations; do not let schema-valid future rows fail accidentally. Wrong-design always chooses vertical, so constrain its base kind to horizontal/cant or derive a genuinely different declaration for vertical rows. Similarly wrong-target chooses vertical and is ineffective for base vertical. These are fixture coverage/contract observations, not endpoint runtime bugs.

Retain tests for report schema after actual endpoint, source unchanged, exact target-order statuses and current original cancellation. Existing neutral Cases schema does not encode unique target IDs (Request duplicate targets should be independently refused by endpoint). Mixed/unknown complex roles must remain incomplete even if one constituent matches requested kind. Full semantic review follows stable endpoint/shared helpers.
