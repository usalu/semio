# Current Renderer8 Store Authority Refresh

Renderer7 immediate current admission is refused after Store advancement. This successor preserves the exact three-descriptor pair and45+5 finite laws, joining the complete captured11/current Store after/body/hash recipe. Physical sizes remain qualified measured14 proposals; Product16 later unrelated first failure does not individually accept them. Fresh current native whole remains mandatory.

Current Store `0b2942bf7b092048f1d046f1c487feb0a43febe32bb688572301c4575304281d`.

```diff
--- captured11Store
+++ currentStore
@@ -70,6 +70,9 @@
 
 #[path = "🧩️composition/🗄️durable-group/🦀️.rs"]
 pub mod durable_group;
+
+#[path = "🧩️composition/📬️publication/🤝️group/🦀️.rs"]
+mod member_group;
 
 // The `crate::os_dsl::DslArtifact`/`crate::os_dsl::DslOps` derive macros emit `::crate::os_store::ArtifactDsl`/`::crate::os_store::OpText`
 // paths (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `store` as an
@@ -348,6 +351,8 @@
     }
 
     fn terminal_is_empty(&self) -> bool { self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() }
+    fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map_or(1, |active| active.next_close_byte_demand()) }
+
 }
 
 impl<P, Mu: self::Mutation<P>> Drop for ArtifactHistoryReadRetirement<P, Mu> {
@@ -1533,6 +1538,29 @@
     Complete,
 }
 
+/// 📏️ Exact next physical release owned by an erased retirement, including its terminal cursor allocation.
+pub fn artifact_retirement_box_byte_demand(owner: &Box<dyn ErasedSnapshotRetirement>) -> usize {
+    if owner.terminal_is_empty() { std::mem::size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }
+}
+
+/// 📦️ Retires a nested payload and then its cursor box under separate, unchanged caller grants.
+pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
+    let Some(owner) = slot.as_mut() else { return Ok(SnapshotRetirementStep::Complete); };
+    if owner.terminal_is_empty() {
+        let extent = std::mem::size_of_val(owner.as_ref());
+        if maximum_items == 0 || maximum_bytes < extent { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+        drop(slot.take());
+        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent });
+    }
+    match owner.close_step(maximum_items, maximum_bytes)? {
+        SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
+        SnapshotRetirementStep::Pending { .. } => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "nested retirement exceeded its exact caller grant")),
+        SnapshotRetirementStep::Complete if owner.terminal_is_empty() => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
+        SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "nested retirement completed with a live payload")),
+        SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
+    }
+}
+
 struct ArtifactStoreVcsRetirement<P, Mutation> {
     vcs: std::mem::ManuallyDrop<Option<ArtifactVcs<P, Mutation>>>,
     active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
@@ -1589,27 +1617,15 @@
 impl<Mu: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreOperationRowsRetirement<Mu> {
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
         if maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
-        if let Some(active) = self.active.as_mut() {
-            return match active.close_step(1, maximum_bytes)? {
-                SnapshotRetirementStep::Complete if active.terminal_is_empty() => { self.active.take(); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) },
-                SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "staged operation retirement requires its terminal witness")),
-                step => Ok(step),
-            };
-        }
+        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, 1, maximum_bytes); }
         if let Some(row) = self.rows.pop() { *self.active = Some(self.factory.retire_owned(row)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
         if !self.rows.terminal_is_empty() { return self.rows.release_empty(maximum_bytes); }
-        if let Some(strings) = self.strings.as_mut() {
-            return match strings.close_step(1, maximum_bytes)? {
-                SnapshotRetirementStep::Complete if strings.terminal_is_empty() => { self.strings.take(); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) },
-                SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "staged operation string retirement requires its terminal witness")),
-                step => Ok(step),
-            };
-        }
+        if self.strings.is_some() { return artifact_retirement_box_close_step(&mut self.strings, 1, maximum_bytes); }
         if let Some(identity) = self.identity.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(identity)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
         Ok(SnapshotRetirementStep::Complete)
     }
     fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.identity.is_none() && self.strings.is_none() && self.active.is_none() }
-    fn next_close_byte_demand(&self) -> usize { self.active.as_ref().or(self.strings.as_ref()).map_or_else(|| self.rows.byte_demand(), |active| active.next_close_byte_demand()) }
+    fn next_close_byte_demand(&self) -> usize { self.active.as_ref().or(self.strings.as_ref()).map_or_else(|| self.rows.byte_demand(), artifact_retirement_box_byte_demand) }
 }
 impl<Mu> Drop for ArtifactStoreOperationRowsRetirement<Mu> {
     fn drop(&mut self) { assert!(std::thread::panicking() || (self.rows.terminal_is_empty() && self.identity.is_none() && self.strings.is_none() && self.active.is_none()), "staged operation rows dropped before bounded retirement"); if self.rows.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.rows); } } }
@@ -1722,20 +1738,7 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(active) = self.active.as_mut() {
-            return match active.close_step(maximum_items, maximum_bytes)? {
-                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
-                SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS retirement nested owner exceeded its exact close grant")),
-                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
-                SnapshotRetirementStep::Complete => {
-                    if !active.terminal_is_empty() {
-                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS retirement nested owner reported Complete without its terminal witness"));
-                    }
-                    drop(self.active.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-            };
-        }
+        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes); }
         let Some(vcs) = self.vcs.as_mut() else {
             self.phase = ArtifactStoreVcsRetirementPhase::Complete;
             return Ok(SnapshotRetirementStep::Complete);
@@ -1806,6 +1809,10 @@
 
     fn terminal_is_empty(&self) -> bool {
         self.vcs.is_none() && self.active.is_none()
+    }
+
+    fn next_close_byte_demand(&self) -> usize {
+        self.active.as_ref().map_or(usize::from(self.vcs.is_some()), artifact_retirement_box_byte_demand)
     }
 }
 
@@ -7874,6 +7881,7 @@
         reservation: ArtifactEnvelopeFieldReservation,
         cx: &mut semio_framework_job::StepContext<'_>,
     ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
+    fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic>;
     fn terminal_is_empty(&self) -> bool;
 }
@@ -7924,6 +7932,7 @@
 pub trait ArtifactOwnedHistoryEntryAuthority<T>: Send {
     fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
     fn take_value(&mut self) -> Option<T>;
+    fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic>;
     fn terminal_is_empty(&self) -> bool;
 }
@@ -8032,6 +8041,15 @@
 }
 
 impl<P, Mutation: Send> ArtifactOwnedSprMutationArrayAuthority<P, Mutation> {
+    fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
+        if let Some(active) = self.active.as_ref() {
+            let demand = active.next_close_byte_demand()?;
+            return Ok(if active.terminal_is_empty() { demand.saturating_add(std::mem::size_of_val(active.as_ref())) } else { demand });
+        }
+        if let Some(retirement) = self.retirement.as_ref() { return Ok(artifact_retirement_box_byte_demand(retirement)); }
+        if self.reservation.is_some() || self.target.value.is_some() { return Ok(0); }
+        Ok(self.values.as_ref().map_or(0, ArtifactStoreOperationRowOwners::byte_demand))
+    }
     fn try_new(
         operation: semio_framework_job::OperationId,
         generation: semio_framework_job::Generation,
@@ -8189,13 +8207,16 @@
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
         if let Some(active) = self.active.as_mut() {
+            if active.terminal_is_empty() {
+                let released_bytes = active.next_close_byte_demand()?.saturating_add(std::mem::size_of_val(active.as_ref()));
+                if maximum_bytes < released_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+                drop(self.active.take());
+                self.depth = 0;
+                self.scalar_entry = false;
+                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
+            }
             return match active.close_step(maximum_items, maximum_bytes)? {
-                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
-                    drop(self.active.take());
-                    self.depth = 0;
-                    self.scalar_entry = false;
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
+                SnapshotRetirementStep::Complete if active.terminal_is_empty() => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
                 SnapshotRetirementStep::Complete => Err(self.diagnostic("artifact-spr.mutation-array-close-false-terminal", 0)),
                 step => Ok(step),
             };
@@ -8208,17 +8229,7 @@
             *self.retirement = Some(self.mutation_factory.retire_owned(value));
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(retirement) = self.retirement.as_mut() {
-            let path = self.path;
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-retirement-fault", offset: 0, line: 0, column: 0, path })? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(self.diagnostic("artifact-spr.mutation-array-retirement-false-terminal", 0)),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| self.diagnostic("artifact-spr.mutation-array-retirement-fault", 0)); }
         if let Some(values) = self.values.as_mut() {
             if let Some(value) = values.pop() {
                 *self.retirement = Some(self.mutation_factory.retire_owned(value));
@@ -8422,6 +8433,12 @@
 }
 
 impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>> for ArtifactOwnedSprEditAuthority<P, Mutation> {
+    fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
+        if let Some(active) = self.active.as_ref() { return match active { ArtifactOwnedSprEditActive::Mutations { authority, .. } => authority.next_close_byte_demand(), _ => Ok(0) }; }
+        if let Some(retirement) = self.retirement.as_ref() { return Ok(artifact_retirement_box_byte_demand(retirement)); }
+        if self.value.is_some() || self.inverse.is_some() || self.forwards.is_some() { return Ok(0); }
+        Ok(self.strings[self.close_string..].iter().find_map(|value| value.as_ref().map(String::capacity)).unwrap_or(0))
+    }
     fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
         if cx.operation() != self.operation || cx.generation() != self.generation {
             return Err(self.diagnostic("artifact-spr.edit-stale", token.start));
@@ -8482,18 +8499,7 @@
             }
             return Ok(step);
         }
-        if let Some(retirement) = self.retirement.as_mut() {
-            let path = self.path;
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-retirement-fault", offset: 0, line: 0, column: 0, path })? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    self.terminal = true;
-                    Ok(SnapshotRetirementStep::Complete)
-                }
-                SnapshotRetirementStep::Complete => Err(self.diagnostic("artifact-spr.edit-retirement-false-terminal", 0)),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| self.diagnostic("artifact-spr.edit-retirement-fault", 0)); }
         if let Some(value) = self.value.take() {
             *self.retirement = Some(self.retirement_factory.retire_owned(value));
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
@@ -8510,11 +8516,11 @@
             let index = self.close_string;
             self.close_string += 1;
             if let Some(value) = self.strings[index].as_ref() {
-                if value.len() > maximum_bytes {
+                if value.capacity() > maximum_bytes {
                     self.close_string -= 1;
                     return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                 }
-                let released_bytes = value.len();
+                let released_bytes = value.capacity();
                 drop(self.strings[index].take());
                 return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
             }
@@ -8595,6 +8601,10 @@
 }
 
 impl<T: FromValue + Send + 'static> ArtifactOwnedHistoryEntryAuthority<T> for ArtifactRepositoryHistoryEntryAuthority<T> {
+    fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
+        if let Some(retirement) = self.retirement.as_ref() { return Ok(artifact_retirement_box_byte_demand(retirement)); }
+        Ok(if self.terminal { ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES } else { 0 })
+    }
     fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
         if cx.operation() != self.operation || cx.generation() != self.generation {
             return Err(self.diagnostic("artifact-envelope.history-entry-stale", token.start));
@@ -8631,29 +8641,14 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(retirement) = self.retirement.as_mut() {
-            let path = self.path;
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-entry-retirement-fault", offset: 0, line: 0, column: 0, path })? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    self.terminal = true;
-                    Ok(SnapshotRetirementStep::Complete)
-                }
-                SnapshotRetirementStep::Complete => Err(self.diagnostic("artifact-envelope.history-entry-retirement-false-terminal", 0)),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| self.diagnostic("artifact-envelope.history-entry-retirement-fault", 0)); }
         if let Some(value) = self.value.take() {
             *self.retirement = Some(self.retirement_factory.retire_owned(value));
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
         if self.raw_len != 0 {
-            if self.raw_len > maximum_bytes {
-                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
-            }
-            let released_bytes = self.raw_len;
             self.raw_len = 0;
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
         }
         self.terminal = true;
         Ok(SnapshotRetirementStep::Complete)
@@ -9334,6 +9329,13 @@
 }
 
 impl<T> OwnedSchemaBoundedArrayAuthority<T> {
+    fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
+        if let Some(decoder) = self.active_decoder.as_ref() {
+            let demand = decoder.next_close_byte_demand()?;
+            return Ok(if decoder.terminal_is_empty() { demand.saturating_add(std::mem::size_of_val(decoder.as_ref())) } else { demand });
+        }
+        Ok(self.active_retirement.as_ref().map_or(0, artifact_retirement_box_byte_demand))
+    }
     fn new(path: OwnedSchemaPath, retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>, decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>) -> Self {
         Self {
             path,
@@ -9442,34 +9444,24 @@
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
         if let Some(decoder) = self.active_decoder.as_mut() {
+            if decoder.terminal_is_empty() {
+                let released_bytes = decoder.next_close_byte_demand()?.saturating_add(std::mem::size_of_val(decoder.as_ref()));
+                if maximum_bytes < released_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+                drop(self.active_decoder.take());
+                self.entry_depth = 0;
+                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
+            }
             return match decoder.close_step(maximum_items, maximum_bytes)? {
-                SnapshotRetirementStep::Complete if decoder.terminal_is_empty() => {
-                    drop(self.active_decoder.take());
-                    self.entry_depth = 0;
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
+                SnapshotRetirementStep::Complete if decoder.terminal_is_empty() => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
                 SnapshotRetirementStep::Complete => Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-decoder-false-terminal", offset: 0, line: 0, column: 0, path: self.path }),
                 step => Ok(step),
             };
         }
-        if let Some(retirement) = self.active_retirement.as_mut() {
-            return match retirement.close_step(maximum_items, maximum_bytes) {
-                Ok(SnapshotRetirementStep::Complete) if retirement.terminal_is_empty() => {
-                    drop(self.active_retirement.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                Ok(SnapshotRetirementStep::Complete) => Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-retirement-false-terminal", offset: 0, line: 0, column: 0, path: self.path }),
-                Err(_) => Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-retirement-fault", offset: 0, line: 0, column: 0, path: self.path }),
-                Ok(step) => Ok(step),
-            };
-        }
+        if self.active_retirement.is_some() { let path = self.path; return artifact_retirement_box_close_step(&mut self.active_retirement, maximum_items, maximum_bytes).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-retirement-fault", offset: 0, line: 0, column: 0, path }); }
         if let Some(reservation) = self.reservation.take() {
             let values = self.values.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-reservation-ledger-missing", offset: 0, line: 0, column: 0, path: self.path })?;
             values.cancel_reservation(reservation).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-reservation-stale", offset: 0, line: 0, column: 0, path: self.path })?;
             return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
-        }
-        if (self.rejected.is_some() || self.values.as_ref().is_some_and(|values| !values.is_empty())) && maximum_bytes < ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES {
-            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
         if let Some(value) = self.rejected.take() {
             *self.active_retirement = Some(self.retirement_factory.retire_owned(value));
@@ -10863,17 +10855,12 @@
             && self.retirement.is_none()
     }
 
+    fn close_retained_retirement(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
+        artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.fresh-vcs-retirement-fault"))
+    }
+
     fn close_owned_snapshot(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
-        if let Some(retirement) = self.retirement.as_mut() {
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.fresh-vcs-retirement-fault"))? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(Self::diagnostic("artifact-envelope.fresh-vcs-retirement-false-terminal")),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return self.close_retained_retirement(maximum_items, maximum_bytes); }
         if let Some(vcs) = self.value.take() {
             self.retirement = Some(Box::new(ArtifactStoreVcsRetirement::new(vcs, self.initial_snapshot_factory.clone(), self.mutation_factory.clone())));
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
@@ -10891,16 +10878,7 @@
     }
 
     fn close_pending_history(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, OwnedSchemaDecodeDiagnostic> {
-        if let Some(retirement) = self.retirement.as_mut() {
-            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("artifact-envelope.history-retirement-fault"))? {
-                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
-                    drop(self.retirement.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(Self::diagnostic("artifact-envelope.history-retirement-false-terminal")),
-                step => Ok(step),
-            };
-        }
+        if self.retirement.is_some() { return self.close_retained_retirement(maximum_items, maximum_bytes); }
         if let Some(active) = self.active.as_mut() {
             let step = match active {
                 ArtifactEnvelopeFreshVcsActive::Snapshot { .. } => Ok(SnapshotRetirementStep::Complete),
@@ -11003,6 +10981,11 @@
 
     fn next_close_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
         if let Some(snapshot) = self.snapshot.as_ref() {
+            if snapshot.terminal_is_empty() {
+                let demand = std::mem::size_of_val(snapshot.as_ref());
+                if demand > self.maximum_close_byte_demand() { return Err(Self::diagnostic("artifact-envelope.snapshot-scaffold-over-admitted-maximum")); }
+                return Ok(demand);
+            }
             if !snapshot.terminal_is_empty() {
                 let demand = snapshot.next_close_byte_demand()?;
                 if demand > self.maximum_snapshot_close_byte_demand {
@@ -11011,14 +10994,27 @@
                 return Ok(demand);
             }
         }
-        if self.retirement.is_some() || self.active.as_ref().is_some_and(|active| !matches!(active, ArtifactEnvelopeFreshVcsActive::Snapshot { .. })) {
-            return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
+        if let Some(retirement) = self.retirement.as_ref() {
+            let demand = artifact_retirement_box_byte_demand(retirement);
+            if demand > self.maximum_close_byte_demand() { return Err(Self::diagnostic("artifact-envelope.fresh-vcs-close-demand-over-admitted-maximum")); }
+            return Ok(demand);
+        }
+        if let Some(active) = self.active.as_ref() {
+            let demand = match active {
+                ArtifactEnvelopeFreshVcsActive::Snapshot { .. } => 0,
+                ArtifactEnvelopeFreshVcsActive::Edits(authority) => authority.next_close_byte_demand()?,
+                ArtifactEnvelopeFreshVcsActive::Changes(authority) => authority.next_close_byte_demand()?,
+                ArtifactEnvelopeFreshVcsActive::Checkpoints(authority) => authority.next_close_byte_demand()?,
+                ArtifactEnvelopeFreshVcsActive::Alternatives(authority) => authority.next_close_byte_demand()?,
+            };
+            if demand > self.maximum_close_byte_demand() { return Err(Self::diagnostic("artifact-envelope.history-close-demand-over-admitted-maximum")); }
+            return Ok(demand);
         }
         Ok(0)
     }
 
     fn maximum_close_byte_demand(&self) -> usize {
-        self.maximum_snapshot_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
+        self.maximum_snapshot_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)
     }
 
     fn maximum_retained_close_bytes(&self) -> usize {
@@ -11039,21 +11035,24 @@
                 if maximum_bytes < demand {
                     return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                 }
-                let step = snapshot.close_step(maximum_items.min(1), demand)?;
+                let step = snapshot.close_step(maximum_items.min(1), maximum_bytes)?;
                 if !matches!(step, SnapshotRetirementStep::Complete) {
                     return Ok(step);
                 }
                 if !snapshot.terminal_is_empty() {
                     return Err(Self::diagnostic("artifact-envelope.snapshot-close-false-terminal"));
                 }
-            }
+                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
+            }
+            let released_bytes = std::mem::size_of_val(snapshot.as_ref());
+            if maximum_bytes < released_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
             drop(self.snapshot.take());
             if let Some(ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, .. }) = self.active.as_ref() {
                 let reservation = *reservation;
                 self.active = None;
                 self.snapshot_target.cancel_snapshot_reservation(reservation)?;
             }
-            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
+            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
         }
         let history = self.close_pending_history(maximum_items, maximum_bytes)?;
         if history != SnapshotRetirementStep::Complete {
@@ -11389,9 +11388,12 @@
                 ArtifactEnvelopeFreshRecordActive::String { .. } | ArtifactEnvelopeFreshRecordActive::Empty { .. } => Ok(0),
             };
         }
-        if self.pending_completed.is_some() || self.active_retirement.is_some() {
-            return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
-        }
+        if let Some(retirement) = self.active_retirement.as_ref() {
+            let demand = artifact_retirement_box_byte_demand(retirement);
+            if demand > self.maximum_close_byte_demand() { return Err(Self::diagnostic("vcs", "artifact-envelope.target-close-demand-over-admitted-maximum")); }
+            return Ok(demand);
+        }
+        if self.pending_completed.is_some() { return Ok(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES); }
         if self.target.vcs.is_some() {
             return Ok(0);
         }
@@ -11454,17 +11456,7 @@
             }
             return Ok(step);
         }
-        if let Some(retirement) = self.active_retirement.as_mut() {
-            let step = retirement.close_step(maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("vcs", "artifact-envelope.target-retirement"))?;
-            if matches!(step, SnapshotRetirementStep::Complete) {
-                if !retirement.terminal_is_empty() {
-                    return Err(Self::diagnostic("vcs", "artifact-envelope.target-retirement-false-terminal"));
-                }
-                drop(self.active_retirement.take());
-                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
-            }
-            return Ok(step);
-        }
+        if self.active_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.active_retirement, maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("vcs", "artifact-envelope.target-retirement")); }
         if !self.close_target_vcs()? {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
@@ -16158,6 +16150,11 @@
 
     pub fn share_current(&self) -> Arc<P> { Arc::clone(self.current.as_ref().expect("initialization retains its current projection")) }
 
+    /// 📏️ Publish the next retained allocation, including its terminal erased cursor box.
+    pub fn next_close_byte_demand(&self) -> usize {
+        self.close_active.as_ref().map_or(usize::from(!self.taken), |active| if active.terminal_is_empty() { std::mem::size_of_val(active.as_ref()) } else { active.next_close_byte_demand() })
+    }
+
     fn replace_current_alias(&mut self, current: Arc<P>) -> Arc<P> { self.current.replace(current).expect("initialization retains its current projection") }
 
     pub fn adopt_current_owned(&mut self, current: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Result<(), P>
@@ -16173,8 +16170,14 @@
         if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
         let Some(active) = self.close_active.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
         match active.close_step(maximum_items.min(1), maximum_bytes)? {
-            SnapshotRetirementStep::Complete if active.terminal_is_empty() => { drop(self.close_active.take()); Ok(SnapshotRetirementStep::Complete) }
+            SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
+                let bytes = std::mem::size_of_val(active.as_ref());
+                if maximum_bytes < bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
+                drop(self.close_active.take());
+                Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes })
+            }
             SnapshotRetirementStep::Complete => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "initialization displaced snapshot has no terminal-empty witness")),
+            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store initialization child exceeded its exact close grant")),
             step => Ok(step),
         }
     }
@@ -16349,17 +16352,8 @@
         if maximum_items == 0 {
             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
         }
-        if let Some(active) = self.close_active.as_mut() {
-            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
-                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
-                SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store initialization child exceeded its exact close grant")),
-                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
-                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
-                    drop(self.close_active.take());
-                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
-                }
-                SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store initialization child reported Complete without terminal-empty ownership")),
-            };
+        if self.close_active.is_some() {
+            return self.settle_current_retirement_step(maximum_items, maximum_bytes);
         }
         match self.close_phase {
             0 => {
@@ -17452,6 +17446,10 @@
 //#endregion 🧺️BatchSource
 
 //#region 🧩️MemberOneItemPublication
+#[path = "🧩️composition/📨️emission/📦️owned/🦀️.rs"]
+mod member_owned_batch;
+pub use member_owned_batch::{MemberStoreOwnedBatch, MemberStoreOwnedBatchRequest};
+
 /// 📨 Owned member wire, transferred without decoding or cloning at the composition boundary.
 #[derive(Debug)]
 pub struct MemberStoreOneItemWire {
@@ -17480,6 +17478,7 @@
     fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
     fn phase(&self) -> ArtifactStoreOneItemPublicationPhase;
     fn progress(&self) -> ArtifactStoreOneItemCheckpoint;
+    fn next_group_byte_demand(&self) -> usize;
     fn fault(&self) -> Option<&str>;
     fn retry(&mut self) -> bool;
     fn acknowledge(&mut self) -> bool;
@@ -17507,6 +17506,8 @@
     fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
         self.publication.progress()
     }
+
+    fn next_group_byte_demand(&self) -> usize { self.publication.next_group_byte_demand() }
 
     fn fault(&self) -> Option<&str> {
         self.publication.fault()
@@ -17706,6 +17707,7 @@
     preparation: Option<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>>,
     item_closing: bool,
     stage: Option<Box<ArtifactStoreBatchStage<P, Mutation>>>,
+    group_preparation: Option<member_group::MemberGroupPreparation>,
     retained_checkpoint: ArtifactStoreOneItemCheckpoint,
     receipt: Option<LaneItemReceipt>,
     attempts: u8,
@@ -17723,6 +17725,9 @@
 }
 
 impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation> {
+    /// 🎟️ Current private group copy or allocation demand, never accumulated credit.
+    pub fn next_group_byte_demand(&self) -> usize { self.group_preparation.as_ref().map_or(1, |group| group.next_byte_demand()) }
+
     pub fn operation(&self) -> semio_framework_job::OperationId {
         self.operation
     }
@@ -17787,7 +17792,7 @@
         let item = if self.item_closing { ArtifactStoreOneItemCheckpoint::default() } else { self.preparation.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default, |owner| owner.checkpoint()) };
         ArtifactStoreOneItemCheckpoint {
             cursor: staged.cursor,
-            completed_items: staged.completed_items.saturating_add(item.completed_items),
+            completed_items: staged.completed_items.saturating_add(item.completed_items).saturating_add(self.group_preparation.as_ref().map_or(0, |group| group.completed_items())),
             completed_bytes: staged.completed_bytes.saturating_add(item.completed_bytes),
             digest: if item.digest == [0; 32] { staged.digest } else { item.digest },
         }
@@ -17835,6 +17840,7 @@
         P: Send + Sync + 'static,
     {
         self.begin_close();
+        if self.group_preparation.is_some() { return Ok(SnapshotRetirementStep::Blocked); }
         if let Some(owner) = self.preparation.as_mut() {
             let step = owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
             if step != SnapshotRetirementStep::Complete {
@@ -17911,6 +17917,7 @@
             && self.preparation.is_none()
             && self.source.is_none()
             && self.stage.is_none()
+            && self.group_preparation.is_none()
             && self.authority.is_none()
             && self.authority_retirement.is_none()
             && self.receipt.is_none()
@@ -20465,6 +20472,7 @@
             preparation: None,
             item_closing: false,
             stage: None,
+            group_preparation: None,
             retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
             receipt: None,
             attempts: 0,
@@ -20512,6 +20520,7 @@
         P: Sync,
     {
         self.ensure_durable_group_idle()?;
+        if publication.group_preparation.is_some() { return Err(VcsError::ValidationFailed("group-preparing batch requires its exact common publication decision".into())); }
         if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
             return Ok(ArtifactStoreOneItemAdvance::Complete);
         }
@@ -20588,6 +20597,20 @@
                 let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its live authority".into()))?;
                 if stage.edit.id.is_empty() || stage.edit.id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || authority.base_applied_edit_count != self.applied_edit_ids.len() {
                     return Err(VcsError::ValidationFailed("domain-prepared batched candidate failed its live cursor authority".into()));
+                }
+                if let Some(edit_id) = self.batch_amend_target(publication.transaction.as_ref()) {
+                    let existing = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its open edit".into()))?;
+                    let capacity = existing.inverse.len().checked_add(stage.edit.inverse.len()).ok_or_else(|| VcsError::ValidationFailed("batched inverse capacity overflow".into()))?;
+                    if let Some(bytes) = existing.inverse.next_capacity_allocation_bytes(capacity).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
+                        if bytes > grant.maximum_bytes {
+                            return Err(VcsError::ValidationFailed("batched inverse backing cannot fit the admitted allocation grant".into()));
+                        }
+                        let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+                        if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
+                            return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
+                        }
+                        return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
+                    }
                 }
                 publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                 Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
@@ -20806,6 +20829,7 @@
         }
         if publication.stage.is_none() {
             let mut staged = Box::new(Self::empty_batch_stage(&authority));
+            staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
             staged.mutation_retirement = (*self.mutation_retirement_factory).clone();
             staged.snapshot_retirement = (*self.snapshot_retirement_factory).clone();
             if staged.mutation_retirement.is_none() || staged.snapshot_retirement.is_none() {
@@ -20817,11 +20841,15 @@
             publication.stage = Some(staged);
         }
         let inverse = &mut publication.stage.as_mut().expect("validated staged batch edit").edit.inverse;
-        if let Some(bytes) = inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
+        let backing = if publication.transaction_open { inverse.next_capacity_allocation_bytes(publication.footprint.work_items) } else { inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items) };
+        if let Some(bytes) = backing.map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
             if bytes > grant.maximum_bytes {
-                return Ok(ArtifactStoreOneItemAdvance::Blocked);
-            }
-            inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+                return Err(VcsError::ValidationFailed("batched inverse backing cannot fit the admitted allocation grant".into()));
+            }
+            let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
+            if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
+                return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
+            }
             return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
         }
         let owner = publication.preparation.as_mut().expect("validated batched fold preparation owner");
@@ -22245,13 +22273,17 @@
         let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(ReplayTurnBudget::operations(usize::MAX));
         let deadline = budget.turn_deadline(deadline_us);
         let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
-        let mut replayed = 0usize;
-        let step = replay.step(&self.envelope.vcs.edits, &mut || {
-            replayed += 1;
-            budget.turn_ends(replayed, deadline)
-        });
+        let step = replay.step_operations(&self.envelope.vcs.edits, budget.operations, &mut || budget.turn_ends(0, deadline));
         match step {
             Ok(ReplayStep::Pending(progress)) => {
+                #[cfg(debug_assertions)]
+                {
+                    static TURNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
+                    let turns = TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
+                    if turns >= 1024 && turns.is_power_of_two() {
+                        eprintln!("[DEBUG] deferred reprojection turn={turns} cursor={} operation={} done={} total={} settlementWork={:?} cleanup={} convergence={:?}", replay.cursor, replay.operation, progress.done, progress.total, replay.message_settlement.as_ref().map(|settlement| settlement.completed_work()), replay.replay_retirement.is_some(), replay.convergence_search);
+                    }
+                }
                 self.pending_reprojection = Some(pending);
                 Ok(Some(ReprojectionAdvance::Pending(progress)))
             }
@@ -25035,6 +25067,7 @@
     from: usize,
     prefix_until: usize,
     prefix_work_progress: bool,
+    operation_progress: bool,
     cursor: usize,
     operation: usize,
     schema: String,
@@ -25099,6 +25132,7 @@
             from,
             prefix_until: from,
             prefix_work_progress: false,
+            operation_progress: false,
             cursor: from,
             operation: 0,
             schema: schema.to_string(),
@@ -25224,13 +25258,28 @@
 
     /// ⏭️ Replays operations until the replay finishes or `deadline`, asked after every operation, answers `true`.
     pub fn step(&mut self, edits: &impl ReplayEdits<Mutation>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
+        self.step_with_operation_cap(edits, usize::MAX, deadline)
+    }
+
+    /// 🔢️ Caps folded mutations while retaining every structural deadline and owner check.
+    pub fn step_operations(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
+        if !self.operation_progress {
+            if self.progress.done != 0 { return Err(VcsError::ValidationFailed("operation-counted replay authority must be selected before work begins".into())); }
+            self.progress.total = self.progress.total.saturating_sub(u32::try_from(self.order.len().saturating_sub(self.from)).unwrap_or(u32::MAX));
+            self.operation_progress = true;
+        }
+        self.step_with_operation_cap(edits, maximum_operations, deadline)
+    }
+
+    fn step_with_operation_cap(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
+        if maximum_operations == 0 && !self.finished { return Ok(ReplayStep::Pending(self.progress)); }
+        let mut completed_operations = 0usize;
         while !self.finished {
             if let Some(retirement) = self.replay_retirement.as_mut() {
                 let maximum_bytes = retirement.next_close_byte_demand().max(4096);
                 let step = retirement.close_step(1, maximum_bytes).map_err(VcsError::InverseRefused)?;
                 if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                     if released_items > 1 || released_bytes > maximum_bytes { return Err(VcsError::ValidationFailed("replay cleanup exceeded its admitted grant".into())); }
-                    if released_bytes > 4096 { eprintln!("[DEBUG] replay cleanup exact capacity release bytes={released_bytes} admitted={maximum_bytes}"); }
                 }
                 if step == SnapshotRetirementStep::Complete {
                     if !retirement.terminal_is_empty() { return Err(VcsError::ValidationFailed("replay cleanup reported terminal with live owners".into())); }
@@ -25260,8 +25309,9 @@
                     if self.operation_preparation_factory.is_some() {
                         if !self.step_prepared_operation(edit, true, deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                         self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                         if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
-                        if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                        if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                         continue;
                     }
                     let index = self.operation;
@@ -25276,8 +25326,9 @@
                         if let (Some(next), _) = fold_operation::<P, Mutation>(state, operation, index as u32) { <Arc<P> as FoldProjection<P, Mutation>>::advance(state, next); }
                     }
                     self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                     if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
-                    if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                     continue;
                 }
                 self.cursor += 1;
@@ -25289,21 +25340,23 @@
                 if self.operation_preparation_factory.is_some() {
                     if !self.step_prepared_operation(edit, false, deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                     self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                     self.progress.done = self.progress.done.saturating_add(1);
-                    if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                     continue;
                 }
                 self.replay_operation(edit)?;
                 self.operation += 1;
+                    completed_operations = completed_operations.saturating_add(1);
                 self.progress.done = self.progress.done.saturating_add(1);
-                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                 continue;
             }
             if !self.step_close_edit(&edit.id)? {
                 if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
                 continue;
             }
-            self.progress.done = self.progress.done.saturating_add(1);
+            if !self.operation_progress { self.progress.done = self.progress.done.saturating_add(1); }
             self.cursor += 1;
             self.operation = 0;
             self.record();
@@ -26456,8 +26509,11 @@
     fn one_item_publication_identity(&self) -> (u64, [u8; 32]);
     fn one_item_wire_publication_supported(&self) -> bool;
     fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>>;
+    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, String>;
     fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String>;
     fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
+    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
+    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
     fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError>;
     /// 🧵️ The live typed snapshot behind an opaque immutable ownership boundary.
     fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String>;
@@ -26672,6 +26728,27 @@
         Ok(Box::new(MemberStoreOneItemPublication { member: Some(Arc::clone(&self.snapshot_read_leases)), publication, group_history: None, group_displaced: None }))
     }
 
+    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, String> {
+        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
+        let factory = self.one_item_preparation_factory.as_ref().filter(|_| self.snapshot_retirement_factory.is_some() && self.mutation_retirement_factory.is_some() && self.backbone.is_none() && !self.owned_disposer_terminal)
+            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
+        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
+            return Err("member batch metadata exceeds its fixed identity capacity".into());
+        }
+        let mut inputs = request.mutations.take_mutations::<Mutation>().ok_or_else(|| "member batch source is already transferred or closing".to_string())?;
+        inputs.reverse();
+        let source = ArtifactStoreBatchSourceOf { authority: Some(Arc::clone(factory)), inputs, marker: PhantomData };
+        match self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor.clone(), HistoryLane::Document, request.group_id.clone(), source, false, request.transaction.clone()) {
+            Ok(publication) => Ok(Box::new(MemberStoreOneItemPublication { member: Some(Arc::clone(&self.snapshot_read_leases)), publication, group_history: None, group_displaced: None })),
+            Err(rejected) => {
+                let (reason, mut inputs) = rejected.into_owners();
+                inputs.reverse();
+                request.mutations.restore_mutations(inputs);
+                Err(reason)
+            }
+        }
+    }
+
     fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
         let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member publication concrete owner type does not match".to_string())?;
         if !publication.member.as_ref().is_some_and(|member| Arc::ptr_eq(member, &self.snapshot_read_leases)) {
@@ -26715,6 +26792,22 @@
             ArtifactStoreOneItemAdvance::Blocked => Ok(ArtifactStoreOneItemPreparationStep::Blocked),
             _ => Err("member group preparation crossed its reserved prepublication boundary".into()),
         }
+    }
+
+    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
+        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group stage concrete owner type does not match".to_string())?;
+        if !publication.member.as_ref().is_some_and(|member| Arc::ptr_eq(member, &self.snapshot_read_leases)) { return Err("member group stage belongs to a different exact store authority".into()); }
+        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
+        if publication.publication.group_preparation.is_none() {
+            publication.publication.group_preparation = Some(member_group::MemberGroupPreparation::new(visibility, publication.group_history.take(), publication.group_displaced.take()));
+        }
+        self.stage_apply_batch_group(&mut publication.publication, visibility, grant)
+    }
+
+    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
+        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group adoption concrete owner type does not match".to_string())?;
+        if !publication.member.as_ref().is_some_and(|member| Arc::ptr_eq(member, &self.snapshot_read_leases)) { return Err("member group adoption belongs to a different exact store authority".into()); }
+        self.adopt_apply_batch_group(&mut publication.publication, visibility, grant)
     }
 
     fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
@@ -26727,6 +26820,9 @@
         }
         if !publication.member.as_ref().is_some_and(|member| Arc::ptr_eq(member, &self.snapshot_read_leases)) {
             return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort belongs to a different exact store authority"));
+        }
+        if publication.publication.group_preparation.is_some() {
+            return self.abort_apply_batch_group(&mut publication.publication, grant).map(|step| if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 } } else { step });
         }
         publication.publication.begin_close();
         if grant.maximum_items == 0 {
@@ -27070,6 +27166,8 @@
         match *self {}
     }
 
+    fn begin_one_item_owned_publication(&self, _request: &mut MemberStoreOwnedBatchRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, String> { match *self {} }
+
     fn advance_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
         match *self {}
     }
@@ -27077,6 +27175,10 @@
     fn prepare_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
         match *self {}
     }
+
+    fn stage_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }
+
+    fn adopt_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }
 
     fn abort_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
         match *self {}
@@ -27327,11 +27429,20 @@
             fn begin_one_item_wire_publication(&self, request: $crate::os_store::MemberStoreOneItemWireRequest) -> Result<Box<dyn $crate::os_store::ErasedMemberStoreOneItemPublication>, $crate::os_store::ArtifactStoreBatchAdmissionRejected<$crate::os_store::MemberStoreOneItemWire>> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_wire_publication(m.as_ref(), request)),+ }
             }
+            fn begin_one_item_owned_publication(&self, request: &mut $crate::os_store::MemberStoreOwnedBatchRequest) -> Result<Box<dyn $crate::os_store::ErasedMemberStoreOneItemPublication>, String> {
+                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_owned_publication(m.as_ref(), request)),+ }
+            }
             fn advance_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::ArtifactStoreOneItemAdvance, String> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::advance_one_item_publication(m.as_mut(), publication, grant)),+ }
             }
             fn prepare_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::ArtifactStoreOneItemPreparationStep, String> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::prepare_one_item_publication(m.as_mut(), publication, grant)),+ }
+            }
+            fn stage_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::ArtifactStoreOneItemPreparationStep, String> {
+                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stage_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
+            }
+            fn adopt_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::ArtifactStoreOneItemPreparationStep, String> {
+                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::adopt_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
             }
             fn abort_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::SnapshotRetirementStep, ::semio_framework_value::ValueError> {
                 match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::abort_one_item_publication(m.as_mut(), publication, grant)),+ }
```
