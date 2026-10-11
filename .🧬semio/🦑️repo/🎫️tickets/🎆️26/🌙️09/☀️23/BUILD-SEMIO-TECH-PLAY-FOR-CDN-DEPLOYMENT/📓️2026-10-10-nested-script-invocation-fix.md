# 2026-10-10 Nested Script Invocation Fix

Cause: `Script`/`BundleScript` require `(root, repoRoot, invocation)`; every direct nested construction still passed one or two arguments and threw `Script invocation must be an object`.

Method: listed every class extending `Script`/`BundleScript` (734 names, transitive, whole main tree) and every `new <Name>(` of them, plus `new (await import(...)).X(` and `new Route/Contract/Command(` forms.

## Production sites (all pass `this.invocation`; all targets are Script subclasses)

| File:line | Class | Change |
| --- | --- | --- |
| `📜️script.ts:178-179` | `NativeOsScript`, `NativeDependenciesScript` | `+ this.invocation` |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts:124` | `CheckAxesScript` | `+ this.invocation` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:43,59` | `SchemaScript`, `RunScript` | `+ this.invocation` |
| `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:599-602` | `RetainedVerificationScript`, `MemberHistoryInput/Id/IdentitySourceScript` | `+ this.invocation` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:192` | `ArtifactPackageContractScript` | `+ this.invocation` |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🟦️typescript/📜️script.ts:34-35` | `ContractTestScript`, `CheckScript` | were one-arg `(this.root)` (repoRoot undefined too) -> `(root, repoRoot, invocation)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:34,40` | `ActivationScript` | one-arg -> `(root, repoRoot, invocation)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:15,54,55,102,108,114,122,139,144,145,147,148` | `TestScript`, `PlaygroundSession{Preview,Generate}Script`, `BenchPluginsScript`, `ScaleFixtureGenerate/CheckScript`, `DistributionBundleScript`, parity `Route`, `PluginWatch/CapabilityLint/Size/BuildScript` | one/two-arg -> `(root, repoRoot, invocation)` |
| `✏️s/🧑‍💻dev/📜️script.ts:34-35` | `PlaygroundSession{Preview,Generate}Script` | `(root, repoRoot, invocation, request)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🏃️execution/🟦️.ts:16-17` | `PlaygroundSessionGenerateScript` constructor | was `(root, request)` + `super(root)`; now `(root, repoRoot, invocation, request)` + `super(root, repoRoot, invocation)`; imports `type ScriptInvocation` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts:108` | `PluginCapabilityLintScript` | one-arg -> `(root, repoRoot, invocation)` |
| `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts:14,49` | `PrintFontProvisioningCommand`, `PrintPipelineVerificationCommand` | `+ this.invocation` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:16` | `Contract` (`ScriptCommand` map value) | `+ this.invocation` |
| `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts:136` | `InferenceDiscoveryOracleScript` (inside `InferenceDiscoveryCheckScript.run`) | `+ this.invocation` |

Not touched: `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts:34` (`GenerateScript`, coordinator-owned; note `🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/🟦️.ts:22` text-matches `new GenerateScript(` in that file).

## Test sites (genuine invocation via `withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"<test>",maximumElapsedMilliseconds:0},{},Date.now()), ...)`; no shared helper existed outside routing's private `fixtureInvocation`)

| File | Class | Change |
| --- | --- | --- |
| `…/📚️library/🔍️discovery/📤️schema-registry/🧪️tests/🟦️.ts:46` | `SchemaScript` | test wrapped in envelope |
| `…/📚️library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts` (4 sites) | `CleanScript` | local `clean(root)` helper, tests made async |
| `…/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts:229,246,248` | `CleanMechanismNewScript`, `NewScript` | test wrapped in envelope |
| `…/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts:176` | `SetupScript` | dynamic import of envelope module |
| `…/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts:6118` | `CleanScript` in `bun -e` child | envelope created inside the child, module path passed as argv |
| `…/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts:19,22` | `CleanScript` | local `clean(root)` helper |
| `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/📏️ownership/🟦️.ts:45-46` | `CheckScript`, `PreviewGeneratedScript` in esbuild stdin program | envelope imported in the bundled program |
| `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts:100` | `PreviewGeneratedScript` in esbuild stdin program | envelope imported in the bundled program |
| `…/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts:63` | `NativeDependenciesScript` | wrapped rejects loop |
| `…/📚️library/⚡️caching/🧪️tests/🎨️styling-outputs/🟦️.ts:38`, `🐍️styling-python-outputs/🟦️.ts:42` | `StylingDotnet*Script`, `StylingPython*Script` in generated fixture `📜️script.ts` | envelope created in the generated script |
| `…/📚️library/🖱️ui/🧪️tests/🧭️router-ownership/🟦️.ts:99` | `DevScript`, `BuildScript` (real `BundleScript` in a vm context) | test wrapped, `invocation` added to context |
| `🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️testing/📍️consumer/🏃️execution/🧪️tests/🟦️.ts:15` | `Witness extends ComputeOwnershipTestScript` | envelope around the run |

