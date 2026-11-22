# gismap Rust (`@semio-tech/gis-gismap-rs`)

Status: NOT green. No gis-crate failure was found; every run was blocked by repo infrastructure that other agents are changing.

## Failures in order
1. `test`: graph slice worker exit 75, missing `wasm/browser/declaration/generated/index.d.ts` (gitignored, generated). Another agent generated it at 01:09; no change by me.
2. Graph slice: `fleet-status/readers` imported `ready/workers` that had not landed. It landed at 01:46 (other agent); no change by me.
3. Graph service: "Closed cache ledger required: area caps leave less free space than the soft reclaim threshold of bazel-workspaces". Cause: new `fleet-cache-index` area (1000 bp of products) raised the explicit products total to 9400 bp, over the ~8540 bp the 3 GiB soft reclaim allows.
   Fix: `semio-actions` budget 2000 -> 1000 bp, restoring the 8400 bp total.
   File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json`.
   The fleet agent may prefer a different split.
4. After 3, `test` ran 1h16m and `cargo` exited 101 (output truncated; not retrievable).
5. A `cargo test --workspace` run then stopped with: Source boundary exceeded, `semio-os-channel` 4009/4000 active lines. Another agent is mid-change there (large-archive ingress, `large-document-archive` test).
6. The last gis-only `cargo test` (-p gis-gismap-tests, -p gis-gismap) died in repo tooling: `hostLearnCeilingV1` / `hostUnderPressureV1` not exported from `host-capacity` (another agent mid-edit).

## Result
Not verified. `check`, `lint` and the verify targets were not run. Re-run `mcp__repo__test {project:"@semio-tech/gis-gismap-rs"}` once the host-capacity and os-channel edits settle.

## Update 04:30 (continuation)
- host-capacity exports landed (03:18); `semio-plugin` compiled again at ~03:50 (E0432 fixed by its owner).
- Real blocker now: gismap depends on stdio png -> pdf page editor crates. `pdf/.../✏️editor/🖼️page/🔆️image-preview/🦀️.rs` had E0597 (`inferred` outlived by `samples`); the pdf agent is rewriting `✍️authoring` (new crate + Cargo.toml still changing at 04:11) so workspace resolution (`cargo update --workspace`, step "cargo:selected-dependencies") fails on gis2d's dependency graph. Not a GIS defect.
- No gis crate test has executed yet, so no test counts or failing test names exist.
