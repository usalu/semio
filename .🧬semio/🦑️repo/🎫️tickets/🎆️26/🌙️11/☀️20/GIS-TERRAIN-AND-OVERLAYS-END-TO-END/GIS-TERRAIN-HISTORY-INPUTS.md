# Terrain History Input Schemas and Fixture

The Terrain editor’s existing History law reported three consequences of one missing reference. `replace-elevation-source` could neither enumerate its inputs nor compile its validator because the runtime resolver had no schema under the authored elevation document’s ID. The mutation derive publishes transitive input schema documents from the shared generated schema index. The elevation document and the replacement mutation leaf were both present on disk and omitted from that index.

The History acceptance law reads the editor’s fixture root. Existing lower-level Terrain mutation cases live alongside the editor and include a derived mesh handle whose content a bare snapshot cannot materialize. The new self-contained editor fixture commits an exaggeration change from 1.25 to 2.5. A native test checks the Terrain reducer’s fresh fold against the Serde JSON fixture. The generic History law supplies the overwrite, alternative and replay checks for that same case.

Both independent Ajv tests passed. They check the neutral fixture against its authored mutation schema and validate the elevation mutation with its exact referenced schema, including a rejected short grid and an accepted clear operation.

The schema-index generator dry run validated its output and found three additions in the GIS root. It also found twelve unrelated changes elsewhere in the worktree. The GIS-only projection adds `gisterrain/elevation.json`, `gisterrain/mutation/replace-elevation-source/schema.json`, and `gisterrain/scene.json`. The shared index write and native verification are waiting for the active GIS Dev build to finish its captured source pass.

The canonical generator’s GIS-only projection is now applied to the shared index. The three expected Terrain IDs resolve to their authored elevation, elevation mutation and scene documents. Native History validation remains queued.
