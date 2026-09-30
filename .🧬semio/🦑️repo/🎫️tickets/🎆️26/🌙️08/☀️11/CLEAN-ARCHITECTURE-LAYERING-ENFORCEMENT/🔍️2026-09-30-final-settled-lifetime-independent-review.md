# Final Settled Lifetime Independent Review

Read-only review of current shared source, 2026-09-30. No build or runtime test executed independently. Parent-reported passes and pending BREP validation are not independent evidence here.

## Actionable Byte Accounting Gaps

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs:278–282`: the generic metadata Vec cursor pops individual values, but its final empty Vec retains the original capacity. The final expansion ordinarily destroys that backing allocation and reports one item and zero bytes at lines 264–266. A large manifest topic/dependency/contribution vector therefore releases its backing under a one-byte grant without admitting its capacity. Transfer the empty backing into a byte-accounted frontier, as native arena retirement already does. `MetadataRetirement::pending` at lines 253–260 also retains Vec capacity while `is_empty` reports true, leaving backing destruction to terminal bundle Drop. Its backing needs an explicit terminal retirement stage if byte bounds include allocated metadata storage.

2. Same file lines 82–85: terminal resource owner removal destroys the boxed concrete owner and reports zero bytes, with no owner shell-size declaration or grant admission. The trait permits arbitrary concrete owner shell sizes. Registry final bundle removal at lines 236–245 likewise reports zero bytes. Admit actual owner/bundle shell byte requirements before removal; handler shell removal already demonstrates such admission at lines 95–100.

3. `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:761–763`: final SessionCapture shell destruction is admitted by any positive byte grant and reports zero released bytes. The final native payload has already drained, so this is shallow destruction; nevertheless it does not implement the requested final-shell byte accounting. The non-final Arc reader release at lines 753–756 is shallow and atomically delegates final state ownership to the surviving reader. The exclusive transition at line 757 additionally allocates a new Arc rather than retaining the extracted state directly.

These are byte accounting findings, not evidence of geometric traversal hidden in terminal destruction.

## Lifetime Boundaries Confirmed From Source

SessionCapture uses `Arc::into_inner` to distinguish a released non-final reader from the actual final authority, retains final family retirement, and has an explicit unfinished Drop refusal. Session `close_step` lines 860 onward checks zero grants and pause before authority sealing. Shared claims census and exclusive kernel/cache acquisition protect native reader handoff; typed retirement transfers all ten arenas, live geometry, cached mesh buffers, and retained tessellation payloads. Cold `Session::close` stops on zero progress and is therefore not an unconditional family-terminal promise when peers remain.

ExtensionBundle now guards manifest, handlers, resource owner and metadata with explicit terminal-empty retirement. Installation accepts `&mut Option<ExtensionBundle>`; an already pending replacement returns false without taking the rejected candidate (`extension/retirement/🦀️.rs:188–201`). Invocation is sealed before resource retirement. Resource Complete is checked against actual terminal emptiness (lines 87–89), and reported item/byte overspend is rejected. Replacement activation waits for predecessor retirement; cancellation pauses the retained registry and resume preserves the sealed family.

The reactor turn at `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1009–1025` routes Request to extension invocation, SuspendRequest to sealing, and schedules retained retirement from positive fuel. Reserved empty-payload cancel/resume requests are routed by plugin source lines 44320–44326. The WASM event bridge now preserves CapabilityChanged and QuotaChanged variants (`🔌️plugin/⚛️reactor/🦀️.rs:1477–1478`); host encoding supplies them at `🔌️plugin/🖥️host/🦀️.rs:3620–3621`. The turn deliberately performs no extension mutation for those variants; this review does not infer a new policy requirement from their presence.

BREP `extension_guest` now attaches `BrepExtensionResources` and four owned handlers instead of unowned captured closures (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:2121–2180`). It delegates real Session close, terminal checks, cancel and resume. The former lifecycle gap is closed in source. Its cold manifest builder at lines 2080–2087 explicitly drops the ColdOwner registry before calling Session.close. The actual BREP fixture drives real nonempty tessellation through zero grant, sealing, pause/resume and terminal checks; no independent execution claim is made.

## Scope

Review covered the changed Session capture/family retirement, native typed frontier, neutral extension owner/metadata registry, real BREP owner, reactor scheduling, and WASM event conversion. No production edits, Git changes, ticket lifecycle mutations, or duplicate builds were performed.
