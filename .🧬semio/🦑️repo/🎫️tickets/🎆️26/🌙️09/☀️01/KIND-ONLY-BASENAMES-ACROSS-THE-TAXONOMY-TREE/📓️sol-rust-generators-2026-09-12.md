# Rust Generator and Oracle Taxonomy Repair

Date: 2026-09-12

## Result

This lane replaced Rust-named and external-library-named generator, probe, and oracle package roots with semantic concern owners and anonymous Rust leaves. The final tree has one generator collection spelling, 🏭️generator. Cargo package metadata remains below each semantic owner at 📦️packages/🦀️rust, while every lib and bin source lives outside the package boundary.

- 45 generator/probe Cargo packages retain their original package identities and explicit targets.
- The shared stdio oracle is the 46th package and now leaves its 969-line implementation at 🔮️oracles/🦀️.rs.
- All 46 package roots contain only Cargo.toml and Cargo.lock.
- The original census contained 89 Rust sources: 78 named implementation leaves and 11 already-anonymous leaves. Every final implementation leaf is anonymous.
- 179 source, manifest, and lock moves were captured with pre-move SHA-256. Eleven test-owner moves and 26 canonical generator-collection moves completed the same ownership repair. Two sources moved through an intermediate destination, yielding 217 physical rename operations and 215 logical old-to-final identities.

## Design decision

Each old implementation crate remains a distinct Cargo package beneath its semantic concern. Combining packages would destroy independently invoked identities: 28 packages expose a bin named generate and 21 expose a target named reader. Package metadata is therefore separate even when the concern name is shared, such as JSON or codec. This preserves all existing package names and target names without compatibility packages.

The two generator spellings were not separate domains. 🏗️generator was canonicalized to 🏭️generator. The 🏗️ emoji remains valid only for the nested generate concern below JSON or codec.

## Schema and normalizer

- Added semantic directory kind generator-codec: 🔁️codec below generator.
- Added semantic directory kind generator-generate: 🏗️generate below generator-codec or json.
- Removed generator-crate.
- Reparented dxf-codec, svg-codec, pdf-codec, recipes, and cli below generator-codec.
- Removed fixed directory shortcuts cargo-source-directory, cargo-build-source-directory, and cargo-probe-source-directory.
- Removed the language-specific packageLocation generator-crate branch from normalization after every consumer moved. The shared discovery package-source classifier remains authoritative.
- Extended the portable kind-only fixture to cover 🏭️generator, 🔬️probes, and 🔮️oracles and to reject 🏗️generator.

Direct validateTaxonomy returned zero problems. Focused inventories prove JSON, CSV, codec, package, Rust ecosystem, reader, probe, and oracle collection resolution in owners with valid ancestor context.

## Test-driven evidence

The portable Nx test was added before the move. Its completed red run reported 6 passing tests and 1 failing test with 41 noncanonical manifests. An earlier five-second observation window expired before that run completed. After the repair, the final registered run passed 7 of 7 tests with 340 assertions in 12.01 seconds. It independently parses Cargo TOML, requires every manifest below a semantic collection to end at 📦️packages/🦀️rust/Cargo.toml, requires each explicit Rust target to be anonymous and outside the package directory, rejects the obsolete generator spelling, and checks live source existence.

Native/parser evidence:

- cargo metadata --no-deps parsed 46 of 46 manifests. All 46 package names and target-name sets match the pre-move records; every target source is 🦀️.rs outside the package root.
- The metadata pass exposed two stale FEM semio-framework-pack relative paths; both were rebased to the same framework pack manifest and the repeated 46-manifest pass was green.
- 45 of 45 Cargo.lock files retain their pre-move SHA-256.
- The full current generator/probe/oracle census found 235 Rust sources and 441 literal path/include referents with zero missing referents.
- Bun.Transpiler parsed 77 of 77 generator/probe 📜️script.ts files. JSON.parse accepted all 36 JSON documents among the 42 descriptor/feature consumers.

Registered Nx native package tests:

- equation-1-any-json-engine: compiled lib, generate, and reader targets; all four harnesses completed with zero failures.
- gif-89a-any-fixture-generator: 7 passed, 0 failed, including process-local deterministic encoding and recipe round trips.
- image-bmp-codec: 4 passed, 0 failed, using the image crate oracle.
- tobj-obj-reader: 1 passed, 0 failed, using tobj.
- png-codec: 3 passed, 0 failed, using png.
- note-oracle-codec: compiled against lopdf and completed with zero failures.
- semio-s-plugin-stdio-test-oracle compiled from the owner-level source and ran 32 of 33 tests successfully. The current DWG assertion reports that its committed catalog lacks no-mutation. No before-state run established provenance, so this is recorded only as an observed current failure.

The broader registered path-emoji-statutes target ran 37 tests: 32 passed and 5 failed. The relevant configurable Cargo source-entry test passed. Current failures concern empty mutation catalog discovery, missing glTF and TSV singular oracle paths during concurrent oracle taxonomy changes, a generated icon include path expectation, and the disconnect projected-scenario expectation. These observations are not attributed to this lane without a before-state run.

