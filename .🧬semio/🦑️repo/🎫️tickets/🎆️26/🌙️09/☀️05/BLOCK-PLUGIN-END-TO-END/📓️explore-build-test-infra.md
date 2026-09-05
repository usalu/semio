# `🧱️block` plugin — build, test, descriptor & registry infrastructure

Scope: `✏️s/🔌️plugins/🧱️block` (crate `semio-s-plugin-block`, manifest
`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`, entry
`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs`, nx project `@semio-tech/block-plugin`).

**Timing note.** At the start of this exploration `git status` showed ~1006 staged changes in
`✏️s/🔌️plugins/🧱️block` (the emoji-rename sweep named in the task). Partway through, a concurrent
agent **committed** that sweep as `02db159aee21995b` (`git log -1 --date=iso -- "✏️s/🔌️plugins/🧱️block"`
→ `2026-09-05 03:53:30 +0200`). Everything below reflects the **current on-disk/working-tree state**
(post-commit), which is what actually matters for correctness. Two files remain unstaged with
unrelated, in-progress work from another agent (retained-tool-job wiring in block2d's editor) —
ignored per instructions:
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`

---

## 1. `#[path]` mount integrity

**Script**: walked the real crate entry
`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs:1` (2707 lines — this is the file `[lib] path = "🦀️.rs"`
in `Cargo.toml:33` actually points at; NOT the top-level `✏️s/🔌️plugins/🧱️block/🦀️.rs`, which is
the non-constitutional shared kernel it `#[path = "../../🦀️.rs"] mod block_shared;`s in at
`📦️packages/🦀️rust/🦀️.rs:23`) plus every `.rs` under `✏️s/🔌️plugins/🧱️block/🗿️artifacts/**`
(2707-line entry + artifact tree = **919** `#[path = "..."]` occurrences total), resolving each
literal relative to its containing file's directory.

**Result: 917/919 resolve to a real file on disk. The 2 that don't are NOT a block-specific
regression** — they're an identical, pre-existing pattern shared verbatim by `cad`, `puzzle` and
`procedural`:

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-block-3d-1/🦀️.rs:40`
  — `#[path = "../../../../../🗄️stdio/🧪️oracle/⚖️law/🦀️.rs"]` (5 `..`) resolves (naively, from the
  file's own directory) to a non-existent
  `.../🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`. Nearest real anchor going
  up from the leaf is `🏅️standards/🔖️1/`; the real target is
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` (needs 8 `..`, not 5).
- Same file/line shape at
  `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-block-5d-1/🦀️.rs:40`.

Cross-checked: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/💎️mutate-puzzle-3d-1/🦀️.rs:40`,
`.../📐️cad/🗿️artifacts/📐️cad/.../🌻️mutate-cad-1/🦀️.rs:40` and 2 `🌀️procedural` sites use the
**exact same 5-`..` literal** and are **equally unresolvable** by naive on-disk join, while
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/.../🧭️mutate-lowpoly-1/🦀️.rs:40` uses 9 `..` and
*does* resolve on disk. These `🧪️tests/<mutate-case>/🦀️.rs` files are **generated-test-host
adapters** (`use semio_repo_test_host::{...}`, doc-commented "a generated test host may not gain a
Cargo dependency on another plugin's crate") — `semio-repo-test-host`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml`) and the sibling
`semio-s-plugin-stdio-test-oracle` crate
(`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml:1-2`: *"Own workspace root: this
crate is linked by generated cache-local test hosts by path and must not join the repository
workspace"*) are assembled into a **cache-local synthetic crate** at test-run time (observed under
`.🧬semio/🦑️repo/⚡️cache/agents/*/cargo-test-hosts/`), where the `#[path]` is relative to the
*generated* location, not the source tree. So this literal-disk check isn't the right validator for
this one mount category — it's a template-copy artifact identical across 4+ plugins, not something
the block rename sweep broke.

