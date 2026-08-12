# Window Taxonomy Audit

## Scope

The structural scope is every Rust window under `✏️s/🔌️plugins/<plugin>/🎛️apps/**/🪟️windows/<window>`.
The required capability facets are `🎬️actions`, `🪛️utilities`, and `🎚️options`. `🍱️panes` and
`🪀️widgets` remain allowed but optional because they describe optional composition, not capabilities every
window must expose.

## Baseline

- Windows: 119
- Windows already containing all required facets: 2
- Missing `🎬️actions`: 117
- Missing `🪛️utilities`: 115
- Missing `🎚️options`: 105
- Missing required facets in total: 337

Every audited window has a Rust `🦀️component.rs` leaf. A missing capability facet is therefore repaired
with `<facet>/🦀️component.rs`. The leaf is an intentionally empty module when the window currently has no
members for that capability; this versions the required directory without inventing behavior.

## Enforcement Design

`🔣️taxonomy.json` remains the single vocabulary source. `windowChildDirs` is the structural allowlist and a
new `windowRequiredChildDirs` completeness list declares the three mandatory facets. The taxonomy
self-validator requires the completeness list to be non-empty, unique, and a subset of the allowlist. The
root policy walks every `🪟️windows` directory below every discovered taxonomy owner's apps tree and emits a
high-priority `taxonomy/window-completeness` breach for each absent required facet.

