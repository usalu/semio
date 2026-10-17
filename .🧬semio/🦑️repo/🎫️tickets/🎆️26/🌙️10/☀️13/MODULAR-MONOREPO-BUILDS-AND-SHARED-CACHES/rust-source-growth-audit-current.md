# Rust Source Growth Audit

Read-only default-visible framework Source inventory at 2026-10-17T06:39:54.510Z. Command: `rg --files --glob '*.rs' --glob Cargo.toml '🧰️framework'`. It observes 6021 regular Rust Source files (86240073 bytes) and 874 parsed package manifests. This is not a complete monorepo/tracked inventory, selected cfg/compiler input closure, compiled crate size, output footprint or build result. Hidden/ignored sources are outside this rg inventory. No Cargo/rustc/metadata/Native/cache image command was run.

Do not assign files to crates by nearest physical manifest. This taxonomy often places explicit library entries outside their package-manifest directories. Only manifests whose declared/default library entry exactly matches a file appear in its table entry. Ownership of other modules and active bytes is not inferred. Genuine selected compiler Source admission is required before crate-boundary/build claims.

The first Python attempt terminated before input reads because system Python lacks tomllib. A default-root Bun subprocess inventory also failed on its nonzero rg outcome. The completed inventory explicitly selects framework and uses the existing @iarna/toml library without installing a dependency.

## Largest Observed Source Files

| File | Bytes | Absolute UTF16 Path Units | Direct Library Manifests | SHA256 |
|---|---:|---:|---|---|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs | 866100 | 137 | No direct entry match | 486f2182be622966a0bf9209fae4392ba0ece7ff211220d70393f6c209fd8624 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs | 294447 | 110 | No direct entry match | fa79cd5f55b7384a09b98c4da5f969a4141a316606004a197087ebbf2f0e0442 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs | 223502 | 92 | No direct entry match | 9c064d2524aaf684d6c9307544bb3d43ec1de48ba48175e198237fe253ed136c |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs | 222788 | 92 | No direct entry match | 647b49df933a5befe612aed152522fbbe2573e2db77472c1cb1ff4f2e87809dc |
| 🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🔬️jobs/🦀️.rs | 219238 | 89 | No direct entry match | b3588d9437685fa76675839ba991f5b22110e16ee2f3ea58dd5c841ffec6d619 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs | 210127 | 107 | No direct entry match | d6cb51e73a52269a3e96f8d17917b6b2e74bbba9cc740311eaa0f4266351bfdb |
| 🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs | 204502 | 101 | No direct entry match | ef6f5d7d3661f34477bdc8b46d7168f7d507d45421f312649bd798ce17d73fe6 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs | 199151 | 110 | No direct entry match | 17c287b49e36798c799e72c656894af55bf4c347fcf3fe0235dadfbe6b8fc403 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs | 195092 | 150 | No direct entry match | dbe0692a1de852333d153cd84d68b139fed4b641f80733aa8c94b7ffb5a4bb51 |
| 🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs | 190379 | 84 | No direct entry match | 3065304abfab65daaaa01e1acd59179bb9a0f14e3d68932b6cc703555d70b3af |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs | 188136 | 92 | No direct entry match | 51cb6bbdaf6272c3d1c86b7a690eeabfad262ef9849194c85307d38a9cba6a60 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs | 187037 | 121 | No direct entry match | cea49f6bef816c0b1acb6d44ac02ef9fde0ca143ea0541177e8c44a573055ba1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs | 183932 | 162 | No direct entry match | 2b1d4450841e102c88343d7912cdfdb792c59b43b76b4364f962d20f4f9c9966 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🦀️.rs | 182146 | 112 | No direct entry match | c4dbcac513934e7ba7b425b9ae6da7fa3dafcf16577aa94038be02f7f458204a |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs | 181888 | 141 | No direct entry match | 91f4bb94f113ba1ee183215946dc8f8fdff3bb039627519d7eac4ff503bcaeab |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs | 180511 | 121 | No direct entry match | fe54267f739fa84bc5d68f9824a037cc608d07a340df22c7af769b490ccba580 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs | 179807 | 97 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/Cargo.toml | 0d4eba2b8087541a2b71af71bb3d24381f71326f0ce1b7208d56ae285a55db33 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/🔬️unit/🦀️.rs | 174356 | 112 | No direct entry match | a93ca414915f973111bfb478b06f1d54a9c84410ce43e9c13ad8f58efde1c5ca |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs | 174032 | 113 | No direct entry match | 65021ed08c3e9046b39bd32049f09c74779d27bae79fbfa2de1562f896cc0f05 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs | 172658 | 110 | No direct entry match | b26ae6a0dc11da2713fab8ef6c46804a665fe7e47023c861937976c460a0cf9e |

## Declared Boundaries for Large Direct Entries

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/Cargo.toml: `{"maxSourceBytes":262144,"maxSourceFiles":32,"maxActiveLines":4000}`

Observed paths exceeding256 absolute UTF16 units: 0. Skipped Source symlinks: 0. Unavailable/changed/invalid rows: 0.

## Next Work

Follow actual direct library roots and cfg/module closure for the largest entries. Continue splitting active facade/builder and fixture ownership seams into genuine bounded domain crates, updating explicit manifests/dependency locks/tests together. Frontier mismatches in plugin-surface-fixture-api-frontier.md and physical Rust compile qualification remain open. A byte inventory cannot certify that any crate is small or recompiles correctly. Preserve private Native/Original and global SDK/publication holds. No Source, manifest, cache, Git state or policy was modified by this audit.
