//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 6 of the cut by subject).

use super::*;

fn retire_original_envelope_cancel(cancel:semio_framework_job::CancelToken,source:RetainedCloneGrant){
    let mut owner=semio_framework_async::CancelTokenRetirement::from_token(cancel);
    for _ in 0..256{if owner.terminal_is_empty(){break}let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(source).unwrap());assert!(step.progress().fits(source));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));}
    assert!(owner.terminal_is_empty());let(_,born,freed)=semio_framework_trace_memory_testing::observe_backing(||drop(owner));assert_eq!((born,freed),(0,0));
}


/// ⚖️ LAW (design §12 composed children): a group carrying a committed tool transaction lands the SAME `TransactionRef`
/// on every operation of every member it touches — the parent and its owned child — because the ref rides each member's
/// own `Apply` command, so the ledger, the revision and the outbound envelopes all see it at apply time.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_stamps_one_tool_transaction_on_parent_and_owned_child() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-transaction-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-transaction-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");
    let parent_ops = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_ops = vec![DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode child op"), DemoMutation::SetN(SetN { n: 3 }).encode_op().expect("encode child op")];
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &child_ops, op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];
    let transaction = semio_framework_os_spr::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "s.stdio.demoparent@1/*#editor#drag".into() };

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta { transaction: Some(transaction.clone()), ..GroupMeta::default() }, &mut identity).await.expect("owned dispatch");

    assert_eq!(receipt.member_edits.len(), 2, "one edit per touched member");
    let parent_edit = parent_store.envelope().vcs.edits.last().expect("parent edit");
    let child_edit = child_store.envelope().vcs.edits.last().expect("child edit");
    assert_eq!(child_edit.mutation_meta.len(), 2, "both child ops land in one edit");
    for (member, edit) in [("parent", parent_edit), ("child", child_edit)] {
        assert!(edit.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&transaction)), "every {member} operation carries the group's transaction");
        assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(receipt.invocation_id.as_str())), "every {member} operation keeps the group identity");
    }
    let plain = coordinator.dispatch_group(
            &parent_ref,
            &mut parent_store,
            &mut [(&mut child_store, ChildDispatch { child: child_ref.clone(), ops: &[DemoMutation::SetN(SetN { n: 4 }).encode_op().expect("encode")], op_schema: &SchemaId("demo/v1".into()), labels: &[] })],
            Vec::new(),
            Vec::new(),
            GroupMeta::default(),
        &mut identity)
        .await
        .expect("plain dispatch");
    assert_eq!(plain.member_edits.len(), 1);
    assert!(child_store.envelope().vcs.edits.last().expect("child edit").mutation_meta.iter().all(|meta| meta.transaction.is_none()), "a group without a transaction stamps none");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


//#region 🔖️PhasePolicyTests
/// 🧪️ Normal (the default policy) rejects an Error-level message exactly like it already
/// rejects Fatal (`dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing` above)
/// — the SAME all-or-nothing law, now driven by `reject_if_policy_rejects` off the UNIONED
/// `preview_wire` messages rather than an immediate per-member `Err`. Exercises lane 1-A's REAL
/// `ArtifactStore::set_merge_policy` (§C6, landed) end to end — no test-only policy fixture
/// needed now that it exists.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_rejects_under_normal_when_a_member_yields_an_error_and_nothing_applies() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-normal".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-normal".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    assert_eq!(parent_store.merge_policy().await, semio_framework_os_spr::MergePolicy::Normal, "Normal is also the default, but set it explicitly so this test does not rely on that");
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected Normal policy to reject an Error-level message, but the dispatch succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, semio_framework_os_spr::MergePolicy::Normal, "the rejection must name the policy that actually rejected it");
            assert!(messages.iter().any(|message| message.level == semio_framework_diagnostic::Severity::Error), "the rejection must carry the Error message that triggered it");
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a policy-rejected group dispatch");
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ The SAME Error-level scenario `dispatch_group_phase1_rejects_under_normal_when_a_
/// member_yields_an_error_and_nothing_applies` rejects is accepted end-to-end under
/// `LaissezFaire` (only `Fatal` is rejected) — both members get a real edit, and the child's own
/// diff stays empty (§C2 LAW 2: an `Error` message's diff carries no change for the target) even
/// though the group as a whole succeeded.
///
/// 🎯️ Sets `LaissezFaire` on BOTH members, not just the parent: `reject_if_policy_rejects` is
/// only the coordinator's GROUP-level gate (checked once, against the parent's own policy,
/// before Phase 2 starts). Phase 2 still dispatches through each member's REAL `dispatch_wire`,
/// which (lane 1-A's now-landed C6 `ArtifactStore::dispatch`) independently enforces THAT
/// member's own `merge_policy()` — a child left at the Normal default would still reject its own
/// Error-level op right here, even though the coordinator's gate already accepted the group.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-lf".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-lf".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity).await.expect("LaissezFaire accepts an Error-level message");

    assert_eq!(receipt.member_edits.len(), 2, "both parent and child got a real edit despite the child's Error message");
    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(1), "the parent's own clean op still applied");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0), "the child's Error-level op's diff carries no change for its target (LAW 2), even though the group was accepted");
    assert!(receipt.messages.iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == semio_framework_diagnostic::Severity::Error), "the child's Error message must still be reported, even on acceptance");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `Vigilant` rejects a plain `Warning` — the strictest of the three policies, and the
/// one level `Normal` (the OTHER two policy tests above/below use) would have accepted.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_phase1_rejects_under_vigilant_on_a_members_warning() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-vigilant".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-vigilant".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::Vigilant);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let child_op = SeverityMutation::SetWarningN(SetWarningN { n: 5 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected Vigilant policy to reject a Warning-level message, but the dispatch succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, semio_framework_os_spr::MergePolicy::Vigilant, "the rejection must name the policy that actually rejected it");
            assert!(messages.iter().any(|message| message.level == semio_framework_diagnostic::Severity::Warning), "the rejection must carry the Warning message that triggered it");
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a policy-rejected group dispatch");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `GroupReceipt.messages` carries the FULL union (both parent's own and the child's),
/// each `target` prefixed with the ORIGINATING member's own `semio_framework_artifact_reference::ArtifactRef::
/// to_uri()` — the discipline that lets a caller with several members in flight tell messages
/// apart. Parent-first ordering matches phase 1's own collection order.
///
/// 🎯️ `dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>`: `MemberFactory` is
/// only required on the CHILDREN type (`Mc`) now — the parent/`Mp` bound dropped it (see
/// `dispatch_group`'s own doc comment for why: only children are ever genesis-constructed).
/// `parent_store` still uses this module's test wrapper regardless: `set_merge_policy`/
/// `merge_policy` are inherent methods the wrapper's own `impl SpaceMember` block
/// deliberately does not re-declare, so calling them on the wrapper autoderefs (via
/// `Deref`/`DerefMut`) straight to the real type's inherent method — no `SpaceMember`
/// vtable indirection, no `Normal`-default trap.
#[semio_framework_async_macros::async_test]
async fn group_receipt_messages_contains_the_union_with_member_path_prefixed_targets() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-parent-union".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "policy-child-union".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 }).encode_op().expect("encode parent op")];
    let child_op = SeverityMutation::SetWarningN(SetWarningN { n: 5 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity).await.expect("LaissezFaire accepts a Warning-level message");

    assert_eq!(receipt.messages.len(), 2, "one Warning from the parent, one from the child");
    assert_eq!(receipt.messages[0].level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(receipt.messages[0].code.0, "mutation.clamped");
    assert_eq!(receipt.messages[0].target, vec![parent_ref.to_uri()], "the parent's own message is prefixed with the parent's own path, collected before any child's");
    assert_eq!(receipt.messages[1].level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(receipt.messages[1].code.0, "mutation.clamped");
    assert_eq!(receipt.messages[1].target, vec![child_ref.to_uri()], "the child's message is prefixed with the CHILD's own path, not the parent's");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#endregion 🔖️PhasePolicyTests

//#region 🔖️RefusedReloadTests
/// 🛡️ A REFUSED reload RETURNS its `VcsError` — it never aborts the process. `set_state` owns the
/// candidate envelope it consumed and `ArtifactEnvelope`'s `Drop` asserts its owners were detached,
/// so letting a `?` drop a POPULATED candidate turned every malformed-document refusal into a
/// process abort instead of the error the caller is meant to read (measured 2026-09-21 as
/// `ArtifactEnvelope::drop ← drop_glue ← set_state::{closure#0} ← reset::{closure#0}`). The refusal
/// path hands the candidate to `ArtifactEnvelope::retire_unadopted`, the live store survives
/// unchanged, and it still closes to terminal-empty afterwards.
#[semio_framework_async_macros::async_test]
async fn a_refused_reload_returns_its_error_and_retires_the_candidate_it_consumed() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "refused-reload", DemoSnapshot { n: Some(3) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let mut rejected = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "refused-reload", DemoSnapshot { n: Some(9) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let orphan = vec![semio_framework_os_spr::EditMessages { edit_id: "edit-that-never-existed".to_string(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "orphan")] }];
    rejected.edit_messages = ArtifactEditMessageLedger::try_from_entries(orphan).unwrap_or_else(|_| panic!("one exact orphan message entry is admitted by the fixed ledger"));
    let error = store.reset(rejected).await.expect_err("a message ledger naming an edit the history never recorded must be refused");
    assert!(format!("{error:?}").contains("invalid message ledger"), "the caller reads the refusal itself: {error:?}");
    assert_eq!(store.snapshot_ref().n, Some(3), "the live store survives a refused reload unchanged");
    close_demo_artifact_store(&mut store);
}

//#endregion 🔖️RefusedReloadTests

//#region 🔖️ErasedCloseDemandTests
/// 🧱️ The ERASED close-byte demand a nested driver reads before it grants.
struct DemandingBufferRetirement {
    buffer: Option<Vec<u8>>,
}

#[test]
fn initialization_runtime_preserves_physical_backing_and_terminal_cursor_box() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let policy=physical_test_close_grant();
    let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<DemoSnapshot>>=Arc::new(DemoInitialSnapshotRetirementFactory);
    for method in ["settle", "close"] {
        for row in fixture["cases"].as_array().unwrap() {
            let extent=row["physicalBytes"].as_u64().unwrap() as usize;
            let mut buffer=Vec::new();buffer.try_reserve_exact(extent).unwrap();assert_eq!(buffer.capacity(),extent);
            let mut runtime=ArtifactStoreInitializationRuntime::new("physical-runtime","demo/v1",Arc::new(DemoSnapshot { n:Some(0) }),[7;32],ActorId("actor:runtime-release".into()));
            let(owner,birth)=admit_artifact_retirement(buffer,policy,|original|DemandingBufferRetirement { buffer:Some(original) }).unwrap_or_else(|(error,_)|panic!("funded runtime cursor birth refused: {error}"));
            assert!(birth.fits(policy));*runtime.close_active=Some(owner);
            let demand=runtime.next_close_release_byte_demand().unwrap();
            let advance=|runtime:&mut ArtifactStoreInitializationRuntime<DemoSnapshot>,grant:RetainedCloneGrant|if method=="settle" { runtime.settle_current_retirement_step(grant) } else { runtime.close_step(&factory,grant) };
            let caller=row["callerBytes"].as_u64().unwrap() as usize;
            let first_grant=RetainedCloneGrant { maximum_release_bytes: caller,..policy };
            let(first,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,first_grant).unwrap());
            assert!(first.progress().fits(first_grant));assert_eq!(allocated,0);assert_eq!(released,first.progress().released_bytes);
            assert_eq!(first.progress().released_bytes,row["releasedBytes"].as_u64().unwrap() as usize);assert_eq!(demand,extent);
            if released==0 {
                assert!(runtime.close_active.is_some());assert_eq!(runtime.next_close_release_byte_demand().unwrap(),extent);
                let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((allocated,released),(0,extent));
            }
            let frame=size_of::<DemandingBufferRetirement>();assert_eq!(runtime.next_close_release_byte_demand().unwrap(),frame);
            let(denied,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,RetainedCloneGrant { maximum_release_bytes:frame-1,..policy }).unwrap());
            assert_eq!((allocated,released),(0,0));assert_eq!(denied.progress(),RetainedCloneProgress::default());assert!(runtime.close_active.is_some());
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,policy).unwrap());assert_eq!((allocated,released),(0,frame));assert_eq!(step.progress().released_bytes,frame);assert!(runtime.close_active.is_none());
            for _ in 0..1024 {
                if runtime.terminal_is_empty() { break; }
                let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||runtime.close_step(&factory,policy).unwrap());
                assert!(step.progress().fits(policy));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            }
            assert!(runtime.terminal_is_empty());
        }
    }
}

