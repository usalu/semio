//! 🧪️ Binds the actual staged reactor prologue to a minimal mounted instance and real Original caller.

use crate::command_origin::OriginalCommandOrigin;
use crate::kernel::*;
use crate::original_media::RawMediaPicker;
use crate::raw_reactor_owner::OriginalMediaAdmission;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Mutex;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NativeCloseKey { pub instance: u32, pub generation: u64, pub allocation: u64 }
impl NativeCloseKey { pub fn instance(self) -> u32 { self.instance } }

pub trait PluginApp {
    fn original_media_live_document(&self) -> Option<(&str, [u8; 32])>;
    fn original_media_format_closed(&self) -> bool { true }
    #[cfg(raw_pdf_issuer_module)]
    fn original_picker_effect(&mut self) -> Option<&mut Effect> { None }
    #[cfg(raw_pdf_issuer_module)]
    fn discard_original_picker_effect(&mut self) {}
    fn advance_original_media_format(
        &mut self, _picker: &mut RawMediaPicker, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> { Ok(()) }
}

pub mod os_vcs {
    pub mod io {
        pub mod binary {
            pub mod entity_identity {
                pub mod control {
                    pub struct EntityIdentityAuthority<'a>(pub &'a ());
                }
            }
        }
    }
}
use os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

pub mod instance_lifetime {
    use super::*;
    pub use super::NativeCloseKey;
    pub struct NativeLifetimeSlot<PA> {
        pub key: NativeCloseKey,
        pub origin: OriginalCommandOrigin,
        pub raw_media: OriginalMediaAdmission,
        pub app: PhantomData<PA>,
    }
    impl<PA> NativeLifetimeSlot<PA> {
        pub fn admit_command_origin(&mut self, key: NativeCloseKey, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
            if self.key != key { return Ok(false); }
            self.origin.admit_step(cx)
        }
        #[cfg(raw_pdf_issuer_module)]
        pub fn admit_original_media(
            &mut self, key: NativeCloseKey, offered: &mut Option<crate::original_media::RawMediaIntent>,
            current: crate::original_media::OriginalMediaLoan<'_>, format_closed: bool, cx: &mut StepContext<'_>,
        ) -> Result<bool, OriginalTurnFailure> {
            if key.instance() != current.instance_id || !self.admit_command_origin(key, cx)? { return Ok(false); }
            if !self.raw_media.terminal_is_empty() {
                if !format_closed { self.raw_media.picker.request_supersession(cx)?; return Ok(false); }
                if !self.raw_media.close(cx)? { return Ok(false); }
            }
            self.raw_media.admit_issued(offered, current, cx)
        }
    }
    pub struct Lifetimes<PA>(pub NativeLifetimeSlot<PA>);
    impl<PA> Lifetimes<PA> {
        pub fn get_mut(&mut self, id: u32) -> Option<&mut NativeLifetimeSlot<PA>> {
            (self.0.key.instance == id).then_some(&mut self.0)
        }
        pub fn next_raw_media_poll(&mut self, closing: &[u32]) -> Option<u32> {
            let id = self.0.key.instance;
            (self.0.raw_media.picker.receipt_due() && !closing.contains(&id)).then_some(id)
        }
    }
}

pub mod life_instance_registry {
    use super::*;
    pub struct LiveApp<PA> { pub app: PA }
    pub struct Cell<PA> { pub instance: Mutex<LiveApp<PA>> }
    pub struct Instances<PA>(pub u32, pub Cell<PA>);
    impl<PA> Instances<PA> {
        pub fn get(&self, id: u32) -> Option<&Cell<PA>> { (id == self.0).then_some(&self.1) }
        #[cfg(raw_pdf_issuer_module)]
        pub fn next_entry_from(&self, index: usize) -> Option<(usize, u32, &Cell<PA>)> {
            (index == 0).then_some((0,self.0,&self.1))
        }
    }
    pub struct PluginRuntime<PA> {
        pub instances: RefCell<Instances<PA>>,
        pub guest_lifetimes: RefCell<instance_lifetime::Lifetimes<PA>>,
    }
    impl<PA> PluginRuntime<PA> {
        pub fn live_instance_key(&self, instance: u32) -> Result<NativeCloseKey, ValueError> {
            let key = self.guest_lifetimes.borrow().0.key;
            if key.instance != instance {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "fixture lifetime is not live"));
            }
            Ok(key)
        }
    }
}

pub enum Event { OriginalMediaControl(OriginalMediaControlEvent) }
#[cfg(raw_pdf_issuer_module)]
pub enum Effect {
    RequestFileOpen {
        args: Option<semio_framework_value::DslValue>, accept: String, import_action: String,
        read_as: Option<String>, multiple: bool,
    },
    #[allow(dead_code)]
    Other,
}

