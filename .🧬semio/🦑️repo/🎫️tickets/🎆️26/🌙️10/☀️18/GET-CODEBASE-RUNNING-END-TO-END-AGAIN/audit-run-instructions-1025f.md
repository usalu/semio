# Audit: how a human runs the three products, and whether the docs agree

Wave 1025f, lane docs-f (read-only). Date 2026-10-25. Repo `/Users/ueli/Documents/semio`.
Nothing was built, tested or run. Target names come from the `📋️project.json` and `package.json` files read directly. Documentation commands were matched against those target names by script, and every `repo run <project>:<target>` in the tracked non-ticket markdown was checked. Paths and ports were read from the configs.

## 0. Result in brief

- Every `repo run <project>:<target>` in tracked non-ticket markdown resolves to a real target, with one exception outside this scope: `🌎️hub/README.md:590` (`@semio-tech/framework-os-dev:build-s-react-release`).
- The product docs contain no raw `cargo`, `nx`, `bazel`, `vite` or `bun install` run instruction. Every mention of those tools is a warning, or an explanation of what `repo` does.
- Real mismatches: the print "authored script" instructions name the wrong script and arguments (M2); the devcontainer port claim is false for the course and projektetage ports (M1); the repo config link is dead (M8); the course prerequisite column is wrong (M4); a dangling `--` in the e2e example (M3).
- Gaps: `♻️mit-bestand` bericht has no documentation at all, projektetage has no README, and `print-rs` is documented nowhere (M5, M6, M7).
- Most product docs are English only (section 5).

## 1. Real targets per product

### 1.1 Dashboard: `@semio-tech/repo-dashboard-rs` (binary `semio`)

Manifest: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json`

| Target | Command (all `repo ./📜️script.ts <x>`) | Notes |
|---|---|---|
| `build` | `build` | `install` depends on it |
| `install` | `install` | dependsOn `build` |
| `run` | `run` | root `package.json` script `dashboard` calls `repo run @semio-tech/repo-dashboard-rs:run --output=stream` |
| `daemon`, `preferences`, `compile` | same verb | |
| `test` | `test` | levels: `test -- quick`, `test-long`, `test-exhaustive`, `test-coverage` |
| `test-unit`, `test-smoke`, `test-journeys`, `test-windows`, `test-load`, `test-cold-start` | same verb | `test-smoke` and `test-journeys` are the lifecycle and launcher checks |
| `test-authority`, `test-execution`, `test-process-deadline`, `test-dispatch`, `test-language`, `test-source-freshness`, `test-ci-source` | same verb | |

The launcher project `@semio-tech/repo-dashboard-launcher-rs` has no targets. Root `package.json` also has `dashboard:install`, which runs `build --exclude-dependencies --skip-cache`. No doc names it.

### 1.2 Architecture course: `@teaching/architecture-course`

Manifest: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`. `package.json` scripts `build`, `test`, `test:e2e`, `typecheck` only forward to these targets.

| Target | What it runs | dependsOn (from manifest) | Cache |
|---|---|---|---|
| `dev` | proctor and site in one terminal; ready on 6061 | assets:build, ui-styling-tokens:generate, **@teaching/proctor:build** | never (continuous) |
| `dev-site` | site only, Vite on 6061 | assets, tokens | never |
| `build` | site for `PROCTOR_URL` | assets, tokens | yes |
| `publish` | stages `dist/release/pages/course` (release) | assets, tokens, build | never |
| `test` | levels `-- quick` (default), `long`, `exhaustive` | generated bindings, assets | yes |
| `typecheck` | | generated bindings, assets | yes |
| `check` | | **@teaching/proctor:build** | yes |
| `test-e2e` | end-to-end gate, args `--topology both|dev|rehearsal` | assets, tokens, **@teaching/proctor:build** | never |
| `preflight`, `ci-browser-prepare` | host check; Chromium system packages | none | never |
| `docker-image-build` | proctor image (Linux x64 only) | **@teaching/proctor:build** | never |
| `docker-image-check`, `docker-image-publish` | image check and publish | none | never |
| `docker-stack-check` | `tag`, `http-port` (18080), `https-port` (18443), `keep` | none | never |
| `docker-stack-bundle` | stack into `dist/proctor` | none | never |
| `deploy-check` | readiness gate, release | assets, tokens, typecheck, publish, **@teaching/proctor:build** | never |

