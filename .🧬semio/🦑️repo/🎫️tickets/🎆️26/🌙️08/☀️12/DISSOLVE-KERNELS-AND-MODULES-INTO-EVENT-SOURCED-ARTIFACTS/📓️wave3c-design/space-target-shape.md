# `🪐️space` — target mutation shape

**For SMO's review before any authoring.** Target file: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs` (the **framework module** — NOT the `✏️s/🔌️plugins/🪐️space/` plugin, which is SMO's and already released).

Separate work packet from flow, per SMO's instruction: this is the **non-generic domain enum** at `:736`, unrelated to the generic wrapper at `💻️os/🔨️modules/🌿️vcs/🦀️component.rs:280` despite the shared name.

## Headline: the verbs are mostly already right. The **diff shape** is the defect.

The 70-hit count overstates the job. Read as vocabulary, this enum is close to conformant — `AddFolder`/`RemoveFolder`/`RenameFolder`/`ReplaceEntryBody` are real semantic verbs with real addresses, not a generic collection wrapper. What is wrong is (a) three verb choices and (b) **the diff stores whole post-mutation records instead of deltas**, which is the doctrine violation and the part worth the effort.

## Current shape (`:736`, 10 variants)

```rust
pub enum CollectionMutation {
    SetName { name },
    AddFolder { folder: CollectionFolder, at: u32 },   RemoveFolder { folder_id },
    MoveFolder { folder_id, parent_id: Option<String> }, RenameFolder { folder_id, name },
    AddEntry { entry: CollectionEntry, at: u32 },       RemoveEntry { entry_id },
    MoveEntry { entry_id, folder_id: Option<String> },   RenameEntry { entry_id, name },
    ReplaceEntryBody { entry_id, body: Box<ArtifactBody> },
}
```

## Target shape

```rust
pub enum CollectionMutation {
    RenameCollection { new_name: String },

    CreateFolder { folder: CollectionFolder, index: u32 },
    DeleteFolder { folder_id: String },
    MoveToCollection { folder_id: String, new_parent: Option<String> },
    RenameFolder { folder_id: String, new_name: String },

    CreateEntry { entry: CollectionEntry, index: u32 },
    DeleteEntry { entry_id: String },
    MoveToFolder { entry_id: String, new_folder: Option<String> },
    RenameEntry { entry_id: String, new_name: String },
    ReplaceEntryBody { entry_id: String, new_body: Box<ArtifactBody> },
}
```

### The three verb changes, justified

1. **`SetName{name}` → `RenameCollection{new_name}`.** `set` requires a real address ("which element, what value"); here the target is the collection root, so it is a document-level identity field. That is `rename` — and it matches the ruling already applied to `set-active-app` → `change-active-app` on the platform module, with `rename` rather than `change` because `name` is the identity-bearing field.
2. **`Add*`/`Remove*` → `Create*`/`Delete*`.** Folders and entries are id-keyed entities with independent existence. The taxonomy reserves `add`/`remove` for set-like membership and ordered-list detach; `create`/`delete` is for id-keyed entities, and `create` explicitly takes "full initial payload (+ optional `index`)", which is exactly `{folder, at}`. `delete` also carries the cascade-capture obligation, which matters here: deleting a folder must capture its subtree.
3. **`MoveFolder`/`MoveEntry` → `MoveToCollection`/`MoveToFolder`.** ⚠️ **RULED BY SMO — and it corrected DKM's proposal.** DKM had proposed `ChangeFolderParent`/`ChangeEntryFolder`, reasoning that re-parenting is neither `move` (absolute spatial) nor `reorder` (never spatial), so by the field test — one scalar link field, identity and contents preserved — it is `change`. **That reasoning is correct in isolation but incomplete.** `📓️derivation-rules.md:23` rule 5 already defines a hierarchy verb:

   > **Per hierarchy** (parent_id / nesting field): `move-to-<container>{id, new_parent}`

   A more specific rule wins over the general scalar rule. So `move-to-folder{entry_id, new_folder}` and `move-to-collection{folder_id, new_parent}`, named for the real container field. Two reasons SMO gave for why this beats `change-*`, both worth preserving:
   - **It reads as the gesture.** People will keep reaching for "move a folder"; a vocabulary that fights natural language loses. `move-to` gives them the word while keeping bare `move` reserved for spatial repositioning — the distinction is carried by the suffix, not by forbidding the root.
   - **It keeps hierarchy operations findable as a family.** Rule 5 pairs `move-to` with `group`/`ungroup` and `flatten`/`unflatten`. Filed as `change-folder-parent`, re-parenting stops looking like a hierarchy operation and starts looking like a scalar setter that happens to hold an id.

   Inverse: itself, with the old parent read from `base`. **No new vocabulary was needed** — the instinct not to mint `reparent` was right, but so was not falling back to `change`.

