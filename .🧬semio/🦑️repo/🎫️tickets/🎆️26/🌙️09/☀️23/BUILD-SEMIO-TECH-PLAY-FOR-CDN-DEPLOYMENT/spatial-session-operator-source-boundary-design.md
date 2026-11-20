# Spatial session source ownership after the mesh export addition

The failed release scan measured 13 Rust files and 263,300 bytes in the spatial session. By the time of the follow-up, peers had moved the test-only physical session custody and capture-close modules out of the production root. Both files are mounted under `#[cfg(test)]`, and the physical custody fixture path resolves from its new location. The new mesh export module is 5,036 bytes, and the current session core is 131,458 bytes.

The live source closure measured 13 files and 256,447 bytes for the spatial session, leaving 5,697 bytes below the 262,144-byte ceiling. The B-Rep engine measured 23 files and 235,976 bytes, leaving 26,168 bytes. The spatial kernel, B-Rep engine and Flow B-Rep trees passed the scoped source preflight. The selected Bun source scanner tests passed all 28 tests and 49 assertions.

## Operator registration boundary

A first-party spatial operator registration library can own the full 24,649-byte adapter that already lives together at `🧰️operator-registration`. It owns the geometry dictionary vocabulary, schema and channel metadata, registration functions, kernel error translation, and topology deconstruction. The session can re-export the registration functions under their current public names. Deconstruction can accept the borrowed kernel and input dictionary in the library while the session's existing wrapper performs the kernel acquisition and retains its public signature. This places the dependency on B-Rep, the neural engine and the value and JSON interfaces in the adapter without creating a reverse dependency on the spatial session. The estimated session savings are about 24 KB, for roughly 30 KB of headroom at the current revision.

No existing lower owner has both the B-Rep and neural dependencies and owns the shared adapter. Placing it in the Flow extension would point the session at one of its own consumers. A dedicated spatial adapter would need the `✏️s` workspace member, a build owner registration, and a new package and dependency edge in 19 checked-in consumer lockfiles. The implementation and its validation should be coordinated outside a live release.

## Existing-owner options

The new bounded mesh export cursor can move to the existing B-Rep engine beside the native preview and mesh-text APIs. Keeping session custody methods would save about 4 KB. The cold invoke dispatcher spans 13,794 bytes including its wrappers, with a further 5,717 bytes of JSON argument and result helpers. A dispatch split needs to keep session handle claims, retirement, cache eviction and injected extension dispatch under their existing authority, which makes it a larger behavior change than the operator boundary.

The supervisor had already started its retry, and this review changed only the ticket note. The independent full release scan of `✏️s`, `🌎️hub` and `🧰️framework` exited zero with no `OVER` rows. Its three generated-token observer diagnostics match the existing audit. [Source preflight observer audit](boundary-errors-audit.md)
