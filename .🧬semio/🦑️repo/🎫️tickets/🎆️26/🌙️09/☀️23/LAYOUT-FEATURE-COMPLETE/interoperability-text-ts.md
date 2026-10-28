# IDML Typography and Story TypeScript Counterpart

## Changes

Added real semantic XML-node transformations using the canonical owned Layout, XML and binary32/binary64 types. The module exports paragraphNode, characterNode, characterColor, readColors, readStyles, readStory, storyFragment, storyFragments and storyNode. It maps font families, categorical font weights, italic/oblique faces, point sizes, leading, tracking, paragraph justification and RGB text colors. BasedOn inheritance resolves iteratively and rejects cycles or missing base styles. Inline overrides become new owned styles with collision-safe identities, including direct AppliedFont and Leading attributes.

Story offsets use native UTF-8 byte positions. Fragment boundaries respect Unicode scalars, style boundaries and line breaks; the fragment generator accepts a caller-selected byte budget and yields between chunks for caller progress/cancellation. XML traversal and inheritance avoid recursive call-stack growth. readStory returns the imported story and derived style collections while leaving the input collections unchanged.

The isolated interchange TypeScript entry exports the module as idmlText. No Layout artifact public root, package composer, native source, shared fixture or unrelated interchange source was modified. Runtime imports are first-party IEEE capture/value helpers; the Layout/XML imports are owned type-only imports.

## Test-First Durable Witnesses

The language-neutral JSON witness and full semantic-law entry were written before the production module. Tests consume the existing shared idml-style-runs fixture, including Ä😀 byte ranges 0..6 and 6..12. New witnesses cover BasedOn inheritance, inline style properties/attributes, generated-id collisions, exact exported XML typography attributes, RGB resources, UTF-8 byte ranges, a 2340-byte Unicode story split under a 512-byte fragment budget, malformed style numerics, missing bases, cycles, invalid text ranges and translucent text-color refusal.

A test-only owned XML port isolates fast-xml-parser XMLBuilder/XMLParser from production. Its adapter reads and writes the real owned XmlNode tree. Expected style/story values and exact exported XML attribute/property labels come from language-neutral fixtures.

## Executed Evidence

The persisted ticket input script text-ts-check.mjs completed and returned:

    { strictTargetDiagnostics: 0, actualSourceFiles: 241, cases: 14, generatedModules: 6 }

TypeScript 5.9.3 constructed the actual imported source graph and strictly checked syntactic plus semantic diagnostics for the implementation, durable test source and interchange entry. No virtual type stubs were used. This is a targeted strict source check; unrelated framework dependency bodies were not all typechecked.

The same script transpiled the actual test, implementation, first-party IEEE/refusal modules and fixture JSON modules, then executed all fourteen cases in Node. The independent test-only XML library was installed fast-xml-parser 5.11.2. Six compiler-output modules were created under the ticket generated directory and removed in the script finally block.

## Scope and Limits

This module owns typography and story semantics over already decoded XML trees. Native Rust execution and complete IDML package interoperability remain separate checks. It does not establish complete Adobe IDML conformance or whole-project TypeScript compilation. Font weights follow the native categorical IDML face-name mapping; arbitrary numeric font-weight identity is not promised across those categories. RGB text export requires opaque colors, matching the native refusal contract.

## Files

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/🔣️.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔀️interchange/📦️idml/🔤️text/🧪️tests/oracle.mjs
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/text-ts-check.mjs
