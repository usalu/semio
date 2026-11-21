# Managed generation exchange import failure

The managed Semio producer stopped during module loading. A direct Bun import through the repository tool registry reproduced the same startup error from the managed producer log: `SyntaxError: Export named 'encodeClosureMemoV1' not found`. No native compilation is needed to reproduce it.

## Call path

- `remoteGenerationExchangeV1().publish` calls `publishGenerationV1` in the generation remote exchange module. That function calls `uploadClosuresV1`, which opens the bounded local `.closures.json` memo, passes it as the fourth argument to fragment inventory, and encodes a replacement when the fragment set changes.
- The closure-shards owner exports the closure entry identifier, fragment inventory, shard packer, and shard reader. Its inventory takes three arguments and validates and decompresses every fragment it reads. It does not yet implement or export either memo function. The matching restore inventory call uses the three-argument API.
- The adjacent transport schema in the same working-tree changes introduces a bounded `ClosureMemo` with a version, an entry digest, and limits, and the committed upload path already caches fragment references. The memo calls instantiate an active part of the shard cutover.

## Verification

The focused import below exited 1 in Bun 1.4.2 on macOS arm64. The failure occurs before a fixture can run through the exchange module. The producer log records the same error.

```sh
repo tools exec bun -- -e 'await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts")'
```

Aligning the caller by removing the memo would discard the upload optimization that the schema and the consumer preserve. The owner-side codec and inventory integration remain part of the fleet change. The managed producer remains blocked until the live module import succeeds.
