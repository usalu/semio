# Computed Fixture Read Audit

TypeScript AST inspection resolves literal filesystem reads and lexical constants composed with join, resolve and dirname. Dynamic paths, parameters and runtime computations remain outside this bounded audit. Missing paths are candidates for review, not proof that every branch executes.

```json
{
  "files": 94,
  "observed": 332,
  "missing": [
    {
      "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts",
      "line": 257,
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json",
      "expression": "join(directory, \"🔣️.json\")"
    },
    {
      "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🟦️.ts",
      "line": 366,
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🔣️.json",
      "expression": "join(directory, \"🔣️.json\")"
    },
    {
      "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🟦️.ts",
      "line": 154,
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json",
      "expression": "join(directory, \"🔣️.json\")"
    },
    {
      "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts",
      "line": 556,
      "path": "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs",
      "expression": "resolve(root, \"../../../../../../..\", \"🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs\")"
    },
    {
      "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts",
      "line": 44,
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json",
      "expression": "join(fixturesRoot, \"🚨️transaction-sentinel-cases/🔣️.json\")"
    }
  ]
}
```
