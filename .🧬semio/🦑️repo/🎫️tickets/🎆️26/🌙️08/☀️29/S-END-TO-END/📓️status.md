# 🖥️ S End to End — status

Goal: `bun ./📜️script.ts dev s` (launch entry `s-react`, port 6070) boots the semio **s** host OS
with the full plugin fleet, and the app is interactive.

## What `s` is

`s` is the **host** playground variant backed by `semio-s-plugin-space`
(`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust`). Being a host variant, its dev build compiles **all ~58
plugins**, not just one. See `📓️explore-s-app-identity.md` and `📓️explore-s-dev-pipeline.md`.

## Wave plan

0. **Rust fleet builds for `wasm32-wasip2`** — the plugin target. (in progress)
1. **Dev server boots on :6070** and materializes plugin modules.
2. **Shell renders and is interactive** — verified in a browser, not asserted.
3. **Gate** — an automated proof wired into `launch.json`.

---

## Wave 0

### Blocker 0.1 — `semio-framework-os-kernel` did not compile for `wasm32-wasip2` — FIXED

A stale `.await` survived a peer's de-async of `ArtifactStore::detach_backbone`:

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:900`

```
error[E0277]: `Result<Option<Backbones>, VcsError>` is not a future
```

`ArtifactStore::detach_backbone` is now **sync** (`🏪️store/🦀️component.rs:15697`,
`-> Result<Option<Backbones>, VcsError>`); every other call site had already been migrated
(`🔌️plugin/🦀️component.rs:24446`, `🖥️host/🦀️component.rs:786`, `🏪️store/🦀️component.rs:23805`) —
this one was missed. Fixed by dropping the `.await` and discarding the `Result` like its peers:

```rust
let _ = self.store.detach_backbone();
```

This one error aborted the **entire** fleet check: nothing downstream of `os-kernel` was reached, so
the earlier "only 1 error" reading was a *reachability mask*, not a clean tree
(cf. the `--keep-going` reachability-masking lesson).

### Fleet check after the fix

(running — results appended below)

With the kernel fixed, the fleet check reached the plugin layer and stopped at exactly one crate:

```
error: could not compile `semio-s-plugin-stdio` (lib) due to 59 previous errors
```

**`semio-s-plugin-stdio` is the single gate for the whole `s` fleet.** Only 1 of the 26 s plugin
crates was even reached — every other one depends on stdio (`space`/`s` itself takes it at
`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:29`). Whatever else is broken downstream is
still invisible behind it.

### The 59 errors, partitioned

| count | code | meaning | owner |
|---|---|---|---|
| 42 | E0046 | `impl Mutation` missing `DESCRIPTORS`, `descriptor` | the leaf migration, one artifact each |
| 8 | parse / E0433 / E0308 | `📊️csv` `🧬️mutations/🦀️.rs` corrupted by a botched edit | one file |
| 7 | E0425 / E0599 | `🧿️semio ✳️drawing` lost shared helpers in a file split | one subset |

The three groups are independent and touch disjoint files, so they are being worked in parallel.

### Blocker 0.2 — the 42 × E0046

`protocol::Mutation<P>` gained two required items. No stdio artifact hand-writes them; the sanctioned
way to get them is `#[derive(dsl::Mutations)]` over per-variant **mutation leaves**. Exactly one
artifact in the repo is already migrated — `🖼️tiff 6.0 ✳️baseline` — and it is the template. The full
recipe, derived by reading it and the derive that powers it, is in
`📓️plan-mutation-leaf-migration.md`.

Notable constraint the recipe pins down: **`NoMutation` cannot survive the migration**. The derive
asserts `is_approved_verb(SEMANTICS.verb)` and `no` is not an approved verb
(`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:105`), so every migrated
artifact drops that variant along with its `Default` derive, its `KINDS` entry, and its arms in the
artifact's own `🧪️tests/mutate-*/🦀️.rs`.

### Environment note — cargo is heavily contended

