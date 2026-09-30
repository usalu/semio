# Removability Enforcement Audit

Read-only source inspection on 2026-09-30. No implementation edits or test execution were performed. Read root AGENTS.md. Paths below are repository-relative.

## Existing Enforcement

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs` owns dependency-cruiser rules, workspace package ownership, and plugin isolation.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:31` runs strict framework source inventory and dependency-cruiser verification without baselines. Its canonical architecture target runs neutral TS dependency-direction tests, Cargo declaration tests, and actual Cargo graph verification.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts` checks source inventory completeness, unresolved local imports, package ownership, declared public exports, resolver verdict agreement, and graph totals.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts` compares independent authored Cargo inventory against metadata, including aliases, normal/dev/build dependencies, optional declarations, and platform declarations.
- Portable fixtures and schemas already exist at library `🧫️fixtures/🧱️dependency-direction/🔣️.json` and `🧬️schema/🧱️dependency-direction/🔣️.json`. Tests at `🧪️tests/🧱️dependency-direction/🟦️.ts` use Ajv and dependency-cruiser as independent validation oracles.
- Root `📜️script.ts:8958` orchestrates owner-contributed `canonical-architecture` targets through Nx. `nx.json` disables caching for that target. Root `📋️project.json` and `.vscode/launch.json` already register architecture/layering verification commands.

## Confirmed Gaps

1. Boundary configuration eagerly reads `✏️s/🔌️plugins` using `readdirSync` at line 24. Removing s prevents the framework policy from loading.
2. Boundary configuration eagerly reads every root workspace package manifest at line 102. Removing an implementation listed in the root workspace manifest prevents policy loading. The strict TS runner repeats that requirement at lines 45–50.
3. Strict TS inventory contains taxonomy framework roots only. Root router sources are omitted, despite taxonomy `_areaLayerComment` expressly requiring repo-wide sources to remain independent of implementations.
4. Root router imports concrete puzzle/FEM tests directly at lines 5, 7, 9, and 14–16. Removing the corresponding plugin prevents root router initialization.
5. Root `runGate` calls dependency-cruiser with hardcoded framework, s, hub, and demonstrator roots. Removed optional areas remain mandatory command arguments.
6. `s-modules-no-plugins` and `no-plugin-to-extension-*` rules exist in CJS, but strict TS verification selects only `framework-no-implementation` and taxonomy semantic role rules. Procedural and CAD extension rules explicitly remain warnings through `GRANDFATHERED_PLUGINS`.
7. Cargo policy declares an `artifact` role but has no plugin-to-artifact prohibition. Its three semantic rules currently cover framework-to-implementation, module-to-plugin/extension, and plugin-to-extension only.

## Recommended Implementation Scope

Extend existing dependency-direction ownership/discovery and neutral fixtures instead of adding a second policy system. General policy should discover present optional owners, reject malformed present owners, and obtain owner contributions through the owner contract. General policy must not eagerly initialize a concrete application directory. Keep strict graph completeness and authored-export checks intact.

Move concrete root test execution into their existing owning package scripts and let the root router execute discovered contributions. Add repo-wide source inventory and a strict repo-wide-to-implementation rule after removing those imports. Obtain active graph roots from present taxonomy areas rather than hardcoded implementations.

Treat plugin core, extensions, and artifacts as explicit ownership roles. Inventory real plugin-to-artifact dependencies before introducing that strict rule; fix concrete violations rather than recording baselines or suppressions. Retire existing warning exceptions as their implementation edges are removed.

## Portable Test Cases

- Missing optional s directory yields an empty plugin inventory and still loads framework policy.
- Missing optional plugin directory removes only that owner's contributions.
- Missing optional artifact directory leaves the plugin contract and package graph valid.
- Missing implementation manifest associated with a removed directory is absent from discovery; malformed manifests in present directories fail.
- Retained general source importing a deleted local target fails unresolved ownership checks.
- General sources cannot import implementation types, dynamic imports, public package aliases, or exported subpaths.
- Owner contributions remain discoverable after adding a previously unknown plugin, without modifying general policy.
- Graph source inventory remains complete after optional area removal; removing general source entries from resolver output still fails.

Use the existing JSON fixtures/schema, Ajv, and dependency-cruiser oracle for language-neutral expected verdicts. Use Cargo metadata as the independent oracle for Cargo declaration cases. Run checks through existing Bun/Nx script targets; extend launch registrations only for new executable commands.
