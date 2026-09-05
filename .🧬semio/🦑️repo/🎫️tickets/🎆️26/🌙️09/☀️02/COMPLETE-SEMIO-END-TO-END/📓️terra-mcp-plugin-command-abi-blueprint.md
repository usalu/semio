# MCP Descriptor-Owned Plugin Command ABI — Current-Source Audit and P0 Blueprint

Date: 2026-09-04  
Scope: read-only current-tree review of MCP stdio/HTTP command dispatch and the installed plugin catalog. No build or runtime command was run for this report. “Source-positive” below is not runtime acceptance.

## Bounded verdict

**RED — no installed descriptor-owned plugin command can currently execute honestly through MCP.** The real `os.agent.probe/v1` document journey is deliberately an OS-owned probe, not an installed-plugin command, so it cannot be credited for this scope. The current MCP action tools can discover descriptor claims and enforce the outer agent-policy check, but they neither bind the descriptor to its generated package identity nor carry a descriptor command address, document scope, command completion lifecycle, or capability broker grant into the guest.

The first useful packet must make one **folder-backed, verified, descriptor-owned editor command** execute via a real artifact document and actual stdio JSON-RPC. It must remain unavailable for hub documents until D0 supplies a verified linked codec/open plan. It must not reuse `os.agent.probe/v1` or fabricate a guest result.

## Current path, with authority boundaries

