import re
base="/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/"
fx=base+"🧪️fill-fixtures-and-mounted-session-helpers/🦀️.rs"
jl=base+"🧱️fill-frontier-checkpoint-and-job-laws/🦀️.rs"
ca=base+"🔎️candidate-acceptance-preview-and-proximity/🦀️.rs"
def rep(s,old,new,count=1):
    assert old in s,old[:100]
    return s.replace(old,new,count)
s=open(fx).read()
# capture
s=rep(s,'''    fn capture_fill_snapshot(original: &mut OriginalFixture, host: &BoardHost) -> crate::editor::puzzle2d::engine::BoardFillSnapshot {
        let mut capture = begin_fill_capture(original, host);
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match original.turn(FILL_TEST_GRANT, |cx| capture.step(host, cx)).expect("original captured descriptor turn") {
                BoardFillCaptureStep::Pending => {}
                BoardFillCaptureStep::Complete => {
                    let snapshot = capture.take_snapshot().expect("complete fill capture owns snapshot");
                    retire_fill_capture(original, &mut capture);
                    return snapshot;
                }''','''    fn capture_fill_snapshot(original: &mut OriginalFixture, host: &BoardHost) -> crate::editor::puzzle2d::engine::BoardFillSnapshot {
        let mut construction = semio_framework_job::OriginalConstructionAdmission::default();
        let mut capture = begin_fill_capture(original, host, &mut construction);
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            match original.turn(FILL_TEST_GRANT, |cx| capture.step(&mut construction, &host.scene, &host.style, &host.brush, cx)).expect("original captured descriptor turn") {
                BoardFillCaptureStep::Pending => {}
                BoardFillCaptureStep::Complete => {
                    let snapshot = capture.take_snapshot().expect("complete fill capture owns snapshot");
                    retire_fill_capture(original, &mut capture);
                    retire_fill_construction(original, &mut construction);
                    return snapshot;
                }''')
s=rep(s,'''    fn begin_fill_capture(original: &mut OriginalFixture, host: &BoardHost) -> crate::editor::puzzle2d::engine::BoardFillSnapshotCapture {
        let mut output = None;
        let mut construction = semio_framework_job::OriginalConstructionAdmission::default();
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| host.begin_board_fill_snapshot(&mut construction, &mut output, cx)).expect("original capture admission") {
                retire_fill_construction(original, &mut construction);
                return''','''    fn begin_fill_capture(original: &mut OriginalFixture, host: &BoardHost, construction: &mut semio_framework_job::OriginalConstructionAdmission) -> crate::editor::puzzle2d::engine::BoardFillSnapshotCapture {
        let mut output = None;
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| host.begin_board_fill_snapshot(construction, &mut output, cx)).expect("original capture admission") {
                return''')
# close closures
old='''semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(match %s.close_step(grant) {
                    InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
                    InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
                    InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(Default::default()),
                }))?;'''
new='''semio_framework_job::close_original_retained_step(cx, demand, |grant| semio_framework_value::RetainedWorkOutcome { result: Ok(match %s.close_step(grant) {
                    InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
                    InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
                    InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(Default::default()),
                }), work: 1 })?;'''
for n in ('placement','capture'):
    s=rep(s,old%n,new%n)
# mount
s=rep(s,'''            if let Some(session) = original.turn(FILL_TEST_GRANT, |cx| pending.admit_pending(config, cx)).expect("same original mounted admission") {
                assert!(pending.terminal_is_empty());
                return session;
            }''','''            let mut output = None;
            if original.turn(FILL_TEST_GRANT, |cx| pending.admit_pending(config, &mut output, cx)).expect("same original mounted admission") {
                assert!(pending.terminal_is_empty());
                return output.expect("admitted mounted session stays in caller custody");
            }''')
# close_fill_session body
i=s.index('            original.turn(FILL_TEST_GRANT, |cx| {\n                if let Some(step) = session.return_original_cancel_alias_step')
j=s.index('            }).expect("same original mounted fill close");')
s=s[:i]+'''            original.turn(FILL_TEST_GRANT, |cx| {
                if session.has_original_cancel_alias_witness(cx.original_cancel_token()).map_err(|error| match error {
                    semio_framework_job_worker::WorkerJobDemandError::Refused(error) => semio_framework_job::OriginalTurnFailure::from(error),
                    semio_framework_job_worker::WorkerJobDemandError::Contention(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original fill session remains contended").into(),
                })? {
                    let turn = cx.with_original_effect(cx.retained_grant(), |cx, effect| session.return_original_cancel_alias_step(cx.original_cancel_token(), effect))?.map_or(semio_framework_value::RetainedWorkOutcome { result: Ok(None), work: 0 }, |(turn, _)| turn);
                    let result = turn.result.map(|step| step.expect("same original witnessed cancellation alias")).map_err(|error| match error {
                        semio_framework_job_worker::WorkerJobDemandError::Refused(error) => error,
                        semio_framework_job_worker::WorkerJobDemandError::Contention(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original fill session alias remains checked out"),
                    });
                    semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome { result, work: turn.work })?;
                } else {
                    session.close_step(cx)?;
                }
                Ok(())
'''+s[j:]
# pump
i=s.index('    /// 🔁️ `WorkerPool::try_submit` is a TRY')
j=s.index('    /// 🔎️ Settles the checked-out step receipt')
s=s[:i]+'''    /// 🔁️ One original-caller turn of the mounted session: the business producer runs on the caller's own funded turn.
    fn pump_fill_session(original: &mut OriginalFixture, session: &mut FillSession) -> WorkerJobPoll {
        original.turn(FILL_TEST_GRANT, |cx| session.pump_one_for_interactive_turn(cx)).expect("original mounted fill pump turn")
    }

'''+s[j:]
# ack
i=s.index('    fn acknowledge_fill_outcome')
j=s.index('    fn adopt_fill_checkpoint')
s=s[:i]+'''    fn acknowledge_fill_outcome(original: &mut OriginalFixture, session: &mut FillSession) {
        for _ in 0..FILL_TEST_PUMP_LIMIT {
            if original.turn(FILL_TEST_GRANT, |cx| session.acknowledge_checked_out_outcome(cx)).expect("original fill outcome acknowledgement") { return; }
        }
        panic!("fill outcome acknowledgement exceeded bounded opportunities");
    }

'''+s[j:]
open(fx,'w').write(s)

def fix_pool(path):
    s=open(path).read()
    s=re.sub(r'^ *let pool = semio_framework_async::WorkerPool::new\([^\n]*\n','',s,flags=re.M)
    s=re.sub(r'^ *pool\.shutdown\(\)\.expect\("bounded snapshot operation succeeds"\);\n','',s,flags=re.M)
    lines=s.split('\n');out=[];sig=''
    for ln in lines:
        if re.match(r'\s*(pub )?fn ',ln):sig=ln
        m=re.search(r'pump_fill_session\(&mut (\w+), &pool\)',ln)
        if m:
            o='original' if 'original: &mut OriginalFixture' in sig else '&mut original'
            ln=ln.replace(m.group(0),'pump_fill_session(%s, &mut %s)'%(o,m.group(1)))
        out.append(ln)
    open(path,'w').write('\n'.join(out))
for p in (fx,jl,ca):fix_pool(p)
