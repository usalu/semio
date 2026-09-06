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

## Wave 2 and 3 — dispatched 2026-09-05 13:30–23:50

| Lane | Model | Scope | Packet consumed | Report |
|---|---|---|---|---|
| `fable-mcp-inference-bridge` | Opus 5 | `inference_submit/events/cancel/approve` tools, typed hub client, policy/handle gating | `📓️fable-explore-mcp-inference-bridge.md` | `📓️fable-mcp-inference-bridge.md` (done: 18 lib laws, 6 process laws) |
| `fable-gis-map-inference-ui-port` | Opus 5 | Slice D port in React ShellHost + WGPU, kernel `Effect::RequestInferenceProposal`, EN/DE | `📓️fable-explore-gis-map-inference-ui-port.md` | `📓️fable-gis-map-inference-ui-port.md` (GIS/WGPU test placeholders pending) |
| `fable-vcs-native-provider` | Opus 5 | second native openable provider (VCS receipts, provider-set link) | `📓️fable-explore-vcs-provider-frontier.md` | `📓️fable-vcs-native-provider.md` (cargo terminal pending) |
| `fable-mcp-artifact-quick-recursion` | Opus 5 | two-frame `ToValue` self-call abort; follow-up: WGPU socket-probe twin + store `Drop` witness guard | sibling blocker | `📓️fable-mcp-artifact-quick-recursion.md` |
| `fable-two-user-space-journey` | Opus 5 | `os-hub:space-journey-check` process gate over SQLite with two real identities | `📓️fable-explore-two-user-journey-readiness.md` | `📓️fable-two-user-space-journey.md` |
| `fable-ai-map-proposal` (follow-up) | Opus 5 | non-`cfg(test)` GIS Map bundle builder, four route-level laws, parent-only committer, `--process` design | `📓️fable-explore-inference-readiness-path.md` | dated follow-up in `📓️fable-ai-map-proposal.md` |
| `fable-execution-target-lease` (follow-up) | Opus 5 | run the three native laws now that the plugin host compiles | `📓️fable-explore-residual-graph-refresh.md` §3.2 | "Native evidence" section in `📓️fable-execution-target-lease.md` |
| `fable-presence-normalization-fixture` | Opus 5 | paired test-only readiness helper so `presence-normalization-check --native` reaches presence logic | `📓️fable-explore-residual-graph-refresh.md` §3.1 | `📓️fable-presence-normalization-fixture.md` |
| `fable-pack-lossless-integer` | Opus 5 | lossless dynamic Pack integers (`TAG_UINT`/`TAG_INT`), Rust + TS + corpus + oracle | `📓️fable-explore-residual-graph-refresh.md` §3.4 | `📓️fable-pack-lossless-integer.md` |
| explorers (Sonnet 5, read-only) | — | two-user journey readiness, inference readiness path, residual graph refresh | — | `📓️fable-explore-{two-user-journey-readiness,inference-readiness-path,residual-graph-refresh}.md` |

Coordinator-owned: `📓️fable-coordinator-repairs.md` (re-exports, stdio projection path, four `pack_schema_hash` repins) and the consolidated `semio-hub --bin os-hub --tests` check in `scratchpad/fable-coordinator-hub-target` (first run: only the Sol db WAL-writer-permit E0308 outside any Fable file; rerun in progress). Lanes must use lane-qualified private target dirs and never clone that target while its lock is held.

### 2026-09-06 00:10 — VCS link incident and Sol repin request

`fable-vcs-native-provider` linked `semio-s-plugin-vcs` into `semio-hub`'s default build (`native-artifact-execution`) while the VCS crate still carries 18 compile errors from the framework migrations (`render` → `ComponentTree`, async io, `DslValue`/`JsonValue`, prelude `Label`, removed testkit laws; the lane's own module produced zero errors). Decision: keep the link (feature-gating it would be the shim the rules forbid) and repair the crate immediately in `fable-vcs-compile-repair`; the five hub-building lanes were told to attribute VCS failures correctly and retry every 15 minutes. Request to the Sol trusted-bundle lane: `trusted-stdio-gis-bundle-check --source` is RED only because stdio's projection was recommitted at 22:02 after the `generationId` pin at 03:53 — repin `7cf0515d… → b96fb865…` (see `📓️fable-vcs-native-provider.md` §7), which also re-verifies the `RECEIPTS 28 → 29` change.
