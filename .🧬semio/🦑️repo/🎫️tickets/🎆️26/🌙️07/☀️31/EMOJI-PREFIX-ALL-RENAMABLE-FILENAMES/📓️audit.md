# Emoji Prefix Policy Audit

## Scope

The configured repo MCP is unavailable in this session, so work continues in the existing open ticket `26/07/31/EMOJI-PREFIX-ALL-RENAMABLE-FILENAMES`.

The enforceable surface is every clean taxonomy area. Ecosystem-owned exact names, dot trees, generated output, Git hooks, Go workspace files, and Next route trees are not renamable. Standards, subsets, and inferences retain their structural prefix conventions.

## Baseline

- The prior `taxonomy/emoji-prefix` rule inspected directories only.
- It only reported missing U+FE0F on entries that already had a prefix.
- It did not inspect files, did not report absent prefixes, and did not enforce sibling uniqueness.
- The clean-area audit found 81 unprefixed renamable entries: 50 Energy engine directories and 31 stdio example assets.
- The existing repo policy command has substantial unrelated breach output; targeted checks use the exported rule to avoid conflating that baseline with this ticket.

## Decision

The policy now distinguishes renamable names from externally fixed names, checks files and directories, normalizes U+FE0F before comparing sibling identities, and traverses only declared clean areas. Tests cover missing prefixes, conventional filename exemptions, normalized collisions, and valid unique siblings.

