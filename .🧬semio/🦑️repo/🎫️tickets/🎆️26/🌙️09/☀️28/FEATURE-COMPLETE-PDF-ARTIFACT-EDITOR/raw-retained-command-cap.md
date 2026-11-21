# Retained command test source limits

The target is the `semio-plugin-retained-command` library test harness (`--lib`) registered at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/📦️packages/🦀️rust/Cargo.toml`. Linux, macOS, Windows, and WASI each allow 32 files, 4,000 active lines, and 262,144 source bytes. The ordinary product library is admitted on all four with 14 files, 2,283 lines, and 246,760 bytes. Test inclusion grows the library to 23 files, 2,557 lines, and 264,833 bytes. The harness exceeds the byte cap by 2,689 on all four platforms. The picker overlay leaves that target unchanged.

The library harness adds the following source files to ordinary product admission. Paths are relative to `🧵️retained-command` unless rooted at `🧰️framework`.

- `🧰️framework/🔨️modules/🧵️job/🧪️testing/♻️close/🦀️.rs`
- `🎟️admission/🧪️tests/🦀️.rs`
- `🎟️admission/🧪️tests/🧫️fixtures/🔣️.json`
- `🏭️factory/🧪️tests/🦀️.rs`
- `🏭️factory/🧪️tests/🧫️fixtures/🔣️.json`
- `🧪️tests/🔬️unit/🦀️.rs`
- `🧪️tests/🧫️fixtures/🚪️raw-allocation-close.json`
- `🧯️raw-allocation-close-oracle/🦀️.rs`
- `🪟️mounted/🚨️structural-refusal/🧪️tests/🧫️fixtures/🔣️.json`

The staged split keeps `🧪️tests/🔬️unit/🦀️.rs` in place and registers it as the `reducer-fault-detail` package integration test. Three existing cases exercise the public `reducer_fault_detail` function and public retained-work byte limit with their original assertions. [Staged split](raw-retained-command-cap-stage.py)

- `a_refused_reducer_step_reports_the_apps_own_fault_code_and_message`
- `an_oversized_reducer_fault_detail_is_clipped_on_a_char_boundary`
- `a_reducer_fault_detail_reads_back_as_the_reducers_own_code_and_message`

[The independent admission probe](raw-retained-command-bounds.ts) passed 20 checks over the live and staged ordinary library and library test harnesses, and the separate staged integration target on the four platforms. It reports 14 files, 2,283 lines, and 246,684 bytes for the staged ordinary library. The staged library test harness has 22 files, 2,518 lines, and 261,682 bytes, leaving 462 bytes. The separate harness has 1 file, 38 lines, and 3,168 bytes. The guarded overlay remains staged during provider captures. A linked run of the split harness is pending an admitted exact retained-command package build.