## Scoped directory inventory limits

The table retains both the initial post-topology checkpoint and the final checkpoint after moving the external-library-named test directories into their semantic owners. Repeated scopes therefore show how the final pass changed.

| Scope | Entries | Violations |
| --- | ---: | ---: |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator | 21 | 1 |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator | 26 | 0 |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator | 27 | 0 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes | 15 | 1 |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes | 16 | 1 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator | 20 | 4 |
| ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust | 8 | 0 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator | 25 | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator | 20 | 3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator | 20 | 0 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator | 26 | 2 |
| ✏️s/🔌️plugins/🗄️stdio/🔮️oracles | 34 | 11 |

Exact unresolved directory paths reported by those scoped checkpoints:

- ✏️s/🔌️plugins/➗️mathematical — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/➗️mathematical — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🔬️quick-xml-svg-codec-src-unit — directory-kind-unresolved: observed at the initial checkpoint, then eliminated by moving the test to `🔁️codec/🧪️tests/🧪️unit/🦀️.rs`; it is absent from the final checkpoint
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🧪️tests/🔬️unit — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/🧪️tests/🔬️unit — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/🧪️tests/🔬️unit — directory-kind-unresolved: Directory has no registered semantic kind
- ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh — directory-kind-unresolved: Directory has no registered semantic kind

Sequence, note, PNG, and the package/oracle roots with valid ancestor contexts were clean. Apart from the explicitly resolved SVG test checkpoint entry, the remaining entries above are ambient ancestor/version/subset vocabulary or pre-existing stdio oracle family/test directory registrations. The SVG codec is unresolved only because its 🧱️base ancestor prevents generator context from resolving. No named Rust leaf or package-body implementation remains in these scopes.

## Authority and consumer files

Taxonomy/test authority:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌳️kind-only-basename/🔣️.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌳️kind-only-basename/🔣️.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts

Producer/probe consumers:

- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔬️probes/📜️script.ts
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts

Oracle descriptors, portable fixtures, and feature consumers:

- ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/➗️equation/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🎛️graphic-control/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧩️application/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧪️tests/♿️mutate-pdf-1-7-ua/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🧪️tests/⚕️mutate-pdf-1-7-h/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧪️tests/📐️mutate-pdf-1-7-e/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧪️tests/🖨️mutate-pdf-1-7-x/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧪️tests/🗄️mutate-pdf-1-7-a/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧪️tests/🧾️mutate-pdf-1-7-vt/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧫️fixtures/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧫️fixtures/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧫️fixtures/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧫️fixtures/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🔣️.json
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json

The launch seed and derived launch file already address these packages by stable Nx project identity, so no new command or path-specific launch entry was required. Existing registered build/check/test commands discovered the new manifests, as demonstrated by the Nx runs.

## Complete move map

### Rust sources, Cargo manifests, and locks