The proctor `@teaching/proctor` has `dev`, `check`, `rebuild`, `health`, `backup`, `restore`, `erase`, `prune`, `test`, `build` and `capacity`. Every one of those names is referenced by the proctor README, and the check found no missing name.

### 1.3 Print

| Project | Manifest | Targets |
|---|---|---|
| `@semio-tech/print` | `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json` | `fonts`, `generate`, `generate-viz`, `build` (dependsOn fonts, deps-tectonic, generate), `watch` (dependsOn build), `build-viz`, `watch-viz`, `test` (dependsOn build), `test-viz`, `test-viz-full`, `test-viz-fixtures`, `test-native-grammar`, `test-macro-staging`, `test-measurement`, `test-toolchain`, `check`, `preview-generated`, `generate-font-metrics`, `deps-tectonic`, `deps-tex` |
| `@semio-tech/print-rs` | `🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/📋️project.json` | `build`, `test`, `test-snapshot-sqlite` (plus `-native`, `-source`), `produce-logical-owner`, `test-logical-owner`, `test-native-emission` |
| `@semio-tech/presentation` | `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/📋️project.json` | `test` (levels `fundamental|quick|long|exhaustive`) |
| `@semio-tech/presentation-react` | `…/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json` | `test` |
| `@semio-tech/mit-bestand-bericht` | `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json` | `build` (all three reports), `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht`, `watch`, `watch-zwischenbericht`, `watch-forschungsbericht`, `watch-kompaktbericht`, `generate-actor-network`, `akteursnetz-validate`, `check-generated`, `preview-generated`, `test` |
| `@semio-tech/mit-bestand-praesentation-projektetage` | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json` | `dev` (port 6050, continuous), `build`, `test`, `canonical-architecture` |

The bericht package has no `scripts` in its `package.json`. The projektetage package has `build` and `test` scripts. Output of `build-zwischenbericht`: `{workspaceRoot}/.cache/b/p/m/print/{SEMIO_BUILD_MODE}/documents/report/zwischenbericht`.

## 2. Human-facing passages (file:line)

### 2.1 Dashboard

- `README.md:125` "Build the products from this checkout with `repo dashboard run workspace:setup`."
- `README.md:141` `repo dashboard run workspace:setup` (once, after cloning).
- `README.md:145` "`semio …` is shorthand for `repo dashboard …`". No doc says where `semio` lives or how to put it on PATH.
- `README.md:643-654` "The four golden paths": `repo dashboard commands dev hub`, `repo dashboard run os-hub:dev --detach --wait-ready`, `repo dashboard tasks`.
- `README.md:757` "Humans choose build, test and run operations in `repo dashboard`."
- `🧰️…/🦑️repo/README.md:104` "Start `repo dashboard` at the monorepo root."
- `🧰️…/🎛️dashboard/README.md:18` "Start with `repo dashboard`; it builds and installs the native executable `semio` when needed."
- `🧰️…/🎛️dashboard/README.md:200-215` CLI verbs: `commands`, `run`, `tasks`, `logs`, `stop|restart|kill`, `open`, `daemon`, `preferences`, `catalog`, `command-tree`. The dispatch in the dashboard source confirms `commands`, `tasks`, `stop`, `kill` and `restart`; the others were not re-checked.
- `🧰️…/🎛️dashboard/README.md:397-430` Tests: `repo run @semio-tech/repo-dashboard-rs:test -- quick`, `:test-execution`, `:test-journeys`, `:test-smoke`.
- `.devcontainer/README.md:44` "Start `repo dashboard` inside the container."

### 2.2 Architecture course

- `🎓️teaching/README.md:71` "Everything runs through the tool-neutral build entry".
- `🎓️teaching/README.md:73-77` table: `repo run @teaching/architecture-course:dev`, `…:test`, `…:typecheck`, `…:test-e2e -- --topology dev --` (dangling `--`, see M3), `repo run @teaching/proctor:test`.
- `🎓️teaching/🏛️architecture/README.md:11` "in development vite on port 6061 proxies the gateway routes to the proctor on 8791".
- `🎓️teaching/🏛️architecture/📚️course/README.md:48-52` Develop, prerequisites, dashboard search.
- `…/📚️course/README.md:58-65` prerequisites table: `repo tools setup all`, `repo dependencies sync --all`, `repo preflight`, `repo cargo --store=shared`, Docker, release mode.
- `…/📚️course/README.md:73-83` target table (dependency column, see M4).
- `…/📚️course/README.md:90-99` quick commands: `repo run @teaching/architecture-course:dev` (proctor 8791, site 6061); `test-e2e -- --topology dev --` (M3); `test-e2e --configuration=release` ("both topologies; the rehearsal needs release"); `deploy-check --configuration=release`.
- `…/📚️course/README.md:113-124` "Not yet verified" note (M10); beside stack: `TEACHING_ARCHITECTURE_COURSE_PORT=6063 PROCTOR_PORT=8793 … repo --tool=bazel run @teaching/architecture-course:dev`.
- `…/📚️course/README.md:128-150` what `dev` does in one terminal; `.🧬semio/🎓️teaching/proctor-dev/` is disposable development data.
- `…/📚️course/README.md:161-190` end-to-end gate: `dev` and `rehearsal` topologies, `--topology rehearsal`, `--grep`.
- `…/📚️course/README.md:272-290` "Proof of concept by hand": publish → `dist/release/pages/course`, `docker-image-build`, `docker-stack-bundle` → `dist/proctor`. The verb is `docker-stack-bundle`, and the output directory matches the manifest.
- `…/📚️course/README.md:352-726` Deploy, EN only: `docker compose up --detach --wait` and the operator commands.
- `🎓️teaching/🛂️proctor/README.md:285-302` dashboard and CLI commands for dev proctor: `repo run @teaching/proctor:dev` (8791), `:check`, `:rebuild`, `:health`, `:backup`, `:restore`, `:erase`, `:prune`.
- `🎓️teaching/🛂️proctor/README.md:552-700` Operate: `docker compose stop proctor`, `docker compose run --rm --no-deps proctor prune …` (EN only).
- `🧰️…/📚️course/README.md:40-47` framework course product (`@semio-tech/course`, `-rs`, `-react`): `repo run @semio-tech/course:test -- quick`. This is not the architecture course.

### 2.3 Print, presentation, bericht, projektetage

- `🧰️…/📓️print/README.md:21-30` "Start `repo dashboard`" then `repo run @semio-tech/print:generate`, `:generate-viz`, `:fonts`, `:build`, `:watch`, `:test -- quick`.
- `🧰️…/📓️print/README.md:32-37` "For one template, choose **Run an authored script**" (script path and arguments: M2).
- `🧰️…/📓️print/README.md:40-43` "`test fundamental` fails if one of them is stale, so run `generate viz`". Both verbs exist.
- `🧰️…/📓️print/README.md:89` "Build it with the **Run an authored script** form and `build viz api` arguments" (M2).
- `🧰️…/🎤️presentation/README.md:22-30` "Start `repo dashboard`" then `repo run @semio-tech/presentation:test -- quick`, `…presentation-react:test -- quick`, `…mit-bestand-praesentation-projektetage:dev --detach --wait-ready`, `…:build`.
- `🧰️…/🦑️repo/README.md:131-155` tool-neutral CLI reference: `repo run <project>:<target>`, `repo run-many`, `repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib`.
- `🧰️…/🦑️repo/README.md:118` repo config link (M8).
- `♻️mit-bestand`: no README for bericht or projektetage. The only mention of bericht run commands is in `package.json` targets, which no doc lists (M5, M6).

## 3. Mismatches and proposed corrections

Severity: High = a human command or claim is wrong today. Medium = misleading, or a gap that blocks a product. Low = wording, stale note or dead link.

### M1 (High): devcontainer port claim is false for course and projektetage

- Doc: `.devcontainer/README.md:11` "It forwards exactly the ports the dashboard commands start"; `README.md:641` "Port forwarding for dashboard services (8787, 6070, 6072/6073, 6066, 6067/6068, 6010, 6274/6277)".
- Config: `.devcontainer/devcontainer.json:65-76` `forwardPorts` has no 6061, 6063, 8791, 8793 or 6050. The law in `🧰️…/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts:145` asserts equality with the fixture `…/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json` field `forwardedCommands`, which lists only os-hub, s, Storybook and MCP Inspector.
- Proposed: add `@teaching/architecture-course:dev` (6061, "Course site"), `@teaching/proctor:dev` (8791, "Course proctor") and `@semio-tech/mit-bestand-praesentation-projektetage:dev` (6050) to `forwardedCommands`, `forwardPorts` and `portsAttributes`. If they are meant to stay unforwarded, change both doc sentences to name the exception. The law fails if only one side changes.

### M2 (High): print "authored script" instructions use the wrong script and wrong arguments

- Doc: `🧰️…/📓️print/README.md:33-36`: script path `…/📓️print/📦️packages/🟦️typescript/📜️script.ts`, then "Put `build viz api` or `watch <template>` in **Arguments**". Also `README.md:89`: "`build viz api` arguments".
- Code: the print product router `…/📦️packages/🟦️typescript/📜️script.ts` registers only `fonts`, `generate`, `preview-generated`, `check`, `test`. `build`, `complete` and `watch` are registered by `🧰️…/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts`.
- Argument rules in that script: `build` needs exactly one document id, so `viz api` (two words) fails. The catalog id is `viz-api` (`…/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json:473`). `watch` throws when it gets any argument.
- Proposed: Script path `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts`; Arguments `build viz-api` (or `build report`, `build paper`, `build phd`, `build flyer`, `build zwischenbericht`, `build forschungsbericht`, `build kompaktbericht`). For watching, use `repo run @semio-tech/print:watch` with no arguments.

### M3 (Medium): dangling `--` in the test-e2e example

- Doc: `🎓️teaching/🏛️architecture/📚️course/README.md:96` `repo run @teaching/architecture-course:test-e2e -- --topology dev --     # the dev topology on fresh ports` (same at `:779`, DE). `🎓️teaching/README.md:77` has the same command.
- The gate usage is `test-e2e [--topology …] [-- <Playwright arguments>]` (`🎭️e2e/🟦️.ts:18`). The trailing `--` has no purpose.
- Proposed: `repo run @teaching/architecture-course:test-e2e -- --topology dev`.

