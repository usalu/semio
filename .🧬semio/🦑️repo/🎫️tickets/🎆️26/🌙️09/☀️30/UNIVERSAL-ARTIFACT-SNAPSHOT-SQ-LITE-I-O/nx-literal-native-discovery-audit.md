# Literal Native Wrapper Discovery Admission

Read-only fresh audit, 2026-10-10. No source edits, execution, or tests. Library paths are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Fresh Facts

`🔌️nx-plugin/📤️arguments/🟨️.mjs:17` now ends `conforms` with `return Object.hasOwn(contract,"const")`. A correctly matching type-less const declaration is admitted. Older graph-time `Original discovered native command refused` logs cannot be attributed to current Unicode/path syntax without new evidence.

`🟨️.mjs:936–951` still adds nativeOwnerCommand only inside `if(nativePolicyTarget)`. It installs executor, wrapper, declaration, cwd and Cargo test policy together. There is no earlier admission branch for a target already declaring the literal native owner wrapper. The runtime refusal in `🔌️nx-plugin/📤️arguments/🟨️.mjs:79` correctly rejects native-wrapper text lacking `options.nativeOwnerCommand`.

Actor's **current** `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json:20–27` no longer declares a manual native wrapper. It declares `bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🎭️actor/📦️packages/🦀️rust -- bun ./📜️script.ts test-original-receiving-source`, cwd `🧰️framework`, forwardAllArgs true. The older actor wire failure remains evidence of its past normalized missing-request state, not proof that this fresh declaration still fails.

Actor's `📜️script.ts:20–33` Source class performs strict TypeScript checking then Bun tests through runOwnedCommand, using original signal and remaining deadline. Its exact router registration is `.register("test-original-receiving-source", OriginalReceivingSourceScript)` at line 48. Its four test inputs are retained-turn root tests, retained-turn policy tests, retained-turn receiving tests, and Actor checkpoint tests. The router also imports Cargo test and Wasm build implementations for its sibling commands. Thus per-file import closure is not equivalent to the selected source command's runtime language.

`targetScriptClosure` (`🟨️.mjs:459`) collects all literal tokens ending 📜️script.ts relative to one outer cwd; it does not interpret nested process `--cwd`. Its existence filter can therefore discard Actor's nested `./📜️script.ts` when resolved from framework root. `nativeTargetCommandInputs:477` additionally unions fallback native command inputs with this closure. That union should be inspected before treating nativePolicyTarget as evidence that the selected command actually uses native execution.

## Uniform Admission Recommendation

Use one literal token parser for declaration and wrapper admission in the existing arguments module, instead of substring detection or a second incompatible regex. Parse the already-declared wrapper structurally: exact Bun driver invocation, `native owner-command`, one manifest, one cwd, `--`, then nonempty original child argv. Require the driver resolves to the repository-owned Cargo script, validate manifest exists and is the project's nativeRoot owner, and require workingDirectory is inside workspace. Validate extracted `{kind:"native-owner-command",manifest,workingDirectory,program,arguments}` against the existing closed lower command schema. Reject shell syntax, duplicate switches, missing divider/child, escaped roots, foreign driver/manifest and any preexisting request differing from the extracted declaration.

In `projectWithDefaults`, process this explicit-wrapper case **before** the inference branch. Install `@semio-tech/repo-lib:owner-command` and the discovered request without wrapping the wrapper again. The inferred nativePolicyTarget case should call the same declaration helper after binding its actual child argv. Keep explicit existing wrapper command byte-for-byte stable for `declared.command===options.command` admission. Discovery must return declared command and request atomically, not rely on a user capability command or late executor fallback. Preserve original selected graph reachability, environment, resource objects, policy, deadline and closed top-level Nx caller contract. Do not make nativeOwnerCommand optional for a wrapper or synthesize default authority.

For ordinary source process commands, resolve the selected script with its declared process cwd and command segment if selected-command ownership needs native classification. Do not classify solely by all sibling router imports or fallback inputs. This cleanly distinguishes genuine handwritten native wrappers from source-only tasks and prevents unnecessary double wrapping.

## Exact Existing Source Test Registration

Library `📦️packages/🟦️typescript/📜️script.ts:122` already routes `test nx-native-child-issuance` to `🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts`, through runRepositoryTestCommand and repoTestArtifactEnvironment with quick budget. Project target is `📦️packages/🟦️typescript/📋️project.json:2012`, `test-nx-native-child-issuance`, command `bun ./📜️script.ts test nx-native-child-issuance`, cache false, forwardAllArgs false. GUI registration exists at `.vscode/launch.json:50181`, command `bun nx run @semio-tech/repo-lib:test-nx-native-child-issuance`.

Extend that existing test seam with neutral explicit-wrapper cases and an independent schema oracle. Cover extracted exact argv, no second wrapper, valid source task classification, missing/differing request refusal, foreign manifest/driver/cwd, quoted Unicode paths, and original deadline/resource identity through issuance. The current fixture covers direct executor issuance, but not projectWithDefaults discovery. Add a discovery assertion using actual normalization rather than merely building equivalent options in test inputs. No test result is claimed here.
