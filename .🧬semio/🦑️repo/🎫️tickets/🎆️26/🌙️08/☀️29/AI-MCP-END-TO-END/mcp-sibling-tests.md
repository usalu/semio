# MCP Sibling Crate Tests

Command: `SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test -p <crate> --lib --no-fail-fast`, one crate at a time. Re-run logs: `🗑️generated/r-<crate>.log` (first-run logs: `t-semio-os-mcp-<crate>.log`).

## Result table

| Crate | Before | After | State |
| --- | --- | --- | --- |
| capabilities | 41/43 | 41/43 (2 fail; 39/43 with a catalog supplied, see below) | remaining, pre-existing |
| consent | no compile | 30/30 | fixed |
| dispatch | no compile | 40/40 | fixed |
| http-transport | no compile | 59/59 | fixed |
| hub-pair | exit 1 | 0 tests, exit 0 | transient, no defect |
| inference-hub | 6/9 | 8/9 (1 fail) | 2 fixed, 1 remaining |
| plugin-runtime | no compile (lib test) | no compile (lib test) | remaining, pre-existing |
| probe-document | 0/1 | 0/1 | remaining, not diagnosed to root |
| routing | blocked | 0 tests, exit 0 | unblocked, no defect |

## Fixed

- consent: `try_recv` on `BridgeOutboxReceiver` is `cfg(any(test, feature = "testkit"))`. Added the bridge `testkit` feature as a dev-dependency in `🙋️consent/📦️packages/🦀️rust/Cargo.toml`.
- dispatch: `semio_os_mcp_capabilities::source_builders` is `testkit`-gated. Enabled `features = ["testkit"]` on the capabilities dev-dependency in `🪝️action-dispatch/📦️packages/🦀️rust/Cargo.toml`.
- http-transport (five defects, all hidden behind the first resolution error):
  - `META_PROTOCOL_VERSION_KEY` import added to `🚚️transport/🧪️tests/🔬️long/🦀️.rs`.
  - `IntoResponse` import added to the same test file and to `🚚️transport/🛡️origin-and-handlers/🦀️.rs` (cfg(test) oracle handlers).
  - `std::io::Read` import added to `🚚️transport/🧪️tests/🔬️quick/🦀️.rs`.
  - `BridgeOutboxReceiver::recv` is testkit-gated: bridge `testkit` dev-dependency added in `📮️http-transport/📦️packages/🦀️rust/Cargo.toml`.
  - Private-type access from the sibling test modules: `ClientWebSocketFrame` (`🔌️websocket-codec`), `BridgeInboundCursor`, `BridgeInboundPhase`, `BridgeWriteCursor` (`🔁️http-transport-state`) widened to `pub(crate)`.
- inference-hub, `every_schema_compiles_and_validates_its_own_example`: the `InferenceSubmitInput` and `InferenceRunInput` catalog shapes referenced `#/$defs/InferenceRetainedPolicy` without carrying it. The shapes now carry it in a `definitions` bucket (the mirror hoists and repoints it, so the committed mirror is unchanged by this); the wire schemas inline the policy and drop the bucket (`💡️inference-tool-schemas/🦀️.rs`).
- inference-hub, `inference_approval_encoding_consumes_the_framework_owned_contract`: the closed contract requires `schema` and `version`, which the owner binds; the neutral `HubInferenceApprovalRequestV1` carries only `jobId` and `proposalHash` (the GIS owner struct adds the envelope). The test now asserts the neutral arguments alone are refused, binds the envelope, then validates and rejects the hostile extra property (`🧬️schema/🧪️tests/🔬️quick/🦀️.rs`).

## Remaining

- capabilities (`search_long::the_installed_catalog_answers_three_intent_queries_with_a_clear_winner`, `search_long::every_installed_plugin_gates_destructive_and_user_path_verbs_and_keeps_chrome_out`): pre-existing, environmental. `discover_catalog_source` needs a caller-supplied native catalog (`SEMIO_PLUGIN_CATALOG` or `install_native_catalog_v1`); without it the roster is empty (`index out of bounds: the len is 0`, `layout.exportPdf is published`). `git show HEAD:` of `🔎️search/🧪️tests/🔬️long/🦀️.rs`, `📇️registry/🧰️catalog-source-discovery/🦀️.rs` and the kernel `📥️runtime/🦀️.rs` is byte-identical to the working tree. The other two installed-catalog tests (raw-input, en/de census) pass vacuously on the empty roster. With `SEMIO_PLUGIN_CATALOG=✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` four of the installed-catalog tests fail on app plugin content (bim destructive flags and descriptions, stdio-pdf extension descriptions, undeclared gesture routes, tied search scores). These are app content audits that belong to the app owner, not the MCP framework. Not changed; moving these census tests to `✏️s/🧑‍💻dev` and giving them the app catalog is the clean fix.
- inference-hub `the_json_mirror_publishes_exactly_the_registry_exports`: `🧬️schema/🔣️.json` (and the TS mirror) is stale against `schemas()`. Differences found by diffing both sides: two descriptions compacted to `[Design notes](README.md#source-notes-N)` in the Rust docs (`ArtifactInferenceRequestV1`, the context channel text), `required` arrays order (the generator sorts), `after` maximum (16 vs `INFERENCE_PROGRESS_MAX_CURSOR`), and number rendering (`0` vs `0.0`). The documented remedy is `repo run @semio-tech/framework-os-mcp-rs:schema-mirror`, which also rewrites `🧬️schema/🟦️.ts`. Two attempts did not finish: the project-graph read restarts every pass because paths change under it ("1 path(s) changed under the read", 407 s and 172 s). Not hand-patched, since the TS mirror must come from the same run and `inference.list` pagination edits are in flight in the same files. Run it when the tree is calm.
- plugin-runtime lib test: pre-existing at HEAD. `🏠️workspace/🪶️sqlite/🧪️tests/guest-input-slot.rs` (md5 identical to HEAD) calls `ControlledRetirement::step(grant)` and `NativeEncodeControl::close_retirement_recipient`, while HEAD's value crate has `step(body, effect)` and only `close_retirement_recipient_into` (E0061/E0599). The lib itself compiles (the module is `cfg(test)`). Rewriting it belongs to the value retirement owner. Two other runs of this crate were cancelled by the host warden (`semio_os_mcp_plugin_runtime` peaked at 3055 MB against a 2980 MB carving budget); the carve needs re-measuring.
- probe-document: `original_mutation_receiving::...keeps_each_original_domain_failure` panics `declared cleanup stock did not close its actual owner` (`🏠️workspace/🔬️probe-document/📥️receiving/🧪️tests/🦀️.rs:17`). Test and receiving source are unchanged against HEAD; only `Cargo.toml` (new dev-dependencies and a `[[test]]`) differs. The close loop drives `OriginalRetirementPump` and the value retirement and decode modules, which have uncommitted edits by another owner. I could not establish whether HEAD behaves the same without a HEAD build, so it is not attributed to our changes and not changed.

## Notes

- hub-pair failed first with `Invalid Cargo workspace member patterns` and now builds (0 tests): a transient from the shared tree.
- routing was blocked only by the plugin-runtime warden cancellation; it now builds (0 tests).
- No source-boundary limit was raised, no test deleted.