| Kind | Old path | New path | Pre-move SHA-256 |
| --- | --- | --- | --- |
| source | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/🏭️generate.rs | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs | aaaec792b65c816e9f803cddd1ace67008d90ddaf9e2eb5df4f368f2c496a205 |
| source | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📖️reader.rs | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs | 1750ca4e6ff2d5d51fd46ef8520cac187c99ca5d5b87a5b0045b36a5f4e74a7f |
| source | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📚️lib.rs | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🦀️.rs | 2d0f1f97d924f50bba58b5d802a544f342767ae0fa3cacfa5b1284ccc478736d |
| manifest | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 383fd8417e292480132de2f1fc64a5b41939cf6469a32119b7151415d44cf2f3 |
| lock | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | ab0491891ede549415e37c8b56f996a2b08e4b5c69ccea3e509a5ddc62f7b888 |
| source | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🦀️oracle-probe/🦀️.rs | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🔮️oracle/🦀️.rs | 16a78580b1f8ad725b3305c87a7ca26042cd3ca129a5485ca71b7296b3a6a3b4 |
| manifest | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🦀️oracle-probe/Cargo.toml | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.toml | 5727acb0fb435400e21c321ed850446c62dad70ae7768cc9c33d20707f918ed3 |
| lock | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🦀️oracle-probe/Cargo.lock | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.lock | d667af10e57d17ca549bf63506906b6dc0ecc9a2895ea6f650c256e263b21479 |
| source | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📊️csv-engine/src/main.rs | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📊️csv/🦀️.rs | 229347e699b736a48faff35e769809359c64b1f72412fcacb62f8cf7c3830319 |
| manifest | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📊️csv-engine/Cargo.toml | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📊️csv/📦️packages/🦀️rust/Cargo.toml | 30460c3e0ed2ab99e8ab3a9d85be234dc3bdcced313cd59eaeef9a2346ecf34c |
| lock | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📊️csv-engine/Cargo.lock | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📊️csv/📦️packages/🦀️rust/Cargo.lock | c6d768ff9f5e7ffc5458051c7f1fe645775f9e32981206db1c682864fcc1fe65 |
| source | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧾️json-engine/src/lib.rs | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🦀️.rs | 6cdfab0cdd4955db5b8c3f0339bfec1904256aea8463b9493c6a5a21a6519d23 |
| source | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧾️json-engine/src/✨️generate.rs | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs | 60ecad766ae25243781436e2fc9e414338fd938f809e401913911a2baea5914d |
| source | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧾️json-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs | 4aaa3187bdccdd197dbf40e6f0452f9f6528f377855b3b292890e36cce35eecc |
| manifest | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧾️json-engine/Cargo.toml | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 3af9008fc27e71867a41a00b154e95b25cf5f466454d6d04bd1a40a793ee3ab2 |
| lock | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧾️json-engine/Cargo.lock | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | 5e965bc876c8b6036b73ed220f7e95d21031f0caf47c0eb426901ab6b144d2b5 |
| source | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/src/🏭️generate.rs | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs | 12019626192340e3c269c6a8a87a88bbff9425e09c6687d2ae58836883e433b9 |
| source | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📖️reader/🦀️.rs | 924c6d40fcfd90062b4a6856c24f18621c97957de137ae1c324d2c143f5e63eb |
| source | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/src/📚️lib.rs | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🦀️.rs | b68a8359699b360d53c7b4267506a2c7f907a9bd795257e2d79fba61e93cb2af |
| manifest | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 848f023b53a716f1d459ed353ed7ceb142a87ff8bd5528cf260525f03c3629c8 |
| lock | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | 54e95eca303f0e1e995585660265835b4861d970d68a28607f8ad6dd20d5388c |
| source | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/src/🏭️generate.rs | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs | 958712639c1fb22452e86dd4f8989647eb3799a3cbd43560eaacc8f3ae109e6e |
| source | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📖️reader/🦀️.rs | 380afd7a6a4afedcce876e50d5aa3afa053c6f3e470499ca7bff8158c29c4f86 |
| source | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/src/📚️lib.rs | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🦀️.rs | 47ed0abb6c3d94382414d99f5e24038c38f9bf8309ac60321cabe5e524ec13d4 |
| manifest | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 22bd2000f31fcdcead2d52767b74c6ccd68da96335a59c52c7aa22879385eff5 |
| lock | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | 7a683575fd910b5b5fb14649e81c7faa1b386942d3d618d20d7a9f3f5915bf6b |
| source | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/✨️generate.rs | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs | 0b4b7f247b16b32bc2cc8dbbcc232fb59a76dde1898863d97da393ad28c21f26 |
| source | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs | 9f583a32184b84ad79c4f04d84ecdd735d1fd057a096ed51f3eaea27984d91df |
| source | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📚️lib.rs | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🦀️.rs | 02fcf0831443b67d169b0223ef26da213b1df93edbda6310907345330d5eaeb6 |
| manifest | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 0ed8ecc821c47c31a84dc78cc859fc1b1687f8395bb2dd9fd1c6a38441993643 |
| lock | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | fc36881b9a789fa79c3504249e61e6b40d6cbb4d9b1293ef8c2380b51350ab7c |
| source | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🦀️oracle-probe/🦀️.rs | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🔮️oracle/🦀️.rs | acd3e3d4cd62c6afc8415a99b58cab13f2c554b729964efae2754a786d7be1c1 |
| manifest | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🦀️oracle-probe/Cargo.toml | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.toml | c3d80497919b5c8f958a37b9e8a3a16b0d48c522d5e35bd7f7196cc96e8770fd |
| lock | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🦀️oracle-probe/Cargo.lock | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.lock | 5c23f98a7da0385aa90a46416765f54df60286ed8f02d87661aa66168d7fe4fc |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🦀️engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧫️fixtures/🦀️.rs | 293f6874803d1256a762aa8a17a3edab71db52e666bc904d1b7ddab00829d708 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🦀️engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.toml | 5dfa82c3d8a4d554a57147fb03bfabbd9d9862ed8a8a9cba5ce3cf0c2b30ee62 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🦀️engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.lock | e01951947a6c12cc3ff73e84c0994f63f83694d16ea8f9fe2d87e86be6e45695 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/🦀️reader/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/🦀️.rs | b4f51fb9e0d1290e33cef46fba3df93794c31b7a89574761d22eb1f7f7d8ae91 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/🦀️reader/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | 28f6baa65bc57c3dd4ab69ffa5c15f25fcc5504725730d2f0887dfdfad190424 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/🦀️reader/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock | 7b31c457c3cf8d44a1c6d8e3ec7b39f8739fdd25180d2d09c63813fcfb51e7da |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🦀️engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/🦀️.rs | c02b7eb82eceb9f9f75acf8574e74c73845c21124f182a8a3dd2cf9e38ad6ede |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🦀️engine/src/📖️reader_main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📖️reader/🦀️.rs | 55ebe320ae9e73a81017ef2fc87284d71d40c82a31cc4b5c8242958edd6efb2f |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🦀️engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.toml | f6688ca072fb9e9ee8a7bb7ef37d655ab4df762c4ff4ac7c89f23b25010bcbd3 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🦀️engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.lock | 706b35420d3be9548b7133ac168212b6f23dc530373c3a355b173d9f12628557 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/🦀️extension-reader/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/🦀️.rs | 03467db5148567559f9215da1d97a4ca7f75f7309eeb282f1f1c4eb4682f1b02 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/🦀️extension-reader/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | 05158a3c7bfdb7d01910d80abc54a3c268f460c3ee21409a6ffe6e627ff676d9 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/🦀️extension-reader/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock | 3fa0d8df27b56d5a8e98a6b46b749b8968122ca047ef5f952def04a6ec6a995c |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🦀️quick-xml-svg-codec/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs | c82cd7231ab7547951448f7b1412d6243187552a71933b7f732ebc386c194f56 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🦀️quick-xml-svg-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | da558fb15e9944f4c1d056d8a77780b59d217ec91c9a57cdde4d9813839b13a4 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🦀️quick-xml-svg-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 18c817146ac1e27faa2393b4875b997a07813f2d212895896bdab9859929b383 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🦀️.rs | 0d18dfc3c06a322dcf6f7aec1f1111bf65f605bcc7e0a6b3182ffb6215391578 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 11ea9cf535510e18265b4dc34db294a5a2d5370cb4100d7180ca72c9af0bdd07 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📖️reader/🦀️.rs | d147ac3c4f105dfb07f86f28f810578e56eead2b4ef8e1680ef93a5652d39bec |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | cd30764f9c24025a654eea05007fd3f7db3de68b4431799ca86651c4737e1a42 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | af943fed51a19a92dd4d9eacb715ea81dda4dedaf73a94d28f3539f46bc6e444 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🦀️.rs | adba46888e3d19695e0e7b2f8788ccea4b1cda9d7d0e4399d90c403ece62f560 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 7881b3cff4bda044d20827001828997e8b171682ba40734c0f5aa45cf46d8e16 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📖️reader/🦀️.rs | d4bad3efa496f83d2b7372b705ac9a6890f95b531b096cd4b06b467bdf988a7b |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 73e0274b2125b5053d176cdf6067b61fbf345ab6cef40c440169f2c172543733 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 9ca914bac0f2e5ea8aa7fc2506137497e4b7f40fcf3043b482b7f57d9b85b3a8 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs | 1c462d3bb663e23964405beb2441cb849a3e5c5905fa4e24260c2b3db4f8746e |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 5155f3787e85192e8e357a51fababd710c3bae98c5cdd05422520e21c5d0e5b4 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📖️reader/🦀️.rs | bd9a0c35d20d4cf8126003a57009d5277e8a0ee7bbc36a0aafd6cb08dceffe14 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | f8179b336ca1e8dbc413cf6425784b5a5847ce09a99cac68b54484d57dc894fe |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 3a4cdb65e00baeb71d27fdfaf8945329b88deece8373a852f9b6e73d6aa962dd |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/🦀️.rs | 4a1689a9c4ab3152e64a2958a13cd0713d4b8178a19f8f6d916dc99c04d5c3f2 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 583811fed4a8c5c244de1146ea0255e17240fb70fec8c95a09aac81b09860af8 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📖️reader/🦀️.rs | eb3bb6f10c4dd45c49db6d83edfa8a1a10e444e5bd1ac884c3886bfb5261c149 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | c1f0853f1f847f7b9710e0df618f5e761dd22bcac242f459e85136ccc8ce3240 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 53de0b7b7e8cc30905327fa1340560b29da666b70edfac2a779692ff20c46570 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/🦀️.rs | ac61878b0f9966adc0c91f1fac4ebe8a1cf661ff71793fc6124c16be9d43c5d3 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | a681df7c0379cb40974e50d7770a99c0c72563a5d4710daa6388850d8acb0ea9 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📖️reader/🦀️.rs | e30192f26f14709da56fc6e95349642dc0079645a4506dfc8e15d0d18680eef8 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 2498fcd4782af8ac6996bdce89cac44dc0578d01e971e39f15997eb9e673093d |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | b04b93161e30cb5c12595940c4fc1883394f4aa04318d19da155664c24e3e3ee |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/🦀️.rs | be8d89ed741f6bba1617fc245f4366d34dd66206b36863497f87b63cdf5e8c26 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 76903d94f57f99f20a5c8377912bcbef9d530a0216cdf6484dfa0d250e47d002 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📖️reader/🦀️.rs | ceb1391542d4bd623032bd85c526b499c9ae5d7b43469b26031892761d4dd027 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 00d8299f66e8f9e6750f293c6711e47ce42ebaea707c9c03c1194a9b69ef6935 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 12b415c04db58b06223216d9449ebf0934a1f2500aaf6358e4cfd14146d9996f |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🦀️.rs | 993f161b7c5f1c410e0903655c64c56de97e05d6823a2374dd1cc3946b0bc990 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 8d66593352d0f786f887cb3a4336352505507fa88313b549d87ebf6f3b40ba52 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📖️reader/🦀️.rs | a1e792a41d26cc0e19024961ebe916925cb2a25b94ae373eb086f38f33d0a147 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 2ab97c5f213cc8e6d60139529295f2828137274e5b04ff53c16fed28baadc0ca |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | b542854d975383dcf4179514ac18c666fd8082e3cb7bd1d74c8f17a5736e0f5d |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🦀️.rs | 38194791c9262273c4c94eec706d66c54c14c8747d809ce4f6d772d9f46371c8 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | ae646278b9ea915cb91bdb0d99e8d4c0a1a3f890bd11d9cc221de2c03c85e1ec |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📖️reader/🦀️.rs | 5694ec8da962afe5ac9c922280aa9c8bd407a0ac3edc622fb2efbdf896f6afbd |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 7db20829b0f86febb2d1fb0c9f4c9bb5f38e9faaf1a0e518b80c8ad629802af6 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | ecfce6f807923eb9a96148c210156c0c3061757675106541ec8b04cfbe0ad824 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/⚖️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs | bedbfc14bfc3144584dff4433e7a792ae8ee5ae367025ad036ef0177ffe0ed49 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/⚖️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | b3ab3c3b59db26178b4b24b01eea67f80325e448823d560ef1efe0f9696c232f |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/⚖️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📖️reader/🦀️.rs | e1b25820c35e86275151c8284c19f7f88d775a10e48bfc6075a77133b624bb86 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/⚖️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | c68e31cb99575f81854613b2b4d1ea86e6714afd40ef1132d44cc2834a4572f3 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/⚖️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 30778948c514d6a896e88ad83ecfdeb8e4a95e618d3b86bdd9c2b45fd39dfad7 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🦀️engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/🦀️.rs | b9213c04befd98eeb3f0f2da2c1fea0a1fbdfe469e5c72fa8c819b54b9ba3e51 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🦀️engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.toml | a69a7a3cec6603b87c126e65790caa87b94bb245b9eda986f45d3ee0409e1746 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🦀️engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.lock | 55e2c15b54ef9a58cb606b948444ee776ece01cdaf29fdcb7703787117b02f6b |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🦀️lopdf-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/🦀️.rs | 57062fc3e3de4800e3ac8821da2a76ac581afe59ba2f87013e822436209c2a02 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🦀️lopdf-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/🏗️generate/🦀️.rs | 1bef89ab33b6dda46ce66c862a309ddd5d3d05ef27f1e41ffa86fe8056928b37 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🦀️lopdf-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📖️reader/🦀️.rs | 1d58875f7c30f2ad6f6d695f4e5e9800fd7c2e782f619249eec3c5d212a28866 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🦀️lopdf-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | d7c6ed8bb72f82f9b2e098dbce54e8fbb8db8a46a0a58d8186061295d9529e24 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🦀️lopdf-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | efe28a92bed71651f2297f38c97bd6a838609e6fcd763289e036fb08a40c3cd3 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🦀️quick-xml-oracle-codec/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs | ce3a6f951d3d06b343fc2aee43e4a7b09e8c8c68bfa98e7235e37ff7b2c690d0 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🦀️quick-xml-oracle-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | a4ca51b507a4a389cf10746dc98d306e7ab9fa1d4426a51221bdfa53090db7eb |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🦀️quick-xml-oracle-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 1dbfc5fee61adf7cd62ed6b97dfee805e1f53c27b7c09b831527ffa4ba3f0a2f |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🦀️png-codec/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs | 3dc2fa45e6ad432a1a2fe773938e41bd40982cb0c160b1eacc11d7a7d2b95908 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🦀️png-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 270fb1fd6157015beb5748ec804cfbde32e4a706f64b4d70472b6dc21a58958d |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🦀️png-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 1514228ae2abe4a5f47de06532cfb7c178f2861e0920b402003dd9a57fdab9d9 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🦀️jpeg-jfif-codec/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs | 81ee4e438784d34525777c147397fd90597978335bc3ca75e6776716dddfcc3a |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🦀️jpeg-jfif-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | a0caad50a28ef4b5f4f42f1a083440e57eacfd81fc33c365dc1d18339e897423 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🦀️jpeg-jfif-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | b6356beb61f97d095a20ec4606ad70b710e7536df555957dddcea3222e3686d6 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🦀️riff-avi-codec/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs | 5e6b15f984005902ff70720062a45663554ecb3ea127a42c0d2f74fa487e9e03 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🦀️riff-avi-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | f1354b5bf839ec6349c64dfd4f4bf8bdbdb46385422b57496ee40d327a19c5d4 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🦀️riff-avi-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 88bfec3e826b030004900600eddb09e7c34b7a786e2f21fe3d8f405a780e2733 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🦀️engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧫️fixtures/🦀️.rs | ec4513c48daae7e6bb8999165bce29e58fd1ee5e68be35fda16a5de5da4fbb9f |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🦀️engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.toml | bf0e7de32058707824acd670c8e8e417bac24db4d6642fcf03d17790a96e838d |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🦀️engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.lock | 1d3c339d7cfb761e8375a5f28f33a0d00f382ea09a59af7879836c7a432cb1d0 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🦀️tiff-ifd-codec/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs | c8b6edc2824ed928657e576e3b957832476bee34a5fc17f6dc20749a737843f5 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🦀️tiff-ifd-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | adbcde1a538ae452341c3052ff72fba64ece04b8d3c9348f651b1d2a9280bdd2 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🦀️tiff-ifd-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 08bc20bddede75a99b93eabdf95bf5914fe8779f9bcd6d5d8cb9f1b031975df1 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/🦀️byte-order-reader/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/🦀️.rs | 84d01b540d2980866a09add96847d7f8fe03cd7100df93f221c3b65af58558b4 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/🦀️byte-order-reader/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | 7ab4818dcd5233cf11e34c0c90eacaa182dcda99a197a1f490d7bfb28f5f4a47 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/🦀️byte-order-reader/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock | ff7a77655a03625f33b1c62376a3ca0b31adc120f3ef4612eddb7548bdb1675f |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️tobj-obj-reader/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs | 641eef0262e3e8aba2b7bb1862244620ac12d6219a229731d16740c846d5149e |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️tobj-obj-reader/🧪️tests/🔬️src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🧪️tests/🔬️src-unit/🦀️.rs | b71870ba29e29045cca4f4da2dc007905a21f6bec51627078acecf19b8553c2f |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️tobj-obj-reader/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust/Cargo.toml | e58dd59f9134734855919d87339c76e8c4ad403a4c34cf62187bf1611857a1d1 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️tobj-obj-reader/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust/Cargo.lock | c689b479cb850b4338fc966c2587517defc9542c4724aa8527429242cb9cefbd |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🦀️engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧫️fixtures/🦀️.rs | a387cdf114822b146847e68f44931534a005ae8da90ac187e4c219c66b3d00c4 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🦀️engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.toml | 004265365b7399a65ff09e46a4378debfdda6b867d6dd6126a4fcd2182653f4c |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🦀️engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧫️fixtures/📦️packages/🦀️rust/Cargo.lock | b0fd5ed8ede49ea9dc6c89b61f2edf77a0d8f0a374bfb2c88695273f0d5cced3 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/🦀️resource-reader/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/🦀️.rs | f4a7dc51adb991bcd598bbf2294283a07c1928a9c29b9e6c6f284ba747934d08 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/🦀️resource-reader/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.toml | 9a7d154a9be7471990a5538da76a14d2e407ead04e14d2d8e2e43d277601a66e |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/🦀️resource-reader/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock | 6f4dd856b883be6145cc04a51771327b1c077a274749c3eb14675b167b14d6f8 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🦀️serde-json-engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/🦀️.rs | 7c62b03749118a69231b040d2697ca9d21b5dbf438608e4ca841a688814f80fb |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🦀️serde-json-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 60866972ab0686f9a449139bedd65e6672dd21123a08afc2f987b0c770a20e9c |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🦀️serde-json-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | ede8ece7d4dc401e5e97487d1711d071bca5b7f4055298c0ad715d15107cb1d6 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/🦀️json-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/🦀️.rs | e6bb06eb316e316c2afd4cc29d529200e38cae3fafbb2775a9e6ce833deecf21 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/🦀️json-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/🏗️generate/🦀️.rs | e5d9c244b5b5f423fad24386559d91635653aa096fca78aff50bda181be345d5 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 7438043ea9144e7b7f4c2a80d1fe42e9f95fc108dbb03047681df8b3d12730c3 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | ca145e2644d92db747155c483c03e143b1cd1e231fa7f1cd687c5ac7ab6804f2 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/🦀️oracle-probe/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/🔮️oracle/🦀️.rs | 6dd24cd1e9a106f5c704fd1e214b4e68bc7420b22fa2326f79aa8960a3b9e117 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/🦀️oracle-probe/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.toml | 7bb67f8af7dcd4a3ee019a18ed94a2c9836e6b11d85357a04cfe07d7a2ad853c |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/🦀️oracle-probe/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.lock | 3c54caa76259e5533c07dbfaae8d8de77332ccabf5a007abbf5d208c1f490d2e |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🦀️json-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/🦀️.rs | f5c7237cb664a43defc27a638e8515d0ad800ae10a228092b881ed2b06748e4e |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🦀️json-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/🏗️generate/🦀️.rs | e1795e877b871bc83f98aec314934ba4116b91bcadc4297716deb17c18124d47 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🦀️json-engine/src/📖️reader.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📖️reader/🦀️.rs | b44b159ac7176d4701044a4ce290f519267c111fa9c925efb64a16224fda9bb6 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 77efcb9dc689c4080136906a440c3ec3a4c2254845182fa059cf83a909af4f2f |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | 0edba09bc4e6ae8e3c4cb31eeb9004362c632d6ac598922f4637c6e12f4aa0e7 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🦀️json-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/🦀️.rs | 23dc3c33620c891bd9aed48670c946a48102b6c0bd8499f2103ba86fbe6d65e8 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🦀️json-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/🏗️generate/🦀️.rs | 5f71ab6e6f4a0f5cadd9c0604821afc51dace7421d66edf3f6c4e7b07ac830a2 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | a32c55633d50789caf9d16c4d4220f0f02cdbe56196940e413c4beebd91ca1d6 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | 6f4bf31d5056ff340fd801061fc78649dd226b0b60766ebde43dac6a737a435a |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🔣️json-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/🦀️.rs | 70974e4b1ba7b8056e141e93d3af94004077831efa5ded383d04d6204798822c |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🔣️json-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/🏗️generate/🦀️.rs | d05a4f4b3f25155e72642bbeb721d2a50a19b935ffd6cd2381866eaf8d2b4f81 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🔣️json-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | edccf386db70eda1d07a608a79e3c6022ba15a946cfd017e48a33bbdbaed1f3e |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🔣️json-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | f64f8afc19e2eb6ee5174f21828112f40e41e80f71a094ec20308cf894c0c0a5 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🖼️svg-engine/src/main.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/🦀️.rs | d65098db61c369945e6a3b1583f3be98b1676d3bbd6ee62f4c710e6492842323 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🖼️svg-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | 669fcdcca53b692eac072eaa4ca8c463cf07e293100a80347c6aa5a67399f48c |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/🖼️svg-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | ebd7f6df754ce256566ca304bba7572254c6184ce83b810e0ac360b302344c9a |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/🦀️oracle-probe/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/🔮️oracle/🦀️.rs | 0119bf0ccf17be3bfd3e3cf64f29c7aa5683e92ede94a9308d47346d508ee07a |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/🦀️oracle-probe/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.toml | ff17ec20b41c897df95ac67900f12749cfd1c6f83a07c0aa13e22f71eb77c3e5 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/🦀️oracle-probe/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/🔮️oracle/📦️packages/🦀️rust/Cargo.lock | 57a5f0e14fda184950923a4d7655bc9b6389aab76515d548a5ef4ecb305f3b2c |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🦀️json-engine/src/lib.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/🦀️.rs | 27c973dcb8bd831b42522fb5b92479d5a153ab45dc8c87eae016c74a283dcd64 |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🦀️json-engine/src/🏗️generate.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/🏗️generate/🦀️.rs | 7ad5f3d356bad4e8ba82da7a459827e71b28ba684151edb1226bd4abdf858820 |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🦀️json-engine/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml | 102ad0ab0ce9824cde80ea986409d3aef8679b516ed64f417dc62ef3ba4d3bac |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🦀️json-engine/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock | ee69cba51a6221a783d0062d4f8715957f592f6e104b84b85c8e24528c32610c |
| source | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🦀️image-bmp-codec/src/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs | ac99dda9eeb823ee46dda363560dcca61ad5c09a19d6cd5cf652e5e1ec21433b |
| manifest | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🦀️image-bmp-codec/Cargo.toml | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | cbdfbe0dafece0e774e1960e0da3cd49043a68982e49928201f0ef0270ef9b22 |
| lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🦀️image-bmp-codec/Cargo.lock | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 15af7b5a4d1d3d87ab8e1b42551b7cbea2d7b91d98329e2bdc611931e350519d |
| source | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/⌨️cli/🦀️.rs | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs | 439e26581171cdb398ef7ace8808f464bd1d3ff88eade04314221424201cef31 |
| source | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/📕️pdf-codec/🦀️.rs | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📕️pdf-codec/🦀️.rs | 611d52a6edbb5da1e44c1c27615d21ef8f0c8a41f846999cc3fd88bef0c31e19 |
| source | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/🔣️svg-codec/🦀️.rs | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/🔣️svg-codec/🦀️.rs | a509e6972139dcae65724be44e50419d0fcc679ef793d1ab6d50a0bcd1dae04b |
| source | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/🖊️dxf-codec/🦀️.rs | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/🖊️dxf-codec/🦀️.rs | 13825fdf028a1960ee970fbcb24d038e294fd925a2ad291320b273a9c33c31b8 |
| source | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/🦀️.rs | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs | 6eb8c0aa1c92a26787542b8ec4f719b3916c5465e3eb31a1080e7ded04a3cd68 |
| source | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/🧫️recipes/🦀️.rs | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/🧫️recipes/🦀️.rs | d4ab4d96d053ec5b53dc5eb5d5411126152f019efafe9799d2c9f2de16179074 |
| manifest | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/Cargo.toml | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml | f54d5c03e5fb7eb64e7ea256f7a22092b77dc0093f45c3f6ecd50814bba627d7 |
| lock | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/Cargo.lock | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock | 672c37fc2a11106c5a10c9dd6d513eb52219d100277f67a6f1fb1ca0f153f7b0 |
| source | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/📦️lib.rs | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/🦀️.rs | 8dbfc6c1761851946451d0de20eea242183c0219ed76b46e1267d5cbe2dd10fd |

