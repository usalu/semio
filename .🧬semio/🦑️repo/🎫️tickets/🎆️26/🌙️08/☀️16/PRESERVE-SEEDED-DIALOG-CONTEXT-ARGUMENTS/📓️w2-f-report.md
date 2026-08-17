# Lane 2-F report — `PresenceBar` element

## What was built

`🧱️elements/👥️PresenceBar/` — a compact horizontal roster element, React + wgpu twins, following
`📻️TableAvatar`/`📊️Table`'s co-location structure exactly (no new pattern invented):

- `🟦️component.tsx` — hand-built React component (same tier as `Table`/`TableAvatar`, not routed
  through the generic plugin-surface `Interpreter`). Reuses `TableAvatar` per peer, `role="list"` /
  `role="listitem"` semantics, keyboard-focusable (`tabIndex={0}`), each peer carries
  `data-row-id="peer:{actor}"`, the bar carries the caller-supplied `id`. Overflow past `max`
  (default 5, exported as `PRESENCE_BAR_DEFAULT_MAX`) collapses into one `+N` chip
  (`data-row-id="peer:overflow"`). Empty state renders localized copy. Name + role show via native
  `title` (hover) and `aria-label` (always, incl. keyboard focus for assistive tech). Deterministic
  per-actor colour via `presenceHueForActor` (FNV-1a hash of the actor id's UTF-8 bytes → HSL hue).
- `🧊️component.rs` — wgpu twin. Builds the same roster shape as a plain `UiNode` tree
  (`UiStackNode` root, one child `UiStackNode` per peer carrying `id: "peer:{actor}"`, trailing
  overflow node) using the crate's existing declarative `component::ui` builders — no new `UiNode`
  variant, no edit to the shared `component.rs`. `presence_hue_for_actor` is a byte-for-byte mirror of
  the TS hash. Localized copy (empty state, "more") resolved via `LocalizedLabel::native(en, de)`.
  Gated on the light `wgpu` feature (declarative types only), not `wgpu-engine` — it never needed the
  `WidgetContext`/`chrome` immediate-mode renderer its neighbours (`Button`, `KeyValue`, `Ring`, …)
  extract into, since it was never inline `widgets`-mod code to begin with.
- `🧪️story.tsx` — Storybook stories (Default / Overflow / Empty), same shape as `TableAvatar`'s.

### `data-ui-path` finding (requested by the W0 scout)

Confirmed the scout's suspicion: React `📊️Table` rows emit **only** `data-row-id`, never
`data-ui-path`. `data-ui-path` genuinely exists only as the wgpu↔React parity join for the generic
declarative-surface renderer, `💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx`
— it's computed there (`uiNodePathSegment`/`uiChildPath`, `type[index]#id`) from a `UiNode` tree's own
node `id`s, not emitted by hand-built elements like `Table`/`TableAvatar`, and not produced by the
`elements/*/🧊️component.rs` "widgets" chrome renderers either (`Button`, `KeyValue`, `Ring`, …have no
notion of it — they're a completely separate immediate-mode system with `WidgetContext`/`HitTarget`,
no serialization). `PresenceBar`'s own React DOM always carries real `data-row-id`s regardless of
mounting strategy; the Rust twin's `UiStackNode.id: "peer:{actor}"` choice means that **if** a future
shell embeds `PresenceBar`'s wgpu form through the generic `Interpreter` path, each peer
automatically gets a matching `data-ui-path` ending `.../stack[i]#peer:{actor}` for free, with zero
extra machinery on either side.

## i18n

Added `ui.presence.{roster,empty,overflow,role.author,role.spectator}` — type in
`🧱️elements/📚️I18n/🟦️component.tsx` (append-only), strings in both `uiChromeTranslationBundles.de`
and `.en` inside `⚛️react/📦️index.tsx` (append-only, immediately after the existing `conflict` block
each side lane 2-A/2-B seeded this ticket with). No hardcoded English; per-peer *names* are runtime
data (`peer.label`), never a translation key, matching the rest of the codebase's `Label::data` vs
`app_labels!`/i18n-key distinction.

## Changed files

- New: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️component.tsx`
- New: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs`
- New: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️story.tsx`
- Edited (append-only): `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx` — `ui.presence` schema block
- Edited (append-only): `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` —
  de+en `presence` bundles, `PresenceBar` barrel import/export region, `describe("PresenceBar", …)`
  in-source vitest block
- Edited (append-only): `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` —
  `#[cfg(feature = "wgpu")] pub mod presence_bar;` mount + crate-level re-export

**Not touched** (other lanes are visibly already scaffolding presence mount points — confirmed via
`git status`, not mine): `✏️s/🔌️plugins/🌊️flow/…/✏️editor/👥️presence/🦀️component.rs`,
`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`, the new `👥️presence/📌️empty.md` stub files under
the space plugin's editor/viewer subsets, and the Interpreter/os `component.ts` files another lane has
open. Not wired into any shell, per the brief.

## Commands run + results (real tail pasted)

**TypeScript** — `bun nx run @semio-tech/ui-react:test` (project name confirmed from its
`📋️project.json`) ran fine, no nx hang. It shells out to `bun ./📜️script.ts test` →
`bunx vitest run --config 🧪️vitest.config.ts` (in-source tests, `📦️index.tsx`):

```
Test Files  1 failed (1)
     Tests  10 failed | 515 passed (525)
```

None of the 10 failures are `PresenceBar` (`UnifiedGumball math`, `icon hover animations`,
`CanvasPickMenu`, `Shell components`, `tree helpers` ×2, `VirtualFileSystem` ×4 — all pre-existing,
unrelated files I never touched). Filtered run, `bunx vitest run --config 🧪️vitest.config.ts -t
"PresenceBar"`:

```
Test Files  1 passed (1)
     Tests  5 passed | 520 skipped (525)
```

Ran the full suite twice more back-to-back to check determinism: both times exactly the same 10
pre-existing failures, 515 passed, **zero** `PresenceBar` failures. (First attempt at the locale test
used a full React `render()` + `await changeLanguage`, which *was* flaky under the full 525-test
suite — cross-test contamination from the pre-existing broken tests' uncaught async exceptions, not a
bug in `PresenceBar`. Rewrote it to mirror the existing `ui.ribbon.parent.*` coverage test's own
pattern — read `uiI18n.t()` directly instead of round-tripping a render — after which it was stable
across every rerun.) Logs: `🧪️2-f-vitest-full.txt`, `🧪️2-f-vitest-presencebar.txt`.

Also ran `bun ./📜️script.ts typecheck` out of caution: **187 pre-existing TS errors**, entirely
outside anything I touched (`🛂️manifest/🟦️component.ts`, `💻️os/🟦️component.ts` duplicate
`DirectoryEvent`, `🧱️elements/🪵️Tree`, `📊️Diagram`, `🔣️Icons`, …) — zero errors in `PresenceBar`'s own
files or in the two lines I added to `I18n`. Log: `🧪️2-f-typecheck-prexisting.txt`.

**Rust** — crate confirmed `semio-framework-ui` (from `📦️packages/🦀️rust/Cargo.toml`).
`cargo check -p semio-framework-ui --lib --features wgpu` (the crate's own `📜️script.ts`'s canonical
test feature set is `tui-terminal,wgpu`, but `wgpu` alone is what my module needs):

```
Finished `dev` profile [unoptimized] target(s) in 0.17s
```

Clean, no warnings from my file. Log: `🧪️2-f-cargo-check.txt`.

`cargo test -p semio-framework-ui --lib --features "tui-terminal,wgpu" presence` — **blocked**: the
whole `semio-framework-ui` lib-test binary fails to *compile* (5 `E0277` errors), entirely inside the
shared `🎯️targets/🧊️wgpu/🦀️component.rs`'s own `#[cfg(test)] mod tests` (lines 4771/4845/4846:
`Label: From<&str>` no longer satisfied, `NodeGraphScene::base("[]".into(), …)` no longer typechecks)
— a file I never touched, not in my lease. `git log --date=iso -1` on that file's line range shows it
was last substantively touched 2026-08-14 08:39 (commit `2420304f`), two days before this ticket
opened, so this is pre-existing breakage, not caused by any 2026-08-16 lane; confirmed with the exact
canonical command from the crate's own `📜️script.ts` (`--features tui-terminal,wgpu`), same 5 errors.
Not mine to fix (giant shared file, many lanes' leases). I did **not** claim these tests pass — I
wrote and manually reviewed 4 `#[cfg(test)]` unit tests in my own file
(`presence_hue_for_actor_is_deterministic_and_in_range`,
`build_presence_bar_renders_one_stack_child_per_peer_under_max`,
`build_presence_bar_collapses_past_max_into_one_overflow_node`,
`build_presence_bar_empty_peers_renders_localized_empty_text`) but cannot report them as passing
until whoever owns `component.rs`'s test module fixes it — re-run
`cargo test -p semio-framework-ui --lib --features "tui-terminal,wgpu" presence` once that lands.
Log: `🧪️2-f-cargo-test-presence.txt`.

## sharedFileRequest

None strictly blocking my lease — all edits were append-only inside files already listed as mine
(`⚛️react/📦️index.tsx`'s "identity / binding / routing / presence / check-in" region,
`🖱️ui/🧱️elements/👥️PresenceBar/**`). Flagging for whoever owns
`🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`: its own `#[cfg(test)] mod tests` (around
lines 4771–4846) doesn't compile under the crate's canonical `--features tui-terminal,wgpu`, which
blocks running **any** test in `semio-framework-ui --lib`, mine included.

## Not done

- Not wired into either shell (explicitly out of scope this wave — lanes 2-C/2-D/3-A consume it next).
- No visual (sighted) on-keyboard-focus name reveal beyond the native `title` tooltip (hover) +
  `aria-label` (always, screen readers); deliberate simplification rather than inventing a new
  tooltip/popover pattern not already used by a neighbouring element.
- Rust `presence` unit tests are written but unverified-passing (compile-blocked by pre-existing,
  unrelated breakage in the shared `component.rs`, see above).
