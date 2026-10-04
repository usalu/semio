# Active Schema Source Closure Audit

First snapshot: production source/fixtures/tests are not present yet; contract, corpus schema and script/project/package exist. This is not implementation approval. No native commands or source edits. Re-audit actual code after it appears.

## Concrete Initial Contract And Integration Gaps

- Request `annotations` admits arbitrary nonempty strings, including `$ref`, `$id`, `$dynamicRef`, and every structural keyword. Implementation must refuse overlap with reserved standard structural/reference keys; otherwise caller annotations can silently hide reference obligations. Extension annotations may be allowlisted only as opaque data positions.
- Input baseUri is nonempty string only, so implementation must enforce absolute supported URI and fragment/base policy; request dialect must verify or explicitly govern schema `$schema` mismatch. No document `$id` is required by outer contract, meaning supplied base is real retrieval identity and needs indexed aliases when root declares another `$id`.
- Limits enforce numeric bounds in schema; runtime controls must also admit exact own data and refuse NaN/Infinity/fraction/negative values before loops. Total source budget counts all explicit inputs, not just reached closure. Chunk work must use byte/code-unit definition consistently and yield for asynchronous cancellation.
- Corpus expected projection only asserts document IDs and target four-tuples. Add assertions for unchanged source/id/path/baseUri, resource ownership, origin pointers/raw reference spellings, repeated edge count/order, root exclusion, and no accepted partial output on refusal.
- New project test target has no explicit inputs yet; follow existing validator subtree+sharedGlobals input convention. New launch entry is not present in current grep snapshot. Script import depth resolves neutral process module correctly; timeout 30000 is finite. Its tests currently do not exist; no RED/GREEN claim made.

## Dialect Positions To Gate Explicitly

Draft07: schema maps `definitions`, `properties`, `patternProperties`; single schema `additionalProperties`, `additionalItems`, `contains`, `propertyNames`, `not`, `if`, `then`, `else`; arrays `allOf`, `anyOf`, `oneOf`; `items` either schema or tuple array; `dependencies` either string array (data) or schema. `$defs` may be recognized as adopted extension with explicit policy. `prefixItems`, `dependentSchemas`, `unevaluated*`, `contentSchema` are 2020 positions, not automatically Draft07 schema children merely because values look like objects.

2020: maps `$defs`, properties, patternProperties, dependentSchemas; array prefixItems; items is schema only; singles unevaluatedItems/unevaluatedProperties/contentSchema join normal schema positions. Legacy tuple items should refuse under 2020. Unknown keyword policy may reject rather than ignore, but must not pretend both dialects have identical child grammar.

Both: skip const/default/enum/examples, required/dependentRequired string lists and annotation payloads. Process `$ref` siblings per chosen dialect policy, document it, and test before importing AJV expectations: installed AJV strict:false compiled/refused sibling constraints rather than fully ignoring siblings in the observed Draft07 cases.

Draft07 `$id:"#anchor"` identifies an anchored subschema; 2020 `$anchor` has grammar `^[A-Za-z_][-A-Za-z0-9._]*$` and `$id` permits no nonempty fragment. Decide support/refusal explicitly. Reject `$dynamicRef` and recursive equivalents until real dynamic resolution exists; do not whitelist them as annotations.

## Pointer, Scope And String Hazards

URI percent decoding must happen once before strict ~0/~1 pointer decoding and empty-segment preservation. Resolve pointer against exact nearest resource root; output target pointer remains physical-source coordinate. Resource alias identity and nested `$id` identity are distinct but must map to the same exact subschema owner. Non-schema annotation target refusal prevents const object `{"$ref":"x"}` becoming schema authority.

Raw text strings can contain unpaired surrogate code units. System JSON.parse accepts escaped `\ud800`, while TextEncoder turns that parsed key into U+FFFD bytes. Canonical member duplicate comparison must use decoded string code-unit identity; URI/pointer/sort/raw-source policy must refuse unpaired surrogates or define exact preservation, never silently pass through lossy byte encoding. `path`, `id`, baseUri and raw reference strings need the same admission policy if sorted/encoded as UTF-8.

