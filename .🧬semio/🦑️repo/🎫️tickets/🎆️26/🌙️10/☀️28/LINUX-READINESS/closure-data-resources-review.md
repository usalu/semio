# Closure DATA Resource Registration Review

Static current Source review, 2026-10-29. No validation helper, test, Native command, build, lease or process operation was executed. Root's canonical33879 result predates this metadata field correction:25tests/205assertions plus1prerequisite is historical execution evidence, not current corrected capture acceptance.

## Actual Public Contract And Consumer

The project schema at library/🏗️build/🧬️schema/📋️project/🔣️.json:234–255 places resources inside metadata.semio.closure; closure is closed (additionalProperties=false), and resources references $defs.SourceResourceEntries. That array permits1–64 unique exact anchored projectRoot/workspaceRoot paths, each<=256chars, with no traversal/globs/invalid path characters. It has no filename extension exclusion, so a .ts file is valid DATA. $defs.RuntimeSourceEntries separately requires executable JS/TS suffixes and excludes declaration modules. No cross-array disjointness constraint exists.

Actual admission reader library/🏗️build/🕸️graph/🚪️admission/🟦️.ts:62 validates SourceResourceEntries and freezes a copied array. Actual targetScriptSources in graph/📥️inference/🧭️script-inputs/🟨️.mjs:527 reads target.metadata?.semio?.closure?.resources, resolves anchors and rejects escaping/oversized or duplicate physical resources. A sibling runtimeSourceResourceEntries/resourceEntries field is not consulted by this reader.

In the bounded walk, lines393–396 resolve literal URL reads and set source:false when their exact target is a declared resource. At365 the file enters the DATA/resources set and returns before import analysis. TS suffix alone therefore does not confer executable authority for a declared URL DATA read. Physical-source checks still run and the file remains in the closure.

Direct imports are different: lines372–385 retain each actual import's source flag, and the executable branch367 adds the path to the compiled/modules set and traverses imports. The file set and compiled set are separate; encountering a path first as DATA does not prevent a later executable import because the compiled check, not the general seen flag, controls executable traversal. A TS file can therefore legitimately appear in codeSources and resource observations when actual independent uses justify both. Declaring it as DATA does not downgrade or suppress a real executable import. No consumer rejection for such overlap appears in the reviewed helper; physical duplicate resource entries themselves still refuse.

## Current Browser Leaf

Current C test-browser-dependencies has sourceInputObservation=freshness, one runtimeSourceEntries member (system neutral test TS), and closure.resources containing exactly native/🟦️.ts and the native bootstrap Bash file. The neutral test reads both through readFileSync(new URL(...)); these match the consumer's literal URL DATA route. It separately dynamically imports system/🟦️.ts, which remains executable and imports policy JSON normally. Inputs explicitly include system/**/*, native branch TS and bootstrap Bash. No ignored sibling resource field remains in this inspected target.

Current default C test remains uncached repo:noop with187 unique dependsOn target names, including test-browser-dependencies. DATA registration does not replace canonical per-leaf issuance or narrow the default aggregate. Earlier map capture/test success does not prove current metadata was captured; genuine current generation/readmission remains its separate gate.

## Shell TS Follow-up

Read the exact manifest at framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/📋️project.json: project @semio-tech/framework-os-shell, target test. It has sourceInputObservation=freshness, three runtime entries (existing reducer, existing schema-module law, new Rust-package command test) and closure.resources containing the Rust-package command helper TS plus Rust Cargo.toml. No ignored sibling resource field remains. Test inputs cover the test/schema trees and exact Cargo.toml.

The new test reads helper TS through readFileSync(new URL('../🟦️.ts',import.meta.url)) into TypeScript AST and reads Cargo.toml through the corresponding literal URL. Both exact DATA entries match actual consumer semantics. The helper itself has executable imports, but the AST-only DATA read does not admit those imports. If an existing script route legitimately imports the helper as code, the independent compiled set retains that executable closure; overlap is accepted by the actual public schema/helper, not a grant supplied by DATA metadata. No fresh test/capture execution is claimed.
