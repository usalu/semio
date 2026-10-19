# result-lockref (executor lockref, 2026-10-19)

Driver: `lockref-tools/all.sh` (serial over `🗑️generated/lockref/order.txt`, 291 tracked `[workspace]` manifests, priority: root, framework, hub, teaching, wfc/generation3d/fem/process/cad/energy/stdio, then the rest) calling `lockref-tools/one.sh` (gated while `bun repo dependencies lock|sync` runs; `metadata --locked`; on a lock refusal `update --workspace` then `metadata --locked` again; flake retries). Rows: `🗑️generated/lockref/results.tsv` = manifest, before rc, action, after rc, error.

## Framework membership (coordinator request)
The 11 crates (plus 3 workspace-only stubs) from `audit-gates-1019.md` section 6 are NOT missing from `🧰️framework/Cargo.toml`: each of the 11 declares an explicit `package.workspace = "<up to repo root>"` (root workspace owns them), and the framework workspace is deliberately a subset (405 members vs 560 framework paths in root). The 3 stubs are standalone `[workspace]` roots. No manifest edit made.

## Gate
A coordinator `dependencies sync --all` held the gate for most of the session; driver waits (rule).

## Status at hand-off (INCOMPLETE; gated by fleet)
| workspace | before | after |
|---|---|---|
| `Cargo.toml` (root) | `metadata --locked` rc 0 | rc 0 (no change) |
| `🧰️framework/Cargo.toml` | rc 1: launcher `SyntaxError: Export named createNativeWindowsSystemGrantV1` (peer mid-edit; `bun build` of the launcher passes later) | not re-verified |
| `🌎️hub/Cargo.toml` | `update --workspace` refused: `semio-framework-job` not found at `…/🧵️job/📬️outcome/📦️builder` | manifest fixed (below); `--locked` + update not re-run: blocked |
| other 288 workspaces | not run | `🗑️generated/lockref/order.txt` queue |

Why: `bun repo dependencies sync --all` / `lock --all` ran back-to-back for the whole session (gate), and `update --workspace` additionally waited 30+ min for the exclusive `cargo:registry` lock behind peers' long `cargo check`s. Driver `lockref-tools/all.sh` (resumable: skips manifests already in `results.tsv`) is detached; restart it when the gate is free:
`cd /Users/ueli/Documents/semio && nohup "<C>/lockref-tools/all.sh" >/dev/null 2>&1 & disown`

## Manifest fixes
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml`: removed stale `package = "semio-framework-job"` on `[workspace.dependencies.semio-framework-job-payload]` (crate is named `semio-framework-job-payload` now; this broke every workspace that includes the cad editor, e.g. hub).
- Trinity: `✏️s/🔌️plugins/🔱️trinity/Cargo.toml` had `members = [ ]` (no crate left; jack/rewriting/bridge are their own excluded workspaces). Removed the manifest; dropped its rows from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/📋️constraints/🔣️.json` and `…/🧪️tests/📥️consumption/🔣️.json` and the counts in `…/🧪️tests/📥️consumption/🟦️.ts` (239 -> 238 owners, native29/wasm38 136 -> 135). Root/`✏️s` `exclude` entries for the directory left (still shield the nested workspaces). Test: `bun test` of that file 21 pass / 3 fail; the 3 fail only with "Original parent Script process invocation required" (need the repo launcher envelope), not related.
- Framework nesting: no edit (see above).
