"""Split plans of the osm slice (1-based inclusive line ranges of the pre-split files)."""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
from osmeng import FileSpec

HOST_ASYNC = ["open_scope", "spawn_scoped", "sleep_until", "cancel_scope", "now_ms"]
INTERACTIVE_JOB = ["step", "borrow_outcome", "begin_close", "close_step", "next_close_copy_byte_demand", "next_close_capacity_byte_demand", "next_close_release_byte_demand", "next_close_depth_demand", "close_demand_without_owner", "register_close_wake", "terminal_is_empty"]
TO_VALUE = ["to_value", "to_value_controlled", "to_object_key_controlled", "value_at_path", "value_shape_at_path", "value_key_at_path"]
FROM_VALUE = ["from_value", "from_value_controlled", "from_object_key_controlled"]


def ch(dir, mod, doc, ranges=None, **kw):
    d = dict(dir=dir, mod=mod, doc=doc)
    if ranges is not None:
        d["ranges"] = ranges
    d.update(kw)
    return d


def services():
    root = f"{M}/🛎️services/🦀️.rs"
    s = FileSpec("services", root, ("component",), [
        ch("🚂️host-runtime", "host_runtime", "🚂️ The process-wide worker pool handle and `TokioHostRuntime`, the one `HostAsyncRuntime` implementation, with its scope table and pool-hosted future tasks.", [(49, 418)]),
        ch("⏲️timer-wheel", "timer_wheel", "⏲️ The single host timer wheel: pure `WheelCore` arithmetic plus the finite driver chain that posts firings to a `CompletionSink`.", [(420, 645)]),
        ch("🧰️compute-pool", "compute_pool", "🧰️ The bounded compute pool: job admission, deadline and cancellation and the retained completion custody.", [(647, 672), (676, 848)]),
        ch("🌐️http-contract", "http_contract", "🌐️ The HTTP pool contract: request and response vocabulary, body cancellation, terminal custody and the blocking transport.", [(850, 1182)]),
        ch("🔌️http-socket", "http_socket", "🔌️ The bounded socket HTTP transport: framing, the cancellation handle and the connect, read-line and read-page drivers.", [(1184, 1500)]),
        ch("🪣️http-pool", "http_pool", "🪣️ The HTTP pool: the per-package byte bucket, the per-actor outstanding credit, the refill driver and the pooled response body.", [(1502, 1876)]),
        ch("📄️fixed-file-storage", "fixed_file_storage", "📄️ Fixed-size file pages and retained fixed-file documents with exact owner custody and atomic publication.", [(1878, 2137)]),
        ch("💾️storage-scheduler", "storage_scheduler", "💾️ The storage scheduler: lane-ordered dispatch under a byte budget and an in-flight bound, with deadline races.", [(2139, 2377)]),
        ch("📮️event-router", "event_router", "📮️ The indexed event router: per-subscriber mailboxes with latest-wins, lossless, coalesced and ring policies.", [(2379, 2613)]),
        ch("🧾️completion-sink", "completion_sink", "🧾️ The completion sink: the only re-entry point from the services into the kernel, and its recording mock.", [(2615, 2660)]),
    ], [(1, 33), (673, 675), "@MODS", (2662, 2666)],
        trait_methods={"HostAsyncRuntime": HOST_ASYNC, "InteractiveJob": INTERACTIVE_JOB, "Future": ["poll"], "Wake": ["wake", "wake_by_ref"]})
    s.no_trait = {"timer_wheel": {"HostAsyncRuntime"}, "http_contract": {"CompletionSink"}}
    s.root_replace = [("`grep -nE 'tokio' 🦀️.rs | grep -v", "`grep -rnE 'tokio' --include='🦀️.rs' . | grep -v")]
    return s


def services_tests():
    rel = f"{M}/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs"
    t = FileSpec("services-tests", rel, ("component", "tests"), [
        ch("🧷️support", "support", "🧷️ Shared test fixtures: the operation context, a small deterministic worker pool and a runtime-clock sleep.", [(42, 61)]),
        ch("🚂️host-runtime", "host_runtime", "🚂️ Tests of `TokioHostRuntime` over an injected worker pool and of its scope table: transitive cancellation, leaked tasks, parking and worker release between polls.", [(63, 82), (84, 180)]),
        ch("🧰️compute-pool", "compute_pool", "🧰️ Tests of the compute pool: bounded bursts, deadlines, stopped pools, original admission refusal and the original compute admission law with its fixture wallet recipient.", [(3, 36), (182, 417), (1554, 1561)]),
        ch("⏲️timers", "timers", "⏲️ Tests of the timer wheel: the pure arithmetic (ordering, boundaries, repeats, disarm, quota) and the finite driver posting through the completion sink.", [(419, 498), (500, 530)]),
        ch("💾️storage", "storage", "💾️ Tests of storage: the fixed file page against the system oracle, retained fixed-file documents and the storage scheduler.", [(532, 553), (555, 648), (650, 818)]),
        ch("📮️events", "events", "📮️ Tests of the event router queueing policies and byte bounds and of the recording completion sink.", [(820, 912), (1541, 1552)]),
        ch("🌐️http", "http", "🌐️ Tests of the HTTP pool over recording, blocking, abandoning and local-socket transports: byte charging, mid-stream drops and cancellation.", [(914, 1539)]),
    ], ["@MODS"])
    t.no_trait = {"compute_pool": {"Wake", "CompletionSink"}, "timers": {"HostAsyncRuntime"}}
    t.root_head = ["//! 🧪️ Component unit tests of the host-side async services, grouped by the unit under test."]
    return t


