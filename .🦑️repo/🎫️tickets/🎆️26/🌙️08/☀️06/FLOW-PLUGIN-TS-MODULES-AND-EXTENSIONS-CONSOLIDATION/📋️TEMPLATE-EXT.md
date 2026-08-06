# 📋️ Per-plugin extension de-sandwich template (W6 Rule B)

Written by the W6 pilot (🌊️flow, ticket `26/08/06/FLOW-PLUGIN-TS-MODULES-AND-EXTENSIONS-CONSOLIDATION`)
from what actually happened de-sandwiching flow's `🧩️extensions/🏗️bim` (a wasm-bindgen Rust crate).
Companion to `📋️TEMPLATE.md` (the plugin-crate merge recipe) and `📋️TEMPLATE-TS.md` (the TS-module fold
recipe). Read the master doc's Discovery contract and Registrar Protocol first.

Use this for a plugin's `🧩️extensions/<e>/` — a dynamically-loaded, separately-published unit (per
Rule B: "extensions stay separate packages, de-sandwiched in place … published names frozen since
they're runtime lookup keys"). **An extension does NOT get folded into the plugin's own crate/package —
it stays its own package, forever a separate build unit** (that's the whole point of it being an
"extension": plugins load it dynamically by its published name string, so merging it into the plugin
crate would silently break that lookup, and even a rename breaks it just as badly as a merge).

---

## 0. Before you touch anything

1. **Open your own ticket** if one isn't already reserved. Scratch files in your ticket folder only.
2. **Inventory the extension.** `find ✏️s/🔌️plugins/<p>/🧩️extensions/<e> -type f` — flow's `🏗️bim` had
   the sandwich `⚡️implementations/🦀️rust/{Cargo.toml, 📦️lib.rs, pkg/}` plus THREE files already
   sitting one level up at the extension's own root (`package.json`, `📋️project.json`, `📜️script.ts`)
   from an earlier partial cleanup — don't assume every extension's starting shape is identical; read
   what's actually there before planning the move.
3. **Record the test baseline.** `cargo test --manifest-path <old-crate>/Cargo.toml --lib` (needs the
   temporary `[workspace]` overlay from TEMPLATE.md §3 if root Cargo.toml is red for unrelated reasons —
   see §5 below, extensions hit the exact same chicken-and-egg problem plugin crates do).
4. **Find dependents.** `grep -rln "<old-crate-package-name>\|<old-crate-name-underscored>" --include=Cargo.toml .`
   — flow's bim extension had none (nothing else in the repo depends on an extension crate; extensions
   are host-loaded by string at runtime, never `path = "…"`-depended-on by other Rust crates). If yours
   has real Cargo dependents, treat it exactly like TEMPLATE.md §0.5/§8.2.
5. **Confirm what's genuinely absent, don't invent structure.** An extension is typically a flat,
   single-purpose wasm module — bim has no `🗿️artifacts`/`🎛️apps` domain tree of its own, just one
   `📦️lib.rs` with inline `#[cfg(test)]` blocks. That's correct and expected, the same "genuinely
   absent, not skipped" pattern TEMPLATE.md documents for 🔋️energy. This ticket's scope is the
   STRUCTURAL de-sandwich only — do not decompose the extension's internal code into taxonomy component
   folders unless that's explicitly asked for; it's a separate, much larger effort out of scope here.

---

## 1. The move — same depth in, same depth out

Fold `⚡️implementations/<lang>/*` into `📦️packages/<lang>/` **at the extension's own root** (not the
plugin's root — the extension is its own owner in the taxonomy, one level deeper than the plugin):

```
🧩️extensions/<e>/⚡️implementations/🦀️rust/{Cargo.toml, 📦️lib.rs, pkg/}
                        ↓ becomes ↓
🧩️extensions/<e>/📦️packages/🦀️rust/{Cargo.toml, 📦️lib.rs, pkg/}
```

