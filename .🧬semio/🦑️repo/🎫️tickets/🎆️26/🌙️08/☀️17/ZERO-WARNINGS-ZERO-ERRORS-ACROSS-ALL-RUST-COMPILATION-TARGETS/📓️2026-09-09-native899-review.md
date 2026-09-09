# Native899 Compiler Review

Native full-scope check ended with code 101. Three diagnostics in the kernel blocked downstream compilation.

## clippy::mem_replace_option_with_some

replacing an `Option` with `Some(..)`

🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:16045

```rust
        let displaced = std::mem::replace(&mut stage.post, Some(post_snapshot));
```

## clippy::mem_replace_option_with_some

replacing an `Option` with `Some(..)`

🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:16045

```rust
        let displaced = std::mem::replace(&mut stage.post, Some(post_snapshot));
```

## clippy::assertions_on_constants

this assertion has a constant value

🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:3004

```rust
    assert!(ITEMS > crate::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY, "the fixture must exceed the fixed edit ledger capacity to prove the ceiling is gone");
```

## Repairs

Used Option::replace to preserve displaced snapshot ownership. Moved the fixed history-fixture capacity assertion into a const block so it is checked during compilation. Both Rust sources parsed before guarded writes.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs
