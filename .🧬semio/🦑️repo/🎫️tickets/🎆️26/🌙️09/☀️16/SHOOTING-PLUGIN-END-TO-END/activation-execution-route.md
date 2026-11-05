# Declared Activation Execution Route

Read-only source handoff; no build, test, activation, server, graph run or queue was started. Commands below are intended only after native acceptance. This report resolves the current route, not an execution result.

## Executable Owner Sequence

From the repository root, execute:

```sh
repo run @semio-tech/framework-os-dev:activate-shooting-react-prototype --skip-cache --lang=en --output=stream
```

Retain actual producer/materialization completion and snapshot the installed identities/receipt. Then execute the same exact command once more with unchanged selected inputs to establish physical stage/activation re-execution rather than a cache hit. If peer inputs changed, report that fact and do not call it an unchanged-input comparison. The native CLI neutral skip-cache flag maps to skip-nx-cache in the current invocation adapter. Do not replace issued repo run with direct Bun/owner script execution.

After successful current publication and unchanged-install comparison:

```sh
repo run @semio-tech/framework-os-dev:serve-shooting-react-prototype --lang=en --output=stream
```

A separately useful declared preparation route exists as repo run @semio-tech/framework-os-dev:prepare-shooting-react-prototype --skip-cache --lang=en --output=stream, but it is not necessary before the first activation command because activation declares that prerequisite. Preparation alone does not publish the runtime receipt/installations.

## Current Target Guarantees

The actual target generator is library/build/graph/inference/playground-targets. Current React activate target has cache:true, parallelism:false, selected transitive outputs/owner-source inputs, and dependsOn prepare-shooting-react-prototype. Preparation depends on registry session, framework-plugin-web:support, infinite fonts, renderer engines, selected engines and boot components. The owner script router registers real PreparationScript/ActivationScript/ServeScript lazy owners.

Current serve-shooting-react-prototype depends only on plugin-registry:session-shooting; it does not guarantee activation. Therefore starting this serve target alone is insufficient freshness/stage proof. The dev-shooting-react-prototype target does depend on activation, but is continuous and adds watch behavior; explicit activation followed by serve provides the bounded reviewable sequence requested. playgroundActivationTargetV1 independently returns the same authored activation target string after validating the variant catalog.

## Actual Evidence Locations

Use the current partition resolver browserProductRootsV1, not retired dev/plugin-modules trees. Its products partition protocol is browser/prototype/modules, browser/prototype/sessions, browser/prototype/runtime. The current generated target output is .cache/b/p/m/browser/prototype/runtime/react/shooting; actual resolver paths at command execution remain authoritative if storage configuration changes.

Activation writes runtime/react/shooting/activation/🔣️receipt.json. Installed extension files and installation metadata are below runtime/react/shooting/extensions/<declared module directory>. Use moduleDirectoryName from the actual catalog mapping instead of assuming a plugin-id directory. The installation owner writes EXTENSION_INSTALL_META and packageHash to that publication. Freshness indices live in runtime/react/shooting/🔖️/<bounded component key>, separate from module artifacts. Current module/component receipt evidence must be collected from the canonical module and component producer outputs described in runtime-acceptance-plan.md.

Retain before/after evidence and command output under this ticket generated directory while execution is active, then retain verdicts in Markdown and perform normal completion cleanup. Actual logs must show the prepare/activate stage executed under fresh issued provenance; task cache hit or the57-test helper result does not satisfy physical installation proof. No standalone route is missing: the real inferred activation target is available, although it still carries normal graph/admission requirements documented in serve-control.md.
