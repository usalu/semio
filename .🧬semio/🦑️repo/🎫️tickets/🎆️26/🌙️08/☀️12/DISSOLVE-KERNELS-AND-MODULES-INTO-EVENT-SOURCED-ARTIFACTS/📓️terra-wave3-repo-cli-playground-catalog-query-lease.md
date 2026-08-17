# Wave 3 Repository CLI Playground Catalog Query Lease

## Handoff And Scope

- Lease: `framework.repo.command.playground-catalog-query`.
- Owner: `🧰️framework/🛍️products/🦑️repo/🎮️commands/📇️playground-catalog-query/🦀️component.rs`.
- The released usage-presentation handoff was confirmed before the edit:

```text
69e589cd050ed5f32aafe13d1b242c03b797da3f66097127e6f946bf12ce1d14  CLI Rust glue
0480f49928be6f074f24b7eeb289b227962d1b287fc14d4ebd51d4a4733114fb  command collection manifest
```

- `catalog` remains a direct import of the already-proven future repository-product `playground-catalog` module. No catalog/process module promotion, repository module-collection manifest, compatibility alias, registry, generator, root-script, launch, or protected path was changed.

## Change

- Added the single packet-prescribed local mount and redirected only the `"catalog"` dispatch arm.
- Preserved `pub mod catalog` and all generated-file decode/pass-through behavior for its later qualified module lease.
- Added the exact command manifest member.
- Added a component-local deterministic tabular-row presentation test.

Post-edit hashes:

```text
36547e9e54a15e72edc0bfd7ce1e3adfc89e835e1eda165cf8616c82d9ddc6f2  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
a4c3339be6c8a452dfb85965879fa16b6916f55489c6e7063b0a4ec9467648ed  🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
5d44b857976feff2c9a4b74a2f532c4cda20fa58f65c7fb06837bb72f7119a1d  🧰️framework/🛍️products/🦑️repo/🎮️commands/📇️playground-catalog-query/🦀️component.rs
```

## Validation

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.playground-catalog-query` | Passed: 1 component, 0 errors, 0 warnings. |
| `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.playground-catalog-query` | Passed: 1 component, 0 errors, 0 warnings. |
| `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` | Passed: 20/20 tests, including `playground_catalog_query::tests::table_text_preserves_catalog_order_and_row_wire_format`. |
| `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache` | Passed: release build. |
| `bun nx run @semio-tech/repo-cli-rs:run -- catalog` | Passed: emitted 58 deterministic tabular rows from `aggregator` through `writer`. |
| `bun nx run @semio-tech/repo-cli-rs:run -- catalog --json` | Passed: emitted the raw generator JSON unchanged. |
| Mount/dispatch sweep | Exactly one `playground_catalog_query` mount and catalog dispatch; retained `pub mod catalog`; no wrapper alias. |
| Diff checks | `git diff --check` and the untracked-component `--no-index --check` emitted no whitespace errors. |

The registered JSON runtime output was the 58-entry, 24,722-byte generator payload below; its read-only SHA-256 was unchanged after runtime execution:

```text
f647dcbac6d863703c60238047e1b1b3c4ce4c4a74d395afd820934a322b2458  🔣️playgrounds.json
first entry: aggregator / demonstrator / react:6023 / wgpu:6123
last entry: writer / writer / react:6062 / wgpu:6162
```

The table runtime printed the same ordered boundary rows in terminal form:

```text
aggregator	demonstrator	react:6023	wgpu:6123
… 56 ordered rows …
writer	writer	react:6062	wgpu:6162
```

The CLI validations emitted existing framework TUI and CLI daemon warnings only. No conclusion is drawn about the separately quarantined framework plugin capability Cargo condition.

## Registrar Request

None. The exact local mount and command collection member are part of this lease; no central registrar action is needed.
