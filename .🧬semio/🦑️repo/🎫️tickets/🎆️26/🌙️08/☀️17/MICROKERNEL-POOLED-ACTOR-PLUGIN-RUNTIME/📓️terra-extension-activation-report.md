# 📓️ terra-extension-activation-report

Packet: `extension-activation`. Owned paths: `🧰️framework/🛍️products/💻️os/🖥️host/**` (install
region), `🧰️framework/🔨️modules/🎠️kernel/**`, plus this ticket folder.

## 🎯️ What was measured before writing anything

- `ActorKind::Extension` exists in `semio-framework-actor` (`🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:299-345`).
  `Kernel::activate` (`…/🎭️actor/🦀️component.rs:2258`) is generic over `ActorKind`, so it already
  accepts `Extension` — but it has **no shard-hint parameter at all** (`pub async fn activate(&mut
  self, package, plugin_ordinal, kind, lane, window, event)`); it always auto-pins via
  `ShardTable::pin`/`pin_avoiding` (least-loaded shard). Grep-confirmed: no `deactivate` method
  exists on `Kernel` anywhere (only `activate`/`submit`/`tick`/`complete`).
- The `.sxt` package format (verify/unpack/pack/content_hash, `extends`, the §4 gate
  `extends_matches_primary_dependency`) already exists and is solid:
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`, mounted into
  `semio-framework-os-kernel` as `crate::os_extension`, re-exported at that crate's root as
  `pub use crate::os_extension as extension;`. **Not owned by me** (path is
  `💻️os/🔨️modules/🧩️extension`, not `💻️os/🖥️host`) — read-only.
  ⚠️ Found but out of scope: that file's own `#[cfg(test)] mod tests` has ~6 calls missing
  `.await` on async fns (`sample_manifest().extends_matches_primary_dependency()` etc.) — R10
  residue, not touched.
- Confirmed **before writing any code**: nothing in `💻️os/🖥️host` (my owned crate,
  `semio-framework-os`) called `store::extension::verify`/`unpack`/`pack` anywhere, and
  `PluginHost` (the one struct in this crate that tracks installed plugins) had zero external
  Rust callers repo-wide (`grep -rn "PluginHost::new\|PluginHost::default\|: PluginHost\b"` — only
  hits inside its own file plus one unrelated `compose/` TS name collision). The mission statement
  ("nothing in `💻️os/🖥️host` ever loads, installs, or activates an `.sxt`") is accurate.

### 🚧️ Concurrent, in-scope-adjacent work discovered mid-session (packet `run-kernel-wiring`)
While reading, `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` and this crate's
`📦️glue.rs`/`Cargo.toml` changed live under me (mtimes minutes apart, new file appeared). That
packet built `NativeKernelRuntime` — a generic `Kernel::activate` + `GuestRuntime::instantiate` +
`ShardExecutor::register` facade, **already accepting any `ActorKind`** including `Extension` —
in this exact owned directory. Its own module doc admits: "this facade cannot be verified by a
green `cargo check` until whichever packet owns [`semio-framework-actor`] finishes" (266 errors
there, an unrelated live async-conversion sweep). I did **not** duplicate this facade; see
"Design decision" below.

## ✅️ What was built

### 1. `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` — new `//#region 🔖️ExtensionActivation`
Pure, decoupled ABI additions (this file is `#[path]`-mounted only into `🛂️manifest/🦀️component.rs`,
itself mounted into exactly 3 crates — `semio-framework`, `semio-framework-graph`,
`semio-s-plugin-stdio`; verified with two independent greps for `#[path.*🎠️kernel` and
`#[path.*🛂️manifest/🦀️component`):

- `ExtensionDescriptor { extension_id, extends, version, content_hash, capabilities:
  Vec<CapabilityId>, capability_requests: Vec<CapabilityRequest> }` — uses only vocabulary this
  file already owns, so it needs no new dependency edge.
- `extensions_extending(plugin_id, installed) -> Vec<&ExtensionDescriptor>` — the exact
  data-driven `extends == plugin_id` query the ruling specifies, no branch on count.
- `scope_capabilities_to_parent(parent_effective, requested) -> Vec<CapabilityId>` — the
  "capabilities scoped to the parent" intersection.
- A `#[cfg(test)] mod extension_activation_tests` with a hand-rolled poll-once `block_on` (tagged
  `🚫️async: E5`, R4 clause 5 — one per crate this file is compiled into), testing both fns
  including a 64-descriptor extends-filter and two capability-scoping cases.

**Verified: `cargo check -p semio-framework --lib` → `EXIT 0`**, clean except pre-existing
`async_fn_in_trait` warnings (R7-sanctioned), 3m09s, `CARGO_TARGET_DIR=…/scratchpad/target-extension-activation`.
Full paste: `sol` can re-run; log kept at the scratchpad path noted below.

