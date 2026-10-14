# Ordered Surface Lookup Qualification

The surface map's mutable ordinal lookup follows the same order array and physical slot as its identifier lookup. Host component retirement can access the original mutable surface without allocating a copied identifier. The neutral sparse-slot fixture inserts three entries, removes the first, reuses the vacant slot, mutates ordinal two, and checks the stable remaining order and absent ordinal.

The closed fixture schema and independent SQLite ordinal oracle passed through actual Nx (`surface-order-oracle2.log`). The source check confirms the original order/slot lookup without identifier cloning. The first route attempt failed before the test because the new route lacked its initial child argument declaration; that declaration was fixed without changing the law.

The Rust heap-observation law is authored alongside the fixture and checks zero allocation/release for lookup, actual sparse reuse, all expected values and terminal retirement. It has not run; the SQLite/source gate is not a native heap qualification. Files: Scenes Wgpu owner `🦀️.rs`; `🧪️tests/🔬️wgpu-admitted-surface-map/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,🧫️fixtures/🔣️.json}`; retained headroom script/project route and launch registration.
