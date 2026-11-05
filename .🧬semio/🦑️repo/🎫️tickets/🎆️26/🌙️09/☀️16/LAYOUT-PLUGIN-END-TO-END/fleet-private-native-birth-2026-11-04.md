# Private Workspace Native Test Birth Exploration

This is read-only architecture evidence. No production or test source was edited and native controller81717 remains the sole full Layout owner.

## Existing Boundaries

`library/🧪️testing/🪪️native-birth/🟦️.ts` prepares `SEMIO_WORKSPACE_ROOT` as the checkout. Its scratch=sandbox only selects the artifact sandbox, then capabilities come from `createBuildBootstrapCapabilitiesV1(checkout, grant.arguments, environment)`. Therefore sandbox scratch alone does not isolate compiler or products storage.

`library/🧪️testing/🧫️private-repository/📥️invocation/🟦️.ts` validates a private root under declared scratch parents, validates every store/control/lease directory stays inside it, and caps deepest paths at256. Its privateRepositoryInvocationV1 creates an in-process envelope via createScriptProcessEnvelope/withScriptProcessEnvelope, derives native capabilities for that root, pins CWD, and retains canonical compiler runtime entries from testCheckoutRootV1. This is useful isolation machinery, but its invoke callback cannot be presented as OS receiver proof.

The existing genuine OS pipeline is `library/🧪️testing/🌱️genuine-birth`: suite fileLaunchers call genuineBirthLaunchersV1→the genuine-birth script→runGenuineBunTestV1. That owner derives build bootstrap capabilities, creates its explicit birth, and invokes an actual Bun test process through runOwnedCommand with the issued envelope and finite remaining deadline. The child receives its Original. The current request.root conflates code checkout/source containment, CWD, workspace authority, artifact routing and tooling stores. Passing a private root today would reject the real checkout test file by source containment; copying sources is not an acceptable fix.

## Minimal Boundary Recommendation

Extend the schema-owned genuine birth selection with an explicit private-workspace native purpose. Keep code checkout separate from workspace authority: resolve/admit the test file and runtime Source entries from the immutable checkout; allocate/validate one private root through the existing private-repository owner before the OS child; derive native compiler/artifact/storage authority from that private root through the actual issuer, not by replacing fields on a received Original. Have the actual OS child receive the newly issued private Native caller and explicit checkout read identity. Carry the existing parent-issued deadline, cancellation and owned drainage to launch and retirement. Use the existing canonical runtime-entry pin owner with the checkout; do not copy Source, relabel a Binary or create a private CARGO_TARGET_DIR override. The native policy must derive the private root's canonical compiler directory.

The receiver matcher must validate this exact declared private purpose/root and canonical native policy rather than infer a private workspace from arbitrary environment fields. The general native-birth matches routine presently validates against actual.workspaceRoot, but prepare still chooses checkout; one shared schema-owned issuer must govern both normal parent launch and declared bare birth to avoid two incompatible births.

The WGPU fixture must take codeCheckout and workspace separately. Current `testWgpuWasmOutputs` reads real project descriptors, consumer Source, compiler entry and Cargo.lock using workspace, and also opens compiler/products stores with workspace. Separate those read-only checkout reads from all mutable stores. Its authored fixture Cargo.toml/Rust bytes already live under output scratch, so those are legitimate authored test data, not copied repository Source. All compiler/products publications, receipt checks, negative child purpose and cleanup must use private workspace Native authority. Tool provisioning/read-only official tool identity needs to remain explicit and policy-owned; no ambient PATH ingress or checkout-store write should be introduced.

## Required Proof

Neutral schema cases should reject checkout-as-private, escaping roots, undeclared purpose and changed canonical native artifact directory. The actual child must report its received workspace, private canonical stores and Source checkout identity as bounded observation DATA; the parent compares them and verifies the checkout compiler/products store is unchanged. Existing WGPU byte/export/publication/negative assertions remain intact. Genuine receiver proof comes from the OS child, never privateRepositoryWorkspace.invoke.

## Consumer Coupling Checkpoint

Native lane confirmed that repositoryCargoConfigurationOwnedV1 derives its compiler ingress Source from request.workspace; preparedBinaryen and the bindgen/root Cargo.lock policy also combine code/tool and mutable-workspace identity. A correct actual private OS birth therefore requires explicit issued immutable code/tool identity through those official owners, in addition to splitting the fixture reads. Arbitrary environment root selectors or renderer-only test options would not close that authority boundary. Shared schema ownership was sent to the parent before implementation.

Proposed API: runPrivateNativeTestProcessV1({original,control,codeRoot,privateWorkspace,purpose}), where the closed purpose selects the declared child entry and native command. The private workspace must be a real owner-issued object, not just a caller path; codeRoot must equal the defining checkout owner. The parent issuer preserves its absolute policy/deadline and admitted control while issuing new private Native workspace authority for an actual child process.

## Authored Implementation Checkpoint

The final parent API is `runPrivateNativeTestProcessV1({original,control,codeRoot,purpose})`; the trusted issuer owns allocation itself, rather than using the bun:test-dependent privateRepositoryWorkspace lifecycle. The direct actual OS receiver is `receivePrivateNativeTestProcessV1(purpose, callback)` with `{original,control,codeRoot,workspace,output}`. Closed purpose Source entries are fixed by schema. Only current exact `run` selections authorize a purpose: @semio-tech/repo-lib:test-private-native-birth for the witness, and @semio-tech/repo-lib:test-wasm-outputs / repo:test-wasm-outputs for WGPU, with only prototype/release suffixes. Incidental argv mentions do not authorize a purpose.

