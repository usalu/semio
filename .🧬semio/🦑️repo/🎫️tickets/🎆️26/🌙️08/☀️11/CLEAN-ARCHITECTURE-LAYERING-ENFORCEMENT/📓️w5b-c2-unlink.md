# W5B — C2 Unlink Investigation (procedural → 7 flow-extension Cargo deps)

## Verdict: BLOCKED — no source edits made

The real WIT-contract change this unlink requires is not a small, safe,
one-pass edit. It is a new host-import surface with **no existing working
backing logic anywhere in the real codebase to mirror**, layered on top of a
host-side extension registry that **does not exist at all** outside an
isolated ticket-folder scratch prototype, layered on top of 7 extension
crates that have **never been wired for the extension-world feature**, on
top of a live in-process geometry-kernel sharing invariant that a real
component boundary would break. Each layer alone might be a one-pass job;
stacked, they are not verifiable end-to-end in this pass. Per this ticket's
explicit stop condition, zero edits were made — investigation only.

## 1. The 7 dependencies, confirmed real (not dev)

`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:49-55`, all in
`[dependencies]` (not `[dev-dependencies]`), all `default-features = false`:

```
semio-s-plugin-flow-extension-brep       ../../../🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust
semio-s-plugin-flow-extension-math       .../🧮️math/...
semio-s-plugin-flow-extension-primitive  .../🔤️primitive/...
semio-s-plugin-flow-extension-logic      .../🧠️logic/...
semio-s-plugin-flow-extension-dictionary .../📖️dictionary/...
semio-s-plugin-flow-extension-list       .../📃️list/...
semio-s-plugin-flow-extension-text       .../📝️text/...
```

Audit finding C2 confirmed as stated.

## 2. Today's mechanism (in-process function pointers, not WIT)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs:71-84` —
  `LINKED_FLOW_EXTENSION_INSTALLERS: LazyLock<Mutex<BTreeMap<String, fn(&mut neural::Registry)>>>`
  and `register_linked_flow_extension_installer(extension_id, install)`. Comment on
  line 71-72 says explicitly: "Preferred over `ContributedExtensionStub` until
  extension-world WIT invoke is wired." `build_flow_extension_registry`
  (line 127-133) calls every registered `install(&mut registry)` in-process,
  synchronously, no serialization, no wire boundary.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:649-662` —
  `ensure_linked_flow_extensions()` calls
  `flow::register_linked_flow_extension_installer("brep", semio_s_plugin_flow_extension_brep::register)`
  (and the other 6) directly — a real Rust function pointer to a sibling
  crate, only possible because of the Cargo dependency. Called from
  procedural's `🦀️component.rs:8`, `🎛️apps/🧊️3d/🦀️component.rs:474,479`,
  `🎛️apps/◻2d/🦀️component.rs:352,357`, and its own test module.

## 3. The WIT contract has no extension-invoke-shaped host import today

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`:

- `world plugin-world { import host; export plugin; }` — procedural (a
  plugin-world guest) can only call into `interface host`. It does **not**
  import `interface extension` — that interface is only ever *exported* by
  `extension-world` guests, and *called* host-side (by the host's own
  `ExtensionRuntime`, not by another guest).
- `interface host` has no `extension-invoke`-shaped function. The closest
  candidate, `invoke-action: func(target: string, invocation: list<u8>) ->
  result<list<u8>, list<u8>>` (line 82), is real WIT but its **only two
  implementations in the entire real codebase are unimplemented stubs**:
  - `🔌️plugin/🖥️host/🦀️component.rs:350` (`HostState`, the real plugin
    host) — `Err(host_fault_bytes("os.host.invoke-action", "invoke-action
    not implemented"))`
  - same file, `:899` (`ExtensionHostState`, the extension host) — same
    "not implemented" fault.
  There is no working pattern anywhere in the real (non-scratch) tree to
  mirror for a new host import — the one thing that looks like it would be
  the template is itself dead code.

