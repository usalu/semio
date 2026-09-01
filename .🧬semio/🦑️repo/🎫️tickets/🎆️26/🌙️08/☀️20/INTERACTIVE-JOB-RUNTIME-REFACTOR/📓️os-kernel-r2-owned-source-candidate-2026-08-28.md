# OS-Kernel R2 Owned Source Candidate

## Scope And Verification Status

Eight exact source replacements in three files are mounted after the genuine OS R1 compiler RED. Parent was notified of the concrete delta before editing. No Cargo, native test, retry, source oracle or feature/profile/budget change has run after this edit. Scoped git diff --check exited0. Read-only validation found all eight exact replacement bodies in current source; all twelve original domain schema/fixture/script/native-leaf files remain byte-identical.

This is an uncompiled source candidate, not a green or full Plugin-coherent release. Mutation owns the separate84 outer-sync cfg(test) diagnostics and is working outside these reserved helpers. No compiler/source hold is requested by this report.

The actual [R1 inventory](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-native-r1-compiler-red-2026-08-28.md) remains92 errors/66 warnings/zero executed tests. Its original4 library blockers, intentional2 Send assertions, and86 cfg(test) diagnostics are preserved. The two helper diagnostics addressed here are separate from Mutation's84. Detach's library blocker remains deliberately unchanged pending real original-parent ownership.

## Exact Owned Delta

Directory's native module imports TokioHostRuntime from the existing semio_framework_os_services crate; its injected runtime/compute/scope constructor and six-law tests are unchanged.

Only ArtifactCodec.compile_dsl and print_mirror, and their respective nested function return types, gain +Send. Their typed P/Mutation bounds are made explicit at those two thunks. ArtifactCodec::of's existing P Send+Sync+'static bound remains; its Mutation boundary gains Sync. No global Mutation or ArtifactApp trait, other codec slot, erased actor future or codec body is changed.

The test-only fixture_runner_handle becomes visible to the enclosing sync module using pub(in super::super), not public crate/runtime visibility. In the quiet-queue fixture, rejection is retained in an Option before branching. On actual rejection, it saves the scalar kind, recovers the exact zero-capture closure, releases the blocked worker, shuts down the original pool, invokes that closure, then fails the original admission expectation. It does not require WorkerSubmitError:Debug, silently drop the returned closure, continue a partially filled queue, or change the successful saturation assertions. This new error branch is source-only until a later actual execution.

The following is the exact applied owned replacement delta, not attribution of unrelated working-tree history:

```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs
@@ owned replacement @@
-    use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle, TokioHostRuntime};
-    use semio_framework_os_services::{AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead};
+    use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle};
+    use semio_framework_os_services::{AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead, TokioHostRuntime};

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
@@ owned replacement @@
-    pub compile_dsl: for<'a> fn(&'a str, &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + 'a>>,
+    pub compile_dsl: for<'a> fn(&'a str, &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>>,

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
@@ owned replacement @@
-    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + 'a>>,
+    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>>,

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
@@ owned replacement @@
-        Mutation: self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpText + OpBinary + Send + 'static,
+        Mutation: self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpText + OpBinary + Send + Sync + 'static,

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
@@ owned replacement @@
-        fn compile_dsl_impl<'a, P, Mutation>(dsl: &'a str, ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + 'a>>
-        where
-            P: Clone + ArtifactDsl + ArtifactPack,
-            Mutation: OpText + OpBinary + self::Mutation<P>,
+        fn compile_dsl_impl<'a, P, Mutation>(dsl: &'a str, ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>>
+        where
+            P: Clone + ArtifactDsl + ArtifactPack + Send + Sync + 'a,
+            Mutation: OpText + OpBinary + self::Mutation<P> + Send + Sync + 'a,

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
@@ owned replacement @@
-        fn print_mirror_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + 'a>>
-        where
-            P: Clone + ArtifactDsl + ArtifactPack,
-            Mutation: OpText + OpBinary + self::Mutation<P>,
+        fn print_mirror_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>>
+        where
+            P: Clone + ArtifactDsl + ArtifactPack + Send + Sync + 'a,
+            Mutation: OpText + OpBinary + self::Mutation<P> + Send + Sync + 'a,

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs
@@ owned replacement @@
-        pub(super) fn fixture_runner_handle(pool: Arc<semio_framework_async::WorkerPool>, generation: u64, mailbox: ArtifactMailboxClose) -> ArtifactActorRunnerHandle {
+        pub(in super::super) fn fixture_runner_handle(pool: Arc<semio_framework_async::WorkerPool>, generation: u64, mailbox: ArtifactMailboxClose) -> ArtifactActorRunnerHandle {

--- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs
@@ owned replacement @@
-            started.wait();
-            for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
-                pool.try_submit(semio_framework_async::Lane::UserVisible, Box::new(|| {})).expect("fill exact quiet queue slot");
-            }
+            started.wait();
+            let mut rejected = None;
+            for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
+                if let Err(error) = pool.try_submit(semio_framework_async::Lane::UserVisible, Box::new(|| {})) {
+                    rejected = Some(error);
+                    break;
+                }
+            }
+            if let Some(error) = rejected {
+                let kind = error.kind();
+                let job = error.into_job();
+                release.wait();
+                pool.shutdown();
+                job();
+                panic!("fill exact quiet queue slot: {kind:?}");
+            }
```

