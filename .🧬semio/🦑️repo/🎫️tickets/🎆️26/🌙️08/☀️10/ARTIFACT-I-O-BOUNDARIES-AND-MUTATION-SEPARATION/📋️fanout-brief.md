# Artifact Io Facets — Fan-out Brief

Read first: `📜️normative-spec.md` and `🧪owner-table.json` in this ticket folder.

## Absolute rules

1. Touch **only** your assigned plugin's files (and that plugin's glue.rs / TS index). Never edit root `📜️script.ts`, taxonomy, kernel, mesh MediaFormat, or framework codecs — those belong to W1–W4.
2. Diff your leaves against the note and cad pilots quoted in normative spec §13 (filled by W5). Do not invent a different shape.
3. Gate **only** with the scoped scanner below — never `bun ./📜️script.ts policy`.
4. On macOS: `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
5. Required formats for your artifact are exactly the `formats` array in `🧪owner-table.json`. Create one format dir per entry with both import and export leaves.
6. Leaves map Snapshot ↔ Neutral only. Call framework codecs; do not re-implement GLB/SVG/etc. in the plugin.
7. Delete old `register_2d_export_handlers` / `register_mesh_*` / `register_solid_*` / `register_dwg_*` from `⚙️engine` and call `io::register()` instead.

## Scoped gate

```bash
bun -e 'const m = await import("./📜️script.ts");
const b = m.policyArtifactIoBreaches(process.cwd()).filter(x => x.scope.includes("PLUGIN_DIR_FRAGMENT"));
console.log(b.length); for (const x of b) console.log(x.kind, "|", x.summary);'
```

Also: `cargo check -p CRATE` then `cargo test -p CRATE --lib`.

## Per-artifact deliverable

```
🚪️io/🦀️component.rs
🚪️io/🟦️component.ts
🚪️io/<format-dir>/📥️import/{🦀️component.rs,🟦️component.ts}
🚪️io/<format-dir>/📤️export/{🦀️component.rs,�📦component.ts}
```

Wire in glue.rs under `artifacts.<ascii>::io`. Update the TS package index.

## Assignments

One agent per plugin (see owner-table `plugin` field). Multi-artifact plugins (block, puzzle, fem, gis, procedural, trinity, norm) own all of that plugin's artifacts.
