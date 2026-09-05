# UI Contract Taxonomy Path Repair

Date: 2026-09-04

## Outcome

The canonical UI-contract owner is green again after concurrent taxonomy renames. The repair is deliberately limited to physical ownership and deterministic generated output:

- the root module now mounts `🦀️🧩️component.rs`;
- the fixed-list source and test mounts now use `📋️🟠️list`;
- the owner script reads the renamed fixed-list and component-copy fixtures;
- component-copy and typed-retirement Rust laws read their renamed `🧪️🧫️fixture` paths;
- Cargo declares the emoji-named exporter source under the portable ASCII test target `typegen_export`;
- the checked-in TypeScript mirror was regenerated from the current 79-type Rust schema.

No compatibility path or duplicate source tree was added.

## Evidence

- `bun nx run @semio-tech/ui-contract-rs:generate --skip-nx-cache`: session `7701`, exit `0`; the exact exporter law passed `1/1` and refreshed the mirror.
- `bun nx run @semio-tech/ui-contract-rs:check --skip-nx-cache`: session `17553`, exit `0`; the mirror is byte-fresh.
- `bun nx run @semio-tech/ui-contract-rs:test-quick --skip-nx-cache`: session `48590`, exit `0`; `160/160` tests passed after the script-owned neutral fixed-list oracle reported `75` checks.

The earlier failed runs were useful ordered diagnostics: stale fixed-list source path, missing component module path, implicit nonportable emoji Cargo target name, stale fixture paths, then generated-mirror drift. Each later run advanced past the prior frontier.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️💎️action.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️copy/🧪️bytes.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️copy/🧪️🧪️🏔️🦋️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️🧪️🏔️🦋️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️📃️document.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️ui-contract.ts`
