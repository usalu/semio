# Authorizer Shebang Import Parser Origin

Read-only current source audit and isolated compiler probe, 2026-10-13. No LAS route/native rerun or code/configuration change.

Concrete hook: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts:37`, `captureNxAuthorizerSourcesV1`, calls `new Bun.Transpiler({...}).scanImports(Buffer.from(bytes).toString("utf8"))`. The original bytes are hashed into closure first at35. Current tools `📜️script.ts:98–100` captures runtime closure, then compiler closure starting at `🔏️authorizers/📜️script.ts`; that actual file has `#!/usr/bin/env bun` at1. Its scanner cannot parse the compiler entry shebang, so failure precedes installation/compiled release authorizer/Nx task/TSC/LAS tests.

Isolated Bun probe using actual local Bun and TS loader/node target reproduced: `scanImports` on `#!/usr/bin/env bun\nimport {readFile} from "node:fs/promises";` throws `Unexpected #!/usr/bin/env bun`. On identical unchanged text, `scan` succeeds with `imports:[{kind:"import-statement",path:"node:fs/promises"}]`; `transformSync` also succeeds. Generated evidence is `🗑️generated/authorizer-shebang-parser-audit.txt`. This is an import-only scanner behavior difference; the shebang is legal real Bun script syntax.

Concrete repair candidate at the owner is system compiler `.scan(originalText).imports`, preserving all original source hashing/capture/no-follow/dependency checks and closure seals. It needs the owning scanner regression test before qualification. No script alias, byte rewriting, seal disablement, ignored parse exception or special dependency fallback is needed. Existing import-decoding and closed relative path validation at38 should remain.

Actual `las-original-receiving-source-current.log` retains the before: `input.ts:1:1`, Unexpected shebang, Bun1.3.14, script nx exit1. It is PRETEST evidence and supplies no LAS or Cargo denominator.