## Executed Small Oracle Observations

Bun + installed AJV/AJV2020, strict:false, exit 0; no network fetch/native build. Exact command/stdout captured in generated/schema-closure-active-audit. These are third-party observations only; AJV stack overflow is a limitation of that specimen, not a proposed closure failure oracle.

```jsonl
{"id":"draft07-id-anchor","compiled":true,"booleanValid":true}
{"id":"draft07-ref-sibling-invalid","compiled":false,"error":"can't resolve reference missing from id https://case.test/root"}
{"id":"draft07-ref-sibling-value","compiled":true,"booleanValid":false}
{"id":"2020-ref-sibling-value","compiled":true,"booleanValid":false}
{"id":"2020-legacy-items","compiled":false,"error":"schema is invalid: data/items must be object,boolean"}
{"id":"draft07-prefixItems-data","compiled":true,"booleanValid":true}
{"id":"2020-const-ref-data","compiled":true,"booleanValid":false}
{"id":"nested-relative-pointer","compiled":false,"error":"Maximum call stack size exceeded."}
{"id":"2020-anchor-leading-digit","compiled":false,"error":"schema is invalid: data/$anchor must match pattern \"^[A-Za-z_][-A-Za-z0-9._]*$\""}
{"id":"2020-id-fragment","compiled":false,"error":"schema is invalid: data/$id must match pattern \"^[^#]*#?$\""}
{"id":"system-lone-surrogate","codeUnit":55296,"encoded":[239,191,189]}

```

The nested-relative pointer specimen caused AJV Maximum call stack size exceeded. Its source-closure outcome still must be finite and resolve the pointer within the nested child resource; do not use a compiler crash as an expected resolver refusal or copy AJV implementation behavior.

## Exact First Owner Snapshot

- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/📦️packages/🟦️typescript/package.json`: SHA-256 `82687a212fff65a18a23b422c095e9711b35b3abc61ce6885753b67904ac1a86` (204 bytes).
- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/📦️packages/🟦️typescript/📋️project.json`: SHA-256 `005f79a5b6ab81a312712c6719d9434ea3e3e7a42a453dde76a897876f92eff6` (431 bytes).
- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/📦️packages/🟦️typescript/📜️script.ts`: SHA-256 `479f9570f5a7472b6a0fa314a9e296a8ecbe46a3e8db11d1ce039316e494d03d` (851 bytes).
- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🔣️.json`: SHA-256 `f478ce87334b4e3b6c9454b0e80e0aa72a4886c3f3e224f2789dc5a8979b82e6` (3189 bytes).
- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🧫️fixtures/🔣️.json`: SHA-256 `2f65d7911180bf69a15a3cda3625d0a21ae3cbd4680c36bcf97be09a7cd5bd19` (24188 bytes).
- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🧬️schema/🔣️.json`: SHA-256 `e2f235c33b4baf85fedff01d118d015a6e6aeb7b395eb3dd05d9111f3ca53e49` (2302 bytes).

## Fixture Snapshot Follow-Up

36 fixture rows appeared. Independently exercised all 19 non-null AJV oracle rows. A naive compile(root) harness fails three base-relative fixtures because inputs intentionally have no `$id`; correct addSchema(root, root.baseUri) then getSchema(root.baseUri) makes every declared oracle compile/instance expectation agree. This is executed third-party evidence, not production GREEN. Preserve retrieval/base URI in oracle harness; never inject a fabricated `$id` into raw input to make tests pass.

Current fixture `id-fragment` deliberately refuses Draft07 nonempty `$id` fragment although actual Draft07 AJV supports anchored `$id`. Treat this as an explicitly unsupported subset choice, not standards equivalence. Additional valuable cases still missing: nested resource local pointer, retrieval URI alias versus declared root `$id`, reserved keyword in annotation allowlist, escaped/literal surrogate pair name equivalence, invalid base/path/id lone surrogate, and 2020/Draft07 cross-position keyword policy.

