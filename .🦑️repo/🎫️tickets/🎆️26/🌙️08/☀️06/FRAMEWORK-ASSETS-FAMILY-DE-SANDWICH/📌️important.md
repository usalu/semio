# 📌️ Important — read before touching `🖼️assets/` again

**Status as of this session: de-sandwich DONE (own tree), font bug root-caused (not fixed — owned by
`ui-wgpu`, out of scope), consumer fixes documented in `📋️registrar-handoff.md` (not applied — out of
scope per ticket constraints, confirmed by `FRAMEWORK-SURFACE-FAMILY-CRATE-CONSOLIDATION`'s identical
precedent).**

- New shape: `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/` (Shape V2, `role = "framework"`,
  `id = "assets"`). All data dirs (fonts, icons, metabolism, compose, logos, cursor, introduction, list,
  badge, images, mesh) live at the owner root as siblings of `📦️packages/`. `AGENTS.md`/`README.md`/
  `LICENSE.md` also moved to the owner root. The `⚡️implementations/🟦️typescript` sandwich is deleted.
- `@semio-tech/logos` and `@semio-tech/icons` (zero external consumers each) were folded into the main
  package's `📜️script.ts` (`generate-logo`/`export-logo` subcommands); a third orphan wrapper,
  `🖼️images/package.json` (`@semio-tech/images`, also zero consumers), was found and deleted too.
- Fixed a genuine `#region`/`#endregion` imbalance in `📜️script.ts` left over from the logo-codegen
  merge (the `🚀️Commands` sub-region under `🔖️CatalogUiCodegen` was never closed). Verified
  `bun build` succeeds on both `📦️index.ts` and `📜️script.ts` (the only build error left is an
  unrelated pre-existing `playwright-core`/`chromium-bidi` optional-dependency bundling quirk,
  reproduced from a clean isolated file — not caused by this ticket).
- **Font bug root cause, confirmed**: `10-400.ttf`/`11-400.ttf` were never missing files — both always
  existed at `🖼️assets/🔤️fonts/😀️noto-emoji/`. The bug is 2 stale relative-path lines in
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️text.rs` (a `ui`-owned file, out
  of this ticket's exclusive ownership) — see `📋️registrar-handoff.md` §5 for the exact 2-line fix and
  why it wasn't applied directly (that file is under active concurrent editing by a UI-ticket session
  right now; 10 of 12 sibling `NOTO_EMOJI_BUCKETS` entries there are *already* correctly de-sandwiched,
  only indices 10–11 were missed).
- Deleting the sandwich broke ~20 outside consumers (Rust `include_bytes!`/`#[path]`, TS/vite aliases,
  plugin `Cargo.toml` asset metadata, root `package.json` workspaces). All are cataloged with exact
  before/after diffs in `📋️registrar-handoff.md` — none were edited directly, per this ticket's
  explicit "never touch root package.json workspaces / any plugin directory / any other framework
  family's directory" constraint (independently confirmed as this wave's established convention by the
  sibling `CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE/FRAMEWORK-SURFACE-FAMILY-CRATE-CONSOLIDATION`
  ticket, which documents the identical pattern for its own family).
- Ticket marked closed in `🎫️ticket.json` (finish pass 2026-08-06); Repo MCP `ticket_close` still
  unavailable — registrar applies `📋️registrar-handoff.md` §1–§12 + JSON block at file end.
