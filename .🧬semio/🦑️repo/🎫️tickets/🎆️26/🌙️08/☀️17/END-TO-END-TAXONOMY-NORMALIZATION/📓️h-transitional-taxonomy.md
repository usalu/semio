# H-TRANSITIONAL-TAXONOMY — removal ledger

## Decision

Taxonomy version 7 already declares every area `clean`, but final convergence must also delete unreachable compatibility logic. These sites are retained only while the physical migration is incomplete; none may survive the empty-second-plan gate.

## Exact taxonomy/discovery residues

- `🧰️framework/🛝️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` still declares `packageMaturityStates: ["clean", "mixed"]` and `migratedMarker: "packages-dir-exists"`.
- `…/🔍️discovery/🟦️component.ts` still exposes `PackageMaturity = "clean" | "mixed"`, derives mixed owners from forbidden implementation directories/owner-root entries, and returns a `mixedOwners` partition.
- Those derived diagnostics remain useful before apply, but after all implementations leave package boundaries they must collapse to one clean shape and the transitional fields/types/branches must be removed rather than preserved as dormant API.

## Exact policy/registry residues

- `🧰️framework/🛝️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` still merges impossible `legacy`/`mixed` area states and conditionally downgrades taxonomy findings to warnings. Version 7's `AreaState` is already only `"clean"`; the merger and warn-only branches must be deleted.
- The same script retains a mutation-facet fallback described as `legacy root 🧬️mutations during migration`. After projection/migration, discovery must accept only the registered schema-owned location.
- Root `📜️script.ts` still models Rust packages as `shape: "legacy" | "taxonomy"`, searches a legacy entry fallback, and carries compatibility paths for old implementation/config/plugin facet layouts. Each relevant branch must be deleted after the full inventory proves its source population is zero.

## Compose wording

Workspace scanners may keep the exact lexical `compose/` exclusion because it remains the one schema-owned opaque prefix even while absent. Comments must describe it as an intentional opaque boundary, not a legacy technology that should be rediscovered or restored.

## Gate

Before ticket closure:

1. Full inventory shows zero owners requiring a legacy/mixed package path.
2. Remove the taxonomy fields, discovery API partitions, registry warning downgrade, and root fallback branches in one incompatible update.
3. Regenerate/check the plugin registry and launch output through their owner.
4. Run the affected discovery, package, policy, and plugin-registry tests.
5. Search the exact taxonomy/package migration terminology again; unrelated domain uses of the words `legacy` or `mixed` are not part of this ledger.
