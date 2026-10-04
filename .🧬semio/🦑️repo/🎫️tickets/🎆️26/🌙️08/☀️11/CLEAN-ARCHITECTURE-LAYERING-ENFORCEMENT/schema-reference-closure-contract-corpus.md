# Independent Schema Reference Closure Contract And Corpus

No production changes. Inputs are provided strings with opaque owner coordinates; this report makes no filesystem presence, symlink, freshness, containment, or persistence claims about those coordinates. Closure means structural schema resources, not referenced instance-data numeric projection.

## Precise Proposed Policy

JSON draft only. The producer must supply exact captured raw text, not reconstructed JSON.stringify output. Preserve `$ref` raw spellings and original source identity; private structural parsing after duplicate refusal is allowed. Reject invalid documents across the explicitly supplied set before producing accepted closure. All origin occurrences are retained independently of unique physical document reachability.

```json
{
  "contractId": "schema-source-closure-v1",
  "input": {
    "root": "exact one OwnedSchemaInput",
    "resources": "explicit finite OwnedSchemaInput array",
    "OwnedSchemaInput": {
      "id": "unique opaque owner identity",
      "path": "opaque exact source coordinate; no filesystem assertion",
      "source": "immutable raw JSON text"
    }
  },
  "output": {
    "root": "same identity/path/raw source",
    "documents": "unique referenced physical inputs excluding root, sorted by UTF-8 id then UTF-8 path",
    "edges": "every reference occurrence: origin input id/path, schema pointer, raw reference, resolved resource URI, target input id/path, target schema pointer",
    "resourceIndex": "root and nested resolved $id plus supported anchors mapped to exact input/schema pointer"
  },
  "memberPolicy": "Reject mandatory before platform JSON.parse",
  "numericPolicy": "no public parsed numeric instance API; retain raw source and parse privately for structural schema discovery",
  "uriPolicy": {
    "rootBase": "require absolute root $id OR explicit absolute base URI per input; id/path alone are not URI bases",
    "nestedIds": "resolve relative $id against nearest schema resource scope",
    "emptyFragment": "whole resource",
    "pointer": "URI percent decode once, then JSON Pointer segment split preserving empty segments and strict ~0/~1 decode",
    "anchors": "explicit static $anchor supported only in declared dialect; unique per resolved resource URI",
    "external": "resolve only into explicit registered resource set; no network or filesystem discovery",
    "dynamic": "reject $dynamicRef/$dynamicAnchor/$recursiveRef/$recursiveAnchor with owned unsupported diagnostic until actual scoped dynamic semantics are implemented"
  },
  "control": {
    "required": true,
    "maximumSourceBytes": "total and per input",
    "maximumSchemas": "schema nodes",
    "maximumReferences": "origin edges",
    "maximumResources": "root+nested resource identities",
    "maximumDepth": "schema nesting",
    "cancelled": "checkpoint before work and each bounded chunk",
    "progress": "monotonic consumed bytes, schemas visited, references resolved, physical inputs admitted",
    "yield": "bounded chunks yield event loop so asynchronous cancellation becomes observable",
    "failure": "no accepted partial closure; deterministic owned diagnostic with exact origin"
  },
  "dataKeywordsSkipped": [
    "const",
    "enum",
    "default",
    "examples"
  ],
  "schemaChildren": {
    "map": [
      "$defs",
      "definitions",
      "properties",
      "patternProperties",
      "dependentSchemas"
    ],
    "single": [
      "additionalProperties",
      "additionalItems",
      "contains",
      "propertyNames",
      "not",
      "if",
      "then",
      "else",
      "unevaluatedProperties",
      "unevaluatedItems",
      "contentSchema"
    ],
    "array": [
      "allOf",
      "anyOf",
      "oneOf",
      "prefixItems"
    ],
    "shapeDependent": [
      "items: schema or legacy schema array by dialect",
      "dependencies: schema values only, skip string arrays"
    ]
  },
  "uniqueness": "all resource URIs root/nested across every supplied physical input, anchors within resource; reject conflict even unused; never overwrite",
  "cycles": "capture finite visited set by physical input; retain repeated and cyclic origin edges; do not recursively expand reference targets without visited identities"
}
```

Relative URI bases require an explicit policy: opaque owner `id` and `path` are not implicit absolute URIs. Use actual system URL behavior for URI resolution and a proper fragment policy, then owned pointer traversal with own-key lookup only; no inherited-property or getter accesses from arbitrary user objects. A nested resource pointer is recorded relative to its physical raw source and distinguished from a pointer relative to its `$id` resource root. A resolved reference to a boolean subschema is legal. A pointer to annotation/instance data must not be treated as an executable schema merely because it exists.

