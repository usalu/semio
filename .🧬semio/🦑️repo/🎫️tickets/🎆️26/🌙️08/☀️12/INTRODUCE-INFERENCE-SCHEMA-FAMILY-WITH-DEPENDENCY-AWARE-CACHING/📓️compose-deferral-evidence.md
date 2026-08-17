# Compose `flat_positions_cache` — deferral evidence (post-close addendum)

Written by the parent session after the ticket closed, re-examining the one remaining item from the
approved plan (P3 §5, "retire compose's coarse `flat_positions_cache`") rather than accepting the
deferral on trust. **Conclusion: the deferral is correct, and for a stronger reason than originally
recorded.** The closing summary cited "93 pre-existing errors"; the real barrier is architectural.

## 1. The crate cannot compile, and the cause is unrelated API drift

```
RUSTC_WRAPPER="" cargo check -p semio-compose-rs   →  92 errors
   91 × error[E0433]  cannot find module or crate
    1 × error[E0432]  unresolved imports
occurrences of `flat_positions` in the error output:  0
```

The E0432 is the informative one:

```
unresolved imports `semio_framework_os_kernel::os_vcs::create_document_vcs_envelope`,
  `…::materialize_document_projection`, `…::ArtifactVcsEnvelope`,
  `…::ArtifactVcsStore`, `…::Operation`, `…::OperationDiff`
  --> compose/client/lib/rs/lib.rs:7817
```

These are **not a `document_*` → `artifact_*` rename tail.** Checked both spellings: neither
`create_artifact_vcs_envelope`, `materialize_artifact_projection`, `ArtifactVcsEnvelope` nor
`ArtifactVcsStore` exists anywhere in `🧰️framework`. What `🌿️vcs` actually exports today is
`ArtifactVcs`, `VcsError`, `create_document_vcs_id`. Compose is written against an **older `os_vcs`
API that has since been restructured**, not merely renamed — so there is no mechanical fix, and any
edit to this crate is unverifiable until its VCS integration is rebuilt. That work belongs to
whoever owns `🌿️vcs`/compose, not to this ticket.

## 2. Even with a compiling crate, the conversion is not a retirement

`compose/client/lib/rs/lib.rs:4884`:

```rust
pub async fn flatten_positions(self: &Arc<Self>, kit: &Arc<crate::kit::Kit>)
    -> HashMap<Id, crate::geom::PositionInput>
{
    if let Some(cached) = self.flat_positions_cache.read().await.clone() { return cached; }
    let computed = crate::geom::flatten::flatten_design_positions(kit, self).await;
    *self.flat_positions_cache.write().await = Some(computed.clone());
    computed
}
```

Three structural mismatches against `store::infer_field`:

1. **async vs sync** — `flatten_positions` is `async`; `infer_field` (`💡️inference/🦀️component.rs:253`)
   is a synchronous `fn … -> BTreeMap<F::Key, F::Value>`.
2. **live mutable state vs immutable snapshot** — `InferredField<P>::compute` is pure over `&P`.
   Compose's `Design` is `Arc<Self>` with `RwLock` interior mutability, mutated in place by
   `insert_piece`/`delete_piece_by_external_id`, each of which calls
   `invalidate_flat_positions_cache()`.
3. **no artifact snapshot exists** — the fold runs over compose's own `Kit`/`Design` types. There is
   no `XSnapshot` for our machinery to key a dependency chain against.

Note the dependency itself is *not* the obstacle: `semio-compose-rs`'s `Cargo.toml` does depend on
`semio-framework-os-kernel`, so `store::InferredField`/`infer_field` are reachable. The obstacle is
that satisfying the contract requires either materialising `Design` into an immutable snapshot value
or making the framework's `infer_field` async — both architectural changes well beyond retiring a
coarse cache, and neither verifiable while the crate has 92 errors.

## 3. What would actually be needed

For whoever picks this up, in order:

1. Rebuild compose's `os_vcs` integration against the current API (unblocks compilation).
2. Introduce a snapshot representation of `Design` that `InferredField` can key on.
3. Then the cache retirement becomes genuine: replace whole-design invalidation on every topology
   edit with per-piece merkle dep-chains, exactly as `puzzle3d`'s `🎛flat-position` does.

Step 3 alone is the ticket's thesis applied to compose; steps 1–2 are prerequisites owned elsewhere.

## Verdict

Deferral stands, upgraded from "93 pre-existing errors, unverifiable" to "unverifiable **and**
architecturally blocked on two prerequisites owned by other tickets." Nothing about it is a shortcut
taken for time.