Peer sessions are running `cargo test -p semio-framework-surface`, `cargo test -p
semio-s-plugin-procedural`, several `semio-os-mcp` binaries and a rust-analyzer workspace check
concurrently. Sub-agents that each run their own `cargo check` simply queue on the target lock and
stall. **Verification is therefore centralized**: agents edit, one check runs here, findings are
dispatched back.

---

## Wave 0 progress — the leaf migration and its parity fallout

All 42 E0046 artifacts are through the fleet (`📓️fleet-report-wave-1.md`), plus the csv repair and the
`✳️drawing` shared-helper repair. Follow-on sweeps completed:

| sweep | scope | result |
|---|---|---|
| Rust fallout | `NoMutation` code references crate-wide | 16 real sites in 6 files — `🧿️semio/🦀️component.rs` (6 `RetireOwned` impls), both gif test adapters, three `mutate-semio-*` adapters — all converted |
| TypeScript parity | 47 `🟦️component.ts` mirrors | `noMutation` union member dropped; several typed-`unknown` fields reconciled against Rust |
| JSON Schema | 45 `🔣️component.json` | `enum` entry + `oneOf` branch removed |
| GraphQL | 33 `🔗️component.graphql` | enum value / union member / orphaned type removed |
| Protobuf | 20 `🛰️component.proto` | member removed; proto3 zero-value reassigned where the removed entry was `= 0`, never renumbering the rest |
| Grammars | 52 `.g4` / `.ebnf` / `.abnf` / `.ksy` | production + alternation reference removed in both directions; opcodes renumbered only where Rust's own `OpBinary` renumbered |
| semio surfaces | 41 `.grammar.semio` / `.protocol.semio` | same, with `dsl::variants_binary` ordinals traced to confirm the shift |

`🧪️oracle/` and `🧪️tests/` catalogs and `.feature` files were deliberately **not** swept: their
`no-mutation` is a scenario id, which is a different thing from the retired enum variant. The ruling
applied everywhere is that such a scenario maps to the identity mutation
`SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })`.

### Pre-existing drift surfaced by the sweep (NOT caused by it, NOT fixed)

Independently reported by four different agents, so it is real:

1. **`semio ✳️drawing`** — its `🔗️component.graphql`, `🛰️component.proto` and `🔣️component.json` mirror a
   completely stale vocabulary (`setCanvasSize`, `insertLayer`, `setNodeStyle`, …) with **zero** name
   overlap against the current Rust enum (`CreateLayer`, `DragNodes`, `FlattenNode`, …). The Rust
   file's own doc comment says it was rewritten to SMO-approved verb dispatch; the other surfaces
   never followed. This needs a full rewrite by that subset's owner.
2. **`semio ✳️any` envelope** — Rust has 19 variants; the GraphQL union, the proto `oneof` and the JSON
   `oneOf` each declare only 14. Missing everywhere: `text`, `table`, `graph`, `object`, `kit`.
3. **bcf, gif 87a, gif 89a, docx, xlsx** — their TS mutation unions are `setSnapshot`-only stubs
   against Rust enums of 13/11/20/12/9 variants, because their TS *snapshot* types are still raw
   `{ entries: {name,data}[] }` stubs. Closing this means modelling those formats in TypeScript.
4. **deflate, binary, pptx** — their `.grammar.semio`/`.protocol.semio` declarations describe wire
   formats their Rust codecs no longer produce (deflate and binary moved to hand-rolled `serde_json`
   during this migration; pptx had already moved to a generic value envelope). Reported rather than
   invented.

### Environment

The shared cargo target lock is saturated by peer sessions (100+ concurrent cargo processes:
`cargo test -p semio-framework-surface`, `-p semio-s-plugin-procedural`, `-p semio-s-plugin-norm`,
two workspace-wide rust-analyzer checks, `dev puzzle2d`). Verification therefore runs against a
private `CARGO_TARGET_DIR` under the session scratchpad, which costs a cold dependency build but
does not queue behind the peers.

### Lock-free verification (while cargo was blocked)

Two audits that need no build were run over the whole crate and caught real defects the fleet could
not have seen without compiling:

