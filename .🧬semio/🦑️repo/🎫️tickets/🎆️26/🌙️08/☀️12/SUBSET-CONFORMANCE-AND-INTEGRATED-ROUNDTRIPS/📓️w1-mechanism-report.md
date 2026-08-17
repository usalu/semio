# W1 Mechanism Gate

| Task | Status | Evidence |
|------|--------|----------|
| W1-MACRO `subset!` | landed | `🔌️plugin/🦀️component.rs` + `📓️w1-macro.md` |
| W1-HARNESS | landed | `store::test_support::{SubsetRoundtripSpec,assert_subset_roundtrip}` |
| W1-IOFID | landed | `io::{IoFidelityClass,IoFidelityDeclaration,list_registered_subset_validator_dialects}` |
| W1-TAX | landed | taxonomy ownership flip + test literals + discovery types |
| W1-GEN | landed | `generate plugin-glue` dry-run works |
| W2-POL | landed medium | 676 medium breaches baseline in `scratch-w2-policy-baseline.txt` |
| W2-LAUNCH | landed | gates 410.11–410.14 |

## G1 decision
**PASSED with caveats**: framework compile of `subset_macro` still blocked by pre-existing E0499 in plugin component (UCAS/SMO churn). Harness/types are in-tree. Proceed to references on SMO-RELEASED plugins; stdio references proceed additively under UCAS awareness.
