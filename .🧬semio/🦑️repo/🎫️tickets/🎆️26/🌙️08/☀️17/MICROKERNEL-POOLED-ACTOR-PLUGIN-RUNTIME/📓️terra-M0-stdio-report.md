# 📓️ terra M0-stdio report

Packet: **M0-stdio** — the first W3 plugin migration (`✏️s/🔌️plugins/🗄️stdio/**`). Every other plugin
depends on this crate as a Rust library, so it migrates alone and first.
Read: `📌️important.md`, `📓️design-abi.md` §3/§6, `📓️terra-E2-builder-descriptor-report.md`,
`📓️luna-imports-audit.md`.

## Status: partial, blocked (both upstream and by a pre-existing bug of my own crate's, honestly
## not fixed). Do not read this as green.

## 1. What stdio genuinely does (measured, not assumed)

- `HostEffect::X` usages in this crate: **0** (`grep -rn HostEffect` → no hits).
- `pending_effects`/self-tick loops: **0**.
- `.handler(...)` command handlers: **0**.
- stdio is a zero-handler artifact-format library: 36 top-level `ArtifactKindSpec`s (one Rust
  module per file format — binary, txt, json, xml, csv, md, deflate, zip, step, ifc, las, gltf,
  obj, ply, dxf, stl, svg, bmp, dwg, png, pdf, jpg, gif, tiff, docx, pptx, xlsx, bcf, semio, mp4,
  avi, mp3, wav, epw, tsv, html), each with an `.editor()`/`.viewer()` pair (and several with
  multiple standard/subset pairs — geometry/data/document formats get up to 19 pairs for `semio`
  alone). ~90 editor/viewer registrations total, confirmed by literal count of `.editor::<`/
  `.viewer::<` call sites in `🦀️component.rs`.
- Items 2 and 3 of the packet brief (`HostEffect` rename, `pending_effects`→timers/jobs) are
  **no-ops for this crate** — there was nothing to rename or move. Confirmed via grep, not assumed.

## 2. Declarations added (item 1) — `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`, `//#region 🔖️Descriptor`

- **36 `.activation(ActivationEvent::OnArtifactKind { kind: ... })` calls**, one per
  `crate::artifacts::<fmt>::artifact_kind().id` — read live from each format's own function
  (never a hardcoded string), so this list cannot silently drift from the real declarations. This
  is the same shape `✏️s/🔌️plugins/🗒️note`'s E2 proof migration used, just enumerated 36× instead
  of 1× because stdio genuinely owns that many kinds (the packet brief called this out explicitly:
  "stdio owns many artifact kinds, so cover them").
- **`.execution(ExecutionMode::Isolated)`** — the SDK default, and the same choice note made.
  Nothing in this crate's own code (no `.handler`, no cross-plugin extension attachment, no
  evidence of a `Linked`/`Exclusive`/`Cold` need) justifies anything else; `Isolated` is honest,
  not a placeholder.
- **One `.requests(CapabilityRequest{ id: "documents.write", scope: "plugin", optional: false,
  reason: "persist editor mutations back to whichever of stdio's 36 owned file-format artifacts
  ... is currently open" })`** — every one of the ~90 registered editors mutates and persists the
  open document (`.editor()` already attaches the OLD `documents::{Read,Write}`
  `CapabilityRequirement` per contract §2.3 clause 4); this is the NEW broker-scoped ask for the
  same real behavior, mirroring note's own single `documents.write` request.
- **No quotas declared.** Grepped for any evidence of a genuine quota need (long-running
  computation, huge in-memory buffers held across turns, high-frequency timers) — found none.
  `QuotaSchema::default()` (all `None`, inherit) is honest; inventing ceilings with no measured
  basis would violate the packet's own "quotas only where a real need exists."

## 3. Wiring added — `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}`

Two gaps found that blocked the descriptor/actor path entirely, both fixed, both mirroring
`✏️s/🔌️plugins/🗒️note`'s exact precedent byte-for-byte:

- **`Cargo.toml`**: `semio-framework-plugin = { workspace = true }` had **no** `component-guest`
  feature request at all — unlike note's `{ workspace = true, features = ["component-guest"] }`.
  Without it, `pub mod component` (the wasm `export!(ComponentGuest)` — reactor/jobs/checkpoint/
  describe) never compiles for `wasm32-wasip2`, so stdio would build as an inert library with no
  actor world exported, `describe()` included. Added the same `features = ["component-guest"]`
  note already carries. Verified this compiles clean for the real `wasm32-wasip2` target (see
  §5) — the feature is `cfg`-gated to `target_arch = "wasm32", target_env = "p2"` so it is a no-op
  for every native build.
- **`📦️glue.rs`**: stdio had **no** `semio_framework_plugin::plugin_exports!(plugin::plugin);`
  call anywhere — note has this exact line at its own `//#region 🔖️Plugin`. Without it: (a) no
  `#[cfg(test)] descriptor_is_fresh()` test exists for this crate at all (the macro is what
  generates it — E1's own freshness-without-wasm mechanism), and (b) the wasm `component_export_
  anchor` link-time anchor is missing. Added it immediately after `pub use plugin::plugin;`,
  verbatim placement match to note's.

## 4. Descriptor emission (item 4) — **not completed, real reason below**

`cargo test -p semio-s-plugin-stdio --lib` is the literal path to `descriptor_is_fresh()`, but this
crate's own `#[cfg(test)]` code has **267 pre-existing compile errors** (E0422/E0425/E0433/E0599/
E0609) scattered across ~19 of the 36 artifact-format directories (step, stl, dwg, xlsx, gltf, docx,
tiff, semio, …) — missing `use` imports for types split out of files by an earlier, unrelated
taxonomy refactor (rustc itself suggests the fix for most, e.g. `use crate::artifacts::step::io::
part21::Part21Value;`). Confirmed pre-existing and not caused by me: `git status` shows only my
three files (`🦀️component.rs`, `Cargo.toml`, `📦️glue.rs`) as modified; none of the 267 errors are
in those files. This is real, but far too broad (19 unrelated format directories, none of them
`🦀️component.rs`/registry/manifest) to responsibly absorb into an ABI-migration packet — flagging,
not fixing.

