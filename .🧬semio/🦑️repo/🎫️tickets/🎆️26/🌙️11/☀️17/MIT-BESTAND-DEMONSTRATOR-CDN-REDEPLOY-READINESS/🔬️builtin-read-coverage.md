# Builtin-Read Import Coverage Refusal

## Symptom
`repo cargo ... check -p semio-framework-value --lib` failed in the Bun capture step: `Closed compiler builtin-read import coverage refused`.

## Where it is produced
- Code: `redirectCapturedBuiltinReadsV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts` (throws `builtin-read-coverage-refused`).
- Message text: authorizers schema `⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json`.

## Invariant
The captured loader redirects every built-in `fs`, `fs/promises` and `module` read of a captured module to the captured reader. The number of rewrites must equal the number of imports that Bun's scanner counts for those specifiers. A `require("node:fs")` through a local `createRequire` binding is rewritten by the tokenizer (count +1) but is not an import for the scanner (count unchanged), so coverage cannot be proven and the module is refused. The only legitimate form is a literal static import.

## Root cause
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🏃️execution/🟨️.mjs` read the Nx registry with `require("node:fs").readFileSync(...)` where `require` is `createRequire(import.meta.url)`. Found by running the real function over every non-test source in `🧰️framework/`: this was the only non-test module refused.

## Fix
Same file: `import {readFileSync} from "node:fs"` and call `readFileSync(registryPath,"utf8")`. No schema or coverage list change: the policy forbids the dynamic form, the static import is the declared interface.

## Verification
`repo cargo --store=shared --configuration=prototype --parallelism=1 -- check -p semio-framework-value --lib` no longer prints the coverage refusal. It passed source capture and reached cargo scheduling, where it waited 843 s for compiler image headroom (9.3 GiB free, 22.4 GiB reserved for commands ahead, other compilers hold all 2649 units). My 900 s timeout then cancelled it (SIGTERM, leases released), so no `cargo check` result exists. Rerun when the image has headroom.
