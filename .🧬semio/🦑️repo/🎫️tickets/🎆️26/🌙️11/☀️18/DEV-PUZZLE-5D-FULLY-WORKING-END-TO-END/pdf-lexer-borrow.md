# PDF Lexer Borrow Fix

The final Hub build stopped with Rust E0499 in the PDF lexer's normal-object length stage. The match borrowed the cursor's phase mutably, and the planner borrowed the whole cursor while that phase borrow was still needed to read the position. The length stage now copies both scalar coordinates before it calls the planner. The planner still owns the phase transition and work receipt.

Source touched:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️lexer/🧭️normal-object/🦀️.rs`

The existing stream fixture exercises the length request, borrowed content range, cancellation and backing retirement. The complete lexer crate and its tests compiled to metadata through the repository scheduler using the Hub lane's cached Rust dependency artifacts. The direct repository Cargo package selection from the S workspace refused the lexer package before compilation.

The separate runtime-test linker spent more than fifteen minutes paging through the static Hub dependencies. I stopped that linker to release memory for the concurrent integration run. Runtime test execution remains unverified. The compiler check completed successfully before that link attempt.

## Independent native test attempt

A separate native invocation attempted to link the actual lexer test crate against the cached model and snapshot dependencies without the primary Cargo lock. The cached rlibs contain metadata stubs and the full metadata is separate. Explicit rlib and rmeta pairs, dependency-path resolution, and disposable archive copies with full metadata substituted each failed with Rust E0463 for the model and snapshot crates. Reflink copying the cached build was unavailable; a full independent copy was stopped to avoid competing with the primary build. These attempts did not run the lexer tests. The earlier metadata compilation succeeded; runtime test verification remains open. The input script `pdf-lexer-native-lane.py` is retained in this ticket.

## Private overlay verification

The native lexer library test suite completed successfully: **18 passed, 0 failed**. The run used a private mount namespace with an overlay over the cached build directory, a copied-up Cargo lock, and `--offline --locked`; it did not take the primary build-directory lock. The retained input is `pdf-overlay-run.sh`. The earlier failed direct-link attempts above are superseded for runtime verification.
