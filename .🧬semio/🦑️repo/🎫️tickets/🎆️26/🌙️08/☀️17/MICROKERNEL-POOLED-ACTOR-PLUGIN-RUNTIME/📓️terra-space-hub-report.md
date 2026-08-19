# 📓️ terra — dyn-space-hub report

Packet: `dyn-space-hub`. Owned families: `✏️s/🔌️plugins/🪐️space`'s `OsBackbonePort` (4) +
`SpaceBackbonePort` (1), `🌎️hub`'s `HubDirectory` (7) + `HostAsyncRuntime` (1) + its 4
`#[async_trait]` sites, `🧰️framework/🛍️products/🦑️repo`'s `AgentRunner` (3) + `DashboardTransport` (2).

## 0. Headline finding — this packet's scope was already completed by two prior packets

Before making any edit I read the ticket folder for existing reports covering these paths and found
two that, between them, already cover 100% of this packet's brief, done under different packet slugs
by (per the report bylines) the same "terra" identity, both already **committed** to `HEAD`:

- `📓️terra-dedyn-fleet-space-report.md` (packet `dedyn-fleet-space`, committed 2026-08-19 19:23:33,
  commit `5e7b8046be`) — covers `✏️s/🔌️plugins/🪐️space`'s `OsBackbonePort`/`SpaceBackbonePort`.
- `📓️terra-dedyn-fw-hub-repo-report.md` (packet `dedyn-fw-hub-repo`, committed 2026-08-18 20:18:05,
  commit `f69271685f`) — covers `🌎️hub`'s `HubDirectory`/`HostAsyncRuntime` and `🦑️repo`'s
  `AgentRunner`/`DashboardTransport`, including all 4 `#[async_trait]` removals.

Rather than redo completed work or invent a second mechanism (against the ticket's own repeated
instruction), I **re-verified every claim in both reports against the current disk state** — freshly,
with two differently-implemented queries per family (python3 regex scanner + `grep -rn`, both with
comment-lines excluded), exactly as rule 21 requires before trusting any negative result — and report
the outcome below. No production file was edited by this packet.

## 1. Verified counts (fresh, this session, both tools agree)

| family | path | starting (per brief) | current (verified) |
|---|---|---:|---:|
| `OsBackbonePort` | `✏️s/🔌️plugins/🪐️space` | 4 | **4**, all documented residue (see §2) |
| `SpaceBackbonePort` | `✏️s/🔌️plugins/🪐️space` | 1 | **1**, documented residue (see §2) |
| `HubDirectory` | `🌎️hub` | 7 | **0** |
| `HostAsyncRuntime` | `🌎️hub` | 1 | **0** |
| `#[async_trait]` sites | `🌎️hub/📇️directory/` | 4 | **0** |
| `async-trait` Cargo.toml deps | `🌎️hub/**` | 1 | **0** |
| `AgentRunner` | `🧰️framework/🛍️products/🦑️repo` | 3 | **0** |
| `DashboardTransport` | `🧰️framework/🛍️products/🦑️repo` | 2 | **0** |

