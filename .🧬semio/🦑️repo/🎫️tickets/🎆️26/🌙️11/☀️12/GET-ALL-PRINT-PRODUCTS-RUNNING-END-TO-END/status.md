# Print Products End to End — Status

## Inventory

- Product: `🧰️framework/🛍️products/📓️print` (`@semio-tech/print`, 24 targets), plus `@semio-tech/print-rs`.
- Documents: 89 catalogued (8 templates, `viz-api`, 80 gallery), light and dark PDF each. Catalog: `🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json`.
- Course assignments built on the print classes: `@teaching/architecture-course:build-parametrization-en|de`, `build-typology-en|de`, `test-assignment`.

## Runs so far (2026-11-12)

No print code has executed yet. 22 of 22 `repo run` attempts were cancelled during graph planning.

| Blocker | Evidence | State |
|---|---|---|
| Plan cancelled by host memory floor while "Reading the project graph slice" (693 owners re-derived per run, ~2 GB each, 16 GB host) | every `*.log` in `🗑️generated/`; `repo doctor`: graph generation ~27 h stale | Sonnet agent fixing; report `graph-slice-starvation.md` |
| `Internal error: Bounded canonical UTF-8 auxiliary DATA required` in `build:source-admission` for `repo show projects` and `repo tools setup toolchains tectonic` | `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🟦️.ts:9`; reproduced three times | Sonnet agent fixing; report `show-projects-auxiliary-data.md` |
| `tectonic 0.16.9` not provisioned on this host | `repo tools list` | waits on the admission fix |

Running three target batches concurrently worsened the memory starvation; runs are serial from now on.
Running a product `📜️script.ts` directly is refused ("Original parent Script process invocation required"), so there is no bypass around the planner.

## Next

1. Admission error fixed, then `repo show projects` publishes a fresh graph generation.
2. Provision tectonic, then run the print targets one at a time: `build`, `build-viz`, tests, `print-rs`, course assignments.
3. Fix print-level failures per target.
