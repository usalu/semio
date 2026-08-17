# TEMPLATE-UI.md — per-element extraction recipe

Written from the `Select` pilot (W2). Every step below was executed for real against the live
`ui-react`/`ui-wgpu`/`ui-tui` packages and verified with `typecheck`/`cargo check`; the gotchas are
things that actually broke, not hypothetical risks.

## 0. Preconditions

- You hold the lock on whichever monolith(s) you're cutting from (`📦️index.tsx` for react,
  `📦️lib.rs` for wgpu/tui) — one agent-session at a time per file, per the concurrency law in
  `📋️master.md`.
- Check `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE/🗺️element-inventory.txt`
  for your element's line range and classification. If it's listed as an "unmatched story" (needs
  manual mapping) or nested inside a large region like `⚙️Canvas`, locate it with `grep -n` for the
  component name / struct name first — the inventory only resolved 2 levels of `#region` nesting.
- **Before touching react's `📦️index.tsx`**: run the region-balance check once per session —
  `#region` opens must equal `#endregion` closes at column 0. The pilot found and fixed a real
  cascading bug (`🧭️ModeDockTabBar` missing its close, which silently misattributed every
  `#endregion` after it for 17,000+ lines). If your session's edit touches a region near where the
  balance was last verified, re-run the check; don't assume the file is still balanced from a stale
  memory of a prior pass.

## 1. React extraction