impl ErasedSnapshotRetirement for DemandingBufferRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(buffer)=self.buffer.as_ref() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original buffer requires supplied depth")); }
        if grant.maximum_release_bytes<buffer.capacity() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let released=self.buffer.take().expect("original physical buffer remains present").capacity();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,released_bytes: released,..Default::default() }))
    }
    fn terminal_is_empty(&self)->bool { self.buffer.is_none() }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.buffer.as_ref().map_or(0,Vec::capacity))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.buffer.is_some()))}
}

struct NestedErasedDriver { inner:Option<Box<dyn ErasedSnapshotRetirement>> }

impl NestedErasedDriver {
    fn close(&mut self, grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{artifact_retirement_box_close_step(&mut self.inner,grant)}
}

struct UnsupportedDemandRetirement;

impl ErasedSnapshotRetirement for UnsupportedDemandRetirement {
    fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owner has no admitted physical retirement contract"))}
    fn terminal_is_empty(&self)->bool{false}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"copy demand is unsupported"))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"capacity demand is unsupported"))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"release demand is unsupported"))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"depth demand is unsupported"))}
}

/// ⚖️ Original buffer and cursor frames require separate actual caller release grants; demand earns no authority.
#[test]
fn an_erased_retirement_publishes_its_physical_close_demand_and_a_nested_driver_consumes_supplied_grants() {
    let unsupported=UnsupportedDemandRetirement;
    assert_eq!(unsupported.next_release_byte_demand().unwrap_err().kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    let grant=physical_test_close_grant();
    let mut buffer=Vec::with_capacity(8192);buffer.resize(8192,0u8);let pointer=buffer.as_ptr();assert_eq!(buffer.capacity(),8192);
    let (owner,birth)=admit_artifact_retirement(buffer,grant,|original|DemandingBufferRetirement { buffer:Some(original) }).unwrap_or_else(|(error,_)|panic!("original buffer cursor birth refused: {error}"));
    assert_eq!(birth.retained_capacity_bytes,std::mem::size_of::<DemandingBufferRetirement>());
    let mut nested=NestedErasedDriver { inner:Some(owner) };
    assert_eq!(nested.inner.as_ref().unwrap().next_release_byte_demand().unwrap(),8192);
    for denied in [RetainedCloneGrant { maximum_items:0,..grant },RetainedCloneGrant { maximum_release_bytes:8191,..grant },RetainedCloneGrant { maximum_depth:0,..grant }] {
        let (result,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(denied));assert_eq!((allocated,released),(0,0));
        if denied.maximum_depth==0 { assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit); } else { assert_eq!(result.unwrap().progress(),RetainedCloneProgress::default()); }
        assert!(nested.inner.is_some());assert_eq!(nested.inner.as_ref().unwrap().next_release_byte_demand().unwrap(),8192);
    }
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(RetainedCloneGrant { maximum_copy_bytes:1,maximum_release_bytes:8192,..grant }).unwrap());
    assert_eq!((allocated,released),(0,8192));assert_eq!(step.progress(),RetainedCloneProgress { copied_items:1,released_bytes:8192,..Default::default() });
    assert!(nested.inner.as_ref().unwrap().terminal_is_empty());
    let frame=std::mem::size_of::<DemandingBufferRetirement>();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(RetainedCloneGrant { maximum_release_bytes:frame-1,..grant }).unwrap());assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert!(nested.inner.is_some());
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(grant).unwrap());assert_eq!((allocated,released),(0,frame));assert_eq!(step.progress().released_bytes,frame);assert!(nested.inner.is_none());
}

//#endregion 🔖️ErasedCloseDemandTests

/// 🪪️ Fixture law `🧫️fixtures/🪪️replica-edit-identity.json`: fresh replicas that author identical
/// gestures never share an edit or wire operation id (the hub ledger dedupes by id).
#[semio_framework_async_macros::async_test]
async fn fresh_replicas_authoring_identical_gestures_never_share_an_edit_or_operation_id() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️replica-edit-identity.json")).expect("replica identity fixture");
    let mut edit_ids = std::collections::HashSet::new();
    let mut operation_ids = std::collections::HashSet::new();
    for _ in 0..fixture["replicas"].as_u64().expect("replica count") {
        let genesis = DemoSnapshot { n: Some(fixture["genesisN"].as_i64().expect("genesis n") as i32) };
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>(fixture["artifactSchema"].as_str().expect("schema"), fixture["documentId"].as_str().expect("document"), genesis, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        for gesture in fixture["gestures"].as_array().expect("gestures") {
            let mutations = gesture["ops"]
                .as_array()
                .expect("gesture ops")
                .iter()
                .map(|op| match (op.get("set"), op.get("add")) {
                    (Some(n), None) => DemoMutation::SetN(SetN { n: n.as_i64().expect("set n") as i32 }),
                    (None, Some(delta)) => DemoMutation::AddN(AddN { delta: delta.as_i64().expect("add delta") as i32 }),
                    _ => panic!("fixture op must be exactly one of set/add"),
                })
                .collect();
            store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, &mut identity).await.expect("fixture gesture applies");
        }
        let edits = &store.log.envelope.vcs.edits;
        assert_eq!(edits.len() as u64, fixture["expect"]["editsPerReplica"].as_u64().expect("edits per replica"));
        let operations: Vec<String> = edits.iter().flat_map(|edit| semio_framework_os_spr::mutation_ids_for_edit::<DemoSnapshot, DemoMutation>(edit)).map(|id| id.0).collect();
        assert_eq!(operations.len() as u64, fixture["expect"]["operationIdsPerReplica"].as_u64().expect("operations per replica"));
        edit_ids.extend(edits.iter().map(|edit| edit.id.clone()));
        operation_ids.extend(operations);
        close_demo_artifact_store(&mut store);
    }
    assert_eq!(edit_ids.len() as u64, fixture["expect"]["distinctEditIds"].as_u64().expect("distinct edits"));
    assert_eq!(operation_ids.len() as u64, fixture["expect"]["distinctOperationIds"].as_u64().expect("distinct operations"));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


