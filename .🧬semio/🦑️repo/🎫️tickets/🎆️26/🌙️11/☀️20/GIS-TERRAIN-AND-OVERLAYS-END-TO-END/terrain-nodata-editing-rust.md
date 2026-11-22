# Terrain NoData Editing Rust Verification

The repository-pinned Rust toolchain compiled the staged elevation-edit inference against the exact `missing_samples` implementation from the canonical exchange patch and the staged `ElevationSource` shape validation. The standalone harness supplies the surrounding framework value error and terrain projection types. The patches were read without modifying production files.

The Rust run passed four tests against the same rectangular fixture used by the independent SQLite Bun oracle. The offset and raise/lower edit preserved the missing cell and its mask. Smoothing returned 54 mm for the edited cell by omitting the missing neighbor. A direct write to the missing cell returned `gis-terrain-elevation-sample-is-missing`. Raise/lower and smoothing also reproduced their expected arrays via a single brush stamp and refused immediate cancellation without changing the source.

Run `verify-terrain-nodata-editing-rust.py` through `repo tools exec python --`. The script validates the input patch contexts, compiles the temporary standalone test binary under the ticket's generated directory, runs it, and removes its generated inputs and binary. Mounted publication and browser behavior were outside this probe.
