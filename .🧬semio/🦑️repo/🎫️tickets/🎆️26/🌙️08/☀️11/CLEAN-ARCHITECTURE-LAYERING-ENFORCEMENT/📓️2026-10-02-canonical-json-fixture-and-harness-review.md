# Canonical JSON Fixture and Harness Review

Read-only source/package inspection; no jobs. Provider intentionally absent for TDD at dispatch.

Existing17 JSON goldens correctly reflect source semantics: multi-field identity priority, alphabetic identity prefix ordering across single-field rows, stable ties, UTF16 astral-before-E000 object keys, UTF8 E000-before-astral row identities, numeric index2/10 before nonindex01, NUL string escaping, mixed/nested array preservation. Five runtime rows match undefined object omission/array null and explicit top unsupported rejection.

## Actual independent-parser contradiction

Installed jsonc-parser/lib/umd/impl/parser.js:146–163 creates object={} and assigns currentParent[currentProperty]=value. It therefore loses parsed own __proto__ property via prototype setter. New test `🧪️tests/🧾️canonical-json/🟦️.ts` compares parse(bytes) to native JSON.parse corpus and later canonicalJson(parse(row.canonical)) idempotency for proto-key. Both can fail independently of owned null-prototype fix. Keep jsonc-parser syntax/errors/visitor validation, but use native JSON.parse for semantic value and idempotency, or a separately owned safe parser reference. getNodeValue must be inspected before assuming it is safe too. jsonc modify/applyEdits root replacement uses JSON.stringify(value) (impl/edit.js:49), so independently prepared golden arrays/object order serialize correctly including native-parsed proto property.

That root-replacement oracle is a real third-party entrypoint, but mainly wraps system JSON.stringify at this shape; it does not independently implement repository array sorting. Handwritten expected oracle arrays remain the independent ordering witness. Avoid overstating third-party canonicalization coverage.

## Schema refusal precision

Schema exact17+5 cardinalities and runtime oneOf success/error split are closed and coherent. Test's row-extra rejection replaces cases with one row, violating minItems17 as well as extra property; it cannot prove row extra-field closure specifically. Preserve all17 rows and modify exactly one to test additionalProperties. Add runtime negative rows retaining5 cardinality: wrong kind, success with both canonical/error, failure with missing error, unknown runtime field. Current IDs are unrestricted strings; uniqueness test covers observed roster but replacing an ID is not a closed enum refusal. If exact named row inventory is normative, own ID enums in schema too.

## Source-admission next cut

Earlier partition map remains valid: pure2467–2594 +IO2761–3057 +admission types and options-aware taxonomy authority. Extracted IO AST harness currently reads umbrella declarations and injects canonicalJson; moving serialization must retarget direct providers where that function is genuinely exercised. No callback into umbrella should remain in new capture owner; otherwise focused15s import cycle persists. Discovery cached taxonomy API is not equivalent to normalization options-aware validated taxonomy parser.
