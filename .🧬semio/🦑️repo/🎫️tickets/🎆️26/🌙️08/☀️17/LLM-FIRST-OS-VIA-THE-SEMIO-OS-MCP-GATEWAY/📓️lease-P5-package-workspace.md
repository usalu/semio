# 📓️ Lease request — P5 — root `package.json` `workspaces` entry

`/package.json` is a registrar-only file (`📌️important.md` §"Registrar-only files"). P5
(conformance-tests) needs one new entry so `bun`/`nx` resolve the new TS package
`@semio-tech/framework-os-mcp` at `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript`.

## Precedent used for placement

P1a already leased (and sol already applied) the sibling Rust-crate entry into `/Cargo.toml`
`[workspace] members`, placing it immediately after the `🖥️shell` Rust crate and before `🛢️db`
(`Cargo.toml` lines 12-14 today):
```toml
"🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
```

`/package.json`'s `workspaces` array has the analogous `💻️os/🔨️modules/*/📦️packages/🟦️typescript`
block at lines 27-30 today, ending with `🖥️shell` immediately before the block switches to
`🦑️repo/🔨️modules/...` entries at line 31. Mirroring the Cargo.toml precedent exactly: insert
immediately after the `🖥️shell` TS entry (current line 30), before the `🦑️repo` block begins
(current line 31).

## Requested diff (insert, no other lines touched)

```json
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript",
```

Only the `"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript",` line is new —
shown with one line of context on each side to make the exact insertion position unambiguous.

**Status: filed at the start of P5's work session, before most of the implementation. Re-check
`/package.json` before acceptance — if this line is already present, sol applied it; otherwise
`bun nx run @semio-tech/framework-os-mcp:test-quick`/`test-long` cannot resolve the workspace
package and P5's report will say so plainly per its brief §4.**
