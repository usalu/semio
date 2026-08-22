# P10ba Next Live Dependency Scout

## Verdict

**Implement one serialized owned replacement packet for `d3-force`.** There is no further safe declaration-only removal after P10az: the current 79-identity JavaScript census and clean parity result still contain active runtime, configuration, script, type-platform, or lock-shared boundaries. `d3-force` is the smallest genuine replacement because it has exactly one direct manifest user, one executable runtime leaf, no dynamic/config/script consumer, no third-party public type leak, and a closed consumed API.

The packet removes exactly **one counted JavaScript identity**, taking the dependency ratchet from **142 total / 79 JavaScript / 63 Rust** to **141 total / 78 JavaScript / 63 Rust** identities, assuming no concurrent manifest change. It is deliberately not a replacement of XYFlow, Dagre, DnD, or the Diagram public host.

Read-only evidence in this snapshot:

```text
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies parity js
rg executable/config/script/type scans for d3-force, dagre, and prior low-evidence candidates
rg bun.lock and the Diagram leaf/public barrel for direct and transitive reachability
```

JS parity is clean at `83 manifests`, `264 external rows`, `115 evidenced rows`, `149 advisory unowned rows`, zero undeclared imports, zero lock mismatches, five fixtures, and 44 lock workspaces. The advisory “no-owned-scope-evidence” marker for `d3-force` is not deletion authority: its package manifest is nested below the actual Diagram leaf, which is exactly why the direct runtime scan is required.

## Why D3 Force Is The Next Bounded Packet

### Complete live footprint

| Surface | Evidence | Disposition |
| --- | --- | --- |
| Direct manifest | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:40` | Remove after the owned replacement. |
| Runtime import | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx:14` imports `d3-force`; the complete adapter occupies lines 172–215. | Replace. |
| Type declaration | `🧱️elements/📊️Diagram/🟦️implementation.d.ts:8-15` declares only the untyped D3 module. | Remove only this declaration block; retain the Dagre block. |
| Consumers | `createDiagramForceSimulation` is used only by Diagram's own force effect/drag callbacks and by its in-source focused test at the UI target barrel. `DiagramProps.forceConfig` has no external repository consumer. | Preserve the owned contract, not D3. |
| Public API / types | The target barrel exports the owned `DiagramForceConfig`, `defaultDiagramForceConfig`, and Diagram surface; it does **not** export `createDiagramForceSimulation` or any D3 type/value. The leaf's `DiagramForceNode`, `DiagramForceLink`, and `DiagramForceSimulation` are structural repository types. | No D3 type leak to migrate. |
| Dynamic/config/script | Exhaustive source/config/script scan finds no `import("d3-force")`, `require`, Vite alias/externalization, CLI invocation, or source string beyond the leaf import/declaration and historical `🔒️dependencies.json`. | No additional migration path. |

`d3-force@3.0.0` has one UI workspace edge at `bun.lock:520` and one resolution at `bun.lock:2523`. Its `d3-quadtree` transitive resolution is solely reachable from that identity and should disappear after Bun reconciliation. `d3-dispatch` and `d3-timer` must remain: XYFlow's active `@xyflow/system` graph reaches them through `d3-zoom`/`d3-transition`. Do not hand-edit or overclaim those shared resolutions.

### Exact owned replacement

Retain the current owned configuration and simulation interfaces, but replace the D3 adapter with an `OwnedDiagramForceSimulation` behind the existing `createDiagramForceSimulation` factory. Its deliberately finite consumed surface is:

- `alphaDecay`, `alphaMin`, `alphaTarget`, `nodes`, `on("tick")`, `restart`, `stop`, and `tick`;
- mutable `x`, `y`, `vx`, `vy`, `fx`, and `fy` on owned force nodes;
- string-or-node link endpoint resolution, link-distance attraction, charge repulsion, collision separation, X/Y centre springs, pinned-node behavior, and finite-value recovery;
- deterministic tick order and initial fallback coordinates, with no random/global external state;
- document/window guards plus cancellable browser scheduling for `restart`, bounded to a frame budget. `stop` and effect cleanup must cancel the pending frame and suppress later tick callbacks. The existing effect may keep its synchronous initial settling loop only if it is bounded; a cooperative frame-batched settle is preferred for larger graphs and must not freeze controlled updates.