def transport():
    rel = f"{M}/🌉️mcp/🚚️transport/🦀️.rs"
    s = FileSpec("transport", rel, ("transport",), [
        ch("🧾️http-surface", "http_surface", "🧾️ The HTTP transport surface: admission, options, event log, state handle, terminal records and the run handle.", [(64, 457)]),
        ch("🔁️http-transport-state", "http_transport_state", "🔁️ The retained HTTP transport state machine: limits, rings, head parsing, connection phases and the per-turn drivers.", [(459, 1276)]),
        ch("🪪️http-authority", "http_authority", "🪪️ The transport authority: connection turns, retained job retry and generation custody.", [(1278, 1416)]),
        ch("📬️http-dispatch", "http_dispatch", "📬️ Owned HTTP response building and the POST, GET and bridge handshake dispatch.", [(1418, 1595)]),
        ch("🔌️websocket-codec", "websocket_codec", "🔌️ The WebSocket handshake and frame codec: accept key, SHA-1, base64 and client and server frames.", [(1597, 1805)]),
        ch("🛡️origin-and-handlers", "origin_and_handlers", "🛡️ Origin and bearer admission plus the Axum differential oracle handlers.", [(1807, 1950)]),
    ], [(1, 13), "@MODS", (1954, 1955)],
        trait_methods={"Write": ["write", "write_all", "flush", "write_fmt"], "NotificationSink": ["publish"]})
    return s


def remote():
    rel = f"{M}/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"
    s = FileSpec("remote", rel, ("remote",), [
        ch("📏️binding-limits", "binding_limits", "📏️ Hub binding limits, the network-budget wait, the refresh retry law and the component stream constants.", [(54, 165)]),
        ch("🗂️authorized-views", "authorized_views", "🗂️ Authorized document, descriptor, package and catalog views and the descriptor cache.", [(169, 238)]),
        ch("🛰️remote-binding", "remote_binding", "🛰️ The hub remote binding: state, phases, progress, observation and the binding itself.", [(167, 167), (240, 807)]),
        ch("🛠️binding-helpers", "binding_helpers", "🛠️ Binding helpers: delegated session exchange, origin and resource URIs, canonical checkpoint text and error mapping.", [(809, 1096)]),
        ch("📦️component-assets", "component_assets", "📦️ Hub component assets: transfer phases, the stream fault ladder and the asset fetcher.", [(1098, 1322)]),
        ch("🧲️native-driver", "native_driver", "🧲️ The native hub binding driver and its authorized directory dial.", [(1324, 1729)]),
        ch("💡️inference", "inference", "💡️ Hub inference on the remote binding and the native driver.", [(1731, 1922)]),
    ], [(1, 1), "@MODS"],
        trait_methods={"HostAsyncRuntime": HOST_ASYNC, "DirectoryTransport": ["http", "get_accepting", "issue_socket_grant", "open_ws"], "HubSocketGrantSource": ["admit_document_socket"], "ToValue": TO_VALUE, "FromValue": FROM_VALUE})
    return s


def bridge():
    rel = f"{M}/🌉️mcp/🧵️bridge/🦀️.rs"
    s = FileSpec("bridge", rel, ("bridge",), [
        ch("📏️wire", "wire", "📏️ Minimal length-prefixed little-endian primitives shared by both frame enums' hand-rolled\n`encode`/`decode`, private to the bridge codec; the TS twin (`🟦️.ts`) mirrors the exact same\nbyte layout independently (that agreement is what the fixtures prove).", promote=(38, 165), consume=[(34, 37), (166, 166)]),
        ch("🧱️shared-types", "shared_types", "🧱️ Shared bridge types: shell kind, flags, approval decisions and withdrawals, refusals and the instance reference.", [(170, 334)]),
        ch("📤️shell-to-gateway", "shell_to_gateway", "📤️ The shell-to-gateway frame enum and its hand-rolled codec.", [(336, 462)]),
        ch("🔍️shell-decode-cursor", "shell_decode_cursor", "🔍️ The bounded shell frame decode: limits, frame kinds, faults, the UTF-8 cursor and the validating decode cursor.", [(464, 908)]),
        ch("🧬️shell-materialize-cursor", "shell_materialize_cursor", "🧬️ The bounded shell frame materialization: the validated frame, owned ranges and the materialize cursor.", [(910, 1429)]),
        ch("📥️gateway-to-shell", "gateway_to_shell", "📥️ The gateway-to-shell frame enum, the conversation text law and the encoded page writer.", [(1431, 1743)]),
    ], [(1, 30), "@MODS", (1745, 1749)],
        trait_methods={"ToValue": TO_VALUE, "FromValue": FROM_VALUE})
    return s


GROUPS = {
    "services": lambda: _services_group(),
    "transport": lambda: [transport()],
    "remote": lambda: [remote()],
    "bridge": lambda: [bridge()],
}


def _services_group():
    a = services()
    b = services_tests()
    b.inherit = a
    return [a, b]
