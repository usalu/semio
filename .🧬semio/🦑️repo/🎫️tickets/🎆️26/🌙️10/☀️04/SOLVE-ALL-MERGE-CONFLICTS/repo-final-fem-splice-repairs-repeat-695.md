# FEM Final Source Splice Repairs — Repeat 695

The parent static source audit identified two composed owners. Neither problem occurs in the immutable H or S declaration. Both guarded changes preserve authored complete owners; no build, compiler, typechecker or test was executed.

- ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🪜️job-and-steps/🦀️.rs: Retired orphan pcg tail: complete exact-S function already exists in run-and-entry; retained the closing PcgJob impl brace and current H retained close body.
  Before SHA-256: 1dbc24be80e631de5af8f5e5dd87f0fa6bb1bd9f3d37d58eed4bb9b646eb2970; after: e97d30d492a2c34b487848598ba699bb8449303fca0ee593160a21da48393248.
- ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🤝️numerical-protocol/🦀️.rs: Coalesced H pub(super) and incoming public modifiers to authored S pub; both complete current helper declarations/bodies now exact S, including generic ExactSizeIterator input.
  Before SHA-256: 2aa919617e3cc8936b051b6703b77d230c44d2e054ad168aaab1229b48b038df; after: a2dc20bcc000558110efe3cc9cd54bfa1af059acfd1d2f80639a8bc21e8c98f0.

The PCG current run-and-entry pcg function was compared as a complete declaration with immutable S and is identical. Its duplicate tail inside PcgJob was removed; the existing impl closing delimiter remains. The current H retained close function and all surrounding constructors/checkpoint code remain untouched. The two numerical functions retain S public access, bodies and generic signature; the duplicate H restricted modifier was the only removed text.

The complete current PcgJob close_retained_step declaration was independently compared with immutable H after repair and is identical (newline/outer whitespace normalization only).
