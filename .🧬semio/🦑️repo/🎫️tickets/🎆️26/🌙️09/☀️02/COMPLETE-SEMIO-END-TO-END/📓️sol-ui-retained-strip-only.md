# UI Retained Native Strip-Only Repair

## Scope

This packet owns only:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts`
- the existing `@semio-tech/framework` package test router `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- this report

It follows the next blocker recorded by `📓️sol-ordered-numeric-strip-only.md`. No ordered-numeric, actor-return, retained validation, Hub Rust, DB, replication/bootstrap, store, MCP/plugin-host, WGPU, root manifest, launch configuration, plan, acceptance, lifecycle, goal, ticket, or `AGENTS.md` source was edited.

## Red Evidence

Node `v24.15.0` rejected the owned production entry at the first parameter property:

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts'
exit 1
🟦️.ts:32 constructor(index: NumericIndex<V>, readonly grant: () => NumericIndexGrant, private readonly retired: (value: V) => void = () => {})
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
```

The permanent owning-package Node child regression was installed before the production repair. Its first Nx run failed before Vitest with the same line 32 diagnostic, so the regression reproduced the actual native-loader boundary rather than a transpiled approximation.

The complete owned module census found one additional parameter-property site at the former line 180: `SiblingKeys` captured `private readonly grant` in its constructor.

## Implementation

- `Table` now declares `grant` and `retired` explicitly and assigns both before the pre-existing `#index` assignment. This preserves the original parameter-property lowering order and the default retirement callback.
- `SiblingKeys` now declares its private grant field explicitly and assigns the ordinary constructor parameter at the same constructor phase as before.
- No table capture, lookup, iteration, edit publication, ordinal allocation, old-root retirement, callback order, sibling hash, graph validation, transaction, hydration, cancellation, or publication behavior changed.
- The complete owned module has zero remaining parameter-property, TypeScript enum, namespace, or import-assignment matches. Actual Node entry and import are the authoritative strip-only parser checks.
- The existing `@semio-tech/framework` test command now runs a real Node 24 `--experimental-strip-types` child before Vitest. The child imports the production retained and numeric modules, reads the existing language-neutral `🔣️owned-nodes.json` fixture through `node:fs`, and uses independent `node:assert/strict`, native arrays, and reference identity.
- The child exercises four fixture laws directly: blocked zero-item grant without publication, exact node identity, deletion/reinsertion ordinal order, and cancellation retirement. It additionally exercises both sibling-key insertion outcomes and complete cleanup.
- Existing renderer retained tests remain the separate AJV schema and Immer value oracle; no new dependency was introduced.

## Verification

### Native source

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts'
exit 0

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts", import.meta.url))'
exit 0
```

### Owning package quick gate

```text
bun nx run @semio-tech/framework:test-quick --skip-nx-cache
exit 0
[DEBUG] retained UI native strip-only oracle: 4 fixture laws, 1087 grants, 4 one-time retirements, 1 cancellation, 2 sibling-key operations
1 file passed
88 tests passed
0 failed
```

The cancellation oracle separately asserts that its retained source object and unpublished candidate object are each retired exactly once. The reported four retirements are the three initial fixture values plus the reinserted replacement in the publication-order scenario.

### Focused retained renderer oracle

```text
bun nx run @semio-tech/framework-renderer-react:test-exhaustive --skip-nx-cache -- '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️.tsx' -t 'Retained UI patch preparation|Retained UI atomic publication'
exit 0
1 file passed
21 selected tests passed
189 unrelated tests excluded by the explicit name filter
```

These 21 selected tests exercise both grant sizes across all eight language-neutral patch cases, every semantic cancellation phase, retained old-root capture, stale/rebound publication, acknowledgement timing, AJV validation, and the independent Immer value oracle.

### Hub build frontier

Both aggregate probes advance beyond the repaired owned module. Neither reaches Cargo because the next independent native strip-only source now fails:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts:22
constructor(source: OwnedUiNodeIndex, private readonly grant: () => NumericIndexGrant)
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]

bun nx run os-hub:build --skip-nx-cache
exit 1
os-hub-admin:build fails at the same retained-validation parameter property before Cargo is invoked
```

Therefore both parameter-property sites in the owned retained root are cleared; no successful admin, aggregate Hub, or Cargo build is claimed.

## Hygiene

- `git diff --check` over both owned source files exits 0.
- The owned unsupported-syntax census exits 0 with zero matches.
- No temporary or generated files were created for this packet, so there is no packet-owned generated subtree to remove.
