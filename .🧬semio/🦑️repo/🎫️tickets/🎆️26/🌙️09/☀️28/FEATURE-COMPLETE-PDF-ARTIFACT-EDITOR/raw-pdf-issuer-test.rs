//! 🎫️ Exercises the actual hidden picker carrier and publication code with two Original caller Sources.

use crate::{loan, STOCK, TURN};
use crate::command_origin::OriginalCommandOrigin;
use crate::instance_lifetime::{Lifetimes, NativeLifetimeSlot};
use crate::kernel::{Effect, FixedCommandPage, CommandPageCursor, ORIGINAL_MEDIA_COMMAND_KIND};
use crate::life_instance_registry::{Cell, Instances, LiveApp, PluginRuntime};
use crate::original_media::{Control, OriginalMediaLoan, Phase, RawMediaIntent, ResultCode, Transport, issue::{issue_args, pending_picker_effect}};
use crate::raw_reactor_owner::OriginalMediaAdmission;
use crate::raw_reactor_prologue_test::NativeCloseKey;
use crate::raw_publisher::continue_original_picker_effects;
use crate::PluginApp;
use semio_framework_job::{Generation, OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::RetainedCloneGrant;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Mutex;

struct Mounted {
    document: &'static str,
    revision: [u8; 32],
    effect: Option<Effect>,
    closed: bool,
}

impl PluginApp for Mounted {
    fn original_media_live_document(&self) -> Option<(&str, [u8; 32])> { Some((self.document, self.revision)) }
    fn original_media_format_closed(&self) -> bool { self.closed }
    fn original_picker_effect(&mut self) -> Option<&mut Effect> {
        self.effect.as_mut().filter(|effect| pending_picker_effect(effect))
    }
    fn discard_original_picker_effect(&mut self) { self.effect.take(); }
}

fn open(cx: &mut semio_framework_job::StepContext<'_>, stamp: OriginalMediaLoan<'_>, placement: [u64; 8])
    -> Result<Effect, semio_framework_job::OriginalTurnFailure>
{
    let mut issued = None;
    assert!(RawMediaIntent::issue(stamp, placement, ORIGINAL_MEDIA_COMMAND_KIND, 24_546, &mut issued, cx)?);
    Ok(Effect::RequestFileOpen { args: issue_args(issued.as_ref().unwrap(), cx)?,
        accept: "image/png,.png".into(), import_action: "import-local-image".into(), read_as: None, multiple: false })
}

#[test]
fn picker_effect_exposes_only_an_accepted_live_capsule_and_supersedes_paid() {
    let clock = || Some(1);
    let mut work = OriginalJobFixtureCaller::admit(OperationId(4), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    let mut source = OriginalJobFixtureCaller::admit(OperationId(5), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    work.retain_dispatch().unwrap();
    source.retain_dispatch().unwrap();
    let old = loan();
    let real_target = [1, 12, 7, 0, 0, 0, 0, 0];
    let original = work.turn(TURN, |cx| open(cx, old, real_target)).unwrap();
    let Effect::RequestFileOpen { args: Some(marker), .. } = &original else { panic!("picker carries its private typed request"); };
    let Some(semio_framework_value::DslValue::Bytes(bytes)) = marker.get("semio:original-media-issue") else { panic!("picker carrier has its exact retained byte shape"); };
    assert_eq!(bytes, include_bytes!("🗑️generated/rm-sdk/picker-marker.bin"));
    let key = NativeCloseKey { instance: old.instance_id, generation: 6, allocation: 41 };
    let runtime = PluginRuntime {
        instances: RefCell::new(Instances(key.instance, Cell { instance: Mutex::new(LiveApp { app: Mounted {
            document: old.document_id, revision: *old.base_revision, effect: Some(original), closed: true,
        } }) })),
        guest_lifetimes: RefCell::new(Lifetimes(NativeLifetimeSlot {
            key, origin: OriginalCommandOrigin::empty(), raw_media: OriginalMediaAdmission::empty(), app: PhantomData,
        })),
    };
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };
    assert!(source.turn(denied, |cx| continue_original_picker_effects(&runtime, cx)).unwrap());
    assert!(runtime.instances.borrow().0 == old.instance_id);
    assert!(runtime.guest_lifetimes.borrow().0.raw_media.terminal_is_empty());
    assert!(source.turn(TURN, |cx| continue_original_picker_effects(&runtime, cx)).unwrap());
    {
        let instances = runtime.instances.borrow();
        let mut live = instances.1.instance.lock().unwrap();
        let effect = live.app.effect.take().unwrap();
        assert!(!pending_picker_effect(&effect));
        let Effect::RequestFileOpen { args: Some(args), .. } = effect else { panic!("picker leaves the authenticated public effect"); };
        assert!(args.get("page").is_none());
        assert_eq!(args.get("originalMedia").unwrap().get("loan").unwrap().get("baseRevision").unwrap().as_str(), Some("13".repeat(32).as_str()));
    }
    let mut wrong_revision = *old.base_revision;
    wrong_revision[31] ^= 1;
    let older_foreign = OriginalMediaLoan { operation_id: old.operation_id + 1, base_revision: &wrong_revision, ..old };
    let forged = work.turn(TURN, |cx| open(cx, older_foreign, [1, 99, 7, 0, 0, 0, 0, 0])).unwrap();
    runtime.instances.borrow().1.instance.lock().unwrap().app.effect = Some(forged);
    assert!(source.turn(TURN, |cx| continue_original_picker_effects(&runtime, cx)).unwrap());
    assert!(runtime.instances.borrow().1.instance.lock().unwrap().app.effect.is_none());
    let transport = Transport { owner: u64::from(old.instance_id), seq: 1, kind: ORIGINAL_MEDIA_COMMAND_KIND, extent: 4 };
    source.turn(TURN, |cx| {
        let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
        let picker = &mut lifetimes.0.raw_media.picker;
        picker.control(old, old, transport, Control::Begin { name: "x.png", media_type: "image/png" }, cx)?;
        let cursor = CommandPageCursor { owner: transport.owner, seq: transport.seq, kind: transport.kind,
            generation: old.generation, instance: old.instance_id, command_index: 0, command_count: 1,
            page_index: 0, page_count: 1, item_count: 1, metadata: 4 };
        let mut input = Some((cursor, FixedCommandPage::try_copy_from(&[137, 80, 78, 71]).unwrap()));
        picker.page(old, &mut input, cx)?;
        Ok(())
    }).unwrap();
    for _ in 0..128 {
        let ready = source.turn(TURN, |cx| runtime.guest_lifetimes.borrow_mut().0.raw_media.picker
            .poll(old, true, cx).map(|ack| ack.is_some())).unwrap();
        if ready { break; }
    }
    source.turn(TURN, |cx| {
        let picker = &mut runtime.guest_lifetimes.borrow_mut().0.raw_media.picker;
        picker.control(old, old, transport, Control::Complete, cx)?;
        assert_eq!(picker.format_input().unwrap().current_input(old, cx)?.unwrap().target, real_target);
        Ok(())
    }).unwrap();
    let next = OriginalMediaLoan { operation_id: old.operation_id + 2, ..old };
    let effect = work.turn(TURN, |cx| open(cx, next, [1, 8, 3, 0, 0, 0, 0, 0])).unwrap();
    work.retire(TURN);
    {
        let reg = runtime.instances.borrow();
        let mut app = reg.1.instance.lock().unwrap();
        app.app.effect = Some(effect);
        app.app.closed = false;
    }
    assert!(source.turn(TURN, |cx| continue_original_picker_effects(&runtime, cx)).unwrap());
    assert!(runtime.guest_lifetimes.borrow().0.raw_media.picker.cancellation_pending());
    assert!(runtime.instances.borrow().1.instance.lock().unwrap().app.effect.as_ref().is_some_and(pending_picker_effect));
    runtime.instances.borrow().1.instance.lock().unwrap().app.closed = true;
    assert!(source.turn(TURN, |cx| continue_original_picker_effects(&runtime, cx)).unwrap());
    assert!(runtime.instances.borrow().1.instance.lock().unwrap().app.effect.as_ref().is_some_and(pending_picker_effect));
    source.turn(TURN, |cx| {
        let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
        let ack = lifetimes.0.raw_media.picker.poll(old, true, cx)?.expect("superseded Complete remains the receipt phase");
        assert_eq!((ack.phase, ack.result), (Phase::Complete, ResultCode::Stale));
        Ok(())
    }).unwrap();
    let mut published = false;
    for _ in 0..1024 {
        source.turn(TURN, |cx| continue_original_picker_effects(&runtime, cx)).unwrap();
        published = runtime.instances.borrow().1.instance.lock().unwrap().app.effect.as_ref().is_some_and(|effect| !pending_picker_effect(effect));
        if published { break; }
    }
    assert!(published);
    {
        let mut reg = runtime.guest_lifetimes.borrow_mut();
        let slot = &mut reg.0;
        assert_eq!(slot.raw_media.picker.issued_loan(), Some(next));
        while !source.turn(TURN, |cx| cx.with_original_retirement(|cx| slot.raw_media.close(cx))).unwrap() {}
        while !slot.origin.terminal_is_empty() {
            source.turn(TURN, |cx| cx.with_original_retirement(|cx| slot.origin.return_step(cx))).unwrap();
        }
    }
    source.retire(TURN);
}
