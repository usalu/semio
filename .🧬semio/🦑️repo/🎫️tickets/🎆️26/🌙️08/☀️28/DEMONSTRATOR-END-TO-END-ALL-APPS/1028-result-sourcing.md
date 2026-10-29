# 1028 result — sourcing

STATUS: RED (blocked by non-sourcing dependency errors; no error inside any sourcing-owned crate reached yet)

Frontier: `semio-hub-sourcing-component` (hub workspace, wasm32-wasip2), log `C/🗑️generated/1028/sourcing-chk7.txt` (rc=1). The compiler now runs (earlier attempts chk1-chk6 died in admission/lock churn from peer edits; `C/1028-tools/sourcing/chk.sh <name> --manifest-path … -p …` retries those). Extension crates (beams/slabs/windows, `✏️s` workspace) not yet checked: they depend on the same chain.

Files changed: none. SHARED CHANGE: none.

## Remaining errors (all in dependencies, not sourcing-owned)
A peer is mid-migration of `semio_framework_value::native_decoding` into the separate crate `semio-framework-value-native-decoding` (`🧰️framework/🔨️modules/🌱️value/🛬️decode/…`, run via `📜️script.ts` there). Consumers are not yet updated:
- `semio-framework-replication-ids` (10 errors, `📡️replication/🆔️ids/…/🛬️receiving/🦀️.rs:13`), `semio-framework-ui-locale` (5, `🖱️ui/🌐️locale/…/🏷️label/🛬️receiving/🦀️.rs:14`), `semio-framework-os-store-link-schema` (2, `🧬️schema/🗿️artifact-reference/🔗️link/…`): `NativeMappedReceiver<I,..>` — `I: ?Sized` size/trait-bound errors (E0277/E0599 birth/begin).
- `semio-s-artifact-stdio-contract-part21` (25): unresolved import/path `semio_framework_value::native_decoding` (E0432/E0433), missing `Part21Value::Receiver`, `copied_bytes`, `is_part21_binary`, `NativeReceive<dyn NativeValueSource>` unmet (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/…/🫴️receiving/🦀️.rs`).
Diagnosis: consumers need the new receiver API of the native-decoding crate (owner = the value/native-decoding migration peer or stdio fleet). I did not touch them to avoid colliding with the in-flight sweep.

## Next
Re-run after the native-decoding migration lands: `C/1028-tools/sourcing/chk.sh chkN --manifest-path 🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/📦️packages/🦀️rust/Cargo.toml -p semio-hub-sourcing-component`, then the three `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/*/🧩️component/📦️packages/🦀️rust/Cargo.toml`.

## Update (after coordinator reassignment)
FIXED `semio-framework-replication-ids`: `🧰️framework/🔨️modules/📡️replication/🆔️ids/🛬️receiving/🦀️.rs` — `impl<I:NativeValueSource+?Sized> NativeReceive<I>` → `impl<I:NativeValueSource>` (NativeMappedReceiver now requires Sized `I`). Crate no longer errors (chk8).
Current frontier (logs `sourcing-chk8.txt` hub component, `sourcing-extb/exts/extw.txt` extensions):
- hub component: `semio-framework-replication-edit` (derive NativeReceive on `dyn NativeValueSource`, fwwasm) and `semio-s-artifact-stdio-contract-part21` (47 errors incl. `0x10000 + ((previous - 0xD800) << 10) + code` i32+u32 at `part21/🔤️lexer-and-parse/🦀️.rs:196`, playbook).
- 3 extensions: all stop at `semio-framework-replication-mutation-retained` (2x E0308, puzzleplugin).
No sourcing-owned crate has errored. Re-run once those owners land.

## Update 20:20
Frontier moved through (all foreign, all fixed/being fixed by peers): replication-ids/edit -> os-store-one-item -> os-store-replay -> store-durable-group-wire/initialization-runtime -> store-engine-core -> now `semio-framework-os-store-engine-stage` (19 errors: duplicate `prepare_staged_diagnostics`/`retire_binding_member`, `Mutation` bounds; `⚙️engine/🗄️stage/📥️lifted/🗄️durable-group/🦀️.rs`, edited <10 min ago by a peer sweep -> skipped per 10-minute rule). Latest log: `sourcing-g10.txt` (hub component; extensions share the same chain, last full run f11-f13 = same error set as f10).
No file changed by me except replication-ids earlier (since reverted by fwwasm to ?Sized, intentional). No sourcing-owned crate has errored yet. Retry: `C/1028-tools/sourcing/chk.sh <name> --manifest-path <Cargo.toml> -p <crate>` (hub component + beams/slabs/windows `-component` crates).

## Split-audit registration fixes (C/1028-audit-splits.md)
1. Energy workspace root `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml`: 11 missing mutation crates appended to `members` (snapshot-sqlite-admission and native-lifecycle/singletons were already members). Repo root `Cargo.toml`: `plugin/👷️job/🛬️receiving` added to `members` after its parent `👷️job` (it was only in `🧰️framework/Cargo.toml`'s list, which sits inside `exclude` there as for siblings).
2. `📋️build-owners.json`: `repo ./📜️script.ts verify build-owners sync` -> added=22 rows (14 energy + others the tree needed). Re-verify leaves 1 breach: `✏️s/🔌️plugins/📏️layout/.../✏️editor/🧪️tests/🔬️unit` (a peer's new layout crate, not mine).
3. `[package.metadata.semio.rust-source-boundary]` (262144/32/4000, sibling values) added to job-receiving, decl-media, decl-staged-form, instance-initialization, instance-construction.
4. Surface fixtures: no change needed (audit false positive). `surface-fixture.json` mirrors exactly the DIRECT dependency set of `semio-framework-plugin-surface-fixture` (asserted by the unit test); the lock covers only surface-fixture and composition-publication, neither of which depends on decl-media/staged-form. No generator run.
Verification: gate census (`gatecensus/census.ts roots.txt`) 0 violators over 1628 closure crates; `repo cargo -- metadata --no-deps` loads for repo root and the energy workspace.

## Update 21:15
Latest checks `sourcing-i10..i13.txt`: frontier = `semio-framework-os-store-engine-command` (`store/🕰️history/🎮️command`), being actively reworked by a peer (file edited seconds before each look; errors dropped 62 -> 4 `ColdMutation` not in scope between runs). Skipped per 10-minute rule. Still no sourcing-owned error. Re-run: `C/1028-tools/sourcing/chk.sh chkN --manifest-path <Cargo.toml> -p <crate>` (hub component; beams/slabs/windows `-component`).

## CHECKPOINT 23:10 (stopped per brief)
STATUS: RED. Frontier moved to the plugin layer, all peer-owned and under active edit: `semio-framework-plugin-instance-jobs` (`🧵️instance/👷️jobs/📥️staging/{⏯️tool-run,🔒️close-ladder,🚪️ingress}` missing imports: `ToolCancellationLease`, `SpaceMember`, `RetainedCloneProgress`, `plugin_grant_funds`, `semio_framework_plugin_reactor_tasks`), `semio-framework-plugin-reactor-wit-bridge` / `reactor-context` (`poll_kernel` Vec<Event> vs &mut Option<Event>, missing `OriginalActorFixedCall/OriginalOwnedJobCall/OriginalActorEntryFailure`, `semio_framework_plugin_abi::wasip2::ActorAdmissionCall` etc.). No sourcing-owned crate has errored. Store layer and replication-ids are past.
Files changed by me: `Cargo.toml` (repo root, +job-receiving member), energy model `Cargo.toml` (+11 members), `📋️build-owners.json` (+22 rows via sync), 5 plugin Cargo.toml rust-source-boundary blocks (see above). No source edits.
Killed my own queued check wrappers (pids 26338-26341 + children). No checks running.
Next: `cd /Users/ueli/Documents/semio && C/1028-tools/sourcing/chk.sh chkN --manifest-path 🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/📦️packages/🦀️rust/Cargo.toml -p semio-hub-sourcing-component` (run detached, wait for `.rc`), then the same for the three `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}/🧩️component/📦️packages/🦀️rust/Cargo.toml` (`-p semio-s-plugin-sourcing-{beams,slabs,windows}-component`) once the plugin layer is quiet.
