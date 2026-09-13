# Bound Router and Literal Environment Grammar Audit

## Status

**Accepted for the two bounded forms.** The independent TypeScript guard now requires the precise import-meta-main predicate, and a hostile new-target-main case proves rejection. The owned lexical parser, TypeScript AST/checker oracle, source inventory, and current registered policy target agree.

## Current Production Forms

| Source | Accepted narrow form | Observed result |
| --- | --- | --- |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts` | A lexically bound `ScriptRouter(...).register(...)` receiver, exactly one `router.run(process.argv.slice(2))` inside an `import.meta.main` guard with no `else`. | Current body fits the owned parser. |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | One intrinsic top-level `process.env.RUST_MIN_STACK ??= "134217728"` before the imported terminal. | Current body fits the owned parser. |

The literal's dedicated Node 24.15.0 and TypeScript 5.9.3 VM controls establish the same four values for the previous arithmetic/call expression and the replacement: absent becomes `134217728`; empty, an explicit override, and `"0"` remain unchanged. This replays only the assignment expression in isolated environments. It does not execute Puzzle, Cargo, or a package router.

## Existing Evidence

- The fixture has 35 fixed-script vectors: 25 new grammar controls plus the prior set.
- Pre-repair historical target: **121 passing, 647 assertions, 0 failures**, with a 48.26 s Bun test body and 48.6 s Nx critical path, cache skipped.
- Post-repair current target: **121 passing, 651 assertions, 0 failures**; 35.18 s Bun test body, 35.6 s target, 35.4 s critical path, cache skipped.
- The route goes through the repository library package router's explicit `package-body-policy` branch, whose 240 s process budget is appropriate to its existing native controls. The native pattern compiler's existing 30 s per-case bound is unchanged.
- Permanent actual-source inventory includes both Print and Puzzle. The policy target declares the relevant library source, fixtures, control/oracle tree, package/project manifests, and Bun/Cargo lock inputs.
- Demonstrator registration and Stdio terminal ownership remain unresolved and are outside this two-form scope.

## Historical Finding: TypeScript Guard Over-Admitted

The owned parser requires the exact `import.meta.main` shape:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:11129` through `:11134` check a `meta` member whose object identifier is `import`.
- Its command-router branch at `:11253` through `:11266` only admits that predicate and rejects an `else`.

The earlier independent TypeScript oracle branch accepted any TypeScript meta-property with a property named main, without requiring import meta. TypeScript represents new target as a meta-property too. It would have accepted:

```ts
if (new.target.main) await router.run(process.argv.slice(2));
```

while the owned parser rejects it. Root reproduced the false admission as 0 passing, 1 failing test, 120 filtered, 143 assertions, 768 ms. The TypeScript branch now requires ImportKeyword plus meta, and vector bound-router-new-target-is-not-import-meta expects unresolved. The post-repair 121/651 target result covers it.

The repair is current; no further admission gap was found in the two bounded forms.

## Limits

This audit does not claim the direct router dispatch executes the Print command or that the Puzzle terminal runs Cargo. It accepts only the two grammar forms after parser parity is repaired. The full policy success establishes the current route and covered controls, not native execution of either product command.