## Why These Send/Sync Bounds

Read the actual two thunk bodies, ParsedDocumentText, ArtifactEnvelopeOwners/ArtifactEnvelope, parse_document_text/replay_ops, parse_document_pack/parse_document_spr, print_document_pack and print_document_text. No source call was treated as harmless simply because it is expected to complete quickly.

| Value retained by an actual future | Concrete source point | Required property |
| --- | --- | --- |
| Borrowed input &str / &[u8] | Both erased thunk parameters | These existing element types are Sync, so their shared references are Send; no new input type bound. |
| Owned initial snapshot P during parsing | parse_document_text11008 → replay_ops; parse_document_spr10660 holds decoded initial_snapshot across decode_history.await | P must be Send. |
| Owned Vec<Mutation>/Edit<Mutation> and accumulated envelope | replay_ops and parse_document_spr build edits across async validation/decode calls | Mutation must be Send. |
| ParsedDocumentText<P,Mutation> with both envelope and snapshot | [Fields9788](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9788); compile thunk9137 keeps parsed while awaiting print_document_pack | Owned P/Mutation state must be Send across the await. |
| &ArtifactEnvelope<P,Mutation> | [print_document_pack10794](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:10794), [print_document_text10053](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:10053), plus parse validation of the envelope | The shared envelope borrow must be Send, requiring its P/Mutation payloads to be Sync. |
| P/Mutation state inside the erased future | Both nested function signatures returning + 'a | Explicit + 'a permits the future to retain those concrete values for the borrow lifetime. The existing outer constructor already requires 'static. |

These are source-grounded reasons for the candidate bounds, not a claim of compiler-proven sufficiency. R1 compiler notes already prove that these two erased futures violate the actor's required Send contract; a fresh compiler gate must still verify all internal auto-trait constraints. No associated Diff bound or blanket trait expansion was guessed. The async bodies and their existing whole-document allocation/retirement behavior are untouched: Send is not a bounded-work, cancellation, or strict envelope-Drop proof.

## Exact Cross-Crate Caller Consequence — Not Patched

Read-only repository-wide ArtifactCodec::of census found Store fixtures/new Send laws, Sync's Demo fixture, concrete MCP Probe codecs, and Plugin declarations/registration. Plugin's actual ArtifactApp::Mutation bound11074 currently has Send but not Sync. Therefore the new constructor qualification is **not** claimed compatible with all current generic Plugin callers.

The concrete unjoined boundary is:

- Plugin DocumentCodecSpec::of and ::foreign nested codec<A: ArtifactApp> at3334/3341.
- DocumentCodecSpec::bare and nested codec at3349–3360, plus document_codec_bare/document_codec_bare_async at3238/3246.
- register_document_codec_for_app<A: ArtifactApp> at29847.
- The generic native_codecs<S,M> fixture at27494 already requires M:Send+Sync.

No Plugin/ArtifactApp source was changed. Parent has been notified; any necessary concrete where-bound/caller join must be separately coordinated and reviewed. Restoring non-Send slots, making the actor local, or globally expanding Mutation solely to conceal a caller mismatch is not proposed.

## Current Original-Parent Detach Plan

The resident authority still has SHA508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f. Actual public ResidentConsumer::handoff_for_close_into, ResidentAdmission::handoff_consumer_into and ResidentRecord::handoff_into accept structural &mut Option destinations. ResidentLedgerRoot::new accepts capacity but no original RuntimeAppCell/Store-field binding. Dag explicitly confirmed that the privately issued RuntimeAppCell→Store FIFO receiver/receipt is absent. No nominal Option borrow, observed admitted_bytes, root address or existing1024-slot FIFO supplies that funding authority.

The selected source join remains the [original-parent proposal](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️store-backbone-original-parent-proposal-2026-08-28.md), now strengthened by actual [R10 Data-only free/refund RED](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-release-r10-semantic-red-2026-08-28.md):

1. Actual funded RuntimeAppCell/Opening preadmits the whole parent/root/FIFO metadata layout. Its private closed Store-field selector issues the receiver bound to the original source consumer/record and exact displaced reservation(slot,generation,count1).
2. One canonical ResidentRecord<ArtifactStoreBackboneRetirement> allocation replaces the retirement Box. The same Store FIFO owns a typed entry/binding; no second pool, additional shell Box, public projection callback or numeric funding substitute.
3. Original parent receipt, exact FIFO slot and actual RecordNode shell backing are admitted and initialized while original backbone/descriptor/session channels remain installed. Checked generation and all receiver capacities are preflighted before any source take.
4. Commit revalidates that exact parent/source/FIFO binding, then transfers backbone plus descriptor into the preinstalled shell. No bump helper that clones cursor/history is used as a fake transaction; semantic revision remains unchanged.
5. SyncSession owns its exact detach request and original command/event receivers through refusal. The current cmd_tx.send-before-Store ordering must become an exact reservation/commit join; sending/waking before Store success is not retained atomicity. The Tokio event receiver cannot simply be cleared: its Drop may drain queued ArtifactEvents, so typed channel retirement remains a real prerequisite.
6. The original root/binding survives typed descendants, empty-shell destruction, exact deallocation, pointerless still-charged residue, separately granted Refund and Clear. R10 proves current close_step does not yet supply this ordering. A public Record alias cannot remain alive through free; the private parent/FIFO binding must replace its access authority at the reviewed handoff.

The current [SyncSession::detach900](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:900) and Store detach/replace_backbone_retained are unchanged. No deletion of await with a discarded Result/Backbones, async-void compatibility, ignored returned owner or cold Drop was used to make compilation appear green. The reviewed Free→Refund packet is distinct from the still-unimplemented Opening/parent-field binding. Its historical source-only paragraphs are superseded only by the separately recorded baseline R10, not by a production fix.

## Observed Source Hashes

These whole-file hashes were read after the owned patch, not asserted frozen against Mutation's subsequent outer-fixture edits:

```json
[
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs",
    "sha256": "56208e0ddbd792fe1351d69a920f3c0472a929841524bd656fe5342fc45816ab"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs",
    "sha256": "ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs",
    "sha256": "37012443ee787d1a05e7826e8c3a8ac35ea0be6d43eba6918dcd7076c15e8d93"
  }
]
```

Original domain fixture verification: count12, drift empty. Actor future remains Send; edit_text_from_envelope/apply_ops_binary slots and bodies, Plugin/ArtifactApp, Store detach, native resident authority and all six selected native laws were not edited by this packet.

No new native attempt is authorized or active. The next native inventory waits for parent review and the joint coherent source release.

