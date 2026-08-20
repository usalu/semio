# 📓️ terra-stdio-finish — report

**Verdict: TOTAL, external, out-of-scope blocker. `semio-s-plugin-stdio` cannot be measured or
reduced this packet — the crate never reaches its own compilation units.** Zero production edits
made this packet (read-only `cargo check`/`git`/`grep` only). Not caused by `stdio-await` or
`replication-r9` (the two packets immediately preceding this one on this ticket) — root cause is a
**live, uncommitted edit in a completely different ticket**
(`26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet `wit-flip`), landed in the window
between `replication-r9`'s report and this packet starting.

Per this ticket's own standing rule: *"If a fix needs a file outside your scope, STOP and report
it."* This report is that stop.

## The blocker, reproduced 3 times with real exit codes

`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (outside `path_scope` — not `✏️s/🔌️plugins/🗄️stdio/**`)
carries this **live, uncommitted** edit (`git status`: `M`, 14 insertions / 24 deletions, file mtime
2026-08-20 12:19:23 — same-day, same-session as this packet):

```diff
-#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
-pub struct UiPatch { ... }
-pub enum PatchOp { ... }
+/// Requires a `semio-framework-ui-contract` dependency on every crate that `#[path]`-mounts this
+/// file — see this packet's report for the exact registrar-request lines (this crate is not on the
+/// registrar-only list for `Cargo.toml`, so the dependency itself is not added here).
+pub use semio_framework_ui_contract::{UiPatch, UiPatchOp};
```

The author's own report (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY/📓️terra-wit-flip-report.md`,
`## registrar-requests`) already names the fix and flags it as pending registrar action — this is
not a surprise to that packet, it is a known, self-documented pending dependency add.

**Reproduced independently, 3 separate `cargo check` invocations, own target dirs, real exit codes
(not through a `| tail` pipe):**

| target | invocation | exit | error |
|---|---|---:|---|
| `semio-s-plugin-stdio --lib` (native) | `CARGO_TARGET_DIR=target-stdio cargo check -p semio-s-plugin-stdio --lib` | **101** | aborts inside `semio-framework` before stdio's own files are ever reached |
| `semio-framework-plugin-host --lib` | `CARGO_TARGET_DIR=target-host cargo check -p semio-framework-plugin-host --lib` | **101** | same |
| `semio-framework-plugin --lib` | `CARGO_TARGET_DIR=target-host cargo check -p semio-framework-plugin --lib` | **101** | same |
| `semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | `CARGO_TARGET_DIR=target-wasm cargo check ...` | **101** | same |

Every one of the four fails with the **identical single error**:

```
🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:873:9:
error[E0432]: unresolved import `semio_framework_ui_contract`:
use of unresolved module or unlinked crate `semio_framework_ui_contract`
error: could not compile `semio-framework` (lib) due to 1 previous error
```

`semio-s-plugin-stdio`'s own `Cargo.toml` (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:37`)
depends directly on `semio-framework = { path = "…/🧰️framework/📦️packages/🦀️rust" }` — the exact
crate that fails. **stdio cannot compile until `semio-framework` compiles; nothing in
`✏️s/🔌️plugins/🗄️stdio/**` can change that.**

## Why this is NOT registrar-only-in-my-ticket, and NOT fixable by me

