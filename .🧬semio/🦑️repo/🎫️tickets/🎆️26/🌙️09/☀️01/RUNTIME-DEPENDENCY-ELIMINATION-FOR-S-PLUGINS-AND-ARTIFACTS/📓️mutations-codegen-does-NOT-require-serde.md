# ❗️ Correction: `dsl::Mutations` / `ArtifactSchema` do NOT require serde

## The claim
A conversion agent concluded, and acted on, this rule:
> "any type reachable from a `#[derive(dsl::Mutations)]` enum's leaves or an `ArtifactSchema`
> Snapshot's field tree needs *unconditional* production `Serialize`/`Deserialize` alongside the new
> value derives (needed by the mutation dispatch codegen and the io serializers) — not
> cfg_attr(test)-gated or removed."

It then RE-ADDED unconditional production serde across four plugins (🕸️dag, 🖨️raster, 🪵️sourcing,
📋️forms) on that basis.

## Measured: the codegen half is FALSE
    🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs            → 0 occurrences of serde/Serialize/Deserialize
    🧬️schema/✨️derive/📦️packages/🦀️rust/🦀️.rs         → 0 occurrences
Neither `#[derive(Mutations)]`, `MutationLeaf`, `DslArtifact`, `CompositeMutation`, nor
`ArtifactSchema` emits any serde bound or path. The mutation dispatch codegen does NOT require serde.

## The io-serializer half is real, but the conclusion is still wrong
The JSON/CSV/PNG/SVG/MD io serializers DO call `serde_json` today — which is exactly the trap this
ticket keeps documenting. Keeping serde on a type to satisfy a `serde_json` io bridge is what LEAVES
SERDE LINKED. The fix is to convert the io serializers to `pack::json`, not to pin serde onto every
mutation leaf and snapshot in the plugin.

## Net effect (production serde, stripper method)
    🕸️dag      148 → 93     🖨️raster  169 → 127
    🪵️sourcing 105 → 74     📋️forms    95 → 62
Real progress, NOT a regression to baseline — but a meaningful share of the ~356 remaining refs was
deliberately re-added on a premise that does not hold, and should be re-examined.

## Rule
Before pinning production serde onto a type "because a derive/codegen needs it", GREP THE MACRO:
    grep -cE 'serde|Serialize|Deserialize' <the derive crate's source>
Zero means the requirement is imagined. This is the mirror image of the dual-derive trap: there,
serde survives because nobody removed it; here, serde is actively re-added to satisfy a requirement
that does not exist.
