# Bootstrapping the TypeScript toolchain on a fresh Windows checkout

Ticket: `26/09/05/PRINT-VISUALIZATION-LIBRARY`. Goal: make
`bun ./📜️script.ts test quick` in `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript`
run to completion after a clean `bun install` with the whole gitignored `**/🤖️generated/`
layer absent.

## Cycle diagnosis

Every `📜️script.ts` imports the shared repo library
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`, which imports

- `@semio-tech/framework` (barrel `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`)
  → `🔨️modules/🛂️manifest/🟦️.ts` → `🤖️generated/{🎚️ui-axes,🪪️manifest,📜️ui-contract}.ts`
- `🎮️playground/🟦️.ts` → `💻️os/…/📇️registry/🤖️generated/🎮️playgrounds.ts`

and the styling package `🎨️styling/📦️packages/🟦️typescript/🟦️.ts` →
`🎨️styling/🤖️generated/🟦️tokens.generated.ts`.

So **no bun generator can load until some generated files already exist**. The Rust side is
symmetric: the `semio-framework-ui-styling` crate `include!`s `🎨️styling/🦀️tokens.generated.rs`,
which is produced by the *bun* styling generator — so cargo typegen cannot run first either.

Two properties made the cycle breakable without reimplementing generators:

1. `🛂️manifest/🟦️.ts` pulls `🪪️manifest.ts` and `📜️ui-contract.ts` with **`import type`**, which bun
   erases — a zero-byte placeholder file is enough at runtime (it is also enough for the Rust
   `include_str!` in the typegen tests, which need the file to exist to compile).
2. The styling generator reads only `🎨️styling/🔣️.json` plus pure helpers; the *content* of
   `🟦️tokens.generated.ts` never feeds back into its own output. A throwaway stub therefore
   produces byte-identical output to the fixed point (verified: regenerating on top of the real
   file changes nothing and `check-generated` passes).

## Ordered bootstrap

Run from the repo root, Git Bash. `RUSTC_WRAPPER=""` is required because `.cargo/config.toml`
sets `rustc-wrapper = "sccache"` and `sccache` is not installed on this machine (the root
`setup postinstall` is what installs it — itself a victim of the same cycle).

| # | File | Produced by | Command |
|---|---|---|---|
| 1 | `🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes.ts` | ticket-local `🟦️bootstrap-ui-axes.ts` (copy of the emit plan in `🖱️ui/📦️packages/🦀️rust/📜️script.ts`) | `bun "…/🟦️bootstrap-ui-axes.ts" C:/git/semio` |
| 2 | empty placeholders: `🛂️manifest/🤖️generated/{🪪️manifest,📜️ui-contract}.ts`, `⏳️async/🤖️generated/🟦️async.ts`, `🎭️actor/🤖️generated/🟦️actor.ts`, `💻️os/…/🖥️shell/🤖️generated/🟦️.ts` | `: > <path>` | needed for `import type` erasure + Rust `include_str!` |
| 3 | stub `🖱️ui/🎨️styling/🤖️generated/🟦️tokens.generated.ts` | hand-written stub (11 value exports, empty bodies) | overwritten in step 4 |
| 4 | real `🟦️tokens.generated.ts`, `🦀️tokens.generated.rs`, palette CSS/C#/Python | repo generator | `cd 🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust && bun ./📜️script.ts generate` |
| 5 | `🛂️manifest/🤖️generated/📜️ui-contract.ts` | repo Rust typegen test | `cd 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust && RUSTC_WRAPPER="" SEMIO_TYPEGEN_OUT="C:/git/semio/🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract.ts" cargo test --features typegen exports_typescript_bindings` |
| 6 | `⏳️async/🤖️generated/🟦️async.ts` | same pattern, crate `semio-framework-async` | `SEMIO_TYPEGEN_OUT="…/⏳️async/🤖️generated/🟦️async.ts" cargo test --features typegen exports_typescript_bindings` |
| 7 | `🎭️actor/🤖️generated/🟦️actor.ts` | crate `semio-framework-actor` | same pattern |
| 8 | `💻️os/…/🖥️shell/🤖️generated/🟦️.ts` | crate `semio-framework-os-shell` | same pattern |
| 9 | `📇️registry/🤖️generated/*` (`🎮️playgrounds.ts`, `🧩️plugins.ts`, `🎠️playgrounds.json`, `🔌️plugins.json`, `🏗️framework.ts`, `🧰️framework.json`, `🖥️hosts.rs`, `🗿️artifacts.rs`) | repo generator | `cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry && bun ./📜️script.ts generate` |

Verification: `bun ./📜️script.ts check-generated` in the styling package reports
`generated artifacts are fresh`; re-running the registry `generate` leaves every output
byte-identical (md5 diff clean).

## Repo fixes (two genuine Windows bugs in generators)

Both are real cross-platform defects — `path.relative()` returns `\`-separated paths on Windows
while the repo's manifests/catalogs are POSIX-normalised everywhere else. Minimal fixes, matching
the `.replaceAll("\\", "/")` idiom already used in the same files:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:841` —
  `validateStylingOutputManifest` compared `\`-separated rendered paths against the
  `/`-separated `🛂️adapters.manifest.json`, so `generate` always threw on Windows.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:271` —
  `cratePath` was `\`-separated, so `registryCatalogInputView.checked()` rejected every
  catalog path (`Registry catalog input is outside its nonopaque owner: ✏️s\🔌️plugins\…`).
  It is also embedded verbatim into the generated catalogs, which must be platform-neutral.

No other repo source was modified. Nothing under `🧰️framework/🛍️products/📓️print/` was touched
except running its scripts.

## Final state

`cd "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript" && bun ./📜️script.ts test quick`
— exit 0:

```
[DEBUG] print: viz coverage 1966/1966 leaves, API 17/17
[DEBUG] print: unit tests passed
```

Root `bun ./📜️script.ts setup postinstall` — exit 0, no output.

## Still open (unrelated to the cycle, not fixed here)

- **`🛂️manifest/🤖️generated/🪪️manifest.ts` is still a zero-byte placeholder.** Its generator is
  `cargo test -p semio-framework --features typegen exports_typescript_bindings`, and
  `semio-framework-os-kernel` does not compile on the current HEAD: 45 pre-existing errors
  (E0277/E0599) around `SpaceHistoryMutation: protocol::Mutation<SpaceHistorySnapshot>` in
  `💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/{🧹️remove-space-alternative,📌️commit-space-checkpoint,🔀️switch-space-alternative}/🦀️.rs`
  and `🏪️store/🦀️.rs:18712+`. Harmless for TS runtime (the import is type-only), but every
  manifest type currently resolves to `any` for typecheckers. Rerun the command above once the
  store mutation trait bound is fixed.
- **`🖼️assets/🔣️icons/🤖️generated/*` cannot be produced on a fresh checkout.** `assets:generate`
  requires `🔣️icons/🤖️generated/🔣️shortcodes.json`, declared in `🔣️taxonomy.json` as
  `external-emoji-shortcodes` (ownership `external`, no target, gitignored) — a pinned gemoji
  snapshot with no in-repo producer. So `🟦️icons.ts`, `🔤️shortcodes.ts` and
  `🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts` stay missing. Only type-only imports
  reach the print graph, so this does not block it, but `@semio-tech/assets` cannot be loaded
  at runtime here.
- **Bun is too old:** installed `1.2.5`, `package.json` pins `packageManager: bun@1.3.14`.
  `Bun.JSONC` does not exist in 1.2.5, so the registry generator's `.vscode/launch.json` step
  (`🖥️launch.ts:readSeed`) throws `seed file … is not valid JSONC` — a false report.
  The `🤖️generated/*` catalogs are written before that step, so they are complete and stable.
- **`sccache` is missing** and `.cargo/config.toml` hard-wires it as `rustc-wrapper`, so every
  cargo invocation needs `RUSTC_WRAPPER=""` until root `setup postinstall` installs it into
  `.🧬semio/🦑️repo/⚡️cache/sccache` (the directory is still empty after a successful postinstall
  run — worth a look, out of scope here).
- `💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🟦️session.ts` is a dev-server runtime artifact, not codegen;
  no bootstrap needed.
