# Animate Dependency Cleanup Command Log

## Source and manifest checks

- `rg -n 'EcoString|ecow|comemo' ✏️s/🔌️plugins/🎞️animate --glob '!target/**' --glob '!🧪️/**'`
  - Exit: 1 after edits (zero matches).
- `rg -n '\\[DEBUG\\]' <touched manifest> <touched text component>`
  - Exit: 1 (zero matches).
- `rustfmt --edition 2021 --check <touched text component>`
  - Exit: 0.
- `cargo metadata --no-deps --format-version 1`
  - Exit: 0.

## Dependency ratchet

Command: `bun ./📜️script.ts verify dependencies`  
Exit: 0.

```text
baseline: 238
current: 205
removed since baseline: 33
new third-party dependencies: 0
rust:comemo: removed
rust:ecow: removed
```

## Real Animate crate check

Command: `CARGO_TARGET_DIR=<PUZZLE ticket target-p4> cargo check -p semio-s-plugin-animate --lib --message-format=short`  
Exit: 101 after the crate itself was reached.  
Result: 1,296 errors and 13 warnings in `semio-s-plugin-animate`; upstream stdio produced 20 warnings.

Representative existing error families:

```text
unresolved import ...animation::Animations
cannot find macro dyn_enum_close
cannot find type Animations / Sobjects
expected concrete output, found impl Future<Output = ...>
missing semio_framework_async_macros in generated example tests
```

No diagnostic named the retired `comemo`, `ecow`, `fontdb`, or `base64` direct rows. The crate/test gate remains open.