`cargo test -p semio-framework --lib` (and the same on `semio-framework-graph`,
`semio-s-plugin-stdio`) could **not** be used to prove the new tests pass at runtime: all three
fail on **pre-existing, unrelated** errors — `semio-framework`: ~29 `#[test] async fn` sites
elsewhere in the crate (the ticket's own documented 769-repo-wide gap) plus a few unrelated E0609
future-field bugs; `semio-framework-graph`: 1,459 pre-existing errors; `semio-s-plugin-stdio`
(pulls `semio-framework-number`): 620 pre-existing errors. None mention my new symbols
(`ExtensionDescriptor`/`extensions_extending`/`scope_capabilities_to_parent`/
`extension_activation_tests`) — grep-confirmed against the full logs.

**Runtime proof instead**: `terra-extension-activation-standalone-verify.rs` (this folder) —
byte-for-byte copy of the two functions, compiled directly with `rustc --edition 2021 -O` (no
workspace graph) and run against a 2,500-descriptor synthetic fixture (50 "plugins" × 50
extensions each, matching the scale fixture's own shape):
```
COMPILE_EXIT=0
ALL ASSERTIONS PASSED: extensions_extending + scope_capabilities_to_parent, 2500-descriptor scale
RUN_EXIT=0
```

### 2. `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` — new `//#region 🔖️ExtensionInstall`
Inside `pub mod host`'s `PluginHost` (feature `os-host-full`):

- `installed_extensions: HashMap<String, InstalledExtension>` field (+ `PluginHost::new()` update).
- `InstalledExtension { manifest: store::extension::ExtensionPackageManifest, content_hash:
  String }` — deliberately the `.sxt`-shaped twin of `kernel::ExtensionDescriptor` above rather
  than a shared type, for the same dependency-edge-law reason `PackagePluginDependency`'s own
  docstring gives (`semio-framework-os-kernel` must never depend on `semio-framework`).
- `ExtensionInstallError { Package(#[from] …), ExtendsMismatch { extension_id, extends, actual } }`.
- `PluginHost::install_extension_package(&mut self, bytes) -> Result<InstalledExtension,
  ExtensionInstallError>` (`async` — it awaits `store::extension::verify`/`content_hash` and
  `ExtensionPackageManifest::extends_matches_primary_dependency`, all genuinely async on disk):
  verifies/unpacks, **re-checks contract freeze §4 at install time** (not just trusting the guest
  SDK's build-time `assert!`), registers keyed by `extension_id`.
- `uninstall_extension_package`, `installed_extension`, `extensions_extending_plugin(plugin_id) ->
  Vec<&InstalledExtension>` (sync — no suspension point; the exact "kernel queries installed
  descriptors for `extends == plugin_id`" query, data-driven over the whole map, zero
  special-casing), `extension_capabilities_scoped_to_parent`.
- 4 new tests in the module's existing `mod tests` block, each building **real** `.sxt` bytes via
  `store::extension::pack` (real zip + real blake3 hash, not a mock): install-and-query across 30
  packages / 3 parent plugins, extends-mismatch rejection, capability scoping, uninstall. One
  test-local `block_on` (tagged E5, R4 clause 5 — separate from `🎠️activation.rs`'s one
  *production* E5 bridge in this same crate, which R2's "at most one per crate" governs
  independently of test code).

### 🧭️ Design decision: why the real `Kernel::activate(…, ActorKind::Extension, …)` call is NOT in this packet's code
Three independent, measured reasons:
1. **`semio-framework-actor` is currently red** (266 errors, an unrelated live async-conversion
   sweep — confirmed independently by `🎠️activation.rs`'s own module doc, written by a different
   concurrent packet). Coupling my install/query code to it would make code that cannot be
   verified even in principle right now.
2. **`run-kernel-wiring`'s `NativeKernelRuntime::activate`** (this exact owned directory, landed
   live during this session) is *already* the generic activation facade — it takes any `ActorKind`
   including `Extension`, mints the `ActorId`, and hands the instance to a `ShardExecutor`. Writing
   a second, parallel `kernel.activate(...)` call site in `component.rs` would duplicate that
   facade rather than compose with it.
3. **Real activation additionally needs a compiled wasm component** (`CompiledHandle`) for the
   extension's `component_wasm` bytes — that compilation step lives in
   `semio-framework-plugin-host` (`GuestRuntimes::compile`, unowned crate).

**The intended composition**, for whichever packet finishes the actor-crate conversion and wires a
live `Kernel` thread (`run-kernel-wiring`/a future `run-through-kernel` packet): on plugin
activation, call `host.extensions_extending_plugin(&package.0)`, then for each result call
`host.extension_capabilities_scoped_to_parent(...)` and feed
`ActorKind::Extension { plugin: package.clone(), extension_id: installed.manifest.extension_id.clone() }`
into `NativeKernelRuntime::activate(...)` (`🎠️activation.rs`, this same crate) exactly the way it
already activates `ActorKind::PluginApp`. No new mechanism needed — the pieces already compose.

## 🚧️ Two honest gaps, not worked around, recommended as `semio-framework-actor` follow-up (not a lease — I never needed to touch that crate to do my own work)
1. **"Pinned to the parent's shard"** — `Kernel::activate`/`ShardTable::pin` have no way to force
   an actor onto a *specific* shard (only "least-loaded" or "least-loaded avoiding a set"). Needs
   a new `ShardTable`/`Kernel` entry point (e.g. `pin_to(actor, shard)` or an `activate_pinned`
   overload) before extension activation can honor this half of the ruling.
2. **Deactivation cascading** — `Kernel` exposes no `deactivate(...)` at all (grep-confirmed: only
   `activate`/`submit`/`tick`/`complete`). There is no primitive to cascade against yet, for
   plugins OR extensions.

Both are inside `semio-framework-actor` (`🎭️actor/**`), outside every packet's currently-listed
`path_scope` I could find — flagging for the coordinator rather than guessing an owner.

## 🚨️ Verification blocker (UNRUN, as instructed when the SDK/actor chain isn't green)
`cargo check -p semio-framework-os --lib` cannot reach my new code today: `semio-framework-os`
depends on `semio-framework-os-kernel`, which is currently red from **unrelated, actively
churning** concurrent edits (confirmed NOT mine by file path in every run below):

| attempt | exit | error count | files (100% unrelated to my paths) |
|---|---|---:|---|
| 1 (baseline, before any of my edits) | 101 | 55 | `os_pack::{DecodeOptions,EncodeOptions}` unresolved (`🎒️pack`) |
| 2 (after my edits) | 101 | 13 | same class, shrinking |
| 3 (`--features os-host-full`) | 101 | 139 | `🏪️store/🔄️sync` missing `.await` (different feature path) |
| 4 | 101 | 11 | `🗣️dsl/📖️grammar` parse errors + `📇️directory` E0728 |
| 5 | 101 | 11 | same |
| 6 | 101 | 10 | same |
| 7 (most recent) | 101 | 20 | same two files, count fluctuating (still churning) |

Every single reported error's `-->` path is `🔨️modules/🗣️dsl/📖️grammar`, `🔨️modules/📇️directory`,
`🔨️modules/🏪️store`, or `🔨️modules/🎒️pack` — never `🖥️host` or `🎠️kernel`. None are files I have
ever edited. The fluctuating, non-monotonic error counts across 7 attempts over this session
(55→13→139→11→11→10→20) are themselves the signature of a live, in-progress, unrelated sweep, not
a stable state — this ticket's own recorded rule 21/`feedback-concurrent-cargo-workspace-churn.md`.

**`semio-framework-os --lib` (my crate) is therefore reported UNRUN, blocked upstream, not by my
own code.** My kernel-file addition (the code sharing the higher-risk cross-crate mount) IS fully
verified (`semio-framework --lib` EXIT 0). My host-file addition is hand-verified against the real
`store::extension` API (read directly from source, every field/signature matched exactly — see
"What was built" above) plus a standalone runtime proof of the identical filter/intersection
algorithm at 2,500-descriptor scale; the one thing NOT independently proven is that `rustc` accepts
the exact byte sequence in `component.rs` — recommend `sol` re-run
`cargo check -p semio-framework-os --lib` once the `dsl`/`directory`/`pack` churn settles.

## 📁️ Files touched
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` — new region, additive only, verified green.
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` — new region + 4 tests + 1 struct field,
  additive only, blocked-upstream (see above).
- Ticket folder (scratch, this packet): `terra-extension-activation-standalone-verify.rs`,
  `terra-extension-activation-standalone-verify` (compiled binary, scratchpad — not committed),
  this report.
- **Not touched**: any `Cargo.toml` (never needed one — the design decision above avoided adding
  the `semio-framework-actor` dependency to `semio-framework-os`, sidestepping that whole
  currently-red crate), `launch.json`, `project.json`, any file outside the two owned paths.

## 📣️ What a sibling or the coordinator must know
1. **`run-kernel-wiring`'s `NativeKernelRuntime::activate`** (`🎠️activation.rs`, same directory
   I own) is the composition point for wiring extension activation for real — see "Design
   decision" above for the exact call shape. No new packet needed to *design* that wiring; it
   needs `semio-framework-actor` green plus a live `Kernel` thread first.
2. **Two `semio-framework-actor` gaps block full correctness** regardless of who wires the call:
   no shard-pin-to-parent primitive, no `deactivate` at all. Named above; recommend routing to
   whichever packet owns `🎭️actor/**` next.
3. **`semio-framework-os-kernel --lib` is red right now** (confirmed 7×, non-mine, still
   fluctuating) — anyone else depending on `semio-framework-os` today will see the same block.
4. **Out-of-scope defect found**: `🧩️extension/🦀️component.rs`'s own `#[cfg(test)]` block has ~6
   missing-`.await` sites (R10 residue) — not fixed (outside owned paths), flagged here per the
   "cross-packet findings must be lifted the moment they are read" rule.
