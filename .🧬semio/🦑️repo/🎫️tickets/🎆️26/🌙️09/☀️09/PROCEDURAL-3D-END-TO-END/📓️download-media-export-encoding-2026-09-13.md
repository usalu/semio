# ⬇️ `DownloadMediaExport.encoding`, carried end to end (2026-09-13)

Lane `download-media-export-encoding` (Opus). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
Repo/semio MCP both failed to connect all session (`repo`: -32602 invalid initialize params;
`semio`: CONNECTION_CLOSED) — no ticket opened/closed/reopened by this lane, no `📓️status.md` or
`🎫️ticket.json` edit, no modifying git command, no dev server started or stopped.

## TL;DR

`📓️io-surface-2026-09-13.md` §7.3 reported that `Effect::DownloadMediaExport.encoding` was dropped
between the guest and the shell. It was — and it was worse than reported: **three** layers were
broken, on **both** renderers, and the defect also hit the artifact's DEFAULT export.

- **Measured root cause** (temporary `[DEBUG]`, live on 6018, since removed):
  `encoding: {tag:"some", val:"base64"}`. A WIT `option<string>` reaches the React door as the
  actor boundary's **tagged variant record**, and
  `🔌️PluginRuntime/🟦️.tsx:987` read it as `typeof val.encoding === "string"` — never true. The
  helper that unwraps exactly this shape (`optionValue`) sat eleven lines above, unused for this field.
- **Second defect, not in the report**: the SHARED decoder `wireEffectToFriendly`
  (`🎭️actor/🖼️wire-turn/🟦️.ts`), which the wgpu `plugin-bridge` uses, had **no `download-media-export`
  case at all** — every plugin export on the wgpu target was a `unmapped effect … dropped` console line.
- **Third defect**: the wgpu Rust shell dropped `Effect::DownloadMediaExport` in `queue_host_effects`'
  fallback arm, and its browser half took `_encoding` and blobbed `data` verbatim.
- **Fix at the owning layer**: the effect contract now OWNS the `(data, encoding) → bytes` rule
  (`kernel::media_export_bytes` + the TS twin `mediaExportBytes`), one shared wire decoder
  (`wireDownloadMediaExport` / `wireOptionValue`) serves both renderer doors, and every download
  half on both renderers answers through the contract. No `atob` branch, no `base64` crate, no
  "anything that is not base64 is text" fallthrough.
- **Laws**: 6 (kernel, Rust) + 4 (io-base64, Rust) + 4 (wgpu shell, Rust) + 1 TS twin driving 8
  fixture rows / 5 refusals / 2 wire-option shapes + 3 new `effect-wire-routes` fixture rows +
  2 pre-existing renderer download laws still green. All foreground.
- **Browser proof on 6018**: `generation3d.dwg`, **1 096 B**, magic `41 43 31 30 31 35` = `AC1015` —
  a real DWG header. The pre-fix `generation3d.stl` is **byte-for-byte `base64(` the post-fix file `)`**.

---

## 1. Root cause, measured

A temporary `[DEBUG]` line in the React door's `download-media-export` case, driven by a real
`Export Document…` on 6018:

```
[DEBUG] wire download-media-export {"keys":["filename","mimeType","data","encoding"],
  "encoding":{"tag":"some","val":"base64"},"encodingType":"object",
  "mime":"model/stl","filename":"generation3d.stl","dataLen":5080}
```

