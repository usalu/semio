# P10ay Remove Unused XState React

## Verdict

**AUDIT-READY for the bounded P10ax packet.** The unused UI-barrel `@xstate/react` facade, its sole live manifest declaration, UI lock snapshot edge, and package resolution are removed. The repository retains active `xstate` declarations, the UI's explicit `xstate` export surface, and the live `xstate@5.32.5` resolution unchanged.

The dependency ratchet is at the expected **142 total / 79 JavaScript / 63 Rust identities**. Every bounded source, UI, renderer-consumer, lint, primitive-policy, formatting, lock, dependency, parity, manifest-source, exact-scan, and diff gate passes.

No Cargo command, modifying Git command, ticket metadata mutation, DnD edit, graph edit, i18n edit, router edit, resizable edit, graphics edit, PDF edit, or unrelated consumer edit was made.

## Precondition And Surface

The pre-edit executable-source scan found exactly one use of `@xstate/react` and `useXStateSelector`:

```ts
export { useSelector as useXStateSelector } from "@xstate/react";
```

There was no importing consumer of that alias. The complete live identity footprint was the export, the UI target manifest row, the UI workspace lock edge, and the single package resolution. The public API removal is intentional; there is no compatibility alias or external-library-derived public type left behind.

## Changes

- Removed `useSelector as useXStateSelector` from the UI React target barrel.
- Removed the exact `@xstate/react` dependency row from the UI React target manifest.
- Reconciled `bun.lock` through Bun with lifecycle scripts disabled. Bun removed the UI workspace edge and the unreachable `@xstate/react@6.1.0` resolution.
- Regenerated the existing manifest/source parity JSON and Markdown evidence.

`use-isomorphic-layout-effect` and `use-sync-external-store` remain lock-reachable through other active packages. They were not hand-edited or treated as part of this packet.

## XState Retention Proof

The active boundary deliberately remains:

- UI manifest: `"xstate": "^5.25.0"`.
- UI barrel: explicit `assign`, `createActor`, `fromCallback`, `setup`, `ActorRefFrom`, `AnyActorRef`, and `SnapshotFrom` exports from `xstate`.
- Lock: UI `xstate` workspace edge and `xstate@5.32.5` resolution.
- Other active declarations and CAD/Puzzle consumers were not changed.

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `bun.lock`
- regenerated `📊️p10-manifest-source-parity.json`
- regenerated `📓️p10-manifest-source-parity.md`
- this implementation report

The shared barrel, manifest, lock, and parity artifacts also contain concurrent Phase 10 work; this packet preserved it.

## Final Gates

| Gate | Result |
| --- | --- |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary` | PASS — Bun reconciliation; lifecycle scripts disabled |
| `bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS — frozen lock; lifecycle scripts disabled |
| UI React `test-quick` | PASS — 19 files, 672 tests |
| UI React `typecheck` | PASS |
| UI React `lint` | PASS; only Bun's existing color-environment warning |
| UI primitive policy | PASS — 0 violations, 2 existing allowlisted files |
| Renderer React `test-quick` | PASS — 4 files, 439 tests |
| Renderer React lint | PASS — region/host-contract lint |
| Exact changed-file Nx format check | PASS |
| Dependency freeze | PASS — historical 238, current 142, removed 96, no additions |
| JavaScript dependency list | PASS — 79 identities |
| Rust dependency list | PASS — 63 identities; no Cargo command run |
| JavaScript dependency parity | PASS — 83 manifests, 264 external rows, 115 evidenced rows, 149 advisory unowned rows, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |
| Manifest/source audit regeneration | PASS — 64 manifests, 576 direct rows, 264 external rows, 75 no-package-scope-evidence candidates |
| Exact executable/config/manifest scan for `@xstate/react` and `useXStateSelector` | PASS — zero matches |
| Exact `bun.lock` scan for `@xstate/react` and `useXStateSelector` | PASS — zero matches |
| Explicit active `xstate` manifest/export/lock scan | PASS — retained |
| Targeted `git diff --check` | PASS |

The historical `🔒️dependencies.json` ratchet deliberately retains `@xstate/react` as a permitted removal. It is neither a live manifest nor a public API/type leak. Ticket research and reports also retain the identity as historical evidence.

## Additional Diagnostic And Residual

An additional renderer-wide `typecheck`, beyond the scout's bounded renderer consumer-test requirement, was attempted. It remains red on extant errors in excluded graphics, surface/WASM host, shell, and worker paths. No diagnostic referenced `@xstate/react`, `useXStateSelector`, either changed source file, or this removal. Repairing that unrelated product-wide baseline would violate this packet's explicit scope, so no excluded source was changed.

There is no changed DOM, event, pointer, keyboard, SSR, hydration, or assistive-technology runtime. Consumer absence is proven by the exact static scan, UI typecheck, complete quick UI suite, and renderer consumer suite; it was not separately exercised across every downstream production route or bundle.
