# Native Resident Initialization Boundary

## Existing Entry Points Read

`plugin_exports!` and `extension_exports!` create persistent parameterless PluginRuntime shells. Their Once installers build the bundle before ordinary dispatch. `owned_abi::PollInput` currently contains events, command_page and Budget; `semio_owned_poll_v1` decodes that value, calls the bundle installer, and invokes `poll_kernel`. The canonical WIT `reactor.poll` also takes only events, command-page and budget. There is no existing initialize/configure export in that reactor interface.

On the host, `GuestRuntime::instantiate` takes compiled component, actor, capabilities and Budget. `WasmtimeRuntime::instantiate` constructs ActorHostState and Store, installs BudgetLimiter and instantiates the component. `OwnedGuestRuntime::instantiate_actor` similarly instantiates the owned artifact. Neither path reserves a native composition domain before construction. SharedEngineConfig/ BudgetLimiter memory limits are not substituted for such a reservation.

Direct parameterless native Runtime construction also appears in the descriptor helper, the Demonstrator fixture, main Plugin test helpers, checkpoint fixtures and lifecycle tests. The descriptor helper is not allocation-free by current source: the exported describe path calls the shared bundle installer first, then allocates its output. An empty runtime shell must become allocation-free, but that future constructor law does not exempt current descriptor/bundle work from admission.

## Native Versus Guest Lifetime

Native UI contract and UI runtime static arenas are shared by every PluginRuntime in the same process. They must be bound once to one exact composition owner, with checked repeat installation for the same private owner. Equality of capacity numbers is not equality of owner identity. A second native runtime borrows a domain reservation from that same composition; it cannot construct another ledger and claim the same UI static backing. Closing one app or one runtime cannot release the UI parent envelope while any other exact UI owner remains.

Each Wasm component memory has a distinct copy of guest statics. Its composition lifetime is that actual component activation, not an app instance id. The host must reserve that guest's declared allocation envelope from its own actual composition before instantiate. The guest subdivides that reservation, including its one UI domain, and does not cause the host to charge the same bytes again as an unrelated UI allocation. Simultaneous native host buffers and guest copies are different physical owners and remain separately charged.

Guest static/data/stack allocation occurs during instantiation, before the first poll. Therefore a first-poll configuration alone cannot retroactively prove preallocation admission. Host pre-instantiation reservation and measured compiled-component static/layout requirements are necessary, separate from the guest's later allocation-free checked configuration installation. No such host reservation is currently mounted.

## Minimal Existing-Poll Field Proposal

Add one required canonical `composition` record to the existing poll input, shared by WIT and the owned native poll route. It carries the shared capacity contract's exact total and control bytes/slots/owners, with no defaults. It is construction configuration, not an app opcode, UI permit, raw-page ACK or return-origin replacement. The already-captured activation/return origin binds it to the exact host guest activation; no extra actor-name, instance-id or synthesized-generation authority is introduced.

The worker/host captures this record before instantiation from its actual admitted guest-domain owner. Every later dispatch/control/ACK uses that frozen original configuration, including old-activation cleanup after routing changes. The guest accepts initial installation only into its original empty composition slot; an exact retry resumes that same slot and any partial construction. A foreign owner/configuration, control-before-install or changed capacity is refused before semantic work. Existing fixed return protocol-fault/refusal handling carries failures; there is no new initialization export or variable-error compatibility branch.

For native in-process callers there is no numeric surrogate configuration-to-permit conversion. An explicit composition object owns the ledger, and the UI install boundary accepts the same private affine domain record/owner. Fixtures construct an explicit composition with declared capacity and share it where native static consumers are involved. Tests do not reset a global ledger or fall back to a hidden pool. The public empty PluginRuntime constructor does not allocate descendants; binding the actual composition and its original registry/Opening records is required before producer entry.

The poll configuration check must precede the current bundle-install Once and every app/registry producer. The current `$ensure()` cannot remain an implicit constructor callable from every export. It becomes a non-initializing exact-composition guard; only the canonical poll initialization phase may begin the original retained bundle construction. Cold bundle construction is not automatically covered by checking configuration: its actual retained construction owner and allocation envelope remain required. This proposal does not award admission credit to that existing synchronous initializer.

