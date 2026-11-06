# Borrowed Binary Source

The canonical OriginalBinaryReadSource interface now has the existing PagedCommandReader implementation and OriginalBinarySlice, an allocation-free short borrow of the actual immutable payload and an actual retained offset. OriginalCommandVarint and OriginalCommandField use this interface. The slice does not own input, retain callbacks, mint a Source, or fabricate a native source view; the enclosing consumer must qualify the actual Source and retained immutable payload on every call.

The neutral borrowed.json/schema corpus precedes implementation and distinguishes actual work from copy allowance. Independent SQLite and TextDecoder oracles plus existing page/varint/UTF8 laws pass: 3 tests, 43 assertions. The actual Rust law reuses the corpus, checks zero-work/payload pointer/heap receipts, exercises the shared UTF8 field atom through the borrowed implementation, and physically retires every produced field. Authored, not compiled.

Registered rustfmt parsed the four actual source/test entries (formatting-diff exit1, no parse error). Actual owning command-ingress cfg(test) Source closure is16files114562bytes1681activeLines, unchanged limits. A first preliminary measurement mistakenly selected the unrelated outer wire facade; the owning entry was then measured correctly.

AppFrame has no current OpBinaryReceiving implementation. Its ordinary async decoder and nested causal-envelope decoder allocate raw complete values; they are not used as a fallback. A complete Source-owned AppFrame receiver and MCP TurnResult consumer remain implementation work, including genuine CausalPayload birth and all 33 tags. Root owns session/map/lifecycle integration. No native/artifact/server success is claimed.
