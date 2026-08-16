# Wave 3 Repository Module-Collection Frontier And Catalog-Query Packet

## Read-Only Boundary

This audit made no source, configuration, generated-output, or registry change. It excludes the active `cli-usage-presentation` lease, every `✏️s/🔌️plugins/🗄️stdio/**` Stage 2 registry/artifact path, root `📜️script.ts`, `.vscode/launch.json`, Cargo workspace files, framework plugin capability core/builder, quarantined kernel/machine/platform/renderer owners, and the protected repository-library TypeScript index.

## Exact-Bijection Result

`🧰️framework/🛍️products/🦑️repo/🔨️modules` is a semantic module collection by taxonomy suffix, but it has no canonical `🔣️component.json`. Its direct children and immediate canonical language leaves are:

| Direct child | Immediate canonical leaves | Authored source scale | Decision |
| --- | --- | ---: | --- |
| `⌨️cli` | none | 3 files / 1,600 lines | In-progress dissolution into named commands and future shared capabilities; do not declare it as a module. |
| `💻️client` | none | 9 files / 75,516 lines | Separate Go CLI, MCP, VS Code, and SQLite boundaries; not one component. |
| `📚️library` | `🐹️component.go` | 8 files / 10,621 lines | Central taxonomy/discovery and protected TypeScript index owner; quarantined. |
| `🔩️native` | none | 2 files / 1,484 lines | Live cross-platform native bootstrap action, selected by root `📜️script.ts`; not dead code. |
| `🖥️server` | none | 20 files / 3,629 lines | Coordinator application plus server library/worker boundaries; not one module. |

The taxonomy validator requires a collection manifest's members to be in exact bijection with **all** direct directories and requires each member to have an immediate canonical component leaf. A partial manifest containing only future `playground-catalog` and `process-invocation` modules would create five `manifest-child-missing` errors. Declaring the five umbrellas as modules would add false `member-component-leaf-missing` and module-consumer failures. Consequently there is no small, conflict-free, exact-bijection lease that can create the repository `🔨️modules` manifest now.

The root-module frontier is dependency ordered:

1. Finish CLI command/capability dissolution so `⌨️cli` disappears rather than becoming a one-consumer umbrella.
2. Independently split or relocate `💻️client`, `📚️library`, `🔩️native`, and `🖥️server` into specific product actions/apps/modules, preserving their active runtime mounts.
3. Only then create the repository module collection manifest for surviving qualified modules, including `📇️playground-catalog` and `🖥️process-invocation` at their repository-product LCA.

`🔩️native` is specifically not deletable: root `📜️script.ts` resolves `NATIVE_BOOTSTRAP_DIR` and invokes `⌨️script.sh` on Linux/macOS or `🪟️script.ps1` on Windows; the repo-client test suite asserts both paths and expected cross-platform build fragments. Its owner needs a future central root-script/runtime lease.

## Proven Future Shared-Capability Graph

| Capability currently inside CLI glue | Independent production terminals | LCA | Future disposition |
| --- | --- | --- | --- |
| Generated playground catalog contract, JSON decode, raw JSON pass-through, and generated-directory resolution | plugin-registry command; playground-development-session command; terminal dashboard presentation; catalog query command | repository product | `framework.repo.module.playground-catalog` at `repo/🔨️modules/📇️playground-catalog` once the collection frontier is clean. |
| Inherited-stdio process invocation | plugin-registry command; playground-development-session command | repository product | `framework.repo.module.process-invocation` at `repo/🔨️modules/🖥️process-invocation` once the collection frontier is clean. |

Tests, assembly glue, and the root fallback dispatcher do not increase either count. No compatibility re-export is allowed when those modules are eventually relocated.

## Next Dependent Terra Packet: Playground Catalog Query

The next narrow CLI action is a catalog-query command, but it must begin only after the active usage-presentation lease releases the shared CLI glue and command manifest paths. It shares nothing with the stdio SCC or the capability quarantine.

```text
current action in: 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
destination:       🧰️framework/🛍️products/🦑️repo/🎮️commands/📇️playground-catalog-query/🦀️component.rs
semantic id:       framework.repo.command.playground-catalog-query
```

### Handoff Gate And Snapshot

At audit time the active usage lease owns these transient values:

```text
69e589cd050ed5f32aafe13d1b242c03b797da3f66097127e6f946bf12ce1d14  CLI Rust glue
0480f49928be6f074f24b7eeb289b227962d1b287fc14d4ebd51d4a4733114fb  command collection manifest
```

They are observations, **not** valid edit preconditions. The catalog-query worker must wait for the usage Terra release report, reread all applicable instructions, confirm no active lease owns either path, and record fresh SHA-256 values before editing. It must stop on any unowned modification.

### Exact Writable Paths

| Path | Operation | Required result |
| --- | --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` | Modify after handoff | Add exactly one local mount; replace only the `"catalog"` dispatcher arm with `playground_catalog_query::run(&root, &parsed)`. Keep all catalog capability code in place for its later qualified-module lease. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` | Modify after handoff | Add exactly the semantic member below; preserve released usage and all earlier command members. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/📇️playground-catalog-query/🦀️component.rs` | Create | Own the `catalog` query's JSON/text output selection and deterministic tabular row presentation, with component-local output tests. |

The required member is:

```json
{
  "directory": "📇️playground-catalog-query",
  "id": "framework.repo.command.playground-catalog-query",
  "kind": "command",
  "responsibility": "Lists generated playground registrations as canonical JSON or tabular terminal text."
}
```

The only mount is:

```rust
#[path = "../../../../🎮️commands/📇️playground-catalog-query/🦀️component.rs"]
pub mod playground_catalog_query;
```

`run(root: &Path, parsed: &ParsedArgs) -> i32` imports `crate::catalog::{load_playground_catalog, playgrounds_json_text, PlaygroundEntry}` directly. This is an intentional temporary direct referrer to the already-proven future `playground-catalog` module, not a wrapper, alias, or new generic helper. Preserve the exact wire behavior: `--json` writes the generator-emitted raw JSON unchanged; text prints one `variant<TAB>plugin-id<TAB>react:<port><TAB>wgpu:<port>` row per catalog entry. The pre-existing raw-JSON fallback/pass-through tests remain with the current catalog capability until its later module move; the new command owns deterministic tabular presentation tests.

### Validation

```text
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.playground-catalog-query
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.playground-catalog-query
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:run -- catalog
bun nx run @semio-tech/repo-cli-rs:run -- catalog --json
rg -n '"catalog"\s*=>|playground_catalog_query|pub mod catalog' -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs 🧰️framework/🛍️products/🦑️repo/🎮️commands
git diff --check -- <three writable paths>
```

The two runtime invocations use the existing registered `run` target; they do not regenerate or edit the protected plugin-registry outputs. Capture the actual table/JSON output in the release report. There is no new package, Cargo, Nx, launch, root-script, schema, registry, or generated-output change.
