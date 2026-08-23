# Terra Next Accepted Dependency Scout After Pngjs — 2026-08-23

## Decision

**ACCEPT exactly one candidate: `globals`.**

This is a real, one-binding retirement rather than a manifest-only deletion. The root direct development dependency feeds the one active import in `.storybook/🟦️lint-tooling.ts`; that factory is imported by the UI React target's `🟦️eslint.config.ts` and therefore drives `@semio-tech/ui-react:lint`. The imported dataset only populates `languageOptions.globals` with `globals.browser` and `globals.node`.

The active lint profile has no `no-undef` rule at all (its printed configuration contains the 1,179 declared names but omits `no-undef`). The other active config contributions are the TypeScript parser and Storybook recommended rules. Removing the map is therefore an exact owned/platform replacement: ESLint's configured TypeScript and Storybook rule set operates without an ambient globals declaration dataset; no external data is copied or reimplemented.

## Independent Evidence

### Boundary and Consumers

At scout time, `bun ./📜️script.ts verify dependencies` reported 137 third-party identities (74 JavaScript and 63 Rust) and no new dependencies. `bun ./📜️script.ts verify dependencies parity js` reported `manifests=83`, `external-rows=259`, `evidenced=110`, `undeclared-imports=0`, and `lock-mismatches=0`.

The complete non-Compose, non-ticket source/manifest scan found exactly:

| Edge                    | Evidence                                                                                                                            |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Manifest                | `package.json` root `devDependencies.globals: ^16.4.0`                                                                              |
| Direct source           | `.storybook/🟦️lint-tooling.ts:3` imports `globals`; lines 25–26 spread `globals.browser` and `globals.node`                         |
| Active owner            | UI React's `🟦️eslint.config.ts` imports `createUiReactLintConfig`; its `📜️script.ts` invokes `eslint --config 🟦️eslint.config.ts .` |
| No other direct binding | Exact import/property scan found no other non-Compose/non-ticket source consumer and no other manifest declaration                  |

`bun x eslint --config …/🟦️eslint.config.ts --print-config …/Diagram/🟦️component.tsx` yielded 1,179 configured global names and no `no-undef` setting. This matters: the direct binding is live configuration code, but its result is semantically inert under the exact enabled rules.

### Differential Gate Already Run

Two independent runs linted the same complete UI React target (10 files).

| Configuration                                                                                                  | Result                                                   |
| -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| Current imported `globals.browser` + `globals.node` map                                                        | `bun x nx run @semio-tech/ui-react:lint` passed          |
| Same imported flat config, with only `languageOptions.globals` replaced in memory by `{}` through ESLint's API | 10 files, 0 errors, 0 warnings, 0 `no-undef` diagnostics |

The stripped run changes no production file and preserves every parser, file selector, ignore, parser option, and Storybook rule. It is the required old-versus-owned/default differential, not a claim based only on configuration inspection.

The relevant functional regression gate also passed: `bun x nx run @semio-tech/ui-react:test-quick` — 20 files, 723 tests passed.

## Executor-Ready Change Packet

1. Before editing, repeat the in-memory ESLint differential above against the UI React target. It must produce identical zero-diagnostic results. Do not add an owned copy of the browser/node names.
2. In `.storybook/🟦️lint-tooling.ts`, delete the `globals` import and only the `languageOptions.globals` object that spreads its browser/node maps. Retain the existing parser, ECMAScript version, source type, JSX parser option, and Storybook configuration unchanged.
3. Remove the one root `devDependencies.globals` declaration from `package.json`.
4. Run `bun install` to reconcile `bun.lock`, then `bun install --frozen-lockfile`. The lock's root workspace dependency tuple should lose `globals`; the package record must remain because the installed ESLint package itself declares `globals ^16.2.0`.
5. Run the permanent gates:

   - `bun x nx run @semio-tech/ui-react:lint`
   - `bun x nx run @semio-tech/ui-react:test-quick`
   - `bun ./📜️script.ts verify dependencies`
   - `bun ./📜️script.ts verify dependencies parity js`
   - literal scans for `from "globals"`, `globals.browser`, `globals.node`, and root manifest ownership
   - `bun x prettier --check .storybook/🟦️lint-tooling.ts package.json bun.lock`
   - scoped `git diff --check -- .storybook/🟦️lint-tooling.ts package.json bun.lock`

## Expected Dependency and Lock Result

The dependency verifier's direct identity boundary should move from **137 to 136**: 73 JavaScript and 63 Rust identities. Do not report a `bun.lock` package-record deletion: `node_modules/eslint/package.json` currently declares `globals ^16.2.0`, so Bun correctly retains the resolved `globals@16.5.0` package as ESLint's transitive dependency after the root direct edge is removed.

## Explicit Non-Selections

- **Sharp — DEFER.** Its previously recorded visual/two-pass print blocker remains. The current `bun x nx run @semio-tech/print:test-quick` still fails before the image pipeline with `ENOENT` for `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🔣️tokens.json`.
- **Dagre — held.** It remains installed pending the accepted Rust/Wasm/OffscreenCanvas lane, as required.
- **jsdom — DEFER.** It has one SVG keyframe parsing edge, but no existing platform or owned XML/DOM replacement was found; retiring it would introduce a new parser rather than use an exact existing replacement.
- **esbuild — not selected.** Its root direct entry has no eligible non-Compose production import; observed direct loader use is Compose or ticket material, making it manifest-only in this scoped lane.
- The remaining one-owner identities inspected have live framework/tooling responsibilities (Wasm materialization, parser, lint/build infrastructure, UI interaction/layout, i18n, graph/presentation, database, or image/canvas paths) without a small, independently demonstrated existing replacement.

## Scope and Tree Safety

No production source, manifest, lock, ticket metadata, checklist, or cache was edited by this scout. The shared tree already contained unrelated ticket additions and a concurrent `bun.lock` modification; they were observed but not altered.
