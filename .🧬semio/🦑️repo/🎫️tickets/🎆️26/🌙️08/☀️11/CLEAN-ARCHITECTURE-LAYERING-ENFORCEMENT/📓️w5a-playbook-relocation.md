# w5a — playbook module relocation (os-flow → playbook plugin): STOPPED, not performed

## Assignment
Move `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs` out of the os-flow product
(`semio-framework-os-flow`, mounted today via `#[path]` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs:45`) into a new
plugin-owned module leaf inside the playbook plugin (e.g.
`✏️s/🔌️plugins/📖️playbook/🔨️modules/🧠️core/🦀️component.rs`), repoint the playbook plugin's own
`📦️glue.rs` to mount it there instead of its current `extern crate flow; pub use flow::playbook;`
re-export (line 25-26), and fix every cross-reference.

## Read first
Read the full 1700-line source file. It is exactly what the task briefing said: a self-contained
playbook-domain module (`PlaybookStep`/`PlaybookBlock`/`PlaybookSpec`/`PlaybookMutation`/`PlaybookDiff`
+ `dsl`/`pack`/`OpText` codec impls, plus the nested `generation_forms` and `builder_kit` sub-modules) —
generic-sounding but genuinely playbook-domain-specific, no reason for it to live in os-flow's own tree.
Confirmed the destination `📦️glue.rs` (`✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs`) already
depends on `flow` (`extern crate flow;` / Cargo dep `flow = { package = "semio-framework-os-flow" }`)
purely to re-export this module — so the plugin already "owns" it in spirit, just not physically.
Confirmed the mount-pattern precedent for a plugin's own `🔨️modules/<name>/🦀️component.rs` leaf
(`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs:26-29` and
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs:1664-1667`, both
`#[path = "."] pub mod modules { #[path = "../../🔨️modules/<name>/🦀️component.rs"] pub mod <name>; }`)
— this part of the plan is sound and I would have used exactly that shape.

## Why I stopped before touching any file
Grepped the whole workspace for `flow::playbook` (the only way anything outside os-flow currently reaches
this module) and separately for internal `playbook::`/`crate::playbook::` usage inside the os-flow crate
itself. Two findings, either one individually would be enough to stop; together they rule out a clean
move entirely as scoped:

**1. os-flow's own `vcs` component depends on the module internally (same-crate, not a mount artifact).**
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:880` does
`use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};`
and uses it for real logic (`widget_to_playbook_block`, `fixture.widgets.iter().filter_map(...)` around
line 929-1028) — this is os-flow's own VCS fixture-generation code needing the playbook domain types, not
just the `#[path]` mount declaration. This is exactly the case my task briefing flagged as "unlikely, but
check" (step 4): "if the os-flow crate itself needs something from playbook's core... if you find this is
needed, STOP, do not create it, report the conflict instead." It is needed. Making os-flow (a framework/os
product) depend on `semio-s-plugin-playbook` (an `✏️s` plugin) to satisfy this would be a new, backwards
layering violation of exactly the kind this whole ticket (CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT) exists
to remove, not add.

**2. Removing the mount from os-flow breaks 32 other files across 4 separate crates, all consuming
`flow::playbook::*` (i.e. depending on `semio-framework-os-flow` directly, not on the playbook plugin) —
far beyond "any file that imports from this module's current mount path" as a small, contained set:**

```
grep -rl "flow::playbook" --include="*.rs" .   →  32 files
```

Broken down by crate (each is a *different* Cargo package, so each would need a *new* Cargo dependency
edge onto `semio-s-plugin-playbook` to keep compiling, not just an import-path fix):

- `✏️s/🔌️plugins/🌀️procedural/…` — 26 files (its own `Cargo.toml` depends on `flow` directly, not on
  `semio-s-plugin-playbook`: `flow = { path = "...os/🔨️modules/🌊️flow/...", package = "semio-framework-os-flow" }`).
  Covers `procedural2d` and `procedural3d` artifact standards (schema/snapshot/diff/mutations, both text
  and binary codecs) plus the 3d/2d generate-mode UI windows and commands.
- `✏️s/🔌️plugins/📋️forms/…` — 3 files (`📦️glue.rs` re-exports `pub use flow::playbook;` itself, plus
  2 files in `🗿️artifacts/📋️forms/…/🧬️schema/{📸️snapshot,🧬️mutations}/🦀️component.rs` — same
  direct-`flow`-dependency situation).
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` — the flow plugin's own glue re-exports
  `pub use flow::playbook;` too (line 18).
- `✏️s/🔌️plugins/📖️playbook/…` — 2 files: `📦️glue.rs` itself (the one I was told to repoint — fine,
  that's the intended edit) and `🧩️extensions/🌀️procedural/🦀️component.rs:5`
  (`use flow::playbook::{visible_blocks, PlaybookBlock};` — this one *is* inside the destination plugin's
  own crate, so it alone would be a trivial `crate::playbook::` repoint once the module physically lives
  there).

None of `procedural`, `forms`, or the `flow` plugin currently list `semio-s-plugin-playbook` as a
dependency (checked each `Cargo.toml`) — they all reach the module exclusively via the `flow` crate that
currently owns it. Repointing 29 of those 32 files (all but the 2 already inside `📖️playbook`'s own tree)
requires either (a) adding a new plugin→plugin Cargo dependency edge from `procedural`, `forms`, and
`flow` onto `semio-s-plugin-playbook` — an architecture decision with cross-plugin fan-out well outside a
"relocate one module + fix its call sites" task, or (b) leaving `flow::playbook` as a working re-export
forwarding into the playbook plugin, which would make os-flow depend on the playbook plugin for #1 above
anyway (same violation) — there is no version of "remove the mount from os-flow, mount fresh in playbook
plugin" that satisfies both #1 and #2 without introducing a new layering edge somewhere.

## What I did NOT do
Made zero edits. Did not touch `🦀️component.rs` (source), either `📦️glue.rs`, or any of the 32
consumer files. `git status` before and after this session shows no changes from this ticket to any file
under `🧰️framework/…/📖️playbook/`, `🧰️framework/…/🌊️flow/`, or `✏️s/🔌️plugins/📖️playbook/`. Repo is in
its original, safely-compiling state with respect to this task.

## Recommendation for whoever picks this back up
This needs to be split, not attempted as one relocation:
1. First decide, at the architecture-owner level, whether `procedural`/`forms`/`flow` plugins are
   *allowed* to depend on the `playbook` plugin directly (plugin→plugin dependency), or whether the
   playbook domain module needs a third home neither `os-flow` nor `s-plugin-playbook` (a lower-level
   shared crate both flow and playbook depend on) so that `os-flow`'s own `vcs` component and the
   `playbook` plugin can each depend on it without either depending on the other.
2. Only once that's settled does "move `component.rs` and fix N call sites" become a well-scoped,
   non-layering-violating task — at that point it's mechanical (the `#[path]`-mount pattern from
   `✏️s/🔌️plugins/🌍️gis` / `🧩️puzzle` above is the right shape for the destination mount).

## Files touched
None.
