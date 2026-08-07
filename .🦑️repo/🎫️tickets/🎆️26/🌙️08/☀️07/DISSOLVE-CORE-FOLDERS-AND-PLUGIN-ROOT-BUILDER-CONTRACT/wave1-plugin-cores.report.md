# Wave 1 — plugin cores report

Ticket: `26/08/07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT`

## Summary

Dissolved `🫀️core` under seven plugin/extension trees (A–G). No `pub mod core` remains in those glue files; `crate::core::` references in those trees are cleared. Animate `🎬️core`, cad module core, and root `📜️script.ts` policy were not touched.

## A) `🏗️fem`

- Lifted kernel siblings: `🏗️model`, `➗️formulation`, `🕸️mesh`, `🔢️sparse`, `📏️elements2d`, `🧊️elements3d`, `🧮️analyses`, `🖥️app-surface` (from `🤝️shared`).
- `📦️glue.rs`: flat kernel `#[path]` modules + top-level `register_all_engines`; `semio_plugin!` setup uses `register_all_engines`.
- Internal paths: `crate::model::` (domain), sibling modules for formulation/mesh/…, `crate::app_surface::` for UI helpers.

## B) `📕️norm`

- Lifted: `📄️document`, `🎚️config`, `🖥️app-surface`.
- Glue: `document`, `config`, `app_surface` modules (no `core` facade).
- Bulk remap: `crate::document::`, `crate::config::`, `crate::app_surface::` across the norm tree (~218 prior `crate::core::` sites).

## C) `🔱️trinity`

- Split monolith into `🌳️ast`, `🔤️lexer`, `🧮️executor`, `🗣️language-service`.
- Glue wires four modules; consumers use `executor`, `language_service`, `lexer`, `ast` (jack shell bin updated).
- `TrinityQueryableGraph` lives in `language_service` to avoid executor ↔ language_service cycle.

## D) `🧱️block`

- FOLD: `🦀️component.rs` at plugin root; glue `pub use block_shared::*`; consumers `crate::Block*` (no `core` mod).

## E) `🪐️space`

- FOLD: `🦀️component.rs` at plugin root; fixed `include_str!` paths; glue `pub use space_shared::*`; consumers `crate::demo_space_projection` etc.

## F) `🌊️flow` extension

- Renamed `🧩️extensions/🫀️core` → `🧩️extensions/🔤️primitive`.
- Crate `semio-s-plugin-flow-extension-core` → `semio-s-plugin-flow-extension-primitive`.
- `📋️project.json` paths updated locally.
- Deferred root/workspace paths: `deferred-flow-ext.json`.

## G) `📜️imperative` extension

- Renamed `🧩️extensions/🫀️core` → `🧩️extensions/📣️effect`.
- Crate `semio-s-plugin-imperative-core` → `semio-s-plugin-imperative-effect`.
- `📋️project.json` paths updated locally.
- No live `imperative_module_core::` references in `📇️registry` (deferred rename recorded in `deferred-imperative-ext.json`).

## Tooling

- Migration script: `wave1-plugin-cores.mjs` (this ticket folder).
- Log: `wave1-plugin-cores-log.txt`.

## Verification

- `cargo check` for workspace **blocked** until Wave 2 applies `deferred-flow-ext.json` (root `Cargo.toml` and flow OS module still point at removed `🫀️core` extension path). Same pattern expected for any crate that transitively loads that workspace member.
- Per-plugin `crate::core::` grep under fem/norm/block/space/trinity: **0** matches.

## Wave 2 follow-ups

- Apply `deferred-flow-ext.json` and `deferred-imperative-ext.json` to root `Cargo.toml`, flow plugin/framework `Cargo.toml`, and any remaining `semio-s-plugin-flow-extension-core` / `semio-s-plugin-imperative-core` strings.
- Optional doc comment cleanup (stale `🫀️core` mentions in comments only).

## Out of scope (other agents)

- `🎞️animate/…/🎬️core`
- `📐️cad/🔨️modules/🫀️core`
- `📜️script.ts` policy edits
