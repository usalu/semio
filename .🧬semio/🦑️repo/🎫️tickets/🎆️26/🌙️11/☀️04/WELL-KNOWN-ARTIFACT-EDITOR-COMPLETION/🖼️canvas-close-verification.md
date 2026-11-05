# 🖼 Host Canvas Close Integration

Tests call production useCanvas through renderHook and locate its actual Mode element's onWindowClose callback. Guest rendering is excluded; UI and shell modules are not mocked. Only persistence acknowledgment and application retirement ports are controlled.

Four passing scenarios: Keep retains panel/plugin without command note; accepted close retires document attachment before app destruction and panel removal; concurrent sibling closes guard final window and Keep preserves owner; replacement instance survives stale pending close. Neutral JSON expected trace and schema live beside saved-state fixtures; Ajv independently validates lifecycle trace.

Files: engine/🧪️tests/💾️saved-state/🖼️canvas.tsx; engine/🧪️tests/💾️saved-state/🧫️fixtures/🖼️canvas.json (engine under 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine). Ticket config: vitest.canvas-close.config.mjs, jsdom single worker.

Command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.canvas-close.config.mjs

Final result: exit0; 1 file passed; 4 tests passed; duration8.22s; tests125ms. Initial harness missing label-overlay maps was fixed; stale-generation test waits for asynchronous serialized close to reach guard before replacing instance. No product edits performed.
