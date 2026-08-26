# Standard and Subset Parent Context

## Decision

`standard` is admitted only with parent kind `standards` or the exact `members-of-artifacts` registry owner; `subset` is admitted only with parent kind `subsets` or `standard`. This covers both canonical owned profiles (`🏅️standards/🔖️…/🪆️subsets/✳️…`) and the schema-frozen foreign dialect shape (`🗿️artifacts/<kind>/🔖️<std>/✳️<subset>`). The resolver no longer treats either broad slug grammar as a global semantic fallback. Exact kind-id slugs such as `🔖️standard` are subject to the same physical-parent authority and cannot bypass the contextual filter.

The projection authority continues to validate captured standard/subset names from their schema specs, while projected-member discovery now supplies the exact collection parent to its semantic resolver.

## Language-agnostic vectors

The retained JSON fixture `🧪️standard-subset-parent-context/🔣️.json` freezes eight positive and negative parent-context cases. A Bun runner also compares canonical collection discovery with `fast-glob` and inventories both a wrong-parent exact-slug fixture and the foreign dialect chain through the production normalizer.

## TDD evidence

The first run failed as intended: `🔖️1` beneath `subsets` resolved globally as `standard` instead of `null`. A first narrow correction then exposed an important counterexample in a 417-entry Writer subset inventory: foreign dialect standard/subset directories became unresolved. The final parent authority admits only the two schema-owned shapes and normalizes missing-VS16 registry members such as physical `📄txt` before resolving their children.

After the schema and resolver changes:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️standard-subset-parent-context.test.ts'
3 pass
0 fail
17 expect() calls
```

Direct shipped-schema validation returned `problems=[]`. The real Writer subset inventory resolved its owned standard, owned subset, foreign artifact member, foreign standard, and foreign subset with zero directory-kind violations. The canonical taxonomy JSON SHA-256 at this checkpoint is `6c26718c16f8e5e2a5bc5d23ba160df313214b1806f02bd93ba4596a94fce1b4`.

## Scope

This closes the schema-context mechanism responsible for the retained snapshot's 8,029 `standard`/`subset` stem ambiguities and 400 directory ambiguities. It does not claim those paths are all normalized: a fresh transaction-v2 inventory must distinguish correctly unresolved stems from paths settled by other exact authorities.
## Portable Rerun

The ticket test now resolves the repository from `import.meta.dir` and imports discovery/normalization through repository-relative paths; it no longer embeds the coordinator's macOS checkout path. Independent rerun:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️standard-subset-parent-context.test.ts'
3 pass
0 fail
17 expect() calls
3.48s
```
