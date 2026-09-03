# 🩹 FE0F Fix: `◻2d` → `◻️2d` (fem / puzzle / block artifacts)

## Scope

Fix the missing U+FE0F variation selector on the `◻2d` artifact-directory name
(SSOT: `🔣️taxonomy.json` → `semanticDirectoryMemberKinds["members-of-artifacts"].memberNames`
declares `"◻️2d"` with FE0F) for the three plugins:

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d`

## Directory rename — already done at session start

By the time this session began, `.🧬semio/…/UNIFIED-ARTIFACT-NAMING-AND-DEDUPLICATION/rename_fe0f_dirs.py`
had already been run (by an earlier turn on this same ticket): it renamed every
`◻2d`/`📄txt`/`📰xml` directory repo-wide (112 renames, logged in
`🗑️generated/dir_rename_log.json`, all status `RENAMED`, none skipped). All three
target directories already carried the correct `◻️2d` name on disk when this
session started (verified with `os.listdir`, and `git status` shows them as `R`
renames pending commit). No `mv` was needed from this session.

Confirmed no directory anywhere in the repo is still named exactly `◻2d`
(defective), and no nested file/dir inside the three artifact trees carries the
defect in its own name — only the reference text inside files needed fixing.

## Reference fixes performed this session

All matching/replacement was done with python3, matching the exact codepoint
sequence (U+25FB not immediately followed by U+FE0F, followed by `2d`) —
never hand-typed into a shell command.

Two passes:

1. **Prefix-anchored pass** (`🗿️artifacts/◻2d` → `🗿️artifacts/◻️2d`) — unambiguous
   since only fem/puzzle/block have this literal path segment. Applied
   repo-wide, excluding `node_modules/target/dist/.git` and excluding
   `.🧬semio/` + `.cursor/` (historical ticket/plan records — left untouched,
   see below). A first attempt at this (full `os.walk`, no candidate-list
   pre-filter) timed out at 2 minutes on the huge tree but had already made
   most of the progress before being killed; a second, `rg`-driven pass on the
   remaining candidate files finished the rest.
2. **Bare-mention pass** — files already established (by pass 1, or by manual
   inspection) as being about one of the three plugins' own `◻2d` artifact
   still had bare `` `◻2d` `` mentions in comments/imports (e.g. a 3D or 5D
   sibling file's docstring referencing its 2D counterpart, or a
   `export * from ".../windows/◻2d/…"` re-export). Fixed all of these,
   per-file, with an assertion on expected match count.

### Final counts (git diff vs HEAD, occurrence-level, excluding directory-rename-only paths)

| Plugin | Files touched | Occurrences fixed |
|---|---:|---:|
| `🏗️fem` | all files under `🗿️artifacts/◻️2d` + siblings referencing it | 188 |
| `🧩️puzzle` | same | 235 |
| `🧱️block` | same | 175 |
| **Plugin-own subtotal** | | **598** |

Cross-reference fixes (files outside the plugin's own artifact tree that
literally reference one of the three renamed dirs):

| File | Occurrences fixed |
|---|---:|
| `.storybook/stories/puzzle/2d/Fixtures.stories.tsx` | 3 |
| `✏️s/🔌️plugins/🔒️policy-allowlist.json` | 8 (fem `🗿️artifacts/◻2d` entries) |
| `📜️script.ts` (root) | 7 (fem `🗿️artifacts/◻2d` entries + 1 bare `` `🏗️fem/◻2d` `` prose mention) |
| `✏️s/🔌️plugins/🗄️stdio/…/🏭️generator/📜️script.ts` | 1 |
| `🧰️framework/…/📺️renderer/…/⚛️react/🧪️index.test.ts` | 2 |
| `🧰️framework/…/🗣️dsl/🧪️fixture-sweep/🦀️.rs` | 1 |
| `🧰️framework/…/🔌️plugin/🧵️retained-command/🧪️fixtures/🔣️scalar-config-cohort.json` | 3 |
| `✏️s/🔌️plugins/🏛️architect/…/🚪️io/🦀️.rs` (bare `` `🧱️block/◻2d` `` prose) | 1 |
| `🧰️framework/…/🧪️test/📦️packages/🟦️typescript/🟦️.ts` (bare `` `fem/◻2d` `` prose) | 1 |
| **Cross-reference subtotal** | **27** |

**Grand total: 625 references fixed** (598 + 27). This is higher than the
ticket's ~539 estimate because it also captured sibling-artifact (3D/5D)
docstring/import mentions of `◻2d` and cross-plugin/doc references not
counted in that estimate.

### Deliberately left untouched (and why)

- `🔒️dependencies.json` entries in `✏️s/🔌️plugins/🔒️policy-allowlist.json` (4
  remaining) reference `🎛️apps/◻2d` under `gis`, `fem`, and `block` — that
  `apps` directory does not exist in any of those plugins (verified). Stale,
  pre-existing dead references unrelated to the `artifacts/◻2d` rename; fixing
  the FE0F there would not make the path resolve, so left as-is.
- `.storybook/scopes.ts` (`🎛️apps/◻2d`) and
  `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🦀️.rs` (self-referential `🎛️apps/◻2d`
  docstring) — same stale-`apps` reason.
- `✏️s/🔌️plugins/🌍️gis/…/✏️editor/🦀️.rs` (`` `◻2d window's renderer` `` prose) and
  `🧰️framework/…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
  (`` `◻2d/**` `` glob) — ambiguous/generic mentions that read as referring to
  the **framework** 2D module, not one of the three plugins; left alone.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — **not
  edited**, per explicit instruction (coordinator-owned SSOT; already has the
  correct `"◻️2d"` entry; it also uses `◻2d` bare in a slugification-example
  comment, out of scope).
- `.🧬semio/` (962 files) and `.cursor/plans/` — historical ticket
  logs/reports and old plan snapshots that captured the *pre-fix* on-disk
  state at the time they were written. Left verbatim as an accurate
  historical record; not live code or config.

## Framework module (`🧰️framework/🔨️modules/◻2d/`) — finding only, not touched

The framework module directory **already has the correct name on disk**:
`🧰️framework/🔨️modules/◻️2d` (confirmed with `os.listdir`, FE0F present). It
does **not** have the same defect and required no rename.

However, several files still reference it using the **old, un-FE0F'd**
spelling `🔨️modules/◻2d`, which is now a dangling/broken path since the real
directory has FE0F:

- `package.json`, `bun.lock`, `🔒️dependencies.json` (root)
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📋️project.json` and
  `.../🦀️rust/📋️project.json` (the module's own self-references)
- `🧰️framework/…/📚️library/📦️packages/🟦️typescript/🧫️fixtures/…/🔣️.json`
  (18 occurrences)
- `🧰️framework/…/🌊️flow/🖍️drawing/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs` (references gis's
  `🎛️apps/◻2d`, separately stale)
- `✏️s/🔌️plugins/🗄️stdio/…/💡️inferences/🦀️.rs` (×2, framework references)

Per instruction these were **not** touched — out of this ticket's artifact
scope, and another session is actively editing that tree. Flagging for the
coordinator/framework-module owner since three plain-text `Cargo.toml`s
(`✏️s/🔌️plugins/🖍️draw`, `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw`,
`🧰️framework/…/🌊️flow`) and the root `Cargo.toml` already got their
`🔨️modules/◻2d` references fixed independently (by a concurrent session, not
by this ticket's work) while this report was being written — so most of these
may already be resolved by the time this is read; the `package.json` /
`bun.lock` / `🔒️dependencies.json` / fixtures-json / drawing.rs / tiled-map.rs
ones were still stale as of this write.

## Codepoint assertions run

```
rg -P --hidden -g '!node_modules' -g '!target' -g '!dist' -g '!.git' \
   -l '◻(?!\x{FE0F})2d'
```
→ 987 files remain (962 historical `.🧬semio`/`.cursor` records, left
verbatim by design; 17 out-of-scope live files, see above and detailed lists
in this report — zero of them under the three plugins' own trees).

```
rg -P --hidden -g '!node_modules' -g '!target' -g '!dist' -g '!.git' \
   -l '◻️️2d'
```
→ **0 matches** — no double-FE0F anywhere in the repo.

Per-plugin defective-file count, before → after:

| Plugin | Before | After |
|---|---:|---:|
| fem | 41 | **0** |
| puzzle | 40 | **0** |
| block | 33 | **0** |

## Cargo check verification (wasm32-wasip2, RUSTC_WRAPPER="")

Crate names confirmed from each plugin's `Cargo.toml`:
`semio-s-plugin-fem`, `semio-s-plugin-puzzle`, `semio-s-plugin-block`.

**Could not get a clean `cargo check` for any of the three** as of this
writing — not because of this fix, but because a different, actively-running
ticket (`.🧬semio/…/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
confirmed via `git status` showing its files as currently uncommitted/modified,
and via file mtimes updating seconds before a retry) is mid-flight removing
`serde` `Serialize`/`Deserialize` derives from shared framework ID/wire types
(`SchemaId`, `ArtifactId`, `MutationId`, `ActorId`, `InteractionState`,
`DomainHover`, `frames::SelectionMode`, `SelectionMethod`, `MergeMode`, …)
across `semio-framework`, `semio-framework-os-kernel`,
`semio-framework-replication`, and `semio-framework-plugin` — crates every
plugin depends on transitively. This leaves the **whole workspace** in a
temporarily-inconsistent state regardless of which crate you check.

Evidence this is unrelated to the `◻2d` fix, gathered over 17 attempts across
~2.5 hours (initial queued wait was ~1 hour behind other sessions' cargo
locks, then repeated re-runs as the concurrent refactor churned):
- **Zero** error or warning, across every attempt, mentions `fem`, `puzzle`,
  `block`, `artifacts`, or `◻2d` in any form.
- `semio-s-plugin-puzzle` and `semio-s-plugin-block` were also tried and hit
  the **exact same** failure, same file, same line, at the same point in
  time — proof it's a shared-dependency problem, not plugin-specific.
- The error set visibly changed between nearly every retry (error counts
  observed: 45 → 15 → 12 → 78 → 50 → 15 → 6 → 12 (regression) → 3 → 2 → 4 → 8
  → 8, across `semio-framework-replication` / `semio-framework` /
  `semio-framework-plugin` in turn) — a live moving target, not a stable
  failure this fix caused. It got as close as **2 remaining errors** in
  `semio-framework-plugin` at one point before the concurrent session's next
  edit reintroduced failures earlier in the dependency chain
  (`semio-framework-replication`).
- The specific files erroring (`📡️wire/🦀️.rs`, `🎠️kernel/🦀️.rs`,
  `🛂️manifest/🦀️.rs`, `🕹️interaction/…/🦀️.rs`) are all listed as `M`
  (modified, uncommitted) in `git status` right now, and `📡️wire/🦀️.rs`'s
  mtime updated live during this session's polling.

**Supplementary static verification performed instead** (does not replace
the mandated cargo check, but independently confirms the reference-rewrite
itself is structurally sound):
- Every `#[path = "…"]` attribute in each plugin's main aggregator
  (`📦️packages/🦀️rust/🦀️.rs`) resolves to a real file on disk: fem 522/522,
  puzzle 1046/1046, block 913/913.
- Recursively, across every `.rs` file under all three plugins' `🗿️artifacts`
  trees: 2496 `#[path]` attributes total, only **5** missing targets — all 5
  are `../../../../../🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` from brand-new
  (`git diff` shows `new file mode`) `mutate-*-1` test fixtures that this
  session never touched, with an unrelated wrong `../` depth count (a
  different, pre-existing bug from whatever concurrent ticket created them —
  not a `◻2d`/FE0F issue, since the same wrong depth would exist regardless
  of the FE0F spelling of the `2d`/`3d`/`5d` segment in the path).
- All 2054 `.json` files under the three plugins' `🗿️artifacts` trees parse
  as valid JSON (no corruption from the sed-like replacements).

One retry (of ~25 total over ~3 hours) got far enough to actually start
compiling `semio-s-plugin-fem` itself — proving the `#[path]` chain into
`🗿️artifacts/◻️2d/**` resolves and rustc parses those files (only warnings,
e.g. unused imports, came from files physically inside `◻️2d/`). That attempt
then hit 147 errors, but every one of them is `dsl::JsonValue` vs `DslValue`
type mismatches, `ToValue`/`DESCRIPTORS` trait-impl gaps, and `Result<_,_> is
not a future` — occurring **symmetrically in both `◻️2d` and its `🧊️3d`
sibling** with identical error shapes (e.g. `E0046` missing `DESCRIPTORS` in
both `◻️2d/…/🎚️config/🦀️.rs` and `🧊️3d/…/🎚️config/🦀️.rs`;
`Fem2dSnapshot`/`Fem3dSnapshot` both "not a future" at the same call
pattern). `git status` shows
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
as a brand-new (`A`, not yet committed) file — matching the code-generator
scripts (`🔨️b3-emit-fem2d-rust.py`, `🔨️b3-emit-fem3d.py`) sitting in the
sibling ticket
`.🧬semio/…/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
— i.e. this is that ticket's in-progress `JsonValue`→`DslValue` codegen
migration for fem's 2D **and** 3D standards together, mid-flight, not
anything to do with the FE0F rename.

**Recommendation**: re-run the three `cargo check` commands below once the
`RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` and
`SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
tickets' in-flight work lands (or stabilizes):
```
cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-fem --target wasm32-wasip2 --message-format short 2>&1 | tail -30
cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2 --message-format short 2>&1 | tail -30
cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-block --target wasm32-wasip2 --message-format short 2>&1 | tail -30
```

## Files changed (this session)

- `.storybook/stories/puzzle/2d/Fixtures.stories.tsx`
- `📜️script.ts` (root)
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
- All files under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/**` and sibling
  fem files that referenced it (`🗿️artifacts/🧊️3d/**`,
  `📦️packages/🦀️rust/🦀️.rs`)
- All files under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/**` and sibling
  puzzle files that referenced it (`🗿️artifacts/🖐️5d/**`)
- All files under `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/**` and sibling
  block files that referenced it (`🗿️artifacts/🖐️5d/**`,
  `🗿️artifacts/🧊️3d/**`)
- `✏️s/🔌️plugins/🔒️policy-allowlist.json` (partial — fem entries only)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🏭️generator/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️fixtures/🔣️scalar-config-cohort.json`

(Directory renames themselves were performed by an earlier turn on this
ticket, not this session — see above.)
