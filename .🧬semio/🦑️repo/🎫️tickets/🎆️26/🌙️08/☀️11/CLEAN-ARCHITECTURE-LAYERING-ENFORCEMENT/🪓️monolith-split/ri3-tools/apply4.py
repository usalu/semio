"""🪝 RI3: the runtime-mailbox lib named `runtime_publication_tests::pause_after_completion_publication()`, a module of its test companion (an upward reference); it becomes a per-thread hook port of the lib, installed by the law, and `enqueue_runtime_completion` is `pub` so the companion law can drive it."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from common_ri3 import *
from edit import apply

LIB = f"{T}/🧊️renderer/🎨️presentation-raster/🦀️.rs"
LAW = REPO + "/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/📥️enqueue/🦀️.rs"
MAN = f"{T}/🪤️runtime-mailbox-tests/📦️packages/🦀️rust/Cargo.toml"
GATE = '#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]\n'
HOOK = GATE + '''thread_local! {
    static AFTER_COMPLETION_PUBLICATION: std::cell::RefCell<Option<Box<dyn Fn()>>> = const { std::cell::RefCell::new(None) };
}

/// 🪝️ Installs (`Some`) or clears (`None`) this thread's hook that [`enqueue_runtime_completion`] runs right after a completion is published and its scene invalidated, before the host waker: the seam a law uses to hold the writer between the two steps.
''' + GATE + '''pub fn set_after_completion_publication_hook(hook: Option<Box<dyn Fn()>>) {
    AFTER_COMPLETION_PUBLICATION.with(|slot| *slot.borrow_mut() = hook);
}

'''
apply([
    (LIB, "fn enqueue_runtime_completion(", "pub fn enqueue_runtime_completion(", 1),
    (LIB, "\n/// 📮️ Publishes one completion and its scene invalidation as ONE observation.", HOOK + "/// 📮️ Publishes one completion and its scene invalidation as ONE observation.", 1),
    (LIB, GATE + "    runtime_publication_tests::pause_after_completion_publication();\n", GATE + "    AFTER_COMPLETION_PUBLICATION.with(|slot| {\n        if let Some(hook) = slot.borrow().as_ref() {\n            hook();\n        }\n    });\n", 1),
    (LAW, "use super::{enqueue_runtime_completion, RuntimeApply, RuntimeCompletion, RuntimeCompletionQueue, RuntimeHostWaker, RuntimePresentationAuthority};\n", "use super::{RuntimeApply, RuntimeCompletion, RuntimeCompletionQueue, RuntimePresentationAuthority};\nuse semio_framework_os_renderer_wgpu_runtime_mailbox::{enqueue_runtime_completion, set_after_completion_publication_hook, RuntimeHostWaker};\n", 1),
    (LAW, "pub(super) fn pause_after_completion_publication() {", "fn pause_after_completion_publication() {", 1),
    (LAW, "            let accepted = enqueue_runtime_completion(queue, presentation, waker, completion);\n", "            set_after_completion_publication_hook(Some(Box::new(pause_after_completion_publication)));\n            let accepted = enqueue_runtime_completion(queue, presentation, waker, completion);\n            set_after_completion_publication_hook(None);\n", 1),
])
