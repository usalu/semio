# `🌿️vcs` — the packet that proves the thesis

APA (#2549) reported that `🌿️vcs`'s demo app is the **only app in the repo** that cannot port to the new pure `genesis() -> Vec<Mutation>` (which replaced `seed(&mut ArtifactStore)`), because it needs multi-command history, checkpoints and alternatives that a flat mutation list cannot express. They correctly did **not** work around it.

They read it as a limitation of `genesis()`. It isn't. It is the clearest evidence in the codebase that **the engine had to leave the artifact**, and `🌿️vcs` is therefore claimed by this ticket rather than dissolved by rote.

## What the demo actually does

`🎛️apps/🌿️vcs/🦀️component.rs:82` — `seed_vcs_demo_history(store: &mut ArtifactStore<…>)`, whose own docstring says its "whole point is exercising the history UI (swimlane graph, checkpoints, alternatives, undo/redo)". It issues a *sequence of commands*:

```rust
store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: feature_a_id });
```

`Vec<Mutation>` cannot express that — and the reason is not expressiveness. It is that **`CheckoutCheckpoint` and `SwitchAlternative` are not mutations at all.** They change nothing about document content. They change *which version you are looking at*. That is navigation — machine state — wearing a mutation's clothes.

A flat mutation list is exactly the right shape for document edits and exactly the wrong shape for a walk through a history graph. `genesis()` is not too weak; it is being asked to encode the wrong thing.

## The misfiling, verified

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4230` — the persisted `ArtifactEnvelope` carries:

| field | what it is | where it belongs |
|---|---|---|
| `schema`, `id` | identity | persisted shared ✅ |
| `vcs`, `backbone` | the history graph itself — checkpoints, alternatives | persisted shared ✅ |
| **`active_alternative_id`** | *which branch this viewer is on* | **machine state** — persisted **local-only** |
| **`cursor`** | *where this viewer's caret/selection is* | **ephemeral shared** — presence |

Two of six fields in the persisted document envelope are not document state. `cursor` is the starkest: a caret position, broadcast per-viewer, stored in the artifact that every collaborator shares. It is the canonical presence value — the brief names cursors and cameras explicitly — and it is currently persisted as content.

This is not a `🌿️vcs` bug. `ArtifactEnvelope` is framework-wide, so **every artifact in the repo** carries these two fields. `🌿️vcs` is simply the only app that exercises them hard enough to make the seam visible.

## The shape

The four-way classification the ticket is built on resolves this cleanly:

- **persisted shared** — the history graph (`vcs.checkpoints`, alternatives, and the mutations hanging off them). Real content, VCS-tracked, `genesis()` stays a pure `Vec<Mutation>` producing exactly this.
- **persisted local-only** — `active_alternative_id`. Which branch *I* have checked out survives my restart and is nobody else's business. Belongs in config, not the envelope.
- **ephemeral shared** — `cursor`. Broadcast to the space on change, never persisted, dropped when I disconnect.
- **ephemeral local-only** — in-flight UI state during a checkout animation.

`CheckoutCheckpoint` / `SwitchAlternative` become **machine events**, not artifact commands. The app's machine holds "where am I in the history graph"; transitioning emits a *presence* event (others see me move) and writes *local* config (I resume there), while emitting **no mutation at all** — because nothing about the document changed.

The demo history then splits honestly in two:
1. `genesis() -> Vec<Mutation>` builds the checkpoint/alternative **graph** — pure, flat, exactly what the new signature offers.
2. A short **machine script** walks that graph to arrange the demo's viewing state.

Note the payoff: the demo becomes *more* faithful, not less. Today it fakes a user's navigation by mutating a store; afterwards it does what a user actually does — drives a machine.

## Why this is the exemplar for the whole ticket

Every other packet dissolves an `⚙️engine` whose contents were misplaced *helpers*. This one dissolves a genuine confusion about **what a document is**. The generic lesson, which every packet should apply: if a "mutation" changes what you *see* rather than what *is*, it was never a mutation.

## Instruction already sent to APA

Leave `🌿️vcs` on the `setup()` escape hatch, do **not** bend `genesis()` to fit it, and do not let it block deleting `PluginBuilder::setup()` for the other 32 plugins. This ticket takes it.

## Sequencing

Touches `ArtifactEnvelope` in `🔌️plugin/🦀️component.rs` — the shared file with a live queue (UCAS #2548 → APA #2549 → this ticket). Envelope surgery is **not** part of the `🌿️vcs` packet itself; it is a separate framework change requiring the write slot, and it lands only after APA's 27-plugin conversion is out of that file. Until then `🌿️vcs` is documented, claimed, and parked — not started.
