# Graph compiler dependency generation

The resident graph reader failed in `requirePhysicalSourceV1` while opening TypeScript compiler bytes. The failing path belonged to retired JavaScript dependency generation `d22a22b6385877c52eb2`. At diagnosis, the live root `node_modules` binding resolved to generation `22f6dd7ea5ff0b28270a`. The canonical run recorded the failure under reference `aa2bba6e` in the ticket log `draw-canonical-dev-source-restored.log`.

Source analysis previously used Bun's resident bare-package resolver to choose the file it hashes for the parser revision. It now uses the existing `typescriptCompilerFileV1` selector, which reads the parser's version declaration and resolves the live install slot to a physical package path. The selected source still goes through observed physical ancestry validation. If a generation moves during the read, one new selection can recover the attempt. The successful read contributes the recorded facts. An absent entry under the same binding still throws. Analysis providers are registered again if the compiler digest changes.

The regression covers retirement before capture with a warm module resolver, retirement between selection and read, and an absent compiler entry under a stable binding. It checks the recorded source paths and verifies the published entry independently with `enhanced-resolve`. The fixture is validated against its JSON Schema with Ajv.

The graph regression, resolution observation tests, compiler selection tests and Rust source analysis tests passed together with 18 tests and 117 assertions. The compiler tests for captured execution passed separately with five tests and 15 assertions. The production diff passed `git diff --check`.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️source-analyses/🧪️tests/🔄️compiler-swap/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️source-analyses/🧪️tests/🔄️compiler-swap/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️source-analyses/🧪️tests/🔄️compiler-swap/🟦️.ts`
