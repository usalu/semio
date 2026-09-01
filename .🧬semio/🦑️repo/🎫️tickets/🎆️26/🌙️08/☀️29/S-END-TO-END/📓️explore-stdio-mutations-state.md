# 🗄️ State of the SEMANTIC-MUTATIONS-OVERHAUL fallout in `s` plugins

Collected by a read-only explorer (Haiku), 2026-08-29.

## Provenance

- HEAD is `bb06c41f73` (`git log --date=iso` → **2026-08-28 11:09:46 +0200**). Commit *messages*
  carry a frozen fake date template — only `%ad` is trustworthy.
- `✏️s/🔌️plugins/🗄️stdio` carries ~3093 dirty/staged files: the overhaul is **uncommitted and
  in-flight**, owned by the ticket `🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL` (54+ subtasks).

## Confirmed breakage

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:252`

```rust
return Vec::new() => {   // not a valid match pattern — file does not parse
```

Because this file does not parse, the whole `semio-s-plugin-stdio` crate fails, and every crate that
links it (notably the demonstrator and `space`/`s` itself) fails transitively.

## The trait the migration must satisfy

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:105-149`

```rust
pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: MutationDiff<P>;
    const DESCRIPTORS: &'static [MutationLeafDescriptor];
    fn descriptor(&self) -> &'static MutationLeafDescriptor;
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Vec<Self>;
    // …remaining methods have defaults
}
```

### Reference implementation to copy from

`🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs`

```rust
const CAUSAL_ADD_DESCRIPTOR: crate::mutation::MutationLeafDescriptor = crate::mutation::MutationLeafDescriptor {
    schema_version: 1,
    owner: "…/🧬️mutations/➕️causal-add",
    semantic_kind: "causal-add",
    display_name: "Causal Add",
    emoji: "➕️",
    aggregate_variant: "CausalAddOp",
    payload_schema: "🛂️schema.json",
    text_opcode: None,
    binary_tag: None,
    invertibility: crate::mutation::MutationInvertibility::ExplicitMutation,
    diff_participation: crate::mutation::MutationDiffParticipation::ApplyOnly,
    outcome_classes: &[crate::mutation::MutationOutcomeClass::Applied],
    composition: crate::mutation::MutationComposition::Atomic,
    required_language_surfaces: &[crate::mutation::MutationLanguageSurface::Rust],
};

impl crate::mutation::Mutation<i64> for CausalAddOp {
    type Diff = CausalAddDiff;
    const DESCRIPTORS: &'static [crate::mutation::MutationLeafDescriptor] = &[CAUSAL_ADD_DESCRIPTOR];
    fn descriptor(&self) -> &'static crate::mutation::MutationLeafDescriptor { &CAUSAL_ADD_DESCRIPTOR }
    fn diff(&self, _base: &i64) -> crate::mutation::MutationOutcome<CausalAddDiff> { … }
    fn inverse(&self, _base: &i64) -> Vec<Self> { … }
}
```

## Ownership note

This area belongs to a peer session's ticket. Per the standing instruction we work **alongside** it:
fix only what blocks `s` from building, keep edits confined to the failing sites, and do not
restructure the overhaul.
