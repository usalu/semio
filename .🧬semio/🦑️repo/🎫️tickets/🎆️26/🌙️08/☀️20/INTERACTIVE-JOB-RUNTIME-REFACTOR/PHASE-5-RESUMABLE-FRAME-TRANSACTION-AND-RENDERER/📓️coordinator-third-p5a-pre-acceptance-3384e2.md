# Coordinator Third P5a Pre-acceptance Counterexamples — 2026-08-24

## Status

**RED.** The second-remediation handoff is not source-acceptable. Its isolated structural verifier
reports 44 rejected mutations, but the mounted production call graph still contains multiple
complete, dynamically allocated, blocking, or synchronous-I/O operations inside one claimed worker
opportunity. The P5a contract requires one semantic unit per grant and forbids hiding complete work
inside a renamed child call. No Cargo, Nx, Wasm, browser, or timing gate was run.

## Acceptance-blocking Counterexamples

### Blocking dynamic find-item authority

`ShellFindItemSink` remains an `Arc<Mutex<Vec<ShellFindItem>>>`. Binding pushes into a dynamic worker
stack; producer callbacks push an unadmitted dynamic owner; retire and take block on the mutex; and
`take` transfers the complete vector. `render_chrome_step` calls that whole take after every retained
main-window opportunity and replaces `self.find_items`, so the replaced vector and all owned strings
may drop recursively in the same grant. This violates fixed pre-admission, no blocking synchronization,
exact MAX + 1 rejection, and one-owner close.

Live evidence: `Shell/🫊️component.rs` lines 129–188 and 9592–9600.

Required repair: one fixed/page, generation-qualified, pre-admitted producer/collector authority with
exact owner refusal, one-item transfer, one-item close, terminal-empty witness, and a non-blocking
single-worker binding. Add faithful mutations for dynamic stack/sink storage, mutex acquisition,
unadmitted push, whole take/replacement, and recursive old-vector destruction.

### Synchronous persistence and complete preference work

Mounted `render_chrome_step` directly calls `load_ui_prefs_once`,
`read_stored_introduction_seen`, `persist_panel_layout_if_changed`,
`write_stored_introduction_seen`, and `persist_ui_prefs_if_changed`. These paths reach `prefs_get` and
`prefs_set`; the preferences load clones the complete preference owner; persistence captures several
dynamic strings, iterates and clones the complete custom-theme map, parses every JSON value, serializes
the complete result, and issues multiple storage writes in one grant. Native preference storage is
file-backed. The presence phase likewise clones dynamic identifiers, may construct a complete plugin
ephemeral snapshot, constructs the complete peer, and calls the host heartbeat synchronously.

Live evidence: `Shell/🫊️component.rs` lines 9486–9503 and 13305–13397; presence call at
9496–9499 and implementation in the `NativeBackboneSync` region.

Required repair: retain preference/introduction/layout/presence work as explicitly admitted I/O or
preview child jobs on the shared pool's appropriate lane. One opportunity may copy/parse/encode/write
one bounded field or page, publish one coalescible presence page, or close one owner. Frame completion
must park on the child or explicitly coalesce it; no synchronous platform storage call may remain in
the frame worker step. Add direct call-graph mutations restoring each opaque boundary.

### The retained paint node is not a bounded paint unit

`Ui::frame_into_step` treats one `paint_node_self` call as a worker opportunity. For a text node that
call executes `wrap_text` for the complete dynamic string, then iterates and emits every wrapped line.
Other widget variants likewise still call complete per-widget painters. A single authored node can
therefore contain arbitrarily large text or dynamic widget contents; node count is not a byte/glyph/
line time bound.

Live evidence: `ui/wgpu/🦀️engine.rs` lines 1046–1052 and
`ui/wgpu/🦀️paint.rs` lines 280–344, with complete text wrap/line emission beginning at
line 404.

Required repair: a retained per-node paint child with byte/glyph/line/item cursors and pre-admitted
candidate output. It must resume exact shaping/paint state, preserve target generation and base
witnesses, reject MAX + 1 with owner identity, and incrementally close. Add a hostile single-node
multi-megabyte text fixture and mutations that restore complete wrapping or all-line emission.

### One scene-slot call still renders the complete scene/image

`Ui::frame_into_step` calls `SceneHost::paint_slot` once and counts that as the scene opportunity.
The mounted `FrameworkSceneHost` immediately delegates to complete `render_component_scene` or
`render_ui_image`. Those functions can traverse/prepare complete scene/image resources and are the
same whole renderers the retained bridge's own comments say are unchanged. Replacing the tree walk
with a per-node lookup did not make the selected leaf's work bounded.

Live evidence: `ui/wgpu/🦀️engine.rs` lines 1065–1070 and
`Interpreter/🫊️component.rs` lines 1119–1134.

Required repair: scene/image leaves must mount their already-required retained producer consumers,
advance one exact page/instance/packet/scalar per opportunity, and block the frame child until their
generation-qualified terminal witness is reached. Add deep scene, large image, and restored whole
renderer mutations.

### Atlas credit and synchronization claims remain false

`PreparedAtlasPages::try_new` checks a page count but reserves only bytes in one global
`std::sync::Mutex<usize>`, then allocates the complete 2,048-slot owner after that reservation. Page
and item process credits are not reserved, mutex acquisition may wait, and the fixed slot backing is
not part of the byte reservation. `close_step` takes the same blocking mutex. The type has no ordinary
Drop recovery for a retained reservation, so any path that drops it before driving `close_step` leaks
process credit rather than converging on mounted terminal cleanup.

Live evidence: `ui/wgpu/🦀️prepared.rs` lines 193–284.

Required repair: use the process permit ledger's checked non-blocking token covering items, pages,
payload bytes, and authority backing before allocation. Retain the exact token; release one admitted
page/backing unit per close grant; make abandonment schedule the same close authority rather than
leaking credit or recursively dropping; add concurrent MAX + 1 identity, poisoned/contended ledger,
allocation refusal, abandoned owner, and interrupted-close laws.

## Gate Consequence

P5a remains open and cannot receive an independent GREEN audit until all five source counterexample
families are removed and the permanent verifier is strengthened with faithful mutations for them.
The current 44/44 result is useful evidence only for the narrower predicates it actually checks.
