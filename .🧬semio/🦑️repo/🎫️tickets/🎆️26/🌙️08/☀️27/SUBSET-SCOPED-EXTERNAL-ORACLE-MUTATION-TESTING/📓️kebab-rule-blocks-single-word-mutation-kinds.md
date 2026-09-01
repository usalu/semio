# 🐛️ `mutation_leaf_kebab` contradicts its own error message, and blocks single-word kinds

Found while tracing the second error family behind `semio-s-plugin-stdio`'s 122 build failures.

## The defect

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:463`

```rust
fn mutation_leaf_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes[0].is_ascii_lowercase()
        && bytes.contains(&b'-')                    // ← demands a hyphen
        && bytes.split(|b| *b == b'-').all(...)
}
```

Its call site (`:439`) rejects with **"semanticKind must be lowercase kebab-case"**.

`rotate` *is* lowercase kebab-case. So is `scale`, `ungroup`, `unflatten`. The `contains(&b'-')`
clause makes a **single-word kind unrepresentable**, which is a stricter rule than the message states
and than the name implies.

## What it costs

`✳️drawing/🧬️schema/🧬️mutations/🔄rotate/🦀️.rs:14`'s `Rotate` carries no `dsl::MutationLeaf` derive —
it cannot, under this rule — while the subset's mutation enum at `🦀️.rs:51` requires
`Rotate: MutationLeaf`. That is the `E0277` family, ~70 of the 122 errors blocking the crate.

The same subset holds several other hyphen-less leaves (`📏scale`, `💫ungroup`, `🎈unflatten`) beside
the hyphenated ones (`➕create-node`, `📐change-stroke-width`, …) that pass fine.

## FIXED — and my earlier reason for not fixing it was wrong

The clause was removed. Checked first, rather than deferred to:

`semantic_kind` is validated here, stored in the descriptor, and compared for **equality** against
`SEMANTICS.kind` (`:1772`). **Nothing downstream splits it on `-`.** The clause had no purpose, and its
own call site's error message already described the correct rule.

### The revert that preceded this was the mistake, not the relaxation

Earlier in this session the same relaxation was applied and then rolled back — the note recorded said
it "caused descriptors to be written that then had to be deleted". Writing descriptors is what the
derive DOES; that was the feature, mistaken for a side effect. The roll-back deferred to the existing
rule instead of asking whether the rule was right.

### What it actually unblocked

Removing the clause alone changed nothing — `Rotate` never carried the derive to begin with. The six
`✳️drawing` leaves were simply **unmigrated**: each had `🔣️payload.schema.json` but no `🔣️.json`
descriptor and no `dsl::MutationLeaf`. They are the ticket's own outstanding "migrate the nested leaves"
item, and every one has a single-word kind — `rotate`, `scale`, `group`, `ungroup`, `flatten`,
`unflatten` — so the rule was **necessary but not sufficient**: no descriptor for them could validate
while it stood.

Descriptors written and the derive added for all six (`📜️one-shot/write-drawing-leaf-descriptors.py`),
with `aggregateVariant` matched to the enum at `✳️drawing/🧬️schema/🧬️mutations/🦀️.rs:51-57`.

**`semio-s-plugin-stdio`: 122 → 50 errors.** The framework crate still builds and this ticket's gates are
unchanged — 119/119, 862 fixtures, coverage 644/658.

## What the remaining 50 are

37 `E0433` + 13 `E0432`, being 30 in docx and 14 in semio — all the incomplete `subsets::any` rename.

**A docx-only, artifact-scoped rewrite was tried and REVERTED.** Keying on the full
`crate::artifacts::docx::standards::v_ecma_376::subsets::any` prefix did correctly leave `🎒️zip`'s own
`subsets::any` alone, but it took the count 50 → **95**: the imports then resolved far enough to expose
**60 `E0046`** (missing trait items) and 9 `E0080` (const-eval), and the remaining `E0433`s turned out to
use a second namespace, `crate::editor::docx::...`, that the rewrite never covered.

That is implementing missing trait methods inside another ticket's migration, not finishing a rename.
Reverted in full — docx is clean against `HEAD` again — and the verified improvement kept.
