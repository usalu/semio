# Declared Owner Source Readiness

The read-only census inspected every current registry `owner.manifest` and declared `nativeRoots/Cargo.toml`. Registry SHA-256 `f2653fbee5dc9568e0fe37f16a2ca0ac8cc96b193ca88cdac322dfec971e66b2` remained unchanged during the census. It contained 1628 owners and 1630 unique Source claims: 1627 regular files with real directory ancestry, and 3 missing files. No symlink or non-regular Source issue was observed.

The registry had grown from 1,622 to 1,628 owners since the preceding identity check. This result is a current point-in-time declaration/readiness check, not a graph publication or Source-stability receipt. No owner was removed, invented or relaxed, and no file was created.

Missing Source:

- `semio-s-artifact-process-process3d-io-sqlite`: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📦️packages/🦀️rust/Cargo.toml`
- `semio-s-artifact-cad-cad-io-sqlite`: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📦️packages/🦀️rust/Cargo.toml`
- `semio-s-plugin-norm-mutation-vectors`: `✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️mutation-vectors/📦️packages/🦀️rust/Cargo.toml`

The missing set was sent to Core before another producer attempt. Exact observations are retained in `🗑️generated/owner-source-census.json`. A later zero-missing census is required before claiming all currently declared entry manifests are physically ready.

A bounded physical-shape follow-up found no project-manifest substitution at the three missing roots. Both declared SQLite package directories are absent; their nearest existing SQLite directories contain inline Rust Source and a snapshot directory. The norm mutation-vectors subtree is entirely absent under the existing tests directory. `owner-missing-source-shape.json` records these observations. They do not alone distinguish pending new packaging from a stale registry claim, so no declaration was rewritten. Core owns the single original 180-second full-readiness loop; this agent does not duplicate it.

## Actual identity correction

Independent @iarna/toml parsing of the norm en1992 workspace resolves its semio-s-mutation-vectors dependency to the existing GLOBAL ✏️s/🧪️tests/🧪️mutation-vectors/📦️packages/🦀️rust/Cargo.toml. That actual package is semio-s-mutation-vectors, role s-module, with library ../../🦀️.rs; its package directory has no project manifest. The wrong norm-local owner had no corresponding Source. Both SQLite modules are explicitly inline under the existing process3d and CAD model crates: schema Rust modules point to ../🚪️io/🪶️sqlite/🦀️.rs (process line26, CAD line28); the model Cargo manifests own those libraries and SQLite snapshot integration tests. A scoped TOML census found no standalone SQLite crate. No production registry generator was found by a scoped executable-Source reference search.

Root authorized exact asset correction: remove only the two nonexistent standalone SQLite Cargo owners, and replace semio-s-plugin-norm-mutation-vectors with the actual global semio-s-mutation-vectors Cargo owner. Every unrelated authored byte, row order and catalog provider was preserved. One unrelated owner arrived after RED and was retained by the immediate prewrite reread. No Rust Source, Cargo file, policy, target or reader behavior changed.

Retained language-neutral registry/🔣️.json and registry/📜️script.ts first failed on the original three false identities/absent manifests and missing actual global identity. GREEN validates the actual shared reader, independent Ajv2020 schema, independent TOML package/dependency paths, real inline module/test ownership and ALL no-follow manifest/native-root claims: 1,627 owners, 1,629 claims, 1,629 regular files, zero errors. Current SHA e8fe374b17aaa1730cd0f5f4adf703f7b1b4ce0e6d7ccda3dc72a7e2df53d916 remained unchanged through GREEN. Exact generated registry-identity-red.txt, registry-identity-green.txt and registry-identity-preservation.json retain evidence; owner-source-provenance.json retains the declaring dependency. This is physical declaration readiness, not graph/backend acceptance. Core received source-settled immediately.
