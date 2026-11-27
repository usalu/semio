# Workspace build owner: provider-generated-import

## Rule
`closure.provider-generated-import` is raised in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` (`scriptProviderGroupV1` and `toolClosureGroupV1`, around lines 1259 and 1276). It refuses a provider closure when any module in the walk of the port, or of any port that walk names transitively (`providerWalksV1`), imports a file that a generator contract declares as an ignored output root.

The `port` in the error is only the requested port. The offending module can sit in any transitively named provider.

## Generated output behind `@semio-tech/ui-rs:generate`
Contract `ui-axes` in `📚️library/🔣️taxonomy.json` owns two ignored output roots:
- `🖱️ui/🎚️axes/🤖️generated/🦀️.rs`
- `🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts`

`🛂️manifest/*` and `🕹️interaction/🟦️.ts` import the generated TypeScript file.

## Static reachability (python BFS over relative imports)
- The `artifact-package` walk names the ports `typescript-check`, `repository-tooling`, `artifact-staging`, `cargo-workspace` and `cargo-admission`. None of them reaches `🛂️manifest` at this moment.
- Only the `extension-package` provider (`💻️os/.../📦️packaging/🔌️provider`) reaches `🛂️manifest` (through `🎁️packs`, `🧾️app-channel` and the plugin installation chain). It is not a port of the `artifact-package` walk.

## Why it failed, and the current state
The failing read happened while other agents were editing `🛂️manifest` (`🏷️localized-labels`, `🤖️generated/*`, `package.json`), `🏃️process/🎛️owned-execution` and related files. Those files were modified within the last 90 minutes.

I did not capture which module the failing walk reached, so the transient edit state is a conclusion drawn from the static check and the mtimes, not something I observed directly.

The rule itself, `ui-rs/📋️project.json`, `taxonomy.json` and the provider port files are unchanged. There is no defect in the rule or in `ui-rs:generate` outputs, and I found nothing to fix in the repo. Suppressing the check would be wrong.

## Verification
- `repo show project workspace` completed with exit 0 and published the sealed graph with 2 owners. The `workspace` owner infers again, and the output has no `BUILD_OWNER_FAILED`.
- `repo run workspace:components -- describe stdio gis cad-extension-aec-building` was started in the background. It was waiting for capacity behind another job when this report was written. Its output is `🗑️generated/describe-out2.txt`.

## Files changed
None in the repo. Only this report and the `🗑️generated` outputs in the ticket folder.

## If it recurs
Add the importing module paths to the `refuseBuildV1` params of `closure.provider-generated-import`, for example by mapping `generatedImports` keys to targets, so the error names the file. The message today names only the port and target.
