# Build Test Receiver Wrapper Inventory

Bounded read-only rg inventory over actual authored framework/plugin/company roots found 24 TypeScript files referencing the helper; 29 immediate redundant wrappers remain across 12 files. This is a lower bound recognizing direct async returns and block bodies whose first statement awaits/returns the helper; statements before helper use need AST follow-up. A broader no-ignore whole-repository scan exhausted its30-second read-only limit and is not a completed census. No all-repository completeness claim follows.

Returning the exact internally observed Promise did not eliminate Bun late-timeout reporting in the actual tiny counterexample. The closed registrar owns real runner completion and canonical failure instead; no global lifetime fix or completed rollout is claimed.

| Actual file | Direct | First-statement block | Lines |
| --- | ---: | ---: | --- |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts | 2 | 0 | 89, 121 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️framing/🟦️.ts | 1 | 0 | 10 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger.ts | 3 | 0 | 23, 77, 94 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts | 1 | 0 | 16 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts | 6 | 0 | 21, 34, 37, 47, 59, 62 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests/🟦️.ts | 1 | 0 | 18 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/🟦️.ts | 1 | 0 | 13 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts | 6 | 0 | 22, 36, 47, 60, 63, 71 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts | 2 | 0 | 32, 68 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/🟦️.ts | 3 | 0 | 20, 22, 30 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🟦️.ts | 1 | 0 | 17 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts | 2 | 0 | 26, 125 |