#[semio_framework_async_macros::async_test]
async fn dispatch_group_borrowed_child_keeps_exact_sources_on_policy_refusal() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️child-dispatch/🔣️.json")).expect("neutral borrowed child declaration");
    let parent_ref=semio_framework_artifact_reference::ArtifactRef{artifact_id:fixture["parent"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.demoparent".into(),standard:"1".into(),subset:"*".into()}};
    let child_ref=semio_framework_artifact_reference::ArtifactRef{artifact_id:fixture["child"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.demochild".into(),standard:"1".into(),subset:"*".into()}};
    let mut parent=ArtifactStore::new(create_document_envelope::<DemoSnapshot,ValidatedMutation>("demo/v1",&parent_ref.artifact_id,DemoSnapshot{n:Some(0)},None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child=ArtifactStore::new(create_document_envelope::<DemoSnapshot,ValidatedMutation>("demo/v1",&child_ref.artifact_id,DemoSnapshot{n:Some(0)},None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator=CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id,fixture["slot"].as_str().unwrap(),&child_ref.artifact_id).await.unwrap();
    let source=vec![ValidatedMutation::SetN(ValidatedSetN{n:fixture["invalidN"].as_i64().unwrap() as i32}).encode_op().unwrap()];
    let expected=source[0].clone();
    let bytes=source[0].as_ptr();
    let cells=source.as_ptr();
    let schema=SchemaId(fixture["opSchema"].as_str().unwrap().into());
    let labels=[semio_framework_ui_locale::LocalizedLabel::data(fixture["label"].as_str().unwrap().to_owned())];
    let dispatch=ChildDispatch::borrowed(child_ref,&source,&schema,&labels);
    assert_eq!(dispatch.ops.as_ptr(),cells);
    assert_eq!(dispatch.ops[0].as_ptr(),bytes);
    assert!(std::ptr::eq(dispatch.op_schema,&schema));
    assert_eq!(dispatch.labels.as_ptr(),labels.as_ptr());
    let mut children=[(&mut child,dispatch)];
    let result=coordinator.dispatch_group(&parent_ref,&mut parent,&mut children,Vec::new(),Vec::new(),GroupMeta::default(), &mut identity).await;
    assert!(matches!(result,Err(VcsError::Rejected{policy:semio_framework_os_spr::MergePolicy::Normal,..})));
    assert_eq!(children[0].1.ops.as_ptr(),cells);
    assert_eq!(children[0].1.ops[0].as_ptr(),bytes);
    assert_eq!(children[0].1.ops[0],expected);
    drop(children);
    assert!(parent.envelope().vcs.edits.is_empty()&&child.envelope().vcs.edits.is_empty());
    assert_eq!(source[0].as_ptr(),bytes);
    assert_eq!(source[0],expected);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[test]
fn immutable_stored_genesis_obeys_the_neutral_law() {
    #[derive(Debug, PartialEq)]
    struct GenesisLawSnapshot { n: i64 }
    static ENCODED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static DECODED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    struct GenesisLawProducer;
    impl semio_framework_os_vcs::io::binary::genesis::ArtifactGenesisProducer<GenesisLawSnapshot> for GenesisLawProducer {
        fn encode_genesis_pack(&self,snapshot:&GenesisLawSnapshot) -> Result<Vec<u8>, ValueError> { ENCODED.fetch_add(1, std::sync::atomic::Ordering::SeqCst); Ok(format!("{{\"n\":{}}}", snapshot.n).into_bytes()) }
        fn decode_genesis_pack(&self,pack: &[u8]) -> Result<GenesisLawSnapshot, ValueError> { DECODED.fetch_add(1, std::sync::atomic::Ordering::SeqCst); let value: serde_json::Value = serde_json::from_slice(pack).unwrap(); Ok(GenesisLawSnapshot { n: value["n"].as_i64().unwrap() }) }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    let born = AdmittedArtifactGenesis::born(GenesisLawSnapshot { n: 0 },&GenesisLawProducer).unwrap();
    assert_eq!(ENCODED.load(std::sync::atomic::Ordering::SeqCst), 1);
    let mut digests = Vec::new();
    for row in fixture["packs"].as_array().unwrap() {
        let raw = row["hex"].as_str().unwrap();
        let bytes: Vec<u8> = raw.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
        let stored:AdmittedArtifactGenesis<GenesisLawSnapshot> = semio_framework_os_vcs::io::text::genesis::native_genesis_from_value(DslValue::String(raw.into()),&GenesisLawProducer).unwrap();
        assert_eq!(stored.facts().snapshot().n, row["decoded"]["n"].as_i64().unwrap());
        assert_eq!(stored.pack(), bytes);
        assert_eq!(stored.to_value(), DslValue::String(raw.into()));
        assert_eq!(stored.facts().digest(), *semio_framework_hash::hash(&bytes).as_bytes());
        let alias = stored.facts().share_snapshot();
        let cloned = stored.clone();
        assert!(Arc::ptr_eq(&alias, &cloned.facts().share_snapshot()));
        assert!(Arc::ptr_eq(&stored.share_pack(), &cloned.share_pack()));
        assert_eq!(stored.facts().digest(), cloned.facts().digest());
        digests.push(stored.facts().digest());
    }
    assert_eq!(ENCODED.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(DECODED.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert_eq!(born.facts().digest(), digests[0]);
    assert_ne!(digests[0], digests[1]);
    assert!(semio_framework_os_vcs::io::text::genesis::decode_genesis_hex_controlled("7B",&mut semio_framework_value::NativeDecodeControl::new(4096,&mut |_|true)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn opened_actor_ownership_obeys_the_neutral_law() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    let actors = &fixture["actors"];
    let actor = ActorId(actors["opened"].as_str().unwrap().into());
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "actor-genesis-law", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut real = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, actor.clone()).await.unwrap();
    real.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    assert!(Arc::ptr_eq(&*real.live.current, &real.envelope().vcs.genesis.facts().share_snapshot()));
    let mut live = ArtifactStore(real, Some(close_test_store::<DemoSnapshot, DemoMutation>));
    live.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.unwrap();
    let first = live.envelope().vcs.edits.iter().next().unwrap();
    assert_eq!(first.actor.as_deref(), actors["bareAuthor"].as_str());
    assert_eq!(first.mutation_meta[0].author_id.as_ref(), Some(&actor));
    let remote = foreign_mutation_envelope(actors["explicitAuthor"].as_str().unwrap(), DemoMutation::SetN(SetN { n: 2 })).await;
    live.dispatch(ArtifactCommand::IngestRemote { envelope: remote }, &mut identity).await.unwrap();
    let foreign = live.envelope().vcs.edits.iter().last().unwrap();
    assert_eq!(foreign.actor.as_deref(), actors["explicitAuthor"].as_str());
    assert_eq!(live.local_actor_id(), &actor);
    live.dispatch(ArtifactCommand::Undo, &mut identity).await.unwrap();
    assert_eq!(live.envelope().transitions.last().unwrap().actor.0.as_str(), actors["transitionAuthor"].as_str().unwrap());
    let files = print_document_pack(live.envelope()).await.unwrap();
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap();
    let mut reopened = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(parsed.into_envelope(), actor.clone()).await.unwrap();
    reopened.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    assert_eq!(reopened.local_actor_id().0.as_str(), actors["reopened"].as_str().unwrap());
    close_test_store(&mut reopened);
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap();
    let mut candidate = parsed.into_envelope();
    candidate.vcs.edits.iter_mut().next().unwrap().actor = None;
    assert!(semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(candidate, actor).await.is_err());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[test]
fn stored_genesis_capture_is_bounded_and_cancellable() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    for row in fixture["packs"].as_array().unwrap() {
        let hex = row["hex"].as_str().unwrap();
        for fuel in [1, 2, 7] {
            let raw = format!("{{\"schema\":\"{hex}\",\"id\":\"capture\"}}");
            let mut cursor = owned_schema_test_cursor(&[raw.as_bytes()]);
            let cancel = semio_framework_job::root_cancel_token();
            let mut sequence = 0;
            let token = loop {
                let mut original_retained_progress_8=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(7, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_8);
                match cursor.step(&mut cx) { OwnedSchemaRecordStep::FieldToken { field_id: 1, token, .. } => break token, OwnedSchemaRecordStep::Pending => {}, _ => panic!("canonical Pack token") }
            };
            let mut original_retained_progress_9=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_9);
            let mut capture = ArtifactGenesisPackCapture::new(token, &cx).unwrap();
            for _ in 0..hex.len()+2 {
                let before = capture.relative;
                let mut original_retained_progress_10=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_10);
                let complete = capture.step(&cursor, &mut cx).unwrap();
                assert!(capture.relative-before <= fuel as usize);
                if complete { break; }
            }
            assert!(capture.complete);
            assert_eq!(capture.bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), hex);
            assert!(capture.hasher.finalize() == semio_framework_hash::hash(&capture.bytes));
            let mut original_retained_progress_11=semio_framework_value::RetainedCloneProgress::default();
        let mut stale = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(2), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_11);
            let before = capture.bytes.len();
            assert!(capture.step(&cursor, &mut stale).is_err());
            assert_eq!(capture.bytes.len(), before);
            cancel.cancel_now();
            let mut original_retained_progress_12=semio_framework_value::RetainedCloneProgress::default();
        let mut cancelled = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()), cancel, semio_framework_job::default_now_us, &mut sequence, &mut original_retained_progress_12);
            assert!(capture.step(&cursor, &mut cancelled).is_err());
            assert_eq!(capture.bytes.len(), before);
            let grant=physical_test_close_grant();
            let(retirement,receipt)=semio_framework_value::factory::OwnedValueRetirementFactory::<Vec<u8>>::default().retire_owned(capture.bytes,grant).unwrap_or_else(|(error,_)|panic!("original captured bytes admission refused: {error}"));
            assert!(receipt.fits(grant));drive_retirement_terminal(retirement);
            let grant=schema_test_close_grant();
            for _ in 0..8 { if cursor.terminal_is_empty() { break; }let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes)); }
            assert!(cursor.terminal_is_empty());
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_hydrators_refuse_actorless_history_from_the_neutral_law() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️actor-genesis/🔣️.json")).unwrap();
    for config in [false, true] {
        for row in fixture["retainedAdmissions"].as_array().unwrap() {
            let initial = DemoSnapshot { n: Some(0) };
            let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "actorless-hydration", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
            if config { envelope.history_shape = semio_framework_os_spr::HistoryShape::Config; }
            else { envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() }); }
            let mut live = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
            live.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.unwrap();
            let files = print_document_pack(live.envelope()).await.unwrap();
            let mut history = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.unwrap();
            history.edits[0].actor = row["editAuthor"].as_str().map(semio_framework_value::SharedUtf8::from);
            history.edits[0].meta.as_mut().unwrap()[0].author_id = row["operationAuthor"].as_str().map(semio_framework_value::SharedUtf8::from);
            let actor = ActorId(fixture["actors"]["opened"].as_str().unwrap().into());
            let mut rejected = false;
            if config {
                let mut hydration = RetainedConfigStoreHydration::<DemoSnapshot, DemoMutation>::from_snapshots(live.envelope().vcs.genesis.clone(), initial.clone(), initial.clone(), history, live.envelope().id.clone(), "demo/v1".into(), complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new().member_store_owners(physical_test_close_grant())), 0, 65536, actor);
                for _ in 0..1_000_000 { match hydration.advance(physical_test_close_grant()) { ConfigStoreHydrationStep::Pending(_) => {}, ConfigStoreHydrationStep::Rejected(diagnostic) => {  rejected = true; break; }, ConfigStoreHydrationStep::Ready(mut store) => { close_test_store(&mut store); break; } } }
                let mut closed = false;
                for _ in 0..100000 { if hydration.terminal_is_empty() { closed = true; break; } let step=hydration.close_step(physical_test_close_grant()).unwrap();assert!(step.progress().fits(physical_test_close_grant())); }
                assert!(closed, "retained hydration closes exactly");
            } else {
                let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: live.envelope().id.clone(), dialect: live.envelope().dialect.clone().unwrap() };
                let (operation, generation) = (semio_framework_job::OperationId(801), semio_framework_job::Generation(1));
                let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_decoded_pack(initial, files.pack, live.envelope().vcs.genesis.facts().digest(), history, expected, None, "demo/v1".into(), complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new().member_store_owners(physical_test_close_grant())), operation, generation, u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 }, actor);
                let mut sequence = 0;
                for _ in 0..1_000_000 {
                    let mut original_retained_progress_13=semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(64, u64::MAX, physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_13);
                    match hydration.step(&mut cx, physical_test_close_grant()) { PersistedDocumentHydrationStep::Pending(_) => {}, PersistedDocumentHydrationStep::Rejected(diagnostic) => {  rejected = true; break; }, PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(mut store)) => { close_test_store(&mut store); break; }, _ => panic!("requested store output") }
                }
                let mut closed = false;
                for _ in 0..100000 { if hydration.terminal_is_empty() { closed = true; break; } let step=hydration.close_step(physical_test_close_grant()).unwrap();assert!(step.progress().fits(physical_test_close_grant())); }
                assert!(closed, "retained hydration closes exactly");
            }
            assert_eq!(!rejected, row["accepted"].as_bool().unwrap(), "config={config}, authors={row}");
        }
    }
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧮️ The fresh VCS release driver funds the actual retained allocation published by its erased owner.
#[test]
fn fresh_vcs_release_publishes_actual_retained_backing_demand() {
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct UnusedEditDecoder;
    impl ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>> for UnusedEditDecoder {
        fn begin_entry(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath, _: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<DemoMutation>>>) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<DemoMutation>>> { panic!("the release-only fixture never decodes a history entry") }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut backing = Vec::<u8>::new(); backing.try_reserve_exact(extent).unwrap(); assert_eq!(backing.capacity(), extent);
        let mut owner = ArtifactEnvelopeFreshVcsAuthority::<DemoSnapshot, DemoMutation> {
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS }).unwrap(),
            snapshot: None, snapshot_target: ArtifactEnvelopeFreshSnapshotTarget::new(), genesis_pack: None, genesis_bytes: None, active: None, pending: None,
            edits: None, changes: None, checkpoints: None, alternatives: None, value: None,
            retirement: Some(Box::new(DemandingBufferRetirement { buffer: Some(backing) })),
            initial_snapshot_factory: Arc::new(DemoInitialSnapshotRetirementFactory), mutation_factory: Arc::new(DemoMutationRetirementFactory), edit_decoder: Arc::new(UnusedEditDecoder),
            maximum_snapshot_close_byte_demand: admission, maximum_retained_snapshot_close_bytes: admission, terminal: false,
        };
        let advertised = owner.next_close_release_byte_demand().unwrap();
        let physical = owner.retirement.as_ref().unwrap().next_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (caller_step, _, caller_released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let caller_expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut progress = vec![(caller_step, caller_released)];
        for _ in 0..512 {
            if owner.terminal_is_empty() { break; }
            let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() };
            let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap());
            progress.push((step, released));
        }
        let funded_terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty(), "exact independently admitted allocation grant drains the fixture before assertions");
        let exact_accounting = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if advertised != physical || !funded_terminal || !exact_accounting || caller_released != caller_expected { failures.push((extent, advertised, physical, funded_terminal, exact_accounting)); }
    }
    assert!(failures.is_empty(), "the fresh VCS driver publishes sufficient exact physical demands: {failures:?}");
}