### M4 (Medium): course prerequisite column omits @teaching/proctor:build

- Doc: `…/📚️course/README.md:73` (`dev`) and `:80` (`test-e2e`) say "assets, tokens"; `:83` (`deploy-check`) lists "assets, tokens, `typecheck`, `publish`".
- Manifest: `dev`, `test-e2e`, `check`, `docker-image-build` and `deploy-check` all list `@teaching/proctor:build` in dependsOn. `dev-site` does not.
- Proposed: add `@teaching/proctor:build` to the `dev`, `test-e2e` and `deploy-check` rows. Line 61 says `test-e2e` needs no proctor build "unless `--proctor <executable>` names one". The manifest dependsOn is static, so whether `--proctor` skips the build is not verified here. Check before keeping that sentence.

### M5 (Medium): bericht has no documentation

- Doc: none. There is no `README.md` in `♻️mit-bestand/📋️bericht/` (the markdown there is annex data only).
- Targets with no doc: `build`, `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht`, `watch-*`, `test`, `generate-actor-network`, `check-generated`, `akteursnetz-validate`.
- Proposed: add `♻️mit-bestand/📋️bericht/README.md` in EN and DE, with:
  - `repo run @semio-tech/mit-bestand-bericht:build` (all three reports)
  - `repo run @semio-tech/mit-bestand-bericht:watch-forschungsbericht`
  - `repo run @semio-tech/mit-bestand-bericht:test`
  - the output path from `build-zwischenbericht` (section 1.3)