This is a repository-owned simulation subset, not a general D3 compatibility layer. Preserve `DiagramProps.forceConfig` and its default values; do not introduce a D3-shaped export or new third-party contract. The existing `updateIntervalMs` public option is currently not consumed by the adapter. Either give it an explicit owned notification-throttling meaning with tests, or leave it inert and document that it remains outside the implemented subset—do not silently change it.

## Required Files And Serialization

Implementation must be limited to:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx` — remove D3 import/adapter, add the owned simulation implementation while preserving Diagram interaction contracts.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️implementation.d.ts` — remove only the D3 declaration region.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` — remove `d3-force`.
- `bun.lock` — Bun-reconciled only.
- A new focused Diagram force test or the existing dedicated in-source Diagram test region, plus packet/audit reports and regenerated dependency-parity evidence.

Do not touch the Dagre implementation, `@xyflow/react` adapter/exports, compose's Dagre Vite stub/external configuration, DnD, i18n, router, resizable, graphics, PDF, or Cargo. Dagre shares the Diagram leaf, UI manifest, `implementation.d.ts`, and lockfile, so its future packet cannot integrate concurrently with this one.

## Behaviour And Browser Risk

Although the dependency is private, its force loop participates in Diagram node dragging. The owned packet must prove:

- deterministic finite positions for identical node/link input, isolated/overlapping nodes, and zero-distance links;
- configured charge, link distance, collision, and centering each affect a bounded fixture;
- fixed `fx`/`fy` nodes never drift; multi-select pinning, drag start/restart, drag movement, and drag stop/unpin retain the current callback/order semantics;
- uncontrolled updates and controlled `onNodesChange` proposals remain distinct, including controlled lag;
- `restart` invokes only subscribed tick listeners, cleanup/`stop` cancels queued animation work, and SSR/static render does not access browser globals;
- no stale frame mutates an unmounted Diagram, and a large graph settles cooperatively within the project interaction budget rather than blocking a frame.

JSDOM can cover contracts and fake-frame cancellation, but it cannot certify real pointer capture, browser animation cadence, or visual force quality. A real-browser drag/cancellation check is a residual before claiming those properties closed.

## Required Gates

1. Focused Diagram force tests covering the matrix above, including fake timers/animation frames and a static-render SSR guard.
2. `bun nx run @semio-tech/ui-react:typecheck`, `test-quick`, `lint`, and `check-ui-primitives`.
3. `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` for the real NodeGraph Diagram consumer; renderer lint. Do not use renderer-wide typecheck as a packet gate while its independently documented graphics/shell/WASM baseline remains red; scan its diagnostics for the changed Diagram file and removal identity.
4. `bun install --lockfile-only --ignore-scripts --no-progress --no-summary`, then the frozen-lock equivalent.
5. Dependency freeze/list/parity and manifest/source regeneration: expect 78 JavaScript / 141 total identities, zero undeclared imports, and zero lock mismatches.
6. Exact executable/config/manifest scans for `d3-force`, `forceImplementation`, and `DiagramForceImplementation`; exact lock scan for `d3-force` and `d3-quadtree`; confirm `d3-dispatch` and `d3-timer` remain XYFlow-reachable. Finish with focused format and `git diff --check`. No Cargo.

## Rejected Alternatives

| Candidate | Why not next |
| --- | --- |
| `dagre` | Active directed-layout implementation with an external type declaration and compose Vite stub/externalization users. It also shares every integration file with D3 Force. It needs a separate rank/layout/cycle/RTL replacement packet. |
| XYFlow, DnD, i18n, router, resizable, Three/Fiber/Drei, PDF.js, `xstate` | Active runtime/public-type/browser boundaries; no declaration-only deletion or complete bounded owned replacement was established here. |
| `@mdx-js/rollup`, `@tailwindcss/typography`, `@nxlv/python`, `@bytecodealliance/jco`, `binaryen`, `dependency-cruiser`, coverage tooling | Exact scans locate active Vite/PostCSS/Nx/CLI/script use. The no-evidence metric cannot override those config/script consumers. |
| React/Node/Three/VS Code type packages, build/test platforms | Direct platform/type boundaries, often multi-manifest and not stale implementation facades. |
| Previously retired P10 identities | Scans find only `🔒️dependencies.json` history, except `postcss-load-config`, which remains active in compose PostCSS configs and manifests. They must not be scheduled again. |

No source/config/manifest/lockfile edit, test, Cargo command, or browser run was performed by this scout.
