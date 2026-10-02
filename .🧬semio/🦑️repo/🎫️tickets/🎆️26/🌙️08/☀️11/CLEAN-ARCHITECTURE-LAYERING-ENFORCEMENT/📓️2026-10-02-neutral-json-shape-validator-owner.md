# Neutral JSON Shape Validator Owner

Read-only source audit; no jobs or code edits.

## Concrete coherent domain

Existing `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` already owns pure JSON-schema object recognition/equality and validation, without runtime imports at its head. This is the established neutral domain for shape validation, below Repo taxonomy/normalization. Prefer implementing a narrowly exported JSON shape contract there, or a schema-registered child facet of that validator, rather than inventing another generic Repo taxonomy helper directory. Child taxonomy naming must be registered first; existing validator owner is immediately grounded in repository structure.

Move the five shared function implementations once only after owning their closed schema/fixture. Taxonomy and normalization import neutral validator directly; neutral validator must not import Repo serialization, taxonomy or normalization. Exact-key comparison needs no Repo canonicalJson: sorted own key arrays can compare length and each element using native strings. Preserve existing duplicate expected-key refusal semantics rather than silently deduplicating.

Public owned types: JsonRecord=Record<string,unknown>; record(value:unknown,name:string):JsonRecord; stringArray(...):readonly string[]; requiredString(...):string; requiredLiteral<T extends string>(value,name,allowed:readonly T[]):T; requireExactKeys(record,keys:readonly string[],name):void. These use only first-party/native types. Caller-owned diagnostic labels/domain must replace hardcoded Taxonomy v7 on unrelated inputs; choose a closed diagnostic code + path context contract before renaming errors. Avoid callbacks into facade, foreign AJV interfaces, runtime schema engine requirement, aliases or generic validator forwarding to taxonomy.

## Actual caller matrix

Taxonomy owner430–452 defines all five; parseTaxonomy and fixedExpiry consume them repeatedly. Normalization9 imports all five from taxonomy. Actual non-taxonomy domains: plan tagged variants requiredLiteral466/499/508/590; generator preview record750–761/requiredString763+; dependency policy JSON records3325+; mutation catalogs record3749+, strings/kinds/scenario fields3758–3764+; publisher manifests4321; mutation descriptor4550; compiler inputs4879+; transaction sentinel5705–5710 exact-key checks; Nx manifests/target options6875–6892. Lines may shift with concurrent cleanup; symbols/domains are stable anchors. No other direct import of the five from taxonomy found in current framework TS census.

## Exact bounded semantics and closed tests

Current record admits any nonnull nonarray JS object, including Date/Map, because signature is unknown rather than JSON-only. stringArray admits empty arrays and empty strings; sparse array holes pass .some. requiredString rejects empty string but does not trim or NFC-normalize. requiredLiteral first requires nonempty string, then exact includes allowed. requireExactKeys sees enumerable own string keys only; symbol/inherited/nonenumerable keys do not participate. These are observable current semantics, not assumptions that all arbitrary runtime values satisfy JSON contract.

Language-neutral corpus for authored JSON: object vs null/array/scalar; empty/valid/mixed string arrays; empty/nonempty whitespace string; exact/case-mismatch/unlisted literals; key order permutations, missing/extra keys, duplicate expected names, own __proto__ key. Independent AJV draft07 object/string/minLength/items/enum/required/additionalProperties schemas provide admission oracle for the JSON rows. Exact-key expected duplicate schema must be rejected by owned API or explicitly preserve current failure; cannot use AJV required duplicate as valid schema oracle.

Runtime-only companion rows decide sparse arrays, Date/Map, inherited/symbol/nonenumerable properties under declared contract. Do not accidentally strengthen these while claiming a pure extraction. If neutral contract deliberately becomes JSON-value-only, handcraft every actual caller and own the changed refusal behavior first. Tests should assert identity of accepted record/string array, generic literal inference via real TypeScript caller, and no input mutation. Bun/TypeScript source compilers plus actual runtime supply implementation parity; AJV remains test-only behind owned fixture adapter.

Existing schema module draft07-oracle and fragment-validation-oracle fixture/tests establish independent validator infrastructure; do not repurpose their existing IDs to conceal new shapes. Register new focused law through actual schema/package permanent scripts and launch entry, retaining existing provider collections. Root current witness work is independent; no recommendation to merge this semantic cut into that receipt.
