# JCO Generator Input Roster Ordering

Actual Host/adapter Source loading refused the current jco-package-adapter generator contract because inputPatterns was not in the discovery owner's exact JavaScript lexical order. Root inspected the actual validator (unique strings plus Array.sort order) and hand-edited only the ten-item roster order. No input was added/dropped, no target/output/config field changed, no generator or Native operation ran. Python independently parsed before/after and checked complete JSON semantic equality except the exact reordered field, with unchanged unique input set. Existing focused Host Source/strict successor will validate actual imports; no all-repository success claimed here.

Current taxonomy SHA-256 `38df28d508a1c00e4c8aff0ea9096f0bb218adad7aa6ecc8bd42738591aab3ca`
