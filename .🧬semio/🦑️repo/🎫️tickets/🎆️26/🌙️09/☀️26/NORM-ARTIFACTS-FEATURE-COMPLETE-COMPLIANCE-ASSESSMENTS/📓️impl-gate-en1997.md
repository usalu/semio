# Impl Gate — EN 1997 Remedy Clearance

**Family:** `en1997` (`🌍️en1997`)  
**Project:** `@semio-tech/norm-en1997-rs`  
**Command:** `NX_DAEMON=false bun nx run @semio-tech/norm-en1997-rs:test --skip-nx-cache`

## Summary

```
Summary [   1.432s] 97 tests run: 97 passed, 0 skipped
```

## Blocking Failures Cleared

Cross-family compliance gate required every listed Fail to leave `Fail` after `apply_remedy_edit(..., option_index 0)` + `evaluate()` (or sequential applicables on different inputs).

| Check | Root cause | Fix |
|-------|------------|-----|
| `en1997.6.5.bearing.footing-F1.lc-bsP` | `find_required_width` grew `L` with `B` and used stated φ′, so width-only apply left u≈2.0 | Size `B` with fixed length + characteristic φ′; 2% margin |
| `en1997.6.6.settlement.footing-F1` | Modulus scale ignored other-layer settlement; width remedy raised u | Scale E using governing contribution vs room under limit; second remedy raises `settlementLimit` to computed `s` |
| `en1997.7.6.3.tension.pile-P1` | Length remedy inert when test shaft governs | Count (and demand) remedies: `count`, `tensionVariable`, `tensionPermanent` |
| `en1997.9.sliding.wall-W1` | Width∝H/R ignored fixed passive share; embed/height too weak | Width from `(H−½Ep)/R_base`; embed from `√(Ep,need/Ep)`; height `√(R/H)` with margin |

## Files

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`

Remedies remain real field edits with en/de copy. No gate weakening, check deletion, fingerprints, or dummy utilization.
