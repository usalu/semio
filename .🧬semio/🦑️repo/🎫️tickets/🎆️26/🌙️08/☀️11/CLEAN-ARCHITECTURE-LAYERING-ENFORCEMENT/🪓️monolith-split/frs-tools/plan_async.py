"""async: lift the clusters and inline modules of ⏳️async/🦀️.rs into children (see splitter.py)."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="async",
    imports=(36, 43),
    root_drop=["AtomicU8", "BinaryHeap", "HashMap", "Hash", "Hasher", "OnceLock", "RwLock"],
    root_cfg={"AtomicU32": ['#[cfg(test)]'], "Future": ['#[cfg(any(test, feature = "testkit"))]'], "Pin": ['#[cfg(any(test, feature = "testkit"))]'], "Context": ['#[cfg(test)]'], "Poll": ['#[cfg(test)]']},
    children=[
        C(kind="inline", mod="browser", folder="🌐️browser", vis="pub", head=61, tail=99,
          doc=["//! 🌐️ Browser-only `Promise`/`JsValue` bridge: owned microtask spawn and Rust-future-to-Promise adapters (excluded from `wasm32-wasip2`)."]),
        C(kind="inline", mod="schema_metadata", folder="🏷️schema-metadata", vis="pub", head=106, tail=159,
          doc=["//! 🏷️ Versioned wire-type rows and the TypeScript projection of this crate's owned vocabulary (`cfg(test)` only)."]),
        C(kind="cluster", mod="operation_context", folder="🪪️operation-context", ranges=[(162, 209)],
          doc=["//! 🪪️ `OperationContext`: the per-operation value carrying cancellation, deadline, trace, lane and capability."]),
        C(kind="cluster", mod="cancel_token", folder="⛔️cancel-token", ranges=[(211, 244), (248, 253), (265, 472)],
          doc=["//! ⛔️ `CancelToken`: the `Arc`-shared tri-state cancellation handle with parent links, the `Cancelled` future and `CancelState`."]),
        C(kind="cluster", mod="scope", folder="🌳️scope", ranges=[(474, 597)],
          doc=["//! 🌳️ Host-runtime vocabulary: scopes, channel policy and the `HostAsyncRuntime` trait."]),
        C(kind="cluster", mod="block_on_entry", folder="🌉️block-on", ranges=[(599, 651)],
          cfg=['#[cfg(any(test, feature = "entrypoint"))]'],
          import_cfg={"Waker": ['#[cfg(target_arch = "wasm32")]']},
          doc=["//! 🌉️ Entry-point-only `block_on`: parks the thread on native, spins a no-op waker on wasm."]),
        C(kind="cluster", mod="lane", folder="🛣️lane", ranges=[(653, 846)],
          doc=["//! 🛣️ Scheduling vocabulary: `ProcessKind`, the six weighted `Lane`s and the `PermitLedger`."]),
        C(kind="cluster", mod="timer_wheel", folder="⏰️timer-wheel", ranges=[(848, 1058)],
          doc=["//! ⏰️ `TimerWheel`: the deadline heap the worker pool fires from, and its `TimerSleep` future."]),
        C(kind="inline", mod="oneshot", folder="🪢️oneshot", vis="pub", head=1065, tail=1177,
          doc=["//! 🪢️ Single-value handoff between one `Sender` and one `Receiver` (replaces `tokio::sync::oneshot`)."]),
        C(kind="cluster", mod="sync_primitives", folder="🚦️sync-primitives", ranges=[(1180, 1326)],
          doc=["//! 🚦️ `Notify` and the owned-permit `Semaphore`: the repo-owned wait primitives."]),
        C(kind="cluster", mod="combinators", folder="🔀️combinators", ranges=[(1328, 1421)],
          doc=["//! 🔀️ Future combinators: `select2`, `race2`, `join2` and the cooperative `yield_once`."]),
        C(kind="cluster", mod="sharded_map", folder="🗺️sharded-map", ranges=[(1423, 1573)],
          doc=["//! 🗺️ `ShardedMap`: the fixed-shard concurrent map and its deterministic shard hasher."]),
        C(kind="cluster", mod="boxed_slots", folder="🧱️boxed-slots", ranges=[(1575, 1661)],
          doc=["//! 🧱️ Boxed fixed-slot constructors, `FixedSlotTableBudget` and the bounded-stack runner."]),
        C(kind="cluster", mod="worker_pool", folder="🧵️worker-pool", ranges=[(1663, 1669), (1679, 1750), (2631, 2646)],
          doc=["//! 🧵️ Worker-pool vocabulary: `Job`, admission errors, `WorkerPoolConfig` and the one process-wide pool accessor."]),
        C(kind="inline", mod="native_pool", folder="🖥️native-pool", vis="", head=1757, tail=2265,
          extra_uses=["use std::sync::atomic::AtomicU32;"],
          doc=["//! 🖥️ Native work-stealing `WorkerPool`: OS worker threads, lane queues, timers and parking."]),
        C(kind="inline", mod="wasm_pool", folder="🕸️wasm-pool", vis="", head=2270, tail=2623,
          doc=["//! 🕸️ Cooperative single-thread `WorkerPool` for wasm targets and tests: deficit-round-robin lane selection over a manual pump."]),
        C(kind="inline", mod="testkit", folder="🎭️testkit", vis="pub", head=2658, tail=2904,
          doc=["//! 🎭️ `ManualRuntime`: an injected-clock `HostAsyncRuntime` test double that needs no tokio."]),
    ],
)


def main():
    plan = Plan(SPEC).build()
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:3000])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:6000])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:60])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
