# UI Hash Native Strip-Only Repair

## Scope

This packet owns only:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts`
- the existing `@semio-tech/framework` package test router `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- this report

It follows the next blocker recorded by `📓️sol-ui-validation-strip-only.md`. The existing neutral `🔣️owned-hash.json` fixture and focused renderer test were reused unchanged. No read-lease, retained-root, Diagram, OS root, Hub Rust, DB, replication/bootstrap, store, MCP/plugin-host, WGPU, root manifest, launch configuration, plan, acceptance, lifecycle, goal, ticket, or `AGENTS.md` source was edited.

## Red Evidence

Node `v24.15.0` rejected the production hash entry before module evaluation:

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts'
exit 1
🟦️.ts:39 constructor(value: unknown, private readonly surface: UiSurfaceByteView | null = null, raw = false)
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
```

The permanent owning-package child oracle was extended before the production repair. After correcting an internal child-program string quotation in that test harness, its red Nx run failed before Vitest with the same line 39 production diagnostic. A complete census of this 192-line module found that constructor parameter property as the sole matching unsupported parameter-property form.

## Implementation

- `JsonBytes` now declares an explicit private readonly `surface` field.
- Its constructor accepts an ordinary `surface` parameter with the unchanged `null` default, assigns that field first, then performs the pre-existing stack initialization. This matches parameter-property lowering order and preserves raw/value frame selection and surface-view identity.
- No JSON frame order, byte encoding, grant admission, chunk limit, FNV update order, captured-source ownership, node release, reader release, cancellation, result transfer, or retirement behavior changed.
- The permanent `@semio-tech/framework` native Node child now also imports the actual hash cursor and reads the existing language-neutral `🔣️owned-hash.json` fixture with `node:fs`.
- The child constructs the fixture's 8,193-byte surface as a random-access byte view and its depth-80 JSON envelope, then independently derives the reference bytes with Node `JSON.stringify`/`Buffer`.
- Production bytes are checked one by one as emitted and are never concatenated or retained as an aggregate. Each production chunk incrementally updates both the expected FNV-1a result and Node `crypto` SHA-256; the latter is compared to Node's reference digest.
- The oracle proves zero-item grant blocking, the 256-byte maximum chunk, exact byte count/order, exact final FNV/revision identity, one-time result transfer, seven distinct cancellation frontiers, captured-index closure, node-owner closure, unchanged source identity, and terminal empty close state.

## Verification

### Native source

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts'
exit 0

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts", import.meta.url))'
exit 0
```

The owned-module parameter-property census returned zero matches.

### Owning package quick gate

```text
bun nx run @semio-tech/framework:test-quick --skip-nx-cache
exit 0
[DEBUG] retained UI native strip-only oracle: 15 fixture laws, 1087 grants, 4 one-time retirements, 1 table cancellation, 7 validation cancellations, 31257 hash bytes in 2453 chunks over 2493 advances, 7 hash cancellations, 3 final hash close steps
1 file passed
88 tests passed
0 failed
```

The 15 fixture laws are four retained-table laws, four retained-validation laws, and all seven retained-hash laws. The hash packet exercised one native hash case over 31,257 exact bytes. The reported three close steps are the close of the successfully completed hash cursor; every cancellation cursor was independently closed to terminal emptiness as well.

### Focused full hash oracle

```text
bun nx run @semio-tech/framework-renderer-react:test-exhaustive --skip-nx-cache -- '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️.tsx' -t 'OwnedHash streams exact insertion-ordered JSON bytes while retaining old surface owners through cancellation'
exit 0
1 file passed
1 selected test passed
209 unrelated tests skipped by the explicit name filter
```

This existing selected test validates the neutral fixture with AJV, compares the production byte stream and FNV result with Node `Buffer`/`JSON.stringify`, and checks source capture, old surface-byte ownership, zero-grant behavior, and every observed cancellation frontier.

### Hub build frontier

Both aggregate probes advance beyond the repaired hash module. Both stop at the same next independent native strip-only source:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts:79
private constructor(mint: object, lease: object, readonly version: number, node: ReadOwner | null)
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]

bun nx run os-hub:build --skip-nx-cache
exit 1
os-hub-admin:build fails at the same retained read-lease line 79 before Cargo is invoked
```

Therefore the sole unsupported constructor parameter property in the owned hash module is cleared. No successful admin, aggregate Hub, or Cargo build is claimed.

## Hygiene

- The scoped diff contains only the explicit hash field/assignment, the cumulative permanent native retained-UI oracle extension, and this report.
- The owned hash unsupported-syntax census has zero matches.
- No temporary or generated files were created for this packet, so there is no packet-owned generated subtree to remove.
