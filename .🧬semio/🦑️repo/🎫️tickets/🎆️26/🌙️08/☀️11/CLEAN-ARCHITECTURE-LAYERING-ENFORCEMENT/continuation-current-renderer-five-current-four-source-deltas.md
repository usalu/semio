# Renderer5 Current Four Authority Deltas

Read-only current Product11 initial-to-current body comparison. No physical layout acceptance or inferred native success.

## 🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs

Captured initial SHA 5c89dcb6ee5cdf6ad18bf1ecbee12a9e7f60aca7d025a5d7d805eab20e3d92e5; previous authority 5c89dcb6ee5cdf6ad18bf1ecbee12a9e7f60aca7d025a5d7d805eab20e3d92e5; current 5bc7a5153581aa804cb55c7a5a724d4a234b757b5a5a2953abfb8264f71176f5.

```diff
--- 
+++ 
@@ -36,7 +36,7 @@
             let demand = destination.chunks.next_allocation_bytes()?;
             if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
             let progress = destination.chunks.reserve_one(demand).map_err(|error| ValueError::from(error.refusal()))?;
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, copied_bytes: 0 }));
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, copied_bytes: 0, released_bytes: 0 }));
         }
         if self.pending.is_none() {
             let mut end = (self.position + PAGED_UTF8_CHUNK_BYTES).min(source.len());
@@ -50,7 +50,7 @@
             self.pending = Some(pending);
             if capacity > grant.maximum_capacity_bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append allocator exceeded its admitted chunk")); }
             self.end = end;
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity }));
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 }));
         }
         if self.position < self.end {
             let mut end = self.position.saturating_add(grant.maximum_copy_bytes).min(self.end);
@@ -59,7 +59,7 @@
             let bytes = end - self.position;
             self.pending.as_mut().expect("native append chunk owner").push_str(&source[self.position..end]);
             self.position = end;
-            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }));
+            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }));
         }
         let bytes = std::mem::size_of::<String>();
         if bytes > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
@@ -68,7 +68,7 @@
         if let Err(pending) = destination.chunks.push_reserved(pending) { self.pending = Some(pending); return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append lost its admitted slot")); }
         destination.byte_len += length;
         self.destination = Some((destination_identity, destination.byte_len));
-        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }))
+        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
     }
     pub fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; true }
     pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {

```

## 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs

Captured initial SHA a51897f27ba16072ae73c886834ea600371bbb2b03edd09191ba7b260c81201a; previous authority ce4e3e61f7f1581d2d3027b60da3e1cd755bcd396fef6d323899c1bdf0fb9238; current c0bb08da0a587142728c408a8382f4b8e7122fd79158d929872cf646bbbfd549.

