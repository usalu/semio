# 📌️ Important — read before touching `🗺️surface/` again

**Status as of this session: consolidation DONE, verification PARTIALLY blocked by out-of-scope issues.**

- New crate: `semio-framework-surface` at `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/`
  (Shape V2, `role = "framework"`). Domains are `pub mod {paint,board_2d,terrain,node_graph,tiled_map}`,
  each wired via `#[path]` to `🗺️surface/{domain}/🦀️component.rs` (mirrors `🧮️math/📦️packages/🦀️rust`'s
  convention).
- The 5 old `.../⚡️implementations/🦀️rust/` dirs (Cargo.toml, lib.rs, script.ts, package.json,
  project.json, pkg/) were deleted. **A concurrent session was repeatedly observed recreating them
  during this ticket's work** (seen at least twice, most recently right before this note was written).
  If you find them again: delete them, the source of truth is `🦀️component.rs` + `📦️packages/🦀️rust/`.
- Full `cargo check`/wasm build is blocked today by TWO issues, neither fixable from inside this
  ticket's exclusive ownership (`🗺️surface/**`) — see `📋️registrar-handoff.md` §1 and §5:
  1. Root `Cargo.toml` workspace members still list the 5 dead old paths (never touch root Cargo.toml).
  2. `infinite_board_port_directed_dag` (a `node_graph` dependency, owned by a different framework
     family) has stale path deps left over from `🧮️math`'s OWN graph-family consolidation.
- Isolated verification (this crate's own `Cargo.toml` + ticket's `🧪️overlay.toml` appended as a
  temporary self-contained `[workspace]`, then stripped again before handoff) got deep into the
  dependency graph — most direct dependencies (`ui_styling`, `dsl`, `infinite_canvas`) compiled clean —
  before hitting the blockers above and unrelated concurrent flicker in `🖱️ui`'s wgpu target
  (`build.rs` repeatedly missing mid-build, another session's in-flight work).
- Repo MCP was unavailable this session — this ticket was NOT closed via `ticket_close`. Do that once
  the registrar-handoff items are applied and a full `cargo check`/wasm build is confirmed green.