| Stage | Current source evidence | Classification |
| --- | --- | --- |
| CLI and process authority | [`📦️bin.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs:1) accepts only `--folder` or `--hub --space`; its tests reject `--token` at [202–209](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs:202). The MCP workspace path obtains hub authority separately through the protected local credential boundary. | Source-positive for “no CLI token”; not command execution proof. |
| Descriptor discovery | `RegistryDiscovery::scan` reads the generated registry and each owner `🔣️.json`, reporting malformed rows as diagnostics, then sorts descriptors [registry 35–59](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️.rs:35). `build_catalog` compiles those descriptors and fails closed to gateway-only if compile fails [MCP 161–176](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:161). | Source-positive discovery only. |
| Catalog ownership | The catalog preserves `CapabilitySource::Command { plugin_id, app_id, mode_id, command_id }` [catalog 260–273](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs:260), and creates distinct app, mode, and plugin command IDs with matching owner data [780–826](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs:780). | Source-positive schema projection; owner is discarded before guest dispatch. |
| Outer MCP authorization and audit | `ActionAdapter::prepare` validates input and calls `policy.authorize_scopes` before `ReadHistory`/`PureCommand` [dispatch 483–516](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:483); audit hashes and redacts sensitive input [437–459](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:437). | Source-positive outer policy/audit only. It is not the guest capability grant. |
| Standard stdio action handler | The registered `action_prepare` and `action_invoke` call `ActionAdapter` with `default_session()` and hard-coded `instance = 0` [MCP 374–398](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:374). `build_server_with_workspace` creates a separate root adapter and registers precisely those closures [596–607](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:596). | **RED:** the stdio server does not call `HeadlessWorkspace::prepare_action`, the sole path that derives an owner plugin’s instance slot. |
| Plugin routing | A `PureCommand` resolves only `plugin_id`; `ReadHistory` uses the numeric instance slot [workspace 1043–1085](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1043). The workspace-only helper correctly calculates that slot before the initial history read [1585–1607](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1585), but is not registered as the stdio action handler. | **RED:** in a multi-plugin catalog, normal `action_prepare` reads the lexicographic slot 0 plugin before the actual command’s plugin is known. |
| Registry/package identity | The generated registry actually contains `packageId`, `hashes.wasmSha256`, and `hashes.descriptorSha256` (for example, `animate`), but MCP parses only `pluginId`, `cratePath`, and `wasmOut` [workspace 70–124](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:70); it reads descriptor JSON and component bytes independently [131–161](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:131). | **RED:** no verification of generated registry package ID, descriptor hash, descriptor plugin ID, wasm digest, or descriptor/component pairing. |
| Chosen app and command address | `open_plugin_artifact_channel` ignores the catalog `CapabilitySource`, unconditionally chooses the first `Editor` app, and passes only an app reference [1098–1111](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1098). | **RED:** plugin-scope, app-scope, and mode-scope command ownership are not carried to the host/guest. |
| Guest capability authorization | Both activation paths issue every descriptor request as `CapabilityToken(0)` with no expiry [workspace 548–565](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:548) and [778–787](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:778). | **RED:** outer policy approval does not mint or intersect guest grants; all descriptor requests are admitted with an immortal sentinel token. |
| Guest command delivery | Native `persistent_command_completion_port_ready()` is literal `false` [908–911](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:908), so the first real exchange returns `channel.not-wired` before ingress [823–848](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:823). `ensure_instance` also converts a completed `InstanceOpen` into a retry-budget fault [769–799](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:769). | **RED:** no initial command can complete. |
| Actual ABI and document | `PureCommand` serializes only `{ capabilityId, input }` and sends empty document/config/draft payloads [965–988](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:965). The sole shared guest implementation rejects a bare command frame for missing exact manifest command key [plugin 21946–21950](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21946). | **RED:** no owner-qualified command ABI, no document bytes, no document scope and therefore no valid descriptor command. |
| Artifact actor lifecycle | `HeadlessWorkspace` owns and drops only `open_probes`; drop closes those ArtifactHost keys [1184–1225](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1184). The routing cache owns plugin channels during an adapter call and calls `exchange` while holding its cache mutex [1157–1166](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1157). | **RED:** no command-owned ArtifactHost actor, close/cancel generation, or nonblocking per-command ownership; a slow guest serializes a plugin under the routing lock. |
| D1 document transport | `ensure_probe_artifact` opens a scoped ArtifactHost document and checks its document key [1361–1408](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1361), but the source explicitly reserves this for `os.agent.probe/v1` [1411–1417](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1411). | Source-positive separate probe, **not** a generic artifact command or command ABI proof. |

## Exact REDs and security consequences

1. **Wrong-plugin history prelude.** The stdio handler’s `instance = 0` means the baseline can come from another plugin before the capability-bearing `PureCommand` routes correctly. A prepared handle can therefore bind an unrelated revision.
2. **Descriptor substitution.** The available generated digests are ignored. A changed descriptor, wrong plugin ID, stale wasm, or mismatched `wasmOut` can become the component compiled for advertised catalog capability claims.
3. **Authority widening at guest admission.** Descriptor requests are granted without a principal/policy intersection, a nonzero opaque token, a document scope, expiry, or revocation epoch.
4. **No canonical command ABI.** The host loses app/mode/command ownership and sends unscoped JSON bytes. The guest correctly rejects them, rather than running a potentially ambiguous command.
5. **No actual document execution.** A command receives empty artifact lanes and has no authenticated document key. This prevents cross-document authorization today only because execution fails closed; replacing the error with a direct call would be an authority bypass.
6. **No end-to-end cancellation/reconnect safety.** There is no command job handle, owner state, close path, or D1 actor-generation fence bound to guest execution. Existing D1 receipt/SocketGrant protections cannot automatically guard a channel that never opens a real descriptor document.
7. **No registered non-vacuous command gate.** `canonical-pair-check` exact-selects four P4-C laws [script 85–103](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:85) and validates credential-marker rejection, but proves neither a descriptor command nor a plugin actor. The registered launch entry is P4-C only [launch 4488–4492](/Users/ueli/Documents/semio/.vscode/launch.json:4488).

## Smallest honest P0 implementation packet

### Scope and nonclaims

Implement exactly one **fixture-backed, descriptor-owned editor command** in a `--folder` workspace. It must mutate or query a real folder artifact through `ArtifactHost`; it must return a real guest `AppFrame`/result through stdio. It does not activate all catalog plugins, extensions, UI actions, inference, or hub documents. It makes no claim for a hub document until the D0 linked native codec/open-plan binding independently verifies the schema and component.

### 1. Verified execution descriptor

Extend the MCP registry load boundary, not raw catalog discovery, with an immutable `VerifiedPluginInstallV1` carrying:

- `plugin_id`, `package_id`, normalized owner root and `wasm_out`;
- the generated descriptor and wasm SHA-256 values, parsed as fixed 32-byte digests;
- canonical descriptor bytes/digest and decoded descriptor with `descriptor.manifest.plugin_id == plugin_id`;
- canonical component bytes/digest with `wasm_out` and registry digest exact equality; and
- the exact selected `CapabilitySource::Command` plus `CommandAddress` (`Plugin`, `App`, or `Mode`).

Missing hashes, duplicate plugin IDs, descriptor ID/package mismatch, invalid digest, missing component, component mismatch, or a capability not owned by that verified descriptor must be terminal `PLUGIN_UNAVAILABLE`; do not degrade an advertised plugin command to a generic component.

The registry already has the source digest material; the necessary first change is to preserve and verify it rather than regenerate it or trust marketplace/cache rows.

### 2. Schema-first invocation and routing

Add `HeadlessPluginCommandRequestV1` to the MCP action contract:

```
catalog_hash, capability_id, exact_command_address,
artifact_document_key, expected_revision, input,
invocation_id, authority_generation
```

`artifact_document_key` must include the authorized space/document/schema binding, not merely an artifact string. The prepared handle stores the complete verified-install identity, address, document key, chosen app/mode, principal ID, expected revision, and D1 actor generation. It cannot be replayed into another workspace, document, plugin, session, or socket generation.

Replace the root closures at MCP [374–398](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:374) with workspace-aware prepare/invoke paths, or change their captured context so they call one workspace-owned action service. Derive the instance from the verified command address before every `ReadHistory`; never use default session or 0 for a live workspace command.

### 3. Capability broker and activation

Create an instance-scoped broker lease from the intersection:

```
authenticated principal × catalog policy × verified descriptor request
× ArtifactDocumentKey × authority_generation × fixed expiry
```

Mint nonzero opaque tokens. Pass only granted requests to `InstanceOpen`; record denied requests as a structured guest-unavailable error without leaking secrets. Revoke the lease before driver/channel/actor close. No descriptor gets a zero token or unbounded expiry.

### 4. Real artifact and command port

Introduce a `HeadlessPluginCommandSession` owned by the workspace/action service. It owns, in order: verified install, ArtifactHost document channels, document codec/store, guest actor, command driver, cancellation token, and D1 authority generation. It must:

1. validate the verified codec/document plan and open the authenticated ArtifactHost key;
2. attach the document/backbone before `InstanceOpen`;
3. send a versioned, size-bounded `CommandIngressV1` that contains `CommandAddress`, input, document lanes/revision, principal/actor metadata, and capability lease references;
4. replace `persistent_command_completion_port_ready() == false` with an owned submit/poll/cancel port, correlating one sequence to exactly one response/fault and bounded cleanup;
5. map guest `Emit`/fault/terminal completion into MCP result/error without fabricating success; and
6. on MCP cancellation, peer disconnect, authority generation change, socket failure, or expiry: reject new ingress, cancel once, wipe staged payloads/lease bytes, close the driver and ArtifactHost key in reverse ownership order. A late success must close and must not mutate/report after cancellation.

Do not retain the routing cache mutex while executing guest code. Cache verified immutable compile products separately; command sessions are per `{document key, actor generation, invocation}` and must release the registry lock before any `execute_actor_turn`.

### 5. First honest target

Select one generated-registry row that has all three current facts: a committed descriptor, its verified wasm digest, and one editor command with a folder-decodable codec. Pin its plugin ID, app ID, command ID, and schema in a checked-in neutral fixture. The selection must be data-driven by the verified install, not “first Editor app.” If none currently meet all facts, the gate must fail before starting stdio; no probe substitution.

This deliberately leaves hub execution RED until D0 has `NativeOpenableCatalogProviderV1`/linked codec identity and the D1 open-plan verifies the same document key.

## Neutral fixture/oracle and gates

### Rust focused laws (exact-one selected)

1. `verified_install_rejects_missing_extra_and_descriptor_or_component_digest_mismatch`
2. `command_prepare_binds_exact_plugin_app_mode_command_document_and_initial_history_slot`
3. `guest_capability_lease_intersects_policy_expires_and_revokes_before_actor_close`
4. `folder_stdio_command_round_trip_uses_real_artifact_host_and_owner_qualified_wire`
5. `command_rejects_cross_document_cross_plugin_mode_and_stale_revision_before_guest_turn`
6. `command_port_bounds_pages_duplicates_and_malformed_or_oversize_guest_result`
7. `cancel_or_generation_change_closes_late_success_without_mutation_or_mcp_result`
8. `disconnect_reconnect_requires_fresh_receipt_actor_and_never_reuses_command_lease`
9. `audit_redacts_capability_lease_component_bytes_and_sensitive_input_on_every_failure_path`

Each law must drive the public stdio/tool route and real `HeadlessPluginCommandSession`; a direct `VcsArtifactApp` unit test is supporting evidence only.

### Language-neutral oracle

Commit a small JSONL corpus, consumed by an independent Bun/Node parser and the MCP test harness, containing:

- valid registry/descriptor/component hashes and an exact `CommandAddress`/document key;
- one valid prepare→invoke stdio transcript with observable revision/result;
- descriptor/component hash mismatch, plugin ID mismatch, missing/extra digest, malformed/ambiguous address, wrong app/mode, foreign document and stale revision;
- unauthorized/expired/revoked lease; malformed, duplicate and over-limit guest completion; and
- cancellation before open, after receipt, after ingress, and late response after authority turnover.

The oracle verifies canonical digest parsing, frame size and address/schema validation independently; it must not call Rust command helpers or accept the OS probe schema.

### Registration required

Add one noncached MCP target, implemented only via the existing `📜️script.ts` convention:

```
bun nx run @semio-tech/framework-os-mcp-rs:plugin-command-abi-check --skip-nx-cache
```

Its script must build the real MCP binary, prove exact-one selection for all nine suffixes, execute every fully-qualified Rust law, execute the Bun neutral oracle, and then run the MCP and hub all-feature checks. Add an adjacent `⚖️gate…plugin-command-abi` launch entry in `.vscode/launch.json`. Keep the existing P4-C `canonical-pair-check` distinct: it is useful transport evidence, not this command ABI gate.

The follow-on process gate must start the real binary as a direct child with the protected fd3 injection path, send only the neutral stdio transcript, and assert byte-clean stdout with no credential, lease, component bytes, or raw document secret in either stdout/stderr/audit.

## Implementation order and acceptance

1. **P0-A, independent:** verified registry/install identity plus neutral digest/address corpus.
2. **P0-B, depends P0-A:** workspace-aware action context and prepared-handle binding; fixes the `instance = 0` route before accepting any command.
3. **P0-C, depends P0-A/B and folder codec:** owner-qualified command ingress, least-privilege broker lease, bounded completion port, and real folder ArtifactHost document session.
4. **P0-D, depends P0-C:** exact registered gate and direct-child stdio process oracle.
5. **Later, separately gated:** hub document execution only after D0 codec/open-plan and D1 authenticated actor readiness prove the same document/schema/package binding.

Bounded acceptance is one verified folder descriptor command producing a real artifact result through stdio, with the nine hostile laws and the non-vacuous gate terminal green. It makes no claim that the remaining installed catalog, extensions, arbitrary app actions, UI, browser, native WGPU, or hub document commands are enabled.
