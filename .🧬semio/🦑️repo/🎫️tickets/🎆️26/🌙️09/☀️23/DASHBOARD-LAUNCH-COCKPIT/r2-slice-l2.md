# R2 Slice L-2: Documentation And Messages

Date 2026-10-08. Scope: dependents audit group C, minus `🎛️dashboard/README.md` (A-1) and `📜️script.ts` comments (L-1).
Coordinator amendment applied: no doc tells anyone to `bun run <alias>` except `dashboard` and `setup`; no `script:<name>` ids are used.

## Verification of ids (facts)

- Nx ids checked against `.nx/workspace-data/project-graph.json` with `r2-l2-has.ts` (all `ok`): `os-hub:dev`, `os-hub:dev-postgres`, `os-hub:build-dev-postgres`, `os-hub:dev-collaboration`, `workspace:{setup,start,dev,lint,format,test,build,publish}`, `@semio-tech/framework-os-dev:{dev-s-react-dev,serve-s-react-dev,dev-s-wgpu-dev}`, the quiz targets (`dev dev-site test typecheck test-e2e check publish docker-image-build docker-stack-bundle`), proctor targets (`dev rebuild health backup restore erase prune capacity check`), `@semio-tech/pets-react:dev`, `@semio-tech/assets:build`, energy `oracle-setup`/`oracle-epjson`.
- Playground `s`: variant `s`, react 6070, wgpu 6066, user slots react 6072/6073, wgpu 6067/6068 (generated `🚀️playgrounds.json`, `🌎️hub/🧩️compositions/🪐️space/…/Cargo.toml:23-27`). The old README said wgpu 6071; the catalog says 6066, so the README now says 6066.
- Declared tools/compounds read from the root `📋️project.json` `metadata.semio.dashboard`: `tool:workspace/{repo-mcp,os-mcp-stdio,os-mcp-http,mcp-inspector-os}`, `compound:workspace/{s-with-hub,s-with-os-mcp,s-users-with-hub}`, `compound:@teaching/architecture-quiz/quiz-with-proctor`. Parameters: proctor `erase` (`handle tag learner dry-run`), `prune` (`older-than` default 7d, `dry-run`), `restore` (`file`); pets `dev` (`port` default 6069, `menagerie`).
- `semio` binary was not built in this slice: command ids were validated against the graph and the registry source (`🎮️registry/🦀️.rs` `CommandId::parse`, `playground_target`), not by `semio commands`. UNVERIFIED at runtime: `--param`, `--detach --wait-ready` spellings (taken from fleet-plan §2.4).

## Files changed

- `README.md`: golden-path section rewritten as dashboard intro + CLI examples + table of ids; CI/CD paragraph names `workspace:<target>` ids (also dropped the false claim that root `generate` and `purge` exist; neither is a root script or target) and the lint/format/test hint.
- `.devcontainer/README.md` (lines 11, 59): forwarded ports are the ports of dashboard commands.
- Teaching: `🎓️teaching/README.md`, `🏛️architecture/README.md`, `❓️quiz/README.md` (Develop table, pets row, publish ids), `❓️quiz/🧱️stack/🟦️.ts` (docstring), `🐾️pets/README.md` (consumer table), `🛂️proctor/README.md` (intro + all annotations).
- `🧰️framework/🛍️products/🐾️pets/README.md`: removed `bun run test:pets…`/`dev:pets:stories` alias hints (coordinator amendment).
- Hub: `🌎️hub/README.md:213`, `🚀️local-bootstrap/🏃️execution/🟦️.ts:105`, `🤝️integration-harness/🟦️.ts:190` (message now names `os-hub:dev`).
- `✏️s/🧑‍💻dev/💡️services/🧪️tests/{🤖️live-agent-loop,💬️agent-reply}/🟦️.ts` (message names `playground:${PLUGIN}` / `semio run …`; "launch line" wording).
- os-dev: `♻️activation/🩺️readiness/🟦️.ts:24`, `🚀️local-hub/🏃️execution/🟦️.ts:9`, `🧪️tests/🧮️program-matrix/🟦️.ts:817` (message).
- Library: `🎮️playground/🔒️preferences/🟦️.ts:13`, `🟦️.ts:2002-2003` (comments).
- Energy: bestest `🦀️.rs:266`, epjson serializer test `🦀️.rs:182`, `🐍️.py:80`; puzzle `📜️script.ts:33,43`; procedural `README.md:12` (`playground:generation3d` with `renderer=react`, example `🥽️mesh-workbench`).
- Presentation and print READMEs; assets generator `🔣️icons/🏗️builder/📽️projection/🟦️.ts:326` and generated `🖼️assets/README.md:9` (edited in step; `check-generated` does not list the README as stale).
- Syntax check (`bun build --no-bundle`) of the 9 edited TS files: all ok. No test asserts a changed message (git grep on the strings: only historical ticket files).

## Remaining hits (`git grep` of the brief, outside `.🧬semio`, `.cursor/plans`, `AGENTS.md`)

| hits | owner |
| --- | --- |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json`, `.gitignore:591` | L-1 (deletion) |
| `📚️library/…/📋️project.json:55,56,570,571,599,600`, `🧪️test/📋️project.json:427,428`, vite globs `♻️mit-bestand/…/🟦️.ts:78`, `🏢️semio-tech/…/🟦️.ts:69`, `👁️watch-policy.json:11`, `📦️package-boundary-classification/🟦️.ts:802`, `🧱️command-composition-source/🟦️.ts:227-233`, `🧫️frozen-seal-ledger/🔣️.json:154-155` | L-1 / owner (seal) |
| `🧪️runtime-bootstrap/🟦️.ts:19,55,59,65` (forwardPorts law) | L-1. `.devcontainer/README.md` now says the ports equal those of dashboard commands; L-1's re-expressed law must match that wording |
| `🎛️dashboard/README.md`, `🌀️daemon/🧪️tests/…/🦀️.rs:44,73`, `🧪️tests/🌀️control-plane/🟦️.ts:56`, `🧫️fixtures/🚀️launch-configurations`, `🧬️schema/🔣️.json:15` | A-1 |
| scalar contract prose `🔢️scalar/🔣️.json:3595` and `🧬️schema/🔣️.json:3683` ("generated launch row") | contract owner (reseal rules UNVERIFIED); not in group C |
| `AGENTS.md:50` | owner |

## Notes for others

- `check-generated` of `@semio-tech/assets` fails for six git-ignored `🔣️icons/🤖️generated/…` files (membership mismatch, `💻️devices/🌧️cloud-download.svg`); pre-existing, not caused by this slice.
- The wgpu playground port is stated as 6066 (catalog), not the stale 6071.
- Local-only serve: message says `playground:s` with `S_LOCAL_ONLY=1`; the playground facts move to registry metadata in slice P-1 (fleet-plan §2.2.1 item 8).
