# Canonical Component Contract Review

Current inventory: `🗑️generated/bazel-component-contract-edits.json`. All eleven consolidated source documents and two relocated test documents are absent at their retired paths; canonical destination definitions exist. Independent recursive JSON-pointer traversal resolves all local references in all three component roots. Fixed retains one intentional external reference to the existing platform mount contract; this is not a dangling retired component ID. A framework-wide literal scan of TypeScript/JSON/MJS/MTS finds zero occurrences of the retired Fixed command and observation IDs supplied by the owner.

Current consumers select the canonical component definition explicitly. Command requests contain no relocated local refs; the cleanup open-lifetime and preparation-observation runtime leaves read protocol constants without independently compiling a detached referenced case schema. Schema-validator consumers must retain the canonical full root wherever their selected definitions use newly prefixed pointers. Existing Source equality/round-trip laws remain required; this read does not substitute for the owner’s pending all-consumer integration gates or claim unchanged constraints solely from successful pointer resolution.

No new concrete Source blocker found in this bounded pass. The inventory is still being extended for test/environment readers, so this is incremental rather than a final all-consumer hold. No runtime, store, Native filesystem, backend census or SDK was executed.

```json
[
  {
    "definition": "ActionBirth",
    "removedAbsent": true,
    "refs": [
      "#/definitions/ActionBirth/properties/parameters"
    ]
  },
  {
    "definition": "OutputSnapshot",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "ModuleSetup",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "ImplementationPolicy",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "Planner",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "Implementation",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "Command",
    "removedAbsent": true,
    "refs": [
      "#/definitions/Command/definitions/Request"
    ]
  },
  {
    "definition": "PreparationReceipt",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "MaterializedPaths",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "PreparationObservation",
    "removedAbsent": true,
    "refs": []
  },
  {
    "definition": "OpenLifetime",
    "removedAbsent": true,
    "refs": [
      "#/definitions/OpenLifetime/definitions/Case"
    ]
  }
]
```

Final12-definition corpus check: independent Python reconstruction of recursively sorted semantic keywords and de-prefixed local references reproduces all12 authored normalized SHA values. All12 corpus-retired paths are absent; two relocated test contracts were separately absent at their old paths. This confirms equality to the retained authored pre-move semantic hashes, not independent historical content recovery. The generated inventory has a concrete malformed StorageLayout removed field (code text instead of a path); producer notified, permanent corpus has the correct retired path.

```json
[
  {
    "definition": "ActionBirth",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "OutputSnapshot",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "ModuleSetup",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "ImplementationPolicy",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "Planner",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "Implementation",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "Command",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "PreparationReceipt",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "MaterializedPaths",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "PreparationObservation",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "OpenLifetime",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  },
  {
    "definition": "StorageLayout",
    "normalizedShaMatches": true,
    "retiredAbsent": true
  }
]
```
