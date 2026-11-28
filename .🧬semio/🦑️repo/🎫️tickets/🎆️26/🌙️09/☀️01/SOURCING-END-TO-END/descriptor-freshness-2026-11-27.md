# Descriptor freshness test results (2026-11-27)

## Route
- `repo cargo` via `native-cargo.sh` was admitted but the capacity gate held the job for about 60 minutes ("source.served" phase, memory short, 8 MB of unrelated workspace-wide output). Its log is `🗑️generated/haiku-descriptor-semio-s-plugin-sourcing-beams-component.txt`. Per the 60-minute limit, that run was stopped (only my own driver PIDs 37067 and their descendants).
- Fallback: native `cargo test -p <crate> -j 4 -- --test-threads=1`, one crate at a time, with the shared env from `extensions-rust-native.sh` (`CARGO_TARGET_DIR=/Users/ueli/t/src-extensions`, no rustc wrapper, debug 0). Driver: `haiku-descriptor-native.sh`.
- Host was tight: disk 14 GB free before the run, 22 GB after.

## Per crate
| Crate | Exit | Passed | Failed | Test |
| --- | --- | --- | --- | --- |
| semio-s-plugin-sourcing-beams-component | 101 | 0 | 1 | `descriptor_is_fresh` FAILED |
| semio-s-plugin-sourcing-slabs-component | 101 | 0 | 1 | `descriptor_is_fresh` FAILED |
| semio-s-plugin-sourcing-windows-component | 0 | 1 | 0 | `descriptor_is_fresh` ok |
| semio-hub-sourcing-component | 0 | 1 | 0 | `descriptor_is_fresh` ok |

## Failure messages (exact)
beams (`🧩️extensions/🪵️beams/🧩️component/📦️packages/🦀️rust`):
```
thread 'descriptor_is_fresh' panicked at 🧩️component/📦️packages/🦀️rust/../../🦀️.rs:3:1:
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🧩️component/📦️packages/🦀️rust/../../🛂️.descriptor.semio: the committed descriptor does not decode as the current descriptor shape, so it is stale — run `repo run workspace:components describe <plugin-id>` (📓️design-abi.md §3) and commit the refreshed 🛂️.descriptor.semio + 🔣️.json
```
slabs (`🧱️slabs/🧩️component/📦️packages/🦀️rust`): same message with the `🧱️slabs` path.

## What `descriptor_is_fresh` compares
It does not compare against a rebuilt wasm or a second descriptor. It decodes the committed `🛂️.descriptor.semio` (next to the component's `🦀️.rs` root) with the current descriptor schema. If decoding fails, the test fails as stale. Its source lives in the shared `semio_plugin` macro layer, which I did not modify.

## Observations
- The windows component descriptor and the hub sourcing component descriptor decode with the current shape (pass).
- The beams and slabs committed descriptors still fail to decode. Those two were not part of the refresh that was just committed, so the refresh appears to cover windows and hub only.
- Fix for beams and slabs: `repo run workspace:components describe <plugin-id>` from built wasm, then commit the refreshed `🛂️.descriptor.semio` and `🔣️.json`. Not done here (no product code or descriptors changed).

## Logs
- `🗑️generated/haiku-descriptor-semio-s-plugin-sourcing-beams-component.log`
- `🗑️generated/haiku-descriptor-semio-s-plugin-sourcing-slabs-component.log`
- `🗑️generated/haiku-descriptor-semio-s-plugin-sourcing-windows-component.log`
- `🗑️generated/haiku-descriptor-semio-hub-sourcing-component.log`
- `🗑️generated/haiku-descriptor-native-driver.log`
- Stalled repo-route log: `🗑️generated/haiku-descriptor-semio-s-plugin-sourcing-beams-component.txt`