**Rename-mount consistency (the actual ask): clean.** Grepped the crate entry + artifact tree for
every OLD name from the task's example list — `🎨️set-active-example`, `📄️txt`, `🟪️stl`,
`🎬️hexagonal-cut-concrete-forest-right` — **zero hits** outside of the unrelated `🗄️stdio/🧪️oracle`
literal (stdio's own directory name, never renamed). The entry file's mounts already read the NEW
names, e.g.:
- `📦️packages/🦀️rust/🦀️.rs:2368` → `#[path = ".../✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]`
- `📦️packages/🦀️rust/🦀️.rs:2362` → `.../✏️editor/🎮️commands/🚫️remove-compatibility-rule/🦀️.rs`
- `📦️packages/🦀️rust/🦀️.rs:2366` → `.../✏️editor/🎮️commands/🗑️remove-handle-kind/🦀️.rs`
- `📦️packages/🦀️rust/🦀️.rs:2688` → `.../📚️examples/➡️hexagonal-cut-concrete-forest-right/🦀️.rs`

The `🧪️oracle → 🔮️oracle` rename (`◻️2d`, `🧊️3d`, `🖐️5d`, all three subsets) is not `#[path]`-mounted
by any `.rs` at all — it's pure data (`🔮️oracle/🔣️.json`) — but it matters for §3 below.

## 2. Descriptor plumbing

**What `describePluginComponent` emits** (`🧰️framework/.../🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:287-296`):
builds `packageName`'s `wasm32-wasip2` component, extracts its core module, then calls
`emitPluginDescriptor` → the `semio-framework-plugin-describe` binary's `describe` subcommand, which
writes **`🛂️.descriptor.semio` + `🔣️.json` straight into `ownerRoot`** — doc comment at
`📜️script.ts:283-284`: *"NOT `🤖️generated/`, which is gitignored"* — these two files are meant to be
**tracked**.

**Block's own wiring is correct and matches the convention.** `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📜️script.ts:14-18`:
```
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-block", join(this.root, "..", "..")));
  }
}
```
`join(this.root, "..", "..")` from `📦️packages/🦀️rust` resolves to the plugin owner root
`✏️s/🔌️plugins/🧱️block` — identical shape to every migrated plugin's own `describe` command. **The
gap is that `describe` has simply never been run/committed for block**: `find
✏️s/🔌️plugins/🧱️block -maxdepth 1` shows no `🔣️.json` / `🛂️.descriptor.semio`, while every other
sampled plugin owner root has both (`🧩️puzzle`, `📐️cad`, `🌀️procedural`, `🕸️dag`, `🌊️flow`).

**`🪪️manifest/` and `🧪️oracle/` at owner root are NOT a universal convention** — the task's framing
("sibling plugins have... at their owner root") overstates it. `🪪️manifest/` only exists on
`🎪️demonstrator` (a special bundler plugin whose `🪪️manifest/🎪️demonstrator/🦀️.rs` assembles *six
foreign plugins'* surfaces into one bundle — irrelevant to block) and on `🌊️flow` (its own thing).
`🧪️oracle/` at owner root doesn't appear on `🧩️puzzle`/`📐️cad`/`🌀️procedural`/`🕸️dag` either. So
block's absence of these two is normal; the real, universal gap is **only** the missing
`🔣️.json`/`🛂️.descriptor.semio`.

**The registry's `check` already treats this as non-fatal.** `📇️registry/📜️script.ts:1968-1975`
documents the severity as *deliberately asymmetric*: "missing descriptor" is a **warning**, only a
descriptor that exists-and-is-wrong is a hard error. Exact code path
(`📇️registry/📜️script.ts:1983-1987`):
```
const descriptorPath = join(entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
if (entry.hashes === undefined && entry.executionMode === undefined && ...) {
  warnings.push(`${entry.pluginId}: no ${descriptorPath} yet — run \`bun ./📜️script.ts describe\` in ${entry.cratePath} after building its wasm32-wasip2 component`);
  continue;
}
```
Block will trip exactly this warning today (`descriptor gate: N/M crates have a 🔣️.json.` at
`📜️script.ts:2030` will count block among the "missing" side), never an error.

**Block IS already correctly wired into the generated catalog rows**, driven by `Cargo.toml`'s
`[[package.metadata.semio.playground]]` entries (`📦️packages/🦀️rust/Cargo.toml:19-31`). Confirmed
present, byte-for-byte, in the tracked generated files:
- `🧰️framework/.../📇️registry/🤖️generated/🎠️playgrounds.json:97-150` — three rows,
  `variant: "block2d"/"block3d"/"block5d"`, `pluginId: "block"`,
  `cratePath: "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust"`, ports `6024-6026`/`6124-6126`, block3d
  carries the `mesh-collection` asset from `[[package.metadata.semio.assets]]`.
- `🧰️framework/.../📇️registry/🤖️generated/🎮️playgrounds.ts:27-29` — same three rows, TS form.
- `🧰️framework/.../📇️registry/🤖️generated/🔌️plugins.json:55` — `"pluginId": "block"` entry.

(Note: the task named these files `🤖️generated/🎮️playgrounds.ts` / `🔣️playgrounds.json` — the actual
sibling of `🎮️playgrounds.ts` is `🎠️playgrounds.json`, carousel not `🔣️`, `🔣️.json` is reserved for
the per-crate descriptor filename in §2/§3.)

## 3. Test infrastructure

**`runCargoTestBudgeted(["semio-s-plugin-block"])`** (`🧰️framework/.../🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1635-1645`
onward) resolves the package name, then runs `cargo nextest`/`cargo test -p semio-s-plugin-block`
(nextest preferred, `llvm-cov` variant if coverage is on) scoped to the current `SEMIO_TEST_LEVEL`
(`--skip <level>::` for levels above the active one, `fundamental` gets a `--test-threads` cap). This
only exercises what's actually compiled **into the `semio-s-plugin-block` crate** — i.e. the entry's
own `#[cfg(test)] mod surface_tests` (`✏️s/🔌️plugins/🧱️block/🦀️.rs:245-273`, 6 async tests: 3
dimensions × {viewer-never-mutates, editor/viewer-share-dialect}) plus every `#[path]`-mounted
`🧪️tests/<fixture>/🦀️.rs` under the mutation/example trees that mount directly into the crate
(the per-mutation `🔺️diff`/`↩️inverse`/fixture tests, e.g. `📦️packages/🦀️rust/🦀️.rs:2362-2368` mounts).
It does **not** run the standalone `🧪️tests/🧩️mutate-block-{2d,3d,5d}-1/🦀️.rs` adapters — those link
`semio_repo_test_host` and are compiled as separate generated-test-host crates (§1), invoked by the
repo's test-generation harness, not by `cargo test -p semio-s-plugin-block`.

