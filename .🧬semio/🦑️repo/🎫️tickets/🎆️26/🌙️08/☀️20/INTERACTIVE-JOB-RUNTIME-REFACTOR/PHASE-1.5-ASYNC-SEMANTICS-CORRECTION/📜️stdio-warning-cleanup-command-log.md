# Stdio Warning Cleanup Command Log

All commands ran from `/Users/ueli/Documents/semio` using the ticket-local target directory when compiling.

## Commands

```sh
CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-stdio-warning-cleanup' \
  RUSTFLAGS='-D warnings' cargo check -p semio-s-plugin-stdio --lib --no-deps
# exit 1: cargo check does not accept --no-deps

CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-stdio-warning-cleanup' \
  RUSTFLAGS='-D warnings' cargo check -p semio-s-plugin-stdio --lib
# exit 101: 21 pre-existing dependency async_fn_in_trait warnings promoted to errors before stdio

CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-stdio-warning-cleanup' \
  cargo clippy -p semio-s-plugin-stdio --lib --no-deps -- -D warnings
# exit 101: 1,230 existing Clippy errors outside this cleanup

CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-stdio-warning-cleanup' \
  cargo check -p semio-s-plugin-stdio --lib
# exit 0 in 1m 18s; 20 warnings remain

rustfmt --check '✏️s/🔌️plugins/🗄️stdio/🦀️component.rs' \
  '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️component.rs' \
  '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️component.rs'
# exit 0
```

## Post-edit Static Checks

```text
UiNode lines not direct use or test-only import: 0
Qualified dyn-app type lines in root component: 0
Root doc comments immediately before dyn_enum_close!: 0
UiNode import files changed: 156
```

The full raw Clippy output is 225,963 tokens / 12,872 lines and could not be retained by the execution interface; its terminating diagnostic is: `could not compile semio-s-plugin-stdio (lib) due to 1230 previous errors`.
