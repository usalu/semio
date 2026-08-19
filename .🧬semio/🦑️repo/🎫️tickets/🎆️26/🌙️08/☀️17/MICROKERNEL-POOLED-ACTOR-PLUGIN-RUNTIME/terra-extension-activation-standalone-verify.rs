// 🧪️ Standalone runtime verification of `extensions_extending`/`scope_capabilities_to_parent`'s
// exact logic (byte-for-byte copied from `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`'s new
// `//#region 🔖️ExtensionActivation`), compiled directly with `rustc` (no workspace dependency
// graph) because `cargo test` on all three crates that mount this file
// (`semio-framework` / `semio-framework-graph` / `semio-s-plugin-stdio`) currently fails on
// PRE-EXISTING, UNRELATED errors — see `📓️terra-extension-activation-report.md` for exit codes.
// This proves the ALGORITHM at runtime; the real mounted file's compilation is proven separately
// by `cargo check -p semio-framework --lib` (EXIT 0, pasted in the report).

use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CapabilityId(String);

#[derive(Clone, Debug, PartialEq)]
struct ExtensionDescriptor {
    extension_id: String,
    extends: String,
}

async fn extensions_extending<'a>(plugin_id: &str, installed: &'a [ExtensionDescriptor]) -> Vec<&'a ExtensionDescriptor> {
    installed.iter().filter(|descriptor| descriptor.extends == plugin_id).collect()
}

async fn scope_capabilities_to_parent(parent_effective: &[CapabilityId], requested: &[CapabilityId]) -> Vec<CapabilityId> {
    requested.iter().filter(|id| parent_effective.contains(id)).cloned().collect()
}

fn block_on<F: Future>(future: F) -> F::Output {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("block_on: future was not ready on first poll"),
    }
}

fn descriptor(extension_id: &str, extends: &str) -> ExtensionDescriptor {
    ExtensionDescriptor { extension_id: extension_id.into(), extends: extends.into() }
}

fn main() {
    // 🧫️ 2,500-descriptor stand-in for the scale fixture's own shape: 50 "plugins", 50 extensions
    // each, extends cycling through them — proves zero special-casing by count.
    let installed: Vec<ExtensionDescriptor> = (0..2500)
        .map(|i| descriptor(&format!("ext-{i}"), &format!("plugin-{}", i % 50)))
        .collect();
    assert_eq!(installed.len(), 2500);

    let matched = block_on(extensions_extending("plugin-7", &installed));
    assert_eq!(matched.len(), 50, "each of 50 plugins gets exactly 50 extensions in this synthetic fixture");
    assert!(matched.iter().all(|d| d.extends == "plugin-7"));
    assert!(matched.iter().all(|d| d.extension_id.parse::<u32>().is_err())); // sanity: ids are "ext-N"

    let none = block_on(extensions_extending("plugin-nonexistent", &installed));
    assert!(none.is_empty());

    let parent = vec![CapabilityId("storage.read".into()), CapabilityId("http:example.com".into())];
    let requested = vec![CapabilityId("storage.read".into()), CapabilityId("storage.write".into())];
    let scoped = block_on(scope_capabilities_to_parent(&parent, &requested));
    assert_eq!(scoped, vec![CapabilityId("storage.read".into())]);

    let empty_parent = block_on(scope_capabilities_to_parent(&[], &requested));
    assert!(empty_parent.is_empty());

    println!("ALL ASSERTIONS PASSED: extensions_extending + scope_capabilities_to_parent, 2500-descriptor scale");
}