```diff
--- 
+++ 
@@ -59,7 +59,7 @@
     /// 🔗️ Retains a projected native field through the actual immutable root lease.
     pub fn project_owned<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F) -> RetainedOwnedProjection<U>
     where F: for<'source> FnOnce(&'source T) -> &'source U {
-        RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project))
+        unsafe { RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project)) }
     }
 }
 
@@ -153,21 +153,27 @@
 #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
 pub struct RetainedCloneGrant {
     pub maximum_items: usize,
-    /// 🧮️ Bounds payload copying and completed-child scaffold release within one clone turn.
+    /// 🧮️ Bounds payload copying within one clone turn.
     pub maximum_copy_bytes: usize,
     pub maximum_capacity_bytes: usize,
+    /// ♻️ Bounds physical owner and scaffold release independently of copying.
+    pub maximum_release_bytes: usize,
     pub maximum_depth: usize,
 }
 
 impl RetainedCloneGrant {
     /// 🎟️ Admits one structural allocation while reserving no payload-copy credit.
     pub fn one_capacity_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
-        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: maximum_bytes, maximum_depth }
+        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: maximum_bytes, maximum_release_bytes: 0, maximum_depth }
     }
 
     /// 🎟️ Admits one payload turn while reserving no allocation-capacity credit.
     pub fn one_payload_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
-        Self { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_depth }
+        Self { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth }
+    }
+    /// ♻️ Admits one release turn without copy or allocation credit.
+    pub fn one_release_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
+        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: maximum_bytes, maximum_depth }
     }
 }
 
@@ -176,6 +182,7 @@
     pub copied_items: usize,
     pub copied_bytes: usize,
     pub retained_capacity_bytes: usize,
+    pub released_bytes: usize,
 }
 
 impl RetainedCloneProgress {
@@ -183,17 +190,18 @@
         Ok(Self {
             copied_items: self.copied_items.checked_add(other.copied_items).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone item progress overflow"))?,
             copied_bytes: self.copied_bytes.checked_add(other.copied_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone byte progress overflow"))?,
+            released_bytes: self.released_bytes.checked_add(other.released_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone release progress overflow"))?,
             retained_capacity_bytes: self.retained_capacity_bytes.checked_add(other.retained_capacity_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained clone capacity progress overflow"))?,
         })
     }
 
     pub fn fits(self, grant: RetainedCloneGrant) -> bool {
-        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
+        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes && self.released_bytes <= grant.maximum_release_bytes
     }
 }
 
 pub fn admit_retained_clone_progress(grant: RetainedCloneGrant, progress: RetainedCloneProgress, scope: &str) -> Result<RetainedCloneProgress, crate::ValueError> {
-    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained clone item, copy, or capacity grant"))) }
+    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained clone item, copy, capacity, or release grant"))) }
 }
 
 /// 🛡️ Checks both granted closure work and the child's exact terminal ownership witness.
@@ -343,7 +351,7 @@
         let Some(retirement) = self.controlled.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
         if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         if retirement.terminal_is_empty() {
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: self.controlled_bytes, retained_capacity_bytes: 0 };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: self.controlled_bytes };
             if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
             self.controlled = None;
             self.controlled_bytes = 0;
@@ -365,7 +373,7 @@
 pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
     match RetainedCloneBinding::close_one(binding, grant.maximum_items)? {
         SnapshotRetirementStep::Complete => Ok(RetainedCloneStep::Complete(Default::default())),
-        SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })),
+        SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes })),
         SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained binding close blocked")),
     }
 }
@@ -395,12 +403,12 @@
         if self.spent {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is spent"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         if self.value.is_some() {
             return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
         }
-        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0 };
+        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0, released_bytes: 0 };
         if !progress.fits(grant) {
             return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
         }
@@ -482,13 +490,13 @@
         if self.closing {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         let source = source.get();
         match self.phase {
             0 => {
                 let planned = source.len();
-                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
+                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                 if !progress.fits(grant) {
                     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                 }
@@ -525,7 +533,7 @@
                     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                 }
                 output.push_str(&source[start..end]);
-                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0 };
+                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0, released_bytes: 0 };
                 Ok(RetainedCloneStep::Progress(progress))
             }
             2 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
@@ -604,12 +612,12 @@
         if self.closing {
             return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         let source_value = source.get();
         if self.phase == 0 {
             let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
             if !progress.fits(grant) {
                 return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
             }
@@ -633,8 +641,7 @@
                 maximum_items: grant.maximum_items.saturating_sub(used.copied_items),
                 maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),
                 maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),
-                maximum_depth: grant.maximum_depth,
-            };
+                maximum_depth: grant.maximum_depth, maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(used.released_bytes) };
             if let Some(child) = self.child.as_mut() {
                 if self.child_value.is_some() {
                     if !child.terminal_is_empty() {
@@ -669,7 +676,7 @@
                 used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                 return Ok(RetainedCloneStep::Complete(used));
             }
-            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 {
+            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 && remaining.maximum_release_bytes == 0 {
                 return Ok(RetainedCloneStep::Progress(used));
             }
             let child = self.child.get_or_insert_with(T::retained_clone_cursor);
@@ -811,7 +818,7 @@
             return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
         }
         let first = self.source.is_none();
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         let source_value = source.get();
         if first {
@@ -957,7 +964,7 @@
         if self.output.is_some() {
             return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         if self.value.is_some() {
             if let Some(child) = self.child.as_mut() {
@@ -974,10 +981,10 @@
                 let bytes = size_of::<T::Cursor>();
                 if bytes > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                 self.child = None;
-                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }));
+                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }));
             }
             let capacity = size_of::<T>();
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
             if !progress.fits(grant) {
                 return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
             }
@@ -990,7 +997,7 @@
                 return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained box clone structural depth limit exceeded"));
             }
             let capacity = size_of::<T::Cursor>();
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
             if !progress.fits(grant) {
                 return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
             }
@@ -1070,7 +1077,7 @@
         if let Some(child) = self.child.as_mut() {
             if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
             if !child.terminal_is_empty() { let step = child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
-            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T::Cursor>(), retained_capacity_bytes: 0 };
+            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
             if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
             self.child = None;
             return Ok(RetainedCloneStep::Progress(progress));
@@ -1126,7 +1133,7 @@
                 if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is closing")); }
                 if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is spent")); }
                 if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
-                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
                 if self.draining {
                     match self.phase {

```

