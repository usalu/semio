# Warm Singleton Retained Source Watch

The component driver's warm singleton watcher previously retired a child as soon as it physically confirmed a changed checkout digest before a link announcement. The driver had already closed Source admission with the original bodies under its retained source witness. A running Rust unit could still have consumed the retained original. In `wRv2THU`, this early decision discarded attempts on the Store and BREP sources before Rust could announce their consumed checksums.

The watcher now defers the early retirement when its retained witness has the exact admitted SHA-256 for the mismatched path. If the source digest has no associated witness, the watcher can still retire the unlinked child and request the same graph re-admission. The guard only affects the watcher's callback. Cargo failure diagnosis, mixed-version checks, unwitnessed consumed checksums, unchecked input validation, and the final witness verification and transfer continue to use the original paths.

The mounted driver fixture performs a real Rust dep-info emission with BLAKE3, writes the new source after Rust exits, and waits 1.5 seconds before announcing the link. The red run retired this child and published the replacement SHA on the retry. Its unbacked control correctly retired the child and reached the fixture's deliberate retry failure. The targeted runner uses the repository process suite and its isolated compiler reservation.

Both targeted cases passed after the change. The retained case launched Cargo once and sealed the original SHA-256 and rustc BLAKE3 checksum and size while the checkout still held the edited bytes. The unbacked case issued two Cargo launches, ended before producing a link announcement, and published no completed build. Scoped whitespace verification passed.

The driver source was updated at 05:58:59 PST with SHA-256 `e9870f4576b7accbb1f9b58b5999c1e4c797dc97fc984b2ff6488bc7cbe4d8b0`. Running release jobs were left alone.

Six existing mounted driver regressions passed. They cover mixed-source retry, repeated mixed-proof refusal, retry for a disappeared consumed source, refusal of an unchecked changed source, and hard Cargo failure after reclaim in one- and two-pass runs.