`filename`, `mimeType` and `data` are flat strings; `encoding` is the tagged option record. The door
read it flat:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:987`
(before this lane, blame `f39d4b0db3b`, 2026-09-10):

```ts
return { downloadMediaExport: { …, encoding: typeof val.encoding === "string" ? val.encoding : undefined } };
```

Three corrections to §7.3's account:

1. The WIT contract and the Rust lowering/raising (`⚛️reactor/🦀️.rs:1651`, `🖥️host/🦀️.rs:2501`,
   `🌐host/🦀️.rs:1013`) were always correct, and so was jco's `.d.ts` (`encoding?: string`). The loss
   is **one line of the renderer's own decoder**, not the effect hop.
2. §7.3 said "`txt` is byte-correct — so only the `Option` is lost". True but incomplete: the
   artifact's **default** format, `stl`, also declares `is_binary: true`
   (`s.stdio.stl.standard.ascii`, §9 follow-up 3 of the io-surface report), so the default export was
   corrupt too. The pre-fix probe saved `generation3d.stl` = **5 080 B beginning `c29saWQgZ2VuZXJh`**
   (= `solid genera`). 5 080 / 3 810 = 4/3, exactly base64 expansion.
3. The wgpu target was not merely "also affected" — it could not export at all (§2.2).

## 2. What was broken, by layer

### 2.1 React door — the option shape
As above. One field, one decoder, every binary export in the repo.

### 2.2 The SHARED decoder — no case at all
`🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts:485` `wireEffectToFriendly` is the decoder the wgpu
`🐚️plugin-bridge/🟦️.ts` calls at five sites (`:655`, `:827`, `:928`, `:1027`, `:1156`). It had cases
for `notify`, `navigate`, `open-external-url`, `set-panel`, … and **none** for
`download-media-export`, so the effect hit the `default` arm: a console warning and `null`. The
module's own header calls out the "third divergent copy" hazard between this file and
`🔌️PluginRuntime`; this is what that hazard cost.

### 2.3 The wgpu Rust shell — dropped, then encoding-blind
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

- `queue_host_effects:4262` — the ONE host-effect funnel both wgpu dispatch paths share — had no
  `DownloadMediaExport` arm, so it fell into `other => "[DEBUG] wgpu-shell effect dropped"`.
- `download_media_export` (wasm32, `:14769` before this lane) took `_encoding: Option<&str>` and did
  `Blob::new_with_str_sequence([data])` — the base64 text, verbatim.
- the native worker (`:14802`) decoded correctly but with the third-party `base64` crate and an
  `unwrap_or_else(|_| data.as_bytes().to_vec())` **silent text fallback** on malformed input.

## 3. The fix, at the owning layer

### 3.1 The effect contract owns the bytes

`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` (region `⬇️MediaExportEncoding`, immediately beside
`Effect::DownloadMediaExport`):

```rust
pub const MEDIA_EXPORT_BASE64_ENCODING: &str = "base64";
pub const MEDIA_EXPORT_UTF8_ENCODING: &str = "utf-8";
pub enum MediaExportEncodingError { Unsupported { encoding: String }, Malformed { detail: String } }
pub fn media_export_bytes(data: &str, encoding: Option<&str>) -> Result<Vec<u8>, MediaExportEncodingError>
```

with the TS twin `mediaExportBytes` / `MediaExportEncodingError` / the two constants in
`🎠️kernel/🟦️.ts`, beside the `Effect` type both renderers already import.

Three decisions worth stating:

- **The vocabulary is closed and named.** A survey of every live producer
  (`grep DownloadMediaExport … | encoding:`) found exactly three spellings: absent (11 sites),
  `Some("base64")`, and `Some("utf-8")` — the last one real, from `puzzle3d`'s `exportFixture`
  (`✏️s/🔌️plugins/🧩️puzzle/…/📤️export-fixture/🦀️.rs:43`). Both textual spellings are named and mean the
  same thing; anything else is `Unsupported`, loudly. The old shells' "not base64 ⇒ text" branch would
  have quietly saved a future `Some("gzip")` as text.
- **No runtime external dependency.** The base64 half delegates to the repo's own
  `semio-framework-io-base64` (already a dep of seven crates), added to `semio-framework`. The
  third-party `base64` crate is a **dev-dependency only**, used as the oracle. The wgpu native worker
  stopped calling the `base64` crate for this path.
- **Stricter than the platform, on purpose.** `atob` accepts `QQ` (unpadded), `Zh==` and `Zm=v`
  (non-canonical / misplaced padding); RFC 4648 §4 does not, and neither half of this contract does.
  So the new TS twin of the codec (`🚪️io/🔤️base64/🟦️.ts`) exists rather than a call into `atob` — a
  boundary whose two implementations disagree on what a valid export is has no law at all.

### 3.2 One wire decoder for both renderer doors

`🎭️actor/🖼️wire-turn/🟦️.ts` now exports `wireOptionValue` / `wireOptionText` /
`wireMediaExportEncoding` and `wireDownloadMediaExport`, and `wireEffectToFriendly` has the missing
case. `🔌️PluginRuntime/🟦️.tsx` imports both and deleted its private copies — the same consolidation
`wireExtensionInvocation`/`wireRespondAnswer` already have, and for the same reason. `🛠️ShellHelpers`'
`mediaExportEncodingText` IS `wireMediaExportEncoding` now, and `downloadMediaExport` delegates to the
kernel contract instead of its own `atob` branch.

The segmented-download lane is untouched: a `semio-segmented-handle-v1:` value in `encoding` is a
handle, not an encoding, and the shell consumes it before either reaches the bytes.

### 3.3 The wgpu shell

A real `DownloadMediaExport` arm in `queue_host_effects`; both download halves answer through
`semio_framework::kernel::media_export_bytes`; a refusal logs and writes nothing, instead of writing
the envelope's text to disk.

## 4. Files

| File | What |
|---|---|
| `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` | **new** `⬇️MediaExportEncoding` region: the two encoding constants, `MediaExportEncodingError`, `media_export_bytes`, test mount |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | TS twin: `MEDIA_EXPORT_BASE64_ENCODING`, `MEDIA_EXPORT_UTF8_ENCODING`, `MediaExportEncodingError`, `mediaExportBytes` |
| `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/⬇️media-export-encoding/🔣️.json` | **the language-agnostic law** — 8 cases, 5 refusals, the two wire-option shapes |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/⬇️media-export-encoding/🦀️.rs` | Rust laws (6) |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/⬇️media-export-encoding/🟦️.ts` | TypeScript twin (8 sections), with `Buffer` + `atob` as independent oracles |
| `🧰️framework/🔨️modules/🚪️io/🔤️base64/🟦️.ts` | **new** strict RFC 4648 TS twin of the module's Rust codec |
| `🧰️framework/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-io-base64` runtime dep; `base64` **dev**-dep (oracle) |
| `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | `wireOptionValue`/`wireOptionText`/`wireMediaExportEncoding`/`wireDownloadMediaExport`; the missing `download-media-export` case; local `some` folded into the shared reader |
| `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📨️effect-wire-routes/🔣️.json` | 3 new rows: tagged `some`, bare, and `none` |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | the fixed door: shared decoder + shared option reader, private copies deleted |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `downloadMediaExport` goes through `mediaExportBytes`; `mediaExportEncodingText` = the shared reader |
| `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `queue_host_effects` `DownloadMediaExport` arm `:4336`; wasm32 half now reads `encoding` and blobs a `Uint8Array`; native worker uses the kernel contract; test mount |
| `…/🧱️elements/🐚️Shell/🧪️tests/⬇️wgpu-media-export-encoding/🦀️.rs` | **new** wgpu laws (4) |
| `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` | `media-export-encoding` + `-native` lane |
| `T/🐍️export-encoding-probe.mjs` | the browser proof |

## 5. Laws (all run in the foreground)

| Lane | Result |
|---|---|
| `bun nx run workspace:media-export-encoding` (TS twin + `tsc --strict` on the codec twin) | green — `cases=8 refusals=5 binary=4 textual=3 oracles=Buffer,atob` |
| `cargo test -p semio-framework --lib media_export_encoding` | **6 passed** |
| `cargo test -p semio-framework-io-base64` | **4 passed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib media_export_encoding` | **4 passed** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | green in 2m20s, warnings emitted (proof it really type-checked); the only new-looking warning, `unused import: js_sys`, is in `⚙️EngineCanvas`, not this lane's file |
| actor vitest, `🖼️wire-turn` suite | 3 new `download-media-export-*` rows green, whole wire-turn suite green |
| react vitest, `🔬️engine-contract` + `🛠️ShellHelpers/🧩️component` + `🔌️PluginRuntime` | **12 passed** — incl. the two pre-existing download laws and "preserves document and download effects" |

