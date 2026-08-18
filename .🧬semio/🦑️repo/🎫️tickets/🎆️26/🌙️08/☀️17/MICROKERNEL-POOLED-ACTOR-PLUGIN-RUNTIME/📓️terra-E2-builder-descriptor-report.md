# 📓️ terra E2-builder-descriptor report

Packet: **E2-builder-descriptor** — the builder API that lets a plugin declare what the descriptor carries, plus a real end-to-end proof migration (`✏️s/🔌️plugins/🗒️note`).
Read: `📌️important.md`, `📓️design-abi.md` §3, `📓️terra-E1-describe-report.md`.

## Honest status summary

| item | state |
|---|---|
| `PluginBuilder` `.activation`/`.extension_point`/`.requests`/`.quota`/`.execution`/`.asset` | **done** |
| `ExtensionBundle` `.mode`/`.requests` (`.extends` pre-existing) | **done** |
| `describe_plugin()`/`describe_extension()` assemble a real `PackageDescriptor` (not empty) | **done** |
| `ContributionSet` populated from real declarations (commands/panels/file_types/topic/artifact/inference/io/composer) | **done**, `menus`/`themes`/`mutation_services` stay empty — real gaps, not invented (see below) |
| Design fork: extend `Plugin`/`PluginManifest` vs. side-channel registry | **decided — side-channel, for the plugin side only** (justification below) |
| `descriptor_is_fresh()` hash-blank fix | **done** — a real, previously-undiscovered gap in E1's own test (see below) |
| Proof migration: `✏️s/🔌️plugins/🗒️note` gets `.activation`/`.execution`/`.requests` | **done** |
| Real emitter run against note's built wasm | **blocked** (pre-existing, out-of-scope wasi-import issue — see below); **substituted** with an equivalent native harness that calls the exact same `describe_plugin()` and hash-patches with the real wasm's real SHA-256 |
| `🛂️descriptor.semio` + `🔣️descriptor.json` committed at note's owner root | **done**, real data, `descriptor_is_fresh()` passes against it |
| Three unrelated pre-existing bugs discovered while verifying | **two fixed** (in-scope, `✏️s/🔌️plugins/🗒️note/**`), **one flagged, not fixed** (out of scope) |

## 1. Design fork: extending `Plugin`/`PluginManifest` vs. a side-channel — decided

E1's report presented two options and asked this packet to pick one. I split the decision by role:

- **Extensions: extended `ExtensionManifest` directly** (added `execution: ExecutionMode`, `capability_requests: Vec<kernel::CapabilityRequest>`). This is the "single source of truth" option E1 preferred in principle, and it was actually reachable here: `ExtensionBundle`/`ExtensionManifest` are defined *inside* the `//#region 🧩️Extension` subregion of `plugin_runtime` — squarely inside my owned "`plugin_runtime describe region`" — and have exactly two construction sites (`ExtensionBundle::new()`, `extension_manifest()`'s fallback), both in the same file. No cascade risk.
- **Plugins: a side-channel (`plugin_runtime::PluginDescriptorExtras`, a thread-local installed by `PluginBuilder::try_build()`)**. `Plugin`/`PluginManifest` are defined in the `app` module of `🔌️plugin/🦀️component.rs`, which is **not** in E2's owned paths (only "`builder + plugin_runtime describe region`" is — `important.md` rule 3 is explicit that a region name inside a shared file is not ownership). Concretely, `PluginManifest` is constructed as a full struct literal at **~10 sites across files I do not own** (`🔌️plugin/🖥️host/🦀️component.rs`, `📺️renderer/…/Shell/🧊️component.rs`, `🌉️mcp/🧫️fixtures/🦀️component.rs`, plus `🖥️host`'s test fixtures) — adding required fields there would break compilation in all of them, which is a change I have no lease for and B1b/other packets are actively live in some of those exact files right now. The side channel avoids this cascade entirely.

Why the side channel doesn't "drift" in the sense E1's report worried about: `PluginBuilder::try_build()` installs `PluginDescriptorExtras` **at the exact same call**, from the **exact same builder fields**, that constructs `Plugin` itself (`🏗️builder/🦀️component.rs`, right before `Ok(plugin)`). There is no second, independently-maintained copy that could diverge over time — it's a second output of one assembly step, not a parallel registry. `describe_plugin()` then reads `plugin_manifest()` (existing) and `plugin_descriptor_extras()` (new) — the "same builder output `plugin_manifest()` already reads" the packet brief asked for.

