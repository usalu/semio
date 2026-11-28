# 🧭 Browser e2e plan (bim dev playground)

1. Start `bim-react` (launch.json), port 6302. Open `http://localhost:6302/?example=house`. The `?example=` param comes from ticket `26/10/08/BIM-PLUGIN/bim-react-preflight.ts` and is not yet confirmed against the shell.
2. Ready: `[data-slot="surface-role-chip"][data-app-id="s.bim.model@1/*#editor"][data-role="editor"]` (≤180 s) plus a `canvas` (≤120 s), with no fault text. The locale is locked to `en`.
3. Guest loads: 200 for `/🏬️bim/🔣️.json`, `/🛂️.descriptor.semio`, `/🌉️bridge.js`, `/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js`. Failure signs: 404 "Diagnostic browser module is unavailable", pageerror, console error, "Optional engine was actually imported without a publication".
4. Windows: Plan, Section, Sheet, 3D, Schedule, Family editor (de: Grundriss, Schnitt, Blatt, Mengen, Familieneditor).
5. Export: "measures" button → "Export" → "IFC4 (STEP)". Progress shows "Analysing the model" then "Writing the file". Output `Family-House.ifc`: `ISO-10303-21;`, `FILE_SCHEMA(('IFC4'))`, 30 walls, 15 slabs (4 `.FLOOR.`).
6. Examples: demo, house, office. `checks` has only a `.rs` file, so check whether it can be loaded.
7. Reference script: `26/10/08/BIM-PLUGIN/bim-react-full-shell.ts` (Vite on 16302, `launchBrowserV1`).
