# 📖️ Semio Grammar/Protocol Identity Fix — `.semio` header terminal

## Defect

272 raw grep hits for `stdio.json`/`stdio_json` in `*.semio` files under `✏️s` (excluding the real
`🗄️stdio/🗿️artifacts/🔣️json` artifact):

```
rg -n -e 'stdio\.json' -e 'stdio_json' --glob '!node_modules/**' -g '*.semio' "✏️s" | grep -v '🗄️stdio/🗿️artifacts/🔣️json'
```

Two declaration shapes, both wrong everywhere they appear outside the real `stdio.json` artifact:

- `📖️component.grammar.semio` line 7: `header = "schema" SP "stdio.json" NL`
- `📡️component.protocol.semio` line 4: `schema stdio.json`

## Discrimination

Of the 272 raw hits, **6 lines (4 files)** are legitimate prose comments inside `🗄️stdio`'s own
`📝️md` and `🖊️dxf` artifacts, referencing the real `stdio.json` artifact for comparison (e.g. `#
same shape \`stdio.json\`'s own diff`, `# \`stdio.json\`'s own snapshot protocol gives its opaque
RFC8259 payload`). Those artifacts declare their own correct schemas (`stdio.md`, `stdio.dxf`) and
were matched only because the *comment text* mentions `stdio.json`. Left untouched. Verified by
checking the exact declaration patterns only:

```
grep -c ':header = "schema" SP "stdio\.json" NL$'   → 129
grep -c ':schema stdio\.json$'                        → 137
129 + 137 = 266 (the real fix count); 272 − 266 = 6 (the excluded comment lines)
```

**266 files fixed.**

## Slug source per directory

For each of the 266 declaration lines, the directory's own `🟦️component.ts` docstring
(`` /** ... for `<slug>` ... */ ``) supplied the ground-truth slug — **265 of 266** resolved this
way. `🅰️component.g4` (`DOCUMENT: 'schema' [ ]+ '<slug>' ;`) was the planned fallback but was never
needed (every directory that has a `.semio` declaration also has a `.ts` sibling with the docstring,
except one).

The one exception: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/…/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
has **no** `.ts` or `.g4` sibling in its directory at all. Derived by convention instead:
- The file's own `protocol space.mutations` line (already correct — artifact.facet, unchanged) gives artifact=`space`, facet=`mutations`.
- The artifact folder `🗿️artifacts/🪐️space` is self-named (artifact name == plugin name), exactly like `🕸️dag/🗿️artifacts/🕸️dag`, whose confirmed-correct sibling slug is `dag.dag.snapshot` (doubled prefix).
- The artifact's own `🦀️component.rs` `artifact_kind()` confirms this is the space plugin's self-referencing index artifact (`id: "space.sspace"`, `SPACE_INDEX_DIALECT.artifact_kind: "s.space.space"` — the `s.` here is the *app*-level namespace, a different axis from the schema slug, consistent with every other confirmed slug never carrying an `s.` prefix).
- Applied slug: **`space.space.mutations`**, following the `dag.dag.snapshot` doubling precedent.

Every other directory's slug came straight from its sibling `.ts`/`.g4` — no other derivation was
needed. Examples spot-checked: `norm.din18599.snapshot`, `trinity.rewrite.diff`, `dag.dag.snapshot`,
`block.block2d.diff`, `space.home.diff` (confirms the `space.home` divergence noted in the brief),
`puzzle.puzzle5d.mutations`, `gis.gismap.diff`, `fem.fem3d.mutations`.

## Counts by plugin (266 total)

| Plugin | Fixed | Plugin | Fixed | Plugin | Fixed |
|---|---|---|---|---|---|
| 📕️norm | 90 | 🎥️shooting | 6 | 🎬️sequence | 5 |
| 🧱️block | 18 | 🎞️animate | 6 | 🏭️process | 5 |
| 🧩️puzzle | 15 | 🌿️vcs | 6 | 📜️imperative | 5 |
| 🏗️fem | 12 | 🌊️flow | 6 | 💠️lowpoly | 4 |
| 🌀️procedural | 12 | ✒️writer | 6 | ➗️mathematical | 4 |
| 🔱️trinity | 10 | 🕸️dag | 6 | | |
| 🌍️gis | 8 | 🪐️space | 6 | | |
| 🪵️sourcing | 6 | 🔋️energy | 6 | | |
| 📸️remodel | 6 | | | | |
| 💡️reasoning | 6 | | | | |
| 🏛️architect | 6 | | | | |
| 🎪️demonstrator | 6 | | | | |