**1. Module-tree parse check.** `rustfmt --edition 2021 --emit stdout` on the crate root
(`📦️glue.rs`) walks every `mod`/`#[path]` declaration transitively, so it proves both that every
file parses and that every declared module actually exists. It found six `#[path]` attributes in
`semio ✳️drawing`'s aggregate pointing at leaf folders that do not exist — the paths had been written
against short leaf names (`🫓flatten`, `🎈unflatten`, `🧷group`, `🔄rotate`, `📏scale`, `💫ungroup`)
while the folders on disk carry the full kebab kind (`🫓flatten-node`, `🧷group-nodes`, …). All six
corrected; the crate root now walks clean. These sit under `#[cfg(test)]`, so `cargo check --lib`
would never have reported them.

**2. Leaf-descriptor audit.** All **913** `🔣️.json` leaf descriptors were checked for the four
invariants the `dsl::MutationLeaf` derive enforces: folder name ends with `semanticKind`, `emoji`
equals the folder-name prefix, `owner` ends with the folder name and resolves on disk, and the leaf
carries a `🦀️.rs`. Exactly one violation:
`✳️presentation/…/🧊set-textbox-blocks` — the derive asserts `SEMANTICS.kind == to_kebab("SetTextBoxBlocks")`,
which is `set-text-box-blocks`. Folder renamed to `🧊set-text-box-blocks` and the `#[path]`, the
descriptor `owner` and the leaf's doc comment brought with it. The **op-text keyword** stays
`set-textbox-blocks` — it is a separate vocabulary the derive does not see, and the committed
`.feature`, Python oracle and fixtures all speak it.

### Oracle catalogs deliberately untouched

49 `🧪️oracle/🦀️component.rs` files still handle `"no-mutation"`. That is correct and must stay: each
oracle is an INDEPENDENT implementation that must never link the subject crate, and its
`"no-mutation"` is a scenario id in its own vocabulary, not the retired Rust variant.

---

## Wave 0 — first real compile of the migrated crate

With the private target dir, `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --lib`
finally ran. **329 errors** on the first pass, in four clean classes — and two of them were defects
the fleet could not have seen, because they live outside the artifact folders the agents owned.

### 🅐 `📦️glue.rs` shadowed every migrated leaf module (the big one)

`📦️glue.rs` is the crate's generated module tree. For 21 artifacts it declared its own stub

```rust
#[path = "."]
pub mod set_snapshot {
    #[path = "…/📄set-snapshot/🔺️diff/🦀️.rs"]      pub mod diff;
    #[path = "…/📄set-snapshot/↩️inverse/🦀️.rs"]    pub mod inverse;
    #[path = "…/📄set-snapshot/🦀️.rs"]             mod component;
    pub use component::*;
}
```

left over from the older sweep convention. An explicit `pub mod` beats the aggregate's
`pub use component::*` glob re-export, so this **shadowed** the real leaf module — and for 7 of them
it also compiled the leaf's `🦀️.rs` a **second time**, in a scope with none of the aggregate's
imports, which is why `Serialize`, `Deserialize`, `agg_diff` and every snapshot type "could not be
found" in files that plainly imported them via `use super::*`.

Removed exactly the 21 stubs whose leaf name the aggregate itself declares. The other **98** leaf
stubs in `📦️glue.rs` were kept deliberately: `✳️mesh` and `✳️brep` migrated earlier under a different
convention where the leaf delegates to sibling `super::diff::` / `super::inverse::` modules that only
`📦️glue.rs` mounts. Removing those would have broken two working subsets.

### 🅑 `semanticKind` must be verb-entity, never a bare noun