/// 🪜️ VCS retirement forwards each exact nested allocation demand without losing physical cursor releases.
#[test]
fn vcs_retirement_propagates_nested_physical_allocation_demand() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut buffer = Vec::new(); buffer.try_reserve_exact(extent).unwrap(); assert_eq!(buffer.capacity(), extent);
        let mut owner = ArtifactStoreVcsRetirement::<DemoSnapshot, DemoMutation> {
            vcs: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(Some(Box::new(DemandingBufferRetirement { buffer: Some(buffer) }))),
            phase: ArtifactStoreVcsRetirementPhase::Complete, initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(Arc::new(DemoInitialSnapshotRetirementFactory))), mutation_factory: std::mem::ManuallyDrop::new(Some(Arc::new(DemoMutationRetirementFactory))), closing_factories: std::mem::ManuallyDrop::new([None,None]),
        };
        let demand = owner.next_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let caller_grant = RetainedCloneGrant { maximum_release_bytes: caller, maximum_capacity_bytes: admission, ..physical_test_close_grant() };
        let (caller_step, _, caller_released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(caller_grant).unwrap());
        let caller_expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut progress = vec![(caller_step, caller_released)];
        for _ in 0..512 { if owner.terminal_is_empty() { break; } let grant=RetainedCloneGrant { maximum_capacity_bytes:admission,maximum_release_bytes:admission,..physical_test_close_grant() };let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));progress.push((step, released)); }
        let terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } owner.close_step(RetainedCloneGrant { maximum_capacity_bytes:admission,maximum_release_bytes:admission,..physical_test_close_grant() }).unwrap(); }
        assert!(owner.terminal_is_empty());
        let exact_accounting = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if demand != extent || !terminal || !exact_accounting || caller_released != caller_expected { failures.push((extent, demand, terminal, exact_accounting)); }
    }
    assert!(failures.is_empty(), "VCS retirement publishes and funds exact nested physical allocations: {failures:?}");
}