`RenameFolder`/`RenameEntry`/`ReplaceEntryBody` are already correct and carry over unchanged (modulo `name` → `new_name` per the naming mechanics).

## ⚠️ The design constraint, confronted rather than inherited

`CollectionDiff` (immediately after the enum) stores **whole post-mutation records**:

```rust
pub struct CollectionDiff {
    pub name: Option<String>,
    pub add_folder: Option<CollectionFolder>,
    /// 🔢️ Companion to `add_folder` — the insertion index … kept as a sibling field rather than
    /// nested inside `add_folder` since the derive engine has no first-class "record + position" shape.
    pub add_folder_at: Option<u32>,
    pub remove_folder_id: Option<String>,
    pub move_folder: Option<CollectionFolder>,     // ← whole record
    pub rename_folder: Option<CollectionFolder>,   // ← whole record
    …
}
```

The enum's own doc comment states the reason: `Move*`/`Rename*`/`ReplaceEntryBody` diff as the whole post-mutation record "rather than a bare field delta — sidesteps the derive engine's lack of nested-`Option` support (a 'was this field touched, and to what new *optional* value' diff shape) while staying exactly as replayable." The `add_folder_at` comment independently confirms a second gap: no "record + position" composite shape.

**Both gaps are real.** I verified them in the declaration rather than taking the comment's word for it.

**Does it violate the doctrine?** Yes, on the letter and on the substance. The mutation policy's third mechanical gate requires a real `pub fn diff` that builds the sparse diff **directly from `(payload, base)`** — never apply-then-capture, never a snapshot clone. A `rename_folder: Option<CollectionFolder>` carrying the entire post-rename folder is a record clone standing in for a two-field delta `{folder_id, new_name}`. The comment's defence ("staying exactly as replayable") is true and beside the point: replayability is not the property the rule protects. The rule exists so a diff records *what changed*, so that concurrent edits to disjoint fields of the same folder can merge — which a whole-record diff cannot do, because it asserts every field.

**DKM's position: handcraft the diff; do NOT extend the derive engine.**

- The taxonomy already sanctions this — domain verbs require "handcrafted diff + inverse", and the four mechanical gates demand a *real* `pub fn diff`, not a derived one. Handcrafting is the conformant path, not a workaround.
- Extending the derive engine to support nested `Option` and a record+position composite is a change to shared machinery that every other facet in the repo compiles through, in the middle of a five-session refactor. The blast radius is the whole `dsl::DslDiff` surface; the benefit accrues to one facet. Wrong trade today.
- The file already handcrafts its `OpText`/`OpBinary` codecs for exactly this class of reason (`//#region 🔖️HandcraftedOpCodecs`, with the note "Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only"). Handcrafting the diff is consistent with how this module already handles derive gaps.

So: `CollectionDiff` becomes sparse per-field (`renamed_folder: Option<(String, String)>` shaped as an explicit id+new-value pair, `changed_folder_parent: Option<(String, Option<String>)>`, etc.), built directly from `(payload, base)`, with `#[derive(dsl::DslDiff)]` dropped in favour of a handwritten impl if the derive cannot express it.

