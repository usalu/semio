# Normalization Runtime Catalog Assets

Independent audit traced two more library JSON files to synchronous Clean plan, validation and apply reads. They are current normalization authority catalogs and now live under assets, retaining exact bytes. Constants and serialized sentinel authority coordinates now call them catalogs. The sentinel authority uses catalogPath/catalogContentHash consistently; no legacy field fallback was added. Test reads follow the same current paths. Runtime verification follows below.

## Preserved Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🚨️transaction-sentinel-cases/🔣️.json",
    "sha256": "c67b436abca4b9005efce6023f74bd900de5e88187ee9eb3c4328c2397bd081b"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/💉️ticket-important-exact-mutations/🔣️.json",
    "sha256": "a5012fa365145d27001fdd21d87e072f2b7a0d7992d017b7753390febf24a195"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🖼️normalization-catalogs/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/💉️ticket-important-exact-mutations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🚨️transaction-sentinel-cases/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"
]
```

## Runtime Verification

The focused canonical key-contract and virtual-sentinel consumer tests passed: 8 tests, 66 assertions. The first run exposed a stale repository-shaped symlink string introduced by the test relocation. The virtual target and its expectation now match the unchanged language-neutral protocol example, `../file.txt`.

The retained `🧑‍💻coordination/🧪️normalization-catalogs/📜️script.ts` executes the actual production inventory and planner against the actual repository. A filesystem module wrapper observes reads while delegating the original filesystem function and preserving bytes; its observations are reset after inventory. The planner then reads both catalogs from their current asset locations. This bounded read-boundary proof passed with 16 inventory entries and runtime `[DEBUG]` evidence.

The broader planner subsequently rejects an existing compiler input manifest because its rows are not path-sorted. That failure is recorded separately and is not treated as a successful full normalization plan. No apply operation ran. Initial attempts to observe Bun filesystem imports using only a property spy did not observe the imported function; the retained proof uses an explicit module wrapper.
