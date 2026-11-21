//! 🧪 Holds the first native dialog open while the live shell retires another document's picker loan.
extern crate self as semio_framework;
extern crate self as semio_renderer_wgpu_program_bridge;
pub extern crate semio_framework_kernel as kernel;
pub use semio_framework_value::DslValue;

#[path = "media.rs"]
mod media;
#[path = "picker.rs"]
mod picker;

pub use media::NativeOriginalMediaRequest;
use kernel::{CommandIngressStatus, Effect, Event, OriginalMediaWireAck, OriginalMediaWireControl,
    OriginalMediaWirePhase, OriginalMediaWireResult, ORIGINAL_MEDIA_COMMAND_KIND};
use media::NativeOriginalMediaTransfer;
use semio_framework_value::Number;
use std::{cell::{Cell, RefCell}, future::Future, io::Read, path::PathBuf, pin::Pin, rc::Rc, task::{Context, Poll, Waker}};

thread_local! {
    static WORKERS: RefCell<Vec<Pin<Box<dyn Future<Output = ()>>>>> = RefCell::default();
    static DISMISS: Cell<bool> = const { Cell::new(false) };
    static OPEN: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
    static CANCEL: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

#[derive(Default)]
struct OpeningState {
    native_original_media: Vec<picker::NativeOriginalMediaTask>,
    import_transfer_next: u64,
}

struct DocumentState { session: Option<Session>, spawned_session: Vec<Session> }
struct Session { plugin_id: String, app: App, instance_id: u32 }
struct App { controller_id: String }
struct PluginState { plugins: Vec<ProgramBridgeEntry> }
struct Owner { app_instance_id: u32, parent_document_id: Option<String> }

#[derive(Clone)]
struct ProgramBridgeEntry {
    plugin_id: String, package_id: Option<String>, component_sha256: Option<String>, documents: Vec<(u32, String)>,
}

impl ProgramBridgeEntry {
    async fn read_app_document_identity(&self, instance: u32) -> Result<Owner, String> {
        OPEN.with_borrow_mut(|open| open.push(instance));
        let document = self.documents.iter().find(|(id, _)| *id == instance).unwrap().1.clone();
        Ok(Owner { app_instance_id: instance, parent_document_id: Some(document) })
    }

    async fn deliver_original_media<F: Future<Output = ()> + Send>(
        &self, request: &NativeOriginalMediaRequest,
        source: Result<Option<(impl Read + Send, String, &'static str, u64)>, String>,
        observe: impl Fn(Vec<Effect>) -> F + Clone + Send,
        current: impl Fn() -> bool + Send + Sync, progress: impl FnMut(u64, u64) + Send,
    ) -> Result<bool, String> {
        NativeOriginalMediaTransfer::run(request, 103, source, |event| {
            let observe = observe.clone();
            async move {
                let Event::OriginalMediaControl(event) = event else { panic!("expected media control") };
                assert!(matches!(event.control, OriginalMediaWireControl::Cancel));
                assert_eq!(event.extent, 0);
                assert_eq!(event.loan.instance_id, request.instance_id());
                assert!(request.matches_document(event.loan.instance_id, Some(&event.loan.document_id)));
                CANCEL.with_borrow_mut(|cancel| cancel.push(event.loan.instance_id));
                observe(vec![Effect::Notify { message: event.loan.operation_id.clone() }]).await;
                Ok(CommandIngressStatus::OriginalMediaAck(OriginalMediaWireAck {
                    loan: event.loan, owner: event.owner, seq: event.seq, kind: event.kind, extent: 0,
                    phase: OriginalMediaWirePhase::Cancel, page_index: None, result: OriginalMediaWireResult::Accepted,
                }))
            }
        }, current, progress).await
    }
}

#[derive(Clone, Default)]
struct CancelToken(std::sync::Arc<std::sync::atomic::AtomicBool>);
impl CancelToken {
    fn root_now() -> Self { Self::default() }
    fn cancel_now(&self) { self.0.store(true, std::sync::atomic::Ordering::Relaxed); }
    fn is_cancelled_now(&self) -> bool { self.0.load(std::sync::atomic::Ordering::Relaxed) }
}

struct ShellDetached<T>(Rc<RefCell<Option<T>>>);
impl<T: 'static> ShellDetached<T> {
    fn spawn(worker: impl Future<Output = T> + Send + 'static) -> Self {
        let result = Rc::new(RefCell::new(None));
        let slot = Rc::clone(&result);
        WORKERS.with_borrow_mut(|workers| workers.push(Box::pin(async move {
            *slot.borrow_mut() = Some(worker.await);
        })));
        Self(result)
    }

    fn take(&self) -> Option<T> { self.0.borrow_mut().take() }
}

mod ui_host {
    use super::*;

    pub struct NativeFileDialogRequest;
    impl NativeFileDialogRequest {
        pub fn open(_: [&str; 1], _: bool) -> Self { Self }
    }
    pub async fn select_native_paths(_: NativeFileDialogRequest) -> Vec<PathBuf> {
        std::future::poll_fn(|_| if DISMISS.get() { Poll::Ready(Vec::new()) } else { Poll::Pending }).await
    }
}

fn tick_workers() {
    WORKERS.with_borrow_mut(|workers| workers.retain_mut(|worker|
        worker.as_mut().poll(&mut Context::from_waker(Waker::noop())).is_pending()));
}

fn request(instance: u32, document: &str, operation: &str) -> NativeOriginalMediaRequest {
    let string = |value: &str| DslValue::String(value.into());
    let number = |value| DslValue::Number(Number::UInt(value));
    let loan = DslValue::object([
        ("operationId".into(), string(operation)),
        ("generation".into(), string("8000000000000007")),
        ("instanceId".into(), number(u64::from(instance))),
        ("documentId".into(), string(document)),
        ("baseRevision".into(), string(&"0123456789abcdef".repeat(4))),
    ]);
    let request = DslValue::object([("originalMedia".into(), DslValue::object([
        ("loan".into(), loan),
        ("kind".into(), number(u64::from(ORIGINAL_MEDIA_COMMAND_KIND))),
        ("maximumFileBytes".into(), number(24546)),
    ]))]);
    NativeOriginalMediaRequest::parse(&request, instance).unwrap().unwrap()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut mount = || (
        args.next().unwrap().parse::<u32>().unwrap(), args.next().unwrap(), args.next().unwrap(),
    );
    let mounts = [mount(), mount()];
    let program = ProgramBridgeEntry {
        plugin_id: "pdf".into(), package_id: Some("pdf".into()), component_sha256: Some("current".into()),
        documents: mounts.iter().map(|(id, _, document)| (*id, document.clone())).collect(),
    };
    let mut sessions = mounts.iter().map(|(id, controller, _)| Session {
        plugin_id: program.plugin_id.clone(), app: App { controller_id: controller.clone() }, instance_id: *id,
    });
    let docs = DocumentState { session: sessions.next(), spawned_session: sessions.collect() };
    let plugins = PluginState { plugins: vec![program.clone()] };
    let mut opening = OpeningState { import_transfer_next: 1, ..OpeningState::default() };
    let issues: Vec<_> = args.collect::<Vec<_>>().chunks_exact(2).map(|issue| {
        let id = issue[0].parse::<u32>().unwrap();
        (id, issue[1].clone())
    }).collect();
    let mut issue = |(id, operation): &(u32, String)| {
        let (_, controller, document) = mounts.iter().find(|(instance, _, _)| instance == id).unwrap();
        opening.queue_original_media(program.clone(), controller.clone(), request(*id, document, operation));
    };
    for loan in &issues { issue(loan); }
    tick_workers();
    println!("{:?}", OPEN.with_borrow(Clone::clone));
    println!("{:?}", CANCEL.with_borrow(Clone::clone));
    let receipts = opening.poll_original_media(&docs, &plugins);
    let effects: Vec<_> = receipts.into_iter().map(|receipt| {
        assert!(matches!(receipt.result, Some(Ok(false))));
        assert_eq!(receipt.effects, vec![Effect::Notify { message: issues.last().unwrap().1.clone() }]);
        receipt.controller
    }).collect();
    println!("{effects:?}");
    let (picker_id, _) = opening.active_original_media().expect("first picker remains open");
    assert!(!opening.cancel_original_media(picker_id + 1));
    assert!(opening.cancel_original_media(picker_id));
    DISMISS.set(true);
    tick_workers();
    assert_eq!(opening.poll_original_media(&docs, &plugins).len(), 1);
    assert!(opening.active_original_media().is_none());
    DISMISS.set(false);
    let (id, controller, document) = &mounts[1];
    opening.queue_original_media(program, controller.clone(), request(*id, document, "0000000000000003"));
    tick_workers();
    println!("{:?}", OPEN.with_borrow(Clone::clone));
}
