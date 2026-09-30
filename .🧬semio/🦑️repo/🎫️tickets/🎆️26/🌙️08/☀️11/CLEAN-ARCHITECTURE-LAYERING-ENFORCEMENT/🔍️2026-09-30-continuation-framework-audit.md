# Framework Inference Ownership Continuation Audit

Read-only source audit on 2026-09-30. No source mutations, Git mutations, runtime verification, or tests were performed. Only this retained report was authored. Root, products, OS, s, and GIS instructions were read; no more-specific AGENTS exists under the inspected directory/MCP paths.

## Finding

The outstanding coupling is a complete GIS approval client embedded in general OS infrastructure, not merely badly named structs. Moving the structs alone cannot establish removability. Framework production owns the GIS route, ring validator, proposal reducer, history state, worker messages, renderer chrome, and SVG geometry. MCP discovery already has a neutral contribution mechanism and should retain it.

## Evidence and Ownership

All paths below are relative to the repository root. Let `OS` denote `🧰️framework/🛍️products/💻️os` and `MCP` denote `OS/🔨️modules/🌉️mcp`.

| Current owner | Concrete responsibility | Correct destination |
| --- | --- | --- |
| `OS/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1998` onward | `GisMapInference*` request, receipt, page, preview, validator, port error; undo handle/request/receipt at 2230 onward | GIS-specific client extension; generic frontier remains directory owned |
| `OS/🔨️modules/📇️directory/🧬️schema/🟦️.ts` | First-party closed parsers, seals, phase reducer, localized copy, affordances and `projectGisMapInferencePreviewOverlayV1` | GIS client/view extension |
| `OS/🔨️modules/📇️directory/🔌️client/🦀️.rs:1191` | Hardcoded `/inference/gis-map` transport and typed submit/events/approval/undo calls | GIS transport extension using neutral directory credential/scope and HTTP interfaces |
| `OS/🔨️modules/📇️directory/🦀️.rs` and `OS/🟦️.ts:593` | General public reexports of all specific contracts | Remove; consumers import their specific owner directly |
| `MCP/💡️inference/🦀️.rs:479,792,801,838,1129` | GIS preview schema, five-point ring law, preview field in event page, GIS undo receipt in generic approval receipt, approval undo transport | Hub/GIS gateway contribution |
| `MCP/💡️inference-bridge/🟦️.ts` | GIS paths, hardcoded service, Hub schema exports and route text checks, proposal rectangle checks | Hub integration oracle |
| `OS/🧬️schema/🔣️.json` | `GisMapApprovalHistory*`, `GisMapInferencePort*` definitions | GIS extension schema; worker envelope stays OS owned |
| `OS/🔨️modules/📺️renderer/🧬️schema/{🔣️.json,🟦️.ts}` | GIS-specific renderer protocol | GIS contribution payload schema |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx:224` | GIS port view, region/longitude/latitude labels, SVG projection | GIS view contribution |
| Shell/ShellHost React and wgpu targets, `OS/🟦️.ts:1459`, wgpu server transport | Hardcoded status/history message plumbing | Neutral contributed panel/status delivery with owner-bound payload |
| `🌎️hub/💡️inference/🧬️schema/{🔣️.json,🦀️.rs,✅️approval/🦀️.rs}`, runtime, `🌎️hub/🏗️bootstrap/🦀️.rs` | Server GIS proposal/approval/undo publication authority | Keep server authority in Hub, composed with GIS codecs; consume outward-owned domain contract |

Directory root schema JSON was scanned but no `GisMap` definitions were found there; the specific fixture schemas live primarily in the OS root schema. Do not attribute the domain contract to the wrong JSON document during extraction.

The MCP ring validator checks schema/job/digest/derived region and axis-aligned closure. The TypeScript renderer projects coordinates into a 100×100 viewbox. Preserve those exact domain semantics and add finite-number vectors rather than weakening the transport into unchecked arbitrary JSON. `EditedArtifactFrontierV1` is already neutral; retain it in directory and use it from the domain contract.

## Coherent Inversion

1. Author the specific contract schema under the GIS artifact/extension ownership tree, alongside the inference owning the geometry. Derive/update Rust and TypeScript domain decoders and domain test corpus together. Hub-specific route/approval authority belongs under Hub's inference extension, not framework. Reconcile the existing Hub schema with this single outward owner instead of introducing duplicate schemas or aliases.
2. Keep framework inference discovery and generic job tracking. Existing `ContributedInferenceMetadata` and `InferencePayloadContract` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:6077` already publish owner, artifact target, dependency order, input/output schema, artifact binding, and `InferenceCommitBinding { action }`. MCP's `contributed_inference_descriptors` already discovers services transitively through installed descriptors. Use these declarations to dispatch and validate owner payloads, retaining the normal action policy/undo/ledger path.
3. A domain transport implementation should implement neutral first-party transport/contribution interfaces. Inject it from specific composition. Do not teach general directory clients about GIS service IDs or paths. Retain bounded bytes, request cancellation, credentials, scope encoding, cursor bounds and server outcome handling in the neutral mechanism.
4. Preview should be a schema-addressed owner payload bound to job/proposal identity, validated against the registered owner schema. Geometry interpretation and drawing stay in the GIS contribution. The general framework must not expose ring, longitude, latitude, region, GIS-specific history, or service-schema constants. Do not use an unvalidated JSON escape hatch.
5. Route preview/control output through existing plugin contribution and scene mechanisms, extending neutral schema only where needed. The generic shell hosts a contribution; domain code produces scene/view output and localized controls. Make the mechanism removal-safe: missing owner yields a bounded unavailable outcome and no panel. Avoid a global singleton GIS registry.
6. Durable approval/undo semantics stay server authorized and owner bound. A generic committed edit receipt may carry neutral revision/frontier/action identities; GIS undo request/schema and inverse construction stay specific. Do not simply rename `GisMap*` to `Inference*` while retaining the ring or `/gis-map` route.
7. Replace all exports, imports, fixtures and executable routes atomically; remove old names completely. Preserve no compatibility facade and no migration script.