**Test-function count in the whole block tree**: `930` total (`3` plain `#[test]` + `927`
`#[semio_framework_async_macros::async_test]`, `0` `#[tokio::test]`) via
`grep -rE '#\[(tokio::)?test\]|#\[semio_framework_async_macros::async_test\]' --include='*.rs'`.

**Files per subset** (`✳️any`, all 3 dimensions `◻️2d`/`🧊️3d`/`🖐️5d`):
- `🧪️tests/**`: one `🧩️mutate-block-{2d,3d,5d}-1/` fixture case each, plus one `🧪️tests/` dir per
  mutation (~35 mutations × 3 dimensions) and per example (`🎬️demo-session`,
  `🎬️hexagonal-cut-concrete-forest-left`, `➡️hexagonal-cut-concrete-forest-right`, `🏢️nakagin-capsule`).
- `.feature` files: exactly 3 — one per `🧩️mutate-block-<dim>-1/🥒️.feature`.
- `.py` files: exactly 3 — one per `🧩️mutate-block-<dim>-1/🐍️.py` (python adapter).
- `🔮️oracle/🔣️.json`: exactly 3 — `◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`,
  same for `🧊️3d`, `🖐️5d`.

**Discovery is taxonomy-driven and filename-exact** —
`🧰️framework/.../🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:75-108` loads the frozen contract from
`🔣️taxonomy.json` (`testsDirName: "🧪️tests"`, `testFixturesDirName: "🧫️fixtures"`,
`testAdapterFileKinds: {"🦀️rust": "rust-source", ..., "🐍️python": "python-source"}`,
`testContributionDirName: "🔮️oracle"`, `testCaseSlugPattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$"`). Every
kind is looked up by its **exact canonical emoji-prefixed filename**
(`testFilenameForKind`/`testLocationPath`, `🟦️.ts:143-159`) — this is precisely why memory warns
discovery silently returns 0 on filename drift: a directory or file one emoji off the taxonomy's
`fileKinds` entry just never matches the glob, no error. The repo also enforces a **global**
`pathEmojiPolicy` (`identity: "single-emoji-grapheme"`, every git-visible file/directory needs a
leading single-emoji identity) in `🔣️taxonomy.json`. Read together with `testContributionDirName:
"🔮️oracle"` (not `🧪️oracle`), the block-owned `🧪️oracle → 🔮️oracle` rename in §1 is a **real,
necessary discovery fix**: before it, block's three `✳️any/🧪️oracle/🔣️.json` contributions used a
name that no longer matches the taxonomy's contribution directory name, so they would have been
silently invisible to oracle-contribution discovery. Confirmed post-rename: all three subsets now sit
at `.../✳️any/🔮️oracle/🔣️.json`, and no stray `.../✳️any/🧪️oracle` references remain anywhere in the
block tree. Likewise, the ~90 test-case directories that gained a leading emoji in the rename batch
(e.g. `renames-node-kind-to-gate` → `✏️renames-node-kind-to-gate`, `removes-in-handle` →
`🚫️removes-in-handle`) are compliance fixes for the same global `pathEmojiPolicy`, not renames
block invented on its own.