1. Find the region: `grep -n "^// #region <Name>$\|^//#region <Name>$"` in the barrel
   (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`). Note exact
   start/end line numbers (the `#region`/`#endregion` lines themselves).
2. Extract the FULL region body (including the `#region`/`#endregion` comment lines and any
   leading top-of-region comments) to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/<Element>/🟦️component.tsx`.
3. **Identify every external symbol the region uses that isn't a keyword/local/React built-in.**
   Grep the extracted text for capitalized identifiers (components/types) and lowercase call
   sites (`cn(`, `useX(`, adapter objects like `reactHostPort.`). For each, `grep -n` the barrel for
   where it's actually `export`ed/defined — it is almost never in the region you're extracting; it's
   scattered across `🔌️Adapters`/`🎼️Utilities`/`🐹️Element`/`🔌️Ports` or similar not-yet-extracted
   "core-candidate" regions (per the inventory's classification).
4. Write the new file's imports in two tiers:
   - **True third-party imports** (React, Radix primitives, etc.) — import directly from the real
     package, exactly as the barrel currently does.
   - **Everything else (still living in the barrel)** — ONE relative import from the barrel,
     explicitly marked:
     ```ts
     // 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
     // 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
     // dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf
     // file without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
     import { symA, symB, type TypeC } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
     ```
     (Path is repo-structure-dependent — compute it with `path.relative()` from the element dir to
     the barrel dir, don't guess the `../` count by hand.)
   - Do **not** invent a different pattern per element. Every leaf that still needs barrel symbols
     uses this exact marker so `grep -r "🚧️W3-interim"` is a reliable completeness gate for W6.
5. In the barrel, replace the removed region body with:
   ```ts
   // #region <RegionName>
   import { Sym1, Sym2, ... } from "../../../../🧱️elements/<Element>/🟦️component.tsx";
   export { Sym1, Sym2, ... };
   // #endregion <RegionName>
   ```
   **This must be `import` + `export`, never a bare `export { ... } from "...";`.** A pure re-export
   does not create a local binding — any OTHER code still living in the same barrel file that
   references your extracted symbols unqualified (very common; e.g. `<Select>` used inside some
   other still-inline component) breaks with `Cannot find name 'X'`. The pilot hit this on the very
   first extraction. Import-then-export is the only barrel pattern that keeps both external
   consumers and same-file internal consumers working.
6. Run `bun ./📜️script.ts typecheck` from the react package dir. Compare the error **set** (file +
   error code), not just the count, against the pre-extraction baseline
   (`🧪️export-snapshot-before.txt` doesn't carry typecheck output — snapshot your own "before" run
   if you want a clean diff). Zero new errors, zero errors in your new file = done. Pre-existing
   errors elsewhere in the barrel (there are ~96, all traced to framework-core's missing generated
   bindings, an unrelated `🖼️assets` module, and styling readonly-property errors — none of them
   yours to fix) are expected and must not shift in count.

## 2. wgpu extraction

wgpu's `widgets` mod is one big block (`pub mod widgets { ... }`, ~1200 lines) containing shared
generic enums (`WidgetNode<E>`, `ControlNode<E>`) that EVERY widget's variant lives in, plus each
widget's `render_*`/`measure_*` functions. **Only the render/measure function bodies move** — the
shared enums, `measure_widget`/`render_widget` dispatch, and small per-widget data structs
(`SelectItem`, `KeyValueEntry`, …) that the enums reference stay in `widgets.rs`-equivalent
(currently still inline in `📦️lib.rs`, not yet split into its own target-dir file — that's a
separate W3 task, not part of this recipe).

1. Find your functions: `grep -n "fn render_<name>\|fn measure_<name>"` in wgpu's `📦️lib.rs`.
2. **Map which top-level `pub mod` each dependency actually lives in** before writing anything —
   this file has ~22 top-level sibling `pub mod` blocks at crate-root scope (`chrome`, `draw`,
   `geometry`, `input`, `theme`, `widgets`, …), not one flat namespace. `grep -n "^pub mod "` to get
   the list, then for each helper your function calls (border/icon/text-drawing primitives, `Rect`,
   `HitTarget`/`HitKind`, `Level`, …), find which one contains it and note the fully-qualified
   `crate::<mod>::<item>` path. Everything the pilot's `Select` functions needed was already `pub`
   at the struct/fn level — check with `grep -n "pub (fn|struct|enum|const) <Name>"`, but you still
   need the *module* path, which requires locating the `pub mod` block the definition falls inside.
3. Write `🧱️elements/<Element>/🧊️component.rs`: `use crate::widgets::{...}` for siblings that stay in
   `widgets` (the shared `WidgetContext`, small per-widget structs, other `widgets`-local helper
   fns), `use crate::<mod>::{...}` for everything in a different top-level mod. Mark the extracted
   fns `pub(crate)` (not `pub(super)` — see wiring below, this is NOT nested inside `widgets`).
4. **Wire with a CRATE-ROOT-level `#[path]` declaration, never nested inside `widgets { }`.** Add,
   immediately before `pub mod widgets {`:
   ```rust
   #[path = "../../../../🧱️elements/<Element>/🧊️component.rs"]
   mod <element_lowercase>;
   ```
   then inside `widgets` mod's own top `use` block: `use crate::<element_lowercase>::{render_x, measure_x};`.
   **Do not** try `#[path]` on a module declared *inside* the `widgets { }` block itself — rustc
   resolves that path as if `widgets` had its own on-disk directory (`.../wgpu/widgets/<your-path>`)
   even though `widgets` is an inline block with no such directory, and the file read fails with "No
   such file or directory" no matter how many `../` you add, because that phantom `widgets/` segment
   genuinely doesn't exist on disk for the OS to traverse through. Top-level (crate-root) `#[path]`
   siblings don't have this problem — that's the wiring pattern to use everywhere in this file.
5. The original call sites inside `widgets` (e.g. `render_select(...)` called from `render_widget`'s
   match arms) need **no changes** — the `use crate::<element>::{...};` import brings the names into
   `widgets`' own scope exactly like any other import, so unqualified calls keep resolving.
6. Verify: `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-ui-wgpu
   --features engine` (the default-feature check alone won't exercise everything the render path
   pulls in). If `cargo check`/`cargo metadata` fails with an error about a totally unrelated crate
   path (a different plugin's `Cargo.toml` missing) — that's very likely a concurrent session's
   in-flight migration (`git status --porcelain | grep <that-plugin>` to confirm files are staged
   `D`/`M` for it), not your bug. Poll (`cargo metadata --no-deps` in a retry loop, 15–30s apart) or
   move on to independent work and come back — do not touch root `Cargo.toml` to work around it.

## 3. tui extraction

Same shape as wgpu, simpler: tui's `widget` mod (not `widgets`, singular) is the one place per-widget
`*State` structs and their `paint_*`/`*_on_key` functions live. No generic `WidgetNode<E>` enum layer
to worry about — tui widgets are concrete `enum WidgetState { Select(SelectState), ... }`, and only
the free `paint_x`/`x_on_key` functions move; the `*State` struct and its variant in `WidgetState`
stay in `widget` mod (shared vocabulary, same reasoning as wgpu's `SelectItem`).

1. `grep -n "fn <name>_on_key\|fn paint_<name>"` in tui's `📦️lib.rs`.
2. Map dependencies the same way as wgpu (`grep -n "^pub mod "` — tui has ~13 top-level mods:
   `geometry`, `theme`, `text`, `cell`, `ansi`, `event`, `scene`, `layout`, `widget`, `chrome`,
   `engine`, `backend`, `host`). `select_on_key`/`paint_select` only needed items from `cell`,
   `event`, `geometry`, `text`, `theme`, and the sibling `widget` mod itself (`SelectState`,
   `WidgetSignal`) — all already `pub`, confirmed by the fact that `widget` mod's own top `use`
   block already imports them (if `widget` can `use` it, so can your new crate-root sibling module).
3. Same crate-root `#[path]` wiring as wgpu — `mod <element>;` right before `pub mod widget {`, then
   `use crate::<element>::{fn_a, fn_b};` inside `widget`'s own top `use` block.
4. Verify: `cargo check -p semio-framework-ui-tui`. Note `cargo clippy -D warnings` on this crate
   currently fails on ~16 **pre-existing** lints in unrelated widget code (`checked_div`,
   `too_many_arguments`, `map_unwrap_or`) — not something this ticket introduced or is responsible
   for fixing; don't let it block your `cargo check` verification.

## 4. Cross-cutting lessons (apply to every wave, not just W2)

- **A Cargo-dependent-file sweep must not be scoped to `Cargo.toml` only.** When a Rust package
  moves, grep for its old path across `*.ts`/`*.tsx`/`*.js`/`*.cjs` too — `.storybook/main.ts`,
  `.storybook/scopes.ts`, every plugin's `vitest.config.ts`/`vite.config.ts` that aliases
  `@semio-tech/<pkg>` to a literal path, and any TS file that imports a Rust-side file directly (e.g.
  styling-TS re-exporting styling-Rust's `🎨️tailwind.config.ts`). The W1 pass on `styling` initially
  swept only `Cargo.toml` and missed 16 non-Cargo files referencing
  `🎨️styling/⚡️implementations/🦀️rust/🟦️vite-elements-assets.ts` and 5 more referencing
  `…/🎨️tailwind/🎨️tailwind.config.ts` / `…/📜️script.ts` / `…/🔣️tokens.json` — all silently broken
  until a second, file-type-unrestricted grep pass caught them. Always run:
  `grep -rl "<old-path>" --include="*.ts" --include="*.tsx" --include="*.js" --include="*.cjs"
  --include="*.json" --include="*.toml" . | grep -v node_modules | grep -v /target/`
- **When a package's own directory moves to a DIFFERENT depth, its self-referencing dependency
  paths must be recomputed from the NEW location, not patched by adding a fixed "+1" to the old
  string.** wgpu and tui both moved from depth 6 to depth 7, but their own dependency on `styling`
  (which stayed at depth 6) needed **4** "../" from the new location — not 3 (the old count) and not
  a naive "+1" guess either, because styling's OWN relative position to wgpu/tui changed by more
  than one level once `🎯️targets` was inserted. Compute every such path with
  `path.relative(newOwnerDir, targetDir)`, verify it resolves with `existsSync`, and only then write
  it — don't do the arithmetic by hand twice in the same file (the pilot did, and got it right the
  second time only because `cargo check`'s manifest-load error caught the first mistake immediately).
- **The historical "up-and-back-down" `#[path]` self-reference trick is dead weight once a crate's
  `lib.rs` and its manifest share a directory (no `src/` subdir).** Every instance found in this
  migration (`ui_wgpu`, `ui_tui`, `ui_styling`'s own `[lib] path`, plus `#[path]`-attributed
  `generated.rs`/`icon_name.rs` includes) simplified to a bare `path = "📦️lib.rs"` /
  `#[path = "🤖️generated.rs"]` once moved. Don't preserve the old trick's now-pointless traversal
  literally — simplify it in the same commit as the move; it's less code to get wrong next time.
- **Before extracting a React element via the `🚧️W3-interim` barrel-import pattern, grep the extracted
  text for barrel-`const`-typed symbols used OUTSIDE a function/component body** — module top level,
  including inside top-level object/array literals (demo fixtures, `cva(...)` calls, `Context.Provider`
  default values assigned to a top-level `const`). A `🚧️W3-interim` import of such a symbol is a genuine
  ES-module circular-import initialization-order bug: the barrel imports the element (to re-export it)
  and the element imports the symbol back from the barrel, so whichever module the loader reaches first
  in the cycle can see the other's top-level `const`/`export let` still in its temporal dead zone —
  `typecheck` is blind to this (it's a runtime-only failure), only `bun ./📜️script.ts test` catches it,
  by crashing the WHOLE barrel module at import time (`TypeError: undefined is not an object` /
  `Cannot access '<name>' before initialization`), not just the one test that exercises the symbol.
  Found 6 elements hit this via `reactHostPort` (`.forwardRef`/`.createContext`/`.memo` at module top
  level), 2 via `cn` (its dependency `twMergeUi`), 1 via `sceneHostPort`, 1 via `uiDataLabel` — see
  `📋️w0-status.md`'s "W3 follow-up" section for the full incident writeup. **Fix**: extract the symbol
  (+ minimal dependency closure) to its own `🧱️elements/🫀️core/<Name>/🟦️component.tsx` file with no
  import back into any element, barrel `import`-then-`export`s it from there (same rule as step 5 —
  never a bare re-export), and the affected element(s) import it DIRECTLY from the core file instead of
  via the barrel. If the symbol is an `export let` reassigned elsewhere in the barrel (a host-port-style
  "swap the implementation" pattern), add a `set<Name>()` setter function to the core file — an ES
  import binding can't be assigned to directly, so the barrel's reassignment site must call the setter
  instead of `=`. A symbol read only inside hooks/render-body code is unaffected and can stay on the
  normal `🚧️W3-interim` barrel-import path — only module-top-level reads need this treatment.
- **After creating (or inheriting) a barrel `🚧️W3/W4-interim` re-export region for an already-extracted
  source file, mechanically diff the source's real `export`ed names against the region's import/export
  list** (`grep -oE "^export (function|const|type|interface|class) [A-Za-z0-9_]+"` on the source,
  sorted, `comm -23` against the barrel region's imported-name list) — don't eyeball a 70+-name list. A
  partial re-export list type-checks fine (the barrel's own `import {...}`/`export {...}` line is
  syntactically and structurally valid even with names missing) but produces a plain `undefined` /
  `ReferenceError` at every downstream consumer, at USE time not at TDZ — a different failure signature
  from the circular-import bug above, easy to conflate but requires a different fix (add the missing
  name(s) to both the source's own `export` keyword if it was a bare `const`, and the barrel's
  import/export list) rather than a core-file extraction. Found in the renderer-engine-react barrel
  (`…📺️renderer/🧑️‍🎨️engine`): 5 of `ShellHelpers`'s 72 exports were simply absent from the barrel's
  `ShellHelpers` region, plus 2 more symbols (`DEFAULT_PANEL_WIDTH_PX`, `registeredPuzzle3dBrushMeshes`)
  that were bare `const` (missing `export` entirely) in the source file itself — see `📋️w0-status.md`'s
  "W4 follow-up" section for the full incident writeup.
