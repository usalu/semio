# result-ws-resolve (executor ws-resolve, 2026-10-19)

Task: `cargo metadata` (the launcher runs `cargo fetch --locked` first) must succeed for every tracked `[workspace]` manifest (291, non-ticket). Driver: `ws-resolve-tools/one.sh` (one manifest; retries launcher flakes; `UPDATE=1` runs `update --workspace` once when the lock is stale or missing). Raw per-workspace logs and `results.tsv` live in `🗑️generated/wsr/`. Required env: `SEMIO_SCRIPT_POLICY` with `maximumElapsedMilliseconds` (without it the launcher aborts with "Finite original maintenance authority required").

## The three blockers

| workspace | before | after | fix |
|---|---|---|---|
| `🧰️framework/Cargo.toml` | `3d-brep-construction` has no feature `testkit` | rc 0 | peers added the feature meanwhile; also repaired a codemod-mangled comment in `…/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (a dependency line had been pasted into a comment, breaking TOML at L135) |
| `Cargo.toml` (root) | `ui-wgpu-layout` manifest: `semio-framework-trace` without path; then manifest parse errors; then a dependency cycle | rc 0 | layout-stack now `workspace = true` (peer); removed optional dep `…ui-engine-tests` (+ `dep:` feature entry) from `…/🎪️host-winit/📦️packages/🦀️rust/Cargo.toml` (cycle host-winit -> ui-engine-tests -> host-winit; no source used it); `update --workspace` |
| `🌎️hub/Cargo.toml` | `os-mcp-capabilities` `testkit` feature referenced undeclared `dep:semio-framework-ui` | rc 0 | peer fixed the manifest (now `ui-declaration`); lock stale -> `update --workspace` |

## Sweep over the remaining workspaces (288 tracked `[workspace]` manifests besides the three)

State at 15:15: framework, root and hub resolve (rc 0). 28 further manifests resolve (rc 0, mostly `✏️s/🔌️plugins/*/🏭️bridge`, `✒️writer`, `➗️mathematical`, `✏️s/Cargo.toml`). Not yet resolved:

| workspace | state | note |
|---|---|---|
| `…/🎛️dashboard/🧪️tests/🧭️journeys/…/Cargo.toml`, `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/…/Cargo.toml` | no `Cargo.lock` ("cannot create the lock file") | driver now runs `update --workspace` for this case (pass 2 queue) |
| 7 manifests (`🧑‍💻dev/🧹️fixture-sweep`, `🧑‍💻dev/🧩️composition`, `🧑‍💻dev/🔱️trinity/🪆️graph`, `🀄️wfc` `🧊️3d` and `🧱️grid3d` + their bridges) | rc 142, launcher alarm (550 s) | machine load (about 70) plus `dependencies sync/lock` gating; re-queued |
| about 255 others | not run yet | queue `🗑️generated/wsr/todo.txt` |

Why the sweep is incomplete: one launcher call costs 1.5-5 min under fleet load and the coordinator's rule (one `bun repo cargo`, wait during `dependencies lock/sync`) serializes it; a `sync --all` held the gate for more than 40 min. The first pass ran 12 in parallel and was stopped on the coordinator's request.

Resume (serial, gated, updates stale or missing locks): `cd /Users/ueli/Documents/semio && tr '\n' '\0' < "<C>/🗑️generated/wsr/todo.txt" | UPDATE=1 xargs -0 -n1 -P1 "<C>/ws-resolve-tools/one.sh"`; results append to `🗑️generated/wsr/results.tsv` (columns rc, manifest, first error). The driver is still running detached when this file was written.

## Changes (file list)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (comment repaired)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎪️host-winit/📦️packages/🦀️rust/Cargo.toml` (cycle removed)
- `Cargo.lock`, `🌎️hub/Cargo.lock` and the locks of the resolved standalone workspaces (`update --workspace`)
- `ws-resolve-tools/one.sh` (driver)

## Verification

`one.sh <manifest>` = launcher `metadata --format-version 1 --manifest-path <ws>` (includes `cargo fetch --locked`): rc 0 for framework, root, hub and the 28 listed in `🗑️generated/wsr/results.tsv`. No build/check/test was run.