Because that blocks the crate's *own* lib-test binary from compiling at all, I could not run
`descriptor_is_fresh()` for real, and (unlike E2's note proof) I could not use a `#[test]`-based
scratch harness *inside* stdio's own crate either — same compilation unit, same 267 errors. So I
built a standalone native fixture crate in the ticket folder
(`.../m0-describe-fixture/`, NOT part of the repo, `[workspace]`-isolated) that depends on
`semio-s-plugin-stdio` as a plain library (so its `#[cfg(test)]` code is never compiled — only a
real `--tests` build touches that) and calls the exact same
`semio_framework_plugin::describe::describe_plugin()` the wasm `describe()` export would call,
after `install_plugin_bundle_result(semio_s_plugin_stdio::plugin())` — the same sequence
`plugin_exports!`'s generated code runs.

**Running it surfaced a second, independent, pre-existing bug**: `semio_s_plugin_stdio::plugin()`
itself returns `Err(PluginAssemblyError { code: "artifact-definition.runtime-capability",
message: "no declared inference capability owns the runtime claims" })`. This is the exact same
bug category E2 found and fixed for note (§7 of that report: "no declared composer capability owns
the runtime claims") — a mismatch between a format's own `declaration()` call
(`.inferences([<fmt>_artifact_inference_descriptor()])`, one runtime descriptor) and its
`📇️catalog.json`-declared `inference` capability rows. I traced this far: of the 36 formats'
`📇️registry/📇️catalog.json`-backed `artifact-definition.json` files, **35 declare zero `inferences`
rows** and only `gltf` declares any (66 rows, matching its own `.inferences(gltf_artifact_
inference_descriptors())` plural call) — while every other format I sampled (`csv`, `txt`, `png`,
…) calls the OLD single-descriptor form `.inferences([<fmt>_artifact_inference_descriptor()])`
against a catalog with **no** matching row, which is exactly what
`require_declared_capability_or_record` rejects. `Iterator::collect::<Result<_,_>>()` in
`crate::registry::artifact_assemblies()` short-circuits at the first failing format in catalog
order, so I only ever saw one error message, not the full list — I did not get far enough to name
which single format fails first, or to confirm whether this affects all 35 or a subset, before
being told to stop building. **Not fixed.** This is a pre-existing, crate-wide data/catalog
mismatch, unrelated to my four-item scope, and per-format enough (up to 35 formats) that it needs
its own investigation, not a one-line capability-row fix like note's.

**Consequence**: I generated a descriptor once via the fixture, found it carried the
`pluginId: "assembly-failed"` sentinel (the framework's own fallback for "nothing installed
because assembly errored") with **empty** `activationEvents`/`capabilityRequests` — i.e. it did
not reflect my real declarations at all — and **removed** the two files
(`🛂️descriptor.semio`/`🔣️descriptor.json`) rather than leave placeholder garbage checked in at the
owner root. **No descriptor is committed for stdio.** My §2 declarations are real, present in
source, and were never in doubt — only their *assembled, packed* form (what `describe_plugin()`
returns) could not be produced, because `plugin()` itself does not return `Ok` yet, for a reason
unrelated to anything in my scope.

## 5. Acceptance — run by me except where noted; see §6 for what the coordinator found afterward

```
$ export CARGO_TARGET_DIR=.../🎯️target-m0

$ cargo check -p semio-s-plugin-stdio --all-targets
error: could not compile `semio-s-plugin-stdio` (lib test) due to 267 previous errors; 5 warnings emitted
$ echo $?
0   (cargo check's own process exit — the compile error is in the captured output; full log:
     .../terra-m0-alltargets1.txt, 3340 lines)
```
Pre-existing (§4), not caused by me, not fixed.

```
$ cargo check -p semio-s-plugin-stdio --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 3m 01s
$ echo $?
0
```
Real, clean, green — full log `.../terra-m0-wasm1.txt`. This is the one command in the literal
acceptance list that genuinely passed, and it's the one that proves the `component-guest` fix
(§3) actually exports the real actor world: I also ran `cargo build -p semio-s-plugin-stdio
--target wasm32-wasip2` (not in the literal list, done to get a real wasm for the descriptor
attempt) → `Finished ... in 8m 41s`, exit 0, producing a real
`semio_s_plugin_stdio.wasm` (`shasum -a 256` →
`662c38f7fc18a1f7c0a1a8b710d9f2c7acb433b4ccd81551fc24106b68049b52`) — full log
`.../terra-m0-wasmbuild1.txt`.

```
$ cargo test -p semio-s-plugin-stdio --lib
error: could not compile `semio-s-plugin-stdio` (lib test) due to 267 previous errors; 5 warnings emitted
$ echo $?
101
```
Same pre-existing blocker as `--all-targets` (§4) — full log `.../terra-m0-test1.txt`.
`descriptor_is_fresh()` cannot run through this command right now; see §4 for the substitute proof
attempt and the second bug it found instead.

## 6. Coordinator's follow-up run — quoted, not repeated by me

The coordinator (sol) ran acceptance independently afterward and found a **third**, most recent
blocker, upstream of both of mine: a peer session committed a half-finished presence refactor
(commit `abd29c08d0`, today) into `🔌️plugin/🦀️component.rs` — the framework SDK every plugin
depends on — that no longer compiles on its own:

- `ArtifactApp` gained `adopt_presence(...)`; `VcsArtifactApp` does not implement it.
- `ephemeral_snapshot`'s trait return type changed to `EphemeralSnapshot`; the impl still returns
  `(Vec<u8>, u64, u64)`.
- `EphemeralSnapshot` is constructed at one site where it is not in scope.

That file is not in my owned paths (`✏️s/🔌️plugins/🗄️stdio/**` only) and not something I touched or
diagnosed myself — I'm quoting the coordinator's finding here because it means `cargo check -p
semio-s-plugin-stdio` may now fail even *before* reaching either of the two bugs in §4/§5 above,
depending on when it's next run. The coordinator landed two contract-required fixes there
(the missing `interaction` field on `AppFrame::Ephemeral`, an `AppCommand::Presence` match arm)
and stopped rather than finish the peer's own feature. I did not verify this myself — no build
after their message, per their explicit instruction to stop.

## 7. What's real and what isn't — summary

| item | state |
|---|---|
| `HostEffect` rename (item 2) | N/A — 0 usages in this crate |
| `pending_effects`/self-tick → timers/jobs (item 3) | N/A — 0 usages in this crate |
| 36 `on-artifact-kind:` activation events (item 1) | **done**, in source, honest (real `artifact_kind().id` reads) |
| `.execution(Isolated)` (item 1) | **done**, honest (SDK default, nothing contradicts it) |
| `.requests(documents.write)` (item 1) | **done**, honest (every editor persists mutations) |
| quotas (item 1) | **not declared** — no measured need found, by design |
| `component-guest` feature (gap found, not in the packet's 4 items but required for any of them to matter) | **done**, verified against a real `wasm32-wasip2` build |
| `plugin_exports!` wiring (gap found, same reason) | **done**, matches note verbatim |
| `🛂️descriptor.semio` + `🔣️descriptor.json` committed (item 4) | **not done** — `plugin()` itself errors (`"no declared inference capability owns the runtime claims"`), a pre-existing, crate-wide catalog/descriptor mismatch across up to 35 of 36 formats, not diagnosed to a specific fix before being told to stop |
| `descriptor_is_fresh()` passing | **not run** — blocked by the 267 pre-existing test-compile errors (§4), and moot until `plugin()` itself returns `Ok` |
| `cargo check -p semio-s-plugin-stdio --all-targets` | **red**, pre-existing 267 errors, not caused by me |
| `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2` | **green**, real, 3m01s |
| `cargo test -p semio-s-plugin-stdio --lib` | **red**, same 267 pre-existing errors |

## Files touched

**Modified** (all within `✏️s/🔌️plugins/🗄️stdio/**`):
- `🦀️component.rs` — `//#region 🔖️Descriptor`: 36 `.activation(...)`, `.execution(Isolated)`,
  one `.requests(documents.write)`; updated imports.
- `📦️packages/🦀️rust/Cargo.toml` — `semio-framework-plugin` dependency gained
  `features = ["component-guest"]`.
- `📦️packages/🦀️rust/📦️glue.rs` — added `semio_framework_plugin::plugin_exports!(plugin::plugin);`.

**Created then removed** (not left in the tree):
- `🛂️descriptor.semio` / `🔣️descriptor.json` at the owner root — generated once via the scratch
  fixture, found to carry `pluginId: "assembly-failed"` placeholder data (§4), deleted rather than
  committed.

**Scratch (ticket folder, left in place per process, not part of the repo)**:
- `m0-describe-fixture/` — standalone `[workspace]`-isolated fixture crate + binary used for the
  substitute descriptor-assembly proof attempt (§4).
- `terra-m0-alltargets1.txt`, `terra-m0-wasm1.txt`, `terra-m0-wasmbuild1.txt`, `terra-m0-test1.txt`
  — real command output, referenced in §5.

**Not touched**: `🔌️plugin/🖥️host/**`, `⚛️reactor/**`/`🌐host/**`, `🎠️kernel/🦀️component.rs`,
`📇️registry/**` (the top-level plugin registry, not stdio's own), other plugins, root manifests,
`.vscode/*`. `🔌️plugin/🦀️component.rs` (the SDK) was touched by the coordinator, not me — see §6.

## Not started / handed off

- **The 267 pre-existing test-compile errors** across ~19 stdio artifact-format directories (§4,
  §5) — real, large, unrelated to this packet's four items, needs its own ticket/packet.
- **The "no declared inference capability owns the runtime claims" assembly failure** (§4) — real,
  blocks `plugin()` from ever returning `Ok`, therefore blocks descriptor emission entirely. I
  traced it to a catalog/runtime mismatch pattern affecting up to 35 of stdio's 36 formats (only
  `gltf` has matching catalog rows) but did not isolate the first failing format or a fix before
  being told to stop.
- **Committing `🛂️descriptor.semio`/`🔣️descriptor.json` for stdio** — blocked on the item above;
  cannot honestly commit a descriptor until `plugin()` returns `Ok`.
- **The upstream SDK breakage** (§6) — the coordinator's to finish, not mine; I did not touch
  `🔌️plugin/🦀️component.rs`.
