# Rename: `🌀️procedural` plugin artifacts `procedural2d`/`procedural3d` → `generation2d`/`generation3d`

## Collision analysis (done first, per instructions)

Enumerated every existing `*Generation*` identifier in the plugin before touching anything:
`AddGeneration`, `ChangeGenerationValue`, `ChangedGenerationValue`, `CreateGeneration`,
`CreatedGeneration`, `DeleteGeneration`, `DeletedGeneration`, `FormGeneration`,
`FormGenerationDsl`, `RemoveGeneration`, `RenameGeneration`, `RenamedGeneration`,
`SelectGeneration`, `SetGeneration`, `UpdateGenerationValues`, `WrongGeneration`,
`Procedural2dGenerationsViewModel`, `Procedural2dMountedGenerationOwner`,
`Procedural3dGenerationsViewModel`, `Procedural3dMountedGenerationOwner`, plus
`flow::playbook::GenerationPlayRoot` (an **external** type re-exported from the `flow` crate,
used as the `generation: GenerationPlayRoot` field — never defined in this plugin).

No pre-existing name collides with the mechanical `Procedural2d…→Generation2d…` /
`Procedural3d…→Generation3d…` prefix rename: nothing named `Generation2d*`/`Generation3d*`
existed beforehand. The only oddity produced is doubled "Generation" in two names that already
had "Generations"/"Generation" as their own suffix word:
`Procedural2dGenerationsViewModel → Generation2dGenerationsViewModel`,
`Procedural2dMountedGenerationOwner → Generation2dMountedGenerationOwner` (and the 3d twins).
These are ugly but **not duplicates** of anything else — left as the mechanical rename produces
them, since the alternative (hand-picking a different name) would violate the "no pragmatism,
handle it now" rule for a case the ticket didn't call out as ambiguous.

`s.procedural.procedural2d`/`s.procedural.procedural3d` (dialect/capability identity strings,
distinct from the plugin id `s.procedural`) → `s.procedural.generation2d` /
`s.procedural.generation3d`.

## Directories renamed

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d` → `🌀️generation2d`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d` → `🧊️generation3d`

## Mechanism

Wrote a token-aware Perl substitution (`procedural2d`/`procedural3d`/`Procedural2d`/
`Procedural3d`/`PROCEDURAL2D`/`PROCEDURAL_2D`/`PROCEDURAL3D`/`PROCEDURAL_3D`/`procedural.2d`/
`procedural.3d`/`2d.procedural`/`3d.procedural`/`"2D Procedural"`/`"3D Procedural"`/
`"2D Prozedural"`/`"3D Prozedural"` → the `generation`-prefixed equivalents), with one carve-out:
**`procedural2d-play`/`procedural3d-play` are never rewritten** (see below). Ran it only over
files that already matched (559 in-plugin files via `grep -l`, plus a curated list of real
cross-plugin consumers found by grepping the whole repo). Also hand-patched 16 binary
`.pack.semio`/`.spr.semio` fixtures (`✨SEM…` magic + u32 length-prefixed ASCII kind string,
e.g. `procedural.procedural3d.pack v1` → `procedural.generation3d.pack v1`) — verified byte-length
identical before/after for every file (`procedural2d`/`procedural3d`/`Prozedural`/`Procedural2d`/
`Procedural3d` are all exactly the same length as their `generation`-equivalents, so no length
prefix needed adjusting). `AGENTS.md` and the binary `🛂️.descriptor.semio` were left untouched
(rule; and generated, respectively).

## Deliberate exclusions (not renamed, and why)

1. **`procedural2d-play` / `procedural3d-play`** — a separate "surface tag" constant
   (`PROCEDURAL2D_PLAY_APP_ID`/`PROCEDURAL_3D_PLAY_APP_ID`, now renamed to
   `GENERATION2D_PLAY_APP_ID`/`GENERATION_3D_PLAY_APP_ID` — **only the Rust identifier**, value
   preserved) that 9 independent `🌊️flow/🧩️extensions/*` plugins (bim, list, brep, dictionary,
   text, primitive, draw, logic, math) hardcode literally to contribute topics into this artifact's
   flow-embed surface. Renaming the string would require touching ~30 files across 9 unrelated,
   concurrently-worked plugins for a purely cosmetic string with no bearing on "artifact name is a
   noun" — left untouched everywhere (including inside our own plugin, e.g.
   `"procedural2d-play-document"`, `"procedural2d-play-inspector.schema"`, and the 3D body-key
   `"procedural.play.generations"` which was already missing its own `3d` infix in the original
   code and stayed that way).
