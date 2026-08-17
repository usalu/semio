# 🔖 DSL Notation Dead Edge-Label Printer Acceptance

## Scope

Deleted only the private `print_edge_label` definition from `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🖋️notation/🦀️component.rs`. The adjacent live `print_edge_node` and `print_edge` printers are unchanged.

## Preconditions

- HEAD was `0727b80aa6a802cac1760f90fb7a148f74035413`.
- The scoped file was clean in both ordinary and cached diffs.
- Its pre-edit SHA-256 was `926c68749bd10109357f3227bcd017cc613410ce7a26bd5be7cd6c85a86eb977`.

## Acceptance

- Active authored Rust has zero `print_edge_label` references (`rg -n --glob '*.rs' 'print_edge_label' .` returned no matches).
- The ordinary scoped diff is exactly `0` additions and `12` deletions; it deletes only `print_edge_label`.
- `git diff --check -- <notation component>` succeeded.
- The scoped cached diff is empty.
- Final SHA-256: `312e5f7b404564f6ffd3d49ff39dbface32db182afb650d3970992d245265935`.

## Required OS Kernel Gates

Both mandated uncached Nx gates were executed and are blocked by concurrent out-of-scope SPR `AppFrame` contract changes, not the notation edit:

- `bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache` exits nonzero because `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` initializes `AppFrame::Invocation` without `messages` and `AppFrame::Error` without `report`.
- `bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache` exits nonzero for the same `🧵️channel` initializers and matching `AppFrame::Error` initializers in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`.

No DSL lexer, Graph DSL, SPR/store, Cargo, stdio, or registrar files were changed by this lease.
