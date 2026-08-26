# Cargo Cache Ticket Scope

## Defect and correction

The prior `cargo-cache-tag` contract combined a global `**/CACHEDIR.TAG` path with the directory kind `ticket-cargo-target-evidence`. Because that directory kind can lexically recognize any `🧪️target-*`, a production lookalike could receive Cargo fixed-filename authority.

The contract is now conjunctive in two independent dimensions:

- exact governed path grammar: `**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/CACHEDIR.TAG`;
- exact parent directory kind: `ticket-cargo-target-evidence`.

The leading `**/` intentionally admits both canonical ticket roots and embedded ticket roots, while the complete metadata/date hierarchy prevents unrelated `tickets` lookalikes. The directory-kind conjunct still rejects nested Cargo platform subdirectories and noncanonical target parents.

## TDD evidence

The new production-lookalike negative first failed by resolving `cargo-cache-tag`. After narrowing, the six evidenced embedded leaves remain admitted while basename-only, wrong-parent, production, malformed ticket-prefix, and malformed date cases reject.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cargo-cache-tag.test.ts'
4 pass
0 fail
19 expect() calls
```

Direct shipped-schema validation returned `problems=[]`. The canonical taxonomy JSON SHA-256 at this checkpoint is `c12f5582df1a5f95bf9c012bb288d72401e32cef5a51b5ea3ef4a50729eee9f3`.
