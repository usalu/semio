# glTF Module Consumer-Graph Correction

## Scope and Evidence

The scoped command was run after the mutation SCC release:

```text
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

Initial result: 58 components, 7 errors, 0 warnings.

| Finding | Count | Proven disposition |
| --- | ---: | --- |
| `manifest-child-missing` for glTF under the stdio artifact root | 1 | Parent artifact-root registrar lease; not suppressible by glTF |
| `module-consumer-graph-mismatch` for inference-measures, mesh-topology, vector-operations | 3 | Correct discovery adapter |
| `module-production-consumer-minimum` for the same modules | 3 | Consequence of the same adapter false negative |

The three module manifests retain their actual terminal inference consumer IDs. They did not acquire or lose a production consumer in the mutation relocation.

## Cause

The migrated Rust leaves import the logical namespace `schema::modules` with relative `super::…` paths. The modules are intentionally physically located at the subset owner’s sibling `🔨️modules` collection and mounted into `schema` by generated Rust glue. Discovery previously resolved each relative namespace by direct physical-directory lookup only. At physical `schema`, it could not find `modules`, emitted no incoming edge, and therefore computed an empty reverse closure for all three modules.

The mutation SCC made this topology explicit by registering `mutation_dispatch` beside the existing schema modules and removing the former nested mutation-facet mounts. It did not change the three modules’ semantic consumers. The zero-consumer result was thus a source-graph model defect, not evidence to dissolve or weaken those modules.

## Central Correction

Changed only:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

The resolver now maps only the logical `modules` namespace to the nearest ancestor’s physical `🔨️modules` directory after direct lookup fails. It preserves direct physical resolution for every other namespace; it adds no forwarding export, alias, baseline, or path exception.

The regression fixture has two inference components below a physical `schema` directory that import a module physically owned by its nearest ancestor. It proves resolved terminal consumers are `height` and `width` and that the module satisfies the independent-two-consumer rule.

## Validation

```text
bun nx run @semio-tech/repo-lib:test --skip-nx-cache -- --test-name-pattern='logical schema modules'
```

Result: pass (1 test, 0 failures).

```text
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

Result after correction: 58 components, **1 error**, 0 warnings. All six module graph/minimum findings are resolved.

`git diff --check` passes for both central files.

The unrestricted `@semio-tech/repo-lib:test` target was also attempted. It currently exits 1 with 18 failures outside this semantic regression, including dependency-boundary, stale UI-path, command-budget, taxonomy/package-discovery, and workspace-catalog tests. The focused regression passes through its Nx target.

## Remaining Blocking Finding

```text
manifest-child-missing — ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf
```

The real semantic collection root is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`, not the glTF directory. It has 36 direct artifact children and no canonical `🔣️component.json`. A glTF-only declaration there would violate the required exact collection/tree bijection for the remaining 35 children. The stdio plugin is active (`clean`), so structural exclusion is not available and a path-specific exception is forbidden.

The next correct lease owns the stdio artifact-root collection manifest and classifications for all 36 direct artifact children, including any required immediate component-leaf work revealed when those members are declared. It is intentionally not hidden by scoped reporting.