**The path-segment depth from repo root to the manifest's own directory is IDENTICAL before and
after** — `⚡️implementations/🦀️rust` and `📦️packages/🦀️rust` are both exactly 2 segments under the
extension root. This means every one of the crate's external `path = "…/N-levels-up/…"` dependency
strings in `Cargo.toml` needs **zero edits** — count the `../` in one dependency line before the move,
confirm the same count still reaches the same target after, and move on. The ONLY Cargo.toml edit is
`[lib] path`, which goes from the old absolute-style "climb all the way to repo root, then back down"
string (an artifact of the file having lived one sandwich-namespace removed from where `[lib] path` was
computed relative to) to the trivial same-directory form now that `Cargo.toml` and `📦️lib.rs` are
siblings:

```toml
# before (sandwiched):
[lib]
path = "../../../../../../../✏️s/🔌️plugins/<p>/🧩️extensions/<e>/⚡️implementations/🦀️rust/📦️lib.rs"
# after (de-sandwiched):
[lib]
path = "📦️lib.rs"
```

`pkg/` (the wasm-pack output dir, generated + gitignored) moves along with the crate directory
automatically since it's a child of `⚡️implementations/🦀️rust` — no separate handling needed. **Do
NOT try to relocate `pkg/` to the extension's owner root** to satisfy the general Shape V2 rule that
generated dirs live at owner root, not inside `📦️packages` — the hygiene note in the master doc's
"Deletions / hygiene" section explicitly grandfathers `pkg/` as an already-accepted, already-gitignored
generated-output convention distinct from the general `🤖️generated/` rule, and relocating it would
require editing the shared `runWasmPackWebBuild` framework helper (which every wasm-bindgen crate in the
repo calls) — squarely out of scope for a single-extension ticket; flag it instead if you think it's
worth revisiting repo-wide.

---

## 2. The npm-facing package.json — fold it into the SAME language folder, don't invent a TS package

An extension's `package.json` usually has no real TypeScript SOURCE behind it — it exists purely to
expose the wasm-bindgen build's generated JS/wasm output as an npm-resolvable dependency (`"exports":
{ ".": "./…/pkg/….js" }`). Since Normative Shape V2 rule (a) explicitly lists `package.json` alongside
`Cargo.toml` as packaging-code manifests `📦️packages` is allowed to hold, and there is no actual `.ts`
source file to justify a sibling `📦️packages/🟦️typescript/`, **do not create a TS package for this** —
either:

- **(preferred, what flow did)** delete the hand-maintained wrapper `package.json` entirely and point
  the workspace entry directly at `wasm-pack`'s own AUTO-GENERATED `pkg/package.json` (it already has
  the correct `name`/`main`/`module`/`types`/`files` fields — `wasm-pack build` writes it fresh on every
  build, so there is nothing for a hand-written wrapper to add). This is strictly simpler — one fewer
  manually-maintained file that can drift from what the build actually produces — and it's exactly how
  flow's own `🫀️core` wasm crate already does it (grep root `package.json`'s workspaces array for
  `…/pkg` entries pointing at other already-migrated wasm crates before assuming this is a novel choice).
- If your extension's hand-maintained `package.json` carries something `wasm-pack`'s auto-generated one
  can't (extra `dependencies`, a `sideEffects` override, additional `exports` subpaths) — keep it, but
  move it into `📦️packages/<rust-lang-folder>/package.json` alongside `Cargo.toml`, not into a
  separate `📦️packages/🟦️typescript/`.

**Preserve the published npm/crate name exactly, in every case** (`@semio-tech/<original-name>`,
`semio-s-plugin-<...>-extension-<...>`, `semio:<original-component-name>`) — these are runtime lookup
keys (component-model package id, npm import specifier other in-flight code may already reference,
wasm-bindgen's exported symbol names). Nothing about the physical directory move should touch any
identifier string.

---

## 3. Registrar handoff — TWO files, both off-limits to you

Exactly like a plugin crate merge, you cannot touch either of these — end your report with both blocks:

```
Remove from root Cargo.toml members:
    "✏️s/🔌️plugins/<p>/🧩️extensions/<e>/⚡️implementations/🦀️rust",
