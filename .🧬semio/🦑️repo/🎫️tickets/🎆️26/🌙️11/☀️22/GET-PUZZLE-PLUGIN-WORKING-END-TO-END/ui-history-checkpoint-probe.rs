use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, receive_original_retained_work};
use semio_framework_job_payload::{JobPayloadStream, RetainedPayloadBuilder};
use semio_framework_value::{RetainedCloneGrant, RetainedWorkPoll};
use semio_plugin_tool_run::ToolRunCheckpoint;

#[test]
fn checkpoint_replacement_keeps_published_bytes_until_settlement() {
    let stock = RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(71), Generation(2), stock, || Some(1), u64::MAX, 100_000);
    let mut checkpoint = ToolRunCheckpoint::default();
    let mut prior = None;
    for bytes in [[99, 98, 97, 96, 95, 94], [1, 2, 3, 4, 5, 6]] {
        let mut builder = RetainedPayloadBuilder::new(JobPayloadStream::CheckpointState);
        let mut cursor = 0;
        let birth = RetainedCloneGrant { maximum_capacity_bytes: 3, ..stock };
        for _ in 0..1024 {
            if builder.published().is_some() { break; }
            caller.turn(birth, |cx| {
                let turn = if !builder.is_initialized() { builder.advance_initialization(cx) }
                    else if cursor < bytes.len() { builder.append_original(cx, &bytes, &mut cursor) }
                    else { builder.seal(cx) };
                receive_original_retained_work(cx, turn)?;
                Ok(())
            }).unwrap();
        }
        let source = builder.published().expect("published source");
        assert!(source.page_count() > 1);
        let mut received = false;
        let copy = RetainedCloneGrant { maximum_copy_bytes: size_of::<ToolRunCheckpoint>() + 2, ..stock };
        for _ in 0..64 {
            assert_eq!(checkpoint.as_deref(), prior.as_ref().map(<[_; 6]>::as_slice));
            caller.turn(copy, |cx| checkpoint.receive(71, source, &mut received, cx)).unwrap();
            if received { break; }
        }
        assert!(received);
        assert_eq!(checkpoint.as_deref(), Some(bytes.as_slice()));
        prior = Some(bytes);
        while !builder.terminal_is_empty() {
            caller.turn(stock, |cx| {
                semio_framework_job::close_original_effect_step(cx, builder.retirement_demands()?, |effect| {
                    RetainedWorkPoll::Complete(builder.close_step(effect))
                }).map(|_| ())
            }).unwrap();
        }
    }
    caller.retire(stock);
}
