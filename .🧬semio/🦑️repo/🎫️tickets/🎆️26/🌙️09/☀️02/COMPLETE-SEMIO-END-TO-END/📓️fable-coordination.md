# Fable Fleet Coordination — 2026-09-05

Coordinator: Claude Fable 5.1 session `c34c334c-fe3e-420d-a00b-7b4aa1238be5` (repo MCP timed out; ticket managed on disk). Works in conjunction with the GPT-5.6 Sol/Terra fleet already active on this ticket and with the separate Claude sessions on `26/09/05/S-END-TO-END` (frontend boot) and `26/09/05/BLOCK-PLUGIN-END-TO-END`.

Lane selection follows `📓️terra-three-pillar-current-residual-execution-graph.md`: only residual slices with a Terra packet and no Sol implementation report were claimed. Sol-owned slices (public member open, retained Home event page, invite redemption transaction, scoped socket revocation, trusted stdio+gis bundle, Flow addWidget factory, presence-lease source) are not touched.

| Lane | Model | Scope | Packet consumed | Report |
|---|---|---|---|---|
| `fable-directory-command-receipt` | Opus 5 | P0-F: `DirectoryCommandRequestV1`/`ReceiptV1`, durable idempotency on `HubDirectory` (all backends), bounded browser/native/WGPU transport op, gate `os-hub:directory-command-receipt-check` | `📓️terra-directory-command-receipt-transport-p0.md` | `📓️fable-directory-command-receipt.md` |
| `fable-ai-map-proposal` | Opus 5 | P1-C/D slices A–C server-first: `VerifiedGisMapProposalBindingV1`, `HubInferenceRuntime` in `HubState`, four authenticated routes, ledger extension, server-built `CreateRegion`+inverse behind a fail-closed committer port, gate `os-hub:gis-map-proposal-check` | `📓️terra-ai-map-proposal-approval-current-p0.md` | `📓️fable-ai-map-proposal.md` |
| `fable-space-administration` | Opus 5 | `DirectorySpaceAdministrationPageV1`, role-shaped Home rows, `manageSpace` + Administration pane (React + WGPU, EN/DE), retained `DirectoryAdministrationOperation`, gate `os-hub:space-administration-check` | `📓️terra-space-administration-ui-current-p0.md`, `📓️terra-author-space-administration-page-receipt-p0.md` | `📓️fable-space-administration.md` |
| `fable-execution-target-lease` | Opus 5 | P0-C: `DocumentExecutionTargetLeaseFieldsV1`, selection-bound hub asset routes, browser verified lease (SHA-256 + first-party BLAKE3), wasm renderer admitted only with lease → localized renderer-unavailable, native field parity, three gates | `📓️terra-browser-gis-wasm-execution-target-lease-p0.md` | `📓️fable-execution-target-lease.md` |
| `fable-hub-native-qualification` | Opus 5 | Run the registered-but-unqualified hub gates (presence lease native/process, invite transaction native, ordered publication native, event page native/process, admin live journey) and record exact evidence; no production edits | Sol source reports | `📓️fable-hub-native-qualification.md` |
| `fable-explore-mcp-inference-bridge` | Sonnet 5 | read-only packet for wiring `semio-os-mcp` `inference_*` to the new hub inference routes | — | `📓️fable-explore-mcp-inference-bridge.md` |
| `fable-explore-gis-map-inference-ui-port` | Sonnet 5 | read-only packet for Slice D (host-owned ephemeral inference port in React ShellHost + WGPU) | — | `📓️fable-explore-gis-map-inference-ui-port.md` |
| `fable-explore-build-health-and-active-leases` | Sonnet 5 | read-only census of newest compiler receipts and active Cargo leases in `🗑️generated` and `ps` | — | `📓️fable-explore-build-health-and-active-leases.md` |
| `fable-explore-vcs-provider-frontier` | Sonnet 5 | read-only packet for the second native openable provider (VCS) | — | `📓️fable-explore-vcs-provider-frontier.md` |

Rules for every Fable lane: one foreground cargo process at a time, `CARGO_BUILD_JOBS=4`, narrowest targets, shared default target dir, outputs under `🗑️generated/fable-<lane>/`, no edits to `📋️master-plan.md`/`✅️acceptance-matrix.md` (coordinator only), no ticket lifecycle calls, no git-modifying commands.
