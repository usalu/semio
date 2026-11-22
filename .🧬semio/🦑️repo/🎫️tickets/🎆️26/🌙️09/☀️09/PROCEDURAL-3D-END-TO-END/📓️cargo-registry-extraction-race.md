# Cargo registry extraction race (memmap2 / mio)

## Symptom
Concurrent `repo cargo --store=shared` runs failed twice on crate sources in the shared Cargo home: a transient `memmap2` extraction failure and a missing `mio` file.

## Root cause
The Cargo ingress gate itself rewrote extracted registry sources in place, outside Cargo's `.package-cache` lock and outside any lease that excludes readers.

Evidence (all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/`):
- `🟢️ready/💾️storage/🟦️.ts`, `withRepositoryCargoRegistryReadyOnceV1`: the writer path calls `sourcesIntact(true)` (before and after `cargo fetch`). It is guarded only by the per-request lease `cargo:registry-fetch:<home>:<identity>`, not the registry-wide `cargo:registry:<home>` lease that builds hold shared. Two different requests, or a request and a running build, therefore never exclude each other.
- `🧾️fetched/🟦️.ts`, `fetchedSourcesV1(repair=true)` treats any `registry/src/<index>/<crate>` whose files differ from the checksum-verified `.crate` as unsound. Cargo extracts in place: it creates `.cargo-ok` empty, writes the files, and writes `{"v":1}` last. A directory that another Cargo is extracting at that moment is, by this test, indistinguishable from a corrupt one.
- The old `restoreUnpackedV1` then did `rename(path, aside)`, `rename(staged, path)`, `rm(aside, recursive)`. Between the renames the crate directory does not exist, so a concurrent `rustc` or a Cargo extracting the same crate gets ENOENT ("missing mio file"). Cargo's in-flight extraction writes into the moved-aside directory, which is then deleted (the transient `memmap2` extraction failure). Two gates repairing the same crate also destroy each other's result.

Ruled out:
- Cargo's own package-cache lock is not bypassed by per-lane CARGO_HOME: there is one `CARGO_HOME` per image, `cargo/home`, shared by all lanes (`📍️layout/🟦️.ts`, `homeDirectory`). The gate only bypasses it for its own writes.
- No cache GC or reclaim touches `cargo/home` or `registry/src`: the ledger policy lists `cargo-home` as managed by owner `cargo`, written only by the compiler store, and carried across image recreation (`💾️storage/🧾️ledger/🔣️policy.json`, `📏️needs` test: `carry` includes `cargo/home`). I grepped for rm/reclaim of it and found none.
- The Rust image remount is not implicated: the home is carried, and a healthy tree remains. I checked the live home (`.cache/b/r/m/cargo/home`): all 1646 extracted crates are sound against their archives, with no leftover `.repair-` or `.stale-` directories. I could not reproduce tonight's failures from logs; the cause above is derived from the code and shown by the mutation check below.

## Fix
New leaf `🧾️fetched/🧱️install/🟦️.ts` (`installUnpackedV1`), used by `restoreUnpackedV1`:
- absent directory: fully staged beside it, then one atomic `rename` into place; if someone created it first the staging is discarded and the in-place path below runs;
- present directory: completed file by file via same-directory temp file + atomic `rename`, `.cargo-ok` last. The directory is never moved aside, truncated or removed; identical files are skipped, so concurrent extractors writing the same bytes are idempotent and a reader never sees a file or the directory vanish, and a `{"v":1}` marker always implies complete content;
- no recursive delete remains in the repair path.

I chose in-place idempotent completion over a content-addressed symlinked store, because Cargo itself extracts in place into `registry/src` and would bypass any parallel store; making the gate non-destructive removes the only destructive writer.

## Tests
`🧾️fetched/🧱️install/🧪️tests/🟦️.ts`: six concurrent installers plus a simulated in-place Cargo extraction (marker first empty, files, marker last), then six concurrent repairs of a tampered, marker-less directory, while a reader polls continuously. Invariants: the directory never vanishes, no file seen once disappears, a `{"v":1}` marker implies full content, no `.repair-` leftovers, and the final tree equals a Python `tarfile` extraction of the same archive (third-party oracle; archive also built by Python). The existing `fetched sources` test now goes through the production installer. Reverting to the old rename-aside algorithm makes the new test fail (checked with a temporary mutant, removed). There is no GC in this path, so no GC case is tested.

Run: `repo tools exec --light bun -- test ./<file>` on both files: 17 pass.

## Residual
Reader and fetch leases stay as they are (the per-request fetch lease is now safe because repairs are idempotent). A repair that finds a mid-flight extraction writes identical bytes, which is harmless.
