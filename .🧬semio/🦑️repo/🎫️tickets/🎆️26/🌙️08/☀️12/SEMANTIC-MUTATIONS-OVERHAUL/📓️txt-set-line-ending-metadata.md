# TXT Set Line Ending Metadata

## Canonical Cutover

The direct set-line-ending leaf now owns only `🦀️.rs` and `🔣️.json`. The former `🦀️component.rs` and `🔣️component.json` names are absent. The text and binary facet filenames remain unchanged.

`SetLineEndingMutation` derives `dsl::MutationLeaf` with `#[mutation_leaf(contract = ::protocol)]`. The existing serde fields, semantic kind, required language surfaces, text opcode, binary tag, codec facets, and mutation behavior are unchanged.

The leaf test `canonical_leaf_metadata_matches_descriptor_and_provenance` independently deserializes its canonical descriptor through `serde_json`, compares it with `MutationLeaf::DESCRIPTOR`, and checks all relative provenance paths plus a nonzero workspace token.

## Neutral Preflight

The schema-first expected descriptor/provenance fixture is under `🧪️txt-set-line-ending-metadata`. The Nx-wrapped Bun preflight first failed with all 10 intended cutover assertions against the old names and missing derive/test evidence. Retained result: `🧪️txt-set-line-ending-metadata/🧪️precutover-structural-red.log`.

After the direct source/descriptor cutover, the same preflight passed all 10 assertions. Retained result: `🧪️txt-set-line-ending-metadata/🧪️canonical-ast-green.log`.

## Integration Handoff

Root owns the shared mount. Its required canonical path is:

```text
🧬️mutations/✏️set-line-ending/🦀️.rs
```

The exact Rust test filter is `canonical_leaf_metadata_matches_descriptor_and_provenance`. No Cargo or registered STDIO integration test was run in this packet; runtime verification remains root-owned and pending the shared mount.
