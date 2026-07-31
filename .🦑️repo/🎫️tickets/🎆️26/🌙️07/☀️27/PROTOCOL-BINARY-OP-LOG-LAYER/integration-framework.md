# Wave 2 — Framework / WIT sweep

Single agent, critical section (`framework/wit/world.wit`, root `script.ts`, `.vscode/launch.json`
are shared hot files — re-read each fully before editing). Prerequisite: Wave 1 (vcs integration)
is merged; `vcs` now exposes `DocumentBinaryFiles`/`print_document_binary`/`parse_document_binary`/
`set_state_with_projection`/`from_binary`, and `DocumentPackFiles` no longer exists in `vcs`.

Read first: the Wave 1 agent's final report (ask via the ticket folder — check for a
`wave1-report.txt` left in `.repo/🎫️/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/`) for the exact new
vcs API surface. Then `/Users/ueli/Documents/semio/framework/wit/world.wit`,
`/Users/ueli/Documents/semio/framework/plugin/rs/lib.rs` (regions around `DocumentContract` trait
~3344, blanket impls ~3871, `plugin_document_pack`/`plugin_load_document_pack` ~4362, WIT glue
~125), `/Users/ueli/Documents/semio/framework/plugin/host/rs/lib.rs` (`read_app_document_pack`/
`load_app_document_pack` ~341), `/Users/ueli/Documents/semio/framework/product/os/core/rs/lib.rs`
(`export_os_studio_pack` ~1319 + callers).

## What to build

1. **WIT** (`framework/wit/world.wit`): replace `document-pack-files` record (currently `{ pack:
   list<u8>, ops: string }`) with:
   ```wit
   /// Binary counterpart of document-text-files: pack = encoded initial projection, protocol =
   /// the binary op-log container (mirrors vcs::DocumentBinaryFiles).
   record document-binary-files { pack: list<u8>, protocol: list<u8>, }
   ```
   Replace the two `-pack` funcs with `read-app-document-binary` / `load-app-document-binary`
   (same signatures, new names, new record type). No dual surface — greenfield, delete the old
   ones outright, don't keep both.

2. **`framework/plugin/rs/lib.rs`**: update the WIT glue to the new record/func names;
   `DocumentContract` trait fns `document_pack`/`load_document_pack` → `document_binary`/
   `load_document_binary`; the blanket impls (~3871) now call `print_document_binary`/
   `parse_document_binary` and, on load, `set_state_with_projection` (via `from_binary`) instead
   of the old full-replay `set_state` path; rename `plugin_document_pack`/
   `plugin_load_document_pack` → `plugin_document_binary`/`plugin_load_document_binary`.
   `register_document_codec_for_app` — verify it still compiles against the new `DocumentCodec`
   shape from Wave 1; fix call sites if the field names changed.

3. **`framework/plugin/host/rs/lib.rs`**: rename `read_app_document_pack`/`load_app_document_pack`
   → `read_app_document_binary`/`load_app_document_binary`, matching the new WIT func names.

4. **`framework/product/os/core/rs/lib.rs`**: `export_os_studio_pack` → `export_os_studio_binary`
   + update its callers (grep for the old name across `framework/product/os/`).

5. **`framework/sync` `FolderEndpoint`**: the `Pack` variant becomes `Binary { storage,
   document_id, extension, schema }`; `read` = `storage.read_binary(...)` → `(codec.parse)(pack,
   protocol)`, with fallback to text read → `codec.parse_dsl` when no binary files exist yet;
   `write` = `(codec.print)(envelope_json)` → `storage.write_binary(...)`. The `Sqlite` variant
   goes through the same codec-mediated path. `DocumentActor`'s external shape (message types) is
   unchanged — only its internal persistence calls move.

6. **Regenerate plugin-module build artifacts** that embed the WIT world (run whatever build step
   produces `framework/product/os/dev/plugin-modules/*` and the wgpu renderer-modules — check
   `framework/plugin/registry/script.ts` / the relevant `script.ts` build target rather than
   hand-editing generated output).

7. **Policy lints** in root `script.ts`:
   - `//#region 🔧️PolicyRuleProtocolCompleteness` (place near the existing
     `//#region 🔧️PolicyRulePackCompleteness`): clone `policyPackCompletenessBreaches`'s shape —
     any `*.rs` file calling `assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(`
     must also call `assert_document_protocol_round_trip(`. Seed
     `POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST` (next to `POLICY_PACK_COMPLETENESS_ALLOWLIST`) with
     every file that currently fails this check (computed via the same `grep -rl` approach the
     pack rollout used) — this allowlist is what Wave 3 shrinks to empty. `vcs/rs/lib.rs` itself
     should NOT be in the allowlist (Wave 1 already proved the mechanism there).
   - Extend `policyNoPackFilesBreaches`/`policyDiscoverPackFiles` to also discover and ban
     committed `.spr` files, same rationale as `.pack` (authoritative-but-regeneratable, human-
     unreadable, must never be committed).
   - Register both in the `policy` export next to the existing pack policy calls.

8. **`.vscode/launch.json`**: add the eight `🧪️test📼️protocol-*` entries at float orders
   391.91–391.98 (right after the pack crates' test entries), per Part D of the plan — these
   should already exist if Wave 0's closing agent added them; if not, add them here. `.claude/
   launch.json`: no entry needed (no server).

## What NOT to touch this wave

`framework/product/os/hub` and its storage backends, `vcs::BackboneMessage`, WIT
`backbone-send`/`backbone-poll` — all Wave 2b. Any app crate beyond what's needed to keep the
workspace compiling (a WIT/trait rename ripples into every `DocumentApp` impl only through the
trait signature, which recompiles transparently — you should NOT need to touch app source; if you
find yourself editing an app crate's business logic, stop, that's out of your lane).

## Verification

`cargo build --workspace` succeeds (this is the first wave where the full workspace should build
green again after Wave 1's expected `vcs`-consumer breakage). `bun ./📜️script.ts lint` shows the new
protocol-completeness and no-`.spr` policies registered and firing only against the seeded
allowlist (not against files outside it). `cargo test -p semio-framework-sync` passes with the
FolderEndpoint changes.

## Report back

Files touched, the final WIT diff, the exact `POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST` contents
(Wave 3's task list), and confirmation the full workspace builds.
