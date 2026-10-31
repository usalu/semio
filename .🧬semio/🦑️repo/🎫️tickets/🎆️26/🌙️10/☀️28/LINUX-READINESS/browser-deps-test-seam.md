# Browser Dependency Test Seam

Read-only Source inspection, 2026-10-29. No runtime test, native probe, install, lease or process mutation was performed.

## Exact Constraint

Current `prepareDependencies(kind, workspace, invocation, storage, phase, runner, environment, goEnvironment, uvEnvironment)` checks both:
- requireIssuedResourceControlInvocationV1(phase.leaseControl, invocation);
- phase.authority.original === invocation.capabilities.

A genuine phase comes from nativeCallerPhaseV1, which calls readNativeCallerAuthorityV1. An empty-capability framework test Original has capabilities {}, which fails the closed native caller schema. Therefore **a direct successful prepareDependencies branch test using an empty-capability Original is impossible under the current genuine API**. Do not manufacture NativeCallerPhaseV1, replace authority.original, stub the guards, or issue a second envelope in the test.

## Smallest Genuine Empty-Capability Test

Extract only the browser sequence into a production-owned helper, called by the real prepareDependencies browsers branch, e.g. `prepareBrowserDependenciesV1`. That helper accepts the actual ScriptInvocation and its genuinely issued ResourceLeaseControl, platform facts and an injected command port. It checks requireIssuedResourceControlInvocationV1(control, invocation). It does not accept or invent a NativeCallerPhaseV1. The production call supplies phase.leaseControl and the existing `run` closure, so actual tool children remain exactly phase-authorized. The helper owns **both** awaited system prerequisites and the download, making ordering/failure/cancellation tests genuine behavior tests of the production branch body.

Register a framework law using registerBuildTestLawV1 and a declared empty-capability birth. Derive control with createIssuedResourceControlV1(original, "browser:dependency-law", 0). Call the real helper, injecting recording system-package facts/commands. Assert the same Original/control reaches the port, system prerequisites precede browser download, and installer failure prevents download. Neither real storage nor native phase is needed because the injected port performs no effects. Validate fixture/output with both own schema validator and Ajv.

This tests the actual extracted branch body, **not** a direct prepareDependencies invocation. Describe that boundary accurately. A direct prepareDependencies integration law requires the existing declared native-build-caller grant instead of empty capabilities; see below.

Current framework registerBuildTestLawV1 retains private issuance, bounded callback deadline, cancellation and settlement. Use it rather than ad hoc withScriptProcessEnvelope/createScriptProcessEnvelope. A deliberately failing operation may close its process Original; use existing registerIsolatedBuildTestV1 for negative cases that genuinely reject an issued operation, rather than poisoning a later shared law.

## Existing Genuine Native-Grant Alternative

`📚️library/🧪️testing/🪪️native-birth/🟦️.ts` owns declareLibraryBuildTestEnvelopeV1. It registers the native-build-caller issuer and derives capabilities using createBuildBootstrapCapabilitiesV1, matching the official runner. This is the existing supported route for direct nativeCallerPhaseV1 tests.

`📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧪️tests/🟦️.ts` already uses registerBuildTestLawV1("...", "caller-birth", original => ...), creates a genuine issued control then calls nativeCallerPhaseV1. Reuse that pattern only in a suite with the genuine declared native grant. Do not present this as an empty-capability test. Existing old dependency generation tests that call createScriptProcessEnvelope manually are not the model requested here.

Direct integration still needs CargoPreparationStorageV1: {version:1,directory,leaseDirectory,program}, parsed by parseCargoPreparationStorageV1, and runner parameter exactly:
`(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:Readonly<Record<string,string|undefined>>,storage:CargoPreparationStorageV1,phase:NativeCallerPhaseV1)=>Promise<string>`.
The real phase interface is {authority, leaseControl, budget():number, advance(command):Promise<void>, options(environment):OwnedCommandOptions}. Preserve runner identity/phase/signal assertions; never cast a plain phase object.

## Exact Relative Imports

For proposed test directory `📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🌐️system/🧪️tests/`:

| API | Relative import |
|---|---|
| prepareDependencies / DependencyToolRunner | ../../🟦️.ts |
| NativeDependencyToolRunnerV1 | ../../../🟦️.ts |
| nativeCallerPhaseV1 / NativeCallerPhaseV1 | ../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts |
| declareLibraryBuildTestEnvelopeV1 (native alternative) | ../../../../../../🧪️testing/🪪️native-birth/🟦️.ts |
| framework declareBuildTestEnvelopeV1 / registerBuildTestLawV1 / registerIsolatedBuildTestV1 | ../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts |
| createIssuedResourceControlV1 / requireIssuedResourceControlInvocationV1 / ScriptInvocation | ../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts |
| adjacent browser helper | ../🟦️.ts |
| adjacent corpus | ./🔣️.json |

These relative paths were calculated from current filesystem topology, not guessed.

## Contribution Registration

Current owner contribution is `📚️library/⚡️caching/🗂️suites/🔣️.json`.
Its `bootstrap` row has budgetMs15000, typecheck none, trees:["🚀️bootstrap"], platforms darwin/linux, platformCause python3-command. A new test beneath native/🌐️system is currently inside that tree; simply placing a file there does not prove the desired empty-capability birth matches the official row.

Preferred explicit new row `browser-dependencies` with files:["🚀️bootstrap/📦️dependencies/🏗️native/🌐️system/🧪️tests/🟦️.ts"], typecheck none, all darwin/linux/win32 platforms, and a finite budget for recording-only laws. Give the row and test the **same empty capabilities** declaration through the existing runner's contribution/birth schema. Explicit-file ownership should take precedence over the broad bootstrap tree; verify with the existing suite law/collector before claiming registration. The exact generated repo-lib target should follow current contribution generation; do not hand-add parallel root targets.

Existing neutral fixture to move/reuse is `🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain/🔣️.json` browserPackages, with dnf/pacman/zypper exact command arrays and package lists. Existing Ajv browser law is in the adjacent 🟦️.ts. Keep these expected rows and add apt/root/nonroot/win/mac/order/failure/cancellation cases at the new owner. No runtime readiness claim follows from recording-port success.
