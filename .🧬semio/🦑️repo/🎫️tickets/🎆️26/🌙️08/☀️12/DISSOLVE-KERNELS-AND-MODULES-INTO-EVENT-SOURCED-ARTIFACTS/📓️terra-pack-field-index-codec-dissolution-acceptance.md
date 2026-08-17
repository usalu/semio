# Pack Field Index Codec Dissolution Acceptance

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Coordinator-owned OS glue matched SHA-256 `d2e846bf87210e7edc433ce33a6b5a973776c8051dc711815ab961d00a2a9504` and is read-only for this lease.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️index/🦀️component.rs` was clean at SHA-256 `3e1604615b2c76ed9dbb221fac7abf1772ae539765032d6e3eb7e8de74c509d7`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs` was clean at SHA-256 `4f2c34b29b13c0ebcdcc5b846355d6b7063c9f2eac4e20006230fa163e7ad61c`.
- The field-index types and paths have no live production caller outside their definition, reexport, and already-removed glue mount. `KIND_FIELD_INDEX` remains part of the wire format.

## Implementation

- Deleted `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️index/🦀️component.rs`, including its codec implementation and self-tests.
- Deleted only the `🔖️Index` reexport region from `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs`.
- Retained `KIND_FIELD_INDEX` in the facade’s core wire-format reexports. No alias or compatibility surface was added.

## Validation

- Repository-authored stale-symbol/path search for `FieldPath`, `FieldIndexEntry`, `FieldIndexBuilder`, `FieldIndexReader`, `os_pack::index`, and `pack::index`, excluding tickets/history/build/dependency output, returned no matches.
- `KIND_FIELD_INDEX` remains present in the pack facade core reexport.
- Ordinary and cached scoped `git diff --check` validations exited `0`.
- `bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache` exited `0`. The kernel checked successfully with pre-existing compiler warnings; no test target was run.
- No taxonomy, census, formatter, or generator was run.

## Final State

- HEAD remained `0727b80aa6a802cac1760f90fb7a148f74035413`.
- The index leaf is absent.
- Pack facade SHA-256: `19c9abcd30905f3f9d4d01dcd19d501dbc1948ac0afb27dc1d0e468402cf5f12`.
- Cached source diff is exactly index leaf `0` additions / `263` deletions and facade `0` additions / `4` deletions.
- Both leased paths appeared index-staged after their clean baseline. No Git-mutating command was used; that externally controlled index state was preserved.
