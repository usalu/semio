# 🌊 Norm Artifact Compile Wave (2026-11-13)

Shared brief for wave agents. Repo /Users/Shared/semio (macOS arm64, 16 GB). Obey /Users/Shared/semio/AGENTS.md: no git-modifying commands, no worktrees, re-read a file before editing (others edit live), concise code, no comments inside definitions, greenfield root-cause fixes (no compat shims, no `#[allow]` to silence), docstrings start with an emoji. Do not open/close tickets.

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END`.

## Situation
`repo cargo --store=shared --configuration=prototype -- check --keep-going --target wasm32-wasip2 -p semio-hub-norm-component` (run from `🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust`; wrapper `T/normcomp-check.sh check <logname>`) compiles all framework and plugin libraries; 23 norm artifact crates under `✏️s/🔌️plugins/📕️norm/🗿️artifacts` fail with 536 errors. Full error log: `T/🗑️generated/normcomp-c5-keep.log` (grep your crates' sections; do not delete it). Root-cause summary: `T/normcomp.md`.

Known root causes:
1. `BTreeMap` inside `#[derive(RetainedClone)]` types: the framework now offers only `RetainedOrderedMap` (`🧰️framework/…/🌱️value/🧬️retained-clone/🗺️ordered-map`). The gis and procedural plugins already migrated — copy their pattern exactly (find with grep `RetainedOrderedMap` under `✏️s/🔌️plugins/🌍️gis` and `🌀️procedural`).
2. Missing trait impls after framework API changes (`DslField`, `Serialize`/`Deserialize`, `MutationLeaf`, `From<…>`), duplicate imports (E0252), mismatched types (E0308), orphan impls on foreign types (E0117 → implement on a local type or move the impl to the owning crate), missing `semio_framework_job` dependency in manifests.

## Rules for this wave
- Edit only your assigned crates (and a framework file only if the root cause is truly there; then note it in your report).
- Verify with `repo cargo --store=shared --configuration=prototype -- check --keep-going --target wasm32-wasip2 -p <your crate> [-p …]` from the hub component package directory above (same workspace/lock as the component). Never call cargo directly. ONE cargo at a time per agent, run in the background into `T/🗑️generated/<agent>-<n>.log` and wait; five agents share the gate, so do as much source work as possible per compile cycle (fix ALL errors of the log for your crates before the first compile).
- Never start `repo run` (graph) commands.
- Temporary logging carries `[DEBUG] ` and is removed at the end.
- Report: `T/<agent>.md` (crate → errors before/after, files changed, anything left). Final chat message ≤ 8 lines.