Adding a genuine `extension-invoke` function to `interface host` changes a
WIT surface shared by **every** plugin-world and extension-world guest in
the repo (not just procedural) — every guest's generated `Host` binding
would need the new method, and the Rust `Host` trait impl on both
`HostState` and `ExtensionHostState` would need real bodies, not stubs, for
this to be worth anything.

## 4. `ExtensionRuntime` is real (per the prior wave's prototype) but wired to nothing

`🔌️plugin/🖥️host/🦀️component.rs:960-1082` — `ExtensionRuntime` (load_bytes/
manifest/extension_invoke) is exactly the mechanism the w5b prototype
(`📓️w5b-extension-prototype.md`) proved works end-to-end with a real
compiled component. But `grep -rln "ExtensionRuntime"` across the entire
repo (excluding the ticket folder and this agent's own worktree) returns
**only that one file** — it is never constructed, held, or referenced by
any other real module. There is no host-level registry anywhere that:
- discovers the N extension `.wasm` components to load,
- keeps `ExtensionRuntime` instances keyed by `extension_id`,
- would let a new `host.extension-invoke` implementation actually dispatch
  to the right one.

That registry is itself a real piece of infrastructure to design and build,
not a wiring afterthought.

### 4b. Two different things are both named `PluginHost`

- `🔌️plugin/🖥️host/🦀️component.rs::PluginHost` — the real wasmtime
  component loader (wasm bytes → instance), the natural home for an
  extension registry.
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs::PluginHost` (line
  60) — a **native, in-process program supervisor**: `load_plugin` takes an
  already-in-process `LoadedProgram` (manifest + native app registrations),
  no wasm involved at all. This is the path native/dev/test builds use.

procedural's Cargo.toml enables `semio-framework-plugin`'s `component-guest`
feature by default, so it *can* compile as a genuine wasm guest — but
whether the host path actually exercised for procedural at runtime is the
wasmtime `PluginHost` or the native in-process `PluginHost` (or both,
depending on build target) was not resolved by this investigation, and a
real `extension-invoke` host import would need to work correctly under
whichever path(s) procedural actually runs under.

## 5. None of the 7 extension crates build under `component-extension-guest` — the feature doesn't exist in them

Checked all 7 (`Cargo.toml` `[features]` table), all byte-identical:

```
[features]
default = ["component-guest"]
component-guest = ["dep:semio-framework-plugin", "dep:semio-framework"]
```

None declare `component-extension-guest` at all — not as non-default, not
anywhere. "Feature-flip them to build as real extension-world components"
(the task's hypothesis to check) is false as a *default-flip*: the feature
and its `extension_exports!`-based wiring (mirroring the w5b prototype's
from-scratch `w5b-extension-echo` guest crate) does not exist in any of the
7 crates and would need to be authored per-crate, then verified with a real
`cargo build --target wasm32-wasip2` per crate (as the prototype did for
its one crate) — 7x the surface the prototype covered, with zero of it done
yet.

## 6. The brep-kernel geometry-sharing coupling is real and live, not hypothetical

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs` operates on
`GeometryHandle` (from `semio_framework_3d::brep::engine::BrepKernel`) via
`flow_extension_sdk::brep_geometry::tessellate_geometry(&handle, tolerance)`.
Separately, the framework os-flow registry file
(`🌊️flow/📔️registry/🦀️component.rs:29`) imports the **same**
`dispose_geometry/export_solid_json/import_solid_json/retain_geometry_handles/
tessellate_geometry` functions directly, and procedural's own
`ensure_linked_flow_extensions` doc comment says outright (engine
`🦀️component.rs:647-648`): "Registers in-process flow extension operators
so eval + tessellate share one brep kernel." This is a single in-process
`BrepKernel` instance backing both procedural's own tessellation calls and
the brep extension's operators today. Moving brep to a genuine, separate
wasm component means either (a) exposing the `BrepKernel`/`GeometryHandle`
store through new host functions so both sides resolve the same handle
across the component boundary, or (b) giving up the "share one brep kernel"
invariant this code explicitly relies on. Neither is mechanical; both are
real design decisions the audit's "GeometryHandle ids, not raw geometry,
should cross the wire" framing gestures at but does not resolve.

## Why the "safe preparatory part" carve-out was not taken

The task allowed landing just the WIT host import + host dispatch even if
the full unlink was deferred, "if you're confident that part alone is
correct and non-breaking." Not taken, because:

- It would touch `interface host` in `world.wit`, shared by every
  plugin-world **and** extension-world guest in the repo — real blast
  radius, not scoped to procedural.
- With no `ExtensionRuntime` registry wired into either `HostState` or
  `ExtensionHostState` (section 4), any new method's body could only be
  another `Err("not implemented")` stub — i.e. a second dead stub next to
  `invoke-action`'s existing one. That is not verifiable progress; it is
  unreachable code added on spec, and this ticket's own rules require
  validating behavior, not assuming it.
- There is no way to boot a real host + a real compiled extension guest +
  a real compiled procedural guest to prove a round trip in this pass —
  the exact thing that made the w5b prototype convincing (a real compiled
  component, not a type-check) is unavailable here because none of the 7
  extensions build under any extension-world feature yet.

## Recommended follow-up ticket scope (not started here)

1. Design + build the host-side extension registry (discovery, load,
   `extension_id` → `ExtensionRuntime` keying) and decide which `PluginHost`
   (wasmtime vs. native in-process, section 4b) it lives on/is reachable
   from for procedural's actual runtime path.
2. Add `extension-invoke` (or equivalent) to `interface host` in
   `world.wit`, with a real (non-stub) `Host` impl on `HostState` backed by
   (1), regenerate/verify guest bindings repo-wide.
3. Resolve the brep-kernel geometry-sharing design (section 6) — likely a
   host-exposed geometry-handle resolution function, not a guest-side
   change.
4. Author `component-extension-guest` feature + `extension_exports!` wiring
   in each of the 7 flow-extension crates (mirroring
   `w5b-extension-prototype/guest/`), verify each with a real
   `cargo build --target wasm32-wasip2 --release` per crate.
5. Only then: wire procedural's calls through the new host import, delete
   the 7 Cargo dependencies + `register_linked_flow_extension_installer`/
   `ensure_linked_flow_extensions`/`ContributedExtensionStub`, verify
   `cargo check` across procedural + the 7 extensions + host/plugin crates.

## Files read (no files edited)

- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
  (procedural3d engine `component.rs`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (native `PluginHost`)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml` +
  `🦀️component.rs` + `📦️glue.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/{🧮️math,🔤️primitive,🧠️logic,📖️dictionary,📃️list,📝️text}/📦️packages/🦀️rust/Cargo.toml`
  (`[features]` tables only)
- `📓️w5b-extension-prototype.md`, `📓️w5b-flow-core-verdict.md` (this
  ticket's prior-wave background, read first per the task)

## Operational note: agent was sandboxed into an isolated git worktree

This investigation was launched with instructions to work directly in
`/Users/ueli/Documents/semio` and explicitly not use worktree isolation.
The subagent was nonetheless started with `cwd` pinned to
`/Users/ueli/Documents/semio/.claude/worktrees/agent-af15980ad8f731e73` (a
real `git worktree`, branch `worktree-agent-af15980ad8f731e73`, currently at
the same commit as the shared checkout). `ExitWorktree` refused to run
("cannot be called from a subagent with a cwd override"), so the isolated
cwd could not be left. That worktree's own copy of this ticket folder was
missing `📓️w5b-extension-prototype.md` / `📓️w5b-flow-core-verdict.md`
entirely (stale relative to the live tree at worktree-creation time). All
*reads* of the live shared tree were done via explicit absolute
`/Users/ueli/Documents/semio/...` paths, which the `Read` tool allowed
(only literal `git` invocations against the shared path are refused by this
sandbox). The `Write` tool, however, refused to write to that same absolute
shared-tree path and required this file to be written into the worktree
copy instead — so **this findings file physically lives only in the
worktree** (`.claude/worktrees/agent-af15980ad8f731e73/...`), not in the
live shared ticket folder the orchestrator is working in. The orchestrator
will need to copy/merge this file's content into the real
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w5b-c2-unlink.md`
itself. No `git` command was run against the shared checkout, and no source
files were edited anywhere.
