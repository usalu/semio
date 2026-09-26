#!/usr/bin/env python3
"""WG8 s12: the live fair-turns law measured the other half of the monopoly — a 28 MB component's parse
(`OwnedRuntime::compile`, synchronous, ~177 s in a debug host) held the kernel loop for the whole open (the other app's
longest answer 177 s). The parse now runs on the worker pool while the loop serves the other instances' requests, a
handle is reused per content hash, and the open's owner may cancel it while it parses."""
import pathlib

RENDERER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = RENDERER.read_text()


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:120]
    text = text.replace(old, new)


swap(
    "            let compiled = self.guest_runtime.compile(&package_ref, bytes).await.map_err(|error| error.to_string())?;\n",
    "            let compiled = self.compile_serving_others(package_ref.clone(), bytes.to_vec()).await?;\n",
)
swap(
    """        /// ⚖️ Gives the other actors one request between two slices of `actor`'s turn on instance `busy` (the""",
    """        /// 🧩️ Compiles `bytes` on the worker pool while this loop keeps serving the other instances' requests — a
        /// large component parses for minutes in a debug host, and the synchronous parse used to hold the whole
        /// loop — reusing the handle already compiled for the same content hash. The open's owner may cancel it
        /// while it parses ([`Self::turn_cancelled`]); the parse then finishes unobserved on its worker.
        async fn compile_serving_others(&mut self, package: PackageRef, bytes: Vec<u8>) -> Result<semio_framework_plugin_host::CompiledHandle, String> {
            if let Some((_, compiled)) = self.compiled_components.iter().find(|(hash, _)| *hash == package.hash) {
                return Ok(compiled.clone());
            }
            let handoff: Arc<Mutex<(Option<Result<semio_framework_plugin_host::CompiledHandle, String>>, Option<Waker>)>> = Arc::default();
            let worker = handoff.clone();
            let runtime = self.guest_runtime.clone();
            let key = package.clone();
            crate::renderer_worker_pool().submit(
                semio_framework_async::Lane::Background,
                Box::new(move || {
                    let result = semio_framework_async::block_on(runtime.compile(&key, &bytes)).map_err(|error| error.to_string());
                    let waker = {
                        let mut slot = worker.lock().expect("compile handoff lock");
                        slot.0 = Some(result);
                        slot.1.take()
                    };
                    if let Some(waker) = waker {
                        waker.wake();
                    }
                }),
            );
            loop {
                let gap = std::future::poll_fn(|cx| {
                    let mut slot = handoff.lock().expect("compile handoff lock");
                    if let Some(result) = slot.0.take() {
                        return Poll::Ready(Some(result));
                    }
                    slot.1 = Some(cx.waker().clone());
                    drop(slot);
                    if self.turn_cancelled() {
                        return Poll::Ready(Some(Err("kernel: the open was cancelled while its component compiled".to_string())));
                    }
                    if !self.interleaving && self.request_queue.head_is(|request| kernel_request_interleavable(request, u32::MAX)) {
                        return Poll::Ready(None);
                    }
                    self.request_queue.watch(cx.waker());
                    Poll::Pending
                })
                .await;
                match gap {
                    Some(result) => {
                        let compiled = result?;
                        if self.compiled_components.len() == COMPILED_COMPONENT_CAPACITY {
                            self.compiled_components.pop();
                        }
                        self.compiled_components.insert(0, (package.hash, compiled.clone()));
                        return Ok(compiled);
                    }
                    None => self.serve_head_between(u32::MAX).await,
                }
            }
        }

        /// ⚖️ Serves the head request when it may run beside work mid-flight on instance `busy`, marking the gap so a
        /// request served in it never opens another.
        async fn serve_head_between(&mut self, busy: u32) {
            let Some((request, slot)) = self.request_queue.try_next_if(|request| kernel_request_interleavable(request, busy)) else { return };
            self.interleaving = true;
            let outcome = self.serve(request, &slot).await;
            self.interleaving = false;
            if let Some(outcome) = outcome {
                slot.deliver(outcome);
            }
        }

        /// ⚖️ Gives the other actors one request between two slices of `actor`'s turn on instance `busy` (the""",
)
swap(
    """            let suspended = self.runtime.kernel_mut().suspend(actor, None).await.is_ok();
            self.interleaving = true;
            for _ in 0..TURN_REQUESTS_BETWEEN_SLICES {
                let Some((request, slot)) = self.request_queue.try_next_if(|request| kernel_request_interleavable(request, busy)) else { break };
                if let Some(outcome) = self.serve(request, &slot).await {
                    slot.deliver(outcome);
                }
            }
            self.interleaving = false;
            if suspended {""",
    """            let suspended = self.runtime.kernel_mut().suspend(actor, None).await.is_ok();
            for _ in 0..TURN_REQUESTS_BETWEEN_SLICES {
                self.serve_head_between(busy).await;
            }
            if suspended {""",
)
swap(
    """        /// 👀️ Whether the head request satisfies `test` (`false` for an empty or contended queue).""",
    """        /// 🔔️ Wakes `waker` when the next request is admitted — the loop's own wait, borrowed by a request that
        /// serves other instances while it waits for its own work ([`KernelPoolState::compile_serving_others`]).
        fn watch(&self, waker: &Waker) {
            if let Ok(mut state) = self.state.try_lock() {
                state.consumer_waker = Some(waker.clone());
            }
        }

        /// 🔔️ Wakes the loop so a request waiting in a gap re-reads its requester's cancel.
        fn wake_consumer(&self) {
            let waker = self.state.try_lock().ok().and_then(|mut state| state.consumer_waker.take());
            if let Some(waker) = waker {
                waker.wake();
            }
        }

        /// 👀️ Whether the head request satisfies `test` (`false` for an empty or contended queue).""",
)
swap(
    """            if !self.finished && self.request.is_none() {
                self.slot.abandoned.store(true, std::sync::atomic::Ordering::Release);
            }""",
    """            if !self.finished && self.request.is_none() {
                self.slot.abandoned.store(true, std::sync::atomic::Ordering::Release);
                self.queue.wake_consumer();
            }""",
)
swap(
    """        /// ⚖️ True while a request runs in another actor's slice gap ([`Self::yield_slice`]): a gap never nests.
        interleaving: bool,
""",
    """        /// ⚖️ True while a request runs in another actor's slice gap ([`Self::yield_slice`]): a gap never nests.
        interleaving: bool,
        /// 🧩️ The components compiled most recently, newest first, by content hash ([`Self::compile_serving_others`]).
        compiled_components: Vec<(PackageHash, semio_framework_plugin_host::CompiledHandle)>,
""",
)
swap(
    """                serving: Vec::with_capacity(2),
                interleaving: false,
""",
    """                serving: Vec::with_capacity(2),
                interleaving: false,
                compiled_components: Vec::with_capacity(COMPILED_COMPONENT_CAPACITY),
""",
)
swap(
    """    const TURN_REQUESTS_BETWEEN_SLICES: usize = 1;
""",
    """    const TURN_REQUESTS_BETWEEN_SLICES: usize = 1;

    /// 🧩️ How many compiled components the kernel keeps for reuse by content hash — a reopened kind, a second
    /// window of one app — before the oldest is dropped (its live instances keep their own handle).
    const COMPILED_COMPONENT_CAPACITY: usize = 8;
""",
)
RENDERER.write_text(text)
print("compile gap applied")
