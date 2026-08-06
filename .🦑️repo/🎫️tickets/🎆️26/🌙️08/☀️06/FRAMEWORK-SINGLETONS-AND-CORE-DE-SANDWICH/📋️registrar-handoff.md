# 📋️ Registrar Handoff — Framework Singletons + Core De-Sandwich (W8b)

Scope owned by this ticket (exclusive): `🧰️framework/🔨️modules/#⃣hash/**`, `🧰️framework/🔨️modules/🧬️schema/**`,
`🧰️framework/🔨️modules/✍️editor/**`, `🧰️framework/⚡️implementations/**`, `🧰️framework/📦️packages/**`.

Per constraint, **no dependent's `Cargo.toml`/`package.json` was edited outside the trees above, and the
root `Cargo.toml`/`package.json` were never touched.** New Shape V2 crates were built and verified
standalone (temporary workspace overlay, see TEMPLATE.md §3); **old crates were left fully intact and
functional** so the workspace never broke mid-migration. Everything below is what the registrar must do
to cut all four crates over in one atomic pass.

## 1. What moved (new Shape V2 locations, all verified to exist)

| Crate | Old (still live, still a workspace member) | New (built, standalone-verified, NOT yet a workspace member) |
|---|---|---|
| `semio-framework-hash` | `🧰️framework/🔨️modules/#⃣hash/⚡️implementations/🦀️rust` | `🧰️framework/🔨️modules/#⃣hash/📦️packages/🦀️rust` |
| `semio-framework-schema` | `🧰️framework/🔨️modules/🧬️schema/⚡️implementations/🦀️rust` | `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust` |
| `semio-framework-editor` | `🧰️framework/🔨️modules/✍️editor/⚡️implementations/🦀️rust` | `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust` |
| `semio-framework-core` | `🧰️framework/⚡️implementations/🦀️rust` (8.4k-line godfile) | `🧰️framework/📦️packages/🦀️rust` (godfile split into topic `component.rs` folders, see §5) |

Data files moved to **owner root** (Shape V2 `rootDataDirNames`), copied back to the old crate location too
so the old crate keeps working until cutover:
- `🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json`, `🤖️generated.rs`, `🤖️generated/` (byte-identical
  copies still also sit under the old `⚡️implementations/🦀️rust/` — **delete the old copies when you
  delete the old crate dir**, not before).
- `🧰️framework/🤖️generated/🟦️manifest.ts` — ts-rs output for `semio-framework-core`, new owner-root
  location (see §6, this is currently a **byte-copy of the old output**, not yet freshly regenerated —
  regeneration is blocked, see §7).

`pkg/` under the old editor dir is a gitignored `wasm-pack` build artifact, not source — nothing to move.

## 2. Root `Cargo.toml` changes needed (you own this file; I never touched it)

### `[workspace] members` — replace 4 lines
```
🧰️framework/⚡️implementations/🦀️rust                          → 🧰️framework/📦️packages/🦀️rust
🧰️framework/🔨️modules/#⃣hash/⚡️implementations/🦀️rust           → 🧰️framework/🔨️modules/#⃣hash/📦️packages/🦀️rust
🧰️framework/🔨️modules/🧬️schema/⚡️implementations/🦀️rust         → 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust
🧰️framework/🔨️modules/✍️editor/⚡️implementations/🦀️rust          → 🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust
```

### `[workspace.dependencies]` convenience-path comment (line ~266)
```toml
semio-framework-core = { path = "🧰️framework/⚡️implementations/🦀️rust" }  # 58 refs
```
→
```toml
semio-framework-core = { path = "🧰️framework/📦️packages/🦀️rust" }  # 58 refs
```
(Note: my own grep of `path = "🧰️framework/⚡️implementations/🦀️rust"` across all `Cargo.toml` files found
**33 external dependent lines + root + self = 35 files / 36 occurrences**, not 58. The `# 58 refs` comment
looks like a stale/approximate survey count from an earlier ticket — trust the exact list in §3.)

## 3. Every dependent path string that needs updating (exact old → new)

All four crates moved at **the same depth** (`⚡️implementations/🦀️rust` → `📦️packages/🦀️rust`, same
segment count), so **every dependent's relative `../` prefix count is unchanged** — this is a pure
substring replace of `⚡️implementations/🦀️rust` → `📦️packages/🦀️rust` *within the specific path string
that names one of these 4 crates*. Do not blanket-replace the substring repo-wide — other unrelated
crates also legitimately live under `⚡️implementations/🦀️rust` and must not move.