#[path = "🗑️generated/rm-reactor/prologue.rs"]
mod prologue;

#[test]
fn prologue_separates_raw_control_and_page_from_generic_assembly() {
    use crate::{loan, STOCK, TURN};
    use crate::original_media::OriginalMediaPickerIntent;
    use instance_lifetime::{Lifetimes, NativeLifetimeSlot};
    use life_instance_registry::{Cell, Instances, LiveApp, PluginRuntime};
    use semio_framework_job::{Generation, OperationId};
    use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
    struct Pdf;
    impl PluginApp for Pdf {
        fn original_media_live_document(&self) -> Option<(&str, [u8; 32])> {
            Some((loan().document_id, *loan().base_revision))
        }
    }
    let mut source = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 24);
    source.retain_dispatch().unwrap();
    let id = loan().instance_id;
    let key = NativeCloseKey { instance: id, generation: 3, allocation: 100 };
    let runtime = PluginRuntime {
        instances: RefCell::new(Instances(id, Cell { instance: Mutex::new(LiveApp { app: Pdf }) })),
        guest_lifetimes: RefCell::new(Lifetimes(NativeLifetimeSlot {
            key, origin: OriginalCommandOrigin::empty(), raw_media: OriginalMediaAdmission::empty(), app: PhantomData,
        })),
    };
    let mut issuer = None;
    source.turn(TURN, |cx| {
        assert!(OriginalMediaPickerIntent::issue(loan(), [0; 8], ORIGINAL_MEDIA_COMMAND_KIND, 24_546, &mut issuer, cx)?);
        let slot = &mut runtime.guest_lifetimes.borrow_mut().0;
        assert!(!slot.admit_command_origin(NativeCloseKey { allocation: 101, ..key }, cx)?);
        assert!(slot.admit_command_origin(key, cx)?);
        assert!(slot.raw_media.admit_issued(&mut issuer, loan(), cx)?);
        Ok(())
    }).unwrap();
    let mut event = Some(Event::OriginalMediaControl(OriginalMediaControlEvent {
        owner: u64::from(id), seq: 2, kind: ORIGINAL_MEDIA_COMMAND_KIND, extent: 4,
        loan: OriginalMediaWireLoan { operation_id: format!("{:016x}", loan().operation_id),
            generation: format!("{:016x}", loan().generation), instance_id: id, document_id: loan().document_id.into(),
            base_revision: "13".repeat(32) },
        control: OriginalMediaWireControl::Begin { name: "x.png".into(), media_type: "image/png".into() },
    }));
    let mut identity = EntityIdentityAuthority(&());
    let mut page = None;
    let denied = semio_framework_value::RetainedCloneGrant { maximum_release_bytes: 0, ..TURN };
    let waiting = source.turn(denied, |cx| prologue::drive_raw_media(&runtime, &mut event, &mut page, &[], &mut identity, cx)).unwrap();
    assert!(matches!(waiting, Some(CommandIngressStatus::Idle)) && event.is_some());
    let status = source.turn(TURN, |cx| prologue::drive_raw_media(&runtime, &mut event, &mut page, &[], &mut identity, cx)).unwrap();
    assert!(matches!(status, Some(CommandIngressStatus::OriginalMediaAck(ack)) if ack.result == OriginalMediaWireResult::Accepted));
    assert!(event.is_none());
    let cursor = CommandPageCursor { owner: u64::from(id), generation: loan().generation, command_index: 0, command_count: 1,
        instance: id, seq: 2, kind: 137, page_index: 0, page_count: 1, item_count: 1, metadata: 4 };
    page = Some((cursor, FixedCommandPage::try_copy_from(&[137, 80, 78, 71]).unwrap()));
    assert!(source.turn(TURN, |cx| prologue::drive_raw_media(&runtime, &mut event, &mut page, &[], &mut identity, cx)).unwrap().is_none());
    page.as_mut().unwrap().0.kind = ORIGINAL_MEDIA_COMMAND_KIND;
    let status = source.turn(TURN, |cx| prologue::drive_raw_media(&runtime, &mut event, &mut page, &[], &mut identity, cx)).unwrap();
    assert!(matches!(status, Some(CommandIngressStatus::OriginalMediaAck(ack)) if ack.result == OriginalMediaWireResult::Pending));
    assert!(page.is_none());
    let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
    let slot = &mut lifetimes.0;
    while !source.turn(TURN, |cx| cx.with_original_retirement(|cx| slot.raw_media.close(cx))).unwrap() {}
    while !slot.origin.terminal_is_empty() {
        source.turn(TURN, |cx| cx.with_original_retirement(|cx| slot.origin.return_step(cx))).unwrap();
    }
    source.retire(TURN);
}