/// 📋️ The fresh envelope field driver forwards its retained target's physical release demand.
#[test]
fn fresh_field_release_propagates_retained_target_allocation_demand() {
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct ReleaseOnlyCatalog;
    impl ArtifactEnvelopeOwnedFieldCatalog<DemoSnapshot, DemoMutation> for ReleaseOnlyCatalog {
        fn begin_vcs(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<DemoSnapshot, DemoMutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>>> { panic!("release-only catalog") }
        fn maximum_vcs_close_byte_demand(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
        fn maximum_retained_vcs_close_bytes(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES }
        fn begin_snapshot(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>> { panic!("release-only catalog") }
        fn begin_mutation(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<DemoMutation>> { panic!("release-only catalog") }
        fn begin_spr_conflict(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority> { panic!("release-only catalog") }
        fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>>> { panic!("release-only catalog") }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut buffer = Vec::new(); buffer.try_reserve_exact(extent).unwrap(); assert_eq!(buffer.capacity(), extent);
        let mut owner = ArtifactEnvelopeFreshFieldDecoder::new(semio_framework_job::OperationId(7002), semio_framework_job::Generation(1), Arc::new(ReleaseOnlyCatalog), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), ArtifactEnvelopeCompletedRecordRegistry::new(), ArtifactEnvelopeDecodeCompletion::new());
        owner.active_retirement = Some(Box::new(DemandingBufferRetirement { buffer: Some(buffer) }));
        let demand = owner.next_close_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (caller_step, _, caller_released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let caller_expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut progress = vec![(caller_step, caller_released)];
        for _ in 0..8 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap()); progress.push((step, released)); }
        let terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty());
        let exact_accounting = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if demand != extent || !terminal || !exact_accounting || caller_released != caller_expected { failures.push((extent, demand, terminal, exact_accounting)); }
    }
    assert!(failures.is_empty(), "fresh field release publishes and funds exact retained target allocations: {failures:?}");
}

/// 🧱️ A terminal typed snapshot cursor retains its actual scaffold allocation until its whole physical grant arrives.
#[test]
fn fresh_vcs_preserves_terminal_snapshot_cursor_box_under_grant() {
    struct TerminalSnapshotCursor<const N: usize>([u8; N]);
    impl<const N: usize> ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot> for TerminalSnapshotCursor<N> {
        fn accept_token(&mut self, _: OwnedSchemaToken, _: bool, _: &OwnedSchemaRecordCursor, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> { panic!("terminal cursor has no decode work") }
        fn publish_reserved(&mut self, _: &mut dyn ArtifactEnvelopeSnapshotFieldTarget<DemoSnapshot>, _: ArtifactEnvelopeFieldReservation, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> { panic!("terminal cursor has no publish work") }
        fn maximum_close_byte_demand(&self) -> usize { N }
        fn maximum_retained_close_bytes(&self) -> usize { N }
        fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct UnusedEditDecoder;
    impl ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>> for UnusedEditDecoder {
        fn begin_entry(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath, _: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<DemoMutation>>>) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<DemoMutation>>> { panic!("terminal snapshot fixture has no history decode work") }
    }
    fn check<const N: usize>(row: &serde_json::Value, admission: usize) -> (bool, usize, usize, usize, RetainedCloneStep) {
        let mut owner = ArtifactEnvelopeFreshVcsAuthority::try_new(Box::new(TerminalSnapshotCursor::<N>([0; N])), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Arc::new(UnusedEditDecoder)).unwrap_or_else(|_| panic!("bounded terminal snapshot scaffold admits"));
        let demand = owner.next_close_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let retained = owner.snapshot.is_some();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty());
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let reported = step.progress().released_bytes;
        (demand == N && released == expected && reported == released && retained == (expected == 0), N, demand, released, step)
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut results = Vec::new();
    for row in fixture["cases"].as_array().unwrap() { results.push(match row["physicalBytes"].as_u64().unwrap() { 1 => check::<1>(row, admission), 16384 => check::<16384>(row, admission), 65536 => check::<65536>(row, admission), 262144 => check::<262144>(row, admission), _ => panic!("explicit neutral cursor extents") }); }
    assert!(results.iter().all(|row| row.0), "typed terminal snapshot cursor requires exact physical funding: {results:?}");
}

/// 🪺️ The active history array exposes its exact nested edit decoder retirement and scaffold demand.
#[test]
fn fresh_vcs_active_history_decoder_propagates_physical_release_demand() {
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct ReleaseOnlyCatalog;
    impl ArtifactEnvelopeOwnedFieldCatalog<DemoSnapshot, DemoMutation> for ReleaseOnlyCatalog {
        fn begin_vcs(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<DemoSnapshot, DemoMutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>>> { panic!("release-only catalog") }
        fn maximum_vcs_close_byte_demand(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
        fn maximum_retained_vcs_close_bytes(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES }
        fn begin_snapshot(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>> { panic!("release-only catalog") }
        fn begin_mutation(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<DemoMutation>> { panic!("release-only catalog") }
        fn begin_spr_conflict(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority> { panic!("release-only catalog") }
        fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>>> { panic!("release-only catalog") }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut backing = Vec::new(); backing.try_reserve_exact(extent).unwrap(); assert_eq!(backing.capacity(), extent);
        let catalog = Arc::new(ReleaseOnlyCatalog);
        let mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>> = Arc::new(DemoMutationRetirementFactory);
        let edit_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<DemoMutation>>> = Arc::new(ArtifactStoreDecodedEditRetirementFactory { mutation_factory: mutation_factory.clone() });
        let decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>>> = Arc::new(ArtifactOwnedSprEditDecoder { catalog: catalog.clone(), mutation_factory: mutation_factory.clone() });
        let mut edit = ArtifactOwnedSprEditAuthority::new(semio_framework_job::OperationId(7003), semio_framework_job::Generation(1), OwnedSchemaPath::ROOT, catalog.clone(), mutation_factory.clone(), edit_factory.clone());
        *edit.retirement = Some(Box::new(DemandingBufferRetirement { buffer: Some(backing) }));
        let mut array = OwnedSchemaBoundedArrayAuthority::new(OwnedSchemaPath::ROOT, edit_factory.clone(), decoder.clone());
        *array.values = None;
        *array.active_decoder = Some(Box::new(edit));
        let mut owner = ArtifactEnvelopeFreshVcsAuthority::<DemoSnapshot, DemoMutation> {
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS }).unwrap(), snapshot: None, snapshot_target: ArtifactEnvelopeFreshSnapshotTarget::new(), genesis_pack: None, genesis_bytes: None,
            active: Some(ArtifactEnvelopeFreshVcsActive::Edits(array)), pending: None, edits: None, changes: None, checkpoints: None, alternatives: None, value: None, retirement: None,
            initial_snapshot_factory: Arc::new(DemoInitialSnapshotRetirementFactory), mutation_factory: mutation_factory.clone(), edit_decoder: decoder.clone(), maximum_snapshot_close_byte_demand: admission, maximum_retained_snapshot_close_bytes: admission, terminal: false,
        };
        let demand = owner.next_close_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (step, _, actual) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let mut progress = vec![(step, actual)];
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap()); progress.push((step, released)); }
        let terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty());
        let exact = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if demand != extent || actual != row["releasedBytes"].as_u64().unwrap() as usize || !terminal || !exact { failures.push((extent, demand, terminal, exact)); }
    }
    assert!(failures.is_empty(), "active history decoder preserves and funds every exact retained allocation: {failures:?}");
}

/// 🧱️ Envelope and immutable genesis owners retain every terminal cursor until its exact physical grant arrives.
#[test]
fn envelope_and_genesis_preserve_terminal_cursor_physical_grants() {
    struct TerminalCursor<const N: usize>([u8; N]);
    impl<const N: usize> ErasedSnapshotRetirement for TerminalCursor<N> {
        fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
        fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct TerminalSnapshotFactory<const N: usize>;
    impl<const N: usize> ArtifactOwnedValueRetirementFactory<DemoSnapshot> for TerminalSnapshotFactory<N> {
        fn retirement_birth_bytes(&self,_:&DemoSnapshot)->usize { std::mem::size_of::<TerminalCursor<N>>() }
        fn retire_owned(&self,original:DemoSnapshot,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,DemoSnapshot)> {
            admit_artifact_retirement(original,grant,|_|TerminalCursor::<N>([0;N]))
        }
    }
    fn check<const N: usize>(row: &serde_json::Value, admission: usize, failures: &mut Vec<(String, usize, usize, usize)>) {
        let grant=RetainedCloneGrant {maximum_capacity_bytes:admission,maximum_release_bytes:admission,..physical_test_close_grant()};
        let ((terminal,birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_artifact_retirement([0u8;N],grant,TerminalCursor::<N>).unwrap_or_else(|_|panic!("original fixed policy funds terminal cursor source")));
        assert!(birth.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,0));
        let envelope=ArtifactStoreEnvelopeRetirement::<DemoSnapshot,DemoMutation> {vcs:std::mem::ManuallyDrop::new(None),messages:std::mem::ManuallyDrop::new(None),metadata:std::mem::ManuallyDrop::new(None),active:std::mem::ManuallyDrop::new(Some(terminal))};
        let (envelope,envelope_birth)=admit_artifact_retirement(envelope,grant,|original|original).unwrap_or_else(|_|panic!("fixed source policy funds original envelope frame"));assert!(envelope_birth.fits(grant));
        let pack=Vec::new();let digest=*semio_framework_hash::hash(&pack).as_bytes();
        let required=AdmittedArtifactGenesis::<DemoSnapshot>::verified_pack_birth_demand().unwrap();
        let ((error,original,pack),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||AdmittedArtifactGenesis::admit_verified_pack(DemoSnapshot{n:Some(7)},pack,digest,RetainedCloneGrant {maximum_capacity_bytes:required.capacity_bytes-1,..grant}).unwrap_err());
        assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(original.n,Some(7));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let ((genesis,genesis_birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||AdmittedArtifactGenesis::admit_verified_pack(original,pack,digest,grant).unwrap_or_else(|_|panic!("actual fixed policy admits original verified Genesis")));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(genesis_birth.retained_capacity_bytes,0));assert!(genesis_birth.fits(grant));
        let source_bytes=semio_framework_value::factory_arc_birth_bytes::<TerminalSnapshotFactory<N>>();
        semio_framework_value::retained_clone::RetainedCloneBirthDemand {capacity_bytes:source_bytes,depth:1}.admit(grant).unwrap();
        let (factory,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Arc::new(TerminalSnapshotFactory::<N>));assert_eq!((heap.requested_bytes,heap.released_bytes),(source_bytes,0));
        let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<DemoSnapshot>>=factory;
        let ((genesis,owner_birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_artifact_retirement((genesis,factory),grant,|(original,issuer)|ArtifactGenesisRetirement::new(original,issuer)).unwrap_or_else(|_|panic!("fixed policy admits actual Genesis retirement frame")));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(owner_birth.retained_capacity_bytes,0));assert!(owner_birth.fits(grant));
        for (name,mut owner) in [("envelope",envelope),("genesis",genesis)] {
            let original=owner.as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
            let mut found=false;
            for _ in 0..64 {
                if owner.next_release_byte_demand().unwrap()==N { found=true;break; }
                assert!(!owner.terminal_is_empty(),"real original owner must reach its actual terminal child frame");
                let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            }
            assert!(found,"actual {name} reaches original {N}-byte terminal child frame");
            let demand=owner.next_release_byte_demand().unwrap();let caller=row["callerBytes"].as_u64().unwrap() as usize;
            let (step,allocated,actual)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(RetainedCloneGrant {maximum_release_bytes: caller,..grant}).unwrap());
            let reported=step.progress().released_bytes;let expected=row["releasedBytes"].as_u64().unwrap() as usize;
            assert_eq!(allocated,0);assert!(step.progress().fits(RetainedCloneGrant {maximum_release_bytes:caller,..grant}));
            if expected==0 { assert_eq!(owner.as_ref() as *const dyn ErasedSnapshotRetirement as *const (),original);assert_eq!(owner.next_release_byte_demand().unwrap(),N); }
            for _ in 0..64 {if owner.terminal_is_empty(){break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
            assert!(owner.terminal_is_empty());let shell=std::mem::size_of_val(owner.as_ref());assert!(grant.maximum_release_bytes>=shell);
            let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,shell));
            eprintln!("[DEBUG] actual {name} terminal child extent={N} demand={demand} caller={caller} actual={actual} reported={reported}; original pointer retained on denial, outer frame={shell} separately funded");
            if demand!=N||actual!=expected||reported!=actual {failures.push((name.into(),demand,actual,reported));}
        }
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();let admission=fixture["admissionBytes"].as_u64().unwrap() as usize;let mut failures=Vec::new();
    for row in fixture["cases"].as_array().unwrap(){match row["physicalBytes"].as_u64().unwrap(){1=>check::<1>(row,admission,&mut failures),16384=>check::<16384>(row,admission,&mut failures),65536=>check::<65536>(row,admission,&mut failures),262144=>check::<262144>(row,admission,&mut failures),_=>panic!("explicit neutral cursor extents")}}
    assert!(failures.is_empty(),"actual original owners require same-turn independent release funding: {failures:?}");
}

/// 🧺️ The store preserves its terminal disposer allocation under an insufficient physical grant.
#[test]
fn displaced_store_owner_preserves_terminal_physical_grants() {
    struct TerminalOwner<const N: usize>([u8; N]);
    impl<const N: usize> ErasedSnapshotRetirement for TerminalOwner<N> {
        fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    }
    fn check<const N: usize>(row: &serde_json::Value) {
        let grant = physical_test_close_grant();
        let ((owner, birth), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| admit_artifact_retirement([0u8; N], grant, TerminalOwner::<N>).unwrap_or_else(|_| panic!("declared independent test policy admits original terminal frame")));
        assert!(birth.fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (birth.retained_capacity_bytes, 0));
        let mut queue = snapshot_read::ArtifactStoreDisplacedRetirements::new(); queue.push_reserved(owner);
        let original = queue.owners.front().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
        assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap().release_bytes, N);
        let (zero, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
        assert_eq!((allocated, released), (0, 0)); assert_eq!(zero.progress(), RetainedCloneProgress::default());
        let caller = row["callerBytes"].as_u64().unwrap() as usize; let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.close_step(RetainedCloneGrant { maximum_release_bytes: caller, ..grant }).unwrap());
        assert_eq!((allocated, released), (0, expected)); assert_eq!(step.progress(), RetainedCloneProgress { copied_items: usize::from(expected != 0), released_bytes: expected, ..Default::default() });
        if expected == 0 {
            assert_eq!(queue.owners.front().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), original);
            assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap().release_bytes, N);
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.close_step(grant).unwrap());
            assert_eq!((allocated, released), (0, N)); assert!(step.progress().fits(grant)); assert_eq!(step.progress().released_bytes, N);
        }
        assert!(queue.terminal_is_empty()); assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap(), Default::default());
        let backing = queue.backing_release_byte_demand().unwrap();
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.release_empty_backing_step(grant).unwrap());
        assert!(matches!(step, RetainedCloneStep::Complete(_))); assert!(step.progress().fits(grant)); assert_eq!((allocated, released), (0, backing)); assert_eq!(step.progress().released_bytes, backing);
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() { match row["physicalBytes"].as_u64().unwrap() { 1 => check::<1>(row), 16384 => check::<16384>(row), 65536 => check::<65536>(row), 262144 => check::<262144>(row), _ => panic!("declared neutral displaced-owner extents") } }
    let grant = physical_test_close_grant(); let mut queue = snapshot_read::ArtifactStoreDisplacedRetirements::new(); let reservation = queue.reserve_owner_slots(1).unwrap();
    assert_eq!(queue.close_step(grant).unwrap_err().kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap_err().kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    queue.release_owner_slots(reservation).unwrap(); assert!(queue.terminal_is_empty()); assert!(queue.release_empty_backing_step(grant).unwrap().progress().fits(grant));
}

/// 🧺️ The store preserves its terminal disposer allocation under an insufficient physical grant.
#[semio_framework_async_macros::async_test]
async fn installed_store_disposer_preserves_terminal_physical_grant() {
    struct TerminalDisposer<const N: usize>([u8; N]);
    impl<const N: usize> ArtifactStoreOwnedDisposer<DemoSnapshot, DemoMutation> for TerminalDisposer<N> {
        fn close_step(&mut self, _: &mut ArtifactStoreCloseView<'_, DemoSnapshot, DemoMutation>, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self, _: &semio_framework_os_store_engine::lifted_store::ArtifactStore<DemoSnapshot, DemoMutation>) -> bool { true }
        fn demands(&self, _: &semio_framework_os_store_engine::lifted_store::ArtifactStore<DemoSnapshot, DemoMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(Default::default()) }
        fn close_uninstalled_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn uninstalled_demands(&self, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(Default::default()) }
        fn uninstalled_terminal_is_empty(&self) -> bool { true }
    }
    impl<const N: usize> ErasedSnapshotRetirement for TerminalDisposer<N> {
        fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    }
    async fn check<const N: usize>(row: &serde_json::Value, admission: usize, failures: &mut Vec<(usize, usize, usize, usize)>) {
        let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "terminal-disposer", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.unwrap();
        store.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        close_test_store(&mut store);
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut cursor = ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new();
        cursor.phase = ArtifactStoreCursorDisposerPhase::Complete;
        *cursor.active = Some(Box::new(TerminalDisposer::<N>([0; N])));
        let nested_demand = cursor.demands(&store, 0).unwrap().release_bytes;
        let original = cursor.active.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
        let grant_for = |maximum_items, maximum_release_bytes| RetainedCloneGrant { maximum_items, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes, maximum_depth: 1 };
        let (step, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(&mut ArtifactStoreCloseView { store: &mut store }, grant_for(0, N)).unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!((requested, released), (0, 0));
        assert_eq!(cursor.active.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), original);
        let (nested_step, _, nested_actual) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(&mut ArtifactStoreCloseView { store: &mut store }, grant_for(1, caller)).unwrap());
        let nested_report = nested_step.progress().released_bytes;
        if expected == 0 { assert_eq!(cursor.active.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), original); }
        for _ in 0..8 { if cursor.active.is_none() { break; } cursor.close_step(&mut ArtifactStoreCloseView { store: &mut store }, grant_for(1, admission)).unwrap(); }
        assert!(cursor.active.is_none());
        if nested_demand != N || nested_actual != expected || nested_report != nested_actual { failures.push((N, nested_demand, nested_actual, nested_report)); }
        *store.owners.owned_disposer = Some(Box::new(TerminalDisposer::<N>([0; N])));
        store.owners.owned_disposer_terminal = false;
        let demand = store.close_owned_demands(0).unwrap().release_bytes;
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (step, _, actual) = semio_framework_trace_memory_testing::observe_backing(|| store.close_owned_step(grant_for(1, caller)).unwrap());
        let reported = step.progress().released_bytes;
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        for _ in 0..8 { if store.close_owned_terminal_is_empty() { break; } store.close_owned_step(grant_for(1, admission)).unwrap(); }
        assert!(store.close_owned_terminal_is_empty());
        if demand != N || actual != expected || reported != actual { failures.push((N, demand, actual, reported)); }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() { match row["physicalBytes"].as_u64().unwrap() { 1 => check::<1>(row, admission, &mut failures).await, 16384 => check::<16384>(row, admission, &mut failures).await, 65536 => check::<65536>(row, admission, &mut failures).await, 262144 => check::<262144>(row, admission, &mut failures).await, _ => panic!("explicit neutral disposer extents") } }
    assert!(failures.is_empty(), "the terminal store cursor requires exact physical funding: {failures:?}");
}

#[semio_framework_async_macros::async_test]
async fn member_owned_publication_birth_refuses_without_allocating_or_transferring_source() {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/📦️owned/🌱️birth/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["rows"].as_array().unwrap() {
        let mut member = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "birth-refusal", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        member.log.envelope.active_alternative_id = row["line"].as_str().map(str::to_owned);
        let mut factory = DemoOneItemPreparationFactory::admissible();
        factory.stamped_mutation_id = row["stampedId"].as_str().map(|id| MutationId(id.into()));
        member.install_document_store_owners_exact(complete_test_catalog(demo_base_store_owners().with_one_item_preparation(Arc::new(factory),physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let values = vec![DemoMutation::SetN(SetN { n: 8 }), DemoMutation::SetN(SetN { n: 9 })];
        let pointer = values.as_ptr();
        let (mutations, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("funded original source"));
        let mut request = MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(719), expected_generation: member.generation_now(), expected_revision: member.content_revision_now(), actor: row["actor"].as_str().unwrap().into(), group_id: row["group"].as_str().map(str::to_owned), transaction: row["transaction"].as_object().map(|value| protocol::TransactionRef { id: value["id"].as_str().unwrap().into(), tool: value["tool"].as_str().unwrap().into() }), mutations };
        let metadata = (request.actor.as_ptr(), request.group_id.as_ref().map(|value| value.as_ptr()), request.transaction.as_ref().map(|value| (value.id.as_ptr(), value.tool.as_ptr())));
        let (birth, query_allocated, query_released) = semio_framework_trace_memory_testing::observe_backing(|| member.owned_publication_birth_bytes(&request));
        let birth = birth.unwrap();
        assert_eq!((query_allocated, query_released), (0, 0));
        assert!(birth > row["metadataBytes"].as_u64().unwrap() as usize);
        for funding in [ArtifactStoreOneItemGrant { maximum_items: 0, maximum_capacity_bytes: birth, ..one_item_test_policy() }, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 0, ..one_item_test_policy() }, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_capacity_bytes: birth - 1, ..one_item_test_policy() }] {
            let (candidate, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| member.begin_one_item_owned_publication(&mut request, funding));
            assert!(candidate.unwrap().is_none());
            assert_eq!((allocated, released), (0, 0));
            assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
            assert_eq!((request.actor.as_ptr(), request.group_id.as_ref().map(|value| value.as_ptr()), request.transaction.as_ref().map(|value| (value.id.as_ptr(), value.tool.as_ptr()))), metadata);
            assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().len(), 2);
            assert_eq!(member.snapshot_ref().n, Some(0));
        }
        let exact = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_capacity_bytes: birth, ..one_item_test_policy() };
        let (publication, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| member.begin_one_item_owned_publication(&mut request, exact));
        let mut publication = publication.unwrap().expect("exact birth funded");
        assert_eq!((allocated, released), (birth, 0));
        assert!(request.mutations.mutations::<DemoMutation>().is_none());
        publication.begin_close();
        for _ in 0..65_536 { if publication.terminal_is_empty() { break; } let step = publication.close_step(physical_test_close_grant()).unwrap(); assert!(step.progress().fits(physical_test_close_grant())); }
        assert!(publication.terminal_is_empty());
        for _ in 0..1024 { if request.mutations.terminal_is_empty() { break; } request.mutations.close_granted(grant).unwrap(); }
        assert!(request.mutations.terminal_is_empty());
        for _ in 0..65_536 { if matches!(SpaceMember::close_owned_step(&mut member, physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
        assert!(SpaceMember::close_owned_terminal_is_empty(&member));
    }
}


#[test]
fn current_box_retirement_preserves_denied_owner_and_exact_physical_shell() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    let policy = physical_test_close_grant();
    let original = String::with_capacity(4096);
    let pointer = original.as_ptr();
    let birth = std::mem::size_of::<ArtifactStoreStringRetirement>();
    let denied = RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, maximum_capacity_bytes: birth - 1, ..Default::default() };
    let (refusal, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| admit_artifact_retirement(original, denied, ArtifactStoreStringRetirement::new));
    let (error, original) = refusal.err().unwrap();
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert_eq!(original.as_ptr(), pointer);
    assert_eq!((requested, released), (0, 0));
    let (admission, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| admit_artifact_retirement(original, policy, ArtifactStoreStringRetirement::new));
    let (owner, receipt) = admission.unwrap_or_else(|(error, _)| panic!("{error}"));
    assert_eq!(requested, birth);
    assert_eq!(released, 0);
    assert_eq!(receipt.retained_capacity_bytes, birth);
    let address = owner.as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
    let mut slot = Some(owner);
    let current_depth = artifact_retirement_box_demands(slot.as_ref().unwrap(), 1).unwrap().depth;
    for depth in [0, 1, current_depth - 1] {
        let (result, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut slot, RetainedCloneGrant { maximum_items: 1, maximum_depth: depth, maximum_release_bytes: 4096, ..Default::default() }));
        assert_eq!(result.unwrap_err().kind, semio_framework_value::ValueRefusalKind::DepthLimit);
        assert_eq!((requested, released), (0, 0));
        assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), address);
    }
    let (step, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut slot, RetainedCloneGrant::default()).unwrap());
    assert_eq!(step, RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    assert_eq!((requested, released), (0, 0));
    let mut freed = 0;
    let mut born = 4096 + birth;
    for _ in 0..128 {
        let Some(owner) = slot.as_ref() else { break; };
        let grant = policy;
        let terminal_before = owner.terminal_is_empty();
        let shell = std::mem::size_of_val(owner.as_ref());
        let (step, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut slot, grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().retained_capacity_bytes, requested);
        assert_eq!(step.progress().released_bytes, released);
        if terminal_before { assert_eq!(released, shell); assert!(slot.is_none()); }
        else { assert!(slot.is_some()); }
        freed += released;
        born += requested;
    }
    assert!(slot.is_none());
    assert_eq!(freed, born);
}

