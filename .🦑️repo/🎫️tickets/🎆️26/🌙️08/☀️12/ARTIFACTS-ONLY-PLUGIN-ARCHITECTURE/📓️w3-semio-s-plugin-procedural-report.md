# W3 — `🌀️procedural` plugin closure report

Plugin: `✏️s/🔌️plugins/🌀️procedural/` (crate `semio-s-plugin-procedural`).

**Supersedes** the earlier version of this file, which stopped at the clearance gate reading
"absent from RELEASED ⇒ held". That reading was wrong — see `📌️important.md` §"How to read that
file — ABSENCE MEANS FREE, not held", which names `🌀️procedural` explicitly as one of the five
plugins that were incorrectly stopped on for exactly this reason. Re-checked and proceeded below.

## Step 0 — clearance

Read SMO's live predicate `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`.
`🌀️procedural` appears in none of the four sections (RELEASED / HELD in-flight / HELD between-waves /
NOT SMO'S TO RELEASE). Per the file's own binding wording, "ABSENCE FROM THIS FILE MEANS FREE, NOT
HELD" — and `📌️important.md` names `🌀️procedural` directly in its "known-free-by-absence" list.
Proceeded.

## What changed

- **Deleted** `✏️s/🔌️plugins/🌀️procedural/🛂️manifest/🦀️component.rs` — 1-line doc-only stub, confirmed
  unmounted (`grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" 📦️glue.rs` → zero hits before deletion).
- **Deleted** `✏️s/🔌️plugins/🌀️procedural/🎟️capabilities/🦀️component.rs` — same, doc-only, unmounted.
- **Deleted** `✏️s/🔌️plugins/🌀️procedural/🔧️setup/🦀️component.rs` — same, doc-only, unmounted. (The
  plugin's real setup fan-out, `register_exports()`, already lives in the root `🦀️component.rs` and is
  wired via `.setup(register_exports)` on the `Plugin::builder` — this facet directory was always dead
  weight duplicating that doc-comment pattern, per `📓️w0-b-plugin-shape.md` §3.)
- **Deleted** `✏️s/🔌️plugins/🌀️procedural/🎮️play/` (1 file: `AGENTS.md`, no code, no `Cargo.toml`) —
  its content (a "play harness" bundle doc stub) was folded into the plugin-root `AGENTS.md` first (see
  below), then the directory removed. No sanctioned taxonomy slot exists for a standalone plugin-root
  doc-only "play" facet; the plugin's actual play apps (`procedural2d-play`, `procedural3d-play`,
  `📦️packages/🦀️rust/Cargo.toml` `[[package.metadata.semio.playground]]` entries) already live correctly
  under `🎛️apps/◻2d` and `🎛️apps/🧊️3d`.
- **Updated** `✏️s/🔌️plugins/🌀️procedural/AGENTS.md` — appended a `## 🎮️ Play harness` section carrying
  the relocated content (new lines 7-11).

No other files touched. Nothing was moved out of `🎛️apps`/`🗿️artifacts`/`📦️packages` because the plugin
root, after the four deletions above, already matched the target shape exactly — there was no compute
module, app-surface code, or fixture data sitting loose at plugin root.

## Step 3 — escape-hatch call sites

Full repo-wide W0-A census (`📓️w0-a-escape-hatch.md` §2a) already identified the *only* `register_*`
family call site inside this plugin:

```
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:649
    semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural3d_document_from_mesh);
```

Re-confirmed live (`grep -rn "semio_framework_os::" ✏️s/🔌️plugins/🌀️procedural/` → this one hit only).
This call sits inside the owning artifact's own `⚙️engine` module, self-registering `"3d.procedural"`,
the kind this plugin itself declares — the **compliant** shape per §2a of the escape-hatch census.
Per dispatch instruction, left exactly as it is. `procedural2d` and `procedural3d`'s app files
(`🎛️apps/◻2d`, `🎛️apps/🧊️3d`, all their `🎮️commands`/`🎭️modes`/`📌️panels`) carry zero `register_*`
calls of any kind. No escape-hatch violation exists anywhere in this plugin — step 3 required no edits.

The demonstrator plugin's duplicate registration of `"3d.procedural"` (`🎪️demonstrator/🎪️panes/🌱️generator`,
per §2d of the escape-hatch census) is out of this plugin's boundary and is being resolved separately, as
instructed — not touched.

## Step 4 — dependency purge

`grep -rn "semio_framework_os::" ✏️s/🔌️plugins/🌀️procedural/` → one live reference (the compliant call
site above). Per instruction, `semio-framework-os` stays in
`📦️packages/🦀️rust/Cargo.toml:46` — nothing to remove.

Seven Cargo dependencies on the flow plugin's extension crates remain, unmodified, as instructed
(grandfathered known layering violation, out of scope for this wave):
`semio-s-plugin-flow-extension-{brep,math,primitive,logic,dictionary,list,text}`
(`📦️packages/🦀️rust/Cargo.toml:49-55`).

## Step 5 — inventory only (nothing changed)

Swept the whole plugin tree for interior-mutable app state and side-effecting primitives:

```
grep -rn "thread_local!" ✏️s/🔌️plugins/🌀️procedural/                          → 0 hits
grep -rn "std::fs\|std::env\|std::process\|Command::new" ✏️s/🔌️plugins/🌀️procedural/ → 0 hits
grep -rn "fn seed(" ✏️s/🔌️plugins/🌀️procedural/                               → 0 hits
grep -rn "reqwest\|hyper::\|TcpStream\|UdpSocket" ✏️s/🔌️plugins/🌀️procedural/  → 0 hits
```

