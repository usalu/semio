# DSL Notation Dead Edge-Label Printer Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🖋️notation/🦀️component.rs` SHA-256: `926c68749bd10109357f3227bcd017cc613410ce7a26bd5be7cd6c85a86eb977`; clean.

## Consumer Evidence

`print_edge_label` is private, has no call site, registration, reexport, mount, generated consumer, test, example, or production consumer in active authored Rust. The Rust compiler independently reports it as dead code during the green OS kernel gates. Its adjacent notation printers remain live.

## Lease

Delete only the private `print_edge_label` definition. Preserve every live notation printer and all formatting semantics.

Terra writable paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🖋️notation/🦀️component.rs`
- one unique Terra acceptance Markdown

Validation:

```text
bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache
bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache
```

Require active stale reference zero and scoped ordinary/cached diff checks. Do not touch the DSL lexer/Graph DSL, SPR/store, Cargo, stdio, or registrars.