19 leaves failed with `MutationLeaf source authority failed: semanticKind must be lowercase kebab-case`.
The rule is stricter than it reads — `mutation_leaf_descriptor_kebab`
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:396`) returns its `hyphen` flag, so it
**requires** a hyphen. A mutation leaf is always `verb-entity`.

- `💾️binary`'s `Splice` → `ReplaceByteRange` (`replace-byte-range`; `replace` is an approved verb, and
  the leaf already declared `entity: "byte-range"`).
- the `✳️any` envelope's 18 subset wrappers → `Brep` → `ApplyBrep` (`apply-brep`), and so on.

In both cases the Rust variant, the leaf `SEMANTICS`, the leaf folder and the leaf `🔣️.json` had to
move together, because the derive const-asserts `SEMANTICS.kind == to_kebab(VariantIdent)` and
`descriptor.aggregate_variant == VariantIdent`. **The wire tag was pinned unchanged** with
`#[serde(rename = "splice")]` / `#[serde(rename = "brep")]` … so committed fixtures, the oracle
catalogs and the `.feature` scenario ids all keep speaking the vocabulary they already speak — and
the GraphQL/proto/JSON/TS surfaces needed no further edits.

### Result

329 → 2 → 0 errors in the two follow-up passes. A re-run of the 913-descriptor audit under the
now-understood kebab rule reports **0 problems**, and the crate root parses clean.

---

## Wave 3 (prepared) — the acceptance gate

`.storybook/os-plugins.spec.ts` already drives a per-plugin boot matrix over every
`PLUGIN_BUILD_TARGETS` entry, `s` included — but its assertion is that each plugin reaches *a
deterministic boot outcome*, which a **failed** boot also satisfies (`semioOsError` counts). It is a
liveness gate, not a correctness one.

Added `.storybook/s-end-to-end.spec.ts`, which makes the stronger claim `s` has to meet:

1. `s` boots to **ready** — `semioOsError`/`semioOsNotFound` fail the test — and the artifact-missing
   panel must not appear, so a fleet that failed to materialize cannot pass vacuously.
2. The shell renders its structural landmarks: `.semio-scope[data-shell-id]`, `[data-level='base']`,
   `[data-semio-portal-layer]`, and a non-empty `[data-slot='app-name']`.
3. It is **interactive**: the command palette opens on Ctrl/Cmd+K and closes on Escape, and a
   right-click raises the shell's own `[role='menu']`.
4. Zero uncaught page errors and zero significant console errors (same 404 filter the sibling specs
   use).

Every assertion is structural (`data-*` / `role`), never text — the UI is multi-language with no
default language, so a text assertion would encode one.

No launch entry was added: `.storybook/playwright.config.ts` uses `testMatch: ["*.spec.ts"]` over its
own directory, so `bun run test:storybook` (`⚖️gate` → `workspace:test-storybook`) picks the new spec
up automatically.

**Status: written, not yet executed.** It needs a Storybook static build, which needs the plugin
fleet, which is still queued behind the shared cargo lock. It is not claimed as passing.

---

## ✅️ Wave 0 complete — `semio-s-plugin-stdio` compiles for the plugin target

```
cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --lib
    Finished `dev` profile [unoptimized] target(s) in 5m 47s
```

**0 errors.** This was the single gate for the whole `s` fleet — only 1 of the 26 s plugin crates was
even reachable before it, because every other one links stdio (`space`/`s` itself at
`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:29`).

The last error to fall was a third instance of the `to_kebab` class: obj's `InsertTexCoord` /
`RemoveTexCoord` / `SetTexCoord` kebab to `insert-tex-coord`, not `insert-texcoord`. Rather than fix
them one compiler round-trip at a time, the rule was then checked **repo-wide** — a script replicating
the derive's own `to_kebab` compared every `dsl::Mutations` variant against its leaf's `SEMANTICS.kind`
and folder name across **87 aggregates**: 0 mismatches remain. (The same audit's "NO LEAF" lines are
false positives — `✳️mesh`/`✳️brep` keep their `SEMANTICS` in `🦠️mutation/🦀️component.rs` under the
other convention.)

### Note on the ticket's `🗑️generated` folder

A peer sweep (`26/08/20/TICKET-OVERSIZED-ARTIFACT-PURGE`) deleted it mid-session along with the
in-flight compiler logs. The reports in this folder survived; the logs were regenerated. Worth knowing
that generated output here is not durable across sessions.

