# Exact Producer Receipt Adoption

The private UI publication acknowledgement now captures the actual native source's `ActorUiPatchReceipt`, including its exact lifetime and producer-issued patch sequence. It does not derive a sequence from revision, ordinal, or instance identity. The receipt and nested lifetime are frozen; the constructor checks the receipt lifetime against the aggregate owner.

The owned native test producer now emits canonical receipt bytes on each original returned patch, starting at sequence 51 independently of revision 1. No fallback was added to production intake.

## Executed Evidence

- Canonical React `test-long --args='--run -t OwnedInstance'`, receipt R1: 8 passed, 1 failed, 615 skipped. The actual failing assertion observed the missing UI receipt.
- Same target, receipt R2: 9 passed, 615 skipped, 624 total; exit 0, 15.39 seconds. The exact producer receipt, independent sequence, and frozen ownership assertions executed.
- Full outputs: `🧪️renderer-owned-instance-receipt-r1-2026-08-27.txt` and `🧪️renderer-owned-instance-receipt-r2-2026-08-27.txt`.

The new owned scene JSON parser remains an intentional constructor-missing TDD boundary. This focused pass does not certify full-suite collection/typechecking or live renderer adoption. Original output/raw-wrapper ownership remains with the dedicated native lifecycle source.
