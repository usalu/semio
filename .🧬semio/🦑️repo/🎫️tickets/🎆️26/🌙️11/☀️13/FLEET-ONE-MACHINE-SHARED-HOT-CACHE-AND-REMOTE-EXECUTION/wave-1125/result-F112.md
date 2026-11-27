# F112: `schema twins generate` cannot remedy `schema-export-incomplete`; twins gate added

Lane F112, 2026-11-26. Outcome: the premise of F110 class (a) is wrong. **0 twin files change** when the generator runs over the whole repo, so nothing was written, and `schema-export-incomplete` stays at **4489 before and after** (re-dump with `T/🧰️scripts/F110/dump-schema-findings.ts` is identical because no twin changed). The audit note "regenerate twins" ("Not verified by running") does not hold.

## Dry run (generator's own `planTwins`, no `repo run`)
`T/🧰️scripts/F112/twins-plan.ts <out.jsonl> <scratchDir> [scope]` calls `planTwins(<repo root>, [typescript, rust, protobuf, graphql])`, compares every plan with the committed file through the generator's `compare`, and writes any drift to scratch. Result over the whole repo: `plans=5684 changed=0` (31 s). The `json` facet (schema layout) was excluded on purpose: it rewrites schema documents, which F108 owns. `repo ./📜️script.ts schema twins check --under .` semantics are identical (the new gate below runs it with `--strict`, 1 pass).
No hand-written code was at risk and none was touched (no file changed; the Rust splice only replaces annotated structs and refuses foreign-banner files).

## Why regeneration cannot fix the 4489 findings
The generator only projects families that declare a `🧬️twin-dialect/🔣️.json` (`dialectOf` in `🧬️twin-projection/🧭️dialect`), and only the norm plugin (`✏️s/🔌️plugins/📕️norm`) does (din16798 has `projection: off`). Tally of the findings by whether the generator can produce the file:

| Cause | Findings |
|---|---:|
| Family has no twin dialect: twins are hand-written, the generator ignores them (rs 2143, graphql 463, ts 455, proto 438, wit 9) | 3508 |
| Dialect present but `projection: off` (rs 118, graphql 3, proto 3, ts 1) | 125 |
| Norm family, Rust twin: generator emits Rust only for structs annotated `x-semio-rust.derive` (leaf payloads and annotated records), so un-annotated exports (e.g. `En1990AccidentalDelta` in `🔺️diff`) are never emitted | 856 (712 in a planned twin, 144 in an unplanned one) |
| Total | 4489 |

Norm TypeScript twins are complete (no ts finding sits under an enabled dialect). The 4253 "real absence" figure of F110 therefore means: 3633 hand-written twins whose generator does not exist, plus 856 norm Rust exports with no annotation.

## What would actually remedy them (decisions, not mechanics)
1. Norm Rust (856): add `x-semio-rust` annotations (`schema twins annotate` harvests them from committed structs) or declare restricted support with `x-semio-formats`; then `twins generate`.
2. Other families (3633): either give each family a `🧬️twin-dialect` (engine knows no domain; the TypeScript/protobuf/GraphQL renderers are generic but the dialect names a reader contract module that those families lack) and generate, or annotate the exports with `x-semio-formats` to declare that the twin is restricted. Both are design decisions per family.
3. The checker bugs of F110 (215 Rust macro, 9 wit) belong to F111.

## Why twins went stale / is there a gate
The twins were not stale; the checker's expectation exceeds what the generator is specified to emit. A gate for the generator already existed only for norm TypeScript (`📕️norm/🧪️tests/🧪️wire-twins/🧬️drift`). No gate covered Rust/protobuf/GraphQL or any other family.

## Added gate
- Test `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🧪️tests/🧮️twins-freshness/🟦️.ts`: `runTwins(root, [check, --under ., --strict])`, problems/drift/style all empty. 1 pass, 24 s (limit 300 s).
- New suite `schema-twins-fresh` (budget 300000, all three platforms) in `⚡️caching/🗂️suites/🔣️.json`, target `test-schema-twins-fresh` (inputs `schemaSources`, which already covers every `🧬️schema/**`) and its entry in the aggregate list of `⚡️caching/📋️project.json`, landed together in one atomic python patch (`T/🧰️scripts/F112/patch-suite-and-target.py`, temp+rename) under lock `landing.lock.caching` (held for the two renames only, released).
- Verified: route table law `⚡️caching/🗂️suites/🧪️tests/🟦️.ts` 5 pass / 0 fail; `repo typecheck` of the new test passed; dangling-imports exit 0.

## Rust twins for the post-seal cargo check
None changed, so no crate needs a cargo check from this lane.

## Files
- Created: `🧬️twin-projection/🧪️tests/🧮️twins-freshness/🟦️.ts`, `T/🧰️scripts/F112/twins-plan.ts`, `T/🧰️scripts/F112/patch-suite-and-target.py`.
- Edited: `⚡️caching/🗂️suites/🔣️.json`, `⚡️caching/📋️project.json`.
- Scratch (outside the ticket): `<scratchpad>/F112/` (before.jsonl, an.ts).
