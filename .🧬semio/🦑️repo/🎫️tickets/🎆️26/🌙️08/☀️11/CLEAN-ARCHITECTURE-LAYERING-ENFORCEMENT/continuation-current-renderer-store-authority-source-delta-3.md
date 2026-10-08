# Current Renderer Store Authority Source Delta 3

Read-only comparison of fresh current Store versus actual Product9 complete initial source. No physical layout equality or new native success is inferred. Fresh original whole remains required.

```diff
--- 

+++ 

@@ -18354,7 +18354,7 @@

         revision_accumulator.reconcile::<P, Mutation>(&applied_edit_ids, &redo_edit_ids, &envelope.vcs.edits, &supersessions, StackMirrorDirt::ALL, None);
         if !revision_accumulator.unit_flags.is_empty() {
             let mut units = EditReplay::new(ReplayMode::Report, envelope.vcs.genesis.share_snapshot(), applied_edit_ids.to_vec(), 0, &envelope.schema, supersessions.clone(), &envelope.vcs.edits)?;
-            units.step(&envelope.vcs.edits, &mut || false)?;
+            while matches!(units.step(&envelope.vcs.edits, &mut || false)?, ReplayStep::Pending(_)) {}
             revision_accumulator.unit_flags.extend(units.finish()?.unit_flags.iter().map(|(key, flag)| (*key, *flag)));
         }
         let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
@@ -25236,8 +25236,8 @@

                     if !retirement.terminal_is_empty() { return Err(VcsError::ValidationFailed("replay cleanup reported terminal with live owners".into())); }
                     self.replay_retirement = None;
                 }
-                deadline();
-                return Ok(ReplayStep::Pending(self.progress));
+                if deadline() || step == SnapshotRetirementStep::Blocked { return Ok(ReplayStep::Pending(self.progress)); }
+                continue;
             }
             if self.convergence_pending_finish {
                 self.convergence_pending_finish = false;
@@ -25247,8 +25247,8 @@

             }
             if self.convergence_search.is_some() {
                 self.step_convergence();
-                deadline();
-                return Ok(ReplayStep::Pending(self.progress));
+                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                continue;
             }
             let Some(edit_id) = self.order.get(self.cursor) else {
                 self.finished = true;
@@ -25300,8 +25300,8 @@

                 continue;
             }
             if !self.step_close_edit(&edit.id)? {
-                deadline();
-                return Ok(ReplayStep::Pending(self.progress));
+                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
+                continue;
             }
             self.progress.done = self.progress.done.saturating_add(1);
             self.cursor += 1;
```
