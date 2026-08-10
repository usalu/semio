# Puzzle 3D Mesh Loading

## Scope

Continuation of ticket `26/08/07/GET-ALL-APPS-WORKING-END-TO-END`, associated with goal `R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS`.

The repo MCP is not registered in this Codex session. The existing open ticket and goal were validated from their checked-in JSON metadata.

## Reproduction

The registered `Puzzle 3D · React` launch command resolves to `bun run dev:puzzle:3d` and serves the app on `http://127.0.0.1:6013/`.

After the Concrete Forest scene mounts, both World 3D windows enter `ShellFaultBoundary` with:

```text
Could not load /mesh/🧊️hexagonal-cut-concrete-forest-left.glb: Unexpected token '<', "<!doctype "... is not valid JSON
```

The failing request returned `200 text/html` and began with `<!doctype html>` instead of the GLB magic bytes `glTF`.

## Root Cause

Four `mesh-collection` metadata declarations used the nonexistent root `♻️mit-bestand/🖼️assets/🏚️abbau-aufbau`. The checked-in directory is `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau`.

Vite's mesh middleware found no file and called `next()`. The SPA fallback then returned the HTML shell with status 200, and Three's GLTF loader attempted to parse it as glTF JSON.

## Fix

- Correct the root in Puzzle, Block, Shooting, and Demonstrator manifests so every consumer of the shared Concrete Forest GLBs stays consistent.
- Extend plugin-registry validation to reject missing `static-dir` roots, `mesh-collection` roots, and mesh placeholders before catalogs and launch files are accepted.
- Regenerate the derived playground catalog and launch configuration.
- Restore Puzzle 3D's transitional host-to-command bridge for every real declared action, including `setActiveExample` and `registerBrushMesh`; remove the unused `setJackQuery` declaration because it had no implementation or caller.
- Pass the materialization context into release WASM optimization so production builds can resolve their workspace tools.
- Run the Vite production build through Bun and propagate its exit status instead of invoking TypeScript through Node's strip-only loader and silently accepting failures.

## Verification

- `bun nx run @semio-tech/ui-styling:test`: 25/25 tests passed, including all three mesh-collection middleware cases.
- Dev GLB request: `200`, `Content-Type: model/gltf-binary`, 86,112 bytes, and `glTF` magic bytes instead of the HTML shell.
- Dev browser run at `http://127.0.0.1:6213/`: Concrete Forest rendered in both Top and Perspective; switching to Nakagin Capsule Tower remained alert-free after five seconds. No `setActiveExample` or `registerBrushMesh` rejection remained.
- `SEMIO_WASM_OPT=0 bun nx run @semio-tech/framework-os-dev:build --excludeTaskDependencies -- puzzle3d`: passed; Vite transformed 2,751 modules and emitted the release bundle.
- Production preview at `http://127.0.0.1:6313/`: opened Concrete Forest with Top and Perspective present, switched to Nakagin Capsule Tower, and remained alert-free after five seconds. Logs contained no error-level entries; only existing debug warnings for shell-owned `setContributions` and `setAppRegistrations` pushes.
- `SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/puzzle-plugin:test-quick`: the first expanded run exposed the unused, unbridged `setJackQuery` declaration and that declaration was removed. The retry could not complete because a concurrent task deleted `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🔺️diff/🦀️component.rs` while Cargo was compiling; Cargo stopped on that missing, unrelated Puzzle 2D source.

## Repository-Level Constraints

- The aggregate `@semio-tech/framework-os-dev:build` dependency graph is independently blocked before Puzzle by Storybook's unresolved `@semio-tech/coda-desktop/renderer`; the direct Puzzle target with dependencies excluded passes.
- `bun nx run @semio-tech/plugin-registry:check` reaches an unrelated pre-existing registry-script failure: `TAXONOMY_SNAPSHOT_CHILD_DIRS is not defined`.
- The repo MCP is unavailable in this session, so the existing broad ticket could not be closed through `ticket_close`; it also still covers other apps.
