import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:90])
    text = text.replace(old, new)


replace(
    """    /// @emoji 🚀️ Creates one finite-turn actor on the process WorkerPool. Tokio remains only the
    /// platform I/O reactor; it never owns an actor or timer thread.
    pub(super) async fn spawn_actor(""",
    """    /// 🔌️ The one platform I/O reactor every native document actor polls its hub socket against: a
    /// current-thread Tokio runtime whose only thread (`semio-document-io`) parks on the I/O and time
    /// drivers, created on first use and never shut down, like `db_storage_driver_runtime`. Actors keep
    /// running on the `WorkerPool` and only enter this handle while polled. It used to be whatever
    /// runtime the spawning thread happened to be inside; a native wgpu shell spawns from a plain
    /// thread, so the first hub dial panicked with `there is no reactor running` and the document
    /// socket never left `Connecting` (ticket 26/09/23 slice WG8, `hub-live-collaboration-check` run 1).
    /// `None` only when the operating system refuses the reactor, which the dial then reports as a fault.
    fn document_socket_io_reactor() -> Option<tokio::runtime::Handle> {
        static REACTOR: std::sync::OnceLock<Option<tokio::runtime::Handle>> = std::sync::OnceLock::new();
        REACTOR
            .get_or_init(|| {
                let runtime = tokio::runtime::Builder::new_current_thread().enable_io().enable_time().build().ok()?;
                let handle = runtime.handle().clone();
                std::thread::Builder::new().name("semio-document-io".to_string()).stack_size(1 << 20).spawn(move || runtime.block_on(std::future::pending::<()>())).ok()?;
                Some(handle)
            })
            .clone()
    }

    /// @emoji 🚀️ Creates one finite-turn actor on the process WorkerPool. Tokio remains only the
    /// platform I/O reactor ([`document_socket_io_reactor`]); it never owns an actor or timer thread.
    pub(super) async fn spawn_actor(""",
)
replace(
    "        let io_reactor = tokio::runtime::Handle::try_current().ok();\n        let mailbox = cmd_rx.close_handle();",
    "        let io_reactor = document_socket_io_reactor();\n        let mailbox = cmd_rx.close_handle();",
)
path.write_text(text)
print("ok")