### M6 (Medium): projektetage has no README, and its stale paths

- Doc: only the presentation README lists `dev` and `build` (`🧰️…/🎤️presentation/README.md:29-30`). `test`, `canonical-architecture` and the port 6050 are documented nowhere.
- Stale paths: `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/package.json:28` `"directory": "mit-bestand/präsentation/33.projektetage"` and `:41` `"packageRoot": "mit-bestand/präsentation/33.projektetage"`. The real directory is `♻️mit-bestand/🎤️präsentation/📅️33.projektetage`.
- Proposed: correct `repository.directory`. Check the consumers of `semio.app.packageRoot` before changing it. Add a README (EN and DE) that gives the port 6050 (`PRAESENTATION_PROJEKTETAGE_PORT`), `:dev --detach --wait-ready`, `:build`, `:test` and `:canonical-architecture`.

### M7 (Medium): print-rs is documented nowhere; one wording issue

- Doc: no markdown mentions `@semio-tech/print-rs`.
- Proposed: add to `🧰️…/📓️print/README.md` Commands: `repo run @semio-tech/print-rs:test` and `repo run @semio-tech/print-rs:build`, plus `:test-snapshot-sqlite` and `:test-logical-owner` for the snapshot and owner checks.
- Wording: `🧰️…/📓️print/README.md:22` "Each owner target routes to the product's authored scripts through Nx." Human docs should stay tool-neutral. The presentation README (`:23-24`) says "through the build graph". Use that.

