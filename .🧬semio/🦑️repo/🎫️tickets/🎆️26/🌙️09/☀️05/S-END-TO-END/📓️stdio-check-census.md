# stdio compile census — 2026-09-05

Purpose: establish whether `semio-s-plugin-stdio` (the dependency of all 33 `s` plugins) compiles right now, without taking the shared wasm-dev lock.

| Run | Command | Result |
|---|---|---|
| native | `RUSTC_WRAPPER="" CARGO_TARGET_DIR=target-s-e2e cargo check -p semio-s-plugin-stdio --keep-going --message-format=short` | EXIT 101 after dependency compilation; stdio itself never reached. One error: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:22:61: error[E0432]: unresolved import super::schema::DirectorySpaceDetailV1` → `could not compile semio-framework-os-kernel`. |
| wasm32-wasip2 | same with `--target wasm32-wasip2` | EXIT 101, identical kernel error; the kernel is in the wasm dependency graph too. |

Attribution: `📇️directory/🔌️client/🦀️.rs` (modified 04:29) imports `DirectorySpaceDetailV1` from `📇️directory/🧬️schema/🦀️.rs` (modified 04:16, plus `🔣️.json`/`🟦️.ts` twins) and no definition exists anywhere under `🧰️framework` at 04:50. The directory module is being reshaped by the COMPLETE-SEMIO-END-TO-END fleet (directory event page / space detail lanes, see that ticket's `📓️sol-directory-event-page-v1-hub-p0.md`, 03:31). Until they land the schema twin, no Rust crate that depends on the OS kernel compiles, which includes every `s` plugin on both targets.

Peer findings folded in: semio-f4 verified the stdio `#[path]` mount drift from the emoji rename is gone from the main crate (the 7 remaining hits live in `semio-s-plugin-stdio-test-oracle` and test-only fixtures, unreachable from stdio); semio-08 traced the `s.stdio.gltf executable mapping keys diverge` registry mismatch to commit 03100691d5 (fixed). The still-unknown part is whether stdio itself has residual `ToValue`/`FromValue` derive fallout; that can only be measured once the kernel compiles.

Next: rerun both checks when `DirectorySpaceDetailV1` exists; record exit and the stdio-owned error list here.

## Update — 18:21

| Run | Command | Result |
|---|---|---|
| native (after kernel repair) | `cargo check -p semio-s-plugin-norm --lib` (compiles stdio as a dependency) | EXIT 0, 0 errors, 16 warnings |
| wasm32-wasip2 | `RUSTC_WRAPPER="" CARGO_TARGET_DIR=target-s-e2e cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --keep-going` | **EXIT 0** after 86 min 52 s (0 errors, `semio-framework-plugin` 189 warnings) |

Conclusion: `semio-s-plugin-stdio` compiles on both targets with the current tree (kernel `DirectorySpaceDetailV1` import repaired by its owners at ~04:50, lane C's plugin-crate enum refactor consistent). The remaining stdio gap is the missing owner descriptor pair, which only a wasm build + `describe` produces (Wave 2 rebuild).
