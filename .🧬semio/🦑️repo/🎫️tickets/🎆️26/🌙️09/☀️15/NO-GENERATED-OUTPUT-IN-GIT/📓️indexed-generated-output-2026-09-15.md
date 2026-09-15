# Indexed Generated Output Enforcement

## Objective

Ensure Git never indexes wasm binaries, wasm-bindgen/jco JS glue, wasm-pack `🕸️bindings/` trees, or fixture `🌐️browser-bundles/` materialization.

## Changes

- `.gitignore`: `**/*_bg.wasm.d.ts`, `**/🕸️bindings/**`, and fixture-scoped re-ignore for `🌐️browser-bundles/` and `🤖️generated/` after the fixtures rescue block.
- Policy: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/indexed-generated-output/🟦️.ts` plus gate hook in root `📜️script.ts` `runGate`.
- Index cleanup: `git rm --cached` on all paths matching the policy outside ticket evidence and the frame-worker `🤖️generated/🟨️.js` allowlist.

## Verification

```bash
bun ./📜️script.ts verify
# or the focused law test once wired in the repo library test bundle
```

Post-check: `git ls-files '*.wasm'` and `policyIndexedGeneratedOutputViolations(repoRoot)` should both be empty outside the allowlist.

## Follow-up

jcoprobe `🌐️browser-bundles/` remain on disk as ignored build output; fresh clones must materialize them via the owning jco/wasm pipeline before browser/jcoprobe tests (no committed bytes).
