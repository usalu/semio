# result-docfix-f (executor docfix-f, wave 1025f): fixes for audit-run-instructions-1025f

Status (newest first):
- 21:00 (10/25) DONE except the official law run. Per coordinator order I killed my chain (children first, lstart verified: 71752 19:47:39, 71405 19:47:19, 71399 19:47:19, 49422 19:30:14, 39760 19:22:13, 39758 19:22:13; all gone, `repository-graph-facts` shard 7 released). The official `repo:test-runtime-bootstrap` was therefore NOT completed (it spent 1 h 28 min in capacity/graph-plan waits and never reached the law) and is not relaunched this wave. FOR whoever has graph capacity: run `bun repo run repo:test-runtime-bootstrap` once to confirm the law on the new fixture. The law cannot be run graph-free: `testContainerRuntimeBootstrap` needs an issued script invocation and the generated playground catalog (a direct `repo ./📜️script.ts test runtime-bootstrap` fails with "Original parent Script process invocation required"), and the law file is not a `bun test` file.

## Fixed (audit id -> change)
- M1 devcontainer ports: `.devcontainer/devcontainer.json` now forwards 6050 (Projektetage deck), 6061 (Course site), 6063 (Course site beside stack), 8791 (Course proctor), 8793 (Course proctor beside stack) in `forwardPorts` + `portsAttributes`. The law's source of truth `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json` got three `forwardedCommands` (course `dev` 6061, proctor `dev` 8791, projektetage `dev` 6050; the law reads each ready port from `metadata.semio.dashboard.ready` of the named manifest target) and two `forwardedUndeclaredPorts` (6063, 8793: the `stack=beside` form of course `dev` is an environment override, not a declared ready port). `.devcontainer/README.md` and root `README.md` port lists updated.
- M2 print authored script: `🧰️framework/🛍️products/📓️print/README.md` now names `…/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts`, form field "Build owner project" (was "Nx project"), arguments `build <document id>` with the real ids (`viz-api`, not `viz api`); `watch` documented as argument-free; `generate viz` sentence now the real target `repo run @semio-tech/print:generate-viz`.
- M3 dangling `--`: removed in `🎓️teaching/README.md` (EN + DE) and the course README (EN + DE).
- M4 prerequisite column: course README rows `dev`, `check`, `test-e2e`, `docker-image-build` (combined row), `deploy-check` now list `@teaching/proctor:build` (EN + DE); `--proctor <executable>` sentence corrected (it only picks the executable the gate runs; the build dependency still runs).
- M5 bericht: new `♻️mit-bestand/📋️bericht/README.md` (EN, then DE).
- M6 projektetage: new `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/README.md` (EN, then DE: port 6050, dev/build/test/canonical-architecture, output dir, devcontainer). `package.json` `repository.directory` and `semio.app.packageRoot` corrected to the real emoji paths (no consumer of `packageRoot` found by `rg`).
- M7 print-rs: new `🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/README.md` (EN, then DE; all 8 targets); print README Commands lists the print-rs targets; wording "through Nx" -> "through the build graph".
- M8 dead config link: `🧰️framework/🛍️products/🦑️repo/README.md:118` -> `[.🧬semio/🦑️repo/📋️config.toml](../../../.🧬semio/🦑️repo/📋️config.toml)` (target exists).
- M9: root `README.md:145` now says `semio` is the native executable behind `repo dashboard`, installed by the first start, path printed by the dashboard `build` target.
- M10: the stale "2026-10-19 proctor did not compile" note replaced (EN + DE) by the 2026-10-25 course-f verdict (typecheck/test/check/build green; dev/test-e2e/image path not yet recorded green). Re-check after e2e-f reports.
- M11: the two dead `.github/agents/compose.agent.md` links removed from root `README.md` (the `CLAUDE.md` link is expected on a fresh checkout, untouched).
- M12: `repo run @teaching/architecture-course:test-e2e --configuration=release -- --topology rehearsal` added (EN + DE).
- DE versions added (heading `## 🇩🇪️ Deutsch`, EN first): print README, presentation README, dashboard module README (run model, CLI verbs, tests), `🎓️teaching/🏛️architecture/README.md`.
- Not touched: M13 (info), M14 (dash-f's smoke port), the long EN-only deploy chapters of the course/proctor READMEs and the root README DE chapter (out of this lane's list).

## Changed files
- `.devcontainer/devcontainer.json`, `.devcontainer/README.md`, `README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json`
- `🧰️framework/🛍️products/📓️print/README.md`; new `🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/README.md`
- `🧰️framework/🛍️products/🎤️presentation/README.md`
- `🧰️framework/🛍️products/🦑️repo/README.md`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md`
- `🎓️teaching/README.md`, `🎓️teaching/🏛️architecture/README.md`, `🎓️teaching/🏛️architecture/📚️course/README.md`
- new `♻️mit-bestand/📋️bericht/README.md`, new `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/README.md`, `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/package.json`
- tools: `C/docfix-f-tools/links.py` (relative-link check), `C/docfix-f-tools/targets.py` (every `repo run <project>:<target>` in the docs resolves against all `📋️project.json`)

## Verifications
- `targets.py` over all edited/new docs: 0 missing targets (1488 projects scanned).
- `links.py`: 0 dead relative links except the expected `CLAUDE.md` in root `README.md`.
- Python replica of the port law (fixture ports read from the manifests vs `forwardPorts`/`portsAttributes`, labels equal): consistent.
- Official law `repo:test-runtime-bootstrap`: NOT RUN to a verdict (killed, see status line). Replacement unit-level proof: the Python replica above reproduces the law's port assertions (each `forwardedCommands` port read from the manifest `metadata.semio.dashboard.ready`, plus undeclared ports; distinct; `portsAttributes` labels equal the fixture labels). The playground and MCP Inspector ports (6066-6073, 6274) are unchanged, so the law's `forwardPorts == expectedPorts` equality holds if it held before.
- No docs/markdown law target exists in the verify lanes of `result-verify-d.md` (checked `📋️project.json` target names); path-length law unaffected (new files < 256 chars).