Checked, not Script constructions (no change): `new TestScript()` / `new Native()` in taxonomy-pattern-compiler-reuse, artifact-empty-facet-authoring, artifact-empty-facet-authority, taxonomy-leading-grapheme, workspace-contract:5189, native-orchestration test-body (all use a local stub base class); `new Command(repoRoot)` in live-activation test (stub `BundleScript`); `dependencies/tests` already passes `original`.

## runScriptMain / router.run callers

All 391 `📜️script.ts` files with a router use `receiveScriptProcessInvocation`, `withScriptProcessEnvelope`, `runWorkspaceScriptMain` or `runOwnedNxInvocationV1`, except three legacy bundles that called the nonexistent `runBundleScriptMain(router, import.meta.url)` and imported `BundleScript`/`ScriptRouter`/`playgroundDevPortString`/`playgroundPortEnv` from the library index, which no longer exports them (SyntaxError at import). Fixed to the standard bottom line `receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original))` and re-pointed imports to `routing/🟦️.ts` and `📚️library/🎮️playground/🟦️.ts`:

- `👴️leutwiler/💤️realparts-of-powers-z-n/📜️script.ts`
- `♻️mit-bestand/🧺️demonstrator/📜️script.ts`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📜️script.ts`

Also fixed one stale assertion: `…/📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts:57` expected the old `runScriptMain(router, import.meta.url` text; now expects the `receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original` bottom line that the dev script really has.

Left alone (fixture text only, never executed): `runScriptMain(router, import.meta.url)` inside generated source strings in `🔬️workspace-contract/🟦️.ts:6616` and `🔄️transaction-v2/🟦️.ts:321`.

## Verification

Syntax: `bun build --no-bundle` OK on every edited file.

Runtime (nested construction reached, no `Script invocation must be an object`):
- `bun ./📜️script.ts setup deps --bogus` -> NativeDependenciesScript constructed, fails with the domain error `Unknown dependency environment`.
- dev package (envelope env supplied): `distribution bogus` -> domain usage error from `DistributionBundleScript`; `scale-fixture check` -> `fresh`, exit 0; `playground-session check` -> `generated source is fresh`, exit 0.
- `✏️s/🧑‍💻dev`: `playground-session check` -> exit 0 (4-arg constructor path).
- schema package: `test subset-contract` -> `64 vectors passed`.
- legacy bundles: `nosuchcommand` -> router `unknown command` (previously import SyntaxError).
- Tests run green: schema-registry (4 pass), clean-ticket-runs (4 pass, needs `--timeout 120000` under load), router-ownership storybook case, 2d compute (2 pass), graph suite (21 pass), root-clean-scaffold-source scaffold case, cross-platform-bootstrap git case, entity-kinds ownership proof (`bun` direct call).
- Not run: native-dependencies and both styling tests (need Nx/dotnet/uv), exact-cargo-laws, workspace-contract CLI case (describe skipped in this environment; its child snippet was exercised by hand and reaches the cleanup workflow).

Side effect to know: running `CleanScript` for real (clean-ticket-runs test) executes `stray-processes` machine-wide and killed orphaned processes (`ppid=1` rustc and a bun child) while verifying. Do not re-run those tests while an orphan-sensitive build is in flight.

Note: bun requires a `./` prefix for emoji test paths (`bun test ./<path>`); without it bun segfaulted.
