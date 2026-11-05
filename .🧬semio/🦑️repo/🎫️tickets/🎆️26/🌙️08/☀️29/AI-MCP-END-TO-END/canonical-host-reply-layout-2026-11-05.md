# Canonical Host Reply Layout

The first-party provider preflights the declared return span and alignment before changing a resource or calling a receiving port. The owned WASI write/flush success reply now stays inside its twelve-byte result area; the prior sixteen-byte write could overwrite the next guest field.

For WASI 0.2.0, stream-error contains a resource handle or a unit case. Applying the component model variant layout rules gives stream-error size 8/alignment 4; result<unit,stream-error> size 12/alignment 4. Check-write's u64 success payload gives size 16/alignment 8. Clock optional u64, wall-clock datetime, seed pair and pure begin result use sixteen-byte areas aligned to eight. Environment lists and terminal options use eight-byte areas aligned to four.

These sizes are derived from the primary [Canonical ABI alignment and element size rules](https://raw.githubusercontent.com/WebAssembly/component-model/main/design/mvp/CanonicalABI.md) and the declared [WASI 0.2.0 stream types](https://raw.githubusercontent.com/WebAssembly/wasi-io/v0.2.0/wit/streams.wit). This research establishes layout, not execution. Native compilation and sentinel-memory laws remain required; no actual guest success is claimed.
