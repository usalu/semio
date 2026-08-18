# 📓️ lease-request — P7-headless-workspace → 🏃️run `🔖️PluginPaths` extraction

**From**: terra, packet `P7-headless-workspace`
**To**: sol of `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` (applied against `🏃️run`, a peer-owned-adjacent module inside OUR OWN ticket's disjoint territory — `🏃️run` is not exclusively owned by any single packet here, but its `📦️bin.rs` is where this logic currently lives, mid-flight-adjacent to the microkernel ticket's B1/B1b work)
**Files**: `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` (the LIBRARY, not `📦️bin.rs`)

## Ask

`🏃️run/📦️bin.rs` currently owns `find_repo_root()` and `resolve_plugin_paths(repo_root, plugin_ids)`
(lines ~59-115 as of this ticket's baseline), private to that binary's own crate root (a `[[bin]]`
target is its own crate — nothing outside `🏃️run/📦️bin.rs` itself can call these even if they were
marked `pub`, confirmed by reading `📦️bin.rs`'s own module-doc comment on why `[[bin]]` targets don't
inherit the lib's `extern crate … as X` aliases either).

Please extract these into a new `//#region 🔖️PluginPaths` in `🏃️run/🦀️component.rs` (the library),
`pub fn`, so every consumer that needs to resolve a plugin id to its compiled `wasm32-wasip2` artifact
path (today: `🏃️run/📦️bin.rs` itself; this packet's `🌉️mcp/🏠️workspace`; presumably any future headless
driver) calls ONE function instead of each maintaining its own copy — CLAUDE.md's "if code is
repeated, it MUST be close to each other" / no-duplication rule.

## What we did in the meantime (documented temporary seam, not a silent workaround)

`🌉️mcp/🏠️workspace/🦀️component.rs`'s own `//#region 🔖️PluginPaths` reimplements the SAME algorithm
(`find_repo_root`: walk up from `CARGO_MANIFEST_DIR` for `nx.json`, `SEMIO_REPO_ROOT` override first)
independently, but resolves the plugin-id → wasm-path mapping through the JSON MIRROR of the data
`🏃️run/📦️bin.rs` itself reads (`🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json`, the same
`plugin-registry:generate` nx target's output as the `.rs` `PLUGIN_WASM_ARTIFACTS` table `🏃️run` reads
via `include!`) rather than a second copy of the `.rs` table itself — same generated SOURCE OF TRUTH,
different (but equally real) file format. This is the "narrowest workable approach, documented as a
temporary seam" the packet brief explicitly authorizes when the real extraction hasn't landed yet.

Once this lease lands, `🌉️mcp/🏠️workspace/🦀️component.rs`'s own `find_repo_root`/
`load_plugin_registry`/`find_plugin_entry`/`resolve_plugin_wasm_path` functions should be deleted and
replaced with direct calls into `🏃️run`'s new public API — filed here so that follow-up is tracked,
not silently left as permanent duplication.

## Status

Pending as of this packet's report. Not blocking — the temporary seam above compiles and is unit-
tested (`🌉️mcp/🏠️workspace/🦀️component.rs`'s `quick::find_plugin_entry_reports_a_typed_not_found_…`
and the `long::attempt_plugin_activation_against_a_real_note_wasm_when_available` test, which reads
this exact registry JSON at runtime).
