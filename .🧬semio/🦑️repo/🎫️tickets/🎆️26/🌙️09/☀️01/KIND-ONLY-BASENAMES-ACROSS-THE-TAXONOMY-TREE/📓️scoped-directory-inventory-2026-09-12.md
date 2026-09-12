# Scoped Directory Inventory Probe

A read-only invocation of the existing `inventoryTaxonomy` API was scoped to the mathematical equation JSON generator. It completed successfully with phase progress and produced 17 entries. This shows that focused active-owner inventory can be used for the next directory-coverage audit without invoking the full repository normalization planner. It is an inventory run, not a claim that the generator passes taxonomy enforcement.

Scope: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine`.

The reported violations are:

```json
[
  {
    "code": "directory-kind-unresolved",
    "severity": "error",
    "path": "✏️s/🔌️plugins/➗️mathematical",
    "message": "Directory has no registered semantic kind"
  },
  {
    "code": "semantic-stem-unresolved",
    "severity": "error",
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/🏭️generate.rs",
    "message": "Semantic stem has no registered directory kind"
  },
  {
    "code": "taxonomy/kind-only-basename",
    "severity": "error",
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/🏭️generate.rs",
    "message": "Implementation leaf \"🏭️generate.rs\" must use \"🦀️.rs\"."
  },
  {
    "code": "taxonomy/kind-only-basename",
    "severity": "error",
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📖️reader.rs",
    "message": "Implementation leaf \"📖️reader.rs\" must use \"🦀️.rs\"."
  },
  {
    "code": "semantic-stem-unresolved",
    "severity": "error",
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📚️lib.rs",
    "message": "Semantic stem has no registered directory kind"
  },
  {
    "code": "taxonomy/kind-only-basename",
    "severity": "error",
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️json-engine/src/📚️lib.rs",
    "message": "Implementation leaf \"📚️lib.rs\" must use \"🦀️.rs\"."
  }
]
```

The next generator/source lane should re-run this kind of scoped inventory after its semantic owner/package extraction and inspect both directory contexts and active source references. The normalizer currently has a deliberate Rust generator-crate exception, so a clean directory record under that exception is not independently sufficient proof of implementation-neutral design.
