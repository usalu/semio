# Presence Schema Leaves — Verdict: All 10 Genuinely Empty

## Method

For each of the 10 target `👥️presence/🧬️schema/🟦️component.ts` leaves, the sibling `👥️presence/🦀️component.rs` (editor-level Rust presence, one directory up from the schema leaf) was located and read. Per CLAUDE.md, that Rust component is the source of truth for presence; every one of the 10 target artifacts turned out to have such a Rust counterpart, and **every one of the 10 Rust structs is itself `pub struct XPresence {}` with an explicit, evidenced doc comment explaining why it carries no fields** — not an unfinished stub. Mirroring field-for-field therefore means the TS interface must also stay empty. No editor-state fields were fabricated.

Filled reference schemas (`🪐️space/⚙️engine`, `📐️cad`, `💠️lowpoly`, `🧩️puzzle/🧊️3d`, `📏️layout`, `🌀️procedural/🧊️procedural3d`, `📸️remodel`, `🖨️raster`) were read first to confirm the expected shape (`/** @state presence */`-annotated fields, camera/selection/hover naming) — none of that shape applies here because the Rust source of truth says these artifacts route their would-be presence fields elsewhere (local config, or the framework's typed `PresencePeer.interaction`), not because the shape wasn't found.

## Per-file verdicts

All 10 got the same treatment: the empty `export interface XPresence {}` body was kept, and a definitive doc comment was added quoting/summarizing the Rust rationale plus a pointer back to the `.rs` source of truth.

1. **`✏️s/🔌️plugins/📕️norm/👥️presence/🧬️schema/🟦️component.ts`** — `NormPresence`.
   Evidence: `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️component.rs:1-12` — "every norm family app keeps its only view state in `crate::config::NormConfig` (`selected_check_index`); there is no separate shareable live surface to broadcast." Applies to all 15 norm standard apps (DIN/EN/ISO/VDI families).

2. **`.../📜️imperative/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `ImperativePresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-14` — step selection moved to the framework-owned `steps` interaction domain, broadcast via typed `PresencePeer.interaction` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); no other shareable field remains.

3. **`.../🏗️fem/🗿️artifacts/🧊️3d/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `Fem3dPresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-9` — selection is command-transient payload (not shared state); camera / result-display already live on `Fem3dConfig` as `local-ui`.

4. **`.../🏗️fem/🗿️artifacts/◻2d/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `Fem2dPresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-9` — identical rationale to fem/3d, mirrored for `Fem2dConfig`.

5. **`.../🌿️vcs/🗿️artifacts/🌿️vcs/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `VcsDemoPresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-6` — "the VCS play demo keeps all view state in `VcsDemoConfig`... history selection and locale are local config."

6. **`.../🎞️animate/🗿️artifacts/🎬️present/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `PresentPresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:8-11` — tile selection/hover now broadcasts via the framework's typed `PresenceInteraction` ("tiles" domain, `broadcast: true`, same first-class-hover-and-selection ticket); "this app has no OTHER app-specific ephemeral field left to carry."

7. **`.../📖️playbook/🗿️artifacts/📖️playbook/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `PlaybookPresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-8` — former `selected_ids` (peer selection on the block-list builder) moved into the framework's typed `PresencePeer.interaction` ("blocks" domain); no other shareable state.

8. **`.../📋️forms/🗿️artifacts/📋️forms/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `FormsPresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-4` — "forms has no multi-user shareable live state yet; blueprint and try wizard view state stays in `FormsConfig`."

9. **`.../🪐️space/🗿️artifacts/🏠️home/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `HomePresence`.
   Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-4` — "the home launcher keeps panel tab and locale in `HomeConfig`; there is no multi-user shareable live surface on the launcher."

10. **`.../➗️mathematical/🗿️artifacts/➗️mathematical/.../✏️editor/👥️presence/🧬️schema/🟦️component.ts`** — `MathematicalPresence`.
    Evidence: `.../✏️editor/👥️presence/🦀️component.rs:1-8` — "graph edits are document mutations and viewport/locale live in config"; no shareable live surface state yet.

Note: `🏗️fem`'s two TS leaves already carried a one-line "empty shareable live state" comment before this pass (not a bare `{}` as the ticket's generic description implied); the comment was expanded to state the concrete rationale (command-transient selection, config-owned camera) instead of just asserting emptiness.

## Contrast with the filled references

The filled schemas (cad, lowpoly, puzzle/3d, layout, procedural3d, remodel, raster, space/engine) are genuinely different artifacts: those editors keep camera/selection/hover state that has no other transport, so it lives directly on the presence struct with `/** @state presence */` per field. The 10 artifacts in this ticket instead either (a) keep all view state in a per-user local `*Config` that is not itself shareable, or (b) have already migrated their one shareable field (selection/hover) to the framework's generic `PresenceInteraction`/`PresencePeer.interaction` domain mechanism introduced by ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`, leaving nothing artifact-specific to declare here.

## Verification

Ran (real command, real output):

```
bunx tsc --noEmit -p /private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/tsconfig.presence-check.json
EXIT:0
```

The temp `tsconfig.presence-check.json` extends the repo root `/Users/ueli/Documents/semio/tsconfig.json` (same `strict`, `esModuleInterop`, `isolatedModules`, ESNext target/module) with `include` narrowed to exactly the 10 edited files, so it type-checks with the project's real compiler options without pulling in the whole multi-gigabyte workspace. Exit code 0, no diagnostics.

Consumer grep (via `rg`, excluding `node_modules`) for each of the 10 interface names — `NormPresence`, `ImperativePresence`, `Fem3dPresence`, `Fem2dPresence`, `VcsDemoPresence`, `PresentPresence`, `PlaybookPresence`, `FormsPresence`, `HomePresence`, `MathematicalPresence` — found **no other `.ts`/`.tsx` file in the repo referencing any of these type names**; each only appears in its own declaration file. Since the edits only added/expanded doc comments and never touched the (already-empty) interface bodies, there is no behavioral or type-level change to propagate, consistent with the empty consumer set.

## Unfinished / out of scope

Nothing outstanding for these 10 files. Not addressed (out of scope for this pass, flagged only for awareness): the sibling `.rs`, `.json` (JSON Schema), `.proto`, and `.graphql` presence-schema codecs for these same 10 artifacts were not inspected/touched — only the `🟦️component.ts` leaves named in the task were in scope. Other sessions are concurrently touching unrelated parts of the repo (per `git status` at task start); no interference observed with these files.