Sorted closure is unique by physical input identity, excludes root even if a cyclic edge targets it, and uses explicit UTF-8 byte lexicographic ordering rather than localeCompare. Discover/index all supplied schema scopes before resolving references so order does not make forward references fail. Keep root and sibling nested resource IDs in one conflict-detecting index. Anchor uniqueness is per resource URI, so equal anchor names in different nested resources are allowed. Unknown schema vocabularies/reference-bearing keywords must be refused or explicitly supported; silently walking arbitrary object keys can mistake enum/const data for schema references.

## Proposed Valid Cases

```json
[
  {
    "id": "empty-pointer-segments",
    "root": {
      "id": "root",
      "path": "authored/root.json",
      "source": "{\"$id\":\"https://case.test/root\",\"$defs\":{\"\":{\"$defs\":{\"\":{\"type\":\"boolean\"}}}},\"$ref\":\"#/$defs//$defs/\"}"
    },
    "resources": [],
    "expected": {
      "documents": [],
      "edgeTargetPointer": "/$defs//$defs/"
    }
  },
  {
    "id": "annotation-is-data",
    "root": {
      "id": "root",
      "path": "authored/root.json",
      "source": "{\"$id\":\"https://case.test/root\",\"const\":{\"$ref\":\"missing\"},\"default\":{\"$id\":\"https://case.test/fake\"},\"enum\":[{\"$ref\":\"missing2\"}],\"examples\":[{\"$anchor\":\"fake\"}]}"
    },
    "resources": [],
    "expected": {
      "documents": [],
      "edges": [],
      "resources": [
        "https://case.test/root"
      ]
    }
  },
  {
    "id": "cyclic-repeated-external",
    "root": {
      "id": "root",
      "path": "authored/root.json",
      "source": "{\"$id\":\"https://case.test/root\",\"allOf\":[{\"$ref\":\"a\"},{\"$ref\":\"a\"}]}"
    },
    "resources": [
      {
        "id": "A",
        "path": "authored/a.json",
        "source": "{\"$id\":\"https://case.test/a\",\"$ref\":\"b\"}"
      },
      {
        "id": "B",
        "path": "authored/b.json",
        "source": "{\"$id\":\"https://case.test/b\",\"$ref\":\"root\"}"
      }
    ],
    "expected": {
      "documents": [
        "A",
        "B"
      ],
      "edgeCount": 4,
      "rootExcluded": true
    }
  },
  {
    "id": "nested-relative-anchor",
    "root": {
      "id": "root",
      "path": "authored/root.json",
      "source": "{\"$id\":\"https://case.test/base/root\",\"$defs\":{\"child\":{\"$id\":\"child\",\"$anchor\":\"flag\",\"type\":\"boolean\"}},\"$ref\":\"child#flag\"}"
    },
    "resources": [],
    "expected": {
      "documents": [],
      "targetResource": "https://case.test/base/child",
      "targetPointer": "/$defs/child"
    }
  }
]
```

## Proposed Invalid Cases

Diagnostic IDs are owned draft names to settle in the schema contract. Do not couple tests to AJV message wording.

```json
[
  {
    "id": "escaped-duplicate",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"a\":1,\"\\u0061\":2}",
    "error": "duplicate-member"
  },
  {
    "id": "nested-id-collision",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$defs\":{\"a\":{\"$id\":\"same\"},\"b\":{\"$id\":\"same\"}}}",
    "error": "duplicate-resource-id"
  },
  {
    "id": "root-external-collision",
    "rootSource": "{\"$id\":\"https://case.test/root\"}",
    "resourceSources": [
      "{\"$id\":\"https://case.test/root\"}"
    ],
    "error": "duplicate-resource-id"
  },
  {
    "id": "anchor-collision",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$defs\":{\"a\":{\"$anchor\":\"same\"},\"b\":{\"$anchor\":\"same\"}}}",
    "error": "duplicate-anchor"
  },
  {
    "id": "unknown-reference",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$ref\":\"missing\"}",
    "error": "unknown-resource"
  },
  {
    "id": "unknown-pointer",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$ref\":\"#/missing\"}",
    "error": "unknown-schema-pointer"
  },
  {
    "id": "bad-percent",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$ref\":\"#/%ZZ\"}",
    "error": "malformed-reference-fragment"
  },
  {
    "id": "bad-tilde",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$ref\":\"#/a~2b\"}",
    "error": "malformed-json-pointer"
  },
  {
    "id": "malformed-json",
    "rootSource": "{\"$id\":\"https://case.test/root\",}",
    "error": "malformed-json"
  },
  {
    "id": "dynamic-ref-explicit-refusal",
    "rootSource": "{\"$id\":\"https://case.test/root\",\"$dynamicRef\":\"#node\"}",
    "error": "unsupported-dynamic-reference"
  },
  {
    "id": "unknown-input-identity",
    "rootSource": "{\"$id\":\"https://case.test/root\"}",
    "inputsWithRepeatedId": [
      "A",
      "A"
    ],
    "error": "duplicate-input-identity"
  },
  {
    "id": "cancel-before-work",
    "cancelled": true,
    "error": "cancelled",
    "expectedAcceptedPartial": false
  },
  {
    "id": "byte-budget",
    "maximumSourceBytes": 8,
    "rootSource": "{\"$id\":\"https://case.test/root\"}",
    "error": "source-byte-budget",
    "expectedAcceptedPartial": false
  }
]
```