### Tests moved below their semantic implementation owner

| Old path | New path | Move SHA-256 |
| --- | --- | --- |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🔬️engine-src-reader-main-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📖️reader/🧪️tests/🧪️unit/🦀️.rs | f06e2d270237da401e3438b237a889c766a3edb0a4c88e148135b6f4e31dfa95 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🔬️engine-src-reader-main-process-local-determinism/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧫️fixtures/📖️reader/🧪️tests/🧪️process-local-determinism/🦀️.rs | 12d1f7de0f19a9d4d59aa9def5fd11132981f2e58b78c8565d4c81ecfcfd4971 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🔬️quick-xml-svg-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | b2be9cdc8c3c601e02da5492413ecf2bef680ee5a4af13cdab6800877f1b52b9 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🔬️quick-xml-oracle-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | 68c3ae7e6786442529a076fc60436c23d119881ddd33e0467644ff6d2e3940cf |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🧪️tests/🔬️png-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | 1a7b0a93674be474a54b65a10a1c44f59a8df9b4c03f10065237c7cb60adfdbd |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🧪️tests/🔬️jpeg-jfif-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | ddfe0b32e14e26f7f45b2d98636b29b521a93c7181c8dd991546434a2699b01d |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🧪️tests/🔬️riff-avi-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | 6b7ceedde782e293a0a2aae2ae8bf70213ea930c50ebc875bd90ac3f422d82ed |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🔬️engine-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧫️fixtures/🧪️tests/🧪️unit/🦀️.rs | efaf5b3299e29386269439edfce2b36c13c75a0afdccccf316d72e1b1d8ae9d2 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🧪️tests/🔬️tiff-ifd-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | c429c16bb7cf6a6e5568e7ba5b6ea57562e2e889cc8b266ab56b63943af89fa9 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🧪️tests/🔬️image-bmp-codec-src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🧪️tests/🧪️unit/🦀️.rs | 8e1a3cfe1ba99651006226491eefd1a96ffbefefa2ade3d112ce969c6988550e |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🧪️tests/🔬️src-unit/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🧪️tests/🧪️unit/🦀️.rs | b71870ba29e29045cca4f4da2dc007905a21f6bec51627078acecf19b8553c2f |

