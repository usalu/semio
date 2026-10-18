# Result nxneutral: Tool-Neutral Workspace Root, Nx Plugin Move, Wording

Static verification only (`node_modules` is a dangling symlink into the dependencies image; no `bun test`, no cargo builds were run).

## Neutral workspace-root rule (Class A)

Rule, identical to `findWorkspaceRoot` in `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts`: nearest ancestor (<= 32 levels) holding regular files `📋️project.json` and `package.json`, where `package.json` is valid JSON with a top-level `workspaces` array or object (last duplicate wins). A malformed `package.json` beside a build marker refuses (no walk upward). `nx.json` no longer counts.

The shared declaration is the existing corpus `🏠️workspace/🧬️schema/🔣️.json` + `🏠️workspace/🧪️tests/🧫️fixtures/🔣️.json` (12 vectors, already consumed by TS, Python, Go, C#, Bash, PowerShell, Rust test host). Rust implementations now conform to it:

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs` (crate `semio-framework-process-routing-workspace`): added std-only `find_workspace_root(&Path)` and `declares_workspaces(&str)` (own JSON structural validator). New test `🧪️tests/🦀️.rs` (`[[test]] process-routing-workspace-corpus`, dev-dep `serde_json`) runs every corpus vector plus a differential check against `serde_json` (valid and invalid documents).
- `🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs`: `install_default_schema_leaf_reader` now uses new pub `workspace_root_above` (region `WorkspaceRoot`, std-only copy of the same rule). A copy instead of a dependency because the neutrality test asserts the registry has `dependencies == []` and 203 per-artifact `Cargo.lock` files would need edits. A unit test in `🧪️tests/🔬️unit/🦀️.rs` runs the same corpus against it.
- `🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/` (`repo_root_in`, `repo_root!`, `repo_*_in`): now depends on the workspace crate (`Cargo.toml`), tests updated (`package.json` instead of `nx.json`, message "workspace root"). One lock edit needed and done: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.lock` (new package entry + `dependencies` for test-fixtures).
- `…/🔌️plugin/🖥️host/🎁️package-assets/🦀️.rs` `repository_root_above` (production): same rule via `serde_json` (host already depends on it); test fixture writes `📋️project.json` + `package.json{workspaces}`.
- Rust test sites now call `semio_framework_test_fixtures::repo_root!()` (unused `manifest_dir` imports removed): generation3d guest-memory, stdio csv snapshot unit, note sqlite+schema guests, puzzle 5d capsule-dream, `✏️s/🧑‍💻dev/🧹️fixture-sweep` fleet-example-sweep, hub bim/note committed-descriptor, hub draw guest, hub count-lease, dsl carrier-record-lists, OS plugin host + interpreter-runtime owned-instance-open, OS dsl fixture-sweep pilot-resolve.
- TS sites now call `findWorkspaceRoot` (`@semio-tech/process/routing`, or the relative import where one existed): ui device unit (vitest), flow retirement, cad birth, dev flow ownership, dev fixture-sweep ownership, hub document-growth (its private `AGENTS.md`+`nx.json` walk removed), renderer package-integration (3 sites), NodeGraph pinch-wheel, Paint2dHost ownership. `wgpu-browser-boot-cache-inputs` no longer copies a vestigial `nx.json`. pack-json and dsl `absence` fixtures now write `📋️project.json` + `package.json{workspaces:[]}` (they wrote `nx.json`, which the neutral rule has not accepted for a while).

## Nx-only contract moved under the adapter

`🔌️nx-plugin/` -> `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/` (plain `mv`). Relative specifiers inside the 6 moved files recomputed by script; 26 referrers re-pointed (executors.json, library `🟨️.mjs` private-task lists, `⚡️caching` bootstrap/native-orchestration/seal, `🏗️build` caller/tests/fixtures, bazel tests, process-profile test+policy list, snapshot test, plugin exports test, `schema-document-index.json`, retention schema, package `📜️script.ts`, `🧪️test/📋️project.json`). Stale input `…/🔌️nx-plugin/🟨️.mjs` (never existed) in `🧪️test/📋️project.json` now points to library `🟨️.mjs`. Test keys that mirror a moved file's own specifier updated in `🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🟦️.ts` (`../../../../🏗️build/…`). A script verified every `./`/`../` string path to an existing file for all moved and referring files, and all library `🟨️.mjs` and executors.json targets exist.

## Direct Nx CLI in tests (3 sites)

- `routing/🎛️command/🧪️tests/❄️cold`: no longer runs `nx.js run … --skip-nx-cache`. It reads the neutral `repo:script` target (`executor`, `options.cwd`, `options.command`) from the copied `project.json`, writes `📋️project.json`+`package.json{workspaces}` markers in the closed copy and runs that command directly (`SEMIO_WORKSPACE_ROOT`). Not run; needs `node_modules` for the Specific-free cold command.
- `⚡️caching/🧪️tests/📬️task-child` (Nx SDK, `.nx/installation` symlink, daemon modes) is inherently an Nx test: moved with fixtures/schema to `⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/`, imports re-depthed, `witness.json` path shortened, `inferenceTests["task-child"]` in `⚡️caching/📜️script.ts` updated. The target `test-task-child` is unchanged.

## Stale assertion

`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts`: expects `repo:script`, no `entry.nx`, and `scripts.test` = `bun --cwd "<relative root>" repo run <name>:test [--skip-cache]` (verified against the three package.json/project.json by script).

## Class C wording

- 26 plugin `📦️packages/🦀️rust/📜️script.ts`: "Nx-owned deliverable" -> "build-owned deliverable".
- Rewritten comments/errors (no test asserts them; checked by `git grep -F`): demonstrator (site, runtime, e2e, tests), `♻️mit-bestand/📋️bericht`, projektetage script, wgpu renderer scripts (native-entrypoint, browser-build, server, compiler wasm/native, projection, ProgramBridge), hub (commands, local-bootstrap incl. `Missing staged os-hub dev binary` and its regex, mcp support, Dockerfile, brep bench), OS dev (activation x4, benchmarks, vite plugin, distribution x3 incl. completion console lines), run bootstrap, plugin wasm-opt/playground, vite stub text, dotnet, native-build, other scripts (client cli, coordinator go, lint config), print artifacts/tectonic, native bootstrap `🐚️.sh` comment, storybook runner comment, energy oracle skip message, pets README project wording, `🧪️tests/🎚️config`.
- Generator header `@generated by \`bun repo run @semio-tech/ui-contract-rs:generate\`` in `🧬️contract/🧬️schema/🟦️projection/🦀️.rs` and the literal in `🧪️tests/🧱️owners/🟦️.ts` (nothing committed carries the old header).

## `.nx` ignores: kept

Something still writes `.nx`: `.nx/` exists in the checkout (`installation`, `workspace-data-terminal`), `⚡️caching/📦️artifacts/📇️registry/🟦️.ts:22` lists `.nx/workspace-data` as service state, `⚡️caching/🔣️policy.json:69` lists `.nx`. `.storybook/main.ts:143` and `tsconfig.json:33` ignores stay.

## Not done / open

- `AGENTS.md:7` ("`package.json` MUST call `nx`") contradicts the neutral goal (`package.json` scripts call `repo`); `PL/🎞️animate`, `🏛️architect`, `🔋️energy` `AGENTS.md` say "via nx". Not edited (forbidden).
- Nx-specific tests that still read or write `nx.json` or run Nx and stay in place (they test Nx behavior): `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts` (runs `nx run`), dashboard `🧪️tests/🧭️authority` ("never waits for Nx analytics consent"), dashboard registry fixture, and `⚡️caching/**`, `🕸️dependencies/**`, `🧪️tests/🔬️workspace-contract` etc. (Nx cache library fixtures). Recommend moving the analytics test and the production-browser-artifacts test under the adapter next.
- Not touched: `L/🔍️discovery/🟦️.ts:6660` comment (file no longer exists, being split by others); READMEs mentioning Nx (`.devcontainer`, hub, print, test, dashboard) because README fixture tests may hash them; `nx.json` as a known-config filename in `📐️model/…/🏷️kinds/🦀️.rs` and Go; `--skip-nx-cache` flag definition location (not found; remains in tests/fixtures); `bun repo tools setup nx`; `test-nx-*` target names; `@nx/devkit`/`@nx/js` stay (used by the adapter and the mandated-toolchain gate).
- Lock files: the 🧰️framework workspace `Cargo.lock` is already stale for members (no `test-fixtures` package entry, no workspace entry), so cargo will rewrite it on an unlocked run.

## Verification (exact)

- `rustc --edition=2021 --crate-type=rlib` on the workspace crate and on the registry `🦀️.rs`: both compile. A std-only probe over all 12 corpus vectors and 10 `declares_workspaces` cases: 0 mismatches for both implementations.
- `rustfmt --edition 2021 --check --config skip_children=true` on 23 edited Rust files: no parse errors.
- `bun build --no-bundle` on 17 edited TS files: all parse.

## Rust crates to compile and test (by the build owner)

Via `bun repo cargo --store=shared --configuration=prototype -- …`:

1. `semio-framework-process-routing-workspace` (lib + `--test process-routing-workspace-corpus`)
2. `semio-framework-schema-registry` (lib + unit test `workspace_root_above_resolves_every_vector_of_the_neutral_workspace_corpus`)
3. `semio-framework-test-fixtures` (lib + tests)
4. `semio-framework-plugin-host` (`🎁️package-assets` lib + tests), then `semio-framework-os-run`, `semio-framework-os-store-sync`, renderer `kernel-pool` (consumers of `repository_root_above`)
5. Test targets: the crates under `🗣️dsl` (carrier-record-lists), OS `🗣️dsl/🧹️fixture-sweep` (pilot-resolve), plugin host and interpreter-runtime (`owned-instance-open`), `✏️s/🧑‍💻dev/🧹️fixture-sweep`, hub bim/note/draw compositions and `🪶️count-lease`, stdio csv snapshot, note snapshot guests, puzzle 5d examples, generation3d (if `guest-memory` is registered), `ui-contract` (generator header).
6. TS: `bun test` for the 17 edited TS files once `node_modules` exists (`❄️cold`, `🧱️neutrality`, `📬️task-child`, `absence` x2 first).
