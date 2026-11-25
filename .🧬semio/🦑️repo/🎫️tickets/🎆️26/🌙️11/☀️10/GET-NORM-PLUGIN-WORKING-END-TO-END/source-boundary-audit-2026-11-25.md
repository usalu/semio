# Source boundary audit 2026-11-25

Read-only reproduction of the Rust source-boundary count from `readRustUnitSourceClosureV1` and `readRustSourceClosureV1`: files and bytes of the closure reachable from each unit entry, admitted before the compiler against 32 files and 262144 bytes.

Counting rules reproduced by `source-boundary-audit.py`:

- The entry is the lib, each bin, each integration test and the build script (`build.rs`). Each unit is counted on its own.
- Closure: `mod x;` resolves to `x.rs` or `x/mod.rs` (or `#[path]`), inline `mod x { }` moves the module base, `include!` is parsed as source, `include_str!` and `include_bytes!` count as files and bytes without being parsed.
- Bytes are whole-file sizes, not active bytes. Files are unique paths. Items gated by a false `cfg` (including `cfg(test)` for build units) are dropped with their `mod` and `include!`.
- Build units use cfg `macos` debug without `test`. Test units add `test`. Each unit is counted twice: with default features, and with all features (`+all-features`). Dependents enable features such as `test-support` through feature unification, which the default pass cannot see, so the all-features pass is the upper bound.
- Not counted: `maxActiveLines` (4000), `env!`/`concat!`/`OUT_DIR` generated inputs, and files that no `mod` or `include!` reaches.

Scanned 1051 manifests with a `[package]` section. Crates listed below are above 85% of either limit, ranked by the worse of the two ratios.

Over the limit: 9. Above 95%: 16. Above 85%: 32.

| Rank | Crate | Manifest | Worst unit | Files (of 32) | Bytes (of 262144) | Worst % | Build % | Test % | Issues |
| ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | semio-s-artifact-norm-en1992-checks | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 25 | 336,545 | 128.4% | 83.3% | 128.4% | 0 |
| 2 | semio-s-artifact-norm-en1993-checks | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 21 | 312,208 | 119.1% | 62.0% | 119.1% | 0 |
| 3 | semio-s-artifact-norm-en1994-editor | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 38 | 185,326 | 118.8% | 68.8% | 118.8% | 0 |
| 4 | semio-s-artifact-norm-en1998-schema-formulas | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📐️formulas/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 27 | 299,076 | 114.1% | 99.2% | 114.1% | 0 |
| 5 | semio-s-artifact-norm-en1995-kernel | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️kernel/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 18 | 298,301 | 113.8% | 92.5% | 113.8% | 0 |
| 6 | semio-s-artifact-norm-en1993-editor | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 36 | 143,972 | 112.5% | 65.6% | 112.5% | 0 |
| 7 | semio-framework-pack-json | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 27 | 294,031 | 112.2% | 98.3% | 112.2% | 0 |
| 8 | semio-s-artifact-norm-vdi3805-inferences | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 35 | 229,466 | 109.4% | 81.2% | 109.4% | 0 |
| 9 | semio-plugin-reactor-turn | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)] +all-features` | 26 | 275,098 | 104.9% | 100.8% | 104.9% | 0 |
| 10 | semio-plugin-instance-docs | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 18 | 260,673 | 99.4% | 97.8% | 99.4% | 0 |
| 11 | semio-s-artifact-norm-en1997-checks-catalogue | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧬️catalogue-core/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 21 | 260,304 | 99.3% | 77.4% | 99.3% | 0 |
| 12 | semio-s-artifact-norm-en1999-scene | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 15 | 256,257 | 97.8% | 53.0% | 97.8% | 0 |
| 13 | semio-s-artifact-norm-en1994-scene | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 14 | 255,032 | 97.3% | 71.0% | 97.3% | 0 |
| 14 | semio-s-artifact-norm-en1993-model | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 5 | 254,861 | 97.2% | 97.2% | 97.2% | 0 |
| 15 | semio-s-artifact-norm-en1991-mut-rail-load-groups | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️rail-load-groups/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 31 | 47,454 | 96.9% | 96.9% | 96.9% | 0 |
| 16 | semio-s-plugin-norm-app-surface-live-editor | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🎛️live-editor/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 16 | 253,052 | 96.5% | 86.5% | 96.5% | 0 |
| 17 | semio-s-artifact-norm-en1993-schema | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 5 | 248,017 | 94.6% | 94.6% | 94.6% | 0 |
| 18 | semio-plugin-decl-app | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 27 | 245,465 | 93.6% | 93.6% | 93.6% | 0 |
| 19 | semio-s-artifact-norm-en1993-checks-core | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧬️core/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 16 | 238,484 | 91.0% | 86.5% | 91.0% | 10 |
| 20 | semio-s-artifact-norm-en1998-model | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 7 | 237,522 | 90.6% | 90.6% | 90.6% | 0 |
| 21 | semio-s-artifact-norm-en1998-mutations | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 7 | 237,419 | 90.6% | 90.2% | 90.6% | 0 |
| 22 | semio-s-plugin-norm-app-surface-panels-inspection | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧾️panels/🔎️inspection/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 27 | 235,511 | 89.8% | 54.6% | 89.8% | 0 |
| 23 | semio-s-artifact-norm-en1997-scene-core | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧱️core/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 18 | 233,075 | 88.9% | 88.9% | 88.9% | 0 |
| 24 | semio-s-artifact-norm-en1996-scene | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 22 | 230,185 | 87.8% | 59.1% | 87.8% | 0 |
| 25 | semio-s-artifact-norm-din16798-mut-zones-more | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️zones-more/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 28 | 36,313 | 87.5% | 87.5% | 87.5% | 0 |
| 26 | semio-s-artifact-norm-en1992-mut-member-shear | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️member-shear/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 28 | 44,390 | 87.5% | 87.5% | 87.5% | 0 |
| 27 | semio-s-artifact-norm-en1993-mut-foundation-block-grout | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️foundation-block-grout/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 28 | 40,468 | 87.5% | 87.5% | 87.5% | 0 |
| 28 | semio-s-artifact-norm-en1993-mut-joint-plate | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️joint-plate/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 28 | 38,291 | 87.5% | 87.5% | 87.5% | 0 |
| 29 | semio-s-plugin-norm-structural-scene | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 28 | 195,048 | 87.5% | 56.2% | 87.5% | 0 |
| 30 | semio-s-artifact-norm-en1991-scene-members | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene-members/📦️packages/🦀️rust/Cargo.toml` | `build:lib` | 16 | 228,433 | 87.1% | 87.1% | 87.1% | 0 |
| 31 | semio-s-artifact-norm-en1995-scene | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 12 | 226,215 | 86.3% | 36.7% | 86.3% | 0 |
| 32 | semio-s-artifact-norm-en1993-mutations | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | `test:lib[cfg(test)]` | 8 | 223,297 | 85.2% | 76.5% | 85.2% | 0 |

