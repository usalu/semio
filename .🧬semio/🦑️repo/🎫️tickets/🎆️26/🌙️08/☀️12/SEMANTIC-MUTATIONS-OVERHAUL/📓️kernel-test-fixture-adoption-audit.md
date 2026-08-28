# Kernel Test Fixture Adoption Audit

Read-only audit of the fixture contract and actual command source.

## Verified derive alias and four-leaf target

The real current derive spelling is `dsl_derive::CompositeMutation`, proven by the in-place fixture at [`command/🦀️component.rs:1760`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:1760). Its macro implementation is exported by [`dsl derive Rust glue:2015`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:2015). The OS Rust glue aliases itself as `dsl`, not `dsl_derive`, at [`OS Rust glue:32`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs:32); the fixture owner must therefore retain the explicit external `dsl_derive` macro path and use `::protocol` for the leaf derive contract.

The contract's exact four direct composite aggregate leaves are `AddCounter`, `AddCounterTwice`, `AddCounterFourTimes`, and `AddCounterThenNotifyForeign`, in that order. The source confirms the legacy behaviours: two local adds in [`DoubleAdd`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:947), two nested double-add plans in [`QuadAdd`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:966), and one local add plus explicit foreign steps in [`AddThenNotifyForeign`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:995).

## Concrete arithmetic blockers

`AddDiff::apply` uses unchecked `base + delta` and `absorb` uses unchecked `self.delta += other.delta` at [`command/🦀️component.rs:882`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:882). They overflow/panic in debug or wrap in release. `AddOp::inverse` negates `i64::MIN` at [`command/🦀️component.rs:900`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:900), with the same invalid boundary behaviour.

The four-leaf replacement cannot safely preserve these implementations: `AddCounterTwice` and `AddCounterFourTimes` multiply exposure through planner sequencing, while `fold_plan_diff` composes the returned `CounterDiff` through `absorb`. The new owner needs checked addition in apply and absorb, and an explicit non-representable-inverse policy for `i64::MIN`; the present `Mutation::inverse -> Vec<_>` signature cannot itself return a typed overflow error. This is an actual API/design decision required before a truthful inverse-boundary claim, not a macro-import blocker.