### Shared stdio oracle package body

| Old path | New path | Move SHA-256 | Path attributes rebased |
| --- | --- | --- | ---: |
| ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/🦀️.rs | ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🦀️.rs | 8dbfc6c1761851946451d0de20eea242183c0219ed76b46e1267d5cbe2dd10fd | 77 |

The stdio hash equals the original 📦️lib.rs pre-move hash. Its 77 non-dot path attributes were then rebased to the same referents; the 301 stdio path attributes and the broader 441-referent census all resolve.

### Canonical generator collection spelling

| Old path | New path |
| --- | --- |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/⭕️step-line-circle/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/⭕️step-line-circle/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/📋️project.json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/📋️project.json |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏗️generator/🖊️dxf-entities/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🖊️dxf-entities/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🎨️styles/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🎨️styles/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/📋️project.json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📋️project.json |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/📜️document/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📜️document/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🖋️runs/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🖋️runs/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏗️generator/🧱️blocks/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧱️blocks/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/⚠️degenerate/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/⚠️degenerate/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/📋️project.json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/📋️project.json |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/📏️scale/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/📏️scale/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🔷️primitives/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🔷️primitives/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🕸️topology/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🕸️topology/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/🧮️booleans/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧮️booleans/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/📋️project.json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/📋️project.json |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏗️generator/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/📋️project.json | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/📋️project.json |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/📍️move-vertex/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/📍️move-vertex/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🔄️geometry-replace/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🔄️geometry-replace/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🧮️booleans/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧮️booleans/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🧱️topology-build/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧱️topology-build/📜️script.ts |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏗️generator/🧹️topology-remove/📜️script.ts | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧹️topology-remove/📜️script.ts |

## Reference closure

- No active old package-root path or old external-library/source test directory name remains under .vscode, ✏️s, or 🧰️framework.
- No live 🏗️generator directory remains. The only retained spelling is the portable fixture value that asserts it is obsolete.
- All 215 logical old paths are absent and all final destinations are present.
- All explicit Cargo target paths, dependency paths, Rust path attributes, literal include paths, producer working directories, oracle executable paths, descriptors, feature references, and generator project roots were updated in place.

## Generated output handling

All lane diagnostics, Nx graph data, metadata JSON, and compile scratch lived under the ticket 🗑️generated/sol-rust-generators directory. The directory is disposable and is removed after this report is written; this Markdown report is the retained evidence.