## Read against the build

- Build % is the worst build-set unit (lib, bins, build script). Test % is the worst test-set unit (lib with `cfg(test)`, bins, integration tests). The build set is what a plain build admits. The test set only matters for test runs.
- Over the limit in the build set: only `semio-plugin-reactor-turn`, and only with all features enabled (100.8%). Every other over-limit crate is over only in the test set.
- Above 95% in the build set: `semio-plugin-reactor-turn` (100.8%, all features), `semio-s-artifact-norm-en1998-schema-formulas` (99.2%), `semio-framework-pack-json` (98.3%), `semio-plugin-instance-docs` (97.8%), `semio-s-artifact-norm-en1993-model` (97.2%), `semio-s-artifact-norm-en1991-mut-rail-load-groups` (96.9%).

## Validation against recorded compiler numbers

Recorded "exceeds its source boundary" numbers from ticket notes, compared with this script on the current sources:

| Crate | Recorded files / bytes | Script build (default) | Script test (default) |
| --- | --- | --- | --- |
| semio-plugin-reactor-turn | 19 / 262858 | 20 / 260916 | 25 / 271729 |
| semio-plugin-instance-docs | 17 / 264325 and 17 / 263401 | 16 / 256451 | 18 / 260673 |
| semio-s-artifact-norm-en1998-mutations | 13 / 262428 | 6 / 237169 | 7 / 238198 |
| semio-s-artifact-norm-vdi3805-examples | 33 / 32205 | 15 / 17856 | 22 / 26301 |

The script is an estimate, not an oracle. It under-counts in most recorded cases. Likely causes: feature unification from dependents (for example `test-support`, which all-features does not change for en1998-mutations), cfg targets other than macOS, and recorded numbers taken from earlier sources. Treat the table as a ranking, and confirm any crate near the limit with the real admission before splitting it.

Known issue in the scan: `semio-s-artifact-norm-en1993-checks-core` (test set, 10 issues) includes `include_str!` targets under `🧪️tests/🔬️unit/` that do not exist on disk, so its test-set count is a lower bound and the test run would refuse.
