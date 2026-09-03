# UI Validation Native Strip-Only Repair

## Scope

This packet owns only:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts`
- the existing `@semio-tech/framework` package test router `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- this report

It follows the next blocker recorded by `📓️sol-ui-retained-strip-only.md`. No retained-root, read-lease, hash, Diagram, OS root, Hub Rust, DB, replication/bootstrap, store, MCP/plugin-host, WGPU, root manifest, launch configuration, plan, acceptance, lifecycle, goal, ticket, or `AGENTS.md` source was edited.

## Red Evidence

Node `v24.15.0` rejected the production validation entry before module evaluation:

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts'
exit 1
🟦️.ts:22 constructor(source: OwnedUiNodeIndex, private readonly grant: () => NumericIndexGrant)
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
```

The permanent owning-package child oracle was extended before the production repair. Its red Nx run failed before Vitest with the same line 22 diagnostic. A complete census of this 111-line module found this as its only parameter property or other non-erasable strip-only construct.

## Implementation

- `GraphNodes` now declares an explicit private readonly grant field.
- Its constructor accepts an ordinary grant parameter, assigns it first, then performs the pre-existing `source.capture()`. This matches the original parameter-property lowering order and preserves the exact captured-index boundary.
- No node lookup, held-node release, reader release, graph traversal, violation ordering, close order, cancellation state, grant admission, byte accounting, or result transfer behavior changed.
- The existing `@semio-tech/framework` native Node child now also imports the actual validation and owned-node-index modules and reads the existing language-neutral `🔣️owned-validation.json` fixture with `node:fs`.
- Independent `node:assert/strict` assertions execute the fixture's empty graph through the production cursor, prove zero-item grant blocking, exact source capture after the source owner closes, result retirement, validation-owner retirement, and cancellation at every observed step frontier.
- The existing renderer validation test remains the full independent oracle: AJV validates the neutral fixture, Immer/native `Map` builds its reference graph, and `validateUiDocumentCore` supplies expected ordered violations across all eight cases.

## Verification

### Native source

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts'
exit 0

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts", import.meta.url))'
exit 0
```

### Owning package quick gate

```text
bun nx run @semio-tech/framework:test-quick --skip-nx-cache
exit 0
[DEBUG] retained UI native strip-only oracle: 8 fixture laws, 1087 grants, 4 one-time retirements, 1 table cancellation, 7 validation cancellations, 2 sibling-key operations
1 file passed
88 tests passed
0 failed
```

The reported eight laws are four retained-table laws retained from the preceding packet plus four validation-fixture laws. The validation extension executes one neutral empty-graph case through real Node, observes six advancing steps and therefore checks seven cancellation cutoffs (`0..6`), and closes every result/source/cursor owner to terminal emptiness.

### Focused full validation oracle

```text
bun nx run @semio-tech/framework-renderer-react:test-exhaustive --skip-nx-cache -- '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️.tsx' -t 'OwnedValidation preserves graph violation order, captured byte lifetimes and every cancellation frontier'
exit 0
1 file passed
1 selected test passed
209 unrelated tests excluded by the explicit name filter
```

The selected test completed all eight language-neutral vectors and asserted exact ordered violation types for valid, duplicate-key, orphan, cycle, depth, missing-root, node-quota, and empty cases. It also asserted more than 100 cancellation prefixes, source capture after source retirement, final byte-owner release, and zero-grant close blocking. The runner did not surface the debug prefix total, so no more precise prefix count is claimed.

### Hub build frontier

Both aggregate probes advance beyond the repaired validation module. Neither reaches Cargo because the next independent native strip-only source now fails:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts:39
constructor(value: unknown, private readonly surface: UiSurfaceByteView | null = null, raw = false)
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]

bun nx run os-hub:build --skip-nx-cache
exit 1
os-hub-admin:build fails at the same retained-hash parameter property before Cargo is invoked
```

Therefore the sole parameter property in the owned validation module is cleared; no successful admin, aggregate Hub, or Cargo build is claimed.

## Hygiene

- `git diff --check` over both owned source files exits 0.
- The owned validation unsupported-syntax census exits 0 with zero matches.
- No temporary or generated files were created for this packet, so there is no packet-owned generated subtree to remove.
