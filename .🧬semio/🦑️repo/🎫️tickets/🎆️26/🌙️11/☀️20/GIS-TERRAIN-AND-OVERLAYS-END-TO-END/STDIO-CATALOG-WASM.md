# Stdio Catalog Commitment in Browser Components

GIS and VCS publish a dependency on the Stdio catalog and include its exact semantic commitment. Both previously obtained the commitment by constructing the catalog’s artifact assemblies. Those calls retained the Stdio codec implementations in the consumer WebAssembly modules.

The Stdio composition preparation already selects the full roster from the present artifact owners. A projection-only recipe uses the same selector and packages the exact schema bytes and native codec receipts in a single raw DEFLATE bundle. The consumer decodes the bundle once with the first-party bounded inflater, verifies receipt closure, and uses each definition’s authored executable registration identities. It shares the native catalog’s descriptor hashing, budget, serialization, ordering, topic, and package identity. The full catalog still checks each executable factory against its artifact definition before accepting the commitment.

The checked-in Stdio component descriptor held an older 36-definition roster when inspected. The current full composition includes IDML and IDF, so using the checked-in descriptor as the runtime source would commit to a stale selection.

The GIS Dev build recorded the per-artifact include refusal in `gis3d-dev-published-emitter.log` when the consumer crate’s source closure exceeded 262,144 bytes. The single bundle measures 28,039 bytes and expands to 372,470 bytes. The crate’s source and bundle total 28,842 bytes. Preparation and decoding enforce a 128 KiB compressed limit and a 1 MiB expanded limit.

The native roster agreement law submits the consumer commitment to the trusted native validator and compares the full and consumer commitments by topic, typed payload, exact JSON bytes, and an independent Serde JSON decoder. The projection recipe produced 38 definitions and 35 receipts. Both Bun tests passed with 123 assertions. They verify the selected schema owners and exact original schema bytes, compare the consumer receipt projection with the full catalog’s receipt projection and the original artifact JSON, and check the consumer crate’s dependencies and source extent. Python zlib independently verified the compressed and expanded sizes.

The standalone Rust check waited for the shared compiler grant and was canceled to free capacity for the GIS browser build. Rust byte parity and browser linkage remain to be verified by the repository builds.
