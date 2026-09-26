# Fix — EN 1999 Round-4 fingerprint gaming rejection

## Trigger

Coordinator: round-4 close **not accepted**. Durable `en1999.en1990.psi.*` Pass fingerprints violated **CORRECTION 14:37** / ADDENDUM 14:37 (perturbation gaming: fingerprint folds so editable leaves move the report without normative effect).

## Changes

1. **Removed** the Pass fingerprint block in `push_action_field_fails` (`🧬️schema/🦀️.rs`) that encoded ψ₀/ψ₁/ψ₂ + kind + source into `computed`.
2. **Added** normative connection frequent SLS check `en1999.8.sls-freq.{id}` using `sls_connection_effects` so `connections[].actions[].category` changes ψ₁-scaled forces (status/computed/limit/utilization signature) when ULS 6.10b ignores ψ₀ on the lead variable.

## Non-changes

No re-introduction of `fingerprint`, `1e-9 *` field folds, or explanation-only perturbation signatures.

## Evidence

- Perturbation: `every_editable_leaf_influences_a_check` PASS after fingerprint removal + SLS wiring.
- Full suite results recorded in `📓️impl-en1999.md` Runner table.
