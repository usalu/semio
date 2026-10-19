# Result lw5: WP5 s-dev, s-tests, mit-bestand to framework/os

Executor `lw5`. No `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no git-modifying command.

## 1. Totals (dependency-cruiser, same policy as `result-lay0.md`, scoped run)

Scope: `✏️s/🧑‍💻dev ✏️s/🧪️tests ♻️mit-bestand --max-depth 1` (+ `🌎️hub/🧩️compositions/📏️layout` to prove the moved tests are clean). Sources limited to the three WP5 roots. Command: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs <roots> --exclude node_modules --max-depth 1 --config <lay0 config> --output-type json`; aggregation `lw5-tools/agg.ts`.

| rule | before | after |
|---|---:|---:|
| no-cross-package-relative | 173 | 162 |
| no-cross-technology | 121 | 91 |
| no-state-outside-os | 6 | 3 |
| total | **300** (64 files) | **256** (54 files) |

The audit's 393/96 is a different snapshot and target filter; the numbers above are the same tree before and after.

The remaining 256 violations are 157 distinct edges. 154 need a new `exports` entry (manifest freeze), 3 are not code-fixable here (section 4).

## 2. Changes

### 2.1 30 imports re-pointed to existing public entries (20 files)
`lw5-tools/plan.ts` (TypeScript compiler API, adapted from `lay6-tools/ncpr-fix.ts`, but for every `no-cross-package-relative` and `no-cross-technology` edge): a relative import is replaced by the bare `@semio-tech/*` specifier only when an EXISTING export entry is the target file itself, or exposes every imported symbol with the identical (alias-resolved) declaration. Applied list: `lw5-tools/applied-fixes.json`. Targets: `@semio-tech/process/{routing,leases,budget,compiler-capacity,test-budget}`, `@semio-tech/framework/value/{refusal,ieee754}`, `@semio-tech/framework-compiler-ts/syntax/module-tree`, `@semio-tech/ui-react{,/build-tooling}`, `@semio-tech/plugin-registry/catalog`, `@semio-tech/framework-renderer-react`.
Files (all under `✏️s/🧑‍💻dev/`, plus `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx` and `✏️s/🧪️tests/🏘️plugin-composition/🌀️procedural-formats/🗿️artifact-surface/🟦️.ts`): cad script and config test, stdio composition (`artifact-commands`, `commands`, `artifact-graph`) and command-ownership test, gis variant (`publisher`, 4 service tests), catalog (`browser-host/staging`, host-activation, session, projection, integration tests), trinity graph, block coordination story.

