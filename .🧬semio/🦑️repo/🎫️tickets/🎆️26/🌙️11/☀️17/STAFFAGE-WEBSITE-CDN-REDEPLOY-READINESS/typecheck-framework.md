# Staffage Site Typecheck, Framework Errors

`repo tools exec tsc -- -p tsconfig.json --noEmit` in the site typescript package: before 24 errors, after 8 (all remaining are the missing manifest generated module and its follow-ons).

## Source fixes (repo product types)
- `♻️reset/🟦️.ts`: the `observe` callback row type now includes the `"rescan"` phase (matches `NativeRecomputableRetirementProgressV1`). Fixes both TS2345.
- `🪪️identity/🟦️.ts`: the registry `view` map entry type now carries `group?: string` (entries already set it). Fixes TS2339.

## Generators (native, via native.sh)
- Actor typegen (`framework-actor-rs:typegen`, cwd `🎭️actor/📦️packages/🦀️rust`): ok, produced `🎭️actor/🤖️generated/🎭️actor/🟦️.ts`.
- UI contract generate (`🖱️ui/🧬️contract/📦️packages/🦀️rust`): ok, produced `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`.
- Manifest generate (`🧰️framework/📦️packages/🦀️rust`, `generate` through the process command wrapper, see project.json): hit the 900 s timeout (exit 124, cargo compile of the framework crate contended with other builds) and left an empty `🪪️manifest/🟦️.ts`, which I removed. Not produced.

## Remaining 8 errors
All stem from the missing `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (TS2307/2306 in 7 framework files, the follow-on TS2366 in app-definition-refresh-and-context-menu, and the TS7006 `option` in action-argument-resolution). Rerun the manifest generate with a longer timeout once the cargo build is not starved.

## Changed files
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts
- (gitignored generated) 🎭️actor/🤖️generated/🎭️actor/🟦️.ts, 🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts
