# Taxonomy Scope Selection Fix

## Defect

`verify taxonomy report --scope s.stdio.gltf` selected only registered component IDs. The `s.stdio.gltf` owner has unregistered ancestor collection paths, so its report incorrectly showed zero findings while the deterministic census exposed collection manifest/tree violations below the same glTF artifact owner.

## Change

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` now resolves a semantic-id scope to its actual lowest matching owner boundary. A scoped problem is included when its path lies within that boundary or its component ID belongs to the semantic-id prefix. This retains a component-only record/graph view while making unregistered collection-path violations visible to report and enforce modes.

The implementation uses owner ancestry, exact semantic-id segment matching, and path-boundary checks. It introduces no baseline, allowlist, compatibility path, or exception mechanism.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` adds a regression fixture with a scoped semantic owner, an unregistered collection-root behavior error, and a sibling module error outside the matching component directory. The scope must surface both errors.

## Validation

Executed successfully:

```text
bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --test-name-pattern 'semantic collection census'
```

Result: 9 passing tests, 24 assertions.

Executed after the fix:

```text
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

Result: 23 components, 80 errors, 0 warnings. The command no longer reports a false zero. The earlier 84 path-filter count changed because the independently active standards/subset manifest lease added glTF structural paths while this central validation ran. The remaining errors are real and remain visible; no owner can graduate to clean until they reach zero.

`git diff --check` is clean for the changed discovery and test paths.