```jsonl
{"id":"boolean-root","compiled":true,"expected":true,"validations":[true],"expectedValid":[true],"error":null}
{"id":"empty-pointer-segments","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"escaped-pointer","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"percent-pointer","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"annotation-data","compiled":true,"expected":true,"validations":[true],"expectedValid":[true],"error":null}
{"id":"nested-relative-id","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"static-anchor","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"boolean-target","compiled":true,"expected":true,"validations":[false],"expectedValid":[false],"error":null}
{"id":"explicit-external","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"repeated-external","compiled":true,"expected":true,"validations":[true],"expectedValid":[true],"error":null}
{"id":"unused-input","compiled":true,"expected":true,"validations":[true],"expectedValid":[true],"error":null}
{"id":"utf8-document-order","compiled":true,"expected":true,"validations":[true],"expectedValid":[true],"error":null}
{"id":"declared-annotation","compiled":true,"expected":true,"validations":[true],"expectedValid":[true],"error":null}
{"id":"recursive-object","compiled":true,"expected":true,"validations":[true,false],"expectedValid":[true,false],"error":null}
{"id":"nested-id-collision","compiled":false,"expected":false,"validations":[],"expectedValid":[],"error":"reference \"https://case.test/same\" resolves to more than one schema"}
{"id":"unknown-reference","compiled":false,"expected":false,"validations":[],"expectedValid":[],"error":"can't resolve reference missing from id https://case.test/root"}
{"id":"unknown-pointer","compiled":false,"expected":false,"validations":[],"expectedValid":[],"error":"can't resolve reference #/missing from id https://case.test/root"}
{"id":"duplicate-root-external","compiled":false,"expected":false,"validations":[],"expectedValid":[],"error":"schema with key or id \"https://case.test/root\" already exists"}
{"id":"duplicate-anchor","compiled":false,"expected":false,"validations":[],"expectedValid":[],"error":"reference \"https://case.test/root#same\" resolves to more than one schema"}

```

## Tests Appeared, Implementation Still Pending

Actual new tests now use correct retrieval-base keyed AJV oracle, assert exact raw input preservation, repeated origin count, accessor source refusal, sparse resource refusal, path aliases, budget classes, asynchronous scheduled cancellation and monotonic progress. These are substantive additions; they are not a pass claim. No production closure source is present at this second snapshot.

Still add a temporal capture law: after operation begins and reaches first yield, mutate original request/root/resources/limits and ensure admitted owned operation continues from the exact upfront copied inputs/limits or refuses deterministically. Otherwise valid own records can still become mutable caller aliases between awaited chunks. Test getter/proxy controls separately: `cancelled` and callbacks are deliberately executable controls, while limits/request descriptor inspection must not execute arbitrary accessors.

The no-product-import static test bans 🛍️products/🖥️s/🌎️hub strings but does not catch every indirect runtime or old scanner forwarder. Verify the exact canonical member-provider import after production appears. Raw source index targets alone do not certify schema dialect child grammar; include explicit dependentSchemas/dependencies/prefixItems/item-tuple cross-dialect rows before endorsing closure.

## Production Review, Independent Owned Calls

Actual code now exists and imports only canonical owned JSON syntax decoder and member policy. Schema public types are owned/string/source-coordinate shapes; no foreign runtime identity or numeric instance-data API leaks were found in these two source files. Custom annotation allowlist now refuses reserved-key collisions. Request/input/limits are descriptor-copied before first awaited parse; source strings immutable, result input records/arrays/edges/resources frozen. Parsing rejects duplicate decoded names before any numeric coercion and preserves number spelling. Schema-aware maps/arrays/singles are dialect-gated; unknown positions refuse; dynamic/recursive keywords refuse explicitly. These are source findings, not whole-task completion.

### Remaining Control Blockers In This Snapshot

