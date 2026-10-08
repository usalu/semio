# Current UI Six Retirement Authority Refresh

Source18 invocation wrapper refused before a GUI child, tree or plan was created. UI5 remains immutable. The sole current source binding drift is `🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs`, captured14 SHA256 `d773ad671c5f71eec7b7fc32c826fa4c47fd665604741c91ce2e85ae213c9e0d` to actual current Root SHA256 `a61a26d7f86a163f40b3c713ac4128a853db56d0bdd4418371bac413c357d16e`.

The successor proposal carries complete before/after UTF8 bodies and hashes for this fourth current authority transition. All prior three complete transitions and the complete fixture pair are conserved: only the measured UiSurfaceSlot descriptor165984→166040, capacity64/owner520 and original stacks/threshold/assertions unchanged. The fresh Root source does not establish native size or field contribution. physicalLayoutAccepted remains false.

```diff
--- captured14-initial-retirement
+++ current-retirement
@@ -33,6 +33,9 @@
 
 /// 🧮️ Measures the exact inline ownership retained by one deferred field Box.
 pub const fn deferred_birth_bytes<T: RetireOwned>() -> usize { size_of::<Deferred<T>>() }
+
+/// 🧮️ Borrows the exact field type when a declarative owner measures its deferred scaffold.
+pub fn deferred_birth_bytes_for<T: RetireOwned>(_: &T) -> usize { deferred_birth_bytes::<T>() }
 
 /// 🧮️ Measures the exact scalar retirement scaffold allocation.
 pub const fn leaf_birth_bytes<T: Copy + Send + 'static>() -> usize { size_of::<Leaf<T>>() }
@@ -87,8 +90,12 @@
         impl $crate::retirement::RetireOwned for $type {
             fn retirement(self) -> Box<dyn $crate::retirement::RetirementCursor> {
                 let Self { $($field),+ } = self;
-                $crate::artifact_retirement_sequence![$($field),+]
-            }
+                $crate::retirement::sequence(vec![$($crate::retirement::deferred($field)),+])
+            }
+            fn retirement_birth_bytes(&self) -> Option<usize> {
+                $crate::retirement::sequence_birth_bytes(&[$($crate::retirement::deferred_birth_bytes_for(&self.$field)),+])
+            }
+            fn controlled_retirement_supported() -> bool { true }
         }
     };
 }
```

Helper current-native-ui-layout-inputs-6/📜️script.ts; GUI900.249001–002. Planned exact receipt 🗑️generated/current-native-ui-layout-6/green/admission.json. Original 21 own/JSON5 and5 own/AJV controls will execute for the fresh complete authority context before Low admission.
