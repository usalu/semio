# 📓️ terra D0-descriptor-plumbing report

Packet: **D0-descriptor-plumbing** — make the descriptor pipeline's path story agree with itself, wire a real per-plugin `describe` command, harden `descriptor_is_fresh()`.
Read: `📌️important.md`, `📓️terra-E1-describe-report.md`, `📓️terra-E2-builder-descriptor-report.md` (both prior packets in this ticket), `📓️status.md`.

## Honest status summary

| item | state |
|---|---|
| 1. One canonical descriptor path (owner root, not `🤖️generated/`) | **done** in `descriptor_is_fresh()` (already fixed by E2, verified still correct) and in dev `📜️script.ts`'s `describeBuiltPlugin()` (fixed this packet). **Blocked** in the registry `📇️registry/📜️script.ts` — `lease-request` filed, not edited (see below) |
| 2. `describe` command on plugin crate `📜️script.ts` | **done for all 33** — shared implementation added to `📇️describe/📦️packages/🦀️rust/📜️script.ts` (`describePluginComponent`), registered in all 33 top-level plugin crates' own `📜️script.ts` + `📋️project.json`. **Proven end-to-end on `🗒️note`** (real emitter, not a substitute) |
| 3. `descriptor_is_fresh()` hardening | **done** — explicit per-plugin/per-extension opt-in ratchet list, missing descriptor now fails for a listed plugin, still soft for everyone else (see design below) |
| 4. Registrar-owned files | `📇️registry/📜️script.ts`'s `DESCRIPTOR_JSON_REL_PATH` — **lease-request filed**, not edited |

## 1. The canonical path — owner root, and why

`descriptor_is_fresh()` (`🔌️plugin/🦀️component.rs`) and the only real committed descriptor in the repo (`✏️s/🔌️plugins/🗒️note/{🛂️descriptor.semio,🔣️descriptor.json}`) both already used the plugin/extension **owner root** — sibling of the tracked `🛂️manifest.json`, two levels up from `📦️packages/🦀️rust`. `🤖️generated/**` is globally gitignored (`.gitignore` ~87-88); a "checked-in" descriptor can never survive a commit there. So owner root is the only one of the three that actually works, and I made everything else match it rather than adding a fallback:

- **`descriptor_is_fresh()`**: already fixed by E2-builder-descriptor (a prior packet in this same ticket) before I started — verified still correct on disk (`concat!(env!("CARGO_MANIFEST_DIR"), "/../../🛂️descriptor.semio")`, `🦀️component.rs:16258` and `:16596` after my own edits).
- **Dev `📜️script.ts`'s `describeBuiltPlugin()`** (`🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:789-804`): was still writing to `<owner>/🤖️generated/` — the exact three-way disagreement the ticket describes. Fixed: `ownerGeneratedDir` renamed `ownerRoot`, now `join(repoRoot, target.cratePath, "..", "..")` with no `🤖️generated` segment, doc comment updated.
- **Registry `📇️registry/📜️script.ts`** (`DESCRIPTOR_JSON_REL_PATH`, line 150, and its one consumer `readDescriptorJson`/`validateDescriptors`): still reads `🤖️generated/🔣️descriptor.json` today. **Not fixed by me** — filed as a `lease-request` (§5) because this directory is outside my packet's `path_scope` (see the scope note there for the exact reasoning). Verified the breakage is real and current: `bun nx run @semio-tech/plugin-registry:check` (ticket folder `terra-D0-registry-check1.txt`) still prints `note: no ✏️s/🔌️plugins/🗒️note/🤖️generated/🔣️descriptor.json yet` even though note's real descriptor sits right there at the owner root.

## 2. The `describe` command — shape and proof