## Wave 1 — booting `s`

`.claude/launch.json`'s `s-react` entry does **not** serve React. `frameworkOsPlaygroundDevEnv`
(`🦑️repo/…/📚️library/📦️packages/🟦️typescript/📦️index.ts:2490`) defaults `SEMIO_RENDERER` to **`wgpu`**,
so a bare `dev s` builds all 59 plugin crates and then hands off to `trunk serve` — it never reaches
Vite on `S_OS_PORT`. Confirmed by running it: the failure was `wgpu trunk serve failed`.

Added a `served` segment to `runFrameworkOsPlaygroundDev` (`📜️script.ts:184`) and an `s-react-served`
entry to `.claude/launch.json`. It sets `SEMIO_RENDERER=react` + `SKIP_PLUGIN_BUILD=1` — the exact
pair `collabStartUserDevServer` already spawns each collab user with — so the React OS shell serves
over Vite against the `🔌️plugin-modules/` already on disk. It is a command segment rather than an env
var because `launch.json` (how every dev here starts things) carries no `env` field.

With it, `dev s served` correctly takes the React path and proceeds to the framework engine wasm
build (`wasm-pack build … framework_surface`) — which is unavoidable and currently
`Blocking waiting for file lock on build directory` behind peer sessions.

### Note on `.vscode/launch.json`

`.vscode/launch.json` is generated from `.vscode/🧩️launch.seed.jsonc` by
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts`, which renders one entry per
`variant × renderer` from a `devLaunchers` table. Its `🛠️dev🖥️s⚛️react` entry already sets
`SEMIO_RENDERER: react` through a VS Code `env` block, so devs launching from VS Code do get React —
what they do not get is the skip-build path, which is why `served` exists.

The generator was **not** extended with a third "served" axis: that would mean adding a renderer-like
dimension to the shared `devLaunchers` schema for what is a per-run choice, and every existing entry
would have to grow the fields. `served` is registered where it belongs — as a documented segment of
the `dev` command in `📜️script.ts` and as `s-react-served` in `.claude/launch.json`, which has no `env`
field and therefore no other way to reach the React shell at all.

---

## Where wave 1 actually stands

`dev s served` now takes the correct React path and gets as far as the framework **engine wasm**
build (`wasm-pack build … framework_surface`), which is unavoidable and uses the **shared** cargo
target dir. Every attempt has died there the same way:

```
[framework/surface/rs] wasm-pack build --dev --target web --out-dir pkg --out-name framework_surface
    Blocking waiting for file lock on build directory
…
error: spawnSync bun ETIMEDOUT   at buildEngineWasm (…/🧑️‍💻️dev/…/📜️script.ts:1562)
```

That is the 20-minute budget expiring while blocked on the lock — not a code error. The repo's own
message says as much: *"Likely shared cargo target-dir lock contention from another concurrent
session — investigate before retrying."* Peer sessions were running 112 concurrent cargo processes at
the time (`cargo test -p semio-framework-io-base64`, `-p semio-framework-hash`,
`-p semio-framework-replication`, `-p semio-s-plugin-{cad,sourcing}`, …).

The final attempt runs the dev server against a **private `CARGO_TARGET_DIR`** so it cannot be starved
— the same escape the peer `cad` session uses. That costs a cold build of the surface crate tree but
completes instead of being killed.

**Not claimed:** that `s` renders and is interactive. `.storybook/s-end-to-end.spec.ts` is written to
prove exactly that, and has not been executed.

## A peer refactor crossed this ticket mid-flight

Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` is extracting
`base64_standard_encode/decode` out of `📡️replication/⚙️codec` into a new
`semio-framework-io-base64` crate. A fleet check taken at 13:0x caught it mid-creation:

```
⚙️codec/🦀️.rs:55:9: error[E0432]: unresolved import `semio_framework_io_base64`
```

The crate materialized minutes later and is registered in the workspace. Not this ticket's to fix —
the fleet check was simply re-run against the settled tree.
