# Source Admission IO Implementation 51

## Changed Production Footprint

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
  - Added `inventoryTaxonomySources` and its bounded raw all-stage Git parser plus no-follow candidate observer.
  - Preserved `gitRows` unchanged for existing stage-zero reference and transaction callers.
  - The full inventory now rebuilds its admitted candidate map from the public admission observations, retaining only present file, directory, and symlink rows and treating `tracked-path-absent` as non-blocking.

No root script, package export, discovery source, taxonomy file, or actual Compose path was edited or read.

## Executed IO Gate

`bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-io-51/📜️script.ts` passed 27/27 in retained [run-mRKiqe](./🧪️source-admission-io-51/🧫️runs/run-mRKiqe/🔣️.json).

The first source run after the wrapper was added was red only for an undefined local taxonomy-path constant; the retained [run-ab8wre](./🧪️source-admission-io-51/🧫️runs/run-ab8wre/🔣️.json) records that exact implementation error. The next run exposed five opaque-input validation gaps, retained as [run-q4JQvw](./🧪️source-admission-io-51/🧫️runs/run-q4JQvw/🔣️.json); the final wrapper validates case-insensitive Compose segments before any probe and is green.

The green gate covers raw Git stage tuples, stale-index absence, hidden/root/build paths, declared ignored generator output, explicit ticket input, cancellation, symlink-ancestor rejection, conflict tuples, source-population agreement with full inventory, and first/final input hashes. It is a bounded fixture gate, not a whole-workspace census.
