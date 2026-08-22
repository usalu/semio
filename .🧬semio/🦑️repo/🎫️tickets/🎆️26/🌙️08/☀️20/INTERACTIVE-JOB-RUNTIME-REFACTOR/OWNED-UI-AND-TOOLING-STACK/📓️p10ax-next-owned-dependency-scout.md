# P10ax Next Owned Dependency Scout

## Verdict

Implement **one serialized cleanup packet: remove the unused `@xstate/react` UI-barrel façade.** It removes exactly one live JavaScript identity, taking the live dependency ratchet from **143 total / 80 JavaScript** identities to **142 total / 79 JavaScript** identities, assuming no concurrent manifest change. It is the only evidence-backed next packet that is fully ownable without Cargo and does not require recreating a browser interaction runtime.

Do **not** remove `xstate` in this packet. It remains an active state-machine implementation in CAD and has direct CAD, Puzzle, and UI declarations. Do **not** run a second packet concurrently: every plausible successor edits the same UI manifest and monolithic target barrel, and the interactive candidates have unsolved public-type and browser-behaviour contracts.

This is a read-only scout. It consulted the accepted P10 replacement/audit record through `📓️p10aw-independent-owned-select-audit.md`, the earlier packet scouts (`p10ad`, `p10aq`, and `p10p`), current manifests/source/configuration, and current `bun.lock`. It does not recertify any previously accepted implementation.

## Live Evidence

Commands executed on the shared snapshot:

```text
bun ./📜️script.ts verify dependencies parity js
bun ./📜️script.ts verify dependencies list js --format json
rg --glob '!node_modules/**' --glob '!**/.git/**' --glob '!**/.nx/**' --glob '!**/🎫️tickets/**' '(@xstate/react|useXStateSelector)' .
rg -n '"@xstate/react"|@xstate/react@' bun.lock
```

- The census reports **80 JavaScript identities**. JS parity is clean: `undeclared-imports=0`, `lock-mismatches=0`, `lock-workspaces=44`, and five declared lock fixtures.
- The complete live `@xstate/react` footprint is one UI direct declaration, one UI-barrel re-export, and two lock entries. There is no importing consumer of the owned alias `useXStateSelector` in executable source.
- `bun.lock:519` is the UI workspace edge and `bun.lock:2200` is the sole `@xstate/react@6.1.0` resolution. That resolution brings `use-isomorphic-layout-effect` and `use-sync-external-store`, but those may remain reachable elsewhere; their disappearance must be determined only by lockfile reconciliation.
- The target barrel immediately retains `export { assign, createActor, fromCallback, setup, type ActorRefFrom, type AnyActorRef, type SnapshotFrom } from "xstate";` at `📦️index.tsx:18058`. CAD actively imports `createActor` and `setup` at `✏️s/🔌️plugins/📐️cad/.../🎰️stately/🟦️component.ts:6`; CAD and Puzzle both still declare `xstate`.

The zero-dependency boundary remains global: a clean parity report proves declarations and lock reachability agree, not that remaining identities are owned. This packet is a ratchet, not a claim that the Phase 10 exit condition is met.

## Recommended Packet: Retire The Unused XState React Facade

### Exact changed surface

| File | Required change | Reason |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:18057` | Remove `export { useSelector as useXStateSelector } from "@xstate/react";`. Retain the next `xstate` export. | This is the sole executable use and the entire public facade. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:39` | Remove the `@xstate/react` dependency row. | The direct declaration becomes unused. |
| `bun.lock` | Reconcile through Bun after the manifest edit. | Remove the workspace edge and unreachable resolution; do not hand-edit or claim unrelated transitive removals. |
| This ticket folder | Add an implementation report and an independent audit report. | Preserves the Phase 10 evidence trail. |

The public API change is intentional: the sole external-library-derived export disappears. The no-consumer scan is a precondition, not compatibility support. Once removed, no public API directly or indirectly exposes an `@xstate/react` type or value. Existing `xstate` exports are explicitly out of scope and remain a separate, active dependency boundary.

### Required validation