Raw output: `T/🗑️generated/export-encoding/{rust-laws,wgpu-laws,wgpu-wasm-check,engine-contract,segmented-and-runtime}.txt`.

**Third-party oracles.** Rust: the `base64` crate, dev-only, asserted to agree with
`media_export_bytes` in BOTH directions on every base64 fixture row. TypeScript: Node's `Buffer`
base64 decoder AND the platform `atob`, neither of which shares a line with our codec — plus three
rows that pin exactly where our codec is deliberately **stricter** than `atob`, which is the reason
the repo owns one.

**The wire-shape law is the one that would have caught this.** `🔣️.json`'s `wireOptionShapes`
declares both real shapes of one `option<string>`, and §7 of the TS twin drives every fixture case
through `wireEffectToFriendly` in BOTH shapes. The old React door fails that law on row one.

## 6. Runtime proof (6018, React)

`T/🐍️export-encoding-probe.mjs` — boots the generation3d editor, focuses the flow window, opens the
Actions pane, presses `Export Document…`, picks a format in the REAL `role=combobox` listbox (the
seven rows the artifact publishes), presses Execute, and reads the shell's download off disk.
Evidence in `T/🗑️generated/export-encoding/` (`before-fix/`, `after-fix/`, screenshots, `console.txt`,
`results.json`, the downloaded files).

