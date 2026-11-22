//! 🧪 Compiles the staged reactor route and slot methods against real retained-effect dependencies.

extern crate self as semio_framework;
extern crate self as semio_plugin_decl_media;
extern crate self as semio_plugin_reactor_app;
extern crate self as semio_plugin_reactor_lifecycle;

pub mod kernel {
    pub use crate::wire::*;
    pub use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};

    pub enum Event {
        Wake,
        NaturalSourcePrepare(NaturalSourcePrepare),
        NaturalSourceControl(NaturalSourceControlEvent),
    }
    pub enum CommandIngressStatus {
        Idle,
        Backpressure(CommandPageCursor),
        NaturalSourceAck(NaturalSourceWireAck),
    }
}

pub mod original_media {
    pub use crate::{natural_source, natural_source_gate};
}

pub trait PluginApp {
    fn natural_source_capability(&self) -> Option<(natural_source_gate::NaturalSourceDeclaration<'static>, u32)> { None }
    fn natural_source_format_closed(&self) -> bool { true }
}

pub mod instance_lifetime {
    use crate::{kernel::*, natural_source::*, natural_source_gate::*, PluginApp};
    #[derive(Clone, Copy)]
    pub struct NativeCloseKey { pub lifetime: Lifetime }
    #[derive(Clone, Copy)]
    pub struct Lifetime { pub guest_lifetime: u64, pub instance: u32 }
    impl NativeCloseKey { pub fn instance(self) -> u32 { self.lifetime.instance } }
    pub struct NativeLifetimeSlot<PA> {
        pub natural_source: Option<NaturalSourceGuest<{ NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES as usize }>>,
        natural_source_launch: Option<NaturalSourceLaunchGrant>,
        native_created: bool,
        app: std::marker::PhantomData<PA>,
    }
    impl<PA: PluginApp> NativeLifetimeSlot<PA> {
        pub fn admit_command_origin(&mut self, _key: NativeCloseKey, _cx: &mut semio_framework_job::StepContext<'_>)
            -> Result<bool, semio_framework_job::OriginalTurnFailure> { unreachable!() }
    }
    include!("slot-routes.rs");
}

pub mod life_instance_registry {
    use crate::instance_lifetime::NativeCloseKey;
    use crate::PluginApp;
    use semio_framework_value::ValueError;

    pub struct StubRegistry;
    impl StubRegistry {
        pub fn next_natural_source_poll(&mut self, _: &[u32]) -> Option<u32> { unreachable!() }
    }

    pub struct PluginRuntime<PA> {
        pub guest_lifetimes: std::cell::RefCell<StubRegistry>,
        app: std::marker::PhantomData<PA>,
    }
    impl<PA: PluginApp> PluginRuntime<PA> {
        pub fn live_instance_key(&self, _: u32) -> Result<NativeCloseKey, ValueError> { unreachable!() }
    }
}

mod raw_media {
    use crate::{PluginApp, instance_lifetime::NativeLifetimeSlot, kernel::*, life_instance_registry::PluginRuntime};
    use semio_framework_job::{OriginalTurnFailure, StepContext};
    use semio_framework_value::ValueError;

    fn with_slot<PA: PluginApp, R>(
        _: &PluginRuntime<PA>, _: u32, _: &mut StepContext<'_>,
        _: impl FnOnce(&mut NativeLifetimeSlot<PA>, &mut PA, &mut StepContext<'_>) -> Result<R, OriginalTurnFailure>,
    ) -> Result<Option<R>, OriginalTurnFailure> { unreachable!() }

    fn reactor_close_fault(message: &'static str) -> ValueError { ValueError::invariant(message) }
    fn fault(_: CommandPageCursor, _: &[u8]) -> CommandIngressStatus { unreachable!() }

    mod natural { include!("route.rs"); }
}
