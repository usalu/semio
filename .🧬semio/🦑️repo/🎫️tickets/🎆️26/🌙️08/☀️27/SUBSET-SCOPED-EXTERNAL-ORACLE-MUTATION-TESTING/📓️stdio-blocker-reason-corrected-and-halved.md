# 🧱️ The stdio blocker: my stated reason was wrong, and the blocker is now one homogeneous class

## The claim I made last turn, and why it was wrong

I recorded that the leaf migration was blocked because converting the aggregates' inline struct
variants to newtype variants over leaf types **changes the serde wire format**, and therefore moves
every fixture, diff codec and text codec with it. That was asserted, never measured.

Measured (`🗑️temp/wire`, since removed — a standalone serde crate, both tagging styles):

```
internal struct : {"mutation":"setVersion","major":1,"minor":4}
internal newtype: {"mutation":"setVersion","major":1,"minor":4}          IDENTICAL: true
adjacent struct : {"mutation":"setCell","payload":{"row":2,"value":"x"}}
adjacent newtype: {"mutation":"setCell","payload":{"row":2,"value":"x"}} IDENTICAL: true
```

**The wire format is byte-identical in both tagging styles.** A newtype variant over a struct
serialises exactly as the struct variant did, internally *and* adjacently tagged. No fixture, diff
codec or text codec moves. My stated blocker did not exist.

## The real blocker, stated so it can be checked

`✳️drawing` is the already-migrated reference. Reading it shows what a migrated aggregate actually
costs, and it is not the wire format:

* Its vocabulary was **redesigned**, not transformed — `NoMutation` and `SetSnapshot` are gone
  entirely ("replacing the old hand-rolled setter/whole-document-replace/bare-no-op-variant shape"),
  under an SMO-approved verb ruling recorded in its own module docs.
* Every leaf carries a `🔣️.json` encoding **semantic judgments** — `invertibility`,
  `diffParticipation`, `outcomeClasses`, `composition`, an approved `verb`, an `entity`, and a
  past-tense `record` name.
* Every leaf's `diff`/`inverse`/`label`/`target` moves out of the aggregate's match arms.

Counted across the 60 aggregates that still fail: **0–1 leaves against 5–15 variants each, ≈550
leaves to author.** Each needs a verb from the approved table and four classifications that cannot
be derived from the code. That is design work per artifact, and it belongs to the ticket commit
`d394744295` opened — not to this one. Guessing the classifications would produce compiling-but-wrong
metadata, the same failure mode I refused for the docx rename.

## What was fixed instead — 124 → 60, and the 60 are now one class

| | before | after |
|---|---|---|
| stdio errors | 124 (mixed) | **60**, all `E0046` |
| distinct causes | 5 | **1** |

1. **docx rename completed.** 17 files in the artifact tree plus the plugin root `🦀️component.rs`
   (`subsets::any` → `subsets::base`, three namespaces). `✳️any` is an empty husk and every symbol
   resolves uniquely in `✳️base`. This removed docx's own ~37 errors and, as predicted, *raised* the
   visible count to 75 by unmasking 60 previously-hidden `E0046`.
2. **175 leaf descriptors were malformed.** Every one held the **payload type name** in
   `aggregateVariant` (`SetDeclarationPayload`, `ChangeHeaderMutation`, `GltfBindDefaultScenePayload`)
   where `#[derive(dsl::Mutations)]` requires the **enum variant name** (`SetDeclaration`,
   `ChangeHeader`, `BindDefaultScene`). Mapped by the derive's own rule — `kebab(variant) ==
   semanticKind` — which resolved 382/382 leaves with 0 unmapped. Cleared 8 aggregates' `E0080`.
3. **A const-eval waste, fixed.** `mutation_leaf_descriptor_owner` re-tested the 15-byte
   `/🧬️mutations/` marker at every byte of every path after already finding it; `marker` is
   monotone, so the test is now short-circuited.
4. **`long_running_const_eval`**, newly *exposed* (not caused): the const previously **panicked**
   before exhausting its step budget, so fixing the panics let it run to completion. `🧊️gltf` alone
   validates 120 leaves byte-by-byte inside one const. Allowed at the stdio crate root with the
   reasoning written next to it — the evaluation is a bounded walk over a fixed roster, and the
   compiler's own help names allowing it as the remedy.

## The open question I had left for the dev is answered, and my earlier fix was the wrong one

I had flagged "does the relaxed kebab rule stand, or do the six single-word kinds get renamed?" It
is not a preference — the **kernel decides it**:

```rust
// 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:396
const fn mutation_leaf_descriptor_kebab(value: &str) -> bool {
    ...
    hyphen          // ← the return value: a hyphen is REQUIRED
}
```

So `rotate`, `scale`, `group`, `ungroup`, `flatten`, `unflatten` were invalid by contract, and my
earlier relaxation of the *derive's* copy of that check only made the two halves disagree. Both
corrected:

* the derive's `mutation_leaf_kebab` **requires a hyphen again**, matching the kernel;
* the six kinds are renamed to `rotate-node`, `scale-node`, `group-nodes`, `ungroup-node`,
  `flatten-node`, `unflatten-node`, with variants `RotateNode`/`ScaleNode`/`GroupNodes`/…, leaf dirs,
  descriptors, glue modules, text/binary codecs, catalog vectors and fixtures all moved with them.

The six **verbs** stay `rotate`/`scale`/`group`/… — all six are in `APPROVED_VERBS`, and the verb is
a separate field from the kind.

## Two mistakes I made doing it, both caught and repaired

* A blanket `"<kind>"` → `"<new-kind>"` replacement over the drawing tree also rewrote **data**:
  `transform`'s `"scale"` key became `"scale-node"` in 88 places, `DrawNode`'s internally-tagged
  `"group"` likewise, and it corrupted `verb:` and the text-codec opcodes. Reverted in every data
  position and re-applied only at kind positions (58 JSON + 9 Rust files).
* The path rewrite `"/🔄rotate"` → `"/🔄rotate-node"` over `📦️glue.rs` also matched
  `🔄rotate-object` in the `✳️object` subset, producing `🔄rotate-node-object`. Found by resolving
  **every** `#[path]` target in glue against the filesystem (2 broken, both reverted) — the check
  worth keeping for any future path rewrite.

## State

* `cargo build -p semio-s-plugin-stdio` → **60 `E0046`**, nothing else.
* Verification harness **119/119**; `oracleEvidenceCoverage` 574/658; 862 fixtures; 14 mutations
  still named as un-oracled. Coverage is unchanged by all of the above — the rename moved the six
  drawing kinds' identity, and the catalog's `mutationDirectoryName`/`sourceMutationDirectoryName`
  vectors were moved with them (that mismatch was caught by `registry/no-malformed-contribution`,
  which failed 118/119 until the catalog was updated).
* `mathematical` 9 + `sequence` 4 remain blocked: both crates depend on stdio, and their TypeScript
  packages are WASM facades (`export {}`) built from the same Rust — there is no TS bypass.
* `jpg::remove-huffman-table` remains blocked on a JPEG marker writer.