Nothing to inventory: no interior-mutable/`thread_local!` app state, no filesystem/env/process/network
side effects outside test code (there were none at all), no `fn seed(`. No Draft-lane field or
verb-slug proposals apply to this plugin.

## Step 6 — structural verification

1. Closed shape:
```
$ ls -a "✏️s/🔌️plugins/🌀️procedural/"
.  ..  AGENTS.md  README.md  🎛️apps  📦️packages  🗿️artifacts  🦀️component.rs
```
Exactly `🦀️component.rs`, `AGENTS.md`, `README.md`, `🎛️apps`, `🗿️artifacts`, `📦️packages` — nothing else.

2. Every `#[path = "..."]` in `📦️glue.rs` resolves (script walked the file, resolved each non-`"."`
   path relative to `📦️packages/🦀️rust/`, checked `os.path.isfile`):
```
total #[path] entries: 351
non-'.' entries checked: 177
missing: 0
```

3. Dangling references:
```
grep -rn "mod manifest\|mod capabilities\|mod setup\b" 📦️glue.rs   → 0 hits
grep -rn "register_procedural_exports" .  (repo-wide)                → 0 hits (setup was always doc-only, nothing ever called into it)
grep -rln "🌀️procedural/🛂️manifest\|🌀️procedural/🎟️capabilities\|🌀️procedural/🔧️setup\|🌀️procedural/🎮️play" .
  → one hit, repo-root 📜️script.ts:4860 — see sharedFileRequests below.
```

4. Ran the one sanctioned workspace-load check, since a directory was removed from the tree:
```
$ cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
OK
```
Workspace graph loads cleanly.

Also confirmed no `Cargo.toml` existed anywhere under the removed directories before deleting them
(`find <dir> -name Cargo.toml` on all four — zero hits each) and no plugin-root `.DS_Store`/`node_modules`
were present (census already noted procedural had neither).

## `## sharedFileRequests`

- **File**: repo-root `📜️script.ts`
- **Line**: 4860
- **Region**: the plugin-root-shape breach allowlist/notes block (per `📓️w0-b-plugin-shape.md` §1, this
  is the `policyPluginRootShapeBreaches` census, currently report-mode-only per `📌️important.md` "APA's
  five regions... land in report mode only... until APA W5")
- **Reason**: the line reads
  `"✏️s/🔌️plugins/🌀️procedural/🎮️play": "Fold into 🗿️artifacts/<kind>/📚️examples/ once confirmed
  non-placeholder (dir currently holds only AGENTS.md) — 📓️w0-b-plugin-shape.md §5."` — this entry is
  now stale: `🎮️play` was resolved this wave (folded into plugin-root `AGENTS.md`, directory deleted,
  not moved into an artifact's `📚️examples/`). Whoever holds the single-writer slot on repo-root
  `📜️script.ts` (order is APA → UCAS-W6 → SMO, announce on both channels before/after per
  `📌️important.md`) should drop or update this line.
- **Patch file**: none prepared — this is a one-line note removal, described here rather than diffed,
  since I do not touch repo-root `📜️script.ts` from inside a per-plugin boundary.

## `## Concurrent-churn observations`

- Root `🦀️component.rs` files across many plugins were batch-touched at Aug 12 10:50 (noted already in
  `📓️w0-b-plugin-shape.md` §2 as harmless, unrelated tooling pass) — `🌀️procedural`'s own root
  `🦀️component.rs` was not part of that touch and was not modified by this wave either.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` `pluginChildDirs` is, as read at
  the time of this wave, **already** `["🎛️apps"]` only — not the `["🛂️manifest","🎟️capabilities",
  "🔧️setup","🎛️apps"]` the W0-B census captured at ~15:20, and earlier than `📌️important.md`'s stated
  plan ("the flip is the LAST thing APA does"). I did not edit `🔣️taxonomy.json` — this is an
  observation of another session's in-flight state, consistent with the fact that other worked examples
  (`🪐️space`, `💠️lowpoly`) already carry the same closed 6-entry root shape produced here. Flagging in
  case the coordinator needs to know the flip timing moved earlier than the original plan assumed.
- The Rust taxonomy-gate hard `assert!` family (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`,
  fn `assert_taxonomy_components`) sits behind `#[cfg(test)]` (nearest preceding attribute at line 1976,
  no function boundary between it and the assert block at lines 2234-2243) — it is not exercised by
  `cargo check`/`cargo metadata`, only by a `cargo test` run I did not perform (forbidden this wave). Not
  run, not claimed to pass.
- SMO's queued triad-count fix for procedural2d (8 dirs vs 14 dispatch variants, under
  `🧬️mutations/**`) was not touched and not inspected beyond what the pre-existing `📦️glue.rs` mounts
  already show — out of boundary per instruction.

## Honest pass/fail

All six steps completed. Zero escape-hatch violations existed to fix (plugin was already compliant on
that axis). Zero dependency purge was possible or needed (the one live `semio_framework_os::` use is the
sanctioned self-registration). Zero draft-lane inventory items exist in this plugin. Plugin root now
holds exactly the six sanctioned entries. All 177 real `#[path]` mounts resolve. `cargo metadata` loads
the workspace cleanly. No cargo build/test was run (forbidden this wave; `semio-framework-plugin` is
red from another session's in-flight rename per dispatch).

apa-status: complete
