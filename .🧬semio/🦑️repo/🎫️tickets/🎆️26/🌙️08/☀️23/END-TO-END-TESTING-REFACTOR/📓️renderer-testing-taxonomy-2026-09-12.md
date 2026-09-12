
## baseline Runtime

```json
{
  "command": "baseline",
  "suite": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts",
  "counts": {
    "passed": 3,
    "failed": 0,
    "skipped": 0
  },
  "files": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts",
      "state": "pass"
    }
  ],
  "errors": []
}
```

## Exact Authored Changes

The intake ceiling suite now has a direct TypeScript implementation under its semantic intake owner. Vitest names it in the normal case list and no longer uses an inline source selector for this module.

```json
{
  "updated": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts"
  ],
  "created": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🧪️tests/📏️step-ceiling/🟦️.ts"
  ],
  "beforeSha256": "5f0cd94ccd6d8d7aa24e57c871f3ca8f864d4b6a30ead55b9362d6fac8d18599"
}
```

## verify Runtime

```json
{
  "command": "verify",
  "suite": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🧪️tests/📏️step-ceiling/🟦️.ts",
  "counts": {
    "passed": 3,
    "failed": 0,
    "skipped": 0
  },
  "files": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🧪️tests/📏️step-ceiling/🟦️.ts",
      "state": "pass"
    }
  ],
  "errors": []
}
```

## corpus Runtime

```json
{
  "command": "corpus",
  "suite": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx",
  "counts": {
    "passed": 0,
    "failed": 0,
    "skipped": 0
  },
  "files": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx",
      "state": "fail"
    }
  ],
  "errors": []
}
```

## Corpus Runner Limitation

The real Interpreter Vitest source bridge failed during module import before registering tests: `TypeError: themeColorVar is not a function` at UI Scene `🟦️.tsx:630`, imported through the UI React barrel. The conformance corpus therefore has no renderer pass claim. The relocated intake suite passed 3/3 before and after; the corpus separately passed its 62-case Ajv catalog and all six native Rust consumer tests. The unrelated Scene import graph was preserved.
