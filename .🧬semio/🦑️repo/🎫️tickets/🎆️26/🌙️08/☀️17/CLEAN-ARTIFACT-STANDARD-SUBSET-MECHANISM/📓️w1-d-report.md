# 📓️ W1-D report — WIT & host agent (Tasks 1-4)

Agent: W1-D WIT & host agent. Boundary (ONLY writer): `…/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`,
`…/🔌️plugin/🖥️host/🦀️component.rs` (`IoRouter`, host-import impls), the guest-export region in
`…/🔌️plugin/🦀️component.rs`, `🎠️kernel/🟦️component.ts`, `📦️packages/🟦️typescript/🟦️glue.ts` (re-exports
only — turned out to need zero edits, see below). Built on W1-A's `io_mechanism`/`io_schema` (read,
not touched) and W1-C's declaration tree (read, not touched).

## Task 1 — the new WIT surface, verbatim

One real discovery forced a deviation from the ticket's literal signatures: **`from` is a reserved
WIT keyword** (`wit-parser` rejects it — confirmed by a real compile failure, see below). Every new
function uses `source`/`target` instead of `from`/`into` on the wire; the Rust/TS sides keep calling
the concept `from`/`into` internally since Rust trait impls and TS functions don't have to match a
trait/WIT's own parameter names, only positional types.

`interface plugin` (guest exports), appended after `artifact-compose`:
```wit
list-io-entries: func() -> result<list<u8>, plugin-error>;
io-run: func(source: string, target: string, payload: list<u8>) -> result<list<u8>, plugin-error>;
io-sniff: func(source: string, target: string, payload: list<u8>) -> result<u8, plugin-error>;
```

`interface host`, appended after `resolve-artifact-link`:
```wit
io-routes: func(source: string, target: string) -> result<list<u8>, list<u8>>;
io-run: func(source: string, target: string, payload: list<u8>) -> result<list<u8>, list<u8>>;
io-identify: func(payload: list<u8>) -> result<list<u8>, list<u8>>;
```

`resolve-artifact-link` untouched. Old names (`migrate-artifact`, `list-artifact-dialects`,
`artifact-compose`, `io-dialects`, `io-compose`) all kept, byte-identical (D3).

## Payload wire encoding — chosen: JSON

`from`/`into` coordinate strings and `list-io-entries`/`io-routes` descriptors/routes are JSON per
the ticket's own literal spec. For the `payload: list<u8>` slots (`io-run`×2, `io-sniff`,
`io-identify`), I evaluated the two options the ticket named:
- **Pack wire form** (`dsl::to_dsl_value` + `store::pack_rt::encode_wire_value`) — the file's own
  idiom for `manifest`/`read-artifact`/`write-artifact`. Rejected: `dsl::DslValue` has no `Bytes`
  variant (`Null|Bool|Number|String|Array|Object`), so a plain `Vec<u8>` field (`IoPayload::
  Binary(Vec<u8>)`, un-annotated, not `serde_bytes`) would serialize through `serialize_seq` into a
  `DslValue::Array` of one `Number` per byte — not meaningfully more compact than JSON, adds a whole
  extra encode/decode dependency, and I cannot add a `Bytes` variant to `DslValue` myself (out of
  boundary, `🗣️dsl/**`).
- **JSON** (`serde_json::to_vec`/`from_slice` on `io_schema::IoPayload`) — chosen. Same file's
  EXISTING `WireComposeSource`/`WireComposedArtifact` (old mechanism, `🚪️io/component.rs` lines
  ~1311-1324) already carry the OLD `IoPayload` (same 2-variant `Text(String)/Binary(Vec<u8>)` shape)
  as JSON across this EXACT WIT interface's `artifact-compose`/`io-compose` — so this is not a new
  inefficiency I introduced, it is the file's own established precedent for this exact family of
  functions, and picking a DIFFERENT encoding just for the 3 new payload-carrying functions would be
  the actual inconsistency. Flagged as `openQuestions` #1 below — a real `Vec<u8>`-as-JSON-array
  blowup exists for large binary artifacts and is worth revisiting once `DslValue` grows a `Bytes`
  variant or a genuine binary framing is designed, but that is out of this wave's boundary.

