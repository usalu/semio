# S-Compose Integration

<!-- #region Outcome -->

## Outcome

The root command router no longer dispatches to the intentionally deleted `compose/` product tree. The only production file changed by this packet is `/Users/ueli/Documents/semio/📜️script.ts`.

The audit treated both `compose/` and `temp/compose/` as forbidden: neither tree was traversed, read, changed, or used as evidence. Git state and manifests were not modified.

<!-- #endregion Outcome -->

<!-- #region RootScript -->

## Root script changes

- Native OS bootstrap no longer injects the legacy `COMPOSE_REPO_ROOT` override; the owned bootstrap resolves its root itself.
- Bare `dev` now starts the framework OS playground instead of the removed Compose desktop project.
- Removed the dead `dev mcp engine` dispatch into `compose/client/bin/engine`.
- Neo4j defaults are schema-independent and use the first live product graph (`elements`) rather than a `compose` database; the product graph set is now `elements`, `coda`, and `reuse`.
- Dependency-cruiser and Tailwind discovery no longer scan the deleted root.
- Removed Compose-only build slices (`3dm`, `desktop`, `engine`, `sites`), publish slices, and the `query` command/router that targeted `compose/client/lib/query`.
- Removed Compose-specific Python dependency classification, exact obsolete policy allowlist entries, and the structural `compose/**/hub/**` database-policy exception.
- Updated examples and policy prose that named deleted concrete paths.
- Taxonomy CLI artifacts now write beneath canonical semantic directories: JSON at `📊️taxonomy-{operation}/🔣️.json` and summaries at `📓️taxonomy-{operation}/📝️.md`. Writers create those directories; the generated-plan default and apply result can no longer introduce flat noncompliant ticket paths.

The remaining root-script `compose/` literals are intentional lexical boundary guards: taxonomy opaque digest handling, directory walkers, dependency inventory, Storybook discovery, interaction audit, OS-state authority, and state-lane policy. There are 15 `compose/` occurrences and seven exact `"compose"` occurrences; every one is an exclusion, skip, or conditional opaque-tree digest. No mapping or operational target remains.

Direct operational-reference acceptance pattern:

```text
(compose/client|@semio-tech/compose|compose-(desktop|play|docs|sketchpad|3dm|engine)|COMPOSE_REPO_ROOT)
```

Result in `📜️script.ts`: zero matches.

<!-- #endregion RootScript -->

<!-- #region Residuals -->

## Residual references for separate writers

These are outside this packet's sole production ownership and still require reconciliation:

| Family | Current evidence | Risk / required owner action |
| --- | --- | --- |
| Python workspace and test bootstrap | `pyproject.toml:12,36,44`; `conftest.py:40-59`; `.vscode/settings.json:76` | `uv` membership, Pytest discovery, and import-time loading still require deleted `compose/py` and `compose/engine`; Python/config owner must remove or replace them. |
| JS/TS static tooling | `.dependency-cruiser.cjs:5,45,50`; `eslint.config.mjs:121`; `tsconfig.json:19` | Dependency-cruiser self-fixtures, an ESLint override, and TS includes name deleted paths. Preserve only the generic `compose/**` exclusion where policy still requires the lexical boundary. |
| CI | `.github/workflows/playwright.yml:31`; `.github/workflows/gh-pages.yml:32,36`; `.github/workflows/play-sites.yml:20-21` | Workflows invoke absent Nx projects and upload absent Compose distributions. CI owner must delete or retarget those jobs. |
| Devcontainer Neo4j environment | `.devcontainer/post-start.sh:102-132` and following migration region | Login shells still force `NEO4J_DATABASE=compose` and use Compose-named profile/database migration functions, diverging from the root script's live `elements` default. |
| GitHub agent profiles | `.github/agents/compose.agent.md`; `.github/agents/general.agent.md:2-3`; `.github/agents/elements.agent.md:2-3` | Active agent metadata still advertises the deleted technology or uses the wrong identity. |
| Repository documentation | `README.md:4-30,134-174,377,718-831` and later technology matrix links | Images, examples, technology links, and badges resolve under the deleted tree and are now broken. |
| Repo bootstrap branding | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/⌨️script.sh:12-56`; sibling PowerShell bootstrap at lines 42-43 | The bootstrap remains functional through its owned fallback, but retained `COMPOSE_REPO_ROOT` / `COMPOSE_SESSION_START` names should be renamed by its owner as a coordinated public-environment change. |
| Repo CLI implementation/tests | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go:10462-10563` and numerous `🧪️component_test.go` fixtures | Several operational tree/search paths and test expectations still model Compose as a live technology. Repo CLI owner must distinguish generic lexical-exclusion fixtures from deleted-product behavior. |

Historical `.cursor/`, `.kiro/`, `.ralph-tui/`, and Copilot plan records also contain Compose paths. They were classified as archival evidence rather than executable integration and were not changed.

The taxonomy contract remains intentionally unchanged: `🔣️taxonomy.json` and discovery validation still register `compose/` as an opaque lexical exclusion. An absent prefix remains valid.

<!-- #endregion Residuals -->

<!-- #region Verification -->

## Verification

| Check | Result |
| --- | --- |
| `bun -e 'await import("./📜️script.ts"); ...'` | Exit 0; `[DEBUG] root script import ok`. |
| `bun ./📜️script.ts --help` | Expected exit 1 for unknown option; usage rendered successfully and lists no `query` or Compose command. |
| `bun ./📜️script.ts verify dependencies self-test` | Exit 0; `hostile-mutations=17 clean`. |
| `bun nx show projects` | Exit 0; graph discovery completed and contains no case-insensitive `compose` project. |
| `bun nx show project @semio-tech/framework-os-dev --json` | Exit 0; live `dev` target resolves to the owned OS dev script. |
| Direct operational-reference guard | Exit 0; zero matches in root script. |
| Flat taxonomy-artifact guard | Exit 0; no `📊️taxonomy-*.json` or `📓️taxonomy-*.md` root-level output literal remains. |

Final deterministic root-script digest:

```text
sha256  cc39130017cf1402ebe54702740514c539ce93de5917fa04ebca6f8b8c47e88d  📜️script.ts
```

Acceptance status: root import and command help are healthy, the Nx graph is Compose-free, no root command dispatches into the deleted tree, and the lexical opaque exclusion is retained.

<!-- #endregion Verification -->