#[test]
fn artifact_store_group_visibility_retires_original_noncopy_atomic_with_exact_frame_receipt() {
    use semio_framework_value::retirement::{RetireOwned, RetirementStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["visibilityRetirement"].as_array().unwrap() {
        let mut publisher = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
        if row["committed"].as_bool().unwrap() { assert!(publisher.commit()); }
        let view = publisher.view();
        drop(publisher);
        let value = match Arc::try_unwrap(view) { Ok(value) => value, Err(_) => panic!("original publisher and view are uniquely closed") };
        assert_eq!(serde_json::Value::Bool(value.committed()), row["committed"]);
        assert_eq!(serde_json::Value::Bool(value.pending()), row["pending"]);
        let extent = value.retirement_birth_bytes().unwrap();
        let (mut cursor, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| value.retirement());
        assert_eq!((allocated, released), (extent, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: cursor.next_depth_demand().unwrap() };
        for denial in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }] {
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(denial));
            assert!(matches!(step, RetirementStep::BudgetExhausted));
            assert_eq!((allocated, released), (0, 0));
            assert!(!cursor.terminal_is_empty());
        }
        let mut copied = 0;
        for _ in 0..8 {
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(grant));
            assert_eq!((allocated, released), (0, 0));
            match step { RetirementStep::ProcessedBytes(bytes) => { assert!(bytes <= 1); copied += bytes; }, RetirementStep::Complete => break, _ => panic!("atomic-only owner has no hidden heap work") }
        }
        assert_eq!(copied, std::mem::size_of::<semio_framework_os_vcs::ArtifactGroupVisibility>());
        assert!(cursor.terminal_is_empty());
        assert_eq!(cursor.terminal_release_bytes(), Some(extent));
        let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(cursor));
        assert_eq!((allocated, released), (0, extent));
    }
}

#[test]
fn current_typed_option_birth_retains_exact_original_and_paid_shell() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let original = String::with_capacity(4096);
    let pointer = original.as_ptr();
    let mut pending = Some(original);
    let mut active = None;
    let demand = artifact_retirement_owned_birth_demands(&pending).unwrap();
    let paid = physical_test_close_grant();
    for denied in [
        RetainedCloneGrant { maximum_items: 0, ..paid },
        RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..paid },
        RetainedCloneGrant { maximum_depth: demand.depth - 1, ..paid },
    ] {
        let (result, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_admit_owned(&mut pending, &mut active, denied));
        if let Ok(step) = result { assert_eq!(step.progress(), Default::default()); }
        assert_eq!((born, freed), (0, 0));
        assert_eq!(pending.as_ref().unwrap().as_ptr(), pointer);
        assert_eq!(pending.as_ref().unwrap().capacity(), 4096);
        assert!(active.is_none());
    }
    let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_admit_owned(&mut pending, &mut active, paid).unwrap());
    assert_eq!(step.progress().retained_capacity_bytes, born);
    assert_eq!(born, demand.capacity_bytes);
    assert_eq!(freed, 0);
    assert!(pending.is_none());
    let mut total_freed = 0;
    let mut total_born = born;
    for _ in 0..128 {
        let Some(owner) = active.as_ref() else { break; };
        let grant = physical_test_close_grant();
        let (step, turn_born, turn_freed) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut active, grant).unwrap());
        assert_eq!(step.progress().retained_capacity_bytes, turn_born);
        assert_eq!(step.progress().released_bytes, turn_freed);
        assert!(matches!(step, RetainedCloneStep::Progress(_) | RetainedCloneStep::Complete(_)));
        total_freed += turn_freed;
        total_born += turn_born;
    }
    assert!(active.is_none());
    assert_eq!(total_freed, 4096 + total_born);
}

#[test]
fn current_catalog_sources_admit_original_allocations_then_fund_each_ticket() {
    type Owners = DocumentStoreOwners<DemoSnapshot, DemoMutation>;
    type Disposer = ArtifactStoreCursorDisposer<DemoSnapshot, DemoMutation>;
    let capacity = Owners::source_birth_bytes::<DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer>().unwrap();
    let mut originals = Some((DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer::new()));
    let paid = physical_test_close_grant();
    for denied in [RetainedCloneGrant { maximum_items: 0, ..paid }, RetainedCloneGrant { maximum_depth: 0, ..paid }, RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..paid }] {
        let (snapshots, initial, mutations, disposer) = originals.take().unwrap();
        let (refused, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_sources(snapshots, initial, mutations, disposer, denied));
        assert_eq!((allocated, released), (0, 0));
        let Err((_, snapshots, initial, mutations, disposer)) = refused else { panic!("underfunded catalog changed its original source custody"); };
        assert!(!disposer.started);
        assert!(disposer.active.is_none());
        originals = Some((snapshots, initial, mutations, disposer));
    }
    let (snapshots, initial, mutations, disposer) = originals.take().unwrap();
    let (admitted, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_sources(snapshots, initial, mutations, disposer, paid));
    let (mut owners, progress) = admitted.ok().expect("funded original catalog source ingress");
    assert_eq!(allocated, capacity);
    assert_eq!(released, 0);
    assert_eq!(progress.retained_capacity_bytes, allocated);
    assert_eq!(progress.copied_items, 1);
    assert!(!owners.constructor_is_complete());
    let mut constructor_turns = 0;
    for _ in 0..128 {
        if owners.constructor_is_complete() { break; }
        let grant = physical_test_close_grant();
        let (admitted, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.admit_constructor(grant).unwrap());
        assert_eq!(admitted.copied_items, 1);
        assert_eq!(admitted.copied_bytes, 0);
        assert!(admitted.fits(grant));
        assert_eq!(admitted.retained_capacity_bytes, allocated);
        assert_eq!(released, 0);
        constructor_turns += 1;
    }
    assert!(constructor_turns >= 6);
    assert!(owners.constructor_is_complete());
    for _ in 0..128 {
        if owners.uninstalled_owners_terminal_is_empty() { break; }
        let grant = physical_test_close_grant();
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.close_uninstalled_owners_step(grant).unwrap());
        assert_eq!(step.progress().retained_capacity_bytes, allocated);
        assert_eq!(step.progress().released_bytes, released);
    }
    assert!(owners.uninstalled_owners_terminal_is_empty());
}

