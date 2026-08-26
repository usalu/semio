# H-GENERATOR-UNKNOWNS — resolution ledger

## Current boundary

After generator-owner repair the registry has 14 owned, 3 external, 3 unknown, and zero unsafe contracts. The remaining unknowns are not equivalent and must not be solved with one broad exception.

## `ownerless-ui-icons`

The ignored duplicate root `🧰️framework/🔨️modules/🖱️ui/🖼️assets/🔣️icons/🤖️generated` had no live include consumer. The apparent four hits resolved to one already-canonical `#[path]`, one already-canonical OS canvas `#[path]`, and two stale UI-contract doc paths. The canonical assets generator owns `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated`.

Resolution evidence:

1. The duplicate and canonical Rust files were not byte-identical because the duplicate was stale; the canonical file includes the current `FromStr` implementation and lint contract.
2. Both stale doc paths now point to the canonical generated root; both actual Rust includes already did.
3. `rg` now finds the duplicate root only in its taxonomy generator contract.
4. The ignored duplicate was moved recoverably to `/Users/ueli/.Trash/semio-ownerless-ui-icons-2026-08-26` after the owned asset freshness target had passed in the generator-owner repair lane.
5. Remaining action: delete the unknown contract; do not adopt a second owner for the same icon catalog.

## `root-layering-declarations`

`package.json`, `Cargo.toml`, and `go.work` are fixed shared workspace manifests, but only the Bun workspaces array has an existing deterministic writer. Their current grouping as one unknown generated output is false ownership.

Resolution must separate concerns:

- Normalization remains the single structured-reference writer for all three manifests during the transaction.
- Cargo and Go metadata are authored fixed contracts followed by `cargo metadata --locked` and `go work edit -json`; they are not generator outputs.
- The Bun workspace writer may remain a post-apply freshness target, but it may become a generator contract only after the read-only exact preview protocol exists. Until then it must not cause a guessed regeneration.
- Remove the three IDs from `layeringGeneratedContractIds` when the final layering baseline is recalculated; generated classification must reflect an actual owner, not merely suppress legitimate area references.

## `setup-wizard-config`

`.ralph-tui/config.toml` declares Ralph TUI setup-wizard provenance and Ralph owns its discovery path. It should become an exact external fixed-filename contract plus an external generator classification, not an unknown repository generator. Adjacent Ralph files and PRD identifier directories require their own exact tool-governed contracts; `.ralph-tui/**` must not become a second opaque subtree because Compose is the only authorized exclusion.

## Gate

Final strict load has zero `unknown` and zero `unsafe` generator contracts. External entries retain authority/reason/evidence, never run through normalization, and block only when a proposed move contradicts their exact fixed contract.