## 4. TypeScript package — `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/package.json`

Confirmed copy-paste from `cad-js`: `name` is `@semio-tech/block-js`, but:
- `description`: `"📐️ CAD plugin TS: spatial factory runtime/model graph (core), R3F renderer,
  brepjs kernel, construct query language, XState machine adapter, and the runtime composition
  root — folded from the 6 former cad-js-* packages."` — pure CAD boilerplate, block has none of
  these concepts.
- `scripts`: `"test": "bun nx run @semio-tech/cad-js:test"`, same for `generate`/`fixture` — all
  three point at the **cad** nx target, not `@semio-tech/block-js`.
- `dependencies`: `@semio-tech/cad-js-module-spatial-shape`, `@semio-tech/cad-js-module-aec-building`,
  `@semio-tech/cad-js-module-aec-building-energy`, `@semio-tech/cad-js-module-aec-building-structure`
  — CAD-specific modules with no relation to block.
- Correctly customized: only `repository.directory` (`✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript`).

**Important correction to the task's framing**: the two "sibling" examples it names,
`🧩️puzzle`/`📦️packages/🟦️typescript/package.json` and
`🌀️procedural`/`📦️packages/🟦️typescript/package.json`, are **byte-identical copies of the same
cad-js template** (same description, same three `cad-js:*` scripts, same 4 cad-js-module deps, only
`name`/`repository.directory` swapped) — as is `🕸️dag`'s and `🎪️demonstrator`'s. This is a
repo-wide scaffold artifact affecting at least 5 plugins, not something specific to block, and
those two named siblings are **not** good models to copy from.

