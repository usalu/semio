# 📋️ TEMPLATE-FAMILY — merging a framework/product family of crates (not a plugin) into one Shape V2 crate

Written by the hub dress rehearsal (ticket `26/08/06/HUB-PRODUCT-CRATE-CONSOLIDATION-DRESS-REHEARSAL`) from
what actually happened consolidating `🌎️hub`'s 5 crates (`semio-hub` + `semio-hub-directory` +
`-directory-{sqlite,postgres,neo4j}`) into one `semio-hub` crate. Read `📋️TEMPLATE.md` first — that
recipe is written for a **plugin** (constitutional 7-crate layout, `🗿️artifacts`/`🎛️apps` taxonomy,
`app_commands!`, wasm component target). A framework/product family has none of that vocabulary: no
artifacts, no apps, often a real native `[[bin]]` instead of a wasm `cdylib`, and its "backend variants"
are Cargo features, not runtime-loaded plugins. This file is the delta — what differs, and why — for the
upcoming framework wave (math's 52 dirs, ui, compiler, surface, os-kernel/host, repo product).

---

## 0. How to tell you're in this case, not a plugin

- The owner root sits directly under `🧰️framework/` or `🌎️hub/` (or another top-level product owner),
  not under `✏️s/🔌️plugins/`.
- `find <owner> -name Cargo.toml` shows crates with NO `🗿️artifacts/`, `🎛️apps/`, `🛂️manifest/` shape —
  they're either a thin domain library (a trait + DTOs, like `semio-hub-directory`) or a handful of
  backend-swap siblings (sqlite/postgres/neo4j, like hub's directory backends) or, rarer, a real runnable
  binary (`[[bin]]`, no `[[lib]]` at all — hub's own `semio-hub` crate before the merge).
- `grep -rn "<old-crate-name>" --include=Cargo.toml .` outside the family's own dir returns nothing (a
  genuinely isolated family — hub's case) or a short, enumerable list (unlike a 300+-dependent kernel
  crate) — if it's the latter, budget real time for the fan-out and don't assume `TEMPLATE.md`'s "zero
  dependents" framing applies to you.

---

## 1. The `role` metadata decision

`🔣️taxonomy.json`'s `roles` enum is `["plugin", "framework", "product", "hub", "s-module", "extension",
"testkit", "tool"]` — richer than the legacy registry script's `hasSemioRole(text, "plugin" | "framework")`
helper, which only ever checks those two literal values (it predates the taxonomy roles enum and is scoped
to the plugin-registry's own narrow discovery arm; it is NOT the authority on which role value is
"correct" for a non-plugin owner). The real authority is `readSemioMarker` in `🟦️discovery.ts`, which
accepts any value from the full enum and records it verbatim in `DiscoveredPackage.role` — no special
casing per role, so picking the wrong-but-plausible value (e.g. `"framework"` for everything under the
framework umbrella) silently miscategorizes the package for any future role-filtered consumer.

**Rule: use the value that names the OWNER, not the umbrella it lives under.** `🌎️hub` is its own
top-level owner with a dedicated `"hub"` enum value — use it. Reserve `"framework"` for packages whose
owner root genuinely IS `🧰️framework` itself (e.g. `semio-framework-core`); reserve `"product"` for a
product owner that isn't one of the more specific values (`hub`/`s-module`/…). If your family's owner has
no dedicated enum value yet, that's a signal to extend the enum (one line in `🔣️taxonomy.json`) rather
than force-fit an adjacent value — the enum is deliberately open-ended per the master ticket's role list.

Verify your choice is actually exercised, not dead: before your merge, check whether ANY manifest under
your owner's area already has a `📦️packages/<lang>/` dir with a role marker (`find <owner> -name
📦️packages -o -name Cargo.toml`, then grep `[package.metadata.semio]`). If none does, your merge is the
FIRST time `discoverPackages()` will ever see that role value for real — treat it as unproven and sanity
check post-merge with a one-off script calling `discoverPackages(repoRoot)` and asserting your package
appears with the expected role (see this ticket's own verification notes for the exact snippet).

---

## 2. The binary target — `[lib]` + `[[bin]]`, not just `[[bin]]`

A plugin's merged crate is `[lib] crate-type = ["cdylib", "rlib"]` — it's always a wasm component, never a
native executable. A framework/product family may instead have (or need) a **real native binary**. hub's
old `semio-hub` crate had ONLY `[[bin]] name = "os-hub"`, no `[lib]` at all — its ~600 lines of axum/WS
logic lived directly in `bin.rs`, and nothing outside the family imported it as a library.

Decide per-family whether the merged crate needs both:

- **If any sibling crate in the family is a pure-library dependency of the binary** (hub's case:
  `semio-hub-directory` + its 3 backends were libraries the old `semio-hub` binary depended on), keep that
  split as `[lib]` (the library surface, re-exported for anything that reasonably wants the logic without
  the binary) + `[[bin]]` (the executable, `use <crate_name>::…` to reach the lib — this works automatically
  for any `[[bin]]` in the same package as a `[lib]`, no extra `Cargo.toml` dependency entry needed).
- **If the family was already a single crate with only `[[bin]]`**, you may still be ABLE to add a `[lib]`
  purely for organizational reasons (taxonomy tree modules need somewhere to be declared from), but don't
  invent an artificial public library surface nobody asked for — check `grep -rn "<crate-name>"
  --include=Cargo.toml .` outside the family first, same as any other reachability check.
- The plugin convention "`📦️lib.rs` contains no logic at all, ends with `semio_plugin!{}`" is a
  **plugin-specific** lint (`TaxonomyLibShape`, gated on `role == "plugin"`) — it does NOT apply here. A
  product/hub `bin.rs` legitimately contains its real logic (router, handlers, `main`); don't manufacture a
  thin wrapper just to satisfy a rule that isn't checking this role.
- Both `📦️lib.rs` and `📦️bin.rs` are recognized `entryFilenames` for the rust ecosystem in
  `🔣️taxonomy.json` (`"entryFilenames": ["📦️lib.rs", "📦️bin.rs"]`) — both live inside
  `📦️packages/🦀️rust/`, sibling to the manifest, same as a plugin's single `📦️lib.rs`.

---

## 3. Backend-swap sibling crates → Cargo features

When several sibling crates exist ONLY to implement the same trait against a different storage/driver
(hub's sqlite/postgres/neo4j directory backends), they become **Cargo features on the merged crate**, not
always-on code:

1. **Feature-gate the whole `mod` declaration**, not scattered `#[cfg]` inside one file — mirrors the
   established convention already in this repo's `db` facade crate (`semio-framework-os-kernel-db`):
   ```rust
   #[cfg(feature = "sqlite")]
   #[path = "🪶️sqlite/🦀️component.rs"]
   pub mod sqlite;
   ```
   This was NOT invented for this ticket — grep `#[cfg(feature = "sqlite")]\s*pub mod storage_sqlite` in
   `🧰️framework/…/🛢️db/⚡️implementations/🦀️rust/📦️lib.rs` for the precedent before assuming you need to
   design a new pattern.
2. **Distinguish driver deps from stack deps.** Read each old sibling's `[dependencies]`: the actual DB
   client library (`rusqlite`, `sqlx-core`+`sqlx-postgres`, `neo4rs`) is backend-specific and becomes
   `optional = true`, gated by `dep:<name>` in `[features]`. Generic stack deps the plan's own prose might
   mention alongside them (`axum`, `tokio`, `dashmap`, `async-trait`, `serde`) are almost always
   backend-agnostic infrastructure used by the CORE trait/binary regardless of which backend is selected —
   keep those unconditional. Don't feature-gate a dependency just because it appeared in a backend crate's
   `Cargo.toml`; check whether the core (always-compiled) code also needs it.
3. **If a sibling family crate (like `db`) ALREADY has same-named features for the same backend split**,
   tie your new feature to it: `sqlite = ["dep:rusqlite", "db/sqlite"]`. One feature name then controls
   both halves of the stack (hub's directory backend AND `db`'s storage substrate) — no split-brain where
   one half compiles in and the other doesn't. Verify the sibling's feature names line up exactly
   (`grep -n "^\[features\]" -A 20 <sibling>/Cargo.toml`) before assuming a match.
4. **Pick a sensible `default` feature**, not `default = []`. If the family's existing test suite
   (the binary's own tests, not just each backend's) constructs its test fixtures through ONE specific
   backend (hub's tests all use `SqliteDirectory` — the only backend needing no external service), make
   that backend the default and gate the binary's OWN `#[cfg(test)] mod tests` on `feature = "<default>"`
   too: `#[cfg(all(test, feature = "sqlite"))]`. This keeps `cargo test` (no flags) exercising the full
   binary-level suite out of the box, matching pre-merge developer experience, without forcing every
   feature combination to carry a full copy of those tests.
5. **`cargo check --no-default-features` must still succeed** (the plan's own verification gate). Any
   `match` arm in the binary that references a feature-gated type needs `#[cfg(feature = "…")]` on the
   ARM itself (stable, supported syntax), with a catch-all fallback arm outside any cfg. Watch for a
   function parameter that's ONLY read inside cfg-gated arms — with every backend feature off it goes
   genuinely unused; scope an `#[cfg_attr(not(any(feature = "a", feature = "b", …)), allow(unused_variables))]`
   to exactly that combination rather than blanket-suppressing the lint.

---

## 4. Organizing the taxonomy tree when there's no artifacts/apps vocabulary

Plugin taxonomy has a fixed vocabulary (`🗿️artifacts/<a>/{🔺️diff,🗣️dsl,🎒️pack,🔧️op,📡️spr,⚙️engine}`,
`🎛️apps/<app>/{🎭️modes,🪟️windows,…}`) because every plugin shares the same document-app-command shape. A
framework/product family usually doesn't — hub is an identity/tenancy directory service, not a document
editor. **There is no reserved vocabulary for this case; you invent a plain, domain-descriptive component
tree the same way a plugin invents an app-specific window/option name**, following the one universal rule:
below the owner root, only `component.<ext>` files, `📦️packages`, and plain component folders may exist.

hub's shape:
```
🌎️hub/
  📦️packages/🦀️rust/{Cargo.toml, 📋️project.json, 📜️script.ts, 📦️lib.rs, 📦️bin.rs}
  📇️directory/🦀️component.rs              # the core trait + DTOs (always compiled)
  📇️directory/🪶️sqlite/🦀️component.rs      # #[cfg(feature = "sqlite")]
  📇️directory/🐘️postgres/🦀️component.rs    # #[cfg(feature = "postgres")]
  📇️directory/🌐️neo4j/🦀️component.rs       # #[cfg(feature = "neo4j")]
```
One folder per logical domain concept (`📇️directory` — pick a fitting emoji + descriptive name, same
process as picking a plugin's window/option folder name), with backend/variant siblings nested one level
deeper as plain folders. If a family has several unrelated domain concepts (the framework wave's `math`
family, for instance, isn't one trait with backend variants — it's dozens of independent algebra/geometry/
graph domains), give each its OWN top-level component folder rather than forcing them under one invented
parent — the tree should mirror the family's real conceptual boundaries, not an arbitrary container.

`#[path]` mechanics inside a node that is itself reached via `#[path]` (not declared inline): a **top-level**
`mod` declaration inside a real, file-backed module resolves its OWN `#[path]` relative to THAT file's
directory, not cumulatively from the crate's entry point. `📇️directory/🦀️component.rs`'s own `pub mod
sqlite` therefore uses the plain relative path `"🪶️sqlite/🦀️component.rs"` — NOT a full path from the
owner root, and NOT `../../`-prefixed. The leaf-prefixed / cumulative-base rules in `📋️TEMPLATE.md` §2 and
`🔣️taxonomy.json`'s `rustEntryPathRules` describe INLINE nested `pub mod x { pub mod y { … } }` blocks
inside the crate's OWN entry file (`📦️lib.rs`) — a different mechanism that only applies at the point
where the entry file itself declares nesting. Don't over-apply the entry-file convention to every file in
the tree; re-derive which rule applies from where the `#[path]` attribute is physically written (top-level
in a real file vs. inside an inline `mod` block), and verify with a real `cargo check`, not by pattern-
matching against the plugin recipe.

---

## 5. Non-source assets a component needs (e.g. a SQL schema)

Shape V2 tree purity has no home for a bare data file living beside a `component.rs` (a `.sql` schema, a
`.wat` template, anything that isn't itself a per-language component leaf) — `rootDataDirNames` only
applies AT THE OWNER ROOT (`📚️examples`/`🧫️fixtures`/`🤖️generated`/`🖼️assets`/`📇️registry`), never nested
inside the tree, and none of those names fit "a schema the code needs to function" (as opposed to example/
fixture/generated data).

**Rule: if the asset is small and static, inline it as a Rust string/byte constant in the component file
that uses it, replacing `include_str!`/`include_bytes!` entirely.** hub's postgres backend had
`const SCHEMA: &str = include_str!("🛢️schema.sql")`; the merge folded the file's literal contents into
`const SCHEMA: &str = "…";` directly in `🦀️component.rs` — a zero-behavior-change mechanical transform
(the runtime string value is byte-identical), not a rewrite. This is consistent with CLAUDE.md's "handcraft
all assets... without any ugly migrations" directive: an inlined constant IS the handcrafted form, not a
workaround. If the asset is genuinely large (hundreds of KB+) or binary, this stops being practical — flag
it in your report rather than force-inlining; that's a real open question the framework wave should decide
once it hits a case big enough to matter (nothing in hub's family was).

---

## 6. Verification sequence (delta from `📋️TEMPLATE.md` §9)

Same core sequence, run once per meaningful feature combination instead of once total:

1. `cargo check --manifest-path <new>/Cargo.toml --no-default-features` — the core/base-only build.
2. `cargo check/clippy --manifest-path … ` with the **default** feature set (whatever you chose in §3.4).
3. `cargo check/clippy --manifest-path … --all-features`.
4. `cargo test --manifest-path … --all-features` — this is the number that must equal the OLD family's
   summed baseline (every old crate's `cargo test -p <crate> --lib` count, recorded BEFORE you touch
   anything). Backend crates needing a live external service (Postgres via testcontainers/Docker, a real
   Neo4j instance) will show the SAME pre-existing failure mode post-merge as pre-merge if the CI/dev
   environment doesn't have that service — this is not a regression, but you must have actually run the
   PRE-merge baseline yourself to know that's the same failure, not assume it.
5. Only once all of the above are green (or fail identically to the recorded baseline) do you delete the
   old crate directories and write the registrar handoff.

The isolated-verification `[workspace]` overlay trick (`📋️TEMPLATE.md` §3) works identically here — no
family-specific change needed, other than double-checking whether your family's path-dependencies (hub's
`db`/`protocol`) resolve at the SAME relative depth from the new `📦️packages/<lang>/` location as they did
from the old location (count path segments both ways; don't assume — see `📋️TEMPLATE-EXT.md`'s §1 for why
this is a coincidence per-owner, not a guarantee).

---

## 7. Things that will burn your time and aren't your bug

- **Root-workspace commands (`cargo test -p <old-crate>` from repo root) may intermittently fail** with
  `multiple workspace roots found` or `failed to load manifest for workspace member` while OTHER concurrent
  sessions are mid-migration elsewhere in the repo (a sibling plugin's temporary `[workspace]` overlay not
  yet cleaned up, or a crate dir mid-rename). Retry once; if it persists, fall back to the isolated
  `--manifest-path` + overlay trick for baseline-testing the OLD crates too, not just the new one — don't
  block your own ticket on someone else's in-flight state.
- **A shared, heavily-concurrent `target/` dir makes cold `cargo check`/`clippy` builds slow** (multi-minute
  for a small crate) when many other sessions are compiling framework crates simultaneously — this is
  contention, not a sign your Cargo.toml is wrong. Budget generous timeouts and run verification commands
  synchronously (never backgrounded — see the master ticket's ground rules on this).
