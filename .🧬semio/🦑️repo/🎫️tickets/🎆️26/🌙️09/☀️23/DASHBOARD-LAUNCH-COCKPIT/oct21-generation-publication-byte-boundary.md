# Generation Publication Byte Boundary

The canonical generation writer previously checked compressed bytes only. Its own fragment/index/frontier readers bound expanded bytes. A compressible canonical document could therefore be named and written even when that reader could not consume it. This is a source-proven contract mismatch; the role and loaded bytes of previous failed build requests are not established by this finding.

The closed DocumentByteLimit schema and ten language-neutral cases were added before the production change. The existing generation test file now exercises exact ASCII, BMP, astral, escaped, compressible overflow, compressed overflow and invalid limits. The existing independent Python JSON oracle now checks canonical UTF-8 byte counts in its normal fixture check. Actual Bun/Python execution has not run; no runtime red or green result is claimed.

The writer uses packGenerationDocumentV1, declared with a first-party Uint8Array return type. It validates a positive safe integer at most the unchanged declared index ceiling, counts canonical UTF-8 bytes before Buffer allocation/compression, enforces the existing role limit on expanded bytes, uses that byte count for the Brotli hint, then enforces the same limit on packed bytes before hashing/writing. Existing fragment/index/frontier limits and reader behavior are unchanged. Canonical serialization still materializes text; this is not streaming or an established fix for an unidentified historical payload.

Static verification: existing generation scenarios remain semantically identical, the closed fixture passes existing Ajv, the MJS/declaration/TS parse without diagnostics, and standalone system Brotli DATA agrees with all ten expected boundary outcomes. The 1,026-byte compressible vector packs to 14 bytes and exceeds its 1,025-byte expanded ceiling; the four-byte null vector packs to eight and preserves the packed ceiling refusal. Maximum authored source path is 148 UTF-16 units. These checks are not product runtime proof.

The new eleven laws live in an adjacent bounded document-size suite imported by both the existing generation suite and Source0. SQLite JSON computes independent UTF-8 BLOB lengths for each neutral case. The static Node SQLite DATA query matches all ten vectors; actual Bun SQLite/product execution is unrun. The existing roster remains 42 targets, without importing the whole generation suite into Source0.

Changed files:

- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🧬️schema/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🐍️oracle.py
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.d.mts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/📦️document-size/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts

The authored Source0 acceptance plan now explicitly requires all eleven new actual body names, alongside its existing regression names; command, target roster and filters remain unchanged.