### `semio-framework-core` dependents (33 files, `path = "...🧰️framework/⚡️implementations/🦀️rust"`)
```
🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust/Cargo.toml:23
🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/⚡️implementations/🦀️rust/Cargo.toml:18
✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:29
✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml:56
✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:51
✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml:31
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🦀️rust/🌍️world/Cargo.toml:15
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚡️implementations/🦀️rust/Cargo.toml:30
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚡️implementations/🦀️rust/Cargo.toml:17
✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml:40
✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml:40
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:31
✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/Cargo.toml:39
✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:40
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/Cargo.toml:22
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️implementations/🦀️rust/Cargo.toml:14
✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🔨️modules/⚙️engine/⚡️implementations/🦀️rust/Cargo.toml:24
✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/Cargo.toml:31
✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml:39
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/⚡️implementations/🦀️rust/🧩️extensions/📐️brep/Cargo.toml:20
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/⚡️implementations/🦀️rust/Cargo.toml:44
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/⚡️implementations/🦀️rust/Cargo.toml:18
✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🔨️modules/⚙️engine/⚡️implementations/🦀️rust/Cargo.toml:24
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:82
✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust/Cargo.toml:18
✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/Cargo.toml:32
✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/Cargo.toml:33
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/Cargo.toml:30
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/Cargo.toml:30
✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/Cargo.toml:44
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/Cargo.toml:30
✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml:42
✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:73
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml:21
```
The last one was missed by a plain-substring survey since its path has no `🧰️framework/` prefix segment
(it's `path = "../../../../../../../../../⚡️implementations/🦀️rust"`, 9×`../` straight into
`⚡️implementations/🦀️rust`) — same fix, swap `⚡️implementations` → `📦️packages`. This file is **also** an
editor dependent (line 23, alias `framework_editor`, §3 below) — both lines need the swap.

Every one of these lines has the exact shape
`semio-framework-core = { path = "<N-times ../>🧰️framework/⚡️implementations/🦀️rust", package = "semio-framework-core"[, optional = true] }`
— replace only the `⚡️implementations` segment with `📦️packages`, leave `../` count and everything else untouched.

