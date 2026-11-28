# BUILD_OWNER_FAILED on @semio-tech/repo-script-providers (provider-generated-import)

## Symptom
`repo run @semio-tech/framework-rs:generate --lang=en` (01:09, reference 297c0d7b) refused every owner that rests on a script provider port
(`cargo-workspace`, `cargo-command`, `artifact-package`) with
`invalid build.closure.provider-generated-import {"targets":"@semio-tech/ui-rs:generate"}`.

## Where the invariant lives
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`:
`scriptProviderGroupV1` and `toolClosureGroupV1` call `generatedTargetsV1` on the bounded walk of every provider port and of every port it names, and refuse
when a walked module imports a path that a `generatorContracts` output root (`🔣️taxonomy.json`, `ui-axes`, owner `@semio-tech/ui-rs:generate`) produces:
`🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts`. Provider closures must stay free of generated outputs, otherwise the projects naming them could not depend on the producer.
The owner `@semio-tech/repo-script-providers` is `🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/📋️project.json`; its `cargo-workspace` port is
`../🦀️cargo-workspace/🔌️provider/🟦️.ts`.

## Root cause
A concurrent edit, not the inference code. At 00:59 the plugin store (`🔌️plugin/🏪️store/📥️installation`, `📦️packaging`) and the manifest package export map
gained `@semio-tech/framework-manifest/localized-labels` (`🛂️manifest/🏷️localized-labels/🟦️.ts`). The `extension-package` provider port
(`🔌️plugin/🏪️store/📦️packaging/🔌️provider`, a port named by the provider walk) reaches that module, which at that moment took `SHELL_LOCALES` and
`SHELL_TERMINOLOGIES` as values from `../🤖️generated/🎚️ui-axes/🟦️.ts`, an ignored output of `ui-rs:generate`. Every provider closure therefore carried a
generated import and every owner resting on a provider port failed on unchanged inputs.

## State now
At 01:10:33 the same lane rewrote `🏷️localized-labels/🟦️.ts` to read the ids from the owned source `🖱️ui/🎚️axes/🔣️.json` and to import the generated module
type-only (erased in the command scope, so not a closure edge). Verified with a walk using the real `runtimeModuleSourceImports` and `commandUrlReferences`
parsers from `⚡️caching/📥️inference` over the `extension-package` and `cargo-workspace` ports (2793 and 2507 modules): the only specifier mentioning `ui-axes` is the
non-generated `🏗️build/🚀️bootstrap/🎚️ui-axes`. The invariant itself behaves correctly (law row in
`🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🟦️.ts`, "a provider closure that imports a generated output ... refused").

## Dry run
`env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo run @semio-tech/framework-rs:generate --lang=en --dry-run`
(log `🗑️generated/dry2.log`): "graph: plan answered and verified", "tasks: 1 stale, 0 up to date"; no BUILD_OWNER_FAILED. The run then went on to wait for
host capacity to execute the task despite `--dry-run` and was cancelled by SIGINT before it started (no compile).

## Guard
Rule for any module reachable from a provider port (including the manifest package and the plugin store): never value-import `🛂️manifest/🤖️generated/*` or
`🖱️ui/🎚️axes/🤖️generated/*`; read the owned source (`🖱️ui/🎚️axes/🔣️.json`) or use `import type`. No source change was needed on my side.