Guest `io-run`: parses `source`/`target` via `ArtifactDialect::parse_coordinate`, JSON-decodes
`payload` into `io_schema::IoPayload`, looks up the ONE matching `io_mechanism::io_entries()`
descriptor, builds a 1-hop `IoRoute` from it, calls `io_mechanism::io_run` — **never** calls a host
import (single hop only, per Task 2; multi-hop is exclusively the host's job).
Guest `io-sniff`: implemented via `io_mechanism::io_identify` (carrier-scoped by construction, per
the payload law) filtered to `target`; a non-carrier `source` naturally yields `Confidence::None`/`0`
without a new `io_mechanism` API. `list-io-entries`: `serde_json::to_vec(&io_mechanism::io_entries())`.

## Reentrancy guard — exact mechanism

`IoRouter::run_io` (host, `…/🔌️plugin/🖥️host/🦀️component.rs`): resolves the route, THEN — before
executing ANY hop — scans the WHOLE resolved route via a pure helper, `route_reenters_calling_plugin`,
which returns the first hop (if any) owned by `calling_plugin_id`. If found, the ENTIRE call is
refused with no partial execution; only if the WHOLE route is clean does it look up runtimes and
start executing hops. This generalizes `compose`'s existing one-hop self-route refusal (`owner ==
calling_plugin_id` → refuse) to a route of up to 3 hops, and is strictly stronger than a per-hop
inline check (my first draft) because a per-hop check could execute hop 1 successfully before
discovering hop 3 is self-owned — partial execution the doc comment explicitly promises never
happens. `IoRouter::identify` uses the opposite policy for the SAME hazard: it **skips** (not
refuses) the calling plugin's own carrier entries during fan-out, since identify is inherently
best-effort across multiple plugins, not an atomic call.

TS mirror (`ioRun` in `🎠️kernel/🟦️component.ts`): computes every hop's owner via `.map()` BEFORE the
execution loop starts, throwing on the first self-owned hop — same up-front, no-partial-execution
shape. `ioIdentify` filters `callingPluginId` out of the candidate set before fanning out.

## Determinism across load order — guarantee + tests

The host's merged NEW graph (`IoRouterState.io_entries`) is a `BTreeMap<(ArtifactDialect,
ArtifactDialect), IoEntryRoute>` — canonical key order regardless of insertion order — and
`resolve_io_route` (host twin of `io_mechanism::resolve_route`) enumerates every cycle-free simple
path up to `max_hops.min(3)` into a full candidate list, THEN sorts that ENTIRE list once by
`(Reverse(min hop fidelity rank), hop count, joined into-coordinate string)` — never short-circuiting
on the first hit. The winning route is therefore a pure function of the (from,into) KEY SET and
VALUES, never of which plugin registered first. TS `IoEntryGraph.route` mirrors this exactly (full
candidate collection, one final `.sort()`).

**Fixture** (shared by every proof below — two mock plugins): `"stdio"` owns
`s.stdio.binary@raw/*` → `s.stdio.gif@87a/*` at `Exact` fidelity; `"gif"` owns TWO hops —
`s.stdio.gif@87a/*` → `s.stdio.gif@89a/*` at `Canonical` (the real migration), AND a competing
DIRECT `s.stdio.binary@raw/*` → `s.stdio.gif@89a/*` shortcut at `Lossy` — deliberately weaker, so the
"prefers higher minimum fidelity over fewer hops" rule has something genuine to beat: the 2-hop
`stdio→gif` path (min fidelity `Canonical`) must win over the 1-hop shortcut (`Lossy`) despite having
more hops.

**Rust proof** — `#[cfg(test)] mod tests`, region `🔖️IoRouterW1d`
(`…/🔌️plugin/🖥️host/🦀️component.rs`), pure — no `Arc<WasmPluginRuntime>`/wasm needed:
- `io_router_route_is_deterministic_across_load_order` — registers the fixture forward AND reversed,
  asserts the merged graphs AND resolved routes are byte-identical, and that the winner is the 2-hop
  path (not the 1-hop shortcut). **This is the literal "register two mock plugins in both orders"
  proof the ticket asked for.**
- `io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops`
- `io_router_route_respects_max_hops` (bounded to 1 → picks the direct Lossy shortcut)
- `io_router_run_io_reentrancy_guard_predicate` (exercises `route_reenters_calling_plugin` directly)
- `io_router_register_plugin_rejects_conflicting_io_entry_ownership` (exercises `io_entries_conflict`
  directly — the SAME function `register_plugin` calls)

**These 5 tests compile cleanly** (`cargo check -p semio-framework-plugin-host --lib --tests` shows
zero errors/warnings from any line I added) but **could not be executed via `cargo nextest run -p
semio-framework-plugin-host`** — that crate-wide test compile is blocked by a PRE-EXISTING,
out-of-boundary error (`DefaultApp` not imported in `🎚️config/🧬️schema/🧬️mutations/🦀️component.rs`),
confirmed present at this ticket's own start commit via `git show
101a6b4ea83acc82d6fbdc0607e6ae5d876825ae:…/🧬️mutations/🦀️component.rs` (identical broken test body
already there). Patch prepared, not applied (out of boundary) — see `## sharedFileRequests`.

**To get genuine runtime evidence anyway** (never claiming a test passes without running it), I
copied the exact algorithm (`resolve_io_route`/`walk_io_routes`/`io_route_rank`/
`rank_to_io_fidelity`/`route_reenters_calling_plugin`/`io_entries_conflict`, byte-for-byte) plus
minimal standalone `ArtifactDialect`/`IoFidelity` twins into a self-contained script,
`🧪️w1d-io-router-algorithm-check.rs` (this folder), compiled with plain `rustc`, and ran it — **13/13
checks pass**, output captured in `🧪️w1d-io-router-algorithm-check-rust-output.txt`.

**TS proof** — `🧪️w1d-io-router-parity.ts` (this folder, mirrors the surface ticket's
`🧪️w1-d-parity.ts` mechanism exactly: a standalone `bun run` script, not vitest, asserting the SAME
fixture through the real shipped `IoEntryGraph`/`ioRun`). Real output, `🧪️w1d-io-router-parity-ts-output.txt`:
```
[ok] resolved route identical regardless of registration order
[ok] winning route is the 2-hop path, not the 1-hop lossy shortcut
[ok] route fidelity is Canonical (min of Exact,Canonical)
[ok] first hop starts at the binary carrier
[ok] last hop ends at gif89a
[ok] bounded to 1 hop picks the direct lossy shortcut
[ok] a calling plugin owning neither hop runs both hops in order
[ok] ioRun returns the final hop's payload
[ok] ioRun refuses the WHOLE route when the caller owns the first hop (stdio)
[ok] no hop ran before the refusal (no partial execution)
[ok] ioRun refuses the WHOLE route when the caller owns the second hop (gif)

All checks passed
```
I also added a real `describe("IoEntryGraph", …)` `import.meta.vitest` block in `🎠️kernel/🟦️component.ts`
(region `🧪️IoRouterTests`) for ongoing regression coverage — but discovered it (and the PRE-EXISTING
`🧪️ExpandPluginRegistryTests` block in the SAME file) never actually runs: `🧪️vitest.config.ts`'s
`include`/`includeSource` name ONLY `🟦️glue.ts`, so `component.ts`'s in-source tests, imported only
via `export *`, are never scanned. Confirmed via `bun ./📜️script.ts test --reporter=verbose` — neither
`describe("expandPluginRegistry"` nor `describe("IoEntryGraph"` appears anywhere in the verbose test
list, and the total stays "158 passed (158)" identically before and after my edit. Pre-existing
(commit `a5cc4dd9ab`, 2026-08-07, a week before this ticket) — patch prepared, not applied (config
file, unclaimed by anyone's boundary). The `.ts` block is real, correct code, dormant until that
patch lands; the STANDALONE parity script above is what actually proves TS↔Rust parity today, matching
the surface ticket's own precedent 1:1.

## Host `IoRouter` extension (Task 3)

`IoRouterState` gained `io_entries: BTreeMap<(ArtifactDialect, ArtifactDialect), IoEntryRoute>`
(`IoEntryRoute { owner, fidelity, sniffs }`) alongside the untouched OLD `routes`/`runtimes` fields.
`register_plugin` now ALSO calls the new `WasmPluginRuntime::list_io_entries()` (WIT `list-io-entries`)
and preflights BOTH graphs' conflicts before committing either — one all-or-nothing registration, not
two independent ones. `unregister_plugin` drops `io_entries` rows too. New `PluginHostError::
IoEntryRouteConflict` variant, separate from the OLD `IoRouteConflict` (different key shape).
`IoRouter::io_routes`/`run_io`/`identify` implement the three host imports; `WasmPluginRuntime::
list_io_entries`/`io_run`/`io_sniff` call the three new guest exports (mirroring `list_artifact_dialects`/
`artifact_compose`'s existing call idiom exactly — `store_guard`/`bindings_guard`/`prepare_call`/
`plugin_result`). `HostState`'s `Host` trait impl wires the three; `ExtensionHostState` gets the same
"not implemented for extension host" stub shape every other capability there already has.

## Task 4 — TS parity, what changed in each file

- `🎠️kernel/🟦️component.ts` (+341 lines): new region `🔖️IoRouter` — `IoFidelity`/`IoConfidence`/
  `IoEntryDescriptor`/`IoRoute`/`IoEntryGraphPlugin` types, `CARRIER_BINARY_DIALECT`/
  `CARRIER_TEXT_DIALECT` consts, `IoEntryGraph` class (`.build`/`.route`/`.ownerOf`/`.carrierEntries`),
  `ioRun`/`ioIdentify` functions (DI-injected hop runner — this domain-neutral framework module never
  calls a plugin worker itself, same boundary `AppRouter`/`ArtifactMutationRouter` already draw); plus
  region `🧪️IoRouterTests` (dormant pending the vitest.config patch, see above). Reused the EXISTING
  `ArtifactDialect`/`dialectCoordinate`/`dialectEquals` from the `🔖️AppRouter` region rather than
  redefining them.
- `📦️packages/🟦️typescript/🟦️glue.ts` — **zero edits needed**. It already does `export * from
  "../../🔨️modules/🎠️kernel/🟦️component.ts";`, and none of my new names (`IoEntryGraph`, `ioRun`,
  `ioIdentify`, `IoFidelity`, `IoConfidence`, `IoEntryDescriptor`, `IoRoute`, `CARRIER_*_DIALECT`,
  `IoHopRunner`, `IoSniffRunner`) collide with anything else `glue.ts` re-exports (checked by grep
  across every module it wildcards from). This is genuinely additive through an existing wildcard, not
  a silently-skipped task.

## Guest export bodies (Task 2)

`…/🔌️plugin/🦀️component.rs`, `impl Guest for ComponentGuest` (+73 lines): `list_io_entries`/`io_run`/
`io_sniff` added right after `artifact_compose`, using fully-qualified `semio_framework::io_schema::*`/
`semio_framework::io::io_mechanism::*` paths (never a bare `use`, avoiding any collision with the
OLD `io::IoPayload`/`Confidence` re-exports the SAME crate's `app` module already carries — same
separation discipline W1-A/W1-C established for this exact old/new naming clash). Also added
`host_io_routes`/`host_io_run`/`host_io_identify` guest-side wrappers (mirroring `host_io_dialects`/
`host_io_compose`) — the seam design.md §3 names explicitly ("`host_io_run` in Rust, `ioRun` in TS").
`plugin_exports!`/`component_export_anchor` needed NO changes — that macro only wires the link-shim
anchor, independent of which guest exports exist.

## verification

All commands from `/Users/ueli/Documents/semio`, `CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/🎯️target"`.

- `cargo check -p semio-framework-plugin-host --lib` → **clean, 0 errors, 0 warnings** (fixed 2
  `unused-qualifications` warnings my own `use std::collections::BTreeSet` import exposed on
  PRE-EXISTING `std::collections::BTreeSet` spellouts a few regions away — not new debt, cleaned up
  since I caused them).
- `cargo check -p semio-framework-plugin-host --lib --tests` → **2 errors**, both
  `E0422 cannot find struct DefaultApp` in `🎚️config/🧬️schema/🧬️mutations/🦀️component.rs` — PRE-EXISTING
  (proven via `git show` at the ticket's own start commit, see above), zero errors/warnings trace to
  any line I added. Patch prepared: `🔧️patches/w1d-opening-config-mutations-missing-default-app-import.txt`.
- `cargo check -p semio-framework-plugin --lib --tests` (native) → clean, only the SAME
  `derive_artifact_facets!`/`subset!` warnings W1-A's baseline already named — zero new warnings.
- `cargo check -p semio-framework-plugin --target wasm32-wasip2` (bare, no `--features`) → clean, 2
  warnings — but this is a **FALSE POSITIVE for verifying my own guest export bodies**: the SDK
  crate's `Cargo.toml` has `default = []`, `component-guest = []` — the guest `pub mod component {
  ... impl Guest for ComponentGuest { fn list_io_entries()/io_run()/io_sniff() ... } }` block I added
  to is gated `#[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]`,
  so this bare command silently SKIPS the entire block (feature off by default) — it never actually
  compiled my new guest functions. I caught this by re-running WITH the feature explicitly:
  `cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest` →
  **still 0 errors**, now 12 warnings (10 more than the bare run, because the guest module is now
  genuinely compiled). Checked every new warning's line number against my edits: all 12 are
  PRE-EXISTING — 2 are W1-C's already-documented `child_slots`/`link_slots`/`PluginRuntimeRegistry`
  dead-code warnings; the other 10 (`unnecessary qualification` ×5, `function cannot return without
  recursing` ×5, at lines ~19401-19448, `host_port::host_backbone_send`/`host_backbone_poll`/
  `host_backbone_status`/`host_now_ms`/`host_read_asset`) are a DIFFERENT, previously-undiscovered
  pre-existing issue — confirmed via `git show HEAD:…/🔌️plugin/🦀️component.rs | grep -n "pub fn
  host_backbone_send"` showing the identical duplicate-name pattern already present before I touched
  anything (this crate has apparently never been checked with `--features component-guest` explicitly
  before, so this `unconditional_recursion` lint was simply never triggered). Zero of the 12 warnings'
  line numbers fall anywhere near my additions (guest exports ~lines 58-120, `host_io_*` wrappers
  ~lines 270-290). **This is the REAL, load-bearing verification** — the earlier bare-command result
  is misleading and should not be quoted as proof my guest code compiles for wasm32-wasip2.
- `cargo nextest run -p semio-framework-plugin --lib --no-fail-fast` → **230 tests run: 226 passed, 4
  failed, 0 skipped** — IDENTICAL to the ticket's own stated baseline (230/226/4, W1-C's number), same
  4 named failures (`artifact_definition_contract_tests` ×3, `plugin_builder_contract_tests::
  merge_channel_commands_…` ×1). Zero net new tests in THIS crate (my new tests live in the HOST
  crate, which is a separate package) — not made worse, matches exactly.
- `🧪️w1d-io-router-algorithm-check.rs` (standalone, `rustc` compiled) → **13/13 checks pass**, real
  output in `🧪️w1d-io-router-algorithm-check-rust-output.txt`.
- `bun run 🧪️w1d-io-router-parity.ts` → **11/11 checks pass**, real output in
  `🧪️w1d-io-router-parity-ts-output.txt`.
- `bun nx run @semio-tech/framework:test --skip-nx-cache` (the kernel's own test command, found via
  `📦️packages/🟦️typescript/package.json`'s `"test"` script) → **Test Files 2 passed (2), Tests 158
  passed (158)** — unchanged before/after my edit (my new `describe("IoEntryGraph", …)` block does not
  currently execute, see the vitest.config gap above; not a regression, a pre-existing dormant-test
  condition this wave discovered and patched but did not fix).
- `bunx tsc --noEmit -p tsconfig.json` (whole-repo typecheck) → 19 errors, ALL in files I never
  touched (`🔱️trinity/…/🧠️lsp/🟦️component.ts`, two `🗄️stdio` artifact schema files, one vscode
  extension file) — **zero errors trace to `🎠️kernel/🟦️component.ts` or `📦️packages/🟦️typescript/🟦️glue.ts`**.

No WIT-bindings regeneration command exists separately from compilation — `wasmtime::component::
bindgen!` (host) and `wit_bindgen::generate!` (guest) are compile-time proc macros reading
`📜️world.wit` directly off disk; `cargo check`/`cargo nextest run` above ARE the regeneration+verification
step. No generated files were hand-edited.

## sharedFileRequests

1. `🔧️patches/w1d-opening-config-mutations-missing-default-app-import.txt` — one-line missing
   `DefaultApp` import in `🎚️config/🧬️schema/🧬️mutations/🦀️component.rs` (out of my boundary),
   PRE-EXISTING at the ticket's start commit, blocks `cargo nextest run -p semio-framework-plugin-host`
   for anyone (not just this wave's new tests).
2. `🔧️patches/w1d-framework-vitest-config-missing-kernel-includesource.txt` — widens
   `🧪️vitest.config.ts`'s `include`/`includeSource` so `🎠️kernel/🟦️component.ts`'s in-source
   `import.meta.vitest` blocks (mine AND the pre-existing `expandPluginRegistry` one) actually run
   under `bun nx run @semio-tech/framework:test`. Config file, unclaimed by any wave's named boundary.

## openQuestions

1. **JSON payload encoding's `Vec<u8>`-as-JSON-array blowup for large binary artifacts** is real
   (documented above, chosen for consistency with this exact WIT interface's existing precedent, not
   because it is optimal). A later wave that wires `io-run`/`io-identify` to real multi-megabyte
   PDF/GIF/video payloads should reconsider once `io_schema::DslValue` (out of my boundary) grows a
   native `Bytes` variant, or design a dedicated binary framing for `IoPayload` specifically.
2. **Two pre-existing, unrelated blockers discovered and patched, not fixed** (see
   `sharedFileRequests`) — both proven pre-existing via `git log`/`git show`, neither touched by this
   wave's edits.
3. **`IoRouter::identify`'s fan-out is O(carrier entries) sequential guest calls**, each acquiring and
   releasing that plugin's own store mutex in turn (never held across the fan-out) — no batching or
   parallelism attempted, matching this file's existing single-threaded call idiom throughout
   (`compose`/`dialects` are the same shape). Flagging in case a later wave with many carrier-sniffing
   plugins wants to revisit for latency.
4. **The real end-to-end wasm path** (an actual multi-plugin `IoRouter::run_io` crossing TWO real
   loaded `.wasm` components, mirroring `io_router_routes_a_real_cross_plugin_compose_between_two_
   loaded_wasm_plugins` in the `🔖️IoRouterE2e` region) was not attempted — no two real subsets
   registering NEW-mechanism `IoEntry`s exist yet anywhere in the repo (stdio's carrier pilot, W2-P,
   built the subsets but did not register through `declare_artifact`/`io_register` — see
   `📓️recipe-subset.md`). This is expected: W1-D is framework plumbing, the first REAL cross-plugin
   `io-run` only becomes possible once W2 cuts a real plugin over.
5. **A previously-undetected pre-existing warning family**, discovered only because I re-verified with
   `--features component-guest` explicitly (see `## verification`): `host_port::host_backbone_send`/
   `host_backbone_poll`/`host_backbone_status`/`host_now_ms`/`host_read_asset`
   (`…/🔌️plugin/🦀️component.rs`, lines ~19396-19452) trigger `unconditional_recursion` +
   `unnecessary_qualifications` lints under that feature — 10 warnings total, confirmed pre-existing
   via `git show HEAD:…`, zero relation to io/WIT. Likely never surfaced because nothing in this
   crate's own CI/verification history checks `--features component-guest` explicitly (the bare
   `--target wasm32-wasip2` command silently skips the whole guest module, feature off by default —
   also flagged above). Relevant to `26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-
   TARGETS` (a live peer ticket per `📓️status.md`'s "Known live peer sessions" table) — not fixed
   here, out of scope for W1-D, but worth that ticket's attention since it is EXACTLY their mandate
   and this combination of flags may not be in their sweep yet.
