# Ordered Numeric Native Strip-Only Repair

## Scope

This packet owns only:

- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts`
- this report

It follows the next blocker recorded by `📓️sol-actor-return-strip-only.md`. No actor-return, UI retained contract, Hub Rust, DB, replication/bootstrap, store, MCP/plugin-host, WGPU, root manifest, launch configuration, plan, acceptance, lifecycle, goal, ticket, or `AGENTS.md` source was edited.

## Red Evidence

Node `v24.15.0` rejected both production entry and dynamic import at the first of four parameter properties:

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts'
exit 1
🟦️.ts:131 constructor(private readonly nodes: AllocationNode<V>[]) {}
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]
```

The new owning-package regression was run before the source repair. It failed with the same line 131 diagnostic, producing 0 completed native fixture laws.

## Implementation

- Replaced all four TypeScript parameter-property sites with explicit `private readonly` fields and ordinary constructor parameters.
- Assigned every field at the same leading constructor position as the former parameter-property lowering: `TreeAllocation.nodes`; `TreeEdit.key`, `entry`, and `retirement`; `NumericIndexReader.key`; and `NumericIndexEdit.key`.
- Preserved every existing private state initializer, source capture, `Object.freeze`, tree allocation, reservation, iteration, retirement, ordinal, edit publication, and cancellation path.
- Audited the complete 631-line production module for parameter properties, TypeScript enums/namespaces, import assignments, and abstract classes. The post-change unsupported-syntax census is 0.
- Extended the existing owning `📜️script.ts` test command; no new script or dependency was introduced. Its child process invokes real Node 24 with `--experimental-strip-types`, imports the actual production source, reads the existing language-neutral JSON fixture with `node:fs`, and uses independent `node:assert/strict` plus native `Map` ordering.
- The native oracle covers all 6 fixture cases under both grants, 42 operations, 42 one-step cancellations, exact insertion and numeric-sorted iteration, capture isolation, one-time retirement counts, and one competing winner/loser edit.
- Existing AJV schema validation, Immer plus native `Map` oracle, reference-saturation oracle, strict TypeScript check, ordinal boundaries, exhaustive cancellation-prefix lifecycle, and 3,072-operation stress run remain active.

## Verification

### Native source

```text
node --experimental-strip-types '🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts'
exit 0

node --experimental-strip-types --input-type=module --eval 'await import(new URL("./🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts", import.meta.url))'
exit 0
```

### Owning package

```text
bun nx run @semio-tech/value-numeric-index:test --skip-nx-cache
exit 0
laws=12
lifecycle=165
ordinals=2
stress=3072
references=7
invalidIds=5
nativeLaws=12
nativeOperations=42
nativeCancellations=42
nativeConcurrency=1
strictTS=0 diagnostics
```

The result exercises 12/12 language-neutral case/grant combinations in the existing Bun/Immer oracle and another 12/12 through the real native Node/Map oracle. No case was skipped.

### Hub build frontier

Both aggregate probes advance past ordered numeric value storage. Neither reaches Cargo because the next independent native strip-only failure is now:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts:32
constructor(index: NumericIndex<V>, readonly grant: () => NumericIndexGrant, private readonly retired: (value: V) => void = () => {})
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]

bun nx run os-hub:build --skip-nx-cache
exit 1
os-hub-admin:build fails at the same retained-contract parameter property before Cargo is invoked
```

The ordered-numeric blocker is cleared; no successful admin, aggregate Hub, or Cargo build is claimed.

## Hygiene

- `git diff --check` over both owned source files exits 0.
- Direct unsupported-syntax census exits 0 with 0 matches.
- No temporary or generated ticket files were created, so this packet has no generated subtree to remove.
