# Remodel row semantics

Diff schema/🔺️diff/🦀️.rs:356–387 clones the base and processes entries in input order. Insert rejects any matching key then uses partition_point, assuming sorted input; replace, remove and patch address the first matching key and refuse missing target with indexed entity/key paths. No base uniqueness or ordering validation is performed. Patch errors are nested under rows/index/patch. NoPatch entities accept a patch as a no-op when the key exists. Nested members remove first equal and add at partition point, with their own ordering.

Inverse rows are built independently against the original base and sorted by key, not applied in reverse sequence (lines 408–422). Preserve actual behavior even for uncoalesced repeated keys. Neutral cases should include duplicate and unsorted base, every missing op, duplicate insert, same-key sequential operations, NoPatch existing/missing, nested missing or duplicate members, and equal-rank insertion. No runtime execution.
