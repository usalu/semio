# Runtime Component Copy R20 RED

Actual canonical single runtime test: 0 passed,1 failed,92 skipped,0.030s. The real fresh-record Component field allocated32KiB but reported/charged zero. Both cursor and current reconciler were retired before assertion. No allowance or retirement guard was changed.

```text
15:[DEBUG] surface-ownership-oracle checks=16
23:[DEBUG] surface-component-copy turns=1 reported=0 ledger-allocation=0 actual-allocation=32768
25:thread 'reconcile::tests::ownership::surface_ownership_component_copy_charges_actual_surface_backing_before_publication' (5375478) panicked at 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../📏️ownership/🧪️component.rs:68:5:
27:  left: 0
28: right: 32768
50:     Summary [   0.030s] 1 test run: 0 passed, 1 failed, 92 skipped

```