## High Execution Agent Partition

One High agent should own this vertical slice: framework directory contracts/client; MCP inference Hub bridge; OS worker envelopes/root exports; React/wgpu preview consumers; the corresponding domain schemas/consumer imports in GIS and Hub; script/target/launch relocation for these tests. Give that agent sole coordination of the shared `OS/🟦️.ts`, schema exports and bootstrap consumers. This slice is inherently cross-cutting; partitioning it at struct ownership would leave compile failures and domain references in generic renderers.

Exclude Cargo fleet/plugin-to-artifact refactoring, root dependency scanner changes, generated proc-macro provenance, CAD/spatial contributions, broad task-router canonicalization, and unrelated changes in the same shared files. Coordinate any nested UI AGENTS before changing renderer internals. The High agent must inspect target script behavior before running a route; some targets invoke external services or browsers.

## Required Evidence and Existing Routes

Add a language-agnostic extension-removal corpus: a neutral service and GIS service installed, remove GIS contribution, neutral discovery/run/status/panel still work; reinstall/removal are deterministic; missing schema/foreign owner fail closed. Compare validation with existing Ajv and serde_json oracles. Preserve GIS vectors for ring, forged region/job/digest, open/empty/nonfinite geometry, cancellation, stale proposals, failed/uncommitted approvals, foreign/expired handles, idempotent replay, frontier conflicts, rebootstrap and two-author undo/redo. General fixtures must use neutral witness services; retain GIS end-to-end fixtures in Hub/GIS.

Existing tests to relocate or adapt include `OS/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs` (GIS inference around 770), `MCP/💡️inference/🧪️tests/🔬️quick/🦀️.rs`, `MCP/🧪️tests/💡️inference-bridge/🟦️.ts`, renderer scoped-presence/engine-contract/wgpu-command-registry tests, and Hub inference schema/runtime/approval tests. `OS/📦️packages/🟦️typescript/📜️script.ts` itself contains the GIS projection oracle and must lose specific ownership too.

Verified existing Nx project names/targets (not executed in this audit):

- `bun nx run @semio-tech/framework-os-mcp-rs:test-quick`
- `bun nx run @semio-tech/framework-os-mcp-rs:inference-discovery-oracle`
- `bun nx run @semio-tech/framework-os-mcp-rs:inference-discovery-check`
- `bun nx run @semio-tech/framework-os-mcp:inference-bridge-source-check`
- `bun nx run @semio-tech/framework-os-mcp:inference-bridge-process-check`
- `bun nx run @semio-tech/framework-os:gis-map-inference-port-check`
- `bun nx run @semio-tech/framework-os:gis-map-inference-port-browser-check`
- `bun nx run os-hub:gis-inference-ledger-oracle`
- `bun nx run os-hub:gis-inference-ledger-check`
- `bun nx run os-hub:inference-relay-check`

The two general GIS port targets and GIS-specific bridge source/process checks should move to specific composition projects, while generic discovery checks remain framework owned. Implement any new executable route solely in the owning `📜️script.ts`, route it through Nx, and register it in launch.json following the existing order. No directory package target was guessed: directory is mounted in the OS kernel and does not have the inspected `📦️packages/🦀️rust` path.

## Acceptance Limits

Source audit establishes actual coupling and an execution scope; it proves neither compilation nor runtime removability. Completion requires selected native laws and portable oracle parity, generic graph/build with GIS and Hub owners hidden, actual contribution removal/reinstall console evidence, and GIS approval/preview/undo runtime evidence through specific composition. Broad repository deletability remains a separate claim.
