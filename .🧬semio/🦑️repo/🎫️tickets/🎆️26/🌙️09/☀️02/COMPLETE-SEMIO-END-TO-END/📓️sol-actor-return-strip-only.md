# Actor Return Native Strip-Only Repair

## Scope

This packet owns only `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts` and this report. It repairs H3/P0-B from `📓️terra-hub-runtime-failures-audit.md`. No Hub Rust, DB, replication, artifact-bootstrap, store, plugin-host/MCP, WGPU, manifest, plan, acceptance, goal, ticket, or `AGENTS.md` source was edited.

## Red Evidence

Node `v24.15.0` rejected the production source before module evaluation:

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts'
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
🟦️.ts:57 constructor(readonly bytes: Uint8Array, maximum: number)
```

The new focused test was run before the production repair and failed 0/1 with the same Node 24 diagnostic. An initial test draft accidentally invoked Bun through `process.execPath`; the test was corrected to execute `node` from `PATH`, after which it reproduced the real failure.

## Implementation

- `ReturnReader` now declares `readonly bytes: Uint8Array` explicitly.
- Its constructor accepts an ordinary `bytes` parameter, validates it with the unchanged envelope guard, then assigns `this.bytes = bytes`.
- No codec constants, tags, validation, byte writes, reads, framing, or ownership behavior changed.
- The owned file was audited for Node strip-only unsupported parameter properties, TypeScript enums, namespaces, and import assignments. No other unsupported construct remains; direct native entry and import are the authoritative parser checks.
- The in-source actor test suite now launches actual Node in strip-only mode, imports the actual source and page modules, reads the existing language-neutral fixture through Node `fs`, and uses independent `node:assert/strict` plus `Buffer` byte comparisons.
- The native oracle checks all 6 drive vectors, 13 fixed result vectors, and 2 page-result vectors, including encode bytes, decode deep equality, exact page length, padded page bytes, receipt identity, and recovered payload.

## Verification

### Native Source

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts'
exit 0

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts", import.meta.url))'
exit 0
```

### Focused Actor-Return Suite

```text
bun nx run @semio-tech/framework-actor:test-quick --skip-nx-cache -- '../../📤️return/🟦️.ts'
1 file passed
11 tests passed
0 failed
```

The new native child-process regression alone passes 1/1 and covers 21 language-neutral vectors.

### Package Quick

```text
bun nx run @semio-tech/framework-actor:test-quick --skip-nx-cache
8 files passed, 1 file failed
198 tests passed, 1 failed
```

The one failure is outside the owned source in `📤️return/📨️response/🟦️.ts:320`, test `ActorWorkerInboxInventory executes generated heartbeat, ordinary reply and awaited effect traffic together`: expected `error: "Error: post-after-observation"`, received `error: "{}"`. All 11 tests in the owned actor-return source passed. This packet does not modify that response implementation.

### Admin and Aggregate Hub Builds

Both commands advance past the repaired actor-return source, but cannot reach a successful admin build or Cargo because the next independent native strip-only source fails:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts:131
constructor(private readonly nodes: AllocationNode<V>[]) {}
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]

bun nx run os-hub:build --skip-nx-cache
exit 1
os-hub-admin:build fails at the same ordered-numeric parameter property before Cargo is invoked
```

Therefore the original actor-return build blocker is repaired, while aggregate Cargo reachability remains blocked by the separately owned ordered-numeric source. No success is claimed for either build.

## Hygiene

- `git diff --check -- '🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts'` exits 0.
- No temporary or generated files were created for this packet.
