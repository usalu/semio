# Norm Document Contract Audit

The current En1990 native diff explicitly removed whole-document artifact replacement, but JSON/GraphQL/Protobuf still declare it; TypeScript agrees with Rust. Din18599 JSON and other projections still declare an artifact replacement absent from native Rust. The field-parity report therefore lists three representation mismatches across these owners.

En1990's native qK diff is an optional composed child identity. Its JSON projection still references En1990QkList and an inline list of entries. This illustrates why matching top-level field names alone does not establish type or value-contract parity. No Norm source change was made in this audit; each facet must be checked against its committed native inputs and canonical child type before correction.

Applicable Norm AGENTS.md contains domain/research pointers only. This task changes representation ownership, not engineering calculations or standard values.