### 2.2 Layout drawing composition tests moved to the hub (direction fix)
`✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/{🪶️sqlite,🪪️document,🔗️binding}` imported seven JSON fixtures from `🌎️hub/…/📏️layout/🖊️drawing/🧪️tests/🪶️sqlite/🧫️fixtures` (s depending on hub; the Rust twin in the hub owns and `include_str!`s the fixtures). The three directories now live beside the fixtures and the Rust twin: `🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🧪️tests/{🪶️sqlite,🪪️document,🔗️binding}/🟦️.ts`. Relative paths into `🧰️framework/…/🧬️schema` and `✏️s/🔌️plugins/📏️layout/…` were re-based (hub depends on s and framework, the permitted direction). This also removes 3 `no-state-outside-os` hits (hub is outside that rule's `from`).
- `✏️s/🧪️tests/🏘️plugin-composition/📜️script.ts`: both files removed from the test list.
- `🌎️hub/🧩️compositions/📜️script.ts`: new `typescript` command (`TypescriptScript`, runs both files with `bun test` via `runRepositoryCommand`); `📋️project.json`: new `typescript` target; `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: one entry `🧪️typescript🌎️hub🧩️compositions🟦️` (group `4_gate`, order 900.0705) after the `native` entry.

### 2.3 Exports spec for the batch (lay6 format)
- `lw5-tools/exports-needed.json`: 9 packages, 93 new export entries covering 154 importer edges: `repo-lib` 53 (it exports only `./package.json` today), `framework-plugin` 16, `plugin-registry` 15, `framework-os` 4, `framework-os-dev` 1, `framework-renderer-react` 1, `framework-renderer-wgpu` 1, `nx-bootstrap-tools` 1 (frozen bootstrap path, spec only), `s-dev-catalog-ts` 1. No proposed-subpath collisions. Overlaps with `lay6-tools/exports-needed.json` for `framework-plugin`/`plugin-registry`/`framework-os*`: union them. After the entries exist, re-run `lw5-tools/plan.ts <report> apply <out>` against a fresh cruise report; it rewrites the 154 edges (the 8 namespace/dynamic uses need an entry exposing everything: `symbols: "*"`).
- `lw5-tools/dependencies-needed.json`: manifests that must list the new bare specifiers once the freeze lifts: `✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/package.json` (`@semio-tech/framework-compiler-ts`), `✏️s/package.json` (`@semio-tech/ui-react`, `@semio-tech/plugin-registry`, `@semio-tech/framework-renderer-react`). The cruiser does not flag these; `Bun.resolveSync` and tsc resolve all 30.

## 3. Verification (exact commands, outcomes)
All from `/Users/ueli/Documents/semio`, foreground.
- Resolution: `bun lw5-tools/resolve.ts <applied-fixes>`: 30 specifiers resolve from their importer; 27 land on the target file, 3 on the package glue file that re-exports it (`compiler-ts/syntax/module-tree`, `process/leases`, `ui-react/build-tooling`; symbol identity was checked by the type checker).
- tsc (TypeScript API, root `tsconfig.json` options, program over the 25 touched files, `lw5-tools/tsc-touched.ts`): 0 diagnostics. Control: a probe file with a missing export, a missing module and a type error yields exactly TS2305, TS2307, TS2322.
- `bun test ./🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🧪️tests/🪶️sqlite/🟦️.ts`: 19 pass, 801 expects (identical to before the move: 19 pass, 801 expects); `…/🪪️document/🟦️.ts`: 1 pass.
- `bun test` of the touched dev/catalog tests: `host-activation` 5/5, `projection` 1/1. Failures are identical with the edits reverted (`lw5-tools/swap.ts revert|apply`): stdio `command-ownership` (`phase.leaseControl.signal` undefined, 3 fail), catalog `session` (`factory.runIf` is a vitest API under `bun test`). Not related to this WP: gis tests (live dev session or script invocation envelope required), catalog `integration` ("Invalid taxonomy schema" from concurrent taxonomy edits; it ran 6/7 earlier in the session).
- Final cruise: section 1.
- `bun repo run @semio-tech/hub-compositions:typescript` could not be run: the CLI stops after 6 min with "Locked workspace package lacks original admitted Source" in the dependency store (`📦️dependencies/💾️storage`, deps owner). Direct `bun test` of both files is the evidence.

## 4. Still open
- 154 edges: need the exports in 2.3 (manifest batch).
- `no-state-outside-os` 3 (`✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/{👷️worker,🧪️tests/🧰️dependencies}`, `✏️s/🧑‍💻dev/🧩️service-composition/👷️worker`) are policy false positives: they already import the public `@semio-tech/framework-os/modules/store/worker`, but the rule matches the resolved path. Fix in the policy (`🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, rule `no-state-outside-os`): add `dependencyTypesNot: ["aliased-workspace"]` like `crossTechnologyRules`. Not edited (outside my area).
- `@semio-tech/stdio-docx` (2 edges, `✏️s/🧑‍💻dev/🗄️stdio/🟦️.ts` and the `.d.ts`): the package root export points at `./dist/🟦️.js`, a build output that does not exist, so the specifier is unresolved. Needs a source `.` entry or a build.
- `✏️s/🧑‍💻dev/🧱️block/📖️stories/🧭️coordination/🟦️.ts` imports `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb?url`; the framework mesh catalog (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`) names the same file as `source`. Asset ownership decision (framework catalog referencing mit-bestand), not a code fix.
- `bun repo run @semio-tech/hub-compositions:typescript` untested (dependency store error above).

Ticket tools (inputs, kept): `lw5-tools/{plan,agg,edges,resolve,tsc-touched,swap,exports-needed,deps-needed}.ts`, `exports-needed.json`, `dependencies-needed.json`, `applied-fixes.json`, `remaining-edges.json`, `touched.txt`. Generated output deleted.
