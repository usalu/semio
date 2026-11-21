# Directional graph port hover

A same-named input and output now have distinct interaction addresses in the Flow canvas, outline, channel readout, and spatial preview. The codec keeps authored channel names in port metadata and decodes canvas endpoints at the wire commit boundary. Explicit input hover still follows the incoming wire to the visible upstream geometry. A port-level mark highlights its own direction, and a node-level mark covers both.

The channel readout now has one row per direction, with separate hover bindings and localized direction labels. Empty names and names that start with the reserved marker have unique canvas identifiers. The fixture also covers non-ASCII names.

## Verification

- The two focused Rust tests compile the actual codec, graph projection, WGPU port adapter, outline rows, and world mark matching. They use fixture host and port records and check distinct addresses, matching wire endpoints, recovery of the authored channel, and upstream routing.
- Thirty-six Bun tests passed with 100 assertions. These include the production browser pick and hover adapters, SQLite-backed direction laws, preview values and paths, and existing hover timing regressions.
- The scoped Rust syntax check and whitespace check passed. The registered wire reconnection and channel readout tests now use the directional fixture. Two existing preview tests also needed their parsed evaluation and host result passed at the correct types.
- The earlier Cargo process ended without a recoverable result. The replacement registered preview test is compiling dependencies through the repository runner. Its output is retained under the ticket's generated directory for the next check.

The canonical mounted pointer journeys are still pending. In particular, coincident input and output geometry needs a real renderer pick test.
