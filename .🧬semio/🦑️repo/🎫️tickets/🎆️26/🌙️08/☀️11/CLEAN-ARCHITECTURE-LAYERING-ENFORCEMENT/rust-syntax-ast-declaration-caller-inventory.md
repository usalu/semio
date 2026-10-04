# Rust Syntax AST Declaration and Caller Inventory

Read-only installed TypeScript AST/checker analysis of preserved current full discovery source. No production/native/Cargo edits or runs. Initial bare Bun stdin invocation printed usage and did not analyze; corrected `bun run -` executed this capture.

[Closed JSON input capture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/rust-syntax-ast-inventory/closed-input.json) contains exact original declaration bodies, UTF-8 and character spans, SHA256, local transitive dependency edges, import dependencies and caller rows.

Source SHA256 `e43ac92680773bec45f61d98e5fb91ef4aed87838624c83569d8254e25e0fdcb`; 1019997 bytes. 25 declarations; 31 named caller rows; 57 unclassified forms across 9469 enumerated TS/TSX files.

| Declaration | Lines | SHA256 |
|---|---|---|
| `RustStructuralVisibility` | 6149–6149 | `240f865fd514d22f51fcd2383e08b2aebe0d8c65a986a320c5adfb39963b2c38` |
| `RustTokenKind` | 6304–6304 | `3083db20c9f36acb3d83775e87128bc8838fa1e7a134f97558bcd8960692318f` |
| `RustToken` | 6306–6311 | `ffa9a2ca05b8b11bcd95d3623f3a45a49ce3fc2aad2c4182a1f2f5fa4e2ba898` |
| `RustAttributes` | 6313–6316 | `ba517bf102bf3e243edd9bf73c1053bb4da5930d96f204929e6490ebe09d7536` |
| `RustVisibility` | 6318–6321 | `f966f351320b2312dca1d2bdbda9e674d4bd87a37c3c07f20d0a82024a447e64` |
| `rustIdentifierPart` | 6324–6326 | `c2c95400e8779aa621e29b6b56ff9b45aaf4776e32c50113f203277b8887bcb7` |
| `rustStringValue` | 6329–6359 | `0cff3c23069270837b2cbc67c4c120883d3c17369138020eaf1138dfe1476736` |
| `rustTokens` | 6370–6477 | `5ea55032ca33ef753299eef24f8f48ec8a48f9a695e6cc93197527352faf236a` |
| `rustTokenPairs` | 6480–6497 | `c591c3ded896a8f51cfac65091c17a4e7c1f639acc171902e3e5f78d556b502e` |
| `rustIdentifierSymbol` | 6500–6502 | `86ad7d2b25d20ef7055607450cabc88e6df2653e6ae9965ac6fbcfb0030e0cc7` |
| `RustCompileScope` | 6504–6504 | `ec1185ba4defb9e12b52be8ab26acdb9c7701566979f791c53c63debf949446d` |
| `RustCompileExpansion` | 6505–6505 | `3a4f7fcc03ebbdd6bddc95317f027e07b2e35abc65af60134feab64c3e49b2e5` |
| `RustCompileReference` | 6506–6506 | `3d25a734cb237dfcbee8e83672600717e7b1f9c23aa6a4094486cf95b0bcb052` |
| `inspectRustCompileReferences` | 6509–6803 | `71a00882ca1c3839301b3f8ad88e3a54157f36d8fd0bdee1f0eec9a210c06d18` |
| `rustTokenText` | 6806–6812 | `a92f0b9468449b075189d0a7cf463ead4831170159bb5f2d9d186f748b01a44c` |
| `rustTokenSegments` | 6815–6830 | `91d8adbe0c55c5536ebde652d0b9e1435f1922f97675a4266c3b1bebce68b943` |
| `rustAttributes` | 7848–7858 | `42b6bb4e90bfa766e05f70431fa403943d5709b54df00c8e9d370825a444b717` |
| `rustVisibility` | 7861–7867 | `af593ac905d6011442d930916b2381bd79d446d9f2c345a112a4881f0bc9f62f` |
| `rustPathAttributes` | 7870–7884 | `129dcf3e530814a406055855a306cb0c005d6759915aca0e5c38589910f2af76` |
| `rustFindTopLevel` | 8057–8068 | `5f0265d67de4e8c7a0bab6a577ef275291ef7bf68bcff0881ed25c1feeb9e2bb` |
| `RustMetadataAttributeFact` | 8357–8357 | `f4ccd85cc648dfaeec02482d09a85c8dbb723bd4d35849ac9f0e48d3b84301cd` |
| `rustMetadataPath` | 8360–8373 | `4746bfdbaa3d2e6377c69c79b873d3582193335f1dde2d2e890cf0c724b57738` |
| `rustMetadataAttributeHead` | 8376–8392 | `afbe8459000558ab4d2c8714f4082c633c2c98a0facaa29e3a6026548a508513` |
| `rustMetadataAttributes` | 8395–8414 | `8d1fb4f0ee351240c5d6f91a9e670ad742de0092fa0b45f16ade15cbab8ff43f` |
| `rustMetadataDerives` | 8417–8426 | `cb11b66f4314f863e2337b2749fca42cd9dac6c3450cacbecb18e41d7fed86be` |