### M8 (Medium): dead link and wrong filename for repo configuration

- Doc: `🧰️…/🦑️repo/README.md:118` "Repo-wide settings live in [`.🧬semio/🦑️repo/config.toml`](../.🧬semio/🦑️repo/config.toml)".
- Reality: the file is `.🧬semio/🦑️repo/📋️config.toml`. The relative link has one `..` too few. The file is three levels up from `🧰️…/🦑️repo/`.
- Proposed: `[`.🧬semio/🦑️repo/📋️config.toml`](../../../.🧬semio/🦑️repo/📋️config.toml)`.

### M9 (Low): `semio` shorthand without a location or PATH instruction

- Doc: `README.md:145` "`semio …` is shorthand for `repo dashboard …`". `🧰️…/🎛️dashboard/README.md:18` says the executable is built and installed, but not where.
- Proposed: add one sentence with the install location and how to put it on PATH, or drop the shorthand from the examples. The dashboard lane (dash-f) knows the install path.

### M10 (Low): stale verification note with a date

- Doc: `…/📚️course/README.md:113` "On 2026-10-19 the Rust proctor did not compile on the working tree" (and the German copy at `:813`).
- Proposed: replace it after the wave 1025f verdicts are in, with the result and its date. Do not keep a 2026-10-19 claim after the proctor builds.

### M11 (Low): broken links

- `README.md:871` and `:877` link `.github/agents/compose.agent.md`, which does not exist.
- `README.md:843` links `CLAUDE.md`. The file is created by `repo dashboard run workspace:setup -- git` (`README.md:853`), so this one is expected on a fresh checkout.
- Proposed: remove the two agent links or point them at an existing file.

### M12 (Low): rehearsal-only command is not written out

- Doc: `…/📚️course/README.md:97` gives `test-e2e --configuration=release` for both topologies. No command line runs the rehearsal alone. The dashboard form does (`:176`).
- The gate accepts `--topology rehearsal` (`🎭️e2e/🟦️.ts:18`).
- Proposed: add `repo run @teaching/architecture-course:test-e2e --configuration=release -- --topology rehearsal` after line 97.

### M13 (Info): raw tool names in human docs

- `🧰️…/🦑️repo/README.md:228` names `nx reset --onlyCache` inside the cache table. It describes what `repo cache clear` does and is not a run instruction. Keep it or reword it.
- No product doc tells a human to run `cargo`, `nx`, `bazel`, `vite` or `bun install`. Every hit is a negation ("never a plain `bun install`") or the `repo cargo` form.

### M14 (Info): dashboard smoke test assumes port 6061 is free

- `🧰️…/🎛️dashboard/🧪️tests/🧭️journeys/💨️smoke/🟦️.ts:67-68` asserts the course port 6061 is not occupied. It does not pick a free port. Any "pick a free port" change for smoke must change this assertion too.