## 2. Builder additions

`🏗️builder/🦀️component.rs` — `PluginBuilder<Ready>` gained a `//#region 🔖️Descriptor`:
- `.activation(kernel::ActivationEvent)`, `.extension_point(ExtensionPointDeclaration)`, `.requests(kernel::CapabilityRequest)`, `.asset(AssetDeclaration)` — all idempotent-push (mirrors `.capability`'s existing idiom).
- `.execution(ExecutionMode)` — overwrite (single value, default `Isolated`).
- `.quota(QuotaSchema)` — **merges** field-by-field via a new `merge_quota_schema` helper (`incoming`'s `Some` fields win, `None` defers to what's already set) — chosen because `QuotaSchema` is deliberately wide (16 independent `Option` fields per `📓️design-abi.md` §5) and a plugin author calling `.quota(..)` twice for two different concerns (e.g. memory once, timers once) should not clobber the first call.

`try_build()` destructures the six new builder fields and, right before `Ok(plugin)`, calls `crate::plugin_runtime::install_plugin_descriptor_extras(PluginDescriptorExtras { .. })`.

`🔌️plugin/🦀️component.rs`, `plugin_runtime` module — new `pub struct PluginDescriptorExtras { activation_events, capability_requests, extension_points, execution, quotas, assets }`, a `thread_local!` slot, `install_plugin_descriptor_extras`/`plugin_descriptor_extras()` (default when unset), placed next to the existing `PLUGIN` thread-local / `plugin_manifest()` — same shape, same lifecycle.

`ExtensionBundle` gained `.mode(ExecutionMode)` and `.requests(kernel::CapabilityRequest)` (idempotent-push, mirrors `.capability`). `.extends(..)` was already present (per the brief's "if not already present").

## 3. `describe_plugin()`/`describe_extension()` — real assembly

`🛂️describe/🦀️component.rs` (E1's file; touched under the same "this packet's own owned path" precedent E1 itself documented in that file's header comment, which explicitly deferred the builder wiring to this packet).

`describe_plugin()`:
```rust
let manifest = crate::plugin_runtime::plugin_manifest();
let extras = crate::plugin_runtime::plugin_descriptor_extras();
let contributions = plugin_contributions(&manifest);
PackageDescriptor { descriptor_version: 1, role: PackageRole::Plugin, manifest,
    activation_events: extras.activation_events, capability_requests: extras.capability_requests,
    extension_points: extras.extension_points, execution: extras.execution, quotas: extras.quotas,
    contributions, assets: extras.assets, hashes: <empty, see §6> }
```

`ContributionSet` field sourcing (`plugin_contributions`):
- `commands` ← `manifest.commands` (direct).
- `topic_contributions` ← `manifest.topic_contributions` (direct).
- `artifact_contributions` ← `manifest.contributions` (direct — already the same typed shape).
- `file_types` ← flattened from every app's `AppIo.export_formats`/`import_formats` + `document_media_type`, one row per distinct format kind.
- `panels` ← flattened from every app's `panel_tabs`.
- `inference_services` ← `crate::app::list_artifact_inference_services()` filtered to `owner == plugin_id`, mapped field-for-field onto `ContributedInferenceMetadata` (`contributor = owner`, `depends_on = []`) — a single-plugin wasm instance never has another plugin's services registered, so `owner == plugin_id` is a correct, not just convenient, filter.
- `io_entries`/`composer_entries` ← `semio_framework::io::list_composer_entries()` (the real `IO_REGISTRY`), filtered to composer rows this plugin owns (`writes.artifact_kind`), each `(writes, reads)` row expanded into one `IoEntryDescriptor{owner: writes, counterpart: read, direction: Import}` per read dialect plus the `ComposerEntryDescriptor` verbatim.
- `menus`/`themes` stay empty `Vec<DescriptorEntry>` — E1's own survey found no declared-contribution precedent for either; I re-checked and found none either.
- `mutation_services` stays empty — a **newly-found real gap**: the owner mutation roster (`crate::app::mutation_roster_entries`, `WireMutationRosterEntry`) never carries `schema_version`/`algorithm_version` for a document app's own `SemanticMutation::kinds()` rows (only `ContributedMutationMetadata`, which `mutation_services` needs, requires both — and only the *inference* registry tracks versions today). Populating it would mean fabricating version numbers nothing in the codebase declares, which the packet brief explicitly said not to do. Flagged, not invented, same discipline E1 applied to `menus`/`themes`.

**Ownership filter** (`owns_artifact_kind`): every plugin's own IO `Dialect.artifact_kind` in the tree is the bare `"s.<plugin_id>"` coordinate — confirmed by grepping every `const DIALECT: Dialect` under `✏️s/🔌️plugins/**/🚪️io/🦀️component.rs` (`"s.note"`, `"s.flow"`, `"s.raster"`, `"s.jack"`, …), distinct from the *separate* 3-segment `s.<plugin>.<artifact>` `ArtifactRef`/`ArtifactIdentity` capability-row grammar. I got this wrong on the first pass (assumed the 3-segment grammar applied here too, which would have silently produced zero `io_entries`/`composer_entries` for every plugin) and caught it only because the note proof run made the mismatch visible — see §7.

`describe_extension()` — same shape, sourced from `ExtensionManifest` directly (no side channel needed there — see §1). `activation_events`/`extension_points` stay empty: extension points are published *by* host plugins (`PluginBuilder::extension_point`), never declared by the attaching extension, and an extension's own activation is entirely driven by the host's `ExtensionPointDeclaration.activation`.

## 4. `descriptor_is_fresh()` — a real bug found and fixed

Two independent things needed fixing in E1's freshness-test macro (`plugin_exports!`/`extension_exports!`, still inside my owned "plugin_runtime describe region"):

1. **Path.** The macro read `<CARGO_MANIFEST_DIR>/../../🤖️generated/🛂️descriptor.semio`, but this packet's own registrar ruling (`📌️important.md`) supersedes `📓️design-abi.md` §3's `🤖️generated` path: descriptors live at the plugin/extension **owner root**, sibling of the tracked `🛂️manifest.json` — not under `🤖️generated/`, which is globally gitignored (`.gitignore` 87-88) and would mean a "checked-in" descriptor could never survive a commit. Fixed both macro instances to `<CARGO_MANIFEST_DIR>/../../🛂️descriptor.semio`.
2. **Hash comparison (found empirically, not anticipated).** Once I actually committed a *real*, hash-patched descriptor for note (see §6), `descriptor_is_fresh()` failed — always. The checked-in file carries real `wasm_sha256`/`core_wasm_sha256`/`descriptor_sha256` (patched in by the emitter after instantiating the built wasm); a *native* `describe_plugin()`/`describe_extension()` call (which is what this test does, deliberately, to avoid needing wasm) can **never** produce those hashes — the guest genuinely cannot know its own already-built bytes. Byte-comparing the two as originally written would make this test permanently red for **every** crate that ever commits a real descriptor, which is not drift, it's an unavoidable difference in *how* the two sides ran `describe()`. Added `plugin_runtime::descriptor_bytes_with_blank_hashes(bytes) -> Option<Vec<u8>>` (decode → blank `hashes` → re-encode) and made both macros compare the hash-blanked bytes instead of the raw ones. Hash correctness stays `📇️registry:check`'s job (built wasm's SHA-256 vs `hashes.wasm_sha256`), never this test's.

This is a real design gap in infrastructure E1 built, discoverable only once a real descriptor got committed — which had never happened before this packet (0/59 plugins had one).

## 5. Proof migration — `✏️s/🔌️plugins/🗒️note`

`✏️s/🔌️plugins/🗒️note/🦀️component.rs`'s `plugin()` gained:
```rust
.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::note::artifact_kind().id })  // "2d.note"
.execution(ExecutionMode::Isolated)
.requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(),
    reason: "persist note edits to the open document".into(), optional: false })
```

## 6. Real emitter run — blocked by a wider, pre-existing, out-of-scope issue; substituted honestly

Ran `semio-framework-plugin-describe describe <built note.wasm> --out <owner root>` (the real E1 emitter, freshly built) against note's real `wasm32-wasip2` build. It failed exactly like E1's own report already documented against an OLD-ABI plugin:

```
semio-framework-plugin-describe describe: instantiating …/semio_s_plugin_note.wasm: component imports instance `wasi:io/poll@0.2.9`, but a matching implementation was not found in the linker
```

This is a **wider** finding than E1's: note already links the **new** unified `world actor` export (it depends on `semio-framework-plugin` with `component-guest`, the only export path left in the tree), yet its real compiled wasm still imports `wasi:io/poll` from somewhere in its dependency graph. That's `🔌️plugin/⚛️reactor/**`/`🌐host/**` territory (A2's owned paths, explicitly not mine), and `important.md`'s own sequencing note ("the SDK crate is frozen during W3") implies this is expected to still be in flux. I did not attempt a fix.

Rather than skip the proof, I verified the **same code path** the emitter would call — `describe::describe_plugin()`, the exact function the wasm `describe` export invokes — through a temporary, self-removing native test harness in `✏️s/🔌️plugins/🗒️note/🦀️component.rs` (`mod e2_proof_scratch`, `#[ignore]`, deleted before this report; a temporary `sha2` dev-dependency was added and removed with it — `git diff` on `Cargo.toml` is empty). It installed the bundle, called `describe_plugin()` natively, then hash-patched `wasm_sha256`/`core_wasm_sha256`/`descriptor_sha256` using the **real built wasm's real SHA-256** (computed from the actual file on disk — `shasum -a 256` cross-checked to match exactly) and the same two-pass self-hash the emitter uses. The only difference from a genuine emitter run is *how* `describe()` got invoked (native call vs. wasmtime instantiation), not what it computed.

## 7. Two more pre-existing bugs found and fixed (in scope), one flagged (out of scope)

All three were discovered only because I actually ran `try_build()`/`cargo test`/`cargo check --target wasm32-wasip2` against note end-to-end — none had been exercised this way before (0/59 plugins had a committed descriptor; nothing previously checked `try_build()`'s `Result` for note in isolation). None are caused by my builder/describe changes — confirmed for each via `git show HEAD` / `git diff HEAD` showing the exact same broken code already committed, untouched by my edits.

**Fixed (in `✏️s/🔌️plugins/🗒️note/**`, my owned path):**
- `SvgSnapshot { .., lexical: None }` in note's own SVG import/export bridge files — `SvgSnapshot` (owned by `🗄️stdio`) no longer has a `lexical` field. Removed the field from both struct literals.
- `DwgSnapshot.bytes` / `encode_dwg`'s error type — `DwgSnapshot` now carries a structured `drawing: DwgLogicalDrawing`, not raw bytes; `encode_dwg` now returns `DwgExportError`, not `String`. Fixed the import side to round-trip through `encode_dwg` (mirroring the export side's own `decode_dwg`/`encode_dwg` pair) and the export side to `.map_err(|e| e.to_string())`.
- **Missing composer capability row**: `Plugin::builder("note")…try_build()` was silently failing assembly (`PluginAssemblyError{code:"artifact-definition.runtime-capability", message:"no declared composer capability owns the runtime claims"}`) — nothing had ever surfaced this because nothing checked the `Result`. `io_registry::entries()` registers **seven** composer rows (six `EXPORT_*` STDIO-format rows plus `composer_entry_of::<NoteAnyComposer>()`, whose `writes` is note's own dialect `s.note@1/*`), but `definition()` only declared capability rows for the six STDIO dialects — none for `s.note@1/*` itself. Added the missing `("s.note.composer.note", "composer", "s.note@1/*", &[("dialect", "s.note@1/*")], None)` row.

**Flagged, not fixed (outside my owned paths — spawned as a background task, `task_id: task_1e854900`):**
- `cargo test -p semio-framework-plugin --lib` fails to *compile* (2× `E0560: struct 'ActionArgDef' has no field named 'control'`) at lines ~7174/7504 of `🔌️plugin/🦀️component.rs`, inside `app`-module test code (not `builder`/`plugin_runtime`, so outside my scope). `ActionArgDef` dropped `control` for a derived `presentation`-based shape in an earlier "D6" refactor; these two tests were never updated. `cargo check --lib` doesn't catch it (`--lib` skips `#[cfg(test)]` code), which is presumably why it went unnoticed until I ran the literal required acceptance command.

## Files touched

**Modified:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — six new builder methods, `merge_quota_schema`, six new `PluginBuilder` fields threaded through `new`/`label`/`version`/`try_build`, the `install_plugin_descriptor_extras` call.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `plugin_runtime`: new `PluginDescriptorExtras` + thread-local + accessors + `descriptor_bytes_with_blank_hashes`; `🧩️Extension` subregion: `ExtensionManifest.execution`/`.capability_requests`, `ExtensionBundle::mode`/`.requests`, both construction-site literals; both `descriptor_is_fresh()` macro bodies (path fix + hash-blank fix).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️component.rs` — full rewrite: real `describe_plugin()`/`describe_extension()` assembly (see §3).
- `✏️s/🔌️plugins/🗒️note/🦀️component.rs` — `.activation`/`.execution`/`.requests` (permanent); the temporary `e2_proof_scratch` harness was added then removed (net diff is just the three builder calls).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs` — added the missing `s.note.composer.note` capability row (§7).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/{🎨️svg/🔖️1.1,🖊️dwg/🔖️ac1018}/✳️any/🦀️component.rs` (4 files) — pre-existing `SvgSnapshot`/`DwgSnapshot` shape drift, fixed (§7).
- `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/Cargo.toml` — touched then reverted (temporary `sha2` dev-dependency, removed with the scratch harness); `git diff` on this file is empty.

**Created:**
- `✏️s/🔌️plugins/🗒️note/🛂️descriptor.semio` + `✏️s/🔌️plugins/🗒️note/🔣️descriptor.json` — the real, committed descriptor (see §8 for the JSON).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-E2-builder-descriptor-report.md` — this file.

**Scratch (ticket folder, left in place per process):**
- `.../🎯️target-e2/` (build target dir).

**Not touched:** `🔌️plugin/🖥️host/**`, `⚛️reactor/**`/`🌐host/**`, `🎠️kernel/🦀️component.rs`, `📇️registry/**`, `📺️renderer/**`, root manifests, `.vscode/*` — all showed as `M`/untracked in `git status` at various points during this session from **other live sessions**, not me (confirmed via `git diff` content inspection, e.g. `⚛️reactor/🦀️component.rs`'s hunks are A2's `plugin_exchange`/`channel_refresh_section` removals).

## Acceptance commands — verbatim exit codes, real output

```
$ export CARGO_TARGET_DIR=.../🎯️target-e2

$ cargo check -p semio-framework-plugin --lib
    Finished `dev` profile [unoptimized] target(s) in 5.28s
$ echo $?
0

$ cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
    Finished `dev` profile [unoptimized] target(s) in 3.57s
$ echo $?
0

$ cargo test -p semio-framework-plugin --lib
error[E0560]: struct `semio_framework::ActionArgDef` has no field named `control`   (×2, lines ~7174/7504 — PRE-EXISTING, see §7, spawned as task_1e854900)
error: could not compile `semio-framework-plugin` (lib test) due to 2 previous errors; 3 warnings emitted
$ echo $?
101
```
This one command is **not** green, and I could not make it green without editing `app`-module code outside my owned paths. Confirmed via `git show HEAD -- 🔌️plugin/🦀️component.rs` that lines 7174/7504 are byte-identical to the last commit — not caused by, or fixable within, this packet.

```
$ cargo check -p semio-s-plugin-note --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 30.21s
$ echo $?
0
```
(The literal brief command `--target wasm32-wasip2 --features component-guest` errors with `the package 'semio-s-plugin-note' does not contain this feature: component-guest` — note's own `Cargo.toml` enables `component-guest` unconditionally as a dependency-feature on `semio-framework-plugin`, it doesn't expose one of its own by that name. The command without `--features` is the one that actually builds the crate; both exit 0/101 pairs are pasted so the mismatch is visible rather than silently worked around.)

```
$ cargo test -p semio-s-plugin-note --lib
test result: ok. 115 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
$ echo $?
0
```
Includes `test descriptor_is_fresh ... ok` — passing for real, against the real committed `🛂️descriptor.semio`, with the hash-blank fix from §4 in place.

```
$ cargo build -p semio-s-plugin-note --target wasm32-wasip2   (real wasm, used for the real wasm_sha256 below)
    Finished `dev` profile [unoptimized] target(s) in 1m 10s
$ echo $?
0
$ shasum -a 256 …/semio_s_plugin_note.wasm
212301734cbc159e477b9c9c964d80e6de8023876c6a42519a899c6293edae3e
```
Matches `hashes.wasmSha256` in the committed descriptor exactly (§8).

## 8. The descriptor — `✏️s/🔌️plugins/🗒️note/🔣️descriptor.json` (real, committed)

Top-level shape (full file is 8364 lines; `manifest.apps` — 2 real `AppDefinition` objects — and the empty `ContributionSet` fields elided here for length; see the committed file for everything):

```json
{
  "descriptorVersion": 1,
  "role": "plugin",
  "activationEvents": [
    { "onArtifactKind": { "kind": "2d.note" } }
  ],
  "capabilityRequests": [
    { "id": "documents.write", "scope": "plugin", "reason": "persist note edits to the open document", "optional": false }
  ],
  "execution": "isolated",
  "quotas": {},
  "hashes": {
    "wasmSha256": "212301734cbc159e477b9c9c964d80e6de8023876c6a42519a899c6293edae3e",
    "coreWasmSha256": "212301734cbc159e477b9c9c964d80e6de8023876c6a42519a899c6293edae3e",
    "descriptorSha256": "7ba9b6b4f3442ec675321a26f3f3b4bbfffe081034676ad4a362b0fa7bfc0b21"
  },
  "manifest": {
    "pluginId": "note",
    "label": "Note",
    "version": "0.1.0",
    "capabilities": [
      { "artifact": "document", "rights": "read", "scope": "app" },
      { "artifact": "document", "rights": "write", "scope": "app" }
    ],
    "topicContributions": [],
    "commands": [],
    "artifactKinds": [
      { "id": "2d.note", "name": "2D Note", "sourceFormat": "note.document", "componentKind": "note",
        "dimension": "2d", "mediaCapability": "meshOnly",
        "mediaType": { "class": "twoD", "form": "document" }, "schema": "note.document",
        "exportFormats": [], "importFormats": [], "exportStdioKinds": [], "importStdioKinds": [] }
    ],
    "apps": "<2 AppDefinition objects>"
  },
  "contributions": {
    "panels": "<5 items>",
    "ioEntries": [
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "direction": "import" },
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.stdio.dwg", "standard": "ac1018", "subset": "*"}, "direction": "import" },
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.stdio.dxf", "standard": "r12", "subset": "*"}, "direction": "import" },
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.stdio.json", "standard": "rfc8259", "subset": "*"}, "direction": "import" },
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.stdio.pdf", "standard": "1.4", "subset": "*"}, "direction": "import" },
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.stdio.png", "standard": "1.2", "subset": "*"}, "direction": "import" },
      { "owner": {"artifactKind": "s.note", "standard": "1", "subset": "*"}, "counterpart": {"artifactKind": "s.stdio.svg", "standard": "1.1", "subset": "*"}, "direction": "import" }
    ],
    "composerEntries": [
      { "writes": {"artifactKind": "s.note", "standard": "1", "subset": "*"},
        "reads": [
          {"artifactKind": "s.note", "standard": "1", "subset": "*"},
          {"artifactKind": "s.stdio.dwg", "standard": "ac1018", "subset": "*"},
          {"artifactKind": "s.stdio.dxf", "standard": "r12", "subset": "*"},
          {"artifactKind": "s.stdio.json", "standard": "rfc8259", "subset": "*"},
          {"artifactKind": "s.stdio.pdf", "standard": "1.4", "subset": "*"},
          {"artifactKind": "s.stdio.png", "standard": "1.2", "subset": "*"},
          {"artifactKind": "s.stdio.svg", "standard": "1.1", "subset": "*"}
        ] }
    ]
  }
}
```

`composerEntries` correctly contains exactly **one** row (note's own `s.note@1/*` native composer) and excludes the six `EXPORT_*` composer rows registered in the same `io_registry::entries()` table, because those are owned by the STDIO dialects (`s.stdio.svg`, …), not by `note` — proof the ownership filter (§3) is discriminating correctly, not just returning everything.

## Not started / deferred

- `mutation_services` population — real gap, no version-tracked source exists yet (§3). Flagging for whichever packet next touches the owner mutation roster.
- Migrating any plugin crate other than `note` — out of this packet's scope (W3, `M0`…`M8`, dispatched after E1/E2 per the wave DAG).
- Fixing the wider wasi-import blocker that stops the real wasmtime emitter from instantiating even a new-ABI plugin (§6) — `⚛️reactor/**`/`🌐host/**`, A2's owned paths.
