# Focused CJS Parser Prelaunch Refusal

No Native invocation, TTY, log/artifact mutation, or Source edit occurred.

The authored dispatch exactly matches35572bytes/SHA256ede3bf0729d0573ee80239b483ab281ffca6dcd819c23ff2e43e6ee44b0d971e. Its command string is malformed: the single-quoted --test-name-pattern opens with `^(?:` but the final title is followed by `) --timeout` rather than the required closing `$` anchor and singlequote. The timeout/log redirection suffix also appears twice. This contradicts the closed both-anchor/exact-seven-title dispatch requirement.

The executor did not repair/reconstruct the authorized command or execute it. Root was notified to author and review the concrete command. Existing reserved log/artifact paths were not opened for mutation.

Exact command DATA as read (not executed):

```json
"SEMIO_BUILD_MODE=release SEMIO_BUILD_BUDGET_MS=21600000 SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/cjs-parser-probe-9b17e66b0355' '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/🗃️bin/prototype/repo/repo' exec --projects=workspace -- bun test '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts' '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts' '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts' --test-name-pattern '^(?:workspace CommonJS facades consume the canonical default object against SQLite|actual immutable child invokes both CommonJS default-object factories|actual captured workspace CommonJS facades preserve public value behavior|closed installed parser entries match SQLite without executable resolution|actual captured parser witness uses closed entries matching native resolution|captured Source refusal diagnostics retain bounded actual context against SQLite|actual immutable loaders preserve their refusal guards while reporting declaring context) --timeout 90000 >> '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/2026-10-21-cjs-parser-focused-native.log' 2>&1 --timeout 90000 >> '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/2026-10-21-cjs-parser-focused-native.log' 2>&1"
```
