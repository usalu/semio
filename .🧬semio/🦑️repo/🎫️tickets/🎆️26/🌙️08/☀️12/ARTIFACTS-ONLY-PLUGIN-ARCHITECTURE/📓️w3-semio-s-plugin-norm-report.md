# W3 — `semio-s-plugin-norm` (📕️norm) migration report

Ticket: `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` (APA), #2549. Plugin: `📕️norm` (crate
`semio-s-plugin-norm`), directory `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/`.

## Clearance

SMO's live predicate
(`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`) lists
`📕️norm` under **RELEASED**: *"all 15 — 392 triads — 5 facets migrated from scratch + 10 finished;
every dir glue-mounted (no self-wiring left), real OpText/OpBinary, real TS mirrors, `from_snapshot`
decomposition replacing whole-document replace."* Not HELD anywhere, not another session's. Proceeded.

## What changed

### Step 1 — dead facet directories deleted

Checked mounting first: `grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" 📦️glue.rs` (pre-edit) showed
`🔧️setup` mounted at old line 5869 (`#[path = "../../🔧️setup/🦀️component.rs"] mod setup; pub use
setup::register_norm_exports;`); `🛂️manifest` and `🎟️capabilities` had **zero** mount references.

- `🛂️manifest/🦀️component.rs` — 1-line doc-only stub (`//! 🛂️ Manifest facet for 📕️norm — identity
  surfaces live on Plugin::builder in the parent.`), unmounted. **No real fixture data inside** (unlike
  trinity's three JSON manifests) — confirmed by `find 🛂️manifest -type f` returning only the stub.
  Deleted outright.
- `🎟️capabilities/🦀️component.rs` — 1-line doc-only stub, unmounted. Deleted outright.
- `🔧️setup/🦀️component.rs` — **real code**, 51 lines / 48 real, `pub fn register_norm_exports()`: a
  pure fan-out of `crate::artifacts::<x>::engine::register_pilot_languages()` (×15),
  `register_artifact_schema()` (×15) and `register_artifact_inferences()` (×15), plus one
  `crate::config::schema::register_app_schema()` call. Relocated per instruction *before* deletion —
  see next section. Directory then deleted.

No `.DS_Store`/`node_modules` at plugin root (checked, none present).

### Step 1a — the 15-artifact setup fan-out, folded into each artifact's own `⚙️engine`

**Finding before moving anything**: every one of the 15 artifacts' `register_pilot_languages()`
already calls `register_artifact_schema()` as its own first statement (verified for all 15 by
`grep -A2 "pub fn register_pilot_languages"` across every `⚙️engine/🦀️component.rs`, e.g.
`🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:954-955`). This means
the deleted `🔧️setup` facet's separate, explicit `register_artifact_schema()` call for each artifact
(lines 21-35 of the old file) was already redundant with the one inside `register_pilot_languages()`.
Confirmed harmless before relying on it: `register_artifact_schema()` bottoms out in
`::schema::register_artifact_schema_descriptor` →
`register_kernel_artifact_schema_descriptor` (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs:240-246`),
which does `catalog.by_id.insert(descriptor.id, descriptor)` on a `HashMap` — idempotent, no panic,
no duplicate-entry risk, last-write-wins on the identical descriptor value either way.

For each of the 15 artifacts I added a new `pub fn register()` to that artifact's own
`🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`, in a new `//#region 🔖️Register` placed
immediately after `register_pilot_languages()`'s closing brace, calling
`register_pilot_languages(); register_artifact_inferences();` — the schema call is retained exactly
once (inside `register_pilot_languages()`), not duplicated. This exactly mirrors lowpoly's and gis's
precedent of folding a plugin-root fan-out into a single `engine::register()` per artifact. Verified
mechanically with a Python pass (brace-depth matching to find each function's real end, not a text
match) across all 15 files — script output confirmed one insertion per file, no double-inserts, no
misplaced boundary:

```
🗿️artifacts/📓️iso16757/…/⚙️engine/🦀️component.rs   lines 1174-1226 (fn span before insert)
🗿️artifacts/📔️vdi3805/…                              815-867
🗿️artifacts/📕️din4108/…                              954-1006
🗿️artifacts/📗️din16798/…                             1118-1170
🗿️artifacts/📘️en1990/…                               646-698
🗿️artifacts/📘️en1991/…                               662-714
🗿️artifacts/📘️en1992/…                               714-766
🗿️artifacts/📘️en1993/…                               1142-1194
🗿️artifacts/📘️en1994/…                               424-476
🗿️artifacts/📘️en1995/…                               526-578
🗿️artifacts/📘️en1996/…                               421-473
🗿️artifacts/📘️en1997/…                               476-528
🗿️artifacts/📘️en1998/…                               922-974
🗿️artifacts/📘️en1999/…                               586-638
🗿️artifacts/📙️din18599/…                             687-739
```
`grep -c "pub fn register() {" 🗿️artifacts/*/…/⚙️engine/🦀️component.rs` → all 15 report exactly `1`.

**Resolution path check** — `crate::artifacts::<x>::engine::register()` must resolve for the new
`register_norm_exports` to call it. `📦️glue.rs` already carries a pre-existing "pre-migration path
shim" per artifact (comment at glue.rs:928, `// ---- Shims: keep pre-migration module paths resolving
for external callers ----`): `pub mod engine { pub use super::standards::v1::engine::*; }` (14 of 15;
`en1990` shims one level deeper, `pub use super::standards::v1::subsets::any::engine::*;` — same
physical file, different existing shim depth, both confirmed present by grep of all 15
`pub mod engine {` blocks in `📦️glue.rs`). Because these are wildcard re-exports, the new `register()`
fn I added to each engine file is automatically visible at `crate::artifacts::<x>::engine::register()`
with **zero glue.rs shim edits required** beyond the one orchestrator change below.

`📦️packages/🦀️rust/📦️glue.rs`, region `//#region 🔖️Plugin` (was lines 5868-5875): replaced the
`mod setup` + `pub use setup::register_norm_exports;` mount with `register_norm_exports` **defined
directly in glue.rs** — the same pattern trinity used (`register_trinity_exports` "defined directly in
📦️glue.rs's //#region 🔖️Bundle", not a facet dir):

```rust
pub fn register_norm_exports() {
    crate::config::schema::register_app_schema();
    crate::artifacts::din4108::engine::register();
    crate::artifacts::din16798::engine::register();
    crate::artifacts::din18599::engine::register();
    crate::artifacts::en1990::engine::register();
    crate::artifacts::en1991::engine::register();
    crate::artifacts::en1992::engine::register();
    crate::artifacts::en1993::engine::register();
    crate::artifacts::en1994::engine::register();
    crate::artifacts::en1995::engine::register();
    crate::artifacts::en1996::engine::register();
    crate::artifacts::en1997::engine::register();
    crate::artifacts::en1998::engine::register();
    crate::artifacts::en1999::engine::register();
    crate::artifacts::iso16757::engine::register();
    crate::artifacts::vdi3805::engine::register();
}
```

Plugin root `🦀️component.rs:10` — `.setup(crate::register_norm_exports)` — **untouched**, still
resolves: the name now points at the `pub fn` defined at glue.rs's crate root instead of a re-export
from a deleted `mod setup`. Verified: `grep -rn "register_norm_exports"` repo-wide (excluding
`🎯️target`) shows exactly two hits — the definition (glue.rs:5872) and the one call site
(`🦀️component.rs:10`) — no other reference existed to update.

**Global call-order note (intentional, not a bug)**: the old facet called all 15
`register_pilot_languages()` first, then all 15 `register_artifact_schema()`, then all 15
`register_artifact_inferences()` — three passes. The new per-artifact `register()` groups each
artifact's own three registrations together — one pass, 15 groups. Since every registration keys into
a `HashMap` by that artifact's own distinct id (`schema` catalog by `descriptor.id`; verified above)
with no cross-artifact ordering dependency found anywhere in the registration call chain, this
reordering is behaviourally inert at steady state — flagging explicitly rather than asserting it
blind.

### Step 1b — `🎚️config`, `👥️presence`, `📄️artifact`, `🖥️app-surface` — confirmed shared, left in place, NOT guessed into a split

The W0-B census flagged all four of these plugin-root extras as **NEEDS RULING** for norm
specifically, citing the identical open question as `🏗️fem`'s `🖥️app-surface`: *"where does code
shared across ALL of a plugin's sibling apps live under APA?"* Per this wave's explicit instruction
for `🎚️config`/`👥️presence` — *"if they are genuinely shared by ≥2 apps, say so and leave them, filing
the question in your report rather than guessing; a wrong split across 15 apps is expensive to
undo"* — I verified the sharing empirically before deciding, and extended the identical reasoning to
`📄️artifact`/`🖥️app-surface` because the facts are the same:

- **Empirical check**: `find 🎛️apps/<app> -maxdepth 1 -type d` for all 15 apps shows every app has only
  `📚️examples`, `📌️panels`, `🎭️modes`, `🎮️commands`, `⚙️engine` — **zero** apps carry their own
  `🎚️config`/`👥️presence`/`📄️artifact`/`🖥️app-surface`. There is exactly one copy of each of the four,
  at plugin root, consumed by all 15.
- **The plugin's own architectural ruling, already on disk, predates this wave**: `🎚️config/🦀️component.rs`'s
  header states *"Deliberately NOT a per-app `🎛️apps/<app>/🦀️config.rs`: all fifteen compliance apps
  have the identical config shape... so unlike `shooting`'s per-app `ShootingConfig` this is ONE type
  reused by every app rather than fifteen byte-identical copies."* `👥️presence/🦀️component.rs`'s header
  says the same for presence (empty presence, shared by construction).
  `🦀️component.rs`'s doc-comment (📦️glue.rs:13-19) states the same for the other two: *"the fifteen
  standards are structurally identical apps over fifteen genuinely different document schemas, so the
  domain kernel (quantities, clause identity, check results, national annexes, the
  `NormFamily`/`NormHost` contract...) and the app-surface kernel (the one shared config, the media
  ports, the render primitives, the manifest constructors) each exist exactly once."*
- **Why not force a split or a single-owner pick**: unlike trinity's cross-artifact kernel (shared by
  exactly 2 artifacts, physically assigned to the one the DSL is named after, per that report), norm
  has **15** siblings with no natural "primary" — none of din4108/en1990-1999/din16798/din18599/
  iso16757/vdi3805 is more the owner of the shared config/domain-kernel than any other. Duplicating
  into 15 copies contradicts the plugin's own explicit "exactly once" design and the ticket's own
  warning that a wrong 15-way split is expensive to undo. The obvious escape-hatch name (`shared`) is
  in `bannedNameStems` (core/common/util/shared/base/lib/impl per CLAUDE.md), so there is no
  taxonomy-legal single-word substitute available today either.
- **Decision this wave**: left all four in place at plugin root, unmoved, unrenamed. **Filing as
  NEEDS RULING** for whoever owns the "shared-across-N-sibling-apps-of-one-plugin" taxonomy question
  (same open item `🏗️fem`'s `🖥️app-surface` and 🌊️flow/🏭️process/📐️cad's `🧩️extensions` axis are
  waiting on) — not resolved here, not guessed.

This is the one respect in which the plugin root is **not** fully closed to the 6-entry target shape
this wave; every other candidate for relocation (the three dead/real facets) is resolved.

### Step 2 — plugin root, current state

```
$ ls -a "✏️s/🔌️plugins/📕️norm/"
.
..
AGENTS.md
🎚️config          ← flagged, shared by all 15 apps, see Step 1b
🎛️apps
👥️presence        ← flagged, shared by all 15 apps, see Step 1b
📄️artifact        ← flagged, shared by all 15 artifacts, see Step 1b
📦️packages
🖥️app-surface     ← flagged, shared by all 15 apps, see Step 1b
🗿️artifacts
🦀️component.rs
```
No `README.md` existed before this wave either (not something this wave removed — matches the
`📸️remodel`/`🖨️raster`/`🪐️space` precedent of plugins with no README).

`🦀️component.rs`, `AGENTS.md`, `🎛️apps`, `🗿️artifacts`, `📦️packages` are the five sanctioned entries;
the four flagged dirs are the one documented, justified holdout (parallel to trinity's `🔨️modules`
holdout, but for a different concrete reason — genuine N-way shared code with no legal destination,
not a crate-boundary issue).

### Step 3 — escape-hatch call sites

None. `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/📕️norm"` → **zero hits**, whole plugin tree, before
and after edits. `📓️w0-a-escape-hatch.md`'s full call-site census also lists no norm entries. No-op,
confirmed both ways.

### Step 4 — dependency purge

`grep -rn "semio_framework_os::" "✏️s/🔌️plugins/📕️norm"` → **zero hits**.
`grep -n "semio-framework-os" "📦️packages/🦀️rust/Cargo.toml"` → only `semio-framework-os-kernel` (a
different crate, used pervasively via the `dsl`/`store`/`protocol`/`vcs`/`schema` extern-crate aliases
in `📦️glue.rs:21-25`) — the escape-hatch crate `semio-framework-os` was **never a dependency of this
plugin at all**. Nothing to remove.

### Step 5 — inventory only, nothing changed

```
$ grep -rn "thread_local!" "✏️s/🔌️plugins/📕️norm" --include="*.rs"
(zero hits)
$ grep -rn "std::fs::\|std::env::\|std::process::\|Command::new(" "✏️s/🔌️plugins/📕️norm" --include="*.rs"
(zero hits)
$ grep -rn "reqwest\|TcpStream\|hyper::\|std::net::" "✏️s/🔌️plugins/📕️norm" --include="*.rs"
(zero hits)
$ grep -rn "fn seed(" "✏️s/🔌️plugins/📕️norm" --include="*.rs"
(zero hits)
```
`📕️norm` has **no** interior-mutable app state anywhere — no `thread_local!`, no `OnceLock`/`Mutex`
app-side cache, no draft-lane scratch to inventory for any of the 15 apps, no filesystem/env/process/
network IO, no `seed()`. This is the cleanest of the plugins surveyed so far in this ticket (contrast
`💠️lowpoly`'s `LOWPOLY_SCRATCH` thread_local, `🏭️process`/`🌀️procedural`'s live mesh registrations).
Nothing to propose for `Draft` fields or verb-slugs — there is no gestural UI state in this plugin's
model at all; every app's only view-state is the shared `NormConfig.selected_check_index` (already a
config field, not draft state, per the SMO-released `norm.config` facet, out of APA's draft-lane scope
entirely).

One pre-existing, unrelated observation (not touched, out of this wave's scope): `register_io()`
exists in all 15 artifact engines (`crate::artifacts::<x>::io_registry::register()` fan-through) but
was **not** called by the old `🔧️setup` facet and is **not** called by the new `register_norm_exports`
either — `grep -rn "register_io()" "✏️s/🔌️plugins/📕️norm" --include="*.rs" | grep -v "pub fn register_io"`
returns zero call sites anywhere in the plugin, for any of the 15 artifacts. This gap predates this
wave (the old facet never called it either — confirmed by reading the deleted file verbatim before
deletion, quoted in full in the dispatch's own census). Flagging, not fixing: composer/IO registration
being unreachable would be a real behavioural gap if true, but it may also be that `io_registry`'s
`entries()` is discovered declaratively elsewhere (e.g. by the artifact-composition tree) without
needing an explicit `register_io()` call — not verified either way this wave, out of scope for a
structural-relocation pass.

## Files touched

**Updated (new `pub fn register()` added, `//#region 🔖️Register`, no other change):**
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990…📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (all 10)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

**Updated:**
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` — region `//#region 🔖️Plugin`: removed the
  `🔧️setup` mount, added `register_norm_exports` defined directly in glue.rs, fanning out to the 15
  artifact `engine::register()` calls plus the one app-schema call. No other region touched.

**Removed:**
- `✏️s/🔌️plugins/📕️norm/🔧️setup/` (dir + `🦀️component.rs`, real code relocated first per above)
- `✏️s/🔌️plugins/📕️norm/🛂️manifest/` (dir + doc-only `🦀️component.rs`, unmounted, no fixture data)
- `✏️s/🔌️plugins/📕️norm/🎟️capabilities/` (dir + doc-only `🦀️component.rs`, unmounted)

**Not touched (flagged, see Step 1b):** `✏️s/🔌️plugins/📕️norm/🎚️config/`, `👥️presence/`, `📄️artifact/`,
`🖥️app-surface/` — genuinely shared by all 15 apps/artifacts, no taxonomy-legal single destination
exists, left in place rather than guessed.

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` — **not touched**; nothing to purge (§Step 4).

## Step 6 — structural verification

**1. Plugin root shape** (pasted above under Step 2) — 5/9 entries match the fully-closed target
exactly; the 4 extra are the one documented, justified holdout.

**2. Every `#[path = "..."]` in `📦️glue.rs` resolves to a real file** — exhaustive, not sampled.
Python pass, regex-anchored to `^\s*#\[path\s*=\s*"..."\]` (doc-comment prose that merely *mentions*
`#[path]` is not miscounted), resolved each relative to `📦️glue.rs`'s own directory, `os.path.isfile`
on every target:
```
total path attrs: 2318
missing count: 0
```
(`.` self-mounts, used by the grouping `pub mod X { #[path="."] ... }` pattern, excluded by design —
not leaf mounts, matches trinity's/lowpoly's methodology.)

**3. Dangling-reference sweep, repo-wide**, for everything removed:
```
$ grep -rln "📕️norm/🔧️setup\|📕️norm/🛂️manifest\|📕️norm/🎟️capabilities" . \
    --include="*.rs" --include="*.ts" --include="*.toml" --include="*.json" \
    | grep -v "🎯️target\|node_modules\|\.git/"
.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/ROOT-SCRIPT-POLICY-REVIVAL-AND-TAXONOMY-LINT-PREP/canonicalize-check.ts
.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/ROOT-SCRIPT-POLICY-REVIVAL-AND-TAXONOMY-LINT-PREP/baseline-breaches-before.json
```
Both hits are scratch/report artifacts from a different, already-closed ticket dated 26/08/05 (predates
APA entirely) — not live code, not touched, matches the class of stale-reference hit trinity's report
also found and left alone.

`register_norm_exports` reference check:
```
$ grep -rn "register_norm_exports" . --include="*.rs" | grep -v "🎯️target"
✏️s/🔌️plugins/📕️norm/🦀️component.rs:10:        .setup(crate::register_norm_exports)
✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs:5872:pub fn register_norm_exports() {
```
Exactly one definition, one call site — both intact and correctly wired.

```
$ ls "✏️s/🔌️plugins/📕️norm/🔧️setup" "✏️s/🔌️plugins/📕️norm/🛂️manifest" "✏️s/🔌️plugins/📕️norm/🎟️capabilities"
ls: ...🔧️setup: No such file or directory
ls: ...🛂️manifest: No such file or directory
ls: ...🎟️capabilities: No such file or directory
```

**4. `pluginChildDirs` re-check** — already relaxed to `["🎛️apps"]` by an earlier wave (confirmed, not
this wave's edit):
```
$ grep -n -A2 '"pluginChildDirs"' "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
  "pluginChildDirs": [
    "🎛️apps"
  ],
```

**5. The one sanctioned cargo command** (workspace still loads after the edits):
```
$ cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```
No other `cargo check`/`build`/`test` was run, per standing order (SDK is red from another session's
in-flight rename). This confirms manifest/workspace-graph integrity only, not that the crate compiles
— the `#[path]` resolution sweep (item 2) and the register-fn insertion script's brace-depth matching
(§Step 1a) are the structural evidence for the Rust-level edits themselves.

## Step 5 recap — nothing changed, confirmed empty across every category

thread_local!, std::fs/env/process/Command::new, network, `fn seed(` — all zero hits (§Step 5 above).
No Draft-lane fields proposed; no interior-mutable app state exists in this plugin to inventory.

## sharedFileRequests

None required to complete this wave. The one open item (§Step 1b — where genuinely-shared-across-15
config/domain/app-surface code belongs under APA when no single sibling artifact/app is the natural
owner and `bannedNameStems` rules out `shared`/`core`/`common`) is a **policy ruling request**, not a
file-edit request — flagging for whoever owns the cross-cutting-shared-code taxonomy question (the
same open item as `🏗️fem`'s `🖥️app-surface` in `📓️w0-b-plugin-shape.md` §5). No patch to attach; the
decision, once made, is a mechanical move (or a taxonomy addition) either way.

## Concurrent-churn observations

- `git log --oneline -3 -- "✏️s/🔌️plugins/📕️norm"` and `stat -f '%Sm'` on the plugin root showed the
  tree's most recent commits predating this wave's edits, consistent with the repo-wide batch-touch
  noted in `📓️w0-b-plugin-shape.md` (`Aug 12 10:50`) — no sign another session was mid-edit in this
  plugin's directory during this wave.
- Did not touch `🧬️mutations/**` anywhere in the tree — the only files edited were 15 artifact `⚙️engine`
  files (adding one `register()` fn each, outside any `mutations` dir) and `📦️glue.rs`'s `🔖️Plugin`
  region.
- Did not author or touch any `thread_local!`/draft-lane facet — none exist in this plugin (§Step 5).
- Did not rename or re-declare any artifact kind id — all 15 `id:`/`ArtifactKindSpec` declarations
  untouched; only registration *plumbing* moved, never the kind identities themselves.
- Never wrote the banned identifiers (`SetSnapshot`, `NoMutation`, `CollectionMutation`) anywhere —
  confirmed by re-reading every string literal introduced in this wave's edits (none).

## apa-status: partial

Steps 1, 1a, 3, 4, 5 and 6 are complete and fully evidenced above: both dead facets deleted (confirmed
unmounted, no fixture data lost), the real `🔧️setup` fan-out relocated per-artifact into each of the
15 artifacts' own `⚙️engine::register()` with a verified-idempotent dedup of the redundant
`register_artifact_schema()` double-call, the plugin-root `.setup()` hook repointed at a
glue.rs-resident orchestrator (trinity's own precedent for this exact shape), zero escape-hatch call
sites (confirmed, none existed), zero `semio-framework-os` dependency to purge (confirmed, was never a
dependency), and a completely empty Step 5 inventory (no thread_local, no fs/env/process/network, no
seed, no draft-lane candidates in this plugin at all).

**What keeps this `partial` rather than `complete`**: the plugin root is not reduced to the strict
6-entry target shape — `🎚️config`, `👥️presence`, `📄️artifact`, `🖥️app-surface` remain, **by deliberate
decision, not oversight**, because they are genuinely and provably shared by all 15 sibling
apps/artifacts (empirically verified — no per-app duplicates exist anywhere — and independently
confirmed by the plugin's own pre-existing architectural doc-comments), and forcing either a 15-way
duplication or an arbitrary single-owner pick would be exactly the "wrong split, expensive to undo"
outcome this wave's own instructions warned against for the smaller `🎚️config`/`👥️presence` case. This
mirrors `🏗️fem`'s `🖥️app-surface` holdout in the census and trinity's `🔨️modules` holdout in spirit
(a documented, justified exception rather than a silent gap) — filed under `sharedFileRequests` above
as a policy question for the next wave, not guessed here.

**What the consolidated build should check first for this plugin**: (1) that the 15 new `engine::register()`
functions really are reachable via the pre-existing wildcard shim (`pub use super::standards::v1::engine::*;`
or the one-level-deeper `subsets::any::engine` variant for `en1990`) the way this report assumes —
structurally verified (grep-confirmed shim presence for all 15, `cargo metadata` confirms the workspace
graph loads) but not compile-verified, since cargo check/build was intentionally not run this wave;
(2) that removing the standalone, separately-called `register_artifact_schema()` invocation (now only
reached transitively via `register_pilot_languages()`) doesn't matter to any caller that expected it to
be independently callable before the full `register()` fires — grep found no such caller anywhere in
the plugin, but a compile pass is the stronger confirmation; (3) the pre-existing `register_io()`
reachability gap noted in §Step 5, unrelated to this wave's edits but worth a second pair of eyes.
