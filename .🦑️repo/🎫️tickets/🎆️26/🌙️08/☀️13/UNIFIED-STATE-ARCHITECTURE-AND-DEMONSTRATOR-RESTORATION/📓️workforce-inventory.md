# Workforce Inventory — 2026-08-14

## Scope and ticket continuity

The repository MCP resource `repo://goals` was read before continuing work. The existing open
ticket `26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION` exactly covers the request,
so no duplicate ticket was opened. Its goal remains `🎯aioptimizedrepo`.

The app-configured MCP launcher currently fails when the desktop MCP client starts it because its
relative `./📜️script.ts` path is resolved outside the workspace. The repository MCP server itself is
healthy when started from the repository root; its stdio JSON-RPC surface was used directly to read
`repo://goals`, `repo://tickets`, and the MCP tool schemas.

## Current acceptance surface

- 53 plugin app roots exist under `✏️s/🔌️plugins/**/🎛️apps/**/🦀️component.rs`.
- The generated playground registry exposes 58 runnable variants: aggregator, animate, architect,
  aussuchen, bearbeiten, block2d, block3d, block5d, cad, dag, din16798, din18599, din4108, draw,
  en1990–en1999, fem2d, fem3d, flow, forms, generator, gis2d, gis3d, imperative, iso16757,
  koordinator, layout, lowpoly, mathematical, note, playbook, procedural2d, procedural3d, process3d,
  puzzle2d, puzzle3d, puzzle5d, raster, reasoning-wires, remodel, s, sequence, shooting, sourcing,
  trinity-jack, trinity-rewrite, vcs, vdi3805, verfolgen, and writer.
- The canonical `@semio-tech/framework-os-dev:parity` harness can boot each variant in React and
  WGPU, collect runtime/console failures, compare retained structure and pixels, and run behavioral
  probes. Its existing shell probe is insufficient as the sole state proof because it exercises
  framework command-palette chrome rather than an app state transition.
- The quick OS-dev baseline is green: 2 files, 12 tests.

## Live-work coordination

The working tree contains 343 paths changed by concurrent sessions, mostly the open
`FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM` ticket. This workforce therefore owns three disjoint
lanes: shared framework state infrastructure, per-app compile/runtime conformance, and launch/E2E
orchestration. The coordinator owns integration and final all-variant verification. No workforce
member may close this umbrella ticket or use modifying git commands.
