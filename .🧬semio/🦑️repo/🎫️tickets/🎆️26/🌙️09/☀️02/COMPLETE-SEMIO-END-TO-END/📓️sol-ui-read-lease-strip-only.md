# UI Read-Lease Native Strip-Only Repair

## Scope

This packet owns only:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts`
- the existing `@semio-tech/framework` package test router `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- this report

It follows the aggregate frontier recorded by `📓️sol-ui-hash-strip-only.md`. The existing neutral `🔣️read-lease.json` fixture, its schema, and the focused renderer test were reused unchanged. No admin package, retained-root, hash, Diagram, OS root, Hub Rust, DB, replication/bootstrap, store, MCP/plugin-host, WGPU, root manifest, launch configuration, plan, acceptance, lifecycle, goal, ticket, or `AGENTS.md` source was edited.

## Red Evidence

Both Node `v24.15.0` entry paths rejected the 246-line production module before evaluation:

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts'
exit 1

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts", import.meta.url))'
exit 1

🟦️.ts:79 private constructor(mint: object, lease: object, readonly version: number, node: ReadOwner | null)
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
```

Every constructor was inspected. A word-bounded parameter-property census found one real match: public readonly snapshot `version` at line 79. The earlier broad substring scan also printed the ordinary `publication` parameter at line 167 because that identifier begins with `public`; it was not counted as a parameter property. The permanent owning-package child oracle was extended before the production repair, and its red Nx run failed before Vitest with the same line 79 production diagnostic.

## Implementation

- `OwnedUiNodeReadSnapshot` now declares `version` as an explicit public readonly field.
- Its private constructor accepts an ordinary `version` parameter and assigns the field as the first body operation, before exact-mint validation, issued-root construction, and freezing. This preserves the original TypeScript parameter-property lowering order, public visibility, and readonly contract.
- No mint authority, lease identity, version validation, stable snapshot selection, source capture, publication state, acknowledgement, capacity, reader count, node release, cancellation, close sequencing, grant admission, or terminal state changed.
- The permanent framework child process now imports the actual read-lease module under Node strip-only mode and reads all ten laws from the existing neutral `🔣️read-lease.json` fixture with `node:fs`.
- Its independent JavaScript reference lease models staged, cancelled, published, visible, and retiring versions. Production snapshot identity and version transitions are compared against that model.
- Duck-typed retained node owners provide counted captures and independently retiring captured owners. Assertions cover stable repeated reads without recapture, foreign/null acknowledgement rejection, hidden staged snapshots, publication cancellation, publication visibility, old-root retirement before capacity reuse, two-root backpressure, zero-grant preservation, close from every retirement frontier, and terminal emptiness.

## Verification

### Native source

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts'
exit 0

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts", import.meta.url))'
exit 0
```

The repaired source is 247 lines. The exact-module word-bounded constructor parameter-property census returned zero matches.

### Owning package quick gate

```text
bun nx run @semio-tech/framework:test-quick --skip-nx-cache
exit 0
[DEBUG] retained UI native strip-only oracle: 25 fixture laws, 1087 grants, 4 one-time retirements, 1 table cancellation, 7 validation cancellations, 31257 hash bytes in 2453 chunks over 2493 advances, 7 hash cancellations, 6 read cancellations, 16 read captures/releases, 5 cancel turns, 5 publish-retirement turns, 6 final read close turns
1 file passed
88 tests passed
0 failed
```

The 25 laws are four retained-table laws, four retained-validation laws, seven retained-hash laws, and all ten retained-read-lease laws. The one native read case and six cancellation-frontier cases perform 16 captures and exactly 16 releases; zero captured owner remains live.

### Focused full read-lease oracle

```text
bun nx run @semio-tech/framework-renderer-react:test-exhaustive --skip-nx-cache -- '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️.tsx' -t 'ReadLease binds exact consumer commits, keeps stable snapshots and backpressures until retired roots are empty'
exit 0
1 file passed
1 selected test passed
209 unrelated tests skipped by the explicit name filter
```

This existing selected test validates the neutral fixture with AJV, uses the typed node decoder and framework encoder for retained byte owners, compares an independent immutable version reference, and checks stable snapshot identity, exact/foreign/stale acknowledgements, two-root backpressure, zero-grant preservation, retirement-before-reuse, independent subscribers, and byte invalidation only after the last owner closes.

### Aggregate build frontier

Both aggregate builds now advance beyond the repaired read-lease source. Their first shared failure is no longer a native TypeScript strip-only diagnostic:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
vite v7.3.6 building client environment for production...
0 modules transformed
[vite:build-html] Failed to resolve ./🟦️.tsx from 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🌐️.html

bun nx run os-hub:build --skip-nx-cache
exit 1
its os-hub-admin:build dependency fails at the same unresolved HTML entry
```

Read-only inspection confirms `🌐️.html` references `<script type="module" src="./🟦️.tsx">`, while that package currently contains `🟦️.ts` and `📦️index.tsx` but no `🟦️.tsx`. This admin-package state is outside this packet and was not changed. Because Vite stops at HTML entry resolution before transforming any modules, no later strip-only blocker and no Cargo result is claimed.

## Hygiene

- `git diff --check` over the owned production module and cumulative permanent test router exits 0.
- The exact-module parameter-property census has zero matches.
- No temporary or generated files were created for this packet, so there is no packet-owned generated subtree to remove.