Add:
    "✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust",
```

```
Remove from root package.json workspaces (if it was a glob covering the OLD path shape and no longer
matches, e.g. a single-level "🧩️extensions/*" glob that assumed package.json sat directly under it):
    "✏️s/🔌️plugins/<p>/🧩️extensions/*",
Add (one of, matching whichever §2 branch you took):
    "✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust/pkg",        # if you deleted the hand wrapper
    "✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust",            # if you kept a hand wrapper
```

**Check whether the existing workspaces glob already covers your new depth before assuming an edit is
needed** — a plugin with multiple extensions might already have a glob shaped for the post-move depth
if another extension in the same plugin was migrated first.

---

## 4. Cargo package.metadata — copy verbatim, add nothing

`[package.metadata.component]` (component-model package id) and `[package.metadata.semio]`
(`contributes`/`consumes`/whatever the extension declares) move byte-for-byte. Do **not** add a
`role = "extension"` key even though the plan's target discovery contract eventually wants one — that
role enum lands with the W4 mechanism wave's `🔣️taxonomy.json`/discovery-lib rewrite, not per-extension;
adding it early would be inventing a contract key ahead of the mechanism that's supposed to define it.

---

## 5. Verification sequence

Same chicken-and-egg problem as TEMPLATE.md §3 (the new crate isn't a workspace member until the
registrar acts, and root Cargo.toml may ALSO be red for an unrelated reason — an in-flight concurrent
plugin migration elsewhere in the repo can leave `cargo metadata` broken regardless of anything you did).
Use the identical temporary `[workspace]` verification overlay:

| # | Command | Notes |
|---|---|---|
| 1 | `cargo test --manifest-path <new>/Cargo.toml --lib` (with overlay) | must match your §0.3 baseline exactly — a pure structural move, zero test-count drift |
| 2 | `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --manifest-path <new>/Cargo.toml --target wasm32-wasip2` (with overlay) | this one is genuinely slow cold (~10+ minutes pulling in the full transitive framework graph fresh) — don't background it; raise the Bash timeout to the max (600s) and if it still doesn't finish in one call, let it run to completion rather than assuming failure, checking back on it rather than re-issuing the command |
| 3 | `DEVELOPER_DIR=/Library/Developer/CommandLineTools bun ./📜️script.ts wasm` (in the new package dir, with overlay) | the REAL build — runs the actual `wasm-pack build`, regenerates `pkg/` from scratch. This is also your cheapest way to confirm no stray old-named build artifacts survive the move (see §5.1) |
| 4 | delete the overlay + the nested `target/`/`Cargo.lock` the isolated workspace created | same as TEMPLATE.md §8.3 — do this before handoff, every time, no exceptions |

### 5.1 A regenerated `pkg/` is naturally self-cleaning

If the extension's crate was ever renamed (its Cargo `name`/`wasmBaseName` changed at some point in its
history) while `pkg/` still held output built under the OLD name, you'll find two parallel sets of
generated files sitting side by side (e.g. `flow_extension_bim.js` AND
`semio_s_plugin_flow_extension_bim.js` in the same `pkg/` dir) — dead, gitignored, harmless, but
genuinely stray. Running the real `wasm-pack build` (step 3 above) with `--out-dir pkg` **overwrites the
current name's files but does NOT delete files under an old name that no longer gets written** — so
don't assume a successful build alone cleans this up. Diff `ls pkg/` before/after: if stale
differently-named files remain post-build, delete them by hand (they're 100% regenerable, gitignored,
never referenced by the current Cargo.toml/script.ts once you've confirmed nothing greps for the bare
old name anywhere in-tree).