## Verification

**1. Cross-file agreement** (scratch script `verify_slugs.py`): for every one of the 266 corrected
files, re-read the file's own header/protocol terminal after the edit and compare against (a) the
expected slug computed from the sibling, (b) the sibling `.ts` docstring slug (when present), (c)
the sibling `.g4` terminal slug (when present).

```
PASS: 266 / 266
MISMATCHES: 0
```

**2. Re-grep for stray `stdio.json` self-declarations** repo-wide:

```
rg -n -e '^header = "schema" SP "stdio\.json" NL$' -e '^schema stdio\.json$' -g '*.semio' "✏️s" \
  | grep -v '🗄️stdio/🗿️artifacts/🔣️json'
→ (empty, exit code 1 / no matches)
```

Broad re-grep (any mention at all) still returns exactly the same 6 legitimate comment lines in
`🗄️stdio`'s own `md`/`dxf` artifacts — nothing else.

**3. `git status` scope check** — the 266 changed files, and only those:

```
git status --porcelain -- <266 paths>
→ 266 lines, all " M" (modified, tracked, nothing staged/untracked/added)
```

**4. Runtime grammar/protocol conformance test.** Found the real validator: `semio-framework-os-kernel`
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`), feature `dsl-fixture-sweep-full`, module
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`:
- `m5_handcrafted_grammar_conformance::all_discovered_snapshot_grammars_recognize_their_shipped_fixtures`
  auto-discovers **every** `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` under `✏️s/🔌️plugins`,
  parses it with the repo's own hand-written `.grammar.semio` parser (`crate::os_dsl::parse_grammar`),
  compiles a `Recognizer`, and asserts it recognizes the shipped `.dsl.semio` example fixture text —
  this is the direct runtime consumer of the exact terminal string I edited.
- `m5_handcrafted_protocol_conformance::all_discovered_snapshot_protocols_walk_their_shipped_fixtures`
  is the binary-protocol equivalent (`verify_protocol_source` + `walk_protocol`) over every
  `📡️component.protocol.semio`.