`🌎️hub` and `🧰️framework/🛍️products/🦑️repo`: **zero first-party `dyn` anywhere in either tree** (not
just the named families — full-tree scan, 179 `.rs` files across all three owned roots). Only
R1-permitted std hits found: `Box<dyn FnOnce() + Send>` in `HubDbRuntime::run_blocking`
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`).

`✏️s/🔌️plugins/🪐️space`: the 5 remaining `dyn` occurrences (lines 138, 139, 165, 183, 368 of
`🦀️component.rs`) are exactly the ones `dedyn-fleet-space` left as **documented, evidenced,
lease-requested residue** — not oversights. See §2.

## 2. The 🪐️space residue — re-verified as a genuine cross-packet blocker, still live

I independently re-checked both root causes `dedyn-fleet-space` identified, by reading the actual
blocking signatures on current disk (not trusting the prior report's line numbers blindly):

- `open_folder_space_backbone`/`open_file_space_backbone` in
  `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:1980,1985` **still** return
  `Result<Arc<dyn OsBackbonePort>, VcsError>` directly — confirmed by direct grep today. These feed
  `shared_studio_ports`/`register_studio_port`/`register_studio_port_for_test` (4 residue sites); the
  producer erases the type before `🪐️space` ever sees it, and `OsBackbonePort` carries no `Any` bound
  to downcast through.
- `draft_catalog_for` in `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:1525` **still**
  takes `port: &Arc<dyn SpaceBackbonePort>` **by reference** (confirmed today) — `&Arc<Concrete>` does
  not coerce to `&Arc<dyn Trait>` (verified against real rustc by the prior packet,
  `terra-dedyn-space-coerce-probe.rs.txt` in this folder), so `draft_backbone_port`'s return type stays
  pinned to the dyn shape regardless of what it constructs internally.

Both files are **out of this packet's owned path** (`🖥️host/**` and
`🔨️modules/🪐️space/**` are framework, not `✏️s/🔌️plugins/🪐️space/**`) — editing them would violate the
binding rule against touching another packet's files to unblock yourself. The two lease-requests
`dedyn-fleet-space` already filed (§5 of that report) are **still accurate and still unresolved**; I am
not re-filing duplicates, just confirming they still apply. Per this packet's own brief: *"if your side
needs the framework side to change first, emit a lease-request and say so rather than improvising a
second mechanism"* — the mechanism and the lease-requests already exist; improvising a second one (e.g.
an `Any`-downcast hack, or duplicating `store::BackbonePorts` locally) would be exactly the wrong move.

## 3. Mechanism per family (for the record, as documented by the original packets)

- **`OsBackbonePort`/`SpaceBackbonePort`** (🪐️space): closed set, enum already built by `store-dedyn`
  (`store::BackbonePorts`) — reused directly via the framework's own blanket
  `impl<T: store::BackbonePort> {Os,Space}BackbonePort for T`. No `dyn_enum_close!` needed (neither
  trait is owned by this packet). 8 of 13 original sites converted; 5 are architectural residue (§2).
- **`HubDirectory`** (🌎️hub): closed set of 3 cfg-gated backends
  (sqlite/postgres/neo4j) — hand-written `HubDirectories` enum (not `dyn_enum_close!`, whose DSL cannot
  express per-variant `#[cfg]`) + match-delegating `impl HubDirectory for HubDirectories`. All 4
  `#[async_trait]` sites removed (R8), `async-trait` dependency dropped, `#![allow(async_fn_in_trait)]`
  added per R7.
- **`HostAsyncRuntime`** (🌎️hub, 1 use): open extension point by design — generic parameter, matching
  the shape `os-hostasync` already landed elsewhere (`FsStorage<R: HostAsyncRuntime>` etc.); the hub's
  one use became `Arc<HubDbRuntime>` (concrete), inference-driven, no enum.
- **`AgentRunner`** (🦑️repo, 3 uses): closed set of 3 production runners — `dyn_enum_close!` +
  `AgentRunners` enum, with the `available_runners` helper kept generic (never needs heterogeneity,
  avoided the same per-variant-cfg wall `HubDirectory` hit for its test-only 4th impl).
- **`DashboardTransport`** (🦑️repo, 2 uses): looked like "exactly one impl" but a same-file test mock
  proved a real second implementor live in the test suite — corrected to R11's open-set answer,
  `Supervisor<T: Write + Send>` generic, trait+blanket-impl deleted outright.

Full mechanism detail, macro-friction findings, and rustc-verified standalone probes for each family
are in the two source reports (§0) — not re-derived here to avoid a stale duplicate.

## 4. Build attempt (fresh this session)

```
$ CARGO_TARGET_DIR=<scratchpad>/target-dyn-spacehub cargo check -p semio-hub --lib
...
error: could not compile `semio-framework-os-kernel-db` (lib) due to 281 previous errors; 36 warnings emitted
exit=101
```
Full output: `terra-space-hub-check-semio-hub.txt` in this folder. `grep -c "semio-hub\|🌎️hub"` on the
full log: **0** — the failure is entirely inside `semio-framework-os-kernel-db` (not our path,
mid-flight-broken by unrelated in-progress async-trait-flip work), never reaches `semio-hub`'s own
source. This is the **identical** blocker (same crate, same 281-error count) `dedyn-fw-hub-repo`
already reported yesterday — confirms nothing has regressed and nothing has been fixed upstream either;
not re-run for `semio-s-plugin-space` / `semio-framework-repo-cli` since both were already reported
blocked by different, also-still-live upstream churn (`semio-framework-os-kernel`,
`semio-framework-ui`) and re-confirming a second identical-shape block would not change this packet's
conclusion. Reporting acceptance **UNRUN** for the same reason both source packets did: the blocking
crates are outside this packet's path and mid-flight elsewhere.

## 5. Lease-requests

None new. The two filed by `dedyn-fleet-space` (§5 of `📓️terra-dedyn-fleet-space-report.md`) remain
open and unresolved — re-verified still-applicable in §2 above:

1. Owner of `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`: make
   `open_folder_space_backbone`/`open_file_space_backbone` return a concrete/enum type instead of
   `Arc<dyn OsBackbonePort>`.
2. Owner of `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`: make `draft_catalog_for`/
   `DraftCatalog::list_drafts_sweeping_expired`/`DraftCatalog::discard_draft` take their port by value
   or generically, instead of `&Arc<dyn SpaceBackbonePort>`.

## 6. What a sibling or the coordinator should know

- This packet's entire brief was already delivered by `dedyn-fleet-space` + `dedyn-fw-hub-repo`; no
  production edit was needed or made. Treat this report as the **verification pass**, not new work.
- The 5-site `🪐️space` residue is real and durable — it will not close until one of the two
  lease-requests above lands. It is not this packet's (or `dedyn-fleet-space`'s) fault.
- `semio-framework-os-kernel-db` is still red with 281 errors, unchanged from `dedyn-fw-hub-repo`'s
  report yesterday — still blocking any build that depends on it, `semio-hub` included. Worth a
  coordinator nudge if that crate's owning packet has stalled.
- Repo-wide sanity check turned up `dyn HostAsyncRuntime` uses in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` (7 sites) — **not**
  this packet's family (that file is `🔌️plugin/**`, not `🌎️hub/**` or `🦑️repo/**`, and was never named
  in the brief) and not touched. Flagging in case no packet currently owns it.

## 7. Files touched

None (production). Ticket-folder additions: this report, `terra-space-hub-check-semio-hub.txt`.