1. Capture `source: text(row.source, null)` still calls full synchronous unicode scan before JSON decoder byte budget/progress/yield, as Root already noticed. Metadata id/path/baseUri/custom strings also scan synchronously without metadata-byte budget; dense arrays/descriptors and supplied.map(capture) can process huge admitted input sets before first emit/yield. Fix source scan at bounded canonical decode; admit metadata/input-list work under explicit bounded operation accounting before allocating/copying huge frontiers.
2. Schema node field/member map creation, members unicode scan, child-map and array forEach loops, dependencies item.some, reverse frontier push, reference fragment split/segment loop, outgoing edge grouping, and final native sort/comparator UTF-8 encode are synchronous size-proportional work between schema checkpoints. A single wide properties/allOf or long reference/ID can exceed requested chunk work. Budget schemas=1 still builds the whole parent child frontier before refusing child 2. Add bounded child/member/reference-character/frontier accounting and cancellation yields; precompute bounded UTF-8 sort keys and use a controlled sort where needed. Avoid describing chunk as hard work bound until these paths are bounded.
3. Owned JSON decoder `#string` appends one element per decoded code unit then parts.join synchronously, and #number slices potentially large lexemes on completion. Byte scanning yields, but final decoded string materialization is a size-proportional step outside chunk accounting. Explicit document budget bounds total size; it does not bound final callback latency. Assess against expensive-operation cancellation requirement and fix retained/bounded construction if strict per-step work is claimed.

Wide 2000-child owned call with schemas=1/chunk=1 refused schema-budget, and only two schema-phase yields occurred; exact source confirms complete child frontier was materialized between them. This command does not claim wall-time failure; the blocker is unaccounted work versus declared chunk controls.

### Executed Small Reproduction Evidence

```jsonl
{"id":"retrieval-anchor-alias","owned":{"accepted":false,"code":"unknown-anchor"},"oracle":{"compiled":false,"error":"can't resolve reference https://case.test/retrieval#flag from id https://case.test/declared"}}
{"id":"declared-anchor","owned":{"accepted":true,"targets":["/$defs/x"]},"oracle":{"compiled":true,"valid":true}}
{"id":"2020-definitions","owned":{"accepted":false,"code":"unknown-keyword"},"oracle":{"compiled":true,"valid":true}}
{"id":"empty-root-id","owned":{"accepted":true,"targets":[]},"oracle":{"compiled":true,"valid":true}}
{"id":"cancel-large-text","code":"cancelled","yields":1}

```

```jsonl
{"id":"wide-schema-budget","code":"schema-budget","postYields":2,"last":[{"phase":"parse","schemas":0,"inputs":0},{"phase":"schema","schemas":0,"inputs":1},{"phase":"schema","schemas":1,"inputs":1}]}

```

The cancellation specimen interrupted a large string at first decoder yield (owned code cancelled, yields=1), confirming parser chunk cooperation for that path. Temporal capture source code currently copies records before awaits; no mutable input alias was reproduced.

AJV supports 2020 deprecated `definitions`, while owned closure refuses unknown-keyword. Draft07 anchored `$id` also explicitly refused by owned policy. These are supported-subset differences to state and fixture, not automatically runtime bugs or grounds for legacy support. Retrieval-anchor alias lookup changed concurrently from resolved URI to resource.uri during this audit; re-run that specimen against final code before interpreting its earlier result. AJV keyed alias specimen itself refused retrieval#flag when root declared a different `$id`, so use explicit alias policy rather than assert universal AJV parity.

### Exact Production Snapshot

- `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🟦️.ts` SHA-256 `34f711afa77b824526b3f31e4f0d926f0bc7d20ade96b13ff7aa69b40c0e15df` (21598 bytes).
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🟦️.ts` SHA-256 `0811e427a1be43b639fba8595dc826948c499955b8f37b6e83c4f76b6b1a82ed` (9369 bytes).

## Concurrent Fix And Temporal Capture Check

Latest source capture changed to primitive nonempty string type check and preserves `source: row.source` without pre-decoder unicode scan; the known raw-source full-scan blocker is fixed in latest read. Other metadata/child/frontier/sort work noted above still appears synchronous in latest source.

Executed small temporal owned call mutates original root source/id, resources, annotations, and limits at first yielded parse boundary. Operation retained captured original raw source/id, zero original external documents, and frozen output/root. Exit 0. This confirms record/limits capture for this concrete mutation schedule, not arbitrary Proxy resistance.

```jsonl
{"id":"temporal-capture","capturedSource":true,"capturedId":"r","documents":0,"frozenRoot":true,"frozenResult":true}

```