## 🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs

Captured initial SHA 8a8cdb14868ecfa11c62bdfc5091546738d302127bd663734b907a0ff5b777f9; previous authority 8a8cdb14868ecfa11c62bdfc5091546738d302127bd663734b907a0ff5b777f9; current 0cc53e00ed803ebeadf3e9b7b5d258bc1a68de97091097c77bcc21037098542e.

```diff
--- 
+++ 
@@ -259,7 +259,7 @@
                 if self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone cursor is closing")); }
                 if self.spent { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone cursor is spent")); }
                 if self.output.is_some() { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())); }
-                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
+                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                 source.bind(&mut self.source)?;
                 if self.draining {
                     return match self.phase { #(#drain_arms,)* _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct drain state is invalid")) };
@@ -414,7 +414,7 @@
                         if bytes > grant.maximum_copy_bytes { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                         self.output = Some(#construct);
                         self.phase += 1;
-                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0 }))
+                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
                     }
                     _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone state is invalid")),
                 }
@@ -479,7 +479,7 @@
                 if self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone cursor is closing")); }
                 if self.spent { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone cursor is spent")); }
                 if self.output.is_some() { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())); }
-                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
+                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                 source.bind(&mut self.source)?;
                 if self.variant.is_none() {
                     if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }

```

## 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs

Captured initial SHA bf3f7b8a889dda036627e3dc587b497421777cc35f8df3b47c7cdf1851a09910; previous authority bf3f7b8a889dda036627e3dc587b497421777cc35f8df3b47c7cdf1851a09910; current eb801a0a2baae4d7c221bdd3db03956d4845a64613e1bddcd55d033a87d89fba.

```diff
--- 
+++ 
@@ -58,7 +58,7 @@
         if self.closing {
             return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet retained clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         match self.phase {
             0 => {
@@ -86,7 +86,7 @@
                     for index in start..start + count {
                         self.bytes.push_reserved(*source.get(index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone source changed"))?).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "paged octet clone lost its reserved capacity"))?;
                     }
-                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0 }));
+                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0, released_bytes: 0 }));
                 }
                 if grant.maximum_items == 0 {
                     return Ok(RetainedCloneStep::Progress(Default::default()));
@@ -193,7 +193,7 @@
         if self.closing {
             return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         match self.phase {
             0 => match self.inner.advance(source.project(1, PagedUtf8::retained_chunks), grant)? {
@@ -226,7 +226,7 @@
                 }
                 self.output = Some(PagedUtf8::from_retained_chunks_cloned(self.chunks.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone lost its chunks"))?, source.get().len()));
                 self.phase = 3;
-                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), retained_capacity_bytes: 0 }))
+                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), retained_capacity_bytes: 0, released_bytes: 0 }))
             }
             3 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
             _ => Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is spent")),
@@ -421,7 +421,7 @@
         if self.closing {
             return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "paged map retained clone cursor is closing"));
         }
-        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
+        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
         source.bind(&mut self.source)?;
         match self.phase {
             0 => match self.inner.advance(source.project(1, PagedMap::retained_entries), grant)? {

```

