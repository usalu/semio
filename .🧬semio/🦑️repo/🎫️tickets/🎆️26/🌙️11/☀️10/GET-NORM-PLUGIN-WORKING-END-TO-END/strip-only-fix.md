# Strip-only fix

Play's in-source test files failed to load with `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` (parameter property) because Node loads `🦑️repo/🔨️modules/📚️library/` externally and strips types only.

Offenders found by importing the play entry specifiers with node: `🏗️build/🗄️cas/🧹️collector/🧮️plan.ts` (`Rule`), `🤝️shared-worktree/🔁️replica/📇️book/🟦️.ts`, `🤝️shared-worktree/🎣️tiers/🟦️.ts`.

Fix: every non-test `.ts` under the library (27 files, remote-execution, remote-cache, cargo, schema parser-projection, crate-tree, declarations, ...) plus the two shared-worktree files had constructor parameter properties rewritten into explicit typed fields plus `this.x=x` assignments (after `super()` when present). Applied with a ts-AST codemod; `module.stripTypeScriptTypes` now accepts all non-test library files.

Result: the 3 files load; play run is 79 passed, 1 failed. The failure is `playInstalledArtifactSha256` asserting `activationFilesDigest` still exists in `♻️activation/📥️installation/🟦️.ts` (activation logic, out of scope).
