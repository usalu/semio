# Prepared component publication

## Preparation gate

One weakly indexed mutex and condition variable gate serializes Owned preparation per package digest. Waiting callers observe cancellation at most one 50 ms polling interval later under ordinary scheduling. The active claim releases and notifies on every Rust return path. An owner whose strict constructor requests an import, exceeds budget, or returns a refusal leaves the process cache unpublished. A retry can become the next owner.

The final source snapshot is sealed only after its own readiness and progress callback, and the activation scope is subsequently checked in the preparation wrapper. The original startup receives the immutable cache parent while a paid home is held, then opens a fresh Source and reads the real channel. The file digest should be chosen before acquisition of this gate.

## Publication findings

- Cached returns before and after acquiring the gate can bypass a cancellation checkpoint. A cancellation arriving between the post-preparation checkpoint and acquisition of the shared cache lock can also publish a handle. Checking under the cache lock immediately before a hit or publication defines the linearization point.
- The persisted file identity cache keys by path, length and timestamp. On a known identity and cold compiled parent the code can read and compile file bytes without hashing them. The Owned runtime installs the supplied hash as a label. This permits stale file facts or a modified local cache entry to associate the wrong compiled component with a content digest shared by held-byte callers. Current repository callers receive publication custody through the launcher, which mitigates normal writer races within that controlled directory, but the independently persisted hash is outside the model’s verified publication.
- The known-identity cold path reports LoadingCompiledCode and then returns to ReadingComponent. The raw activation reporting port itself has no monotone clamp. Choosing a verified current digest in the read/hash phases before a cache/load lookup also removes that backwards phase transition.
- The process cache key does not include the supplied runtime object or backend. Current headless production routes fetch the one shared Owned runtime. The public helper's type still admits other runtimes under the same digest. Runtime identity enforcement at admission or inclusion in the cache key would make this enforceable at the publication boundary.

## Recheck after the gate correction

The corrected implementation checks cancellation under the compiled-map mutex immediately before copying a cached handle or publishing an entry. Only the exact shared process runtime touches that map. A cold file path is always read and cryptographically hashed before compile and gate selection. Persisted identity facts are gone. The in-process file identity hint can only select an existing entry previously verified from bytes. Its freshness assumes the publisher retains exclusive mutation custody over that path between model publication and gateway exit.

The pinned Rust metadata check passed for the corrected production module. It rebuilt the actual guest runtime, shard, post-turn relay, shard executor and host facade against exact retained dependencies. The check wrote about 4.1 MiB of final metadata and did no native linking or Cargo work. The invocation is recorded in `mcp-prepared-startup-check.ts --mcp-module --current-host`. This audit made no production changes.

## Cache authority regression

The Rust regression runs the corrected cache with the real import-free prepared Core fixture. An unrelated Owned runtime prepares locally before the shared cache exists. The first shared attempt is cancelled when the preparation completion report fires and leaves the shared cache empty. A live retry prepares again and publishes. Cancelled cached access refuses without clearing that record. A second local preparation still has a distinct origin, while the final live shared request uses the existing parent without preparing again.

The Bun WebAssembly and Ajv oracle passed 32 assertions against the same six-step fixture. Pinned Rust type-checked the exact production module and adjacent test. The linked native test passed 1/1 using the Core build from the startup custody proof and the actual host facade. The focused executable occupied 25.2 MB and the five host libraries 29.4 MB. The command was `mcp-prepared-startup-check.ts --mcp-module --current-host --cache-test --execute`, invoked through the pinned Bun registered by `repo`.
