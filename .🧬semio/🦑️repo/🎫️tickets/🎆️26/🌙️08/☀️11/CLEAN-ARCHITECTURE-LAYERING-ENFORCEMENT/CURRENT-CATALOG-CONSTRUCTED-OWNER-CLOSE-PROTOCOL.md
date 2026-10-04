The actual Catalog law violates the existing explicit close protocol. RetainedPackSymbolTable::try_new creates empty backing lists but closed=false. terminal_is_empty requires closed=true; Drop asserts that predicate. Empty content therefore does not imply a terminal owner. A positive-budget close_step reaches Complete and sets closed=true once both backing owners are terminal.

The first successful try_new(...).is_ok() drops its unclosed temporary immediately; two later shadowed configured owners also omit close. Preserve the production close/Drop contract and close each successful law owner before its temporary/shadow lifetime ends, retaining all original kind/refusal assertions. The actual panic is consistent with this source protocol; no production equivalence or new runtime acceptance is inferred.

Evidence: 🗑️generated/pack-integration-native/replay-5/independent-catalog-constructed-owner-close-protocol-1.json (full compiled snapshot defining source and law frames).
