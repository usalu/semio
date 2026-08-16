# Pack Chunk LRU Zero-Consumer Dissolution Acceptance

## Scope

P-03 removes the complete `ChunkLruCache` responsibility from the Pack HTTP component and its sole Pack facade re-export. HTTP range transport, asynchronous source, and optional `ureq` behavior remain outside the edit.

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Pack HTTP SHA-256 before edit: `29cbe081d8dcb4715a62fbbad3f872cf5aa70f69d1f4a119c4129f31e2899c01`
- Pack facade SHA-256 before edit: `19c9abcd30905f3f9d4d01dcd19d501dbc1948ac0afb27dc1d0e468402cf5f12`
- The existing staged facade diff was verified as the released P-01 FieldIndex re-export deletion only.

## Validation

- Active source scan found no whole-symbol references to `ChunkLruCache`, `LruState`, `LruSlot`, or the cache-only `hash_of` fixture.
- Pack HTTP import closure is clean: its cache-only `HashMap` and `ContentHash` imports are absent; retained `Mutex` use belongs to HTTP etag state and the transport test double.
- Scoped ordinary and cached diffs pass `git diff --check`. The facade remains `MM`: its staged P-01 FieldIndex re-export deletion is unchanged, while P-03 is an unstaged `ChunkLruCache` token deletion.
- `bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache` exited `1` before Pack-specific diagnostics, blocked by the known external SPR/store MutationOutcome/reconcile drift. Exact leading blocker: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:23` imports missing `crate::os_spr::ReconcileReport`. The same gate reports that `Mutation::diff` now requires `MutationOutcome<Diff>` (`store` line 5700), `apply_mutation` returns `(P, Vec<MutationMessage>)` where Store expects `P` (including lines 2616, 2777, 4175, 4310, 4416, 4535, 4880, 5497, 6586), and Store calls absent `validate`/`reconcile` APIs. No unrelated repair was made.