The one sibling that **is** correctly customized: `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/package.json`:
```json
"description": "flow · TypeScript module surface (compute thread-pool helpers)",
"semio": { "role": "plugin", "id": "flow-js" },
"dependencies": { "@semio-tech/ui-react": "workspace:*" }
```
— own description, no leftover `scripts` block (relies on nx defaults), a `"semio"` marker block,
and a minimal, real dependency list. Fixing block's `package.json` should follow flow's shape: own
description, drop/rewrite the `cad-js:*` scripts to target `@semio-tech/block-js` (or remove them if
nx defaults suffice, per flow's example), and replace the 4 cad-js-module deps with whatever block
actually needs (its `📦️packages/🦀️rust/Cargo.toml` shows no WASM/R3F rendering deps beyond the
plugin framework itself, so block likely needs far fewer TS deps than either template).

## 5. Storybook

`.storybook/scopes.ts` has **zero** literal references to `block` — no `HAND_CURATED_SCOPES` entry.
Block relies entirely on the generated opt-in mechanism via `Cargo.toml`'s
`[package.metadata.semio.storybook]` (`📦️packages/🦀️rust/Cargo.toml:14-18`: `id = "block"`,
`titlePrefix = "🧱️block"`, `sourceRoots = ["."]`). Ran `buildGeneratedScopes` live (`bun` against
`.storybook/scopes.ts`) and confirmed it produces exactly:
```json
{ "id": "block", "titlePrefix": "🧱️block", "sourceRoots": ["✏️s/🔌️plugins/🧱️block"] }
```
— the opt-in resolves correctly (`discoverPackages` finds the manifest, `readSemioMarkerSubTable`
reads the table, `sourceRoots` is non-empty so it doesn't throw).

**But the scope has no stories.** `Cargo.toml` doesn't set `storyGlobs`, so
`buildScopeStoryGlobs` (`.storybook/scopes.ts:265-269`) falls back to the default derivation
`./stories/${s.id}/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)` → for block, `.storybook/stories/block/**`.
That directory **does not exist** (`find .storybook/stories -iname '*block*'` matches only an
unrelated `framework/hosts/BlockListHost.stories.tsx`). By contrast `🧩️puzzle`'s Storybook coverage
is a **hand-curated** entry (`.storybook/scopes.ts:85`, `id: "puzzle"`) — `puzzle`'s own `Cargo.toml`
has no `[package.metadata.semio.storybook]` table at all — backed by real files at
`.storybook/stories/puzzle/{2d,3d,5d}/*.stories.tsx` (4 files). So block's Storybook registration is
config-only right now: the scope exists and resolves cleanly, but until someone adds
`.storybook/stories/block/**/*.stories.tsx` (or sets an explicit `storyGlobs` in `Cargo.toml`
pointing wherever block's stories should live), Storybook will show the "🧱️block" scope with zero
stories in it.

## 6. Stale artifacts

`🧰️framework/.../🧑‍💻dev/🔌️plugin-modules/🧱️block/*` mtimes (`stat -f %Sm`):
- `semio_s_plugin_block_component.js` — **2026-09-04 17:20:36** (newest generated artifact)
- `🌉️bridge.js` — 2026-09-01 22:58:42
- `🟨️.js` — 2026-09-01 17:03:33
- `semio_s_plugin_block_component.core.wasm`, `semio_s_plugin_block_component.d.ts`, and all
  `interfaces/*.d.ts` — 2026-08-18 20:48:47 (two: `semio-framework-plugin.d.ts`,
  `semio-framework-contributor.d.ts` — 2026-08-17 13:41:56)

`git log -1 --date=iso -- "✏️s/🔌️plugins/🧱️block"` → **2026-09-05 03:53:30 +0200** (commit
`02db159aee21995b`, the just-landed rename sweep).

**All generated dev-plugin-module artifacts predate the latest committed block source** — the
newest one (`semio_s_plugin_block_component.js`, 2026-09-04 17:20) is ~10.5 hours older than the
2026-09-05 03:53 commit, and the WASM core/interfaces are ~2.5 weeks stale (2026-08-17/18). This is
consistent with §2's finding (no `describe` run since the rename sweep): the whole
build→describe→publish pipeline for block hasn't been re-run since these changes landed, so both
the tracked descriptor pair (§2) and this cache-local dev bridge/component are out of date relative
to current source and need a rebuild (`bun ./📜️script.ts describe` in
`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust`, then whatever regenerates
`🧑‍💻dev/🔌️plugin-modules/🧱️block`).