Missing roots: `[]`. Import dependencies: `[{"from":["inspectRustCompileReferences"],"symbol":"posix","module":"node:path"}]`.

The closure follows checker-resolved top-level local declarations, including private helpers/types. Static import/export rows are by spelling and require exact resolved-module identity review before direct rewiring. Namespace/star/dynamic discovery forms are listed for classification; arbitrary computed loaders are not proven absent. Preserve full original registered syntax/native laws; source snapshots do not prove compiler emission or whole-consumer coverage.

## Resolved Named Caller Census

31 of the 31 spelling rows resolve to the exact captured discovery module. The remaining rows are retained in JSON, not claimed as extraction callers.

| File | Line | Imported Symbol | Local Name |
|---|---:|---|---|
| `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/🧪️tests/🔮️ownership/🟦️.ts` | 9 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🌎️hub/🧩️compositions/🪐️space/🧫️fixtures/🧪️tests/🔮️ownership/🟦️.ts` | 12 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts` | 8 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts` | 8 | `rustTokens` | `rustTokens` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts` | 8 | `rustTokenPairs` | `rustTokenPairs` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🧩️preservation/🟦️.ts` | 4 | `rustTokens` | `rustTokens` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🧩️preservation/🟦️.ts` | 4 | `rustTokenPairs` | `rustTokenPairs` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧫️private-reader/🟦️.ts` | 11 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧫️private-reader/🧩️preservation/🟦️.ts` | 5 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts` | 8 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts` | 8 | `rustTokenPairs` | `rustTokenPairs` |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts` | 8 | `rustTokens` | `rustTokens` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts` | 2 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts` | 2 | `RustCompileReference` | `RustCompileReference` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts` | 6 | `RustCompileExpansion` | `RustCompileExpansion` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts` | 3 | `rustTokens` | `rustTokens` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts` | 3 | `rustTokenPairs` | `rustTokenPairs` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts` | 3 | `rustIdentifierSymbol` | `rustIdentifierSymbol` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts` | 3 | `RustToken` | `RustToken` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts` | 3 | `rustTokens` | `rustTokens` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts` | 3 | `rustTokenPairs` | `rustTokenPairs` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts` | 3 | `rustIdentifierSymbol` | `rustIdentifierSymbol` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts` | 12 | `rustTokens` | `rustSyntaxTokens` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts` | 12 | `rustTokenPairs` | `rustTokenPairs` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | 25 | `rustTokens` | `rustSyntaxTokens` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | 25 | `rustTokenPairs` | `rustTokenPairs` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts` | 4 | `inspectRustCompileReferences` | `inspectRustCompileReferences` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🟦️.ts` | 7 | `rustTokens` | `rustTokens` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🟦️.ts` | 7 | `rustTokenPairs` | `rustTokenPairs` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧱️architecture/🟦️.ts` | 11 | `rustTokens` | `rustTokens` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧱️architecture/🟦️.ts` | 11 | `rustTokenPairs` | `rustTokenPairs` |

Namespace/star/dynamic candidate forms (including other discovery submodules and computed paths) remain explicit in the JSON and require manual classification; they are not silently treated as absence.
