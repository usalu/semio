# Canonical seal work during source transfer

The native paged-digest law stalled for ten million turns. The 76-byte first fixture finished canonical traversal and stopped while moving its source. On each retry the seal requested two work units, reported zero completed work, and repeated the same physical quote of 81 copy bytes and 48 release bytes.

The seal now uses its source's computational work demand for the take and close phases and uses the controlled retirement owner's demand while retiring a cancelled edit. The physical grant, work meter, cancellation and destructor checks remain active. The native paged-digest test passed in 4.11 seconds after this change. It compares paged and exactly funded seal results against an independently framed SHA-256 digest.

Source: `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`. Native verification runner: `seal-native.py`.

A separate GDB trace of a document edit found a 75-unit digest action inside a fixed 64-unit store callback. The batch digest interface now supplies the selected owner's work quote. The store reserves that quote before entering the native receiving effect, including the original seal's traversal, source transfer, and retirement phases. Integrated command verification is pending.

The store changes are in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs` and `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs`.

The registered Cargo invocation for `semio-framework-replication --test paged_seal_digest` independently passed the same oracle in 2.87 seconds.
