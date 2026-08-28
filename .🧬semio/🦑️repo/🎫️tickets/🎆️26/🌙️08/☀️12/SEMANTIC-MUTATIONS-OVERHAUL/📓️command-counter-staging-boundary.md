# Command Counter Staging Boundary

The staged fixture owner is `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/`. It contains `Counter = i64`, ordered `CounterDiff { deltas: Vec<i64> }`, four direct leaf directories, descriptor/schema files, and an unmounted transparent `CounterMutation` aggregate.

Corrected leaf behavior: `AddCounter` uses an ordered one-delta diff and declares the `i64::MIN` inverse vector as `[+1, +i64::MAX]`, so Store's reverse replay applies `MAX` then `1`; the composite leaves use `crate::os_spr` types and `dsl_derive` macros. `AddCounterTwice` plans two local adds, `AddCounterFourTimes` nests two twice plans, and `AddCounterThenNotifyForeign` plans one local add followed by ordered foreign steps.

Remaining integration is deliberately held: mount the fixture only from the command test module, rewire old `AddOp`/`DoubleAdd`/`DerivedDoubleAdd`/`QuadAdd`/`AddThenNotifyForeign` consumers, add aggregate JSON envelope and codec surfaces, and reconcile descriptor optional codec claims with implemented text/binary codecs. The four leaf descriptors are not readiness proof. No Cargo, rustc, binary/runtime, or native source test was run.
