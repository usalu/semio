# Bun Exact Directory Branch

Read-only pinned Bun 1.3.14 primary Source review; no installation executed.

WorkspaceMap.processNamesArray lines 131–141 skips empty/dot roots, calls glob.detectGlobSyntax(input_path), then otherwise joins the original input string directly to package.json. That exact-directory branch does not decode escapes.

The proposed raw-brackets/braces fallback is unsafe: src/glob/glob.zig lines 11–37 detects unescaped *, {, [, ? and leading !, not only * and ?. Escaping every special operator can therefore disable glob detection and preserve the backslashes as a filesystem path; publishing raw [ or { enables glob parsing instead of literal matching. This explains the observed escaped source\[1\]/kernel refusal without proving a raw fallback correct. A concrete literal containing these characters needs an explicit genuinely glob-parsed encoding with exact one-directory membership, tested against actual Bun; do not infer correctness from fast-glob alone.

Leading ./! makes ! noninitial and therefore avoids negation detection when no other operator exists. Leading ! alone selects glob/negation. ./ does not change bracket/brace detection. The existing source-directory native oracle must independently model the exact-directory branch only when detectGlobSyntax is false, and actual Bun must test literal brackets, braces, *, ?, leading !, leading ./!, Unicode, external ../ and combinations. Preserve complete package membership and nested collection refusal.

Primary files: https://raw.githubusercontent.com/oven-sh/bun/bun-v1.3.14/src/install/lockfile/Package/WorkspaceMap.zig and https://raw.githubusercontent.com/oven-sh/bun/bun-v1.3.14/src/glob/glob.zig . No current production implementation authorship claimed.
