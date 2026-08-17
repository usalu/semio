# D1 — Fastener Commands Report

Agent: D1  
Ticket: `26/08/09/PUZZLE-DESIGN-PARITY`  
Owned path: `✏️s/🔌️plugins/🧩puzzle/🎛️apps/🖐️5d/🎮️commands/🔗️fastener/`  
App root **not** edited (D7 owns `🎛️apps/🖐️5d/🦀️component.rs`).

## Delivered

Created:

```
🎮️commands/🔗️fastener/🦀️component.rs
```

Public handlers (all take `Puzzle5dActionCtx` + optional JSON args; mutations land via existing `handle_action_impl` → `puzzle5d_operations_from_document_change` Emit path once dispatched):

| Handler | Suggested action id | Args |
|---------|---------------------|------|
| `create_fastener` | `createFastener` | `source`/`target` (aliases `attracting`/`attracted`); optional `id`/`fastenerId`, `fastenerKind`/`edgeKind`, and any of the 8 params |
| `delete_fastener` | `deleteFastener` | `id` or `fastenerId` |
| `retarget_fastener` | `retargetFastener` | `id`/`fastenerId` + `source` and/or `target` |
| `edit_fastener` | `editFastener` | `id`/`fastenerId` + any subset of the 8 params **or** inspector shape `field` + `value`/`delta` (includes **`x`/`y`**) |
| `proximity_connect` | `proximityConnect` | `partId`/`objectId`; optional `radius` (default `PUZZLE5D_PROXIMITY_RADIUS`) |

### Behaviour notes

- **Create** mirrors 3d `createAttraction`: rejects empty/equal endpoints, dangling grips, already-connected pairs, and explicit kind-compatibility failures. Missing grip-kind metadata is permissive (same spirit as board `edgeCreate`).
- **Eight parameters** on create/edit: `gap`, `shift`, `rise`, `rotation`, `turn`, `tilt`, `x`, `y` (defaults `0.0`). Degrees for rotation/turn/tilt per normative spec.
- **Proximity connect** is the 3d relocate auto-attract twin (that helper lives in 3d `transform`, not in `attraction`). Stationary peer grip → `source`; moved part's first grip → `target` (matches engine mapping `source=attracting`, `target=attracted`).
- **Inspection `x`/`y`**: `editFastener` accepts both batch keys and `field: "x"|"y"` with `value`/`delta`. Existing `patchFastener` in `✏️patch` still lacks `x`/`y` (out of D1 ownership) — D7 should either route `patchFastener` through `fastener::edit_fastener` or extend patch; D4 inspection can call `editFastener` immediately.

## Prerequisite for compile (D7 must do first)

Artifact `Puzzle5dFastener` already has `x`/`y`. The **app structural twin** does not yet. Before registering the module, add to `Puzzle5dFastener` in `🎛️apps/🖐️5d/🦀️component.rs`:

```rust
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
```

Then update every `Puzzle5dFastener { ... }` literal in that file (and in `🎮️commands/🔄️transform` `world_relocate`) with `x: 0.0, y: 0.0`.

## Exact registration snippet for D7

### 1) `📦️packages/🦀️rust/📦️glue.rs` — puzzle5d `commands` mod

Insert next to `part` / before `patch` (or beside other lifecycle mods):

```rust
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔗️fastener/🦀️component.rs"]
            pub mod fastener;
```

### 2) App root imports

Change:

```rust
use crate::apps::puzzle5d::commands::{board, brush, camera, engagement, example, fill, grid, hover, lod, part, patch, selection as selection_commands, sun, transform, utility};
```

To:

```rust
use crate::apps::puzzle5d::commands::{board, brush, camera, engagement, example, fastener, fill, grid, hover, lod, part, patch, selection as selection_commands, sun, transform, utility};
```

### 3) `Puzzle5dCommand` enum variants

Add with the other mutations (near `PatchFastener`):

```rust
    CreateFastener = "createFastener",
    DeleteFastener = "deleteFastener",
    RetargetFastener = "retargetFastener",
    EditFastener = "editFastener",
    ProximityConnect = "proximityConnect",
```

### 4) `dispatch_puzzle5d_action` arms

```rust
        "createFastener" => fastener::create_fastener(ctx, args),
        "deleteFastener" => fastener::delete_fastener(ctx, args),
        "retargetFastener" => fastener::retarget_fastener(ctx, args),
        "editFastener" => fastener::edit_fastener(ctx, args),
        "proximityConnect" => fastener::proximity_connect(ctx, args),
```

Optional: keep `patchFastener` but forward so inspection x/y works without a second code path:

```rust
        "patchFastener" => fastener::edit_fastener(ctx, args),
```

(or extend `patch::patch_fastener` with `x`/`y` arms — either is fine).

### 5) Manifest `ActionDefinition`s (`ActionKind::Mutation`)

```rust
            .mutation("createFastener", LocalizedLabel::native("Create Fastener", "Verbinder erstellen"))
            .action_with(ActionDefinition::new_catalog("deleteFastener", LocalizedLabel::native("Delete Fastener", "Verbinder löschen"), ActionKind::Mutation).category("targets"))
            .mutation("retargetFastener", LocalizedLabel::native("Retarget Fastener", "Verbinder umbinden"))
            .mutation("editFastener", LocalizedLabel::native("Edit Fastener", "Verbinder bearbeiten"))
            .mutation("proximityConnect", LocalizedLabel::native("Proximity Connect", "Nähe verbinden"))
```

### 6) Optional follow-ups (not blocking registration)

- Context menu: when `selection.fastener_ids` is non-empty, offer `deleteFastener` (3d does this for attractions).
- `world_relocate` in `🔄️transform` can call `fastener::proximity_connect` instead of its inline loop (corrects source/target orientation to match 3d).
- D6 terminology: add native/reuse × en/de labels for the new verbs if not covered by the generic `fastener` cell.

## Validation run

Scoped compile of the new module requires glue + `x`/`y` on the app twin (single-owner files). Ran instead:

1. `rustfmt --edition 2021` on the new file — clean.
2. Structural check (`🧪d1-fastener-validate.json`): all five `pub fn`s present; eight-param keys present; `app_twin_has_xy: false` (expected blocker for D7).

Full `bun nx test @semio-tech/puzzle-plugin` / cargo test of these actions is **blocked until D7 pastes the snippet** (and adds twin `x`/`y`).

## Files touched

| Path | Change |
|------|--------|
| `🎛️apps/🖐️5d/🎮️commands/🔗️fastener/🦀️component.rs` | **created** |
| `🧪d1-fastener-command-report.md` | this report |
| `🧪d1-fastener-validate.json` | structural validate log |

No git mutations. No edits outside ownership.
