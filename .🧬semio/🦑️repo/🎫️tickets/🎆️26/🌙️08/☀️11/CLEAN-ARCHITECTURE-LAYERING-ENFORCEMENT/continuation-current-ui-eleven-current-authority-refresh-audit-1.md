# UI Eleven Current Authority Renewal

UI10later 3Rootdrifts requirefreshcompletecurrentbeforeafter sourceauthority transitions, same166040fixturepair/assertions/owner/caps/stacks/threshold. No physicalsizeinvariance/nativeacceptance, exactGUI276001–002 next.

```diff
--- UI10-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
+++ UI11-🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs
@@ -299,6 +299,7 @@
     /// 📐️ Observes the actual controlled owner Box required by the next handoff.
     pub fn next_owner_capacity_byte_demand<T: RetireOwned>(&self, present: bool, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
         if !self.is_empty() { return self.next_capacity_byte_demand(maximum_release_bytes); }
+        if present && !T::controlled_retirement_supported() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "typed owner has no controlled close birth authority")); }
         Ok(if present { size_of::<crate::retirement::controlled::ControlledRetirement<T>>() } else { 0 })
     }
     /// 📐️ Observes the next controlled child birth without allocating or moving an owner.
@@ -480,6 +481,16 @@
         close_retained_binding(&mut self.source, grant)
     }
 
+    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
+        if !self.closing { return Ok(0); }
+        self.close.next_owner_capacity_byte_demand::<T>(self.value.is_some(), maximum_release_bytes)
+    }
+
+    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
+        if !self.closing { return Ok(0); }
+        self.close.next_release_byte_demand()
+    }
+
     fn terminal_is_empty(&self) -> bool {
         self.closing && self.value.is_none() && self.close.is_empty() && self.source.is_none()
     }
@@ -604,6 +615,16 @@
         if !self.close.is_empty() { return self.close.step_granted(grant); }
         if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
         close_retained_binding(&mut self.source, grant)
+    }
+
+    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
+        if !self.closing { return Ok(0); }
+        self.close.next_owner_capacity_byte_demand::<String>(self.output.is_some(), maximum_release_bytes)
+    }
+
+    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
+        if !self.closing { return Ok(0); }
+        self.close.next_release_byte_demand()
     }
 
     fn terminal_is_empty(&self) -> bool {
```
```diff
--- UI10-🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️.rs
+++ UI11-🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️.rs
@@ -283,7 +283,7 @@
 /// 🔪️ Splits `"<prefix>-<uuid>"` into `(prefix, 16 raw uuid bytes)`, requiring the
 /// trailing 36 bytes to be a canonical lowercase-hex-with-dashes UUID and a non-empty prefix
 /// — so the round trip through `format_uuid` reproduces the original text exactly.
-fn split_prefix_uuid(id: &str) -> Option<(&str, [u8; 16])> {
+pub fn split_prefix_uuid(id: &str) -> Option<(&str, [u8; 16])> {
     let len = id.len();
     if len < 38 {
         return None;
```
```diff
--- UI10-🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs
+++ UI11-🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs
@@ -39,7 +39,7 @@
 
 /// ✍️ Serializes the 32-byte header: magic, version, flags, `header_crc32` over bytes
 /// `0..20` (CRC-32C, `crate::codec::crc32c`), 8 reserved zero bytes.
-async fn build_header_bytes(required_flags: u32, optional_flags: u32) -> [u8; HEADER_SIZE] {
+pub fn build_header_bytes(required_flags: u32, optional_flags: u32) -> [u8; HEADER_SIZE] {
     let mut buf = [0u8; HEADER_SIZE];
     buf[0..8].copy_from_slice(&MAGIC);
     buf[8..10].copy_from_slice(&FORMAT_VERSION_MAJOR.to_le_bytes());
@@ -331,7 +331,7 @@
 }
 
 /// ✍️ Serializes a commit's fixed 64-byte payload per the contract's exact field offsets.
-async fn write_commit_payload(commit_seq: u64, prev_commit_offset: u64, records_len: u64, record_count: u32, chain_hash: &[u8; 32]) -> [u8; COMMIT_PAYLOAD_LEN] {
+pub fn write_commit_payload(commit_seq: u64, prev_commit_offset: u64, records_len: u64, record_count: u32, chain_hash: &[u8; 32]) -> [u8; COMMIT_PAYLOAD_LEN] {
     let mut buf = [0u8; COMMIT_PAYLOAD_LEN];
     buf[0..8].copy_from_slice(&commit_seq.to_le_bytes());
     buf[8..16].copy_from_slice(&prev_commit_offset.to_le_bytes());
@@ -452,7 +452,7 @@
         if unknown != 0 {
             return Err(ProtocolError::Pack(semio_framework_pack_error::PackError::Refusal(PackRefusal::UnknownRequiredFlags(unknown))));
         }
-        let header = build_header_bytes(options.required_flags, options.optional_flags).await;
+        let header = build_header_bytes(options.required_flags, options.optional_flags);
         sink.write_all(&header).await?;
         let chain_0 = *semio_framework_hash::hash(&header).as_bytes();
         let mut pending_chain_hasher = semio_framework_hash::Hasher::new();
@@ -543,7 +543,7 @@
 
         let commit_seq = self.next_commit_seq;
         let prev_commit_offset = self.last_commit_offset.unwrap_or(0);
-        let payload = write_commit_payload(commit_seq, prev_commit_offset, self.pending_records_len, self.pending_record_count, &chain_hash).await;
+        let payload = write_commit_payload(commit_seq, prev_commit_offset, self.pending_records_len, self.pending_record_count, &chain_hash);
         let flags = frame_flags(false, true, 0);
         write_frame_retained(&mut self.sink, crate::REC_COMMIT, flags, None, &payload).await?;
 
```

NoRootwrite/compiler/guardrelaxation.
