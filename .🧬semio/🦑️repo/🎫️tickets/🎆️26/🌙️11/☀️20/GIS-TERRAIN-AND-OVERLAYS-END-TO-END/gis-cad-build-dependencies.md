# GIS CAD Build Dependencies

All seven GIS Terrain crate manifests declared DWG as a direct dependency. GIS Map's aggregate and terminology test crates also declared DWG, DXF, and PDF. GIS source does not refer to those direct crates. Terrain's editor also reaches the Map aggregate, so removing only the editor's declaration would still compile DWG.

Removed these direct edges from the nine manifests. The Terrain and Map workspaces, GIS bridge, and GIS development service no longer resolve either DWG package. Hub and the fixture sweep still resolve DWG through other consumers. The Stdio drawing import and export crates still resolve the PDF and DXF converters they use.

Changed the six lockfiles that record the removed GIS edges. Cargo's offline workspace refresh also reconciled existing lock drift in the GIS bridge and fixture sweep. It pruned 123 additional unreachable package records from the bridge and removed the already detached GIS Map terminology test package from the fixture sweep. The fixture sweep refresh also reconciled live BRep and EN 1998 formula dependency edges. No package versions, registry checksums, or workspace manifests outside GIS changed.

The read-only graph audit checks that all six lockfiles have no direct GIS CAD edges, the four scoped graphs omit DWG, and all retain the drawing converters. The pinned Cargo accepted all six with `update --workspace --offline --locked` against the repository's provisioned registry. The existing native test build continued independently with the graph it had already resolved.