/// 🏗️ Verifies actual source construction is absent on denial and every later ticket/free is physically paid.
#[test]
fn current_member_source_constructor_is_not_invoked_under_denied_axes() {
    type Owners = DocumentStoreOwners<DemoSnapshot, DemoMutation>;
    type Disposer = ArtifactStoreCursorDisposer<DemoSnapshot, DemoMutation>;
    let capacity = Owners::source_birth_bytes::<DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer>().unwrap();
    let paid = physical_test_close_grant();
    let calls = std::cell::Cell::new(0usize);
    let construct = || {
        calls.set(calls.get() + 1);
        (DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer::new())
    };
    for denied in [
        RetainedCloneGrant { maximum_items: 0, ..paid },
        RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..paid },
        RetainedCloneGrant { maximum_depth: 0, ..paid },
    ] {
        let (refused, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_source_constructor(denied, construct));
        let refusal = refused.err().expect("underfunded source constructor is refused");
        assert!(refusal.owners.is_none());
        assert_eq!(refusal.progress, RetainedCloneProgress::default());
        assert_eq!(calls.get(), 0);
        assert_eq!(allocated, 0);
        assert_eq!(released, 0);
    }
    let (admitted, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_source_constructor(paid, construct));
    let (mut owners, ingress) = admitted.ok().expect("funded source constructor");
    assert_eq!(calls.get(), 1);
    assert_eq!(allocated, capacity);
    assert_eq!(released, 0);
    assert_eq!(ingress.retained_capacity_bytes, allocated);
    assert_eq!(ingress.copied_items, 1);
    assert!(!owners.constructor_is_complete());
    let mut total_born = allocated;
    let mut total_released = 0usize;
    for _ in 0..128 {
        if owners.uninstalled_owners_terminal_is_empty() { break; }
        let grant = physical_test_close_grant();
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.close_uninstalled_owners_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().retained_capacity_bytes, allocated);
        assert_eq!(step.progress().released_bytes, released);
        total_born += allocated;
        total_released += released;
    }
    assert!(owners.uninstalled_owners_terminal_is_empty());
    assert_eq!(total_born, total_released);
}

#[test]
fn ephemeral_member_original_preparation_frame_requires_exact_granted_release() {
    struct TerminalPreparation<const N: usize>([u8; N]);
    impl<const N: usize> ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for TerminalPreparation<N> {
        fn advance(&mut self, _: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> { Ok(ArtifactStoreOneItemPreparationStep::Blocked) }
        fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint { Default::default() }
        fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<DemoSnapshot>> { None }
        fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<DemoSnapshot>> { None }
        fn cancel(&mut self) {}
        fn begin_close(&mut self) {}
        fn close_step(&mut self, _: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn terminal_is_empty(&self) -> bool { true }
    }
    fn check<const N: usize>(row: &serde_json::Value) {
        let mut publication = ArtifactEphemeralOneItemPublication::<DemoSnapshot, DemoMutation>::closing_with_preparation(semio_framework_job::OperationId(1), Box::new(TerminalPreparation::<N>([0; N])));
        let pointer = publication.original_preparation().unwrap() as *const dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> as *const ();
        let authority = |items,release,depth| ArtifactStoreOneItemGrant { maximum_items: items,maximum_copy_bytes: 0,maximum_capacity_bytes: 0,maximum_release_bytes: release,maximum_depth: depth };
        assert_eq!(publication.retirement_demands(0).unwrap(), semio_framework_value::RetirementDemand { release_bytes: N,depth: 1,..Default::default() });
        for denied in [authority(0,N,1),authority(1,N-1,1),authority(1,N,0)] {
            let (step,allocated,released) = semio_framework_trace_memory_testing::observe_backing(||publication.close_step(denied));
            assert_eq!((allocated,released),(0,0));
            if denied.maximum_depth==0 { assert_eq!(step.unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit); }
            else { assert_eq!(step.unwrap().progress(),RetainedCloneProgress::default()); }
            assert_eq!(publication.original_preparation().unwrap() as *const dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> as *const (),pointer);
        }
        let caller=row["callerBytes"].as_u64().unwrap() as usize;let expected=row["releasedBytes"].as_u64().unwrap() as usize;
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(authority(1,caller,1)).unwrap());
        assert_eq!((allocated,released),(0,expected));assert_eq!(step.progress(),RetainedCloneProgress { copied_items: usize::from(expected!=0),released_bytes: expected,..Default::default() });
        if expected==0 {
            assert_eq!(publication.original_preparation().unwrap() as *const dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> as *const (),pointer);
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(authority(1,N,1)).unwrap());assert_eq!((allocated,released),(0,N));assert_eq!(step.progress().released_bytes,N);
        }
        assert!(publication.original_preparation().is_none());assert!(!publication.terminal_is_empty());
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(authority(1,0,1)).unwrap());assert_eq!((allocated,released),(0,0));assert_eq!(step,RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,..Default::default() }));assert!(publication.terminal_is_empty());
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){match row["physicalBytes"].as_u64().unwrap(){1=>check::<1>(row),16384=>check::<16384>(row),65536=>check::<65536>(row),262144=>check::<262144>(row),_=>panic!("authored neutral original frame extent")}}
}

