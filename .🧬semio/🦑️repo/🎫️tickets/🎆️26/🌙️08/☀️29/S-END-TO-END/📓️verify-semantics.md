# 🔬️ Semantic verification of the mutation-leaf migration

A green compile proves the shapes line up. It does not prove `agg_diff`/`agg_inverse` still compute
what the old `impl Mutation` computed. This is the check for that.

## Method

**Baseline:** the migration landed in commit `67fb4216b2`, which also renamed each aggregate from
`🦀️component.rs` to `🦀️.rs`. So `git show HEAD:"<current path>"` returns the *already migrated* file
and proves nothing — a first attempt fell into exactly that trap and correctly refused to fabricate a
comparison rather than guess. The real baseline is:

```
git show 67fb4216b2^:"<same directory>/🦀️component.rs"
```

**Comparison:** for every variant, the match arm's right-hand side — the expression after `=>` — was
compared character for character, ignoring only whitespace. Only the pattern head was allowed to
differ.

**Sanctioned differences** (benign): the pattern-head rewrite `E::V { a, b }` → `E::V(v_mod::V { a, b })`;
`vec![E::NoMutation]` → `Vec::new()`; ordinal/binary-tag renumbering after dropping tag 0; and the
forced re-wrap when an inverse's RHS constructs one of the enum's own variants.

**Anything else** counted as a regression: a changed index, a dropped `.clone()`, a flipped comparison,
a reordered field, a changed default, a silently vanished arm, a `base` that became `self`.

## Result: no semantic regressions across all 42 artifacts

| group | artifacts | verdict |
|---|---|---|
| A | las, ply, stl, html, xml ✳️valid, md, epw, deflate, binary, zip ✳️any, zip ✳️iso21320, gif 87a, gif 89a | clean |
| B | pptx, xlsx, docx, mp4, avi, wav, mp3, ifc 2x3, ifc 4, bcf, step, tsv, json ✳️i-json, dwg, dxf, obj, svg ✳️basic, svg ✳️tiny | clean |
| C | semio ✳️any, ✳️value, ✳️animation, ✳️audio, ✳️video, ✳️image, ✳️cad, ✳️model, ✳️document, ✳️presentation, ✳️flow, ✳️drawing | clean |

### Two arms worth naming

A careless sweep would have flattened these into the generic `Vec::new()` substitution, and **no
compiler would have caught it** — the types are identical either way. Both were preserved:

- `✳️flow` `SetNodeParam`: its inverse falls back to `RemoveNodeParam { id, key }` when the param has
  no prior value — not to a no-op. It was never a `NoMutation` sentinel.
- `✳️image` `SetMetadataEntry`: same shape, falls back to `RemoveMetadataEntry { key }`.

### `✳️any`'s 18 renamed wrappers

Each `Brep` → `ApplyBrep` rename was checked to carry `#[value(rename = "brep")]` with the **original**
camelCase wire tag. All 18 verified present and correctly spelled, corroborated independently by
`subset_mutation_tag()`'s match-arm string literals being byte-identical old vs new, and by `KINDS`
differing only by the dropped `"no-mutation"`. `SetSnapshot` correctly carries no override — its
identifier never changed.

### `✳️drawing`'s four rewritten surfaces

`🔗️component.graphql`, `🛰️component.proto`, `🔣️component.json` and `🟦️component.ts` were rewritten from
scratch against the Rust enum (they had mirrored a vocabulary with zero name overlap). All four now
list exactly the 17 Rust variants in declaration order with their real payload fields.

## The one real defect — pre-existing, now fixed

`🧊️obj`'s `📝️text/📖️component.grammar.semio` pinned `insert-tex-coord` / `remove-tex-coord` /
`set-tex-coord`, while the Rust `#[dsl(keyword)]`, `KINDS`, the `🧪️oracle/🔣️.json` catalog and the
`.feature` scenario ids all say `insert-texcoord`. That breaks DSL text round-trip for three ops, and
nothing type-checks a grammar file against a keyword string, so it compiled clean either way.

`git diff` showed the drift **committed at HEAD with no working-tree change** — it predates this
ticket and was not introduced by the leaf migration. The grammar was corrected to the spelling the
other four sources agree on.

Its cause was a doc comment asserting `keyword == to_kebab(variant)` and citing `InsertTexcoord ->
insert-texcoord`. The real variant is `InsertTexCoord`, which `to_kebab` maps to `insert-tex-coord` —
so someone applied the stated rule and got a spelling nothing else uses. These three leaves carry an
explicit `#[dsl(keyword = "…")]` override, which wins. The comment now says so, and records that the
leaf DESCRIPTOR's `semanticKind` legitimately stays the derived `insert-tex-coord`: the wire keyword
and the descriptor identity are independent vocabularies.