Command run (foreground, isolated `CARGO_TARGET_DIR` in the scratchpad to avoid queuing behind
~60 cargo processes from other concurrent sessions sharing the repo's `target/`):

```
CARGO_TARGET_DIR=".../scratchpad/semio-grammar-target" \
cargo test -p semio-framework-os-kernel --features dsl-fixture-sweep-full m5_handcrafted_grammar_conformance -- --nocapture
```

**STATUS: could not run — the target does not compile, for reasons entirely unrelated to this
change.** Real, actual output (72 `error[...]`, 71 in the fixture-sweep module):

```
error[E0433]: cannot find module or crate `block` in this scope
  --> .../🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs:32:9
error[E0433]: cannot find module or crate `cad_document` in this scope
  --> .../🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs:35:9
error[E0433]: cannot find module or crate `draw` in this scope
error[E0432]: unresolved import `dag_app`
error[E0432]: unresolved import `fem2d`
error[E0432]: unresolved import `fem3d`
error[E0433]: cannot find module or crate `norm` in this scope   (×several — one per Document alias)
... (same shape for gis, home, imperative, layout, lowpoly, mathematical, note_app, playbook,
    present, procedural, process_3d, puzzle, raster, reasoning_mindmap_plugin, remodel,
    semio_framework_os, sequence, shooting, sourcing, space, trinity, vcs_app, writer)

error[E0308]: mismatched types
   --> .../🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs:360:28
360 |                         if has_semio_under(&assets) {
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found future
    = help: consider `.await`ing the Future

error[E0277]: `impl Future<Output = Vec<std::path::PathBuf>>` is not an iterator
   --> .../🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs:357:29
357 |                 for slug in example_slug_dirs(&examples) {
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `.await`

error: could not compile `semio-framework-os-kernel` (lib test) due to 71 previous errors; 64 warnings emitted
```

Root cause, confirmed two ways:

1. **Missing dev-dependencies.** The fixture-sweep module (`🧰️framework/🛍️products/💻️os/🔨️modules/
   🗣️dsl/🧪️fixture-sweep/🦀️component.rs`) `use`s ~30 plugin crates (`block`, `cad_document`, `dag_app`,
   `draw`, `fem2d`, `fem3d`, `norm`, `gis`, `home`, `trinity`, `writer`, …). The owning crate's
   `[dev-dependencies]` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:101`) declares
   **exactly one** crate (`semio-framework-async-macros`) — none of the plugin crates the module
   imports are wired in. The feature cannot compile as committed, regardless of any `.semio` content.
2. **Missing `.await` calls** on several async fns in the same module (`example_slug_dirs`,
   `has_semio_under`, `slug_has_legacy_kind_dirs`, `fixture_files.len()`) — matches this repo's
   already-known, already-recorded async-convention debt (project memory:
   "Semio Async Convention Debt — AGENTS.md:44 left ~53k async fns, 88% non-suspending; repo
   didn't build at 95b8688ee2").

Confirmed pre-existing, not caused by this task: `git status --porcelain` on both files
(`Cargo.toml` and the fixture-sweep `🦀️component.rs`) is **clean** — they match `HEAD` exactly,
last committed 2026-08-28 and 2026-08-22 respectively, days before this session started. This
session's own change set touches **only** `.semio` files (verified: `{'semio'}` is the sole
extension across all 266 changed paths) — no `.rs`/`.toml` file was touched, so this compile
failure cannot be a consequence of the slug fix.

**Sanity check that the base crate itself is healthy** (i.e. that nothing about my `.semio` edits —
which only feed `include_str!` constants, never parsed at compile time — broke the build):
`cargo test -p semio-framework-os-kernel --lib` (no feature flag, same isolated target dir)
**compiles clean** and runs; it aborts on an unrelated pre-existing failure
(`os_pack::value::tests::retained_value_vm_rejects_truncation_utf8_depth_and_counts` — a binary
pack-VM value-truncation test, a completely different subsystem, SIGABRT via `panic in a destructor
during cleanup`). This is further evidence of an unrelated in-progress/unstable area of the repo,
not something introduced here.

Given the intended validator cannot build, `m5_handcrafted_protocol_conformance` (the binary-protocol
sibling test in the exact same crate/file/feature) was not attempted separately — it would hit the
identical missing-dev-dependency compile wall.

**What this means for confidence in the actual fix:** the cross-file agreement check (slug in the
edited file's own terminal == slug in its sibling `.ts`/`.g4`, 266/266 pass) and the re-grep (zero
stray self-declarations of `stdio.json` remain) are the verifications that actually ran and passed.
The intended deeper runtime check — the repo's own hand-written grammar/protocol parser actually
recognizing shipped fixtures against the corrected header terminals — exists in the codebase and is
exactly the right test, but is environmentally blocked by a pre-existing, already-documented defect
several waves outside this ticket's scope (missing dev-dependency wiring + async-convention debt in
`🧪️fixture-sweep`). Flagged as a separate concern below rather than fixed here, since repairing ~30
missing `[dev-dependencies]` entries and dozens of missing `.await` calls across the fixture-sweep
harness is a substantial, unrelated body of work.

## Anything unfinished

The `.semio` grammar-identity fix itself (266 files) is complete and cross-verified. Not fixed here,
and explicitly out of scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`
+ its owning `Cargo.toml`'s `[dev-dependencies]` are broken pre-existing (not by me, not by a
concurrent session's in-flight edit — confirmed clean at HEAD), which means the actual runtime
grammar/protocol conformance sweep could not be executed this session. Flagging that separately
rather than repairing it under this ticket.