All four caller/authoring schemas now require bounded top-level `codeRoot`, beside workspaceRoot/repositoryRoot. Ordinary build/Nx factories explicitly write codeRoot=workspace, authoring resolves and requires that same identity, owner projections carry it, and the authority reader exposes it without fallback. The private issuer derives canonical workspace-native storage fresh and admits defining checkout codeRoot explicitly. Parent policy/deadline/control, transport, child and network ceilings remain bounded by the inherited caller. Finite phase/deadline is checked before allocation/launch and before receiver effects.

The child validates defining checkout, selected entry, real parent PID, actual private CWD, exact canonical native target path, and allocated directory dev/ino bound into its metadata. Runtime Source admission reads codeRoot. Actual Native stores/cleanup use private workspace. Parent observes actual spawn PID and exact bounded completion receipt, verifies deadline/limits/purpose/root, and checks physical directory absence. Metadata-only failure disposal is restricted to the same inode, regular bounded owned metadata and no Native/test-output entries. Store cleanup stays inside the actual child Original and retained runtime Source scope; failed cleanup preserves its primary error. The shared retirement core is extracted into private-repository/🪦️retirement and takes a genuine Original/control directly.

The normal native81717 was cancelled only after the parent's explicit decision that its issued pre-codeRoot grant cannot satisfy the new mandatory schema. Actual exit130, log says leases released after closure1811/2025; no native compiler/assertion verdict is claimed.

### Verification Evidence

- First normal portable run39855 ended0: eight codeRoot cases,16 assertions, independent Ajv agrees. Test was authored before implementation but executed after schema changes during normal queueing; it is GREEN, not an observed RED despite the birth log filename.
- Strict38061 ended2: owned relative-import/read-time errors plus existing Storybook optional declaration errors. Owned errors corrected; explicit unchanged-behavior guard adds declaration===undefined to the existing entry<1 return.
- Strict5948 ended0 across ten roots/import closure.
- Current complete **strict29031 ended0** across ten roots/import closure. Expanded **portable21436 ended0: three tests,40 assertions**. Fresh actual build and Nx constructors at the real checkout each reported validator issues=[], agreed with independent Ajv, and the authority reader returned exact codeRoot/workspaceRoot. Exact purpose-selection cases include both authorized current WGPU routes and reject incidental excluded-target tokens. Logs under 🗑️generated/fleet-host-2026-11-04/private-native-strict-final.log and private-native-portable.log.
- Actual focused CODE owner `testPrivateNativeBirthV1` and OS child witness are materialized. The child opens the real private products store; the parent checks observed identity/limits/retirement with independent Ajv and our validator. Actual execution is still pending, not inferred from source or strict checks.

Changed Source includes caller/authoring/Nx/native-owner schemas; build and Nx types/factories/projections/readers; Nx argument .mjs wire propagation; two explicit Cargo runtime-entry Source calls; genuine-birth private-workspace schema/owner/neutral fixture/tests/actual owner/child; extracted retirement owner and old private unit-test caller delegation. Root owns project catalog/target wiring, Medium owns WGPU and Cargo source/tool-root consumers, Low owns serialized data fixtures.

### Current Source Paths Owned by This Lane

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️caller.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authoring.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📋️contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/📨️envelope/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️orchestration/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🧰️preparation/🕸️engines/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🚪️owner/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/👷️child/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🪦️retirement/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🟦️.ts`

The root-owned actual-environment startup probe68977 is independent of the controlled-empty-environment constructor feature assertions. No epoch cause is claimed without its result/error paths. The scheduler's running private-native-birth label belongs to another logical task; this lane did not launch or control it and does not infer acceptance from that label.

## Current Factory and Wire Proof

Root-owned fresh normal registered Bun probe68977 ended0 with the actual environment and canonical private-native-birth run request: codeRoot and workspaceRoot both the real checkout, tool bazel. This proves current factory/schema coherence. Earlier pre-action refusals85548/18274 remain transition-staleness inference, not native body outcomes.

Read-only trace found one additional explicit neutral orchestration wire constructor omitted mandatory codeRoot. `🏗️build/📥️caller/📤️orchestration/🟨️.mjs` now carries caller.codeRoot, preserving a distinct immutable code identity rather than recomputing the workspace. Targeted portable successor47778 ended0: four tests,45 assertions, including the real neutral wire projector with distinct inherited codeRoot, unchanged native resource reference and independent Ajv. The input is serialized DATA; no ScriptOriginal is minted for that test. This one-line JS change follows strict29031; its existing .d.mts return type already names the updated canonical NativeBuildCallerCapabilitiesV1.

The private schema now has unique $id `semio:repo/private-native-test-process/v1`; parent refreshed the catalog/index registration. Exactly one owned normal declared actual OS witness is live as controller96692: `repo --tool=bazel run @semio-tech/repo-lib:test-private-native-birth`, birth log `🗑️generated/fleet-host-2026-11-04/private-native-os-witness.log`. It passed current bootstrap and entered normal Source/graph fact verification/planning. No actual child/store/retirement pass is claimed yet. No full native20 or activation controller is live in this lane.
