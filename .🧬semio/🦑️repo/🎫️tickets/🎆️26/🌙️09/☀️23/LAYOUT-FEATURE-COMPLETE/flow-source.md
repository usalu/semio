# Retained Flow Frame Source

The native FlowFrameSource trait lends one copied FlowFrame by ordinal through len/get; TextFlowCursor::advance_source consumes it without flattening the source. The TypeScript owned interface and advanceSource counterpart use length/get. Existing slice/array advance remains a convenience call into the same evaluator. Both public TypeScript facades explicitly export the source type.

The ephemeral schema now defines sourceLength0..4294967296, matching every accepted u32 ordinal. Negative/fractional/greater lengths are refused before lookup in TypeScript; native refuses lengths above2^32 on64-bit targets. No accepted ordinal ceiling was reduced. Lookup implementations must retain immutable owners and provide bounded scalar random access. A pending region and zero grant perform no row lookup. The cursor owns no source pages, strings or frame collection.

Test-first evidence: new language-neutral sourceCases and TypeScript source laws were written before production. The original compiler reported missing FlowFrameSource/advanceSource. Actual repaired source execution passes5 existing portable cases across3 grants plus9 paged/sparse/refusal source cases,7 existing refusal assertions and1 cancellation. Independent test-only Three geometry and Ajv schema readers remain behind oracles.mjs; source-backed results match their admitted columns. A sparse test resolves ordinal4294967295 with length4294967296 without allocating the matching array.

Strict TypeScript closure, including the public facade:249 actual source files,0 diagnostics. Native production/test files parse with the installed Rust tree-sitter grammar:2 files, maximum path146. Native laws use the original framework PagedList configured for2 scalar rows per payload page, replay the shared fixture, and cover sparse maximum ordinal/refusal on64-bit. Native compilation and laws are unrun because the repo runner remains stale/unavailable.

Files: scene/🧵️flow/{🔣️.json,🦀️.rs,🟦️.ts,🧪️tests/cases.json,🧪️tests/🦀️.rs,🧪️tests/🟦️.ts}; Layout public TypeScript root facade and its flow query facade. Ticket flow-source-check.mjs is the retained verification input and removes its own generated output.

The canonical document-to-paged-metadata projection remains the next task; this seam alone does not wire export/interactive story flow end to end.
