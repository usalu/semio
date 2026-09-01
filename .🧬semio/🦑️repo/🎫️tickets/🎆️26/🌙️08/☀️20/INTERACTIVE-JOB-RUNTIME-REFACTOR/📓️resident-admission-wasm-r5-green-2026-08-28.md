# Resident Admission Wasm R5 Compile GREEN

Canonical existing check-wasm target completed both declared checks serially: wasm32-wasip2 (0.84s) and wasm32-unknown-unknown (0.97s), Nx0. Same production f00ac674 and fresh selected20 capture [here](./📓️resident-admission-wasm-r5-selected-inputs-2026-08-28.md). Two existing AtomicUsize::fetch_update deprecation warnings per target remain and were not repaired by this executor. This is compilation only, not Wasm execution or a funded live consumer.

Source/compiler hold released at terminal. Native R4 remains actual12/12 with passing stdout not captured; only the aggregate footer was available from that invocation.

```sh
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/value-resident-rs:check-wasm --skip-nx-cache
```

## Complete Captured Tool Output

```text
> nx run @semio-tech/value-resident-rs:check-wasm

> bun ./📜️script.ts check-wasm

    Checking semio-framework-value-resident v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust)
warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:341:22
    |
341 | ...   node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |                    ^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
    |
341 -         node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
341 +         node.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:414:43
    |
414 | ...   unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_|...
    |                                         ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
414 -         unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
414 +         unsafe { pointer.as_ref().aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    |

warning: `semio-framework-value-resident` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-value-resident` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.84s
    Checking semio-framework-value-resident v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust)
warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:341:22
    |
341 | ...   node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |                    ^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
    |
341 -         node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
341 +         node.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:414:43
    |
414 | ...   unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_|...
    |                                         ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
414 -         unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
414 +         unsafe { pointer.as_ref().aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    |

warning: `semio-framework-value-resident` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-value-resident` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.97s



 NX   Successfully ran target check-wasm for project @semio-tech/value-resident-rs
```