| Format | Picked | File | Bytes | First 8 bytes (hex) | Reads as |
|---|---|---|---|---|---|
| **before the fix**, default | `STL Mesh` | `generation3d.stl` | 5 080 | `63 32 39 73 61 57 51 67` | `c29saWQgZ2VuZXJh` — **base64 TEXT** |
| `dwg` | `DWG Drawing` | `generation3d.dwg` | **1 096** | `41 43 31 30 31 35 03 00` | **`AC1015`** — a real DWG version header |
| `stl` | `STL Mesh` | `generation3d.stl` | **3 810** | `73 6f 6c 69 64 20 67 65` | `solid generation3d-preview / facet normal 0 0 -1 / vertex 0.5 0 0 …` |
| `txt` | `Semio Text (whole document)` | `generation3d.txt` | **1 572** | `73 65 6d 69 6f 20 70 72` | `semio procedural.generation3d.dsl v1 …` — the non-encoded lane, unchanged |

The sharpest single fact: `base64(after-fix/generation3d.stl) == before-fix/generation3d.stl`,
byte for byte. The user was being handed the base64 of their file, under their file's name.

No `MediaExportEncodingError`, no `unmapped effect "download-media-export"` in the run's console; the
six `error`-level console lines are pre-existing `[DEBUG]` noise from the contributions/boot chain.

## 7. What is NOT claimed

- **The wgpu page was not driven to a download in the browser.** A wgpu export ends in a
  `<a download>` click from wasm (browser) or a native save dialog — neither is presentable in
  headless Chromium, and this lane does not start or stop dev servers. The wgpu half is proven by
  (a) the 4 native Rust laws, which drive the same fixture through the same contract both halves now
  call and read the two call sites out of the shell's own source, (b) a clean
  `--target wasm32-unknown-unknown` check of the crate, which is the only way the browser-gated arm
  compiles at all, and (c) the 3 shared-decoder fixture rows, since the wgpu bridge's entire
  wire→friendly step is `wireEffectToFriendly`. The end-to-end *click* on 6118 is unproven.
- **`process3d`'s `glb` was not exported in a browser.** It is covered generically (it emits
  `Some("base64")` through the same effect and the same two doors) and by the `binary-glb-base64`
  fixture row, but no live `process3d` export was driven.
- **`las` was not exported.** `generation3d`'s roster offers it; the probe drove `dwg`, `stl`, `txt`.
- **`s.stdio.stl@ascii` still declares `is_binary: true` for a text-only encoder.** That is the
  io-surface report's §9 follow-up 3 and stays open: with this fix the value is merely wasteful
  (base64 round-trip) rather than corrupting, and `is_binary` has other consumers.
- **`📓️io-surface-2026-09-13.md` §7.4** (the 32 KiB host slice vs the ~4 KiB guest envelope) is
  untouched — a different lane.
- Four pre-existing FAILs in the actor suite (`📤️return/🟦️.ts`, `📤️return/📨️response/🟦️.ts`) are an
  ajv `$ref` resolution error (`can't resolve reference …/value/schema.json#/$defs/NonZeroU64`),
  unrelated to this lane and not introduced by it.

## 8. Fix-forward on peers' work

None needed — nothing blocked this lane.

## 9. Follow-ups

1. `🔌️PluginRuntime/🟦️.tsx`'s `wireEffectToFriendly` is STILL a second copy of
   `🖼️wire-turn/🟦️.ts`'s, now sharing three helpers but not the switch. Every effect kind in it is
   one forgotten `option` away from this same defect (`request-file-open`'s `read-as` already reads
   `params["read-as"] ?? params.readAs`, which is the same smell). Fold the two switches into one.
2. `s.stdio.stl@ascii`'s `is_binary` (see §7) — now safe to correct, since the encoding lane is honest.
3. `semio-framework-os-renderer-wgpu` still has `base64 = "0.22.1"` as a **runtime** dependency for
   `decode_data_url` and `request_file_open`; `semio-framework-io-base64` replaces it.
4. The `encoding` vocabulary is now three named values in the kernel, but the WIT still types it
   `option<string>`. A WIT `enum` would make an unknown encoding unrepresentable rather than merely
   refused.
