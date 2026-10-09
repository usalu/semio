# Directory Checkpoint Pair Native IO Ownership

The production binary bodies now belong Directory io::binary::checkpoint_pair. Rust PairCursor, framing constants, record-count helper, decoder and SHA256 verification are physically in the IO module. TypeScript CanonicalPairCursor, constants and decoder are physically in the same IO leaf. Schema retains CanonicalCheckpointPairV1, named refusal type and pure admit/admit_rebootstrap methods; TS retains the corresponding type and pure admission functions. No schema codec reexport or alias remains.

The real native client imports the explicit IO decoder/constants. Worker TS and GIS composition witnesses import that same IO leaf. Root's concurrent Directory JSON IO extraction is preserved: one bottom Directory IO mount, text and binary namespaces mounted together. The native unit laws moved with the body; their authored fixture include coordinate was corrected. The new actual native module_path law identifies os_directory::io::binary::checkpoint_pair, but it is not yet executed.

Neutral owner contract was authored before extraction. Registered source target receipt82496 actual RED32pass/1fail: schema PairCursor ownership. First post-extraction receipt78361 actual RED31pass/2fail exposed two test-reference defects: constant-array entries removed by import rewrite, native law sought in parent instead of physical test module. Both corrected. Source3 receipt3551 actual GREEN exit0: strict TypeScript,33tests58expects, DEBUG explicit owner witness. Existing independently authored framing corpus, node:crypto pack/SPR/aggregate SHA256 witnesses and independent Ajv scope/frontier/owner validation remain active.

Native target is registered and selects the real moved test module. Native execution remains parent-held on shared OS receiving errors; no native pass is claimed. Full artifact IO goal remains open. Logs: generated/space-history-io/directory-pair-source-red.log, directory-pair-source-green.log (failed first post-extraction), directory-pair-source-3.log (GREEN).

## Exact Scoped Source Manifest

Created:
- Directory 🚪️io/🧱️binary/🦀️.rs
- Directory 🚪️io/🧱️binary/🪢️checkpoint-pair/🦀️.rs
- Directory 🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts
- Directory 🚪️io/🧱️binary/🪢️checkpoint-pair/🧪️tests/🔬️unit/🦀️.rs
- OS 🧫️fixtures/📇️directory/🪢️checkpoint-pair-io-owner-v1.json

Updated shared modules, latest-read scoped:
- Directory 🦀️.rs (pure pair type reexports; one actual IO mount retained)
- Directory 🚪️io/🦀️.rs (binary and root's text mounted)
- Directory 🧬️schema/🦀️.rs (pure pair types only)
- Directory 🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs (pure types/admission)
- Directory 🧬️schema/🟦️.ts (native pair bodies removed)
- Directory 🔌️client/🪢️canonical-checkpoint-pair/🦀️.rs
- Directory 🔌️client/🧪️tests/🔬️unit/🦀️.rs
- Store 👷️worker/🟦️.ts (three physical pair imports only)
- OS 🧪️tests/🪢️canonical-checkpoint-pair/🟦️.ts (Bun owner/source route plus independent authored corpus)
- GIS 🧩️service-composition/🧪️tests/🟦️.ts (two dynamic decoder imports only)
- OS Rust package 📜️script.ts, 📋️project.json, package.json (two permanent scoped commands)
- .vscode/launch.json and .vscode/🧩️launch.seed.jsonc (source/native GUI rows)

Removed:
- Directory 🧬️schema/🪢️canonical-checkpoint-pair-v1/🧪️tests/🔬️unit/🦀️.rs (moved actual law body)

Directory prefix: 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory. OS prefix: 🧰️framework/🛍️products/💻️os. Rust package: OS 📦️packages/🦀️rust. GIS prefix: ✏️s/🧑‍💻dev/🎭️variants/🌍️gis. Shared manifests are coauthored with root/General; attribution here identifies only scoped edits.

Follow-on pure admission closure: native/text selection hashes are now admitted explicitly in IO before pure typed comparison, including rebootstrap array equality. Pair Source4 session17630 strictTS33tests58 assertions is actual GREEN; new semantic selection schema/runtime/native witnesses described in `directory-hash-typed-admission-current.md`. Native remains pending.
