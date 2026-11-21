# GIS terrain elevation and export

The Liège example packages 1,089 elevation samples, ranging from 47 to 252 meters, from the Mapzen Skadi N50E005 tile. The committed grid uses a six-arcsecond step and covers Sart Tilman. The authoring script reads big-endian HGT samples and verifies the source checksum. [Source tile](https://s3.amazonaws.com/elevation-tiles-prod/skadi/N50/N50E005.hgt.gz)

The GIS artifact projects the elevation grid authored in its snapshot. Loading the standard example supplies the Liège grid and uses its midpoint as the local origin. Imported positions, routes, and regions are overlays with independent ownership. Changing or clearing them keeps the terrain surface. A terrain without an elevation source returns an explicit export error.

The terrain mesh and viewport share the same surface projection. Coordinates use east, north and up axes in meters. The shared mesh applies the artifact's exaggeration to sample heights and builds the same 2,048 triangles for every format. The neutral JSON fixture specifies the expected corner coordinates, counts and elevation range. The independent glTF reader verifies those values and triangle winding.

The SQLite snapshot stores each authored elevation as a normalized sample row with a dense ordinal and preserves the four geographic bounds as exact IEEE 754 integer words. Tests query these rows in Bun SQLite to verify integrity, ordering, and independent edits. The demo does not declare a separate mesh child. Existing explicit child references retain their identity independently of terrain inference.

The registered demo is a regular Semio terrain document serialized through the native DSL codec. Its constructor retains the checked-in elevation and scenery assets as durable fields that can be exported and loaded again.

The TypeScript source tests passed 13 cases across the public package SQLite tests, semantic SQLite tests, and Ajv document contract. The package runner was waiting for the shared project graph, so Bun resolved workspace package imports directly to their source using a temporary scoped TypeScript project.

## Native verification

The latest completed native GIS package test attempt exited with code 101 before compiling GIS. Rust reported E0425 in `semio-plugin-job-receiving` at `👷️job/🛬️receiving/🏗️steps/🦀️.rs:88` for the missing `semio_framework_job::unquoted_receiving_work` function. The live worktree now defines that function in the owned Job context and re-exports it through the Job facade.

The fresh retry of the GIS Terrain library tests through `repo cargo --store=shared --configuration=prototype` is active. It has reached the Job and Pack framework crates. The GIS mesh, native DSL/Pack and Rust SQLite tests remain pending.
