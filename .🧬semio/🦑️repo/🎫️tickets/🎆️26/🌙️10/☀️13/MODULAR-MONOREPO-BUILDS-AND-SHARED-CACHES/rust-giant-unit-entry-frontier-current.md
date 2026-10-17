# Rust Giant Unit Entry Frontier

Source-only audit on 2026-10-16. No compiler, Cargo metadata, tests, Native, provider, cache, browser, Git mutation, or Source edits. Parent owns ticket lifecycle. This report is the sole authored artifact of this audit. Applicable root AGENTS and product/OS/3D AGENTS were read; no deeper AGENTS existed in the inspected plugin/mesh subtrees. The prior census was consulted for the top three candidates; it is not compilation evidence.

## Actual Target Entries and Mounts

Plugin manifest `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` names `semio-framework-plugin`; its `[lib]` points at the package root. Package root lines 33–34 mount `../../🦀️.rs` as `component`. That component lines 106–107 unconditionally mounts `plugin_runtime`; runtime lines 79–80 include the builder-contract file under `cfg(test)`. Consequently the giant builder contract belongs to the facade library test source path, not a standalone test crate.

The same component line 90 includes the WP7 hub. Hub lines 37–39 mount `app_testing` under `cfg(any(test, feature = "artifact-app-testing"))`; testing lines 214–216 mount the time-travel test under `cfg(test)`. The distinct manifest `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/Cargo.toml` points to a 3-line placeholder root, not to this giant test: its existence does not partition the facade test source.

3D manifest `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` names `semio-framework-3d`; `[lib] path="🦀️.rs"` at lines 15–16. Package root lines 22–23 mounts `mesh`; mesh lines 14–15 mounts modeling, both without cfg attributes. Modeling lines 17–21 mount snapshot preparation and input; lines 13–15 mount jobs tests under `cfg(test)`. This is a production source entry path, subject to successful parsing/resolution; it is not a separate modeling crate.

## Conservative Source Lower Bounds

|Path|Distinct Files in Proven Chain|Bytes|Over 262144 Bytes|
|---|---:|---:|---:|
|Facade builder contract test chain|4|944688|682544|
|Facade time-travel test chain|5|537233|275089|
|3D modeling production chain|3|354249|92105|

These are authored UTF-8 source-byte lower bounds for explicit chains, not cfg-selected compilation closures, actual compiler input byte totals, active-line counts, memory estimates, timings, or evidence of build failure. The test paths are conditional and are not mandatory for a normal library build. Only the 3D chain has no observed cfg guard. Each giant file alone exceeds 262144 bytes. None of these small chain file counts proves the 32-file limit is exceeded. Newline and nonblank-line counts do not establish the 4000 active-line metric. Metadata limits in plugin manifests are 262144 bytes / 32 files / 4000 active lines; the 3D manifest has no authored rust-source-boundary block, so comparison there uses the parent-specified global maximum.

## Decomposition Frontier and Existing Incidence

Recommended first owner: plugin runtime contract tests, with independently owned crates for fluent registration/factory contracts, typed operation delivery, history presentation, and mounted ownership authority. These are genuine behavioral boundaries already visible in the tests; simply moving text into more files mounted by the facade would preserve the giant unit. Private facade imports mean crate extraction requires an intentional owner/test interface before moving laws. Do not infer that existing placeholder packages accomplish this.

Builder file `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` defines `plugin_builder_contract_tests` at line 2 and imports `crate::builder::PluginBuilder` at line 1. Existing tests: `plugin_builder_builds_bundle_from_fluent_spec` line 4335, `plugin_builder_wires_app_factory_for_create_app` line 4349; history panel laws at 7285, 7363, 7403, 7487, 7531; mounted original authority laws at 10578, 10592, 10607. Shared helpers and broad crate-private incidence make a blind textual split unsafe.

