# Warm Prototype Cargo Drift Recovery

The Layout guest run `wmBMDkH` compiled 840 Cargo units and stopped on six missing methods or constants in the plugin instance constructor. Two of its dependency artifacts had been emitted before their shared owners added the current methods. The checkout already contained the methods with the signatures the constructor used.

The repository's actual drift detector identified `instance/stores/📥️staging/🧱️components/🦀️.rs` by reading the run's 1,069-unit graph, compiler output and emitted Rust dep-info. Its stored BLAKE3 begins `3012c236` and the live source begins `65d9cbb6`.

Warm single-component prototype builds now use the same checked retry as warm release builds. The driver finishes the first process, identifies a dependency checksum mismatch, readmits its sources and compares the Cargo unit graph before launching one retry in the same lane. Publication starts after the resulting Cargo process completes.

The red eligibility test failed on the missing guard. The complete fixture suite passed 15 tests and 41 assertions. SQLite independently reproduced the seven eligibility results, and Noble BLAKE3 reproduced the dependency checksums. The real drift detector reproduced the constructor case on the failed build's actual compiler files. The next full guest run will exercise the current driver.