The root workspace `/Cargo.toml` (registrar-only under **this** ticket's rules) already lists
`semio-framework-ui-contract` as a workspace path alias (`Cargo.toml:205` area, `# ---- ui (10)
----` section) — that part is already correct. The missing piece is a **`[dependencies]` entry in
`🧰️framework/📦️packages/🦀️rust/Cargo.toml`** (the `semio-framework` facade crate's own manifest),
plus one in `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml`
(`semio-framework-graph`) — **both files sit under `🧰️framework/**`, entirely outside this packet's
`path_scope`** (`✏️s/🔌️plugins/🗄️stdio/** EXCLUSIVELY`). I did not touch either.

The peer's own report names a **third** registrar-request line, for
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` itself — *that* one **is** inside my
`path_scope`. I deliberately **did not apply it**: adding an unverified dependency line to a crate I
can never compile-test this packet (the blocker upstream means any edit's correctness is
unfalsifiable right now) would be exactly the "false green" this ticket's own rules warn against
("An honest partial result with precise residue is a GOOD outcome; a false green is the WORST
outcome"). The exact line is recorded below for whoever lands this once the upstream two are fixed.

**Line ready for `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`'s `[dependencies]`, NOT applied:**
```toml
semio-framework-ui-contract = { path = "../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
```
(copied verbatim from the peer's report; path resolution not independently re-verified by me this
packet — do so before applying).

## What this means for the 18,591 baseline

**Unmeasured, unchanged, unverifiable this packet.** `stdio-await`'s last independently-measured
number (18,591, itself already flagged there as a likely modest overstate after 171 uncounted
mode-2 fixes) stands as the last real figure. I could not take a fresh measurement — `cargo check
-p semio-s-plugin-stdio --lib` never reaches a single file inside `✏️s/🔌️plugins/🗄️stdio/**`; the
build aborts three crates upstream, inside the `semio-framework` facade. No diagnostic-driven tool
(R10 discipline — this ticket's tools are all span-keyed off rustc's own JSON output) can run
without a diagnostic to key off of. There is no residue taxonomy to update, because there is no
compiler output past this point to taxonomize.

I did **not** attempt to route around the blocker (e.g. a stub/shim `semio_framework_ui_contract`
re-export or a local patch) — that would touch `🧰️framework/**`, outside scope, and would risk
masking the real fix rather than surfacing it.

## Regression baseline — RE-VERIFIED this packet (own target dirs, real exit codes)

| target | result |
|---|---|
| `cargo test -p semio-framework-os-kernel --lib` | **779 passed / 0 failed / 0 ignored** — exact baseline match |
| `cargo test -p semio-framework-os-kernel-db --lib` | **424 passed / 0 failed / 0 ignored** — exact baseline match |
| `cargo check -p semio-framework-os-kernel --lib --target wasm32-unknown-unknown` | **EXIT 0** (55 warnings, all `async_fn_in_trait`, R7-sanctioned) |
| `cargo check -p semio-framework-plugin-host --lib` | **EXIT 101 — RED** (1 error, the `semio_framework_ui_contract` blocker above) |
| `cargo check -p semio-framework-plugin --lib` | **EXIT 101 — RED** (same 1 error) |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | **EXIT 101 — RED** (same 1 error) |
| `cargo test -p semio-framework-plugin-host --lib` (125/0/1) | **NOT RUN** — `--lib` already fails to compile, a `test` run cannot do better |
| `cargo test -p semio-framework-plugin --lib` | **NOT RUN** — same reason |

**os-kernel and kernel-db hold the exact baseline** (they sit *below* the `semio-framework` facade
in the dependency graph — confirmed by build order in the check output: both `Checking` lines with
zero errors appear before the facade's `Checking semio-framework v0.1.0` line that then fails).

**plugin / plugin-host / stdio are a REGRESSION relative to this packet's own briefed "CURRENT
VERIFIED STATE"** (which stated these three GREEN). This is not something I caused — I made zero
production edits — and it was already independently discovered and reported by the immediately
preceding packet on this ticket (`replication-r9`, see its report's "Gates I could NOT
independently reach" section, same `semio_framework_ui_contract` error, same file, same mtime
12:19:23, confirmed unchanged between that packet's measurement and mine).

## Forced-rebuild dropped-future census (R12/R13/R17) — NOT RUN, correctly

R17: *"a red crate cannot report dropped futures... a census taken while a crate is red is
meaningless."* `semio-s-plugin-stdio` is not merely red — it never reaches its own compilation
units at all. Running the census would report 0 by construction and that 0 would mean "could not
measure," not "no dropped futures," which is exactly the false-negative R12/R17 exist to prevent.
Skipped, per the same discipline `stdio-await` used for the same reason at its own end.

## Recommendation — cross-ticket, needs sol (or whoever bridges tickets)

This is a **cross-ticket** blocker: the fix lives in `SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`'s
scope (its own `wit-flip` packet already wrote the exact registrar-request), but the crate it broke
(`semio-s-plugin-stdio`, and transitively `semio-framework-plugin`/`plugin-host`) belongs to *this*
ticket. Two Cargo.toml edits outside both packets' path_scope will unblock everything downstream in
one move:

1. `🧰️framework/📦️packages/🦀️rust/Cargo.toml` — add the `semio-framework-ui-contract` dependency
   (registrar-owned, framework side).
2. `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml` — same, for `semio-framework-graph`.
3. Once (1)+(2) land, apply the stdio-side line recorded above to
   `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` and re-run this packet — the report's tool
   inventory from `stdio-await` (`insert-await.py` + the 8 hand-built repair tools, all still in
   this ticket folder) is ready to resume exactly where it left off, no rework needed.

**Until then, `semio-s-plugin-stdio` — and this ticket's own `plugin`/`plugin-host` regression
gates — cannot be measured, fixed, or accepted by any packet, regardless of path_scope.** This is
not a "needs more time" situation; it is a hard compile-order dependency that no amount of work
inside `✏️s/🔌️plugins/🗄️stdio/**` can route around.

## Files touched (production) — NONE

Zero production files edited this packet, in or out of scope. Read-only `cargo check`, `git
status`/`git diff`/`git log`, and `grep` only. No `git`-modifying command run.

## Ticket-folder artifacts from this packet

- `terra-stdiofinish-remeasure1.txt` — full `cargo check -p semio-s-plugin-stdio --lib` output,
  the initial re-measurement showing the blocker.
- `terra-stdiofinish-pluginhost.txt` — `cargo check -p semio-framework-plugin-host --lib`, real
  exit 101 captured (not through a pipe).
- `terra-stdiofinish-plugin-lib.txt` — `cargo check -p semio-framework-plugin --lib`, exit 101.
- `terra-stdiofinish-plugin-wasip2.txt` — `cargo check -p semio-framework-plugin --lib --target
  wasm32-wasip2 --features component-guest`, exit 101.
- `terra-stdiofinish-oskernel-wu.txt` — `cargo check -p semio-framework-os-kernel --lib --target
  wasm32-unknown-unknown`, exit 0, confirming the regression baseline for that target.

All five are in this ticket folder (`.txt`, per the scratch-file rule). `CARGO_TARGET_DIR` for
every build stayed under the session scratchpad (`target-stdio`, `target-host`, `target-wasm`,
`target-wu` — all reused, none created), per the ticket-folder-EPERM rule; only the log text was
copied into the ticket folder afterward.