### `semio-framework-hash` dependents (4 external + root + self = 6)
```
Cargo.toml:29                                                                        (workspace member list)
✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml:43        (path dep, alias `framework_hash`)
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚡️implementations/🦀️rust/Cargo.toml:18
🧰️framework/⚡️implementations/🦀️rust/Cargo.toml:21              (old core's own dep on hash — moot once old core is deleted)
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/⚡️implementations/🦀️rust/Cargo.toml:22  (alias `framework_hash`)
```
Replace `🔨️modules/#⃣hash/⚡️implementations/🦀️rust` → `🔨️modules/#⃣hash/📦️packages/🦀️rust` in each (the new
`semio-framework-core`'s own `Cargo.toml` already points at the new hash location — nothing to do there).

### `semio-framework-schema` dependents (0 external + root + self = 2)
```
Cargo.toml:225                                                                       (workspace member list)
```
No other crate depends on schema today — lowest blast radius of the four.

### `semio-framework-editor` dependents (2 external + root + self = 4)
```
Cargo.toml:210                                                                       (workspace member list)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml:23  (alias `framework_editor`)
✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml:32                                (alias `framework_editor`)
```

### `semio-framework-core` dependents that need **zero** per-file edits (already `.workspace = true`)
A crate-name survey (`grep -rl 'semio-framework-core' --include=Cargo.toml`) turned up 6 more consumers
beyond the path-string list above — these already inherit the dependency from root
`[workspace.dependencies]`, so **once you fix the single root line in §2 they resolve to the new location
automatically, no edit needed**:
```
✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml:40   semio-framework-core = { workspace = true }
✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/Cargo.toml:38     semio-framework-core.workspace = true
✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml:37        semio-framework-core = { workspace = true }
✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml:47        semio-framework-core = { workspace = true }
✏️s/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml:29               semio-framework-core = { workspace = true }
✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml:44              semio-framework-core = { workspace = true, optional = true }
```
This is worth calling out as the better long-term pattern — every plugin/module still on an explicit
`path = "..."` for `semio-framework-core` (§3's 33-file list) could be migrated to `.workspace = true`
in the same registrar pass, collapsing all future core moves to the single root-line edit these 6 already
enjoy. Not done here since the ticket's constraint was "do NOT edit dependents' Cargo.toml path strings
outside your trees."

## 4. Root `package.json` `workspaces` array — 2 lines (bun/npm workspace globs, separate from Cargo)
```
line 23: "🧰️framework/🔨️modules/✍️editor/⚡️implementations/🦀️rust"  → "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust"
line 35: "🧰️framework/🔨️modules/🧬️schema/⚡️implementations/🦀️rust" → "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"
```
`semio-framework-hash` and `semio-framework-core` (rust) have **no `package.json`**, hence no
`workspaces` entry to touch for them. Run `bun install` after editing to refresh `bun.lock`.

No hits in `.vscode/launch.json` or `nx.json` — both reference projects by **nx project name**
(`@semio-tech/framework-core-rs`, `@semio-tech/framework-schema`, `@semio-tech/framework-editor-rs`, all
preserved unchanged in the new `📋️project.json` files) rather than hardcoded paths, so **no launch.json
or nx.json edit is needed** — nx re-resolves each project's `cwd` from its `project.json` automatically
once the workspace member path flips.

## 5. Framework-core godfile split (already done, verify structure)

`🧰️framework/📦️packages/🦀️rust/📦️lib.rs` now `#[path = ...]`-includes 4 sibling topic folders at the
framework owner root (matches the SHAPE V2 amendment: "framework godfile splits become
`<topic>/🦀️component.rs` folders"):
```
🧰️framework/🎯️action-bus/🦀️component.rs
🧰️framework/🔺️mesh/🦀️component.rs         (mesh + dwg + media + artifact-kind + app-io + config/command-grammar)
🧰️framework/🖥️platform/🦀️component.rs
🧰️framework/🧩️ui/🦀️component.rs           (manifest/action/command types)
🧰️framework/🧩️ui/🧠️kernel/🦀️component.rs  (nested — kernel types: ids, capability, presence, window, VCS-adjacent)
```
Byte-identical round-trip was verified by the ticket's own `work/split.ts` script when it ran (see ticket
history) — re-run `bun ./.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH/work/split.ts`
again post-cutover if you want to re-diff against the old godfile before deleting it.

## 6. ts-rs retarget (framework-core is the one real active ts-rs driver, per M8's finding)

- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`'s `generatedManifestPath()` now resolves to
  `🧰️framework/🤖️generated/🟦️manifest.ts` (owner root), replacing the old
  `🧰️framework/⚡️implementations/🟦️typescript/generated/manifest.ts` (non-emoji `generated/`, sandwiched).
- `🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts` (the TS consumer — still at its old sandwiched
  path, see below) now imports from `"../../🤖️generated/🟦️manifest.ts"`. Verified with a runtime `bun -e`
  dynamic-import smoke test — resolves and evaluates cleanly.
- The old `🧰️framework/⚡️implementations/🟦️typescript/generated/` dir was deleted (superseded, single
  source of truth is now the owner-root `🤖️generated/`).
- **The TS *implementation* directory itself (`🧰️framework/⚡️implementations/🟦️typescript`) was
  deliberately left at its old sandwiched path** — it's in-scope per the ownership grant
  (`🧰️framework/⚡️implementations/**` covers both `🦀️rust` and `🟦️typescript`), but de-sandwiching it
  wasn't in the user's explicit Target list for this pass, and it has an external write-dependency from
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts` (a **different, actively-being-restructured
  UI ticket**, out of my ownership) which writes `🤖️generated/🟦️ui-axes.ts` into this same TS dir. Moving
  it now would silently break that other agent's write path without my being allowed to also patch it.
  **Recommend a follow-up ticket** to de-sandwich `🧰️framework/⚡️implementations/🟦️typescript` →
  `🧰️framework/📦️packages/🟦️typescript` once the UI restructure ticket lands, updating both the
  `ui_wgpu` writer and `package.json` workspaces (line 22: `"🧰️framework/⚡️implementations/🟦️typescript"`).

## 7. Blocker — transitive crates outside this ticket (updated 2026-08-06)

`semio-framework-ui-wgpu` at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` **now has a `Cargo.toml` again** (§7's
"missing manifest" condition is cleared), but the crate **does not yet compile** on the default feature
set that `semio-framework-core` pulls in:

- `📦️lib.rs` gates engine modules with `#[cfg(feature = "wgpu-engine")]`, while `Cargo.toml` exposes
  `engine` (not `wgpu-engine`).
- Engine sources reference `crate::wgpu::…` but there is no `pub mod wgpu` re-export shim in `📦️lib.rs`.

**Action for UI ticket / registrar**: align feature naming and module wiring, then re-run:
```
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --manifest-path "🧰️framework/📦️packages/🦀️rust/Cargo.toml"
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --manifest-path "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml"
```

`semio-framework-editor` is **additionally** blocked today on `semio-framework-compiler-math` not being
published from the consolidated math package (`🧮️math/📦️packages/🦀️rust`) while `semio-framework-compiler`
still lists it as a path dep — math-family registrar work must land first (or compiler deps retargeted).

After both transitive chains are green, regenerate the real ts-rs manifest (replacing the byte-copied stand-in from §6):
```
bun nx run @semio-tech/framework-core-rs:generate
```
(Compare against pre-move output if desired; split was code-motion only.)

## 8. Temporary verification overlays — delete before/at cutover

Each new crate's `Cargo.toml` has a clearly fenced `# ==== 🧪️ TEMPORARY VERIFICATION OVERLAY ====` block
(`[workspace] members = ["."]` + duplicated `[workspace.package]`/`[workspace.dependencies]`/
`[profile.*]`/`[workspace.lints.*]`, copied verbatim from root so standalone `cargo check` fingerprints
matched). **Delete that whole fenced block** in all 4 new `Cargo.toml`s when wiring them into the real
workspace, and delete the nested build caches:

**2026-08-06 note:** overlays had been accidentally truncated to comment headers only (no `[workspace]`
table). They were **restored** in this pass so standalone verification works again; do not remove until
cutover.

```
🧰️framework/📦️packages/🦀️rust/{target/,Cargo.lock}
🧰️framework/🔨️modules/#⃣hash/📦️packages/🦀️rust/{target/,Cargo.lock}
🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/{target/,Cargo.lock}
🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/{target/,Cargo.lock}   (none generated yet — blocked, see §7)
```
`target/` is gitignored repo-wide; the nested `Cargo.lock`s are currently tracked-and-staged in git's
index from this ticket's work — remove them along with the overlay.

## 9. Old crate dirs — keep until cutover, then delete

Do **not** delete these until the root `Cargo.toml`/`package.json` switch is live and a full
`cargo check --workspace` passes, since they are still real, functional workspace members:
```
🧰️framework/⚡️implementations/🦀️rust                       (old core — keep 🧰️framework/⚡️implementations/🟦️typescript, see §6)
🧰️framework/🔨️modules/#⃣hash/⚡️implementations/🦀️rust
🧰️framework/🔨️modules/🧬️schema/⚡️implementations/🦀️rust    (+ its 🔣️entity-kinds.json/🤖️generated.rs/🤖️generated/ copies)
🧰️framework/🔨️modules/✍️editor/⚡️implementations/🦀️rust     (+ its gitignored pkg/ wasm-pack output — safe to just rm -rf)
```

## 10. Compile status per crate (`DEVELOPER_DIR=/Library/Developer/CommandLineTools`, 2026-08-06)

| Crate | Standalone `cargo check` | Notes |
|---|---|---|
| `semio-framework-hash` | ✅ pass | Overlay restored after truncation; zero external deps beyond `blake3`. |
| `semio-framework-schema` | ✅ pass | Overlay restored; entity-catalog `build.rs` reads owner-root `🤖️generated.rs`. |
| `semio-framework-editor` | ❌ blocked | Overlay restored; fails resolving `semio-framework-compiler-math` via `infinite_canvas` → `compiler` (math consolidation, §7). |
| `semio-framework-core` | ❌ blocked | Overlay restored; fails compiling transitive `semio-framework-ui-wgpu` (`crate::wgpu` / `wgpu-engine` vs `engine`, §7). Godfile split + public `pub use` surface in new `📦️lib.rs` unchanged vs old. |

**Old `⚡️implementations/🦀️rust` dirs were not deleted** — root workspace still lists them; cutover remains registrar-owned (§2–§3).

## 11. Dependent-list size summary

| Crate | Needs a path-string edit | Needs zero edit (`.workspace = true`) | + root + self | Total files touching this crate |
|---|---|---|---|---|
| `semio-framework-core` | 34 | 6 | 2 | **42** |
| `semio-framework-hash` | 4 | 0 | 2 | **6** |
| `semio-framework-schema` | 0 | 0 | 2 | **2** |
| `semio-framework-editor` | 2 | 0 | 2 | **4** |

(Cross-checked two ways this pass: a literal old-path-string survey, §3, plus a broader crate-name survey
that also catches `.workspace = true` consumers and paths without the `🧰️framework/` prefix segment —
the latter caught the `➗️mathematical`/`📜️imperative`/`📸️remodel`/`🔱️trinity`/`◻2d`/`🧊️3d` `.workspace = true`
consumers and the renderer-engine wgpu target's direct-path core dependency that the first pass missed;
both are folded into §3/§3's `.workspace = true` subsection now. The root `Cargo.toml`'s own `# 58 refs`
comment on the `semio-framework-core` line still looks like a stale survey number from an earlier ticket
— 42 is what's actually on disk today.)
