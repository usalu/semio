# React Engine Contract Reconciliation

## Scope

Reconciled the thirteen failures identified in `📓️terra-react-engine-census-triage.md`. The failures were stale expectations, brittle source anchors, a Board mount synchronization race, a transformed-URL fixture read, and DOM test leakage. No React production behavior changed.

## Contract updates

- Catalog choices now assert the current canonical encoded tuple and its resulting `kindId`; removed presentation-only `schema` from the expected choice.
- Board peer isolation waits for all four mounted sessions to publish their initial empty selection before clearing spies and testing peer-scoped routing.
- Sync browse routing uses the physical `data-semio-sync-browse` control rendered through the portal. English/German wording is asserted through the locale corpus, and the locale is restored after the law.
- The sync fixture is imported through its repository module path and the WGPU host-I/O source through Vite `?raw`, avoiding transformed browser URLs in `readFileSync`.
- Declarative input lookup is scoped to the rendered view.
- The shared Flow parameter fixture/schema and mounted host law use the current `nodeGraphEdit` envelope: one bounded `setSlider` operation with explicit gesture and commit fields.
- World edge hover asserts the current highlighted token.
- The self-gating world law uses a controlled unresolved round trip to prove one in-flight/latest-only behavior, then checks the four live callbacks return `dispatchSettled`.
- World publication assertions include the current full DOM projection, exact per-window/document scope behavior, and rendered-instance `activeObjectId`.
- The projection pane law uses `getElementById`, which supports the authored punctuation without relying on absent jsdom `CSS.escape`.
- The action-owner roster records both declared `translateSelection` owners.

## Source boundary

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json`

## Verification

`SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run <engine-contract> --config <react-config> -t <thirteen repaired laws> --reporter=dot`

Result: **13 passed, 648 skipped**, one file passed, exit 0, 16.11 seconds. The run emitted the suite's existing jsdom canvas and React `act` diagnostics; neither produced a failed law. This is the focused thirteen-law result, not a fresh full 661-law engine-contract census.

A final neutral/schema check selected the strict Ajv graph-parameter law plus the localized physical browse-control law: **2 passed, 659 skipped**, exit 0.

*** Add File: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️astra-sol-ingress-resident.md
# WGPU Ingress And Resident Boundary

## Contributions ingress

`wgpuContributionsIngressSize` now encodes command and view exactly once and derives the enclosing pack byte length from unsigned-varint widths. It no longer builds JavaScript number arrays or duplicate whole-envelope bytes. The independent law compares derived lengths to the actual encoder at varint boundaries and at the measured 190,701-byte contribution shape.

Focused result: **15/15 passed** in the WGPU extension-dispatch test.

## Resident refresh

The neutral fixture records the measured 835,986-byte full-document reservation and independent ceilings: 62 full populated documents admit, while the 63rd refuses on bytes before the 64-slot ceiling. Production capacities were unchanged. The Rust law populates the documents rather than opening cold builders, and the TypeScript law validates the same measured arithmetic.

Focused React result: **5/5 passed**. Root's later UI55 full run reported **642/642 passed, 0 skipped**, including the populated 62/63 law. No Cargo command was run in this packet.

## Source boundary

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎟️resident-refresh-budget/🟦️.ts`
