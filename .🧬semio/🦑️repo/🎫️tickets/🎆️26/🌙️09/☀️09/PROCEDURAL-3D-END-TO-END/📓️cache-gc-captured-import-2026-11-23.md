# Cache gc: Undeclared captured Source import

## Symptom
`repo cache gc --lang=en` failed 8 times with `Undeclared captured Source import: ../../../../🏗️build/🪢️slots/🟦️.ts` from the Node captured-source loader (`registerHooks` resolve hook), followed by `closure-read-startup ... comparison: eof`.

## Root cause
- Importer: `🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts` line 12, `import {type DeclarationV1} from "../../../../🏗️build/🪢️slots/🟦️.ts"` (exactly four `../` from `💾️store`). An inline type-only import: every named binding carries `type`, no default binding.
- The capture scanner `closureLiteralSitesV1` (`⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts`) takes static sites from `TranspilerV1.scan` (Bun or the portable scanner). Both follow TypeScript semantics and treat `import {type A} from "x"` as erased, so the edge never entered the captured import map. The TypeScript parser oracle in the tests shares that rule.
- The Node loader runs sources through `stripTypeScriptTypes`, which keeps the bare `import {} from "x"` (and `export {} from "x"`). Node therefore resolves the specifier, the hook finds no declared edge and refuses. Not a stale sealed closure: the closure matched the scanner, the scanner disagreed with the runtime.
- The first inline-type-only edge into a module otherwise outside the capture (store, gc/supersession, fleet/wire) exposed the gap.

## Fix
`closureLiteralSitesV1` adds a TS-only pass (`inlineTypeOnly` regex) that reports `import {type A, type B} from "x"` and `export {type A} from "x"` as static, lenient, scanned sites (lenient: a missing relative target is observed as absent, not fatal). Fully erased forms (`import type {..}`, `export type {..}`) stay excluded. No path special case.

## Test
`🧭️sites/🧪️tests/🟦️.ts`, "inline type-only import and re-export sites are captured as Node loads them": runs real `node` `stripTypeScriptTypes` over single, multi, export-from and multiline inline-type forms plus erased `import type`/`export type`, asserts every specifier Node leaves is a captured site and the fully erased ones are not. 7 pass via `repo test-file`.

## gc re-run
`repo cache gc --lang=en` completed (`🗑️generated/cache-gc-fixed.txt`), no `Undeclared` lines.
- Cache entries: within budget, 0 B evicted.
- Machine bounds: footprint-logs 248 KiB, read-verdicts 26.96 MiB, rest 0 B; about 27.2 MiB total.
- Image compaction of rust, dependencies and graph deferred with `lease-timeout` (two ticket builds hold leases; gc stayed lock-aware). Re-run after those builds finish to reclaim image slack.
- Volume free space went 13 GiB to 27 GiB during the session; not attributable to this gc run.
