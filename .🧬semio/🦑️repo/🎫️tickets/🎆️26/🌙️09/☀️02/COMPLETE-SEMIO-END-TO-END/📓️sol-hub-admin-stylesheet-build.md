# Hub Admin Stylesheet Build Contract

## Scope

- Owned only the hub-admin stylesheet dependency contract: `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🎨️.css`, its existing `📜️script.ts` router, and two neutral fixtures.
- Did not edit the shared framework stylesheet, OS/directory authority, hub Rust, DB, root scripts, manifests, or launch configuration.

## Red diagnosis

The original admin stylesheet imported the nonexistent target-local path:

`../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css`

The canonical shared stylesheet is `🧰️framework/🔨️modules/🖱️ui/🎨️.css`. The React target manifest already exports `./🎨️.css` to that root aggregator, and the OS renderer already imports the same canonical file directly.

Before changing the CSS, the permanent oracle provided this TDD red:

```text
bun nx run os-hub-admin:test --skip-nx-cache
```

Exit `1`, before Vitest: `Hub admin stylesheet imports differ from the canonical dependency graph`.

## Change and permanent oracle

- Replaced the stale admin import with `../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️.css`.
- Preserved the single `@import` before the unchanged `@source "."` and `@source "../../🧱️elements"` directives.
- Added `🧪️tests/🔣️stylesheet-graph.schema.json` and `🧪️tests/🔣️stylesheet-graph.json`.
- Extended the permanent package router with an independent Node URL/filesystem oracle. It checks exactly 5 laws, package/repository containment, realpath identity, the React package export, absence of a target-level compatibility duplicate, and every relative import reachable from the shared root stylesheet.
- Green oracle output: `[DEBUG] hub admin stylesheet graph oracle: 5 laws, 1 canonical import, 2 Tailwind sources, 3 resolved shared imports across 4 stylesheets`.

## Verification

| Command | Result |
| --- | --- |
| `bun nx run os-hub-admin:test --skip-nx-cache` | Exit `0`; both entry/style oracles; 2 files and 10 tests passed; Vitest duration `12.24s`. |
| `bun nx run os-hub-admin:test --skip-nx-cache -- exhaustive` | Exit `0`; both entry/style oracles; 2 files and 10 tests passed; Vitest duration `12.18s`. |
| `bun nx run os-hub-admin:build --skip-nx-cache` | Initial post-fix run exited `0`; 1,704 modules transformed; Vite built in `30.23s`. |
| `bun nx run os-hub:build --skip-nx-cache` | First post-fix aggregate run rebuilt admin successfully in `18.11s`, then Cargo returned status 101 amid concurrent build-directory activity. |
| `cargo build --release --manifest-path Cargo.toml --message-format short` from `🌎️hub/📦️packages/🦀️rust` | Exit `0`; `semio-hub` compiled; release profile finished in `1m 17s`, so no Rust source error was established by the prior aggregate status. |
| Current `bun nx run os-hub-admin:build --skip-nx-cache` reproduction | Reaches 1,704 transformed modules, then exits `1` at the independent OS/directory export drift below. |

The successful Vite build retained pre-existing warnings: one invalid generated scrollbar selector, unresolved runtime `/asset/...` URLs, browser externalization of Node builtins, mixed dynamic/static imports, and chunks over 500 kB. None stopped that build.

## Exact next frontier

The current reproducible blocker is outside this packet:

```text
🧰️framework/🛍️products/💻️os/🟦️.ts (3759:9): "descriptorDigestEncodingV1" is not exported by
🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️.ts
```

This appeared after the clean stylesheet build had already succeeded and belongs to the concurrent OS/directory authority lane. No fix was attempted here.

## Residual outside scope

The workspace root `📜️script.ts` still contains broader source-census logic referring to the nonexistent target-local React stylesheet. That root script was explicitly outside this packet and was not edited.
