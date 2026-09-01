# Collision Census — `🧰️framework/🔨️modules`

Source: `🗑️temp/🔣️vocab-plan.json` (`.unresolved`), `--scope "🧰️framework/🔨️modules"`, baseline
`bb06c41f73f0122fbed315b7487428b976f99921`. 220 raw rows (`collision-byte` / `collision-case-fold` /
`collision-nfc` / `collision-same-kind` / `collision-vs16-fold`, 44 each — verified by `jq` dedupe on
`(destination path, sorted sources)`) collapse to **44 distinct cases**.

Pattern breakdown (by colliding source basenames):

| pattern | count | meaning |
|---|---:|---|
| `🦀️component.rs` × `🧪️component.rs` | 23 | Rust impl vs. Rust test module, same dir, both generic-stemmed |
| `🟦️component.tsx` × `🧪️component.test.tsx` | 16 | React impl vs. Vitest test, same dir |
| `📦️index.ts` × `🧪️index.test.ts` | 1 | TS impl vs. bun:test test, same dir |
| `⌨️component.rs` × `🧊️component.rs` | 2 | tui-target vs. wgpu-target Rust implementation, same dir |
| `📦️glue.rs` × `🦀️component.rs` | 2 | package glue vs. owner implementation, both hoisted to owner root |

## Full list (44)

| # | destination | source A | source B |
|---:|---|---|---|
| 1 | `🌱️value/🗂️ordered/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 2 | `🌱️value/🗂️ordered/🧺️set/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 3 | `🎠️kernel/📤️return/🏠️source/📚️entries/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 4 | `🎠️kernel/📤️return/📦️content/💌️message/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 5 | `🎭️actor/📄️page/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 6 | `🎭️actor/📤️return/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 7 | `🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 8 | `🎭️actor/🦀️.rs` | `📦️packages/🦀️rust/📦️glue.rs` | `🦀️component.rs` |
| 9 | `📡️replication/📡️wire/🏠️local-interaction/🌳️root/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 10 | `📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 11 | `📡️replication/📡️wire/🏠️local-interaction/📡️transport/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 12 | `📡️replication/📡️wire/🏠️local-interaction/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 13 | `🖱️ui/🎨️styling/🟦️.ts` | `📦️packages/🟦️typescript/📦️index.ts` | `📦️packages/🟦️typescript/🧪️index.test.ts` |
| 14 | `🖱️ui/🔨️modules/⌨️control-keybinding-context/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 15 | `🖱️ui/🧠️runtime/📤️output/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 16 | `🖱️ui/🧬️contract/♻️retirement/🌳️typed/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 17 | `🖱️ui/🧬️contract/♻️retirement/📋️patch/📨️pending/📄️whole/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 18 | `🖱️ui/🧬️contract/♻️retirement/📋️patch/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 19 | `🖱️ui/🧬️contract/♻️retirement/📮️handback/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 20 | `🖱️ui/🧬️contract/♻️retirement/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 21 | `🖱️ui/🧬️contract/⚖️compare/📄️document/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 22 | `🖱️ui/🧬️contract/⚖️compare/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 23 | `🖱️ui/🧬️contract/🎟️resident/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 24 | `🖱️ui/🧬️contract/📄️document/🎟️assembly/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 25 | `🖱️ui/🧬️contract/📋️copy/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 26 | `🖱️ui/🧬️contract/🔗️bindings/📋️copy/🦀️.rs` | `🦀️component.rs` | `🧪️component.rs` |
| 27 | `🖱️ui/🧬️contract/🦀️.rs` | `📦️packages/🦀️rust/📦️glue.rs` | `📦️packages/🦀️rust/🦀️component.rs` |
| 28 | `🖱️ui/🧱️elements/↕️Collapsible/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 29 | `🖱️ui/🧱️elements/⌨️Command/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 30 | `🖱️ui/🧱️elements/☑️Checkbox/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 31 | `🖱️ui/🧱️elements/☑️Select/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 32 | `🖱️ui/🧱️elements/☑️Select/🦀️.rs` | `⌨️component.rs` (tui) | `🧊️component.rs` (wgpu) |
| 33 | `🖱️ui/🧱️elements/✏️Input/🦀️.rs` | `⌨️component.rs` (tui) | `🧊️component.rs` (wgpu) |
| 34 | `🖱️ui/🧱️elements/🎚️Slider/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 35 | `🖱️ui/🧱️elements/🎚️Toggle/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 36 | `🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 37 | `🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 38 | `🖱️ui/🧱️elements/📊️Diagram/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 39 | `🖱️ui/🧱️elements/📋️MenuItem/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 40 | `🖱️ui/🧱️elements/📑️Tabs/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 41 | `🖱️ui/🧱️elements/📻️TableAvatar/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 42 | `🖱️ui/🧱️elements/🗨️Popover/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 43 | `🖱️ui/🧱️elements/🧾️Form/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |
| 44 | `🖱️ui/🧱️elements/🪵️Tree/🟦️.tsx` | `🟦️component.tsx` | `🧪️component.test.tsx` |

(paths above are relative to `🧰️framework/🔨️modules/`, elided for width)

## Classification

**(b) GENUINELY DISTINCT — 42 cases** (#1–7, 9–26, 28–44 except 8/27): every `🦀️component.rs`/
`🟦️component.tsx`/`📦️index.ts` sits beside a role-tagged sibling (`🧪️` test, or `⌨️`/`🧊️` target)
that is byte-different content, not a copy. Root cause traced to the engine (see report): the
generic-stem short-circuit in `canonicalFile` (`🧹️normalization/🟦️.ts:3122-3124`) returns the bare
kind-only leaf for ANY file whose trailing stem is generic (`component`, `index`, …) — including
`.test`-suffixed and role/target-tagged ones — before it ever reaches the `roleContext`/`targets`
routing that would otherwise disambiguate them into `🧪️tests/` or `🎯️targets/<slug>/`. Both
destination directory kinds already exist in `🔣️taxonomy.json` (`tests`, `wgpu-target`,
`tui-target`) — confirmed by the pre-existing precedent at
`🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs` and at
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/{⌨️tui,🧊️wgpu}/`. Fixed by hand-moving each
colliding file to that directory (no taxonomy.json edit needed).

**(a) DUPLICATION — 0 cases.** Neither #8 nor #27 turned out to be true duplication on inspection
(see below) — the census here originally expected some, per the ticket's cited precedent, but both
investigated cases were false positives for that diagnosis.

**(c) ENGINE-EXPOSED, NOT DUPLICATION — 2 cases** (#8, #27): both are `📦️glue.rs` files whose
`classifyPackageRole`/`classifyGlue` grammar (`🧹️normalization/🟦️.ts:3003-3030`) correctly detects a
`struct`/`impl` in the file and marks it `role: "implementation"`; `packageImplementationDestination`
(`🧹️normalization/🟦️.ts:3141-3157`) then unconditionally hoists ANY generic-stem
`role: "implementation"` package file to `${owner}/${kindOnly}` — the same slot the owner's real
`🦀️component.rs` already occupies, with no check for a second claimant. In #8 the `struct KernelHost`
was a genuine (105-line) `wasm_bindgen` FFI wrapper — legitimate code, wrongly generic-stemmed. In
#27 the `struct SchemaMetadata` was a 969-line schema-metadata table, also legitimate, also wrongly
generic-stemmed. Both fixed by extracting the offending `struct`/`impl` block out of `glue.rs` into
its own file under an *existing* semantic directory kind (`🔗️bindings`, `🧬️schema`), leaving `glue.rs`
pure declaration (`mod`/`use`/`extern crate`/attributes only) — see report for the exact diffs.
