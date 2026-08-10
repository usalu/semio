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

## Verification

Pending.
