# Flow Owned Law Current Compiler Receipt

Actual targeted native test 98595 exited 1 before its runtime law.

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
  --> 🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/../../🎛️controlled/🦀️.rs:52:57
   |
52 |     let mut cursor = semio_framework_value::retirement::owned_retirement(value);
   |                                                         ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
  --> 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/../../📥️decode/🫳️borrowed/🦀️.rs:21:127
   |
21 | ...io_framework_value::retirement::owned_retirement(self.parser)}
   |                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

warning: unnecessary qualification
 --> 🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/../../🎛️controlled/🦀️.rs:9:81

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
  --> 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/../../📥️decode/🫳️borrowed/🦀️.rs:21:127
   |
21 | ...io_framework_value::retirement::owned_retirement(self.parser)}
   |                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

warning: unnecessary qualification
 --> 🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/../../🎛️controlled/🦀️.rs:9:81
  |
9 | ...lue else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected diagnostic object")) };
  |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
```
