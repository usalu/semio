# Sharded Inventory and CLI Closure

## Result

The inventory artifact writer now emits deterministic owner shards strictly below 5 MiB at `📊️shards/🔖️<sha256>/🔣️.json`, followed by a canonical manifest at `📊️shards/🔣️.json`. The retained pre-transaction-v2 monolith at the inventory data root is neither overwritten nor removed.

Inventory canonical hashing yields top-level and row chunks incrementally; neither `entries` nor `violations` is passed to `canonicalJson` as a complete array. A small witness reconstructs the exact canonical bytes and matches Node SHA-256, while a chunk-size assertion proves no yielded chunk equals the whole array representation.

Owners and source paths use UTF-8 byte order. Violation closure uses the full byte-sorted identity `(path, code, severity, message)` and is invariant under reversed input order. Manifest, payload, ledger, counts, boundaries, duplicate identities, available shard closure, and reconstructed inventory digest all fail closed.

## Publication boundary

Each immutable content-addressed payload is staged, byte/digest verified, and renamed into place before publication. Every declared payload is verified again, then one staged manifest rename is the visibility commit. Only after that commit are prior unreferenced digest directories removed. Retained staging evidence, digest collisions, symlinks, malformed directory contents, missing payloads, and unreferenced payloads block publication.

The inventory progress protocol includes `write-shards` start, per-shard progress, and manifest completion. No full inventory was run at this checkpoint.

## Root CLI closure

Authority-bearing plan, resume, and cancellation paths are rejected lexically before ticket or inventory access when they leave the repository, target either opaque prefix, traverse a symlink, or cross a non-directory ancestor. Operation-inapplicable options fail rather than being ignored.

Plan Markdown and console output expose all seven operation groups: moves, embedded ticket roots, relocations, symlink target edits, evidence removals, edits, and regenerations. Apply evidence is written before terminal handling, but any terminal state other than `committed` now throws and produces a nonzero process status.

## Independent evidence

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️inventory-artifact-shards.test.ts'
7 pass
0 fail
49 expect() calls
```

The suite includes `fast-glob` publication parity, WebCrypto SHA-256 parity, child-process exit-status checks, opaque-path early failure, and option closure. A root bundle check completed with 11 modules in 46 ms and produced the retained 3.14 MB `🧪️root-sharded-inventory-build.js` evidence artifact.

An independent rerun after the Cargo/schema checkpoint remained green: `7 pass`, `0 fail`, `49 expect()` calls in 21.08 seconds.
## Portable Budget Rerun

The ticket test now resolves root imports and the repository from `import.meta.dir`; no executable test embeds the coordinator's macOS checkout path. The real-root opaque CLI child was removed because equivalent no-follow/opaque behavior is already proved against an isolated fixture, respecting the explicit Compose exclusion. Three redundant Bun subprocesses were also removed after the same exported committed-state boundary was already exercised directly; the focused gate fell from 19.39 seconds to 6.06 seconds without dropping a behavioral assertion class.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️inventory-artifact-shards.test.ts'
7 pass
0 fail
44 expect() calls
6.06s
```
