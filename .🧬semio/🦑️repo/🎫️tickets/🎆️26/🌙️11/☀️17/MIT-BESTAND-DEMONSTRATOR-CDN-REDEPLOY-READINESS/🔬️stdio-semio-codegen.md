# Stdio Semio Code Generation

The wzhjRpw Cargo artifact record and local unit receipt identify the exact `semio_s_artifact_stdio_semio` archive as 1,066,037,756 bytes with SHA-256 `08f8e80787c48cc8403729a62da1ee5e877c3a833aa4784eb255d5034827c519`. The unit receipt recorded 370,632 ms of compiler wall time and 3,499,278,336 peak RSS bytes. The queue's physical-charge reading is a separate host metric. The retained archive has 16 LLVM bitcode units. Pinned LLVM streamed the two largest members directly into a name and line census.

| Unit | Bitcode bytes | IR lines | Defined functions | Largest repeated family |
| --- | ---: | ---: | ---: | --- |
| cgu.02 | 154,121,260 | 1,814,035 | 12,974 | 6,696 `admit_original_construction` bodies and closures, 811,675 function-body lines |
| cgu.00 | 99,840,184 | 3,557,742 | 8,626 | 2,722 `with_loan` bodies beneath `StepContext::with_original_effect_at`, 2,260,521 function-body lines |

The census attributes functions by their Rust v0 symbol strings. It includes outlined closures and counts function-body definitions across generic substitutions. Total IR also includes globals and metadata. The counts are a measure of repeated compiler input, with link-time retained size and compile-time savings still unmeasured.

The earlier [generic retirement audit](🎈️component-bloat.md) found large `ControlledRetirement<T>::step_original` bodies. The two sampled units now concentrate their large functions in original Work loans and native construction. In cgu.00, 18 Work-loan functions are 983 lines each and carry the same `PagedListReservationOutcome` result. Their symbols trace through `StepContext::with_original_effect_at` to the hydration document's `step_decode_forward` closure with different snapshot and mutation types. The enclosing Work-loan primitive is one short function. The symbol's size comes from the inlined paid callback, receipt settlement and recovery.

A focused release-codegen probe used 24 typed callbacks and the live `with_original_effect_at` body, with the retained WASI Job Work and value-control crates. Pinned Rust used `opt-level=s`, 16 codegen units, linker-plugin thin LTO and one compiler thread for both variants. The probe emitted two bitcode units in each variant.

| Measure | Original | Outlined |
| --- | ---: | ---: |
| LLVM bitcode bytes | 807,608 | 600,592 |
| Function-body IR lines | 29,211 | 19,043 |
| Function-body lines in 24 Work loans | 19,499 | 8,831 |

The release-profile repeats used 1.095 and 1.093 seconds respectively. The 0.002-second difference is below practical timing resolution. A subsequent component build can measure how the 25.6% probe bitcode reduction affects the full Stdio archive.

Job Context now outlines the type-independent settlement and recovery in two non-inlined helpers. The typed callback, result and drop scope stay in `with_original_effect_at`. Both the original and candidate native functions matched the Python reference for denied expired Work, billed Work, explicitly zero Work, cancellation of an unperformed action, over-ceiling refusal with the typed result dropped before return, and retirement after the normal deadline. The deployed source extraction passed the same six cases. Pinned Rust also type-checked the complete current Context crate against the retained WASI dependency graph. The original Context SHA-256 was `b0c12ba9cecce05420d2a3dd1f87c3a44ce641fc1ab6073f15849f86727f440d` and the new one is `9efed311246dd494dbf01c37a168649a522e11880d454b1bdb866f3e24c118fd`.

The raw census is reproducible with the pinned toolchain's `llvm-ar p` for each named member, piped through `llvm-dis -o - -` and `stdio-ir-census.py`. No archive or IR copy is needed. The byte identity comes from the compiler's unit-cache receipt.

`step-work-ir.py <retained-lane>` measures the current implementation. Add `--native` for the neutral behavior cases or `--check-context` for the full WASI crate type check. Give it `--context <original context source>` to repeat the A/B experiment from the pinned original source.

## Original Construction Admission

The second focused probe compiled 24 typed construction candidates against the actual retained WASI Job, Value, Retained Clone and Native Owned crates from `wONtSjE`. Both variants used the same release codegen profile as the Work probe. The candidate shares the admission, native receive and settlement control through a stack-borrowed callable. It retains each typed candidate's original `with_limits` call, allocation marker, progress computation and publication point. A scoped guard discards an untouched constructor where the original owned receive closure discarded it.

| Measure | Original | Shared control |
| --- | ---: | ---: |
| LLVM bitcode bytes | 2,335,584 | 1,257,432 |
| Function-body IR lines | 50,116 | 23,205 |
| Construction-family IR lines | 31,119 | 7,980 |
| Construction-family functions | 264 | 106 |

Both variants passed all eight canonical native construction tests, including allocation refusal and its billed Work, candidate custody on an over-quote, independent Source identity and retirement before another receive. The native deadline sweep also asserted that a constructor captured and then left unused after preparing the receiving admission dropped exactly once. The existing SQLite accounting law passed. The candidate is held for a later build window while `wONtSjE` is live.

`construction-ir.py <retained-lane>` repeats the WASI experiment. `construction-native-setup.py` builds the small native test dependencies under the ticket's generated directory, and `construction-ir.py <retained-lane> --native` runs the native comparisons from that dependency set. The generated source is passed to rustc in memory.