#[test]
fn live_read_factory_admission_preserves_exact_registry_generation_on_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["grant"];
    let grant = RetainedCloneGrant { maximum_items: policy["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes: policy["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes: policy["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes: policy["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth: policy["maximumDepth"].as_u64().unwrap() as usize };
    let mut registry = Some(semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new());
    let original = Arc::new(17i32);
    let factory: Arc<dyn ArtifactOwnedValueRetirementFactory<i32>> = Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<i32>::default());
    let lease = registry.as_ref().unwrap().try_issue(Arc::clone(&original)).unwrap();
    let index = lease.index;let generation = lease.generation;
    let mut read = ErasedSnapshotRead::new(Arc::clone(&original),lease);
    let pointer = read.get::<i32>().unwrap() as *const i32;
    let birth = ReturnedSnapshotReadRetirement::<i32>::constructor_capacity_bytes();
    assert!(birth<=grant.maximum_capacity_bytes);
    for denied in [RetainedCloneGrant { maximum_items: 0,..grant },RetainedCloneGrant { maximum_capacity_bytes: birth-1,..grant },RetainedCloneGrant { maximum_depth: 0,..grant }] {
        let (result,allocated,released) = semio_framework_trace_memory_testing::observe_backing(||read.admit_retirement::<i32>(registry.as_ref().unwrap(),&factory,denied));
        assert!(result.is_err());assert_eq!((allocated,released),(0,0));
        assert_eq!(read.get::<i32>().unwrap() as *const i32,pointer);
        assert!(registry.as_ref().unwrap().contains(index,generation));
        assert_eq!(registry.as_ref().unwrap().occupied_count(),1);
    }
    let (result,allocated,released) = semio_framework_trace_memory_testing::observe_backing(||read.admit_retirement::<i32>(registry.as_ref().unwrap(),&factory,grant));
    let(owner,progress)=result.unwrap();assert_eq!((allocated,released),(birth,0));assert_eq!(progress,RetainedCloneProgress { copied_items: 1,retained_capacity_bytes: birth,..Default::default() });
    assert!(read.terminal_is_empty());assert!(!registry.as_ref().unwrap().contains(index,generation));assert_eq!(registry.as_ref().unwrap().occupied_count(),0);assert_eq!(Arc::as_ptr(&original),pointer);
    let mut owner=Some(owner);
    for _ in 0..32 {if owner.is_none(){break;}let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||artifact_retirement_box_close_step(&mut owner,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
    assert!(owner.is_none());
    let cleanup = RetainedCloneGrant { maximum_release_bytes: fixture["census"]["maximumCloseBytes"].as_u64().unwrap() as usize,..grant };
    let mut registry=registry.take().map(SnapshotReadRegistryAliasRetirement::new);
    for _ in 0..32 {if registry.is_none(){break;}snapshot_registry_alias_close_step(&mut registry,cleanup).unwrap();}
    assert!(registry.is_none());
}

#[test]
fn current_shared_grant_coverage_preserves_every_denied_axis() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["grantCoverage"]; let d = &row["demand"];
    let demand = semio_framework_value::RetirementDemand { copy_bytes: d["copyBytes"].as_u64().unwrap() as usize, capacity_bytes: d["capacityBytes"].as_u64().unwrap() as usize, release_bytes: d["releaseBytes"].as_u64().unwrap() as usize, depth: d["depth"].as_u64().unwrap() as usize };
    let supplied = physical_test_close_grant(); assert!(artifact_retirement_grant_covers(supplied,demand));
    for row in row["denials"].as_array().unwrap() {
        let mut denied = supplied; let value = row["value"].as_u64().unwrap() as usize;
        match row["axis"].as_str().unwrap() { "maximumItems" => denied.maximum_items=value, "maximumCopyBytes" => denied.maximum_copy_bytes=value, "maximumCapacityBytes" => denied.maximum_capacity_bytes=value, "maximumReleaseBytes" => denied.maximum_release_bytes=value, "maximumDepth" => denied.maximum_depth=value, _ => panic!("unknown plain physical axis") }
        let (covered,born,freed)=semio_framework_trace_memory_testing::observe_backing(||artifact_retirement_grant_covers(denied,demand));assert!(!covered);assert_eq!((born,freed),(0,0));
    }
}

#[test]
fn field_close_failure_preserves_real_receipts_through_the_original_decode_job() {
    use semio_framework_job::InteractiveJob;
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    struct FailedField { owner: ControlledRetirement<Vec<u8>>, fail_once: bool }
    impl ArtifactEnvelopeFieldDecoder<(), ()> for FailedField {
        fn accept_field_token(&mut self, _:u16, _:OwnedSchemaToken, _:bool, _: &OwnedSchemaRecordCursor, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic> { Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) }
        fn finish_record(&mut self, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic> { Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete) }
        fn maximum_close_byte_demand(&self)->usize { 65_536 }
        fn maximum_retained_close_bytes(&self)->usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES }
        fn next_close_copy_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_copy_byte_demand().unwrap()) }
        fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_capacity_byte_demand(copy).unwrap()) }
        fn next_close_release_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_release_byte_demand().unwrap()) }
        fn next_close_depth_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic> { Ok(self.owner.next_depth_demand().unwrap()) }
        fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeDiagnostic> {
            let step=self.owner.step(grant).unwrap();
            if std::mem::take(&mut self.fail_once) { return Err(OwnedSchemaDecodeDiagnostic { code:"field.close-after-original-birth",offset:0,line:0,column:0,path:OwnedSchemaPath::ROOT,retained_progress:step.progress(), refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated }); }
            Ok(step)
        }
        fn terminal_is_empty(&self)->bool { self.owner.terminal_is_empty() }
    }
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🚪️io/🚫️refusal/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(corpus["nativeLaw"],"field_close_failure_preserves_real_receipts_through_the_original_decode_job");
    let mut caller=OriginalEnvelopeCaller::admit_cleanup("failed-field");
    let mut cx=caller.context();
    let registry=ArtifactEnvelopeFieldDecoderRegistry::new();
    let field=FailedField { owner:ControlledRetirement::new(vec![7;1024]).unwrap(),fail_once:true };
    let mut job=ArtifactEnvelopeDecodeAuthority::<(),()>::try_new(artifact_envelope_decode_test_cursor(&[b"{}"]),&registry,Box::new(field)).unwrap_or_else(|_|panic!("original field authority admission"));
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65_536,maximum_release_bytes:65_536,maximum_depth:64};
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cx.with_retained_loan(grant,|cx|job.close_step(cx)));
    let progress=cx.retained_progress();
    let failure=step.expect_err("original failed field remains typed");assert_eq!(failure.primary.retained_progress(),progress);assert!(failure.receipt_refusal.is_none());
    assert!(allocated>0);assert_eq!(progress.retained_capacity_bytes,allocated);assert_eq!(progress.released_bytes,released);assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));assert!(!job.terminal_is_empty());
    let mut detached=None;
    for _ in 0..4096 {drive_test_envelope_field_return(&registry,&mut detached);if job.terminal_is_empty(){break}let before=cx.retained_progress();let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||cx.with_retained_loan(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(cx.retained_progress(),before.checked_add(step.progress()).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
    assert!(job.terminal_is_empty());assert!(registry.terminal_is_empty());
    drop(cx);
    caller.retire(grant);
}

#[test]
fn normal_decode_release_records_original_wallet_even_after_child_fault(){
 use semio_framework_job::{InteractiveJob,StepContextOwner,StepBudget,OperationId,Generation};
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 struct WalletField{first:bool,old:Vec<u8>,owned:Option<ControlledRetirement<Vec<u8>>>}
 impl WalletField{fn demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.first{return Ok(semio_framework_value::RetirementDemand{copy_bytes:23,capacity_bytes:64,release_bytes:32,depth:3})}let owner=self.owned.as_ref().unwrap();Ok(semio_framework_value::RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}}
 impl ArtifactEnvelopeFieldDecoder<(),()>for WalletField{
  fn accept_field_token(&mut self,_:u16,_:OwnedSchemaToken,_:bool,_:&OwnedSchemaRecordCursor,_:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic>{Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)}
  fn finish_record(&mut self,_:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactEnvelopeFieldDecodeStep,OwnedSchemaDecodeDiagnostic>{Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)}
  fn maximum_close_byte_demand(&self)->usize{65536}
  fn maximum_retained_close_bytes(&self)->usize{ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES}
  fn next_close_copy_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(0).map(|d|d.copy_bytes).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.copy",OwnedSchemaPath::ROOT).with_native(e))}
  fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(copy).map(|d|d.capacity_bytes).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.capacity",OwnedSchemaPath::ROOT).with_native(e))}
  fn next_close_release_byte_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(0).map(|d|d.release_bytes).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.release",OwnedSchemaPath::ROOT).with_native(e))}
  fn next_close_depth_demand(&self)->Result<usize,OwnedSchemaDecodeDiagnostic>{self.demand(0).map(|d|d.depth).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.depth",OwnedSchemaPath::ROOT).with_native(e))}
  fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeDiagnostic>{
   if !self.first{return self.owned.as_mut().unwrap().step(grant).map_err(|e|OwnedSchemaDecodeDiagnostic::before("wallet.cleanup",OwnedSchemaPath::ROOT).with_native(e))}
   if !artifact_retirement_grant_covers(grant,self.demand(grant.maximum_copy_bytes).unwrap()){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let mut original=Vec::new();original.try_reserve_exact(64).unwrap();original.extend_from_slice(b"original field payload!");let freed=self.old.capacity();drop(std::mem::take(&mut self.old));let born=original.capacity();self.owned=Some(ControlledRetirement::new(original).unwrap());self.first=false;
   Err(OwnedSchemaDecodeDiagnostic::before("field.wallet-after-real-turn",OwnedSchemaPath::ROOT).with_progress(RetainedCloneProgress{copied_items:1,copied_bytes:23,retained_capacity_bytes:born,released_bytes:freed}))
  }
  fn terminal_is_empty(&self)->bool{!self.first&&self.owned.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)}
 }
 let mut cleanup_caller=OriginalEnvelopeCaller::admit_cleanup("wallet-cleanup");let mut cleanup_cx=cleanup_caller.context();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🚪️io/🚫️refusal/⏱️step/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let supplied:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();let registry=ArtifactEnvelopeFieldDecoderRegistry::new();let mut job=ArtifactEnvelopeDecodeAuthority::<(),()>::try_new(artifact_envelope_decode_test_cursor(&[b"{}"]),&registry,Box::new(WalletField{first:true,old:vec![7;32],owned:None})).unwrap_or_else(|_|panic!("original wallet field admission"));job.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
 let((mut owner,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||StepContextOwner::new(OperationId(98146),Generation(17),supplied).unwrap());assert!(receipt.fits(supplied));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));let((cancel,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||semio_framework_job::CancelToken::admit_root(supplied).unwrap().unwrap());assert!(receipt.fits(supplied));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));
 for axis in fixture["deniedAxes"].as_array().unwrap(){let mut grant=supplied;match axis.as_str().unwrap(){"maximumItems"=>grant.maximum_items=0,"maximumCopyBytes"=>grant.maximum_copy_bytes=0,"maximumCapacityBytes"=>grant.maximum_capacity_bytes=0,"maximumReleaseBytes"=>grant.maximum_release_bytes=0,"maximumDepth"=>grant.maximum_depth=0,_=>panic!("closed original grant axis")};let mut receipt=RetainedCloneProgress::default();let mut sequence=0;let mut cx=owner.context(StepBudget::new(7,1500,grant),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();cx.set_stage("wallet.heap-warmup");let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||job.step(&mut cx));assert!(matches!(step,Ok(None)),"denied axis {axis} answered {:?}",step.as_ref().map(|value|value.is_some()).map_err(|error|(&error.primary,&error.receipt_refusal)));assert_eq!((born,freed),(0,0),"denied axis {axis}");assert_eq!(cx.retained_progress(),Default::default());assert_eq!(cx.fuel_remaining(),7);assert_eq!(job.released_field_bytes(),0);assert!(matches!(job.state,ArtifactEnvelopeDecodeState::ReleaseCancelled));}
 let mut receipt=RetainedCloneProgress::default();let mut sequence=0;let mut cx=owner.context(StepBudget::new(7,1500,supplied),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||job.step(&mut cx));let failure=step.expect_err("funded original field failure remains complete");assert_eq!((born,freed),(64,32));let field_receipt:RetainedCloneProgress=serde_json::from_value(fixture["receipt"].clone()).unwrap();assert_eq!(failure.primary.retained_progress(),field_receipt);assert!(failure.receipt_refusal.is_none()&&failure.context_refusal.is_none());assert_eq!(cx.retained_progress(),field_receipt,"the failed field posts once and publishes no yield outcome");assert_eq!(cx.operation(),OperationId(98146));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);assert_eq!(cx.fuel_remaining(),6);assert_eq!(job.released_field_bytes(),32);let ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic)=job.state else{panic!("real field fault remains attached")};assert_eq!(diagnostic.code,"field.wallet-after-real-turn");assert_eq!(diagnostic.retained_progress,field_receipt);drop(cx);
 let mut detached=None;for _ in 0..4096{drive_test_envelope_field_return(&registry,&mut detached);if job.terminal_is_empty(){break}let before=cleanup_cx.retained_progress();let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||cleanup_cx.with_retained_loan(supplied,|cx|job.close_step(cx)).unwrap());assert_eq!(cleanup_cx.retained_progress(),before.checked_add(step.progress()).unwrap());assert!(step.progress().fits(supplied));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(job.terminal_is_empty());assert!(registry.terminal_is_empty());while !owner.terminal_is_empty(){let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));}retire_original_envelope_cancel(cancel,supplied);drop(cleanup_cx);cleanup_caller.retire(supplied);
}

#[test]
fn decode_fault_descriptor_borrows_original_payload_until_paid_nested_close() {
    use semio_framework_job::{InteractiveJob, JobOutcomeKind, JobOutcomeView, StepContextOwner, StepBudget, OperationId, Generation};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️io/🚫️refusal/📬️outcome/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let wallet: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️io/🚫️refusal/⏱️step/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let original_grant: RetainedCloneGrant = serde_json::from_value(wallet["grant"].clone()).unwrap();
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[b"{}"]), &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    ).unwrap_or_else(|_| panic!("original fault decoder admission"));
    let diagnostic = ArtifactEnvelopeDecodeAuthority::<(), ()>::diagnostic("field.wallet-after-real-turn");
    assert_eq!(diagnostic.code, law["diagnostic"].as_str().unwrap());
    job.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
    let((mut owner,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||StepContextOwner::new(OperationId(98146),Generation(17),original_grant).unwrap());assert!(receipt.fits(original_grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));
    let mut sequence = 0;
    let mut detached = None;
    let mut descriptor = None;
    let((cancel,receipt),born,freed)=semio_framework_trace_memory_testing::observe_backing(||semio_framework_job::CancelToken::admit_root(original_grant).unwrap().unwrap());assert!(receipt.fits(original_grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,freed));
    for _ in 0..4096 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut receipt = RetainedCloneProgress::default();
        let mut cx = owner.context(StepBudget::new(7, 1500, original_grant), cancel.clone(), || Some(1000), &mut sequence, &mut receipt).unwrap();
        let result = job.step(&mut cx).unwrap().map(|result| result.into_descriptor());
        assert!(cx.retained_progress().fits(original_grant));
        let Some(mut result) = result else { continue };
        if result.kind() == JobOutcomeKind::Fault {
            descriptor = Some(result);
            break;
        }
        assert_eq!(result.kind(), JobOutcomeKind::Yield);
        assert!(matches!(result.acknowledge(original_grant), RetainedCloneStep::Complete(_)));
    }
    let mut descriptor = descriptor.expect("original fault is published after exact source owners close");
    assert!(registry.terminal_is_empty());
    assert!(!job.terminal_is_empty());
    let original = job.fault.published().unwrap() as *const _;
    for _ in 0..2 {
        let (view, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| job.borrow_outcome(&descriptor).unwrap());
        let JobOutcomeView::Fault { detail, admission } = view else { panic!("exact fault descriptor") };
        assert_eq!((born, freed), (law["borrowHeapBytes"].as_u64().unwrap() as usize, 0));
        assert_eq!(detail as *const _, original);
        assert!(std::ptr::eq(admission, descriptor.admission()));
        assert_eq!(detail.single_page().unwrap(), law["diagnostic"].as_str().unwrap().as_bytes());
    }
    assert!(matches!(descriptor.acknowledge(original_grant), RetainedCloneStep::Complete(_)));
    drop(descriptor);
    let child_depth = job.fault.retirement_demands().unwrap().depth;
    assert_eq!(job.next_close_depth_demand().unwrap(), child_depth + law["parentDepth"].as_u64().unwrap() as usize);
    for denied in [RetainedCloneGrant { maximum_items: 0, ..original_grant }, RetainedCloneGrant { maximum_depth: child_depth, ..original_grant }] {
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(7,1500,denied),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();
        let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| job.close_step(&mut cx).unwrap());assert_eq!(step.progress(),cx.retained_progress());
        assert_eq!((born, freed), (0, 0));
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!(job.fault.published().unwrap() as *const _, original);
        assert!(!job.terminal_is_empty());
    }
    for _ in 0..4096 {
        if job.terminal_is_empty() { break; }
        let mut receipt=RetainedCloneProgress::default();let mut cx=owner.context(StepBudget::new(7,1500,original_grant),cancel.clone(),||Some(1000),&mut sequence,&mut receipt).unwrap();
        let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| job.close_step(&mut cx).unwrap());assert_eq!(step.progress(),cx.retained_progress());
        assert!(step.progress().fits(original_grant));
        assert_eq!((step.progress().retained_capacity_bytes, step.progress().released_bytes), (born, freed));
    }
    assert!(job.terminal_is_empty());
    while !owner.terminal_is_empty() {
        let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(original_grant));assert!(step.progress().fits(original_grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));
    }
    retire_original_envelope_cancel(cancel,original_grant);
}