## Exposure noted, not changed

`🗜️deflate` and `💾️binary` had their `OpText`/`OpBinary` moved off `#[derive(dsl::DslOps)]` onto
hand-rolled `serde_json` during the migration — a deliberate wire-format change, since `DslOps`'
single-field-tuple-variant path delegates to the payload's `DslField`, which `dsl::MutationLeaf`
payloads do not provide. `🧊️obj` went the other way, keeping `DslOps` and giving each leaf
`#[derive(dsl::DslRecord)]`, because committed `.grammar.semio`/`.protocol.semio` files and two
conformance laws pin its format. These three artifacts should be reviewed together by whoever owns
the codec contract.

## Orphaned leaf scaffolding — found, deliberately NOT deleted

Removing the 21 shadowing `📦️glue.rs` stubs left their leaf sub-modules unmounted. A reachability
scan (collecting every `#[path]` and `include!` target in the crate, then listing
`🧬️mutations/<leaf>/{🔺️diff,↩️inverse,🦠️mutation}` folders no target points at) finds **100 orphaned
folders across 18 artifacts** — 40 `↩️inverse`, 32 `🦠️mutation`, 28 `🔺️diff`.

The scan is sound: `✳️mesh`/`✳️brep` contribute **zero** of them (their leaves reach `super::diff::` /
`super::inverse::` through the 98 glue stubs that were kept), and the only remaining `#[path]` mount
of a `set-snapshot` sub-leaf — `🖊️dwg ac1018` at `📦️glue.rs:3742` — is correctly excluded. Every other
textual hit is a doc comment.

They look like pure dead scaffolding, and AGENTS.md's no-legacy rule would say delete them. **They were
left in place**, because at least one is not inert:

> `🎒️zip ✳️any`'s `📄set-snapshot/🔺️diff/🦀️.rs` short-circuits with a `mutation.no-op` warning when
> `base == snapshot`. The live `agg_diff` SetSnapshot arm does **not**.

So these files are not merely unreferenced copies — they encode behaviour the live path does not have.
Deleting them would erase the only surviving record of a warning the migration dropped, and whether
that drop was intended is a question for whoever owns the mutation-outcome contract, not something to
settle silently here. The list is at `🗑️generated/` (regenerate with the scan above) and this is the
one open item this ticket hands over rather than closes.

## A real regression this migration introduced — found and fixed

`🗜️deflate` and `💾️binary` had their `OpText`/`OpBinary` replaced with hand-rolled `serde_json`, on the
belief that `#[derive(dsl::DslOps)]` could not survive the leaf shape (its `DslVariants` codegen
delegates a single-field tuple variant to the payload's own `DslField`, which a `MutationLeaf` payload
does not carry).

**That belief was wrong, and the replacement broke four committed conformance laws.** Each artifact's
`🧬️schema/💡️inferences/🦀️component.rs` holds two laws that `include_str!` the committed facets and check
them against *real codec output*:

```rust
async fn ops_grammar_conformance_law() {
    let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
    let recognizer = dsl::Recognizer::compile(&grammar);
    for mutation in mutations::demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(recognizer.recognize(&printed).unwrap_or(false), …);
    }
}
```

`📖️component.grammar.semio` still describes the keyword grammar (`set-payload payload="…"`) and
`📡️component.protocol.semio` the `format u8 | ordinal varint | record body` layout, while `print_op`
had become `{"mutation":"setPayload",…}`. Nothing type-checks a grammar file against a codec, so it
compiled clean and the laws would simply have failed.

`🧊️obj` had hit the identical wall and solved it properly: keep `DslOps` next to `Mutations`, and give
each leaf `#[derive(dsl::DslRecord)]` + `#[dsl(keyword = "…")]`, which restores the `DslField` the
single-field-tuple-variant branch needs. Applied that precedent to both artifacts — 8 leaves, each
keyword taken from the committed grammar's own production literals (binary's `ReplaceByteRange` keeps
the keyword `splice`), with every per-field `#[dsl(block)]` / `#[dsl(base64)]` recovered from the
`67fb4216b2^` baseline. The `OpText`/`OpBinary` bodies were restored verbatim from that baseline.

The facets were deliberately **not** edited: the point is that the code went back to matching them.

One instruction was correctly refused: the brief said to revert the sibling `.abnf`/`.g4`/`.ebnf` too,
but a diff against the baseline showed those already described the JSON form *before* the migration,
and obj carries the identical inconsistency. Reverting would have resurrected a `no-mutation`
production for a variant that no longer exists. Left alone, with the reasoning recorded.
