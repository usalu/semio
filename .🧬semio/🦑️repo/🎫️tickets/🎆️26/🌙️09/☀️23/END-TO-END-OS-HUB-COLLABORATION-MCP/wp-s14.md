# S14 — 35/35 plugin kinds inside `dev s` (from S13's 30/35)

Slice S14, session 9, 2026-09-23. Continues S13 (`📓️s13-thirty-five-on-rebuilt-guests.md`, cut mid-rebuild).
Bar: every one of the 35 plugin artifact kinds spawn → mutate → undo → redo locally, History rows in en + de;
plus the 60-component guest rebuild carrying S12's `bounded_presence_root_retirement_factory` default.

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read only.

## 0. Infrastructure

| what | value | state |
|---|---|---|
| serve | **6230** → hub 7730 | **HTTP 200** (pre-rebuild staged guests; restart after activate) |
| hub | **7730** bun hold pid recorded in `generated/s14-hub-hold-pid.txt`, empty `s14-boot` | **/readyz 503** (`trusted-catalog-never-published`) — acceptable for local plugin sweep (S11 pattern) |
| wasm mutex | held by **tc5** since 14:44 (trusted-catalog-bootstrap); queue tc5 → wp-o3 → gj3 → wp-o1 → **s14** | s14 waiting; long-lived shell 29379 keeps hub+queue alive |
| restage | `s14-restage-all.sh` resume (4 ok markers: writer/mathematical/procedural/flow) | queued |
| serve helper | `s14-serve.sh` default **6230** / hub **7730** | updated |


## 1. Inherited five survivors (from S13 §1)

| kind | S13 reading | S14 plan |
|---|---|---|
| stdio | product fix landed (kit args + editor bridges) — needs guest rebuild | rebuild + remeasure |
| energy | probe re-pin `create-zone` with all required fields | remeasure |
| trinity | probe re-pin `patchNodes` with live nodeIds | remeasure |
| writer | declaration: real verbs `in_palette: false` | root-fix so rail can mutate |
| sourcing | declaration: curation verbs hidden | root-fix so rail can mutate |

## 2. Product fixes this slice (measured in source; guests pending rebuild)

| kind | root fix | file |
|---|---|---|
| writer | `setText` promoted from `writer_hidden_operation` to palette Mutation with required `text` arg | `✒️writer/…/✏️editor/🦀️.rs` |
| sourcing | `curationSetCount` promoted from `hidden_operation` to palette Mutation with `objectId`/`delta`/`value` args | `🪵️sourcing/…/✏️editor/🦀️.rs` |
| stdio / energy / trinity | inherited from S13 product+probe pins (already in tree) | S13 §1 + `🐍️s6-all-kinds-sweep.mjs` |
| sweep | pins `writer→setText`, `sourcing.objectId=LIVE_ID`, app pins for stdio/writer/sourcing/trinity/energy | `🐍️s6-all-kinds-sweep.mjs` |



## 3. 60-component guest rebuild (filling)

- Resume restage under mutex after tc5 released; progressed **21/60** materialize-ok then aborted on `architect`.
- **Root fix:** `architect` program editor `absorb_register_mutation` called `.apply` without `MutationDiff` in scope — added `use protocol::{Mutation, MutationDiff}` (file linked as `links/architect-editor.rs`).
- Re-queued resume (ok markers for 21 completed). Waiting on mutex (tc5 again at 17:06).
- Presence factory default confirmed in framework source; guests still pending activate for hash verify.


## 4. Full sweep en + de (filling)

## 5. Honest gaps (filling)

## 6. Files changed (filling)

| file | change |
|---|---|
| architect program `editor` rs | import `MutationDiff` so register absorb compiles |
| `s14-serve.sh` | default port 6230 / hub 7730 |
| `s14-restage-all.sh` | resume-capable 60-component chain |
| `s6-all-kinds-sweep.mjs` | predecessor pins (writer/sourcing/energy/trinity) — unchanged this turn |
| writer/sourcing editors | predecessor palette promotions — verified still in source |