Add source-specific UTF-8 cases using exact byte arrays (invalid UTF-8 cannot be represented as raw JSON text strings): isolated 0x80, overlong 0xC0 0xAF, truncated 0xE2 0x82, surrogate UTF-8 0xED 0xA0 0x80. Define separate policy for valid JSON escaped lone surrogates; system JSON.parse accepts them and UTF-8 reencoding replaces unpaired surrogate code units. Reference/name identity must not be silently normalized by that replacement. Valid escaped-name duplicate fixtures should include `\u0061`, `a\/b`, and escaped surrogate-pair equality against literal emoji. Sorting vectors should include `a`, `z`, `ä`, `😀`; byte ordering must remain identical on all hosts.

Cancellation during chunk parsing/resource indexing/reference resolution and byte/schema/reference/resource/depth budgets require independent laws with no accepted partial closure. Progress should count input-byte admission once per physical document; repeated edges still count separately toward reference budget. Yield cadence is bounded and cancellation test must fire on a later event-loop turn, not only pre-set cancelled state.

## Actual Third-Party Oracle Evidence

Executed small Bun readonly command with installed AJV 8.20.0 (Draft07 and `ajv/dist/2020.js`), strict:false, no network fetching; explicit resources supplied via addSchema. Exit 0. Captured exact command and stdout under ticket generated/schema-reference-oracle. These are validation/compiler behavior observations, not execution of the proposed source-closure implementation. AJV cannot certify text duplicate refusal after platform parsing; that law needs a lexical/member oracle separately. AJV supports dynamic references; proposed source authority deliberately refuses them until it implements matching semantics.

```jsonl
{"id":"empty-segments","compiled":true,"valid":true,"errors":[]}
{"id":"escaped-pointer","compiled":true,"valid":true,"errors":[]}
{"id":"percent-pointer","compiled":true,"valid":true,"errors":[]}
{"id":"nested-relative-id","compiled":true,"valid":true,"errors":[]}
{"id":"anchor","compiled":true,"valid":true,"errors":[]}
{"id":"explicit-external","compiled":true,"valid":true,"errors":[]}
{"id":"unknown-reference","compiled":false,"error":"can't resolve reference missing from id https://case.test/root"}
{"id":"duplicate-root-external","compiled":false,"error":"schema with key or id \"https://case.test/root\" already exists"}
{"id":"duplicate-nested-id","compiled":false,"error":"reference \"https://case.test/same\" resolves to more than one schema"}
{"id":"annotation-ref-data","compiled":true,"valid":true,"errors":[]}
{"id":"repeated-external","compiled":true,"valid":true,"errors":[]}
{"id":"recursive-shape","compiled":true,"valid":true,"errors":[]}
{"id":"dynamic-reference","compiled":true,"valid":true,"errors":[]}

```

AJV confirms empty pointer segments, percent-decoded pointer names, tilde escapes, relative nested IDs, static anchors, explicit external resources, recursive object schemas, and repeated references. It rejects unknown references, root/external ID collision, and conflicting nested IDs. Annotation data containing `$ref` does not create reference obligations. Pure A→B→A resource capture is tested by the closure corpus conceptually; the executed oracle used a finite recursive object schema to avoid confusing validator stack behavior with source-closure termination.

## Oracle Receipt

- `oracle-input.txt`: 2355 bytes, SHA-256 `c3866daf374642986c9a0352b74c1742d3fe5d59f5f653c663a1779a755f1673`.
- `oracle-output.txt`: 1030 bytes, SHA-256 `3c99e5eb7ee6a5a55cbdf75eeb04d30f813c86fa2ded58707d19a5873564e459`.
- `oracle-stderr.txt`: 0 bytes, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
