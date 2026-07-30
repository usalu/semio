---
technology: architect
emoji: 🏛️
---

# Architect

Headless architectural __KEEP_pluginming__ technology: [architect/plugin](plugin/rs/lib.rs) domain model and analysis (`architect_spine`), [architect/plugin](plugin/rs/lib.rs) s/OS DocumentApp for program editing with undirected adjacency-matrix UI.

## Program (`architect_spine`)

- **Spine:** Objectives → stakeholders → users → activities → functions → elements → requirements → relationships/adjacencies → constraints → criteria → decisions → validation
- **Registers:** 65 feature-area typed registers with VCS `CollectionOp` patches
- **Adjacency:** Undirected canonical pairs, conflict detection, matrix view
- **Analysis:** gap, conflict, dependency, capacity, workflow, risk, scenario, report, search, trace, exchange

## Conventions

- Docstrings start with a unique emoji; no comments inside definitions
- Regions in `lib.rs` and `src/` modules
- `bun ./script.ts test` via nx `@semio-tech/architect-spine`
- Do not depend on coda, compose, puzzle, or geometry crates

## Stack

- Rust crate `architect_spine`
- `mathematical_graph` for undirected topology helpers
- `vcs` for document operations and undo
- Plugin: `semio-framework-plugin`, WASM `semio:architect`
