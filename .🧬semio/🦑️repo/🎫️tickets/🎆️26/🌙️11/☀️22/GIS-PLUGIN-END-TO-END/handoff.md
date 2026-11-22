# GIS Plugin End to End: Handoff Prompt

Continue ticket `2026/11/22/GIS-PLUGIN-END-TO-END`: "Get the gis plugin working end to end." Reopen it with `ticket_reopen`, or use the `repo` CLI if the repo MCP tools time out with "PreToolUse hook did not respond". Follow `CLAUDE.md`.

Ticket folder: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/`. Read these reports there first:
- `open-gaps-audit.md`: the prioritized feature gaps.
- `planning-latency.md`, `planner-closure-fix.md` and `registry-follow-check.md`: planner diagnosis and fixes.
- `gis-plugin-and-services.md`, `hub-gis-gates.md`, `gismap-rust.md`, `gisterrain-rust.md` and `surface-wasm-and-gis3d-dev.md`: earlier state.

## Done so far
- Restored deleted tsv and csv receiving fixtures, the sqlite snapshot `🫴️receiving` fixtures under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📑️tsv,📊️csv}`.
- Added `cad-geometry-bindings` and `spatial-kernel-session-bindings` generatorContracts in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.
- Fixed the stdio-mp3 package:
  - `📜️script.ts` now runs `runScriptProgram()`.
  - Added `bundleRuntime:"runtime"`.
  - Added `metadata.semio.artifactPackage` in `📋️project.json`.
  - Deleted a stray `.d.ts`.
- An agent fixed dist-package metadata and self-imports: dag, demonstrator-playground, procedural-generation2d, stdio-js/obj/stl, trinity-*, writer, lowpoly, reasoning-wires and space-*.
- The spatial-kernel session package `📜️script.ts` loads its test laws lazily, so the `wasm` task no longer consumes its own bindings.
- Planner (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/`):
  - Stable owner failures now throw at once.
  - Owners reached only through file edges are admitted as ownership-only.
  - Registry-pending native-owner faults go through registry adoption and are never treated as stable failures.
  - Unit tests pass: owner-isolation 21/21, registry-follow 8/8, admission 10/10.
- Rust (via `repo cargo --lang=en --store=shared --configuration=prototype -- test -p <crate> --lib`):
  - The gisterrain library tests pass 21/21.
  - gismap test compile errors were fixed (call sites, a `Box::pin`, the label asset install).
  - Not yet confirmed: the final gismap rerun, and `vector_label_anchor_uses_line_midpoint_for_transportation_names` in the mvt-decoder crate. The agent was halted mid-run and wrote no `gis-rust-crate-tests.md`.

## Next steps
1. Rerun the gismap crate tests and the label-anchor test through `repo cargo`, one at a time because the host has 16 GB. Fix failures at their root cause.
2. Rerun the GIS test sweep in the background:
   `repo run-many --lang=en --targets=test --projects=@semio-tech/gis-js,@semio-tech/gis-gismap-js,@semio-tech/gis-gismap-rs,@semio-tech/gis-gisterrain,@semio-tech/gis-gisterrain-rs,@semio-tech/gis-plugin --output=static`
   - Planning has been taking 1–5 hours.
   - Failures name a reference id; look it up in the resident service log `/var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-*.log`.
   - Earlier failures came from other agents' half-created owners or files mid-edit: the process route table, the HubFirstRun wgpu stub, derived-replay.
3. Run the gates:
   - Targets: native-build, mcp-build, user-path-check, security-check, native-codec-check, component-codec-check, gis-map-proposal-process-check, trusted-stdio-gis-bundle-process-check, trusted-stdio-gis-bundle-browser-check, typecheck, wasm.
   - Projects: `@semio-tech/s-gis-services-native`, `@semio-tech/gis-plugin`, `os-hub`, `@semio-tech/s-dev-composition`, `@semio-tech/framework-surface-rs`.
4. Verify end to end:
   - `repo run os-hub:dev`.
   - The s shell (`repo run @semio-tech/framework-os-dev:dev-gis3d-react-prototype`, HTTP 200 on 6070).
   - gis2d (6040/6140) and gis3d (6083/6183).
   - Browser acceptance with `🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/verify-s-shell-terrain.mjs`.
   - Then the gaps in `open-gaps-audit.md`: terrain brush and archive reopen, the 2D envelope gates, the 65×65 grid, LOD and labels.
5. Remaining planner speedup idea from `planning-latency.md`: stop implementation-stamp changes outside the requested closure from invalidating owner records.
6. Finish:
   - Delete `🗑️generated/`.
   - `ticket_close` with a summary and every file touched: the files above plus the agents' changes listed in the reports.