**If SMO would rather fix the derive engine properly**, that is a defensible alternative and I'll take the instruction — but it should be its own ticket with its own blast-radius assessment, not a side effect of this wave.

## Inverse story

| Variant | `inverse(base)` |
|---|---|
| `RenameCollection` | itself, old name from `base` |
| `CreateFolder` | `DeleteFolder{folder_id}` |
| `DeleteFolder` | `CreateFolder{folder, index}` from `base` **plus the captured subtree** — every descendant folder/entry re-created leaves-first |
| `ChangeFolderParent` / `ChangeEntryFolder` | itself, old parent from `base` |
| `RenameFolder` / `RenameEntry` | itself, **old name looked up from `base`**, never a captured id |
| `CreateEntry` | `DeleteEntry{entry_id}` |
| `DeleteEntry` | `CreateEntry{entry, index}` from `base` |
| `ReplaceEntryBody` | itself, old body from `base` |

All return `Vec::new()` when the target id is absent from `base`.

## Consumer impact, by owning session

| File | Owner | Change |
|---|---|---|
| `💻️os/🔨️modules/🪐️space/🦀️component.rs` | **DKM** | the enum, the handcrafted sparse diff, triad dirs, the inverse impls |
| `✏️s/🔌️plugins/🪐️space/**` | **SMO** | ⚠️ if anything there constructs these variants by name, the rename `Add*`→`Create*` / `Move*`→`Change*Parent` reaches it. `🪐️space` is RELEASED in your predicate file, so **DKM will send you the exact delta rather than reaching in**. Not yet enumerated — deliberately deferred until you rule on the verbs, since the list depends on the final names |
| `💻️os/🔨️modules/🌿️vcs/🦀️component.rs:280` | UCAS | untouched — different type |

## SMO rulings — received 2026-08-12, binding

| Question | Ruling |
|---|---|
| hierarchy re-parenting verb | ⚠️ **`move-to-<container>`** per derivation rule 5 — corrects DKM's `change-*` proposal. See §3 above |
| `SetName` → `RenameCollection` | ✅ **APPROVED** — identity field on the document root is `rename`. Parallel to `set-active-app`→`change-active-app`; both lose `set`, differing only in whether the field is the identity |
| `Add*`/`Remove*` → `Create*`/`Delete*` | ✅ **APPROVED** — id-keyed entities; `create` takes "full initial payload (+ optional index)", which is `{folder, at}` exactly |
| handcraft the diff vs extend the derive engine | ✅ **APPROVED: handcraft.** SMO adopted DKM's reasoning as the ruling — *replayability isn't the property the rule protects; mergeability is. A whole-record diff asserts every field, so two users renaming a folder and moving it cannot merge.* The comment defending the current shape is true and irrelevant: it answers a question the rule wasn't asking. Handcrafting follows local precedent (`//#region 🔖️HandcraftedOpCodecs`), it is not an exception |

**Derive-engine gap → write up as a separate finding, do not fix here.** Two concrete misses, both verified in the declaration rather than taken from the comment: (1) no nested-`Option` support — no "was this field touched, and to what new *optional* value" shape; (2) no "record + position" composite, which is why `add_folder_at` exists as a sibling field to `add_folder`. Extending `dsl::DslDiff` mid-refactor has a blast radius of every facet in the repo to benefit one. Whoever takes that ticket should start from these measurements rather than rediscover them.

## Open questions still outstanding

1. **`CreateFolder{index: u32}`**: the field is `at: u32` today and clamps (`>= folders.len()` appends). Keep clamping, or make out-of-range an error? Clamping is friendlier but makes the inverse index ambiguous when it clamped.
2. **`DeleteFolder` cascade**: confirming the inverse must capture and restore the whole subtree, not just the folder record.
