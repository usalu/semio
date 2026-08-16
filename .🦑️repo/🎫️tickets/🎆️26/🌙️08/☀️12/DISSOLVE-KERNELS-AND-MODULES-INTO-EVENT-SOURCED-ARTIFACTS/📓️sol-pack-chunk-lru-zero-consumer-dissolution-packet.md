# Pack Chunk LRU Zero-Consumer Dissolution Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Pack HTTP SHA-256: `29cbe081d8dcb4715a62fbbad3f872cf5aa70f69d1f4a119c4129f31e2899c01`; clean.
- Pack facade SHA-256: `19c9abcd30905f3f9d4d01dcd19d501dbc1948ac0afb27dc1d0e468402cf5f12`; externally staged only by released P-01, whose cached diff removes the independent field-index reexport region.

## Consumer Evidence

`ChunkLruCache`, `LruState`, `LruSlot`, and their cache tests have no production consumer. The only external reference is the Pack facade reexport. The HTTP source itself does not construct or call the cache. Same-file tests do not qualify as consumers.

## Lease

Delete the complete Cache responsibility and its cache-only tests from Pack HTTP. Remove `ChunkLruCache` from the facade reexport while preserving every remaining HTTP symbol and the released P-01 cached diff. Remove imports and module documentation that become stale. Do not touch HTTP transport/source behavior, async Pack, OS central glue, Cargo, the field-index deletion, generated files, or any other path.

Writable paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌐️http/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs`

Validation:

```text
bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache
```

If the gate remains blocked by the external SPR/store MutationOutcome/reconcile drift, record the exact unchanged blocker and rely only on zero live references, source-level import closure, and ordinary/cached diff checks; do not repair unrelated APIs.
