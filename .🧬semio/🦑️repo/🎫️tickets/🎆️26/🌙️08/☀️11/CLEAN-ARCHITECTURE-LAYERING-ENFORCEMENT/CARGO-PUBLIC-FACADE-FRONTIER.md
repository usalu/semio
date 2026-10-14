# Cargo Public Facade Frontier

Bounded static next-frontier review only. No production/schema/catalog edits, imports, rules/probes/tests or native execution. This does not reopen the accepted 32-file metadata lane or assert a full dependency-direction check passed.

## Existing Enforceable Policy

Taxonomy `dependencyDirections.publicApi` defines prefix `no-cross-package-relative-`, SDK and no tooling-role exceptions. Direction construction `🏗️construction/🟨️.cjs:258–278` groups package semantic source owners and forbids imports from outside the closest owner into any target except actual authored package export targets. The rule explicitly includes scripts, types and literal dynamic imports. Direction `🟦️.ts:42–60` includes these rules in strict public-API enforcement. Therefore cross-owner private General imports below violate a present path rule; they are not merely style preferences. No rule runner was executed.

## Exact Current Imports and Existing Equivalents

Repo endpoint `library/🏃️process/🦀️cargo/🏃️execution/🟦️.ts`:

| Line | Current General Target | Existing Public Equivalent |
|---|---|---|
| 6 | Process package facade `🧪️testing/🦀️execution/🟦️.ts` | Physical facade exists, but Process package.json has no export key targeting it. No existing authored package export equivalent; do not pretend an alias exists. |
| 7 | Process private `🧭️routing/🟦️.ts` | Process `./routing` facade exports checkScriptInvocation and ScriptInvocation. |
| 8 | Process private `🧪️testing/🦀️cargo/🟦️.ts` | Process `./cargo-test` exports the reader and requested policy/request types. |
| 9 | Process private `🧪️testing/🎚️budget/🟦️.ts` | Process `./test-budget` exports resolveTestLevel. |
| 10 | Process private `🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` | Process `./cargo-laws` exports configuredExactCargoLawPolicyV1. |
| 11 | Schema validator private `✅️validator/🟦️.ts` | `@semio-tech/schema-validator-ts` authored root facade exports validateJsonSchemaSubset. |

Native capability reader `library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts`:

- Line2 directly imports General execution schema JSON. The canonical General schema is its correct domain owner and has a registered canonical ID/index; there is no authored TS package export for this JSON asset. Schema ownership is sound, while the current cross-owner JSON import is still outside authored package exports. A future schema asset/public contract decision is required rather than inventing a facade alias.
- Line3 imports CargoExecutionBackendV1 directly from General implementation. The physical execution facade already reexports it, but that facade itself lacks a package export as noted above.
- Line4 imports private validator implementation despite the existing root validator facade.
- Its other Process type/runtime imports shown at lines8–10 already target existing public routing/leases/execution facades.

New General execution implementation line2 imports validator private implementation across a different General semantic source owner; the existing validator root facade provides the same function. Its Process-internal routing/owned-execution imports stay inside the same Process semantic owner, so they are not violations of this cross-owner rule. The new execution facade's own reexport into its defining implementation likewise stays inside Process and is legitimate.

## Public Type and Asset Boundaries

The new execution facade explicitly reexports first-party request/backend/source/declaration/contracts/command/continuation types. These types contain only first-party interfaces and language primitives; no external library type is exposed through them. Its execute function references first-party ScriptInvocation and runOwnedCommand types; those existing APIs expose system/framework ports, not a third-party runtime dependency. The native capability public interface references the first-party backend type rather than exporting a foreign class. No external-runtime-library API leak was found in these bounded declarations.

The canonical General schema owns request/backend/declaration/continuation shapes. Specific native capabilities reference it by canonical schema ID and register its document with the first-party validator. This does not transfer schema authority to fixtures or Specific code. Existing canonical catalog/index ownership remains accepted; the issue here is package import/export enforcement, not schema identity.

## Current Hashes

- Process package.json: `09ecdb9c54ac496f7d00c220e24b57c9ef845271a76a5fc980a92fec85f18387`.
- General execution implementation: `e6006fda02353d74ac97a979736f7257feaef00612f999f93bbb9992dde13e9a`.
- New execution facade: `a1ffd4c94a562975b07b7cf7638bd388673ab89758a5c7bc98380cc598d398a7`.
- Repo endpoint: `766b399ee9e56a03bd4e55a021f55e6ae516757c5b48dd63a432913cf9e77d33`.
- Native capability reader: `9e34c36b622304e21e332eb0c1628476903a38f8dd0a086c37522c0df30198c8`.
- Enforcement construction: `c8b8c6148d7b130dd8c2362da74d2180b180177a213932b09921890af6d82c4b`.

Root received the enforceable frontier. No aliases, raw Cargo path, compatibility layer or metadata reopening is proposed; actual implementation remains separately assignable work.
