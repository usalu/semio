# JSON Source Boundary Carve

Read-only current source proposal. Actual native guard reported root semio-framework-pack-json closure exceeds262144 bytes while admitting operation.rs7838 with7661 remaining and15 remaining files. That is not merely177bytes excess: later normal files still need admission. No quota override/minification/source copies proposed.

## Small coherent lower owner

Move module ownership (not duplicate source) of canonical typed JSON encoding to a new lower package rooted at existing 🛫️encode domain. Exact normal set:
- 🛫️encode/🔤️text/🦀️.rs — 7161 bytes
- 🛫️encode/🔣️scalar/🦀️.rs — 5950 bytes
- 🛫️encode/🧭️tree/🦀️.rs — 46117 bytes
- 🛫️encode/🧭️tree/📨️wire/🦀️.rs — 14714 bytes

Four files total73942bytes. New small root can mount existing child paths and reexport the same authored types. Root pack-json should replace its three inline module mounts with explicit lower-package reexports; testkit tree forwarding must follow the new owner.

Cross-boundary imports: text uses value::paged::Utf8Text. Scalar imports local ArtifactCanonicalJsonText and first-party pack-json-float. Tree imports local text/node/scalar/escape plus value/value-persistent. Wire imports super tree+cursor and value. No root JsonError/Value/parser dependency was found in this group. The trait and its foreign-type implementations remain in one lower owner, preserving coherence. Root consumers keep explicit reexported first-party types.

## Existing boundaries and cycles

Current root manifest already depends on pack-json-float. Existing pack-json-native-input and pack-json-source-sink both depend on root pack-json; moving operation or parser code there while root reexports them would create a cycle. Existing retained/schema package is a different domain. Canonical encoding is a genuine independent domain and removes enough source to restore headroom without raising bounds.

The full authoritative root normal closure was not recomputed here; compiler guard remains authority. The four-file carve set and byte count are direct current filesystem evidence, not a guessed complete package total. Native tests must validate the changed dependency graph and unchanged behavior afterward. Existing tree cfg-test submodules and testkit users need mounting at the new owner, not extra normal source dependencies.
