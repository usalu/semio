# GIS Terrain artifact and media lifecycle

The GIS Terrain editor routes both map and elevation media through Semio's registered original import job. The job submits typed artifact mutations to the mounted Store. The Store applies sparse reversible diffs and preserves sibling document fields and independent mesh references. A second import replaces the existing map or elevation source. The fixtures cover the forward changes and restoration of the exact initial source across undo and redo.

GIS Terrain stores an optional `elevationSource` as a WGS84 grid of 33 by 33 signed whole-meter samples. The replacement mutation has an explicit clearing value and inverse. The first-party value, DSL and Pack codecs, TypeScript schema, and SQLite projection preserve the source. The bundled example authors the local Liège elevation alongside its GIS features. User-authored bounds and heights are independent of incoming map overlays.

The existing Semio plugin media SDK owns the bounded structured export job shared by both GIS editors. GIS Map publishes `2d.map` and GIS Terrain publishes `gis.elevation-grid` and `mesh.document`. The registered export factories borrow the exact mounted snapshot and publish bounded chunks through the original output lease. Terrain's map input also accepts its lossless intrinsic transport. The structured map route refuses intrinsic values that conventional JSON cannot carry exactly, including octets, nonfinite IEEE words, positive signed number tags, and duplicate object members.

The interactive copy grant is 32 KiB. The full GIS Map bundled example is 243,464 bytes on disk. A cold factory that charges the entire persisted collection against one turn's copy grant stalls after a substantial import. The editor uses the shared cold Store preparation and admits incoming collections under the existing 1 MiB Store publication bound.

The document contract preserves the native binary64 exaggeration field. The editor retains its existing 0.5–5 controls, with a document default of 0 in the type and a scale of 1 for the ordinary authored example. The new local terrain renderer explicitly refuses invalid projection factors. The mutation path now rejects nonfinite exaggeration.

## Verification

- Ajv validated the local source JSON shape, persisted sparse diff, replacement and clearing vocabulary, and the independently constructed history fixture. It rejected the wrong raster dimensions and elevations outside the signed sample range.
- The Python reference mutation model replayed the separate committed source-creation vector and restored its exact prior snapshot through its independently computed inverse.
- The source schema, TypeScript and normalized SQLite suites passed their target checks in the terrain model workstream.
- `repo tools exec git -- diff --check` passed.
- An earlier repository Cargo run stopped before the GIS crates on E0015 and E0502 in `semio-framework-3d-brep-representation`. Live upstream source repaired both issues before the current mounted test commands were submitted.
- The Terrain editor test retried past BRep and Store, then stopped before the GIS crates on E0425 in `semio-plugin-job-receiving`. Its captured build missed `semio_framework_job::unquoted_receiving_work` from the call in `plugin/👷️job/🛬️receiving/🏗️steps/🦀️.rs:88`. Live Job Context defines and exports the helper, and the live Job facade re-exports that Context. The terrain model workstream started a fresh build after the live sources converged.
- The separate GIS Map mounted test cleared compiler-image capacity and is compiling shared framework prerequisites. Its new fixture requires output larger than the interactive per-turn copy grant and compares all emitted feature records.
- The post-convergence Terrain editor retry is queued for host memory under the official repository Cargo runner. It has yet to start compiling. Both repository commands remain active.
- The mounted test commands have yet to verify GIS Map export, GIS Terrain import/history, or either GIS Terrain export. The source-level media audit confirmed that the production interactive scheduler forwards the original caller's retained grant. The normal mounted policy has 16 MiB of capacity and 32 KiB of copy per turn, which can fund the 1 MiB structured export reservation and 4 KiB output pages.
