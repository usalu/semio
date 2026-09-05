# Keycap Identity Validation

The taxonomy loader rejected the existing `1️⃣standard-1` and `2️⃣standard-2` directories because its emoji grammar admitted only Extended Pictographic code points. The path statute and the independent installed emoji-regex oracle already recognize each keycap as one emoji.

A twelve-case language-neutral fixture now covers numeric, hash, and asterisk keycaps; existing pictographic and joined identities; bare digits, missing presentation, stacked identities, and isolated selectors. The regression was observed failing specifically for `1️⃣` before changing the validator. A shared canonical identity predicate admits the explicit keycap sequence alongside the existing pictographic grammar. It does not select or rename anything.

Exact-member inventory validation still recognizes the first prefix of existing entries so the current corrupted names can be loaded and diagnosed. It does not exempt those entries from the separate single-emoji path statute: stacked names remain violations. Tightening inventory loading before repairing those physical owners caused the unrelated existing stacked glTF entries to prevent every catalog load; that attempted tightening was removed immediately.

Test evidence lives under `🗑️generated/keycap-red.txt` and `🗑️generated/keycap-green.txt`. The initial command without a relative-path prefix discovered no tests; it was corrected, and only the subsequent executed red/green runs are evidence.

Final executed suite: **22 passed, 0 failed, 507 assertions**. The independent oracle compares presentation-folded Unicode identities, matching the repository's selector-insensitive identity rule while separately requiring explicit presentation in taxonomy declarations.
