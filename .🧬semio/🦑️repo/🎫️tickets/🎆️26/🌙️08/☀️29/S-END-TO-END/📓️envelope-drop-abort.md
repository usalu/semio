# 🧺️ The envelope-drop abort that kept `s` from booting

## Symptom

`s` reached the browser, fetched its plugin module, rendered the shell chrome — and then stamped
`data-semio-os-error="s"` with `No plugins loaded`. The Playwright spec `.storybook/s-end-to-end.spec.ts`
failed all three tests on the same assertion: `s must boot READY, got "error"`.

Browser console (captured live, iframe story `🛠️framework🖥️os-plugins--s`):

```
thread '<unnamed>' (1) panicked at 🧰️framework/.../🏪️store/🦀️.rs:2467:9:
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner
[DEBUG] program worker s#1 error type=RuntimeError framesBytes=n/a
Framework OS boot failed  RuntimeError: unreachable
```

The same abort reproduced headlessly on the `describe` export under Node, so the failure is in the
guest, not in the browser host.

## Why static search could not find it

`ArtifactEnvelope<P, Mutation>` has a `Drop` impl that asserts `owners_detached`, a flag only
`into_owners()` sets. Dropping an envelope any other way aborts the guest. That makes the defect a
**runtime** abort with no compile-time signal, and the offending expression does not look like a drop:
it looks like an ordinary field read.

Greps over `create_document_envelope` / `ArtifactEnvelope::from_owners` returned ~60 sites, nearly all
of them legitimately consumed by `ArtifactStore::new` and friends. Two read-only exploration passes
over the `Plugin::builder(...).try_build()` path found nothing, because the real site is not in the
builder at all.

The guest has no backtrace support (`RUST_BACKTRACE` is inert under wasip2), so the fix was to make
the assertion self-identifying — a temporary `[DEBUG]` message carrying `schema` and `id`:

```
[DEBUG] artifact envelope terminal shell reached Drop ... (schema=os.workflow, id=demo-studio)
```

That named the envelope in one run, and `demo-studio` grepped straight to the call site. The `[DEBUG]`
message has since been reverted.

## Root cause

`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`, `create_backbone_document`:

```rust
vcs: create_document_envelope::<P, Op>(schema, id, initial_snapshot, None).vcs.clone(),
```

The envelope is built as a temporary, `.vcs` is cloned out of it through `Deref`, and the temporary is
then dropped at the end of the statement — tripping the assert. `s`'s boot path reaches it through
`parse_demo_space_document()` → `create_backbone_document(S_WORKFLOW_SCHEMA, DEMO_STUDIO_ID, ...)`,
which is why the envelope identified itself as `os.workflow` / `demo-studio`.

Four further sites in the same file had the identical shape — `&backbone_envelope_of(document)` passed
by reference into a reader, with the temporary dropped immediately afterwards. Those are on the
save/export path rather than the boot path, so they had not aborted yet, but they are the same defect.

## Fix

1. `create_backbone_document` now consumes the envelope instead of cloning through it:

   ```rust
   vcs: create_document_envelope::<P, Op>(schema, id, initial_snapshot, None).into_owners().vcs,
   ```

   This also removes a clone of the whole vcs history.

2. The borrow-then-drop shape is now impossible to write by accident. A scoped lender owns the
   discipline, and the four export/materialize sites go through it:

   ```rust
   fn with_backbone_envelope<P, Op, R>(document: &BackboneDocument<P, Op>, read: impl FnOnce(&ArtifactEnvelope<P, Op>) -> R) -> R {
       let envelope = backbone_envelope_of(document);
       let result = read(&envelope);
       drop(envelope.into_owners());
       result
   }
   ```

   Call sites converted: `materialize_backbone_snapshot`, `export_backbone_pack`,
   `export_backbone_dsl`, `encode_backbone_payload`.

## Lesson for this codebase

The retirement-discipline types (`ArtifactEnvelope`, `ArtifactStore`, `ArtifactStoreStringRetirement`)
turn a forgotten move into a process abort rather than a compile error. Any API that hands out an owned
one of these — `backbone_envelope_of` did — should hand it out through a scoped lender instead, so the
retirement cannot be forgotten. A bare `-> ArtifactEnvelope<P, Op>` is a trap.
