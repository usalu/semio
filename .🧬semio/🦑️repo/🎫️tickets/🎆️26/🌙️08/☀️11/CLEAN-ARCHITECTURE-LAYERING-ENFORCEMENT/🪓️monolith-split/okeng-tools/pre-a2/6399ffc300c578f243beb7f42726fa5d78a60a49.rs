//! 📥️ The original snapshot-input turns of the member snapshot open operation: grants, system receipts, retained source and bounded close (test only; extracted from the operation module so it can follow the member-open unit tests).
use super::*;
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};
use semio_framework_trace::observe_heap_allocations_on_this_thread;

#[test]
fn member_actual_snapshot_input_turns_preserve_original_grants_and_system_receipts() {
    let plain: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let row = &plain["snapshotInput"];
    let policy = &row["policy"];
    let policy = RetainedCloneGrant { maximum_items: policy["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: policy["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: policy["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: policy["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: policy["maximumDepth"].as_u64().unwrap() as usize };
    let bytes = row["payloadBytes"].as_u64().unwrap() as usize;
    assert!(bytes < 128);
    for copy in row["copyGrants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut source = vec![7; bytes + 2]; source[0] = bytes as u8;
        let request = super::super::tests::request_for(&source);
        let mut decoder = ProducedMemberSnapshotOpen::<crate::DslValue,ArtifactPackSnapshotDecoder<crate::DslValue>>::begin(request).unwrap_or_else(|_| panic!("original paid snapshot input"));
        let source_identity = decoder.request.as_ref().unwrap().pages.as_ref().unwrap() as *const _;
        let cancel = root_cancel_token(); let mut sequence = 0;
        for (axis, blocked) in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: bytes - 1, ..policy }, RetainedCloneGrant { maximum_depth: 0, ..policy }].into_iter().enumerate() {
            let mut actual_retained_progress = RetainedCloneProgress::default();
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(512, 999, policy), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
            cx.set_stage("member-open.heap-warmup");
            let (step, heap) = observe_heap_allocations_on_this_thread(|| decoder.step(&mut cx, blocked));
            let variant = match &step { MemberSnapshotOpenStep::Pending(_) => "pending".to_string(), MemberSnapshotOpenStep::Ready => "ready".to_string(), MemberSnapshotOpenStep::Rejected(diagnostic) => format!("rejected {diagnostic:?}") };
            assert!(matches!(step, MemberSnapshotOpenStep::Pending(_)), "copy {copy} blocked axis {axis} answered {variant}");
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0), "copy {copy} blocked axis {axis} answered {step:?}");
            assert_eq!(decoder.input.capacity(), 0);
            assert_eq!(decoder.request.as_ref().unwrap().pages.as_ref().unwrap() as *const _, source_identity);
        }
        let mut born = 0;
        for turn in 0..4 {
            let mut actual_retained_progress = RetainedCloneProgress::default();
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(512, 999, policy), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
            let (step, heap) = observe_heap_allocations_on_this_thread(|| decoder.step(&mut cx, policy));
            let MemberSnapshotOpenStep::Pending(step) = step else { panic!("snapshot input remains pending before typed decode") };
            assert!(step.retained_progress.fits(policy)); assert_eq!(cx.retained_progress(),step.retained_progress);
            let before = (decoder.input.len(),decoder.input.capacity());
            let (second,heap_second)=observe_heap_allocations_on_this_thread(||decoder.step(&mut cx,policy));
            let MemberSnapshotOpenStep::Pending(second)=second else {panic!("same original input wallet remains denied after its single item")};
            assert_eq!(second.retained_progress,Default::default());assert_eq!((heap_second.requested_bytes,heap_second.released_bytes),(0,0));assert_eq!((decoder.input.len(),decoder.input.capacity()),before);
            assert_eq!(heap.requested_bytes, step.retained_progress.retained_capacity_bytes);
            assert_eq!(heap.released_bytes, step.retained_progress.released_bytes);
            born += heap.requested_bytes;
            if born != 0 { break; }
            assert!(turn < 3);
        }
        assert_eq!(born, bytes);
        let pointer = decoder.input.as_ptr(); let mut copied = 0;
        let copy_policy = RetainedCloneGrant { maximum_copy_bytes: copy, maximum_capacity_bytes: 0, ..policy };
        for turn in 0..bytes.max(1) {
            let before = decoder.input.len();
            let mut actual_retained_progress = RetainedCloneProgress::default();
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(512, 999, policy), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
            let (step, heap) = observe_heap_allocations_on_this_thread(|| decoder.step(&mut cx, copy_policy));
            let MemberSnapshotOpenStep::Pending(step) = step else { panic!("only actual input turns are selected") };
            let progress = step.retained_progress;
            assert!(progress.fits(copy_policy));assert_eq!(cx.retained_progress(),progress);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(progress.copied_bytes, (decoder.input.len() - before) * 2);
            assert_eq!(decoder.input.as_ptr(), pointer);
            assert_eq!(decoder.input.as_slice(), &source[1..1 + decoder.input.len()]);
            copied += progress.copied_bytes;
            if copy < 2 { assert_eq!(decoder.input.len(), before); break; }
            if decoder.input.len() == bytes { assert_eq!(copied, bytes * 2); break; }
            assert!(turn + 1 < bytes);
        }
        cancel.cancel_now();
        let mut actual_retained_progress = RetainedCloneProgress::default();
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(512, 999, policy), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
        let (step, heap) = observe_heap_allocations_on_this_thread(|| decoder.step(&mut cx, RetainedCloneGrant::default()));
        assert!(matches!(step, MemberSnapshotOpenStep::Rejected(MemberOpenDiagnostic::Cancelled)));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(decoder.input.as_ptr(), pointer);
        for turn in 0..512 {
            let (step, heap) = observe_heap_allocations_on_this_thread(|| decoder.close_step(policy).unwrap());
            let progress = step.progress(); assert!(progress.fits(policy));
            assert_eq!(heap.requested_bytes, progress.retained_capacity_bytes);
            assert_eq!(heap.released_bytes, progress.released_bytes);
            if decoder.terminal_is_empty() { break; }
            assert!(turn + 1 < 512);
        }
        assert!(decoder.terminal_is_empty());
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(decoder));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] actual snapshot input copy={copy} originalBytes={bytes} born={born} retained source and pointer=true funded System receipts=true cancelBeforeTypedDecode=true terminalDrop0");
    }
}
