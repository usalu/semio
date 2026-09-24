import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs")
text = path.read_text()
anchor = "/// 🛑️ A successful `Effect::CancelJob` removes the job in the same turn, before step.\n"
assert text.count(anchor) == 1
law = '''/// 🧰️ A framework reserved tool job (`semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND`: undo, redo,
/// the selection and clipboard verbs) is live-only: the spawning turn's outcome hands the host the job's
/// exact shard-minted `JobTurn`, the seed starts the job without ever checkpointing the guest (a whole
/// block2d checkpoint outgrew the fixed checkpoint pages and closed the seed, so native undo never
/// applied — ticket 26/09/23 slice WG8 §1.4), a replay request for it is refused, and the host's steps
/// with that turn run it to `Done` and deliver `Event::JobCompleted` to the spawning actor.
#[semio_framework_async_macros::async_test]
async fn a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(23);
    let package = PackageRef { package: PackageId("block".to_string()), hash: PackageHash([7u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let job_id = 71u64;
    let mut spawning_turn = MockGuestRuntime::idle_turn().await;
    spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND.to_string(), input: b"undo".to_vec(), placement: JobPlacement::Isolated });
    mock.script_turn(actor, spawning_turn).await;
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    mock.script_job_step(actor, JobStep::Done { output: b"undone".to_vec() }).await;
    mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;

    let (transport, probe) = LoopbackTransport::paired().await;
    probe.push_inbound(encode_event_envelope(actor, 1, &fixture_instance_close_event()).await).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
    shard.register(actor, instance);
    assert_eq!(pump(&mut shard).await.expect("spawning turn"), 1);
    let reported = decode_outcomes(&probe.take_outbound().await).await.into_iter().find_map(|outcome| match outcome {
        ShardOutcome::Turn { actor: turn_actor, jobs, .. } if turn_actor == actor.0 => Some(jobs),
        _ => None,
    });
    let reported = reported.expect("the spawning turn's outcome");
    assert_eq!(reported.len(), 1, "the host learns the live job's turn with the spawning turn");
    assert_eq!(reported[0].job, job_id);

    let authority = retain_replay_seed(&mut shard, actor, job_id).await;
    assert_eq!(authority, reported[0], "the reported turn is the seed's exact step authority");
    let GuestInstanceState::Mock(state) = &shard.instances.get(&actor.0).expect("registered instance").state else { panic!("mock instance") };
    assert_eq!(state.checkpoint, None, "a reserved tool job never checkpoints the guest before it starts");
    let request = shard.replay_seeds.iter().flatten().find(|seed| seed.job == job_id).and_then(|seed| seed.seed.as_ref()).map(|seed| seed.request).expect("retained seed request");
    let refusal = shard.validate_replay_request(actor.0, authority, request).expect_err("a live-only reserved job has nothing to replay from");
    assert!(refusal.to_string().contains("live-only"), "{refusal}");

    probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: authority }).await).await;
    assert_eq!(pump(&mut shard).await.expect("first step"), 1);
    probe.push_inbound(encode_payload_envelope(actor, 3, Payload::JobStep { turn: JobTurn { step_sequence: 1, ..authority } }).await).await;
    assert_eq!(pump(&mut shard).await.expect("terminal step"), 1);
    assert_eq!(pump(&mut shard).await.expect("deferred completion turn"), 1);
    let steps: Vec<JobStepOutcome> = decode_outcomes(&probe.take_outbound().await)
        .await
        .into_iter()
        .filter_map(|outcome| match outcome {
            ShardOutcome::Job { publication, .. } if publication.turn.job == job_id => Some(publication.outcome),
            _ => None,
        })
        .collect();
    assert!(matches!(steps.as_slice(), [JobStepOutcome::Yield, JobStepOutcome::Complete { candidate }] if candidate.output == b"undone"), "{steps:?}");
    let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
    assert!(matches!(completed, Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) if bytes == b"undone"), "{completed:?}");
}

'''
text = text.replace(anchor, law + anchor)
path.write_text(text)
print("ok")
