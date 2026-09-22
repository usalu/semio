# 📓️ PROPOSED DIFF — members-aware twins for the registered convergence/idempotency laws

Owner: the peer holding `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (brief v4 no-touch list).
Raised by: the `knowledge` fix agent, ticket 26/09/19, 2026-09-22.

## The defect

`artifact_app_laws`' registered-fixture constructors are hard-wired to `VcsArtifactApp<A>`, i.e. the DEFAULT member
roster:

```rust
pub async fn new_registered_app<A, Manifest>(manifest: Manifest) -> VcsArtifactApp<A>            // :7227
pub async fn paired_registered_apps<A, Manifest, Build>(channel, manifest) -> (VcsArtifactApp<A>, VcsArtifactApp<A>)  // :7431
pub async fn assert_two_registered_instances_converge<A, P, Manifest, Build>(…)                  // :7559
pub async fn assert_registered_ingest_idempotent<A, P, Manifest, Build>(…)                       // :7638
```

An app whose `genesis_child_pack` mints a composed child therefore fails construction:

```
ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store:
  Fault { code: "plugin.internal",
          message: "derived child dialect 's.stdio.semio@v1/text' is not declared by this app's member roster" }
  at 🔌️plugin/🦀️.rs:22744
```

The members-aware constructor already exists one screen above — `new_app_with_registry_and_members::<A, M>`
(:7219) — so this is purely a missing generic on four helpers, not a new mechanism.

## Blocked tests (this ticket's `knowledge` topic; other topics will have their own)

- `semio-s-artifact-mathematical-equation`
  - `editor::equation::commands::set_algorithm::tests::ingest_operations_is_idempotent_for_equation`
  - `editor::equation::commands::set_algorithm::tests::two_instances_converge_disjoint_edits_via_backbone`
  - (equation's roster is `semio_s_artifact_stdio_semio::SemioMembers`; its children are
    `s.stdio.semio@v1/{text,table,value}`)
- `semio-s-artifact-imperative-procedure`
  - `editor::procedure::component::unit_tests::two_instances_converge_disjoint_edits_via_backbone`
    (children `s.stdio.semio@v1/{flow,text}`)

## Proposed shape — additive, no existing caller changes

Rust does not allow default type parameters on functions, so each helper gains a `_with_members` twin and the
existing one delegates to it with the current default roster. Sketch:

```rust
/// 🧬️ Members-aware twin of [`new_registered_app`]: an app whose `genesis_child_pack` mints a
/// composed child needs the roster that can NAME that child's dialect, exactly the way
/// `new_app_with_registry_and_members` already does for the unregistered constructor.
pub async fn new_registered_app_with_members<A, M, Manifest>(manifest: Manifest) -> VcsArtifactApp<A, M>
where
    A: ArtifactApp + Default,
    M: super::SpaceMember + super::MemberFactory + Send + 'static,
    Manifest: std::future::Future<Output = App>,
{
    let definition = manifest.await.definition;
    let mut app = VcsArtifactApp::with_registry(A::default(), AppActionRegistry::from_definition(&definition)).await;
    app.bind_instance_id(meta("local").instance_id).await;
    app
}

pub async fn paired_registered_apps_with_members<A, M, Manifest, Build>(channel: &str, manifest: Build)
    -> (VcsArtifactApp<A, M>, VcsArtifactApp<A, M>)
where /* same bounds, plus M */ { /* body of paired_registered_apps, with M threaded through */ }

pub async fn assert_two_registered_instances_converge_with_members<A, M, P, Manifest, Build>(
    channel: &str, manifest: Build, command_a: A::Command, command_b: A::Command,
    probe: impl Fn(&VcsArtifactApp<A, M>) -> P,
) where /* same bounds, plus M */ { /* body, calling paired_registered_apps_with_members */ }

pub async fn assert_registered_ingest_idempotent_with_members<A, M, P, Manifest, Build>(
    manifest: Build, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P,
) where /* same bounds, plus M */ { /* body, calling new_registered_app_with_members */ }
```

and the four existing helpers become one-line delegations at whatever roster
`VcsArtifactApp<A>`'s default member parameter names today, so every current caller is untouched.

`settle_registered_typed_operation`, `close_registered_fixture_app`, `exchange_and_assert_convergence` and
`settle_history_verb` are already generic over `M` (see `settle_history_verb` at :7456), so nothing else in the
ladder needs to move.

## Call sites that switch once the twins land

```rust
// ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/…/🎮️commands/🧮️set-algorithm/🧪️tests/🔬️unit/🦀️.rs
assert_two_registered_instances_converge_with_members::<EditorApp<EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _, _>(…)
assert_registered_ingest_idempotent_with_members::<EditorApp<EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _, _>(…)

// ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:397
assert_two_registered_instances_converge_with_members::<EditorApp<ImperativePlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _, _>(…)
```

No plugin-side change is possible without the twins: the roster is a type parameter of the app the helper
constructs, and the helper never exposes it.