## 4. Ports per product

| Product | Port | Where it comes from | Forwarded by devcontainer |
|---|---|---|---|
| Dashboard `semio` | no TCP port of its own found (local transport, not verified in detail) | | n/a |
| Dashboard smoke | needs 6061 free | smoke test (M14) | n/a |
| Course site (`dev`, `dev-site`) | **6061** | `TEACHING_ARCHITECTURE_COURSE_PORT`; `…/📦️packages/🟦️typescript/📋️project.json:155,209`; `…/package.json:52` | **no** (M1) |
| Course site, beside stack | 6063 | `…/📋️project.json:173,220` (`steady` and `stack=beside`) | no |
| Course proctor (`@teaching/proctor:dev`) | **8791** | `PROCTOR_PORT`; proctor README `:67`; `🚀️deploy/🔣️.json:10` | **no** (M1) |
| Course proctor, beside stack | 8793 | `…/📋️project.json:174,221` | no |
| Course e2e gate | fresh OS-selected loopback ports | `test-e2e` | n/a |
| Course docker stack check | 18080 HTTP, 18443 HTTPS (defaults) | `docker-stack-check` options, course README `:702` | n/a |
| Course production stack | 80 and 443 (`COURSE_HTTP_PORT`, `COURSE_HTTPS_PORT`, plus 443/udp) | `🚀️deploy/compose.yaml:72-74` | n/a |
| Print, print-rs, presentation(-react), bericht | none (no server; `watch` only) | manifests | n/a |
| Projektetage | **6050** | `PRAESENTATION_PROJEKTETAGE_PORT`; `…/📋️project.json:21,33`; `…/package.json:46` | **no** (M1) |

## 5. EN and DE coverage

| Doc | EN | DE | Gap |
|---|---|---|---|
| `README.md` (root; dashboard golden paths 643-654, build 755-781, devcontainer 607-) | yes | no German section (only a mention at `:668`) | all EN |
| `.devcontainer/README.md` | yes | no | all EN |
| `🎓️teaching/README.md` | yes | yes (`:82`) | OK |
| `🎓️teaching/🏛️architecture/README.md` | yes | no | EN only |
| `🎓️teaching/🏛️architecture/📚️course/README.md` | yes | DE from `:727`: develop, prerequisites, commands, chain, e2e, publish | Deploy (`:352-726`) and proof of concept (`:272-351`) EN only |
| `🎓️teaching/🛂️proctor/README.md` | yes | DE from `:720`: run commands table only | Environment (`:61`), Operate (`:552-700`, docker compose) EN only |
| `🧰️…/📚️course/README.md` (framework course) | yes | no | EN only |
| `🧰️…/📓️print/README.md` | yes | no | EN only |
| `🧰️…/🎤️presentation/README.md` | yes | no | EN only |
| `🧰️…/🦑️repo/README.md` | yes | DE from `:1242` (`:1284` build tools) | not all sections checked |
| `🧰️…/🎛️dashboard/README.md` | yes (431 lines) | one German fragment | essentially EN only |
| bericht | none | none | no documentation |
| projektetage | none (only the presentation README) | none | no README |

## 6. Checks run and limits

- Read: all `📋️project.json` and `package.json` files of the three products, the dashboard CLI source for verbs, the devcontainer law and its fixture, the course e2e gate usage, the print router and test scripts, the tectonic template script and its catalog.
- Doc checks: every `repo [--tool=…] [dashboard] run <project>:<target>` in tracked non-ticket markdown was matched against the target table of all manifests. Result: one miss outside scope (`🌎️hub/README.md:590`). Relative markdown links in the product docs were checked; the three broken ones are in M8 and M11. Backticked file paths were checked; most hits are abbreviations relative to a directory, so they are not reported.
- Not verified: whether `--proctor` skips `@teaching/proctor:build` (M4); the dashboard daemon transport (no TCP port found); the `playground` ports 6070, 6072, 6073, 6066 to 6068 named in the root README and devcontainer (playground registry is out of scope, and the values were not located in non-test configs).
- Nothing was built, tested or started. No file other than this report was changed. The scratch checker scripts and their listings were removed after use.
