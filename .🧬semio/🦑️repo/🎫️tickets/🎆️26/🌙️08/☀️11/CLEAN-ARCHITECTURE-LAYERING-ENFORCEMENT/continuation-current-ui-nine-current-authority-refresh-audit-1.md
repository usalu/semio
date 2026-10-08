# Current UI Nine Source Authorities

UI8laterRootdrift retained. Fresh9 carriescompletecaptured14before/currentafter bodies/hashes for1renewed authorities, five total completecurrenttransitions. Same166040fixturepair/owners/caps/stacks/threshold/assertions unchanged. No ABI attribution/physicalsizeacceptance.

```diff
--- UI8-🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs
+++ UI9-🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs
@@ -430,7 +430,7 @@
                     released_items += 1;
                 }
                 RetirementStep::BudgetExhausted => {
-                    if released_items == 0 && released_bytes == 0 && cursor.next_close_byte_demand().is_some_and(|demand| demand > maximum_bytes) {
+                    if maximum_bytes != 0 && released_items == 0 && released_bytes == 0 && cursor.next_close_byte_demand().is_some_and(|demand| demand > maximum_bytes) {
                         return Err(crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "owned retirement byte grant is smaller than its next physical release"));
                     }
                     break;
```

ExactGUI273001–002/plannedreceipt 🗑️generated/current-native-ui-layout-9/green/admission.json. Actualsourcecontrols next,noRootwrite/compiler.
