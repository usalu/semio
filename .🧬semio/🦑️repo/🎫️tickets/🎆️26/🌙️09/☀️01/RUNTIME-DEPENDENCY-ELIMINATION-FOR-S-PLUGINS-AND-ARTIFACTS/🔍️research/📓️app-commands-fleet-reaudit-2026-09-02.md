# `app_commands!` Fleet Re-audit

Date: 2026-09-02

## Outcome

The reported serde regression class is closed in the current repository state. All three
`semio_framework_plugin::app_commands!` arms derive only `Clone`, `Debug`, `PartialEq`, first-party
`ToValue`/`FromValue`, and `dsl::DslOps`. No serde JSON wire exists in the generated implementation,
so a serde opt-out would be the wrong API: serde is globally absent from the macro contract.

The removal is already present in `HEAD` (`f15dbf0d83a`, 2026-09-01). This re-audit adds a stronger
compile-time regression fixture: all four fake payload types in `app_commands_tests` now omit
`Serialize`/`Deserialize`. Reintroducing serde derives on any generated enum arm will therefore fail
the framework test build because its wrapped payloads do not implement those traits.

## Current Fleet Inventory

The original textual count conflated mentions with expansions. Current Rust source has 146 files
containing the string `app_commands!`, but only 50 files match the invocation form
`app_commands!\s*\{`. Those 50 expansions belong to 29 plugin crates.

Every one of the 29 owning manifests currently has no direct `serde` dependency:

| Expansions | Package |
| ---: | --- |
| 15 | `semio-s-plugin-norm` |
| 3 | `semio-s-plugin-block` |
| 3 | `semio-s-plugin-space` |
| 2 | `semio-s-plugin-fem` |
| 2 | `semio-s-plugin-gis` |
| 2 | `semio-s-plugin-procedural` |
| 1 | `semio-s-plugin-animate` |
| 1 | `semio-s-plugin-architect` |
| 1 | `semio-s-plugin-cad` |
| 1 | `semio-s-plugin-dag` |
| 1 | `semio-s-plugin-demonstrator` |
| 1 | `semio-s-plugin-draw` |
| 1 | `semio-s-plugin-flow` |
| 1 | `semio-s-plugin-forms` |
| 1 | `semio-s-plugin-imperative` |
| 1 | `semio-s-plugin-layout` |
| 1 | `semio-s-plugin-lowpoly` |
| 1 | `semio-s-plugin-mathematical` |
| 1 | `semio-s-plugin-note` |
| 1 | `semio-s-plugin-playbook` |
| 1 | `semio-s-plugin-process` |
| 1 | `semio-s-plugin-raster` |
| 1 | `semio-s-plugin-reasoning-mindmap` |
| 1 | `semio-s-plugin-remodel` |
| 1 | `semio-s-plugin-sequence` |
| 1 | `semio-s-plugin-shooting` |
| 1 | `semio-s-plugin-sourcing` |
| 1 | `semio-s-plugin-vcs` |
| 1 | `semio-s-plugin-writer` |

`semio-s-plugin-procedural` alone retains a direct `serde_json` dependency, for a boundary unrelated
to `app_commands!`; none of the other 28 invoking manifests declares either serde-family crate.

The reproducible inventory lives in the ticket's `📜️script.ts` and runs with:

```text
bun 📜️script.ts audit
```

## Mathematical and FEM State

Both acceptance crates are now fully past the workaround state described by
`📓️final-plugin-manifests.md`:

- `semio-s-plugin-mathematical`: no direct `serde` or `serde_json` manifest dependency and no
  production `serde::`, `Serialize`, `Deserialize`, or `#[serde(...)]` code reference. The ten types
  temporarily restored for the old macro contract are first-party-codec-only again.
- `semio-s-plugin-fem`: no direct `serde` or `serde_json` manifest dependency and no production
  serde derive/attribute/API reference. The 37 payload structs plus `FemDof` and
  `FemCombinationTerm` are first-party-codec-only again.

## Verification

The required first gate was run in a ticket-scoped isolated Cargo target because another agent's
workspace-wide check held the shared target lock:

```text
cargo check -p semio-framework-plugin
```

It reached framework dependency compilation, then failed in unrelated, concurrently modified
`semio-framework-geometry` code:

```text
error[E0425]: cannot find function `circle_path_elements` in this scope
  --> 🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs:399:9
```

The geometry file is `MM` with a large first-party geometry rewrite owned by another concurrent
wave; this ticket did not overwrite that in-flight work. Because the explicitly requested
framework-clean prerequisite is not satisfied, downstream mathematical/FEM checks have not yet
provided a real plugin compilation signal in this re-audit.
