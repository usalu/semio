# Frozen receipt phase batching

The native SDK trace of the pinned WFC job sampled a frozen receipt at 32,768 Step dispatcher iterations and epoch 118,515. The interpreter also executed guest instructions during other samples. With the first component the job returned a retryable budget refusal and preserved the 306-byte Pack and 204-byte SPR.

Ordinary validation now continues into the next completed phase of the same authenticated receipt while its Source turn still has fuel. Each phase retains its previous operation limit and uses the original dual native borrow. A pass stops at a pending phase, a drained budget, a deadline or an issuer boundary. The 32-phase bound applies even with ample funding. Bootstrap and ledger settlement still use their separate original issuer turns.

Five language-independent cases cover zero, two and 32 escrow rows, exhaustion and parking. SQLite checked the transition order and boundaries. The three Bun laws passed 21 assertions. The linked Rust transition functions passed all five cases. The initial Rust run failed because the original implementation had no multi-phase pass.
