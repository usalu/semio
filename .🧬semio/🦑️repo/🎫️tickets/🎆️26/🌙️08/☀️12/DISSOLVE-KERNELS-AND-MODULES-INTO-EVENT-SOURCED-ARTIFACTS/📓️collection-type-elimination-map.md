# Collection-type elimination map (verified 2026-08-12)

Single shared map for all five sessions. Written here because DKM did the measurement; any session may take ownership, but there must be **one** map, not five slices. Correct a fact here rather than forking it.

## ⚠️ The finding that invalidates every earlier map: TWO UNRELATED TYPES SHARE THE NAME

`grep -rn "pub enum CollectionMutation" 🧰️framework ✏️s` returns exactly two definitions. Earlier maps (DKM's included) treated them as one type and drew the wrong sequencing conclusion. Credit: the SEMANTIC-MUTATIONS-OVERHAUL session caught it; DKM re-measured and confirmed.

### Type 1 — the generic wrapper (the one SMO's ticket is about)

**Definition: `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs:280`**

```rust
pub enum CollectionMutation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}
```

Support cast in the same file: `apply_collection_mutation`, `inverse_collection_mutation`, `collection_diff_from_mutation`, `CollectionDiff`, `Identified`, `Patchable`, `ItemPatch`.

Why it must die: `Patch{id, patch}` carries an option-bag payload. The taxonomy forbids option-bag payloads on a mutation outright — they may survive only as diff-INTERNAL types. An all-optional patch struct is itself proof the fields are set one at a time, which is precisely why `update` does not apply to it.

### Type 2 — a non-generic domain enum, unrelated

**Definition: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:736`**

```rust
pub enum CollectionMutation {           // no type parameters
    SetName { name: String },
    AddFolder { folder: CollectionFolder, at: u32 },
    RemoveFolder { folder_id: String },
    MoveFolder { folder_id: String, parent_id: Option<String> },
    RenameFolder { … },
    …
}
```

A domain vocabulary for a collection tree. `Set*`-shaped and CRUD-flavoured, so squarely in DKM's mandate, but **a different job from Type 1, with a different success criterion.** It must never share a work packet or an exit gate with the flow conversion.

Its own doc comment records a deliberate design constraint: `Move*`/`Rename*`/`ReplaceEntryBody` diff as the **whole post-mutation folder/entry record** rather than a field delta, to sidestep the derive engine's lack of nested-`Option` support ("was this field touched, and to what new *optional* value"). This is in tension with the doctrine rule that a diff is built from `(payload, base)` and never a snapshot clone. DKM's W3c design agent must either fix the derive limitation or preserve the shape **deliberately and say so** — silent inheritance is not acceptable.

### `📡️spr` is NOT a definition site

`📡️spr/🦀️component.rs:27` re-exports the name through `crate::os_spr::command`. Module mounts: `📡️spr/🎮️command` at `💻️os/📦️packages/🦀️rust/📦️glue.rs:160`, `🌿️vcs` at :217.

**Consequence — this is the part that changes planning:** the definition does **not** sit behind a frozen claim in a way that forces a three-session serialization. Deleting the re-export is trivial once consumers are gone; the real removal is a single edit in `🌿️vcs`. Any plan that sequences around "the definition is in `📡️spr`" is built on a false premise.

## The 9-file surface, by owning session

| File | Type | Owner | Status |
|---|---|---|---|
| `💻️os/🔨️modules/🌿️vcs/🦀️component.rs` | 1 (**definition**) | UCAS | last step; removal only after all consumers convert |
| `💻️os/🔨️modules/📡️spr/🦀️component.rs` | 1 (re-export) | UCAS | trivial; follows the definition |
| `💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` | 1 (re-export path) | UCAS | trivial; follows the definition |
| `💻️os/🔨️modules/🏪️store/🦀️component.rs` | 1 | UCAS | consumer |
| `💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` (40) | 1 | **DKM** | W3c — decompose per-field |
| `💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` | 1 | **DKM** | W3c |
| `💻️os/🔨️modules/🪐️space/🦀️component.rs` (70) | **2** | **DKM** | W3c — separate packet |
| `💻️os/🦀️component.rs` | 1 | APA | consumer; near end of chain |
| `💻️os/🖥️host/🦀️component.rs` | 1 | APA | consumer; near end of chain |

Plus the plugin-side bridge that is SMO's hard floor: `✏️s/🔌️plugins/🌊️flow/…/🧬️mutations/🦀️component.rs` constructs Type 1 in `from_framework_mutation`/`to_framework_mutation`. DKM must send SMO the target flow enum shape **before** authoring so the bridge changes in step or disappears.

## Elimination order

1. **DKM** converts its three (`🌊️flow/🌿️vcs`, `♾️infinite/…/🕸️dag`, and `🪐️space` as its own packet). Gate: target shapes sent to SMO first.
2. **SMO** updates or deletes the plugin-side flow bridge once the shape lands. Their ticket's exit is plugin-scope (zero banned tokens under `✏️s/`); framework elimination is explicitly not in their close, so this is a continuation, not a blocker on them.
3. **APA** converts `💻️os/🦀️component.rs` and `💻️os/🖥️host/🦀️component.rs`.
4. **UCAS** converts `🏪️store`, then removes the definition in `🌿️vcs` and the `📡️spr` re-exports. Last, because it can only happen once every consumer above is done.

## Standing correction protocol

Every entry above was measured, not remembered. Two sessions have already been wrong from memory on this exact topic in a single day. If you are about to act on a line here, re-run the grep — and if it disagrees, fix this file rather than working around it.