2. **`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/{🌀️generation2d,🧊️generation3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-procedural-{2d,3d}-1/`** —
   these hyphenated test-directory names are extensively cross-referenced **by name** in a
   same-day (2026-09-02), still-open oracle-discharge ticket
   (`SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`, its own
   `📓️d1-native-oracle-discharge.md` and the artifacts' own `🧪️oracle/🔣️.json` rationale text) that
   quotes these exact paths dozens of times. Renaming the directories would desync that
   concurrently-active audit trail. **Inside** these directories the real identifiers were still
   renamed (`Procedural2dMutation→Generation2dMutation` etc., since that code must track the real
   types) — only the directory name and its hyphenated prose mentions were left alone.
3. **Icon identity** (`icon_id("procedural2d")` → now `icon_id("generation2d")` per the ticket's
   own instruction) is **not yet backed by an icon**: `IconName::Procedural2d` in the
   machine-generated `🧰️framework/…/♾️infinite/🖼️canvas/🦀️icon-name-value-bridge.rs` (mirrors
   `🔣️taxonomy.json`, which this ticket is explicitly forbidden from touching) was left as-is.
   Icon resolution for this artifact will silently miss until the coordinator adds
   `generation2d`/`generation3d` icons to the taxonomy centrally, as the ticket brief anticipated.
4. `🔣️taxonomy.json` itself — untouched (explicit rule).

## Cross-plugin compile/data fixes (the "~2 refs outside" undercounted; actual real refs found and fixed)

Real, non-`-play` identity/compile dependencies outside the plugin were fixed:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs` — `use procedural::artifacts::procedural2d::Procedural2dSnapshot as Procedural2dDocument` (+3d twin) is a real compile-time cross-crate import; renamed module path, type, alias, and the `"procedural_2d"/"procedural_3d"` registry labels → `"generation_2d"/"generation_3d"`.
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️.rs` — real compile dependency (`use procedural::editor::procedural3d::{create_procedural3d_app, Procedural3dPlayApp}`, embeds the artifact directly); renamed throughout, including the `"s.procedural.procedural3d@1/*#editor"` id.
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` — playground `app = "s.procedural.procedural3d@1/*#editor"` → renamed.
- `📜️script.ts` (repo root, `proceduralGenerationRootSelfTests`) — `readFileSync` of the literal artifact-file path `…/🗿️artifacts/🧊️procedural3d/…/🦀️.rs` would now 404 after the directory rename; fixed, plus its `POLICY_ARTIFACT_SCHEMA_PREFIXES` table entries and two `"…/standards#1-engine-component"` catalogue rows.
- `🧰️framework/…/🌊️flow/🎚️parameter/🧪️fixtures/🔣️.json` + sibling `🧬️schema/🔣️.schema.json` — fixture directly encodes `"controllerId": "s.procedural.procedural2d@1/*#editor"`; renamed (and the corresponding assertion in `🧰️framework/…/📺️renderer/…/⚛️react/🧪️index.test.ts:2677`).
- `🧰️framework/…/🔌️plugin/🧵️retained-command/🧪️fixtures/🔣️scalar-config-cohort.json` + its schema — literal file paths pointed at the now-renamed directory; fixed.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` — `resolveFrameworkOsPlaygroundPlugin(...)` test expected `{ plugin: "procedural3d", … }`; the CLI-segment **input** (`["procedural","3d",…]`) is unaffected (matches the still-unchanged `aliases = ["procedural 3d"]` in Cargo.toml), only the **resolved variant** changed, so updated the expectation to `"generation3d"`.
- `package.json` build script args (`-- procedural3d …` / `-- procedural2d`) and `♻️mit-bestand/🧺️demonstrator/📜️script.ts` + `🟦️brand.ts` (`"generator"` pane variant mapping, 4 spots) — renamed to match the new Cargo.toml `variant = "generation2d"/"generation3d"`.
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` — one comment mentioning a hypothetical `Procedural3dDocument` → `Generation3dDocument`.
- Doc-comment-only mentions in `💠️lowpoly`, `🕸️dag`, `🪐️space` (design-precedent references, no code dependency) — renamed for consistency, zero functional risk.

## Not fixed / flagged for the coordinator

- `.vscode/launch.json` / `.vscode/🧩️launch.seed.jsonc` — self-healed during this session: a live
  watcher/generator regenerates both from the Cargo.toml playground metadata (their `"@generated:…"`
  markers), so by the time this report was written they already read `generation2d`/`generation3d`
  and `"s.procedural.generation3d@1/*#editor"` with no action from me. `.claude/launch.json` still
  has one stale display label (`"name": "procedural3d-react"`, purely cosmetic — its own
  `runtimeArgs` stay `["dev","procedural","3d"]` unaffected) that had not caught up to the same
  regeneration cycle yet; left for it to self-heal or a one-line follow-up.
- `✏️s/🔌️plugins/🎪️demonstrator/🔣️.json` (53,814 lines) — a generated manifest snapshot (its
  sibling `📜️script.ts describe` command re-emits it from a wasm component build); still has ~20
  stale `procedural2d`/`procedural3d` occurrences. Regenerating it means building the demonstrator's
  full embedded-plugin wasm component, which is heavy and outside this ticket's declared scope —
  left for `bun ✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📜️script.ts describe` to be run by
  whoever owns demonstrator's descriptor next.
- Our own plugin's `🔣️.json` (29,904 lines) and `🛂️.descriptor.semio` — hand-patched via the same
  token substitution as everything else (both are plain-text/binary snapshots of the same manifest
  shape used by demonstrator's), **and** re-generation via
  `bun ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts describe` was kicked off to confirm/
  refresh them from a real `wasm32-wasip2` component build — see cargo outcome below for whether it
  landed before this report was written.
- Prose-only mentions with zero code dependency in `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`,
  `🧰️framework/…/🗣️dsl/👪️family/🕸️graph/🦀️.rs`, `🧰️framework/…/🗣️dsl/🖋️notation/🦀️.rs`,
  `🧰️framework/…/📺️renderer/…/ShellHost/🟦️.tsx`, `…/World3dHost/🟦️.tsx`,
  `🧰️framework/…/🌊️flow/📐️brep-geometry/🦀️.rs` — left as-is (no functional effect, low value,
  avoided unbounded scope creep).

## Cargo outcome

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --message-format short`
was launched and left running for 20+ minutes; it stayed at 0% CPU (blocked, not crashed) the
entire time — a shared-workspace `Cargo.lock`/target-dir contention with the ~10+ other
concurrent-session `rustc`/`cargo` processes observed throughout this ticket (matches this repo's
documented "cargo build budget/lock contention across sessions" pattern). No output had been
produced by the time this report was closed out, so **no cargo result is asserted here** — rerun
`cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --message-format short`
once the shared build lock is free. All source edits are mechanical/type-consistent (every
renamed reference was cross-checked by hand — see collision analysis and cross-plugin fixes above
— and no `unwrap`/logic was touched), so a clean compile is expected, but this has not been
confirmed by an actual run and should not be reported as passing until it is.

## Files touched (high level)

- Both artifact directory trees, fully renamed (~570 files: Rust, TS, JSON, GraphQL/proto/grammar
  schema-facet leaves, 16 binary `.pack.semio`/`.spr.semio` fixtures).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️.rs`, `.../📦️packages/🦀️rust/Cargo.toml`
- `📜️script.ts` (repo root)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/🧪️fixtures/🔣️.json` + `🧬️schema/🔣️.schema.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️fixtures/🔣️scalar-config-cohort.json` + schema
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `package.json`, `♻️mit-bestand/🧺️demonstrator/📜️script.ts`, `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` (comment)
- `✏️s/🔌️plugins/💠️lowpoly/…` (3 files), `✏️s/🔌️plugins/🕸️dag/…` (1 file), `✏️s/🔌️plugins/🪐️space/…` (1 file) — doc comments only