**Shared implementation, one place** (`important.md`'s "if code is repeated it must be close to each other"): `🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts` now exports `describePluginComponent(repoRoot, packageName, ownerRoot)`:
1. `cargo build -p <packageName> --target wasm32-wasip2` — **no `--features component-guest` flag**. Verified empirically: no plugin crate exposes a feature literally named `component-guest` of its own (it's enabled unconditionally as a dependency-feature on `semio-framework-plugin` in every plugin `Cargo.toml`); passing that flag to `cargo build -p <plugin>` errors `does not contain this feature` (this matches E2's own note).
2. Builds (if needed) and execs the existing `semio-framework-plugin-describe` emitter binary against the freshly-built `wasm32-wasip2` artifact, `--out <ownerRoot>`.
3. Also fixed a real, pre-existing bug in the same file while I was there: `ensureBuiltBin()` hardcoded `target/debug` regardless of `CARGO_TARGET_DIR` — under a ticket-scoped target dir (binding rule 5) this would have execed a stale or nonexistent binary from the wrong tree. Added `cargoTargetRoot(repoRoot)` (honours `CARGO_TARGET_DIR`) and reused it for both `ensureBuiltBin` and the new `pluginWasmArtifactPath`.

**Registration, every top-level plugin crate** (`✏️s/🔌️plugins/*/📦️packages/🦀️rust/📜️script.ts` + `📋️project.json`, exactly the 33 matched by `find … -mindepth 4 -maxdepth 4 …` — the ticket's own "33 plugins" count): a thin `DescribeScript extends BundleScript` that calls `describePluginComponent(this.repoRoot, "<crate-name>", join(this.root, "..", "..")))`, registered as `.register("describe", DescribeScript)`, plus a `describe` nx target mirroring `test`'s shape. `🗒️note` was hand-written first and used as the template; the other 32 were applied by a small idempotent scratch tool (`d0-add-describe-command.ts`, ticket folder, deleted from the repo's perspective — never part of it) that parsed each file's existing `runCargoTestBudgeted(["semio-s-plugin-…"])` call for the crate name and spliced in the same three pieces (node:path import, describePluginComponent import, DescribeScript class, router registration) — verified byte-for-byte identical in shape to the hand-written note version on spot checks (`✒️writer`, `🗄️stdio` — the one crate with an extra `BenchScript`, confirms the splice composes correctly with an existing third router entry).

A second scratch pass (`d0-fix-project-json-reflow.ts`) collapsed an unwanted `namedInputs.default` array reflow the first tool's `JSON.parse`/`JSON.stringify` round-trip introduced (multi-line instead of the original single-line array) — every one of the 32 `📋️project.json` diffs is now exactly the new `describe` target block, nothing else.

**Verification of the batch, all 33**: `Bun.Transpiler({loader:"ts"}).transformSync(source)` on every file — 0 syntax errors. `JSON.parse` on every `project.json` — 0 malformed files. (`bun build --target=bun` was tried first and rejected as a check — it fails identically on all 33, including the untouched, hand-written `🗒️note`, on an unrelated pre-existing `playwright-core`/`chromium-bidi` resolution error transitively pulled in by the shared library import graph; not caused by this packet, not a real signal here.)

### The real round-trip proof — `🗒️note`, twice

First attempt hit the `wasi:io/poll@0.2.9` linker gap the coordinator then fixed (WASI never wired into any `Linker` in this repo, plus a fuel budget 18× too small) — **that fix is the coordinator's, not mine**, credited here, not re-claimed. I re-read `📇️describe/📦️packages/🦀️rust/📦️glue.rs`+`Cargo.toml` from disk after the coordinator's message and confirmed the fix (`wasmtime-wasi` dependency, `WasiCtx`/`WasiView`/`add_to_linker_sync`, `DESCRIBE_FUEL_BUDGET = 2_000_000_000`) — my own `📜️script.ts` in the same directory needed no change, it only calls the binary's existing `describe <wasm> --out <dir>` CLI contract, unchanged.

Before that fix landed I still needed to prove the *plumbing* (not the instantiation), so I ran the exact code path (`describe::describe_plugin()`) through a temporary, self-removing native harness in `✏️s/🔌️plugins/🗒️note/🦀️component.rs` (mirrors E2's own `e2_proof_scratch`; added and fully removed, net diff on that file today is only the pre-existing peer edit — verified via `git diff HEAD`), hash-patched with the note wasm's **real** SHA-256 (`shasum -a 256`, cross-checked). That was necessary anyway: a live peer (`CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`) had migrated note's declaration channel since E2 committed the last descriptor, so the committed file was genuinely stale — re-running `describe` is the intended maintenance action, not a workaround.

After the coordinator's fix landed, I re-ran the **real** command (`bun ✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📜️script.ts describe`, no substitution) and diffed its output against the native-substitution version:

```
$ diff terra-D0-note-descriptor-before.json ✏️s/🔌️plugins/🗒️note/🔣️descriptor.json && echo IDENTICAL
IDENTICAL
$ shasum -a 256 terra-D0-note-descriptor-before.{json,semio} ✏️s/🔌️plugins/🗒️note/{🔣️descriptor.json,🛂️descriptor.semio}
445f595ba39e31d94a8a78affdc878ac527dd68830ee9290a0a7f39f9c382b99  terra-D0-note-descriptor-before.json
445f595ba39e31d94a8a78affdc878ac527dd68830ee9290a0a7f39f9c382b99  ✏️s/🔌️plugins/🗒️note/🔣️descriptor.json
da860bf3928eba1ade0b8a0bdf266a2a74c2bf558160de4759ff65a242bd8e4f  terra-D0-note-descriptor-before.semio
da860bf3928eba1ade0b8a0bdf266a2a74c2bf558160de4759ff65a242bd8e4f  ✏️s/🔌️plugins/🗒️note/🛂️descriptor.semio
```

Byte-identical — the real wasmtime-instantiated emitter and my native substitution computed exactly the same descriptor, including the same real wasm SHA-256. This also independently corroborates the coordinator's own claim (their run produced a descriptor sha256-identical to the committed one).

**Second real attempt, `🔋️energy`** (a small plugin, not touched by the live peer ticket's current batch): `bun ✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📜️script.ts describe`. The build step (`cargo build -p semio-s-plugin-energy --target wasm32-wasip2`) succeeded (confirmed `Finished dev profile … in 2m 03s`), but produced only `libsemio_s_plugin_energy.rlib` — no `.wasm`. Root cause: `energy`'s own `[lib]` table in `Cargo.toml` has no `crate-type` key (defaults to `rlib`-only), unlike `note`'s explicit `crate-type = ["cdylib", "rlib"]`. This is a real, pre-existing, per-crate Cargo.toml gap — not this packet's to fix (crate-type is build-migration territory, not descriptor plumbing) and not something my `describe` command should paper over: it correctly built what was asked, then correctly reported the emitter's own honest "file not found" error. No files were written for energy (verified: `🛂️descriptor.semio`/`🔣️descriptor.json` do not exist under its owner root). Per the coordinator's explicit scope correction (don't chase breadth, the peer ticket owns active plugin migrations), I stopped here rather than working through the other 31.

## 3. `descriptor_is_fresh()` hardening — design

**Problem**: previously, a missing committed descriptor was a silent pass — 32 of 33 plugins could (and did) report green while never having emitted anything.

**Design — explicit, visible, per-crate opt-in ratchet list**, entirely inside the macro-generated test body (my only owned surface in this file):

```rust
const DESCRIPTOR_MIGRATED_PLUGINS: &[&str] = &["note"];
match std::fs::read(expected_path) {
    Ok(expected) => { /* unchanged byte-compare-once-hash-blanked, same as before */ }
    Err(_) => {
        assert!(!DESCRIPTOR_MIGRATED_PLUGINS.contains(&plugin_id.as_str()), "…");
    }
}
```

and the symmetric `DESCRIPTOR_MIGRATED_EXTENSIONS: &[&str] = &[]` (starts empty — no extension has committed one yet) in `extension_exports!`.

- **A crate not in the list**: behaves exactly as before — missing descriptor is a soft, silent pass. This is the "guard the change so the fleet does not go red before D1..Dk land" requirement.
- **A crate in the list**: missing descriptor is now a **hard `cargo test` failure**, not silent. Verified: `cargo test -p semio-s-plugin-note --lib` → `test descriptor_is_fresh ... ok` when the file is present and fresh; deleting the committed file (not done permanently, just reasoned through/would-fail) would flip it red because `"note"` is in the list.
- **Visible per plugin**: the list is a literal array of plugin/extension ids, in one place, in a doc-commented region explaining the convention ("extend it, never shrink it, as each D-packet lands"). A future packet emitting a real descriptor for e.g. `stdio` adds `"stdio"` to this one array.
- **Repo-wide ratchet, not just this test**: `📇️registry:check`'s own pre-existing `validateDescriptors` census line (`descriptor gate: N/<total> crates have a 🔣️descriptor.json.`) is the thing that should trend toward `<total>` as the list above grows — I did not build new census tooling since a correct one already exists (it's just currently blind to note's descriptor because of the `🤖️generated/` path bug, §5).
- `plugin_id`/`extension_id` are read via `plugin_manifest().plugin_id` / `extension_manifest().extension_id` **after** `__semio_install_plugin_bundle()`/`__semio_install_extension_bundle()` — verified safe: both accessor functions internally call `ensure_plugin_initialized()`/`ensure_extension_initialized()`, which are `#[cfg(feature = "component-guest")]`-gated no-ops on a native `cargo test` and, for the extension path, explicitly skip re-installing when the bundle slot is already `Some`.

This cannot silently report green for a plugin that has emitted nothing **once it opts in** — before opt-in it makes no freshness claim at all (honest: it hasn't tried), which is the deliberate transition state the packet brief asked for.

## peer-coexistence

Mandatory liveness check before editing `🔌️plugin/🦀️component.rs` (table entry in `important.md`'s live-peer warning) and `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (heavily shared, confirmed live during this session — see below):

| file | `git log -5` (real dates via `--date=iso`) | mtime at check time | cargo/build activity | verdict |
|---|---|---|---|---|
| `🔌️plugin/🦀️component.rs` | latest commit 2026-08-18 13:00:07 (`830f2a4269`) | 12:22:26 (before my session) | `ps`/`pgrep` found no cargo/rustc process touching this file at check time | **not a moving target** — edited surgically inside the `descriptor_is_fresh` region only |
| `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` | not checked via mtime — `git diff HEAD` after my edit showed **substantial unrelated content already present** (a `publishShardWorker`/`SHARD_WORKER_FILE` region, a `bench` region — packet H2/V1b-bench, same ticket, different packet) | n/a | none observed mid-edit | **live peer content confirmed present, absorbed correctly** — my edit is confined to the `describeBuiltPlugin` function body + its doc comment; `git diff HEAD` after my edit shows exactly that region changed, the peer's H2/bench additions sit untouched around it |
| `✏️s/🔌️plugins/🗒️note/🦀️component.rs` | live peer (`CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`) staged diff present at session start (`plugin_assembles_a_real_manifest_not_the_assembly_failed_stub` test + the `.declare_artifact(…)` migration) | n/a | `git status --porcelain` showed staged (`M `), not actively re-touched during my session | **absorbed, not overwritten** — my temporary `d0_proof_scratch` module was added after and fully removed before finishing; `git diff HEAD` on this file today shows only the peer's pre-existing addition |

No file was skipped due to contention. The `🗒️note` descriptor going stale mid-session (§2) *was* exactly this kind of peer traffic surfacing correctly — the freshness test caught real drift from a real concurrent migration, then `describe` (this packet's own deliverable) fixed it.

## 4. `lease-request` — registry `DESCRIPTOR_JSON_REL_PATH`

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts

edit 1 — line 150:
-const DESCRIPTOR_JSON_REL_PATH = ["..", "..", "🤖️generated", "🔣️descriptor.json"];
+const DESCRIPTOR_JSON_REL_PATH = ["..", "..", "🔣️descriptor.json"];

edit 2 — doc comment, lines 144-149 (above the constant): replace the "🤖️generated/" convention
description with: descriptors live at the plugin/extension OWNER ROOT (two levels up from the crate's
Cargo.toml, sibling of the tracked 🛂️manifest.json), matching descriptor_is_fresh() (component.rs,
fixed by E2-builder-descriptor) and the only real committed descriptor in the repo
(✏️s/🔌️plugins/🗒️note/🛂️descriptor.semio + 🔣️descriptor.json). NOT under a further 🤖️generated/
segment — that directory is globally gitignored (.gitignore ~87-88), so nothing written there can
ever survive a commit.

That is the ONLY code change needed: readDescriptorJson (:186-194) and validateDescriptors's
descriptorPath (:1911) both already derive from this one constant — no other line references the old
path.

why: this is the last leg of the three-way path disagreement this packet was sent to close. Verified
live and current: `bun nx run @semio-tech/plugin-registry:check` (terra-D0-registry-check1.txt, this
ticket folder) still prints `note: no ✏️s/🔌️plugins/🗒️note/🤖️generated/🔣️descriptor.json yet` even
though note has a real, fresh, committed descriptor sitting at the owner root right now (verified
`cargo test -p semio-s-plugin-note --lib` → `descriptor_is_fresh ... ok`). Every other piece
(descriptor_is_fresh, dev script's describeBuiltPlugin, every plugin's own new describe command) now
writes/reads the owner root; only this file still disagrees.

scope note: 📌️important.md lists `🔌️plugin/📦️packages/🟦️typescript/📇️registry/**` as registrar-only.
That literal path does not exist in the tree today — the real registry lives at
`🔌️plugin/📇️registry/**` (confirmed via `find … -maxdepth 3`, no `📇️registry` directory exists
anywhere under `📦️packages/🟦️typescript`). This packet's own path_scope does not list the registry
directory under any name (unlike component.rs and the dev script, which ARE explicitly scoped), and
explicitly instructs: file a lease if registry turns out to be registrar territory. Given the
important.md glob's clear intent (the registry TypeScript package is registrar-owned) and that
D0's path_scope deliberately excludes it, I filed this instead of editing directly, even though an
earlier packet in this same ticket (E1-describe) DID edit this file directly under its own,
different, path_scope.
```

## Files touched

**Modified:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `descriptor_is_fresh()` region only, both `plugin_exports!`/`extension_exports!` macros (hardening, §3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` — `describeBuiltPlugin()` function body + doc comment only (owner-root path fix, §1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts` — added `cargoTargetRoot`, `pluginWasmArtifactPath`, `describePluginComponent` (exported); fixed `ensureBuiltBin`'s `CARGO_TARGET_DIR` blindness (§2)
- `✏️s/🔌️plugins/*/📦️packages/🦀️rust/📜️script.ts` (all 33) + matching `📋️project.json` (all 33) — `describe` command + nx target (§2)
- `✏️s/🔌️plugins/🗒️note/🛂️descriptor.semio` + `🔣️descriptor.json` — regenerated fresh against the current (post-peer-migration) declaration tree; real emitter run, real wasm hash (§2)

**Not touched (lease filed):** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (§4)

**Scratch (ticket folder, left in place per process):**
- `d0-add-describe-command.ts`, `d0-fix-project-json-reflow.ts` — batch-registration tools, idempotent, safe to re-run
- `terra-D0-*.txt` — every command's raw output (§ below)
- `terra-D0-note-descriptor-before.{json,semio}` — the native-substitution descriptor, kept for the diff-check in §2
- `🎯️target-d0/` — this packet's cargo target dir

## Acceptance commands — verbatim exit codes

```
$ export CARGO_TARGET_DIR=…/🎯️target-d0
$ cargo check -p semio-framework-plugin --lib
    Finished `dev` profile [unoptimized] target(s) in 4m 11s   (only pre-existing warnings, none touching my edits)
$ echo $?
0
```

```
$ bun ✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📜️script.ts describe      ← the real describe round-trip
described …/semio_s_plugin_note.wasm ("note", role=Plugin) -> …/✏️s/🔌️plugins/🗒️note/🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256=a60a593e311b5e4b6e366884638095c8dec2aa0e6bed9792163d6f2cef35a5b7)
$ echo $?
0
```

```
$ cargo test -p semio-s-plugin-note --lib
test descriptor_is_fresh ... ok
test result: ok. 115 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
$ echo $?
0
```

```
$ bun nx run @semio-tech/plugin-registry:check
… (still) note: no ✏️s/🔌️plugins/🗒️note/🤖️generated/🔣️descriptor.json yet — run `bun ./📜️script.ts describe` in ✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust after building its wasm32-wasip2 component
 NX   Successfully ran target check for project @semio-tech/plugin-registry
$ echo $?
0
```
**Before/after count: unchanged (0 crates seen), by design.** `check`'s own count cannot move until the `lease-request` above lands — I verified this honestly rather than papering over it: the registry file still reads the old path, so it is blind to note's real, fresh, committed descriptor. This is the single remaining gap in the plumbing chain.

```
$ bun ✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📜️script.ts describe    ← second real attempt, different plugin
    Finished `dev` profile [unoptimized] target(s) in 2m 03s            ← cargo build succeeded
semio-framework-plugin-describe describe: reading …/semio_s_plugin_energy.wasm: No such file or directory (os error 2)
$ echo $?
1
```
Honest negative result — root cause identified (§2), not this packet's to fix, no files written.

## Remaining gaps

1. **Registry `check` still cannot see any committed descriptor** — blocked on the lease in §4. This is the single blocking gap left in the "make one path canonical" half of the packet.
2. **32 of 33 plugin crates have `describe` wired but unproven** — only `note` (real round trip) and `energy` (honest failure, pre-existing crate-type gap) were actually run. The other 31 have the same mechanical registration, syntax-checked, not build-tested — per the coordinator's explicit instruction to prefer proving the mechanism over breadth, and because most of those 31 crates are either live-peer territory (`CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`'s batch 1/2 plugins) or have their own unknown build gaps (like energy's missing `crate-type`) that are each a separate D-packet's problem, not this plumbing packet's.
3. **`extension_exports!`'s hardened test body was not exercised end-to-end** — no extension crate's `#[cfg(test)]` currently invokes it in a way I confirmed compiles+runs (one sample extension crate, `semio-s-plugin-imperative-math`, compiled and tested clean, but doesn't itself invoke `extension_exports!`). The macro DEFINITION compiles cleanly (`cargo check -p semio-framework-plugin --lib`, exit 0), and it is structurally identical to the plugin macro's already-proven body, but a real extension invocation was not run.
4. **`DESCRIPTOR_MIGRATED_PLUGINS`/`DESCRIPTOR_MIGRATED_EXTENSIONS` are the honest, minimal ratchet lists as of this packet** — `["note"]` and `[]`. Every future D-packet that lands a real descriptor for another crate must add that crate's id here, in the same file, same region — this is the enforcement mechanism, not a suggestion.