1. Before editing, repeat a scoped executable-source and consumer scan for both `@xstate/react` and `useXStateSelector`; fail the packet if a real consumer appears.
2. Run `bun install --lockfile-only --ignore-scripts --no-progress --no-summary`, then `bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary`.
3. Run `bun nx run @semio-tech/ui-react:typecheck`, `bun nx run @semio-tech/ui-react:test-quick`, `bun nx run @semio-tech/ui-react:lint`, and `bun nx run @semio-tech/ui-react:check-ui-primitives`.
4. Run `bun ./📜️script.ts verify dependencies`, `bun ./📜️script.ts verify dependencies list js --format json` (expect 79), and `bun ./📜️script.ts verify dependencies parity js` (expect zero undeclared imports and zero lock mismatches).
5. Prove exact absence in executable source/config/manifests and current lock for `@xstate/react` and `useXStateSelector`; run targeted `git diff --check`. Do not run Cargo.

There is no changed DOM or browser event implementation, so no new pointer, keyboard, SSR, hydration, or assistive-technology behaviour is claimed. The only browser residual is the ordinary one for an API-removal packet: consumer absence is statically and test-suite verified, not exercised across every downstream bundle/route.

## Remaining UI Runtime Boundaries: Why They Are Not This Wave

| Identity packet | Real consumers and public leak | Lock relationship | Browser/behaviour risk | Disposition |
| --- | --- | --- | --- | --- |
| `@dnd-kit/core`, `@dnd-kit/sortable`, `@dnd-kit/utilities` | Active Table and Tree leaves; `📦️index.tsx:18039-18042` directly re-exports values and `DragEndEvent`/related types. | UI workspace edges at `bun.lock:511-513`; package resolutions plus accessibility/transitive support remain. | Pointer/touch sensors, keyboard/screen-reader sensors, collision/overlay/cancel semantics, sortable controlled state. | One future atomic 3-identity redesign only; never pair it with this cleanup. |
| `@xyflow/react`, `d3-force`, `dagre` | Diagram and Ports are active. `Diagram` imports graph/layout types; `📦️index.tsx:16`, `:134`, and `:18053` expose XYFlow-derived contracts. | UI edges `:520-522`; XYFlow system and D3/Dagre graphs have transitive lock reachability. | Node/edge selection, connection/collision, viewport transforms, deterministic layout, dragging, and graph accessibility. | Keep as a dedicated graph-runtime packet; P10o owned type ports did not remove the active runtimes. |
| `react-resizable-panels` | Active Resizable and Canvas leaves; their props and imperative handles derive from `ResizablePrimitive`. The barrel leaks it at `📦️index.tsx:18087`. | UI edge `bun.lock:529`, one package resolution at `:3676`. | Pointer capture, split resizing, nested panels, min/max layouts, controlled lag, focus, and SSR measurement. | Requires an owned split-layout interface plus real-browser coverage; not a deletion. |
| `i18next`, `i18next-browser-languagedetector`, `react-i18next` | UI barrel and Label/ShellScope are active; `useTranslation` is re-exported, leaking external hook/types. | UI edges `:523`, `:524`, `:528`; three separate resolutions. | Locale detection, SSR/client divergence, async resources, language fallback, screen-reader language and multi-shell scope. | Treat all three as one i18n-runtime redesign, not a stale row. |
| `react-router` | Active UI barrel import/re-exports at `📦️index.tsx:142` and `:18062`. | UI edge `bun.lock:530`, resolution `:3678`. | Route state, navigation/history, focus/scroll restoration, SSR hydration. | Active routing boundary; no declaration-only removal. |
| `@react-three/drei`, `@react-three/fiber`, `three` | Scene/UI barrel imports and direct re-exports (`:18046-18048`). | UI edges `:514-515`, `:531`, with broad graphics transitive graph. | WebGL lifecycle, render-loop scheduling, pointer raycasting, asset loading and device variation. | Dedicated graphics replacement only. |

`pdfjs-dist` is also still an active PDF runtime rather than proof of a completed removal; P10r replaced its convenience layer, not the underlying renderer. Tooling/platform identities (Bun/Nx/Vite/Vitest/Playwright, TypeScript/ESLint, Storybook/MDX/Tailwind, React/React DOM, Node/VS Code/MCP/database/native image and geometry tooling) are separate build, test, platform, or product-runtime boundaries. They are not safe to delete merely because an ownership-parity scan is clean.

## Serialization And File Disjointness

The recommended packet touches only the UI target barrel, its manifest, lockfile, and reports. Although its leaf code is otherwise disjoint from DnD, graph, i18n, resizable, router, and graphics work, **none is integration-disjoint**: each changes the same `package.json`, `📦️index.tsx`, and `bun.lock`. Integrate P10ax alone, then take a fresh census before selecting a behavioural packet.

No source, config, manifest, lockfile, or Git state was modified by this scout. No Cargo, browser, SSR, hydration, or production-build command was run.
