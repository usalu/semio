# UI Schema Reference Review

Path text updated during fixture placement. No constraints or allowed roots were added, but three existing scalar schema `const` path values changed together with their authored catalog coordinates. This is material schema reference content and needs semantic consumer review.

File: `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json`

JSON Pointer: `/$defs/BrowserHostDeclaration/description`

Value: `🌐️ Language-agnostic declaration of the browser UI host port. Bounds are the ceilings the owned Rust host enforces (`🖥️host/🪟️window/🦀️.rs` `BROWSER_HOST_*`, `📡️event/🦀️.rs` `BROWSER_EVENT_*`, `🌉️abi/🦀️.rs` `ABI_*`) and the laws in `🧪️tests/🧫️fixtures/📐️browser-host-limits.tsv`.`

File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json`

JSON Pointer: `/$defs/ScalarDeclaration/properties/closePlan/const/reader/path`

Value: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️tests/🧫️fixtures/🔣️.json`

File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json`

JSON Pointer: `/$defs/ScalarDeclaration/properties/closePlan/const/readerAlias/path`

Value: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️tests/🧫️fixtures/🔣️.json`

File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json`

JSON Pointer: `/$defs/ScalarDeclaration/properties/closePlan/const/page/path`

Value: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧪️tests/🧫️fixtures/🔣️.json`

File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json`

JSON Pointer: `/closePlan/reader/path`

Value: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️tests/🧫️fixtures/🔣️.json`

File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json`

JSON Pointer: `/closePlan/readerAlias/path`

Value: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️tests/🧫️fixtures/🔣️.json`

File: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json`

JSON Pointer: `/closePlan/page/path`

Value: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧪️tests/🧫️fixtures/🔣️.json`

Source reader audit: repository-local text searches for `scalar/🔣️`, `scalar/🧬️schema`, `resident/🔢️scalar`, `original-reader`, and `reader.path` in UI and repo-library TypeScript/Rust found no direct owning Source consumer of the three scalar example catalog fields. The `host` schema reference is description text. The scalar schema selectors are existing `const` values beneath `ScalarDeclaration.closePlan`, with corresponding fields in the authored scalar catalog JSON. Generic schema/catalog admission consumers still require Root audit; absence of a direct text match does not establish runtime safety.