Time-travel owner: plugin history-edit acceptance/replay laws. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` lines 1–6 explicitly tie the Rust laws to a language-agnostic JSON fixture and independent TypeScript reducer twin; fixture include is line 13. It imports `super::*` and facade-private mutation fixtures at lines 8–9. Existing long remote replay/cancellation law is line 2083, interior undo cancellation law 2224, hostile input law 3323. Extract public acceptance laws into dedicated history-edit law crates, and keep owner-private assertions with their actual owner; do not route all new crates back through one facade test target.

Mesh owner: retained mesh modeling jobs. `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` Work enum begins line 48 and enumerates algorithm families; a suitable crate boundary is topology/document primitives versus retained modeling algorithms, with algorithm-family crates only where independence is established. Current `use super::*` at line 3 and mesh root re-export at lines 16 prove coupling to mesh types. Existing jobs test mount is modeling lines 13–15; root exposes `MeshModelingJob`, progress, step, and translation types. Snapshot and input are already separate files inside the same unit, not independent crates. Further algorithm extraction requires explicit dependency direction before implementation.

## Evidence Fingerprints

|Source|Bytes|Newlines|SHA-256|
|---|---:|---:|---|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs`|1939|37|`5fd02bc5244881b9e503f8f8fe868da845c68251ed520a149480a339c0ec482c`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|43693|803|`5440d06bc46f2a293b6a5a8b69806c8c41732a280b569ee693ab48a16d2e47b1`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs`|5291|80|`0c7a7253a0c20a24494c832d087583a317bc1850a94a22a77b4b2958174a00f1`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`|893765|10650|`2a418e4cdb81ce3c41de2d8630541926a5d14b487221da4a1a3b48d526dab7d0`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs`|2062|40|`62d71b0f93c523e1b32d900eb8ec7b927511dd10d578372b4a7c6a5e51ecff3f`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs`|195092|2746|`440652b146ac7c7886734a4bc5c03ce5d60b520bb46c869305c6a48dc536e62f`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`|294447|3393|`fa79cd5f55b7384a09b98c4da5f969a4141a316606004a197087ebbf2f0e0442`|
|`🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/🦀️.rs`|2780|51|`d38341655189263a4ef404e59f3c898f7a2b0f4dd83e574e6683ef0faf66702f`|
|`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs`|84644|1519|`c7a96616bda4130ba6258d32d738527d65df07a9bf0b3f19e2fdc452f7f5bede`|
|`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs`|266825|2462|`f2b0c1daae44e03c1724fa9cc18a723a5f51672a4b72100d9d7e88b6ab6d8b96`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`|22234|216|`0526a84112bc547e3dab01b55b1ab207bb5dd761134908a2f347476391fce84a`|
|`🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml`|1746|37|`e46eea67abd345124b467c120603683a7e103a1bdc2dadbb1ccc0a06ba12932f`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/Cargo.toml`|1066|31|`d98e7e501f528509ba2d6caa8b5f52c2f51b294506217caafd57f30ece14f134`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/🦀️.rs`|335|3|`ff3197bbb1e97c5535fc40df8f17d4a5be02806f1133f58d0708a2c3640d0b30`|
|`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🧺️snapshot/🦀️.rs`|5599|57|`db61aaaa6422dcf8e210084a9ed1df1c7baf91ce7ab6f2a32a6ccec89e08c73b`|
|`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/📥️input/🦀️.rs`|4065|59|`74aada70ffb2f0e33b74afc7377218567c3a60bb5af8d2f93d7169fd33244c19`|

## Stability and Limits

Before/after re-read: all inspected bytes and SHA-256 fingerprints remained identical.
Bounds: only the three candidate source chains, direct mount files, matching manifests, and local declarations were inspected; no recursive module closure, macro expansion, target graph, or caller completeness was asserted. The earlier whole-file report read exceeded output budget and was truncated; only its opening census/candidate rows were relied on. All reported source sizes are actual filesystem reads, not census assumptions. No passing tests or confirmed runtime behavior is claimed.

## Bounded External Caller Incidence

Literal call-site inspection in plugin Rust sources found the following representative production and test incidence. This is not a complete call graph; no code ran. The retained job API is already consumed across logical domains, so extraction must retain a domain-neutral modeling API with callers updated together.

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs:879`: `mesh.bevel_job_owned(&edges, positive(input, "amount")?, count(input, "segments", 1, 64)?).map_err(mesh_error)?`. Before/after SHA-256 `90fe4595ac875ad4dc63b8d221bb6eb71552f1daf35ceb4e4f596b3389ceb06d`; stable=True.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs:255`: `for mut job in [source.mirror_job(MirrorAxis::X,0.0).unwrap(),source.subdivide_faces_job(&[FaceId(0)]).unwrap()] {`. Before/after SHA-256 `fd9f38f61b290d7a04deee015b91e3da01deae3ad769bd35c9b80f88c3a377bd`; stable=True.

Modeling family declaration evidence: `Work` line 48; `Mirror` line 1162; `Subdivide` line 1186; `Bevel` line 1904; `Decimate` line 2162. Public retained job constructors occur at lines 443–506. These give concrete family ownership candidates, not proof that each is already dependency-independent.
