# Staged route gate tests

Contract: `Requires = none | dependencies | staged`, command `routes`, gate `(command, segments, invocation)`, lazy loader `(invocation, segments)`.

## Changed
- registry tests `🗂️registry/🧪️tests/🟦️.ts` and `🔣️.json`: `staged` in expectedRequires, `expectedRoutes`, `routeRequirements` rows (longest prefix, fallback, empty segments), gate rows (staged+available not build-owned -> no provisioning, staged+absent -> provisions, non-matching/partial prefix -> command requirement, longest route wins, none route), invalid registries (duplicate route refused by admission, bad route requires, empty segments, missing `requiresStaged` label), route help lines, route reasons in every language, router passes segments to the gate.
- routing tests: lazy loader receives the segments after the command name and loads once.
- zero-touch law `🏗️build/📥️source/🛗️zero-touch/🟦️.ts`: `staged` accepted and treated like `dependencies` (external packages allowed, closure followed); test row added.

## Results
- `repo test-file` registry: 68 pass, 0 fail. Routing: 49 pass. Zero-touch: 34 pass, 3 fail (real-checkout closure laws, unrelated: workspace specifiers in `caching/.../📤️publication` and `.mjs` seam-static; not touched by this change).
- Typecheck (`repo run @semio-tech/repo-lib:typecheck`) did not finish within 580 s (graph slice still reading); unverified.
