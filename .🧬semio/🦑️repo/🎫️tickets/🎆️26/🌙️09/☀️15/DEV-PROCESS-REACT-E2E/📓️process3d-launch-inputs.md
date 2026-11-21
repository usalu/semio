# Process3d Browser Launch Inputs

The Process3d React and WGPU launch targets inherited the broad `framework-os-dev` default input because their generated live rows omitted `inputs`. Selected owner admission expands an omitted list to `["default"]` and treats matched modules as compiler inputs unless the target declares freshness. The default input covers the dev tree and unrelated tests. Activation declares its own raw component owner globs. Those globs also gave the compiler role to module files they matched, including an extension build driver that activation reads as part of its source fingerprint.

The focused fixture calls the production target and session generators against the authored Process3d hub and four machine component manifests, the real runtime component resolver, and the real application catalog provider. It uses an empty native preparation plan to keep this metadata check independent of Cargo and excludes the workspace graph. It asserts both prototype and release, all eight React and WGPU dev and serve commands, stable environment values, exact prepare edges, and cache settings. SQLite independently checks source roles, and Picomatch independently checks representative paths against the production input matcher.

| Target | Generated behavior |
| --- | --- |
| Process3d session | Runs `repo ./📜️script.ts session process3d` and depends on `@semio-tech/s-dev-composition:generate` |
| React serve | Runs the React serve command and depends on the session |
| React dev | Runs the same React serve command and depends on React activation |
| WGPU dev and serve | Run the WGPU server on port 6122 with the authored composition path and depend on WGPU activation |
| React prepare | Depends on the session, browser support, fonts, Surface, Editor and NodeGraph WASM, and the Process3d hub with its Concrete, Metal, Robotic and Wood components |
| WGPU prepare | Depends on the session, browser support, fonts, WGPU WASM and the three WGPU boot and worker generators, and the same five components |

The target generator now gives continuous browser targets explicit inputs. React observes the authored development contribution JSON for freshness and declares its Vite config and browser entry as literal runtime sources. WGPU declares the authored composition module as a literal runtime source. When the optional declarations are absent, the generator supplies the default React Vite config and entry or the default WGPU server composition. The generated commands, environment, cache class and dependency edges match the original fixture.

Both activation targets keep their exact transitive task outputs, raw owner globs, deterministic cache class and runtime output directory. Their matched source role changes to freshness. This keeps raw source edits in the activation fingerprint while the launcher scripts and explicitly declared dynamic modules retain compiler authority. It removes the broad compiler-root scan and retains the activation content walk.

`process3d-launch-inputs.ts --require-fixed` checks the new metadata on the actual generator output. `--all-variants` adds a metadata-only sweep of the declared browser variants and the fallback clauses. `--closure` asks the production script-closure interface to resolve the literal runtime entries independently of the ordinary command closure. These modes run without serving browsers or compiling components.

## Verification

The baseline `repo tools exec --light bun -- ./<ticket>/process3d-launch-inputs.ts --require-fixed` exited 1 before the generator edit. For `dev-process3d-react-prototype`, the expected input was the literal contribution JSON and the actual input was `undefined`.

The corrected source passed `repo tools exec --light bun -- ./<ticket>/process3d-launch-inputs.ts --require-fixed --all-variants --closure` and `repo tools exec --light git -- diff --check`. The real target generator received 74 physical component manifests from the two declared catalogs and produced 158 distinct variants. All 1,264 generated live browser rows carried the expected narrow input metadata, including rows missing the React contribution or WGPU composition. The browser metadata sweep had zero refused rows. An empty native Cargo plan isolated the browser proof from each separate native prerequisite closure.

| Runtime entries passed to the real script-closure interface | Observed files | Compiled files |
| --- | ---: | ---: |
| Process3d React Vite config and browser entry | 3,320 | 2,671 |
| Process3d WGPU composition | 1,886 | 1,327 |
| Default React Vite config and browser entry | 3,303 | 2,659 |
| Default WGPU server composition | 1,876 | 1,321 |

The compiler's returned module set contains every literal runtime entry in each row. The unrelated dev configuration test appeared in the inherited default matches of all eight original Process3d live targets. The corrected generator excludes it from both the declared input matches and the four runtime closures. The fixture isolates the dynamic-root walk by omitting the ordinary command in this call. The same fixture asserts the emitted command and dispatch edges separately.

The measured runtime closures contain thousands of files. Declaring runtime entries disables command-provider stops in `keepsWholeWalkV1`. The isolated proof covers removal of default glob matches and preservation of runtime imports. Launch timing and persisted activation cache hits require a separate runtime measurement.

The existing inference cache test already invokes the same target generator against synthetic component manifests. Its checked-in regression can stay small by adding a contributed variant and a variant that uses the defaults, then asserting explicit live inputs, exact runtime entries, and freshness-only activation while retaining the existing command, dependency and cache assertions. The ticket's physical catalog sweep and script-closure evidence can remain separate from the unit test.

The combined closure check then ran the production target defaults and script resolver on both the actual serve command and its declared runtime entries. It passed in all four cases. Every command-only compiled module stayed present in the combined result. The combined compilable module counts were 2,697 for Process3d React, 1,410 for Process3d WGPU, 2,685 for default React, and 1,404 for default WGPU. Run this check with `--require-fixed --all-variants --closure --combined`. The live launch duration and activation cache reuse remain unmeasured.

The repository's existing graph admission test now asserts the generated React and WGPU default input declarations for both profiles, including the runtime entries and freshness-only activation. It also requires the activation inputs to retain transitive outputs and both boot component source trees. The focused Bun invocation passed 104 assertions, including the existing cancellation checks. Scoped whitespace checks passed.

At 16:03 UTC the focused launch-input verifier passed again against the current checkout with `--require-fixed --all-variants --closure --combined`. It checked all 1,264 targets across 158 variants. Normalized action closures retained the complete declared runtime entry lists with 2,704 compile files for Process3d React, 1,413 for Process3d WGPU, 2,692 for default React and 1,407 for default WGPU. Both Process3d preparation plans still require its own component and the concrete, metal, robotic and wood components. This checks the present generator and normalization, before the browser launch under the older admitted source reaches target execution.

## Native run and smoke

The native targets also inherited the OS dev default compiler inputs. The fixture failed before the generator edit with `--require-fixed --native-launch`, where `run-process3d-native-prototype.inputs` was undefined. The corrected targets observe the existing transitive producer outputs, a literal owned launch policy when declared, and each applicable startup mesh catalog root from the selected app's asset declarations. The session and native binary prerequisites, arguments, environment and launch command match the original generator.

The default policy is statically imported by the native command. Raster's policy is read at runtime through its declared workspace-relative JSON path. The fixture exercised the production selector and observed no dynamic default read and exactly one read of the Raster policy. The native launcher still takes the scoped asset declarations from the retained session, including static directories and tile proxies. The existing mesh provider follows the explicit catalog path to the collection data it serves.

The isolated verifier passed with `--require-fixed --native-launch --all-variants --native-closure`. It checked 632 native run and smoke targets across 158 variants, including four with an owned launch policy and 20 with an authored startup mesh catalog, alongside the 1,264 browser targets. Picomatch independently agreed with the production input matcher and SQLite agreed on the freshness role. The real Process3d native script closure equals the ordinary command-only closure with 2,000 observations and 1,418 compiled modules. It includes the launcher and observes the default policy JSON. The unrelated dev configuration test is absent from the compiled module set.