## Every Export Must Join The Same Guard

The complete shared actor macro currently calls `$ensure()` from poll, start_job, step_job, take_segmented_download_chunk, checkpoint, restore and describe. cancel_job bypasses ensure and directly enters the job registry. The plugin's owned exports separately call ensure from poll, start_job, step_job, cancel_job, checkpoint, restore and describe. Both families must call the same exact admitted original-composition guard before any producer, rather than each invoking a private initialization fallback.

The host must complete the admitted initialization poll before describing or dispatching another producer export. Calling describe first is no longer allowed to implicitly build a bundle. Result-bearing exports report the canonical retained fault without replacing the original root. Existing void/list-returning exports need an exact host preflight and a fixed fail-closed guest trap on invalid entry unless they are removed in the already-planned canonical paged-return cutover; returning an empty list or success is not a refusal. This is an explicit required gate, not an assertion that current signatures already carry a typed refusal.

Closing-phase guards distinguish new work from cleanup of exact previously issued jobs/returns/ACKs. Revoking new operations must not disable the original composition's drain path. Each existing cleanup owner remains registered and is validated before access; neither a closing boolean nor numeric job absence is a retirement receipt.

The owned allocation exports are an additional bootstrap boundary: `semio_owned_alloc_v1` currently allocates before input parsing, and owned poll decodes a whole JSON input before ensure. Host pre-instantiation admission must include the exact static bootstrap/input storage, and the canonical fixed-page input cutover must retain that original source before decoding. A variable unadmitted allocator call cannot be declared safe merely because the later poll field is valid. No extra initialization export, JSON compatibility union or side allocator authority is proposed.

Native bundle installer link shims currently register the installer function; they do not themselves justify invoking its bundle constructor. Direct native producer entry points and any `ensure_plugin_initialized` path must require the same actual composition binding. Native fixtures explicitly bind the real original composition before driving those producers.

## Existing UI Admission Consumers

The current source census finds three direct production reservation entry points: `UiDocumentBuilder::new` in contract/📦packages/🦀rust/🦀document.rs, `UiDocumentAssembly::open` in contract/📄document/🎟assembly/🦀component.rs, and runtime reconcile's resident reservation helper. All three must use the same installed UI parent domain, preserving the original permit's root/output split and final paired release. Runtime static handback/output backing registration remains on that same domain and cannot register a second capacity total.

`UiDocumentAssembly::open_with_permit` consumes an existing exact root permit rather than reserving again. Document slots retain owner1 through aliases/typed retirement; runtime Ready/Published/Ack retains owner2. Their existing exact close and atomic deferred-return paths are the policy/ownership surfaces to reuse. Test-only direct try_reserve callers require explicit fixture composition installation too; no test reset is introduced.

## Required Tests Before Mounting

- Two real native Runtime shells sharing one explicit composition cannot duplicate UI parent credit; a different private parent with equal numeric capacity is refused.
- Two actual guest activations retain independent guest domains funded by the same host composition; same-activation numeric app id reuse does not reinstall or release that domain.
- Host admission refusal precedes the real component instantiate call. Guest first-poll refusal precedes bundle/app construction. Partial construction and constructor unwind retain the exact original slots.
- Exercise every listed WIT and owned producer export before initialization, during partial initialization, after valid binding and during close. Observe actual bundle/producer invocation counts; an uninitialized describe or start_job must not reach the constructor. Include the owned allocation/input bootstrap as its own exact source admission law.
- Identical configuration retry is idempotent; changed capacity or foreign activation refuses without replacing the original owner. Old ACK/control dispatch retains its captured configuration.
- One runtime/app close leaves the other native runtime and shared UI descendants live. Composition close waits for exact final UI/static/registry/guest obligations, not a slot count or absence of an app id.
- Native fixtures use explicit capacity and real owner identity; Wasm checks execute the same canonical configuration path. No default total or threshold increase is part of the packet.

Status: proposed minimal field and concrete source census only. No schema/ABI field, caller, global installation, parent reservation or guest allocation authority has been changed. Parent and host/transport owner must agree this one field before ABI edits; the allocation-free capacity RED4 remains independent.
