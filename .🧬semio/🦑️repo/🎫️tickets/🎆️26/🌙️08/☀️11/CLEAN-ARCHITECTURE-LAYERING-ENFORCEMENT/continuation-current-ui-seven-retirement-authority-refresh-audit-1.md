# Current UI Seven Retirement Authority Refresh

Current retirement authority advanced UI6a61a… to `3c528db24b7ac190d90a9d00571429415650108748b55b0f0220664fc9bd2f28`. UI7 carries complete captured14before/currentafter full bodies/hash, retaining otherthree currenttransitions and same166040descriptor fullfixturepair. Alloriginalassertions/capacities/owners/stacks/threshold conserved; physicalLayoutAccepted remainsfalse, freshnativewholemandatory.

```diff
--- UI6-currentRetirement
+++ UI7-currentRetirement
@@ -181,13 +181,30 @@
     fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
 }
 
-struct DequeCollection<T:RetireOwned>{values:ManuallyDrop<std::collections::VecDeque<T>>,backing_bytes:usize}
-impl<T:RetireOwned> RetirementCursor for DequeCollection<T>{
-    fn close_step(&mut self,maximum_bytes:usize)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete}if maximum_bytes==0{return RetirementStep::BudgetExhausted}if let Some(value)=self.values.pop_back(){return RetirementStep::Child(value.retirement())}let released=maximum_bytes.min(self.backing_bytes);self.backing_bytes-=released;RetirementStep::Bytes(released)}
-    fn terminal_is_empty(&self)->bool{self.values.is_empty()&&self.backing_bytes==0}
-}
-impl<T:RetireOwned> Drop for DequeCollection<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned deque retired before terminal-empty");unsafe{ManuallyDrop::drop(&mut self.values)}}}
-impl<T:RetireOwned> RetireOwned for std::collections::VecDeque<T>{fn retirement(self)->Box<dyn RetirementCursor>{let backing_bytes=self.capacity()*std::mem::size_of::<T>();Box::new(DequeCollection{values:ManuallyDrop::new(self),backing_bytes})}}
+struct DequeCollection<T: RetireOwned>(ManuallyDrop<std::collections::VecDeque<T>>);
+impl<T: RetireOwned> RetirementCursor for DequeCollection<T> {
+    fn close_step(&mut self, _: usize) -> RetirementStep {
+        self.0.pop_front().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
+    }
+    fn terminal_is_empty(&self) -> bool { self.0.is_empty() }
+    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
+        self.0.front().map_or(Some(0), RetireOwned::retirement_birth_bytes)
+    }
+    fn terminal_release_bytes(&self) -> Option<usize> {
+        size_of::<Self>().checked_add(self.0.capacity().checked_mul(size_of::<T>())?)
+    }
+}
+impl<T: RetireOwned> Drop for DequeCollection<T> {
+    fn drop(&mut self) {
+        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned deque retired before terminal-empty");
+        unsafe { ManuallyDrop::drop(&mut self.0) }
+    }
+}
+impl<T: RetireOwned> RetireOwned for std::collections::VecDeque<T> {
+    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(DequeCollection(ManuallyDrop::new(self))) }
+    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<DequeCollection<T>>()) }
+    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
+}
 
 struct VectorIterator<T:RetireOwned>(ManuallyDrop<std::vec::IntoIter<T>>);
 impl<T:RetireOwned> RetirementCursor for VectorIterator<T> {
```

Exact GUI258001–002. Plannedreceipt 🗑️generated/current-native-ui-layout-7/green/admission.json. Allpriorreceipts/refusals retained,noRootwrite/compiler.
