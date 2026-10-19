# FEM3D Session Whole Owner Comparison

## implMountedState::close_step

### B to H

```diff
--- 

+++ 

@@ -1,3 +1,3 @@

-    fn close_step(&mut self, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
         let maximum_bytes = grant.maximum_release_bytes;
         match self.close_lane {
@@ -54,5 +54,5 @@

             3 => {
                 if let Some(numerical) = self.numerical.as_mut() {
-                    let (terminal, items, bytes) = numerical.close_step(grant);
+                    let (terminal, items, bytes) = numerical.close_step(cx, grant);
                     if !terminal {
                         return session_pending(items, bytes);
```

### Incoming whole function

```rust
    fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
        let maximum_bytes = grant.maximum_release_bytes;
        Ok(match self.close_lane {
            0 => {
                if let Some(candidate) = self.candidate.as_mut() {
                    let before = candidate.backing_usage();
                    let (terminal, items, bytes) = candidate.close_step(maximum_bytes);
                    let after = candidate.backing_usage();
                    if !self.backing.release(before.0 - after.0, before.1 - after.1) {
                        return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D visual candidate process credit mismatch" });
                    }
                    if !terminal {
                        return Ok(session_pending(items, bytes));
                    }
                    if !candidate.terminal_is_empty() {
                        return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D visual candidate false terminal" });
                    }
                    self.candidate = None;
                    return Ok(session_pending(1, 0));
                }
                self.close_lane = 1;
                session_pending(1, 0)
            }
            1 => {
                if let Some(displaced) = self.displaced.as_mut() {
                    let (terminal, items, bytes) = displaced.close_step(maximum_bytes);
                    if !terminal {
                        return Ok(session_pending(items, bytes));
                    }
                    if !displaced.terminal_is_empty() {
                        return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D displaced lease false terminal" });
                    }
                    self.displaced = None;
                    return Ok(session_pending(1, 0));
                }
                self.close_lane = 2;
                session_pending(1, 0)
            }
            2 => {
                if let Some(current) = self.current.as_mut() {
                    let (terminal, items, bytes) = current.close_step(maximum_bytes);
                    if !terminal {
                        return Ok(session_pending(items, bytes));
                    }
                    if !current.terminal_is_empty() {
                        return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D current lease false terminal" });
                    }
                    self.current = None;
                    return Ok(session_pending(1, 0));
                }
                self.close_lane = 3;
                session_pending(1, 0)
            }
            3 => {
                if let Some(numerical) = self.numerical.as_mut() {
                    if !numerical.terminal_is_empty() {
                        let (_, progress) = numerical.close_step(cx, grant)?;
                        return Ok(PluginLifecycleStep::Progress(progress));
                    }
                    self.numerical = None;
                    return Ok(session_pending(1, 0));
                }
                self.close_lane = 4;
                session_pending(1, 0)
            }
            4 => {
                if let Some(solver) = self.solver.as_mut() {
                    let (terminal, items, bytes) = solver.close_step(maximum_bytes);
                    if items != 0 && !self.backing.release(items, bytes) {
                        return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D solver process credit mismatch" });
                    }
                    if !terminal {
                        return Ok(session_pending(items, bytes));
                    }
                    self.solver = None;
                    return Ok(session_pending(items, bytes));
                }
                self.close_lane = 5;
                session_pending(1, 0)
            }
            5 => {
                if let Some(snapshot) = self.snapshot.take() {
                    let Some(witness) = snapshot.return_to_registry_witness() else {
                        return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D snapshot already returned" });
                    };
                    self.snapshot_return = Some(witness);
                    return Ok(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotRead<Fem3dSnapshot>>()));
                }
                if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
                    return Ok(session_pending(0, 0));
                }
                if self.snapshot_return.take().is_some() {
                    return Ok(session_pending(1, size_of::<store::os_store::snapshot_read::SnapshotReadReturn>()));
                }
                self.close_lane = 6;
                session_pending(1, 0)
            }
            6 => {
                if !self.backing.terminal_is_empty() {
                    return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D process backing credit remains live" });
                }
                self.close_lane = 7;
                session_pending(1, 0)
            }
            _ => PluginLifecycleStep::Complete(RetainedCloneProgress::default()),
        })
    }
```

## free::recover_abandoned_one

### B to H

```diff
--- 

+++ 

@@ -1,3 +1,3 @@

-fn recover_abandoned_one(registry: &mut Registry, app_instance_id: u32, grant: RetainedCloneGrant) -> Option<PluginLifecycleStep> {
+pub(super) fn recover_abandoned_one(registry: &mut Registry, app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Option<PluginLifecycleStep> {
     let shell = registry.recovery_cursor;
     registry.recovery_cursor = (registry.recovery_cursor + 1) % SHELL_CAPACITY;
@@ -36,5 +36,5 @@

     }
     state.cancel.cancel_now();
-    let step = state.close_step(grant);
+    let step = state.close_step(cx, grant);
     if !matches!(&step, PluginLifecycleStep::Complete(_)) {
         if let Err(state) = recovery.restore_owner(identity, state) {
```

### Incoming whole function

```rust
fn recover_abandoned_one(registry: &mut Registry, app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
    let shell = registry.recovery_cursor;
    registry.recovery_cursor = (registry.recovery_cursor + 1) % SHELL_CAPACITY;
    let recovery = registry.recoveries[shell].clone();
    let Some((identity, publication)) = recovery.publication.get() else { return Ok(None); };
    if identity.app_instance_id != app_instance_id || recovery.reserved.get() != Some(identity) {
        return Ok(None);
    }
    if publication == MountedRecoveryPublication::Retained {
        let retained = registry.shells[shell].try_borrow().ok().is_some_and(|owner| owner.as_ref().is_some_and(|state| state.identity == identity && state.done));
        if !retained {
            return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D retained job handoff identity mismatch" }));
        }
        recovery.clear_publication(identity);
        return Ok(Some(session_pending(1, 0)));
    }
    let mut state = recovery.take_owner(identity);
    if state.is_none() {
        let Ok(mut shell_owner) = registry.shells[shell].try_borrow_mut() else { return Ok(Some(session_pending(0, 0)))};
        if shell_owner.as_ref().is_some_and(|state| state.identity == identity) {
            let discovered = shell_owner.take();
            drop(shell_owner);
            if let Some(discovered) = discovered {
                if let Err(discovered) = recovery.publish_owner(identity, discovered) {
                    std::mem::forget(discovered);
                    return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state recovery slot collision" }));
                }
            }
            state = recovery.take_owner(identity);
        }
    }
    let Some(mut state) = state else { return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state owner is not discoverable" }))};
    if state.identity != identity {
        std::mem::forget(state);
        return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state generation mismatch" }));
    }
    state.cancel.cancel_now();
    let step = match state.close_step(cx, grant) {
        Ok(step) => step,
        Err(failure) => {
            if let Err(state) = recovery.restore_owner(identity, state) { std::mem::forget(state); }
            return Err(failure);
        }
    };
    if !matches!(&step, PluginLifecycleStep::Complete(_)) {
        if let Err(state) = recovery.restore_owner(identity, state) {
            std::mem::forget(state);
            return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state restore collision" }));
        }
        return Ok(Some(step));
    }
    if !state.terminal_is_empty() {
        if let Err(state) = recovery.restore_owner(identity, state) {
            std::mem::forget(state);
        }
        return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned state false terminal" }));
    }
    drop(state);
    let Some(active) = registry.app_slots.lookup(identity.app_instance_id).map(|slot| slot.index()) else { return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D recovery lost app lease" }))};
    for slot in 0..ACTIVE_CAPACITY {
        if registry.current[slot].is_some_and(|current| current.identity == identity && current.shell as usize == shell) { registry.current[slot] = None; }
        if registry.retained_results[slot].is_some_and(|retained| retained.identity == identity && retained.shell as usize == shell) { registry.retained_results[slot] = None; }
    }
    if registry.retiring[shell] && registry.retiring_app[shell] == identity.app_instance_id {
        registry.retiring[shell] = false;
        registry.retiring_app[shell] = 0;
        registry.retiring_count[active] = registry.retiring_count[active].saturating_sub(1);
    }
    recovery.clear_publication(identity);
    if !recovery.release(identity) {
        return Ok(Some(PluginLifecycleStep::Blocked { reason: "FEM3D abandoned recovery authority false terminal" }));
    }
    registry.release_credit(shell as u16);
    registry.shell_request[shell] = None;
    registry.release(shell as u16);
    Ok(Some(session_pending(1, 0)))
}
```

## free::retire_one

### B to H

```diff
--- 

+++ 

@@ -1,3 +1,3 @@

-fn retire_one(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
     MOUNTED.with(|registry| {
         let mut registry = registry.borrow_mut();
@@ -15,5 +15,5 @@

                 return PluginLifecycleStep::Blocked { reason: "FEM3D retiring shell empty" };
             };
-            state.close_step(grant)
+            state.close_step(cx, grant)
         };
         if matches!(step, PluginLifecycleStep::Complete(_)) {
```

### Incoming whole function

```rust
fn retire_one(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(slot) = registry.app_slots.lookup(app_instance_id).map(|slot| slot.index()) else { return Ok(session_pending(0, 0))};
        for request_slot in 0..ACTIVE_CAPACITY {
            let replacement_ready = registry.current[request_slot].filter(|current| current.app_instance_id == app_instance_id).is_some_and(|current| registry.shells[current.shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.done && state.numerical_done && state.solver.as_ref().is_some_and(Fem3dSolverView::ready))));
            if replacement_ready && registry.retained_results[request_slot].is_some() { if !registry.retire_retained_results(app_instance_id, request_slot) { return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D result retirement capacity" }); } return Ok(session_pending(1, 0)); }
        }
        let shell = registry.maintenance_cursor;
        registry.maintenance_cursor = (registry.maintenance_cursor + 1) % SHELL_CAPACITY;
        if !registry.retiring[shell] || registry.retiring_app[shell] != app_instance_id {
            return Ok(session_pending(0, 0));
        }
        let step = {
            let Ok(mut owner) = registry.shells[shell].try_borrow_mut() else { return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D retiring child is borrowed" })};
            let Some(state) = owner.as_mut() else {
                if registry.recoveries[shell].publication.get().is_some_and(|current| current.1 == MountedRecoveryPublication::Recover) {
                    return Ok(session_pending(0, 0));
                }
                return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D retiring shell empty" });
            };
            state.close_step(cx, grant)?
        };
        if matches!(step, PluginLifecycleStep::Complete(_)) {
            let terminal_identity = registry.shells[shell].borrow().as_ref().filter(|state| state.terminal_is_empty()).map(|state| state.identity);
            let Some(identity) = terminal_identity else {
                return Ok(PluginLifecycleStep::Blocked { reason: "FEM3D retiring state false terminal" });
            };
            *registry.shells[shell].borrow_mut() = None;
            registry.retiring[shell] = false;
            registry.retiring_app[shell] = 0;
            registry.retiring_count[slot] = registry.retiring_count[slot].saturating_sub(1);
            registry.recoveries[shell].clear_publication(identity);
            assert!(registry.recoveries[shell].release(identity), "FEM3D normal retirement recovery authority mismatch");
            registry.release_credit(shell as u16);
            registry.shell_request[shell] = None;
            registry.release(shell as u16);
            return Ok(session_pending(1, 0));
        }
        Ok(step)
    })
}
```

## free::maintenance_step

### B to H

```diff
--- 

+++ 

@@ -1,8 +1,8 @@

-pub fn maintenance_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
     let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
     if maximum_items == 0 {
         return session_pending(0, 0);
     }
-    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, grant)) {
+    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, cx, grant)) {
         return step;
     }
@@ -13,4 +13,4 @@

         return session_pending(items, bytes);
     }
-    retire_one(app_instance_id, grant)
+    retire_one(app_instance_id, cx, grant)
 }
```

### Incoming whole function

```rust
pub fn maintenance_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
        let step = maintenance_step_turn(app_instance_id, cx, grant)?;
        Ok((step, terminal_is_empty(app_instance_id)))
    })
}
```

## free::close_step

### B to H

```diff
--- 

+++ 

@@ -1,8 +1,8 @@

-pub fn close_step(app_instance_id: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
+pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PluginLifecycleStep {
     let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
     if maximum_items == 0 {
         return session_pending(0, 0);
     }
-    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, grant)) {
+    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, cx, grant)) {
         return step;
     }
@@ -36,5 +36,5 @@

         }
         drop(registry);
-        retire_one(app_instance_id, grant)
+        retire_one(app_instance_id, cx, grant)
     })
 }
```

### Incoming whole function

```rust
pub fn close_step(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    semio_framework_plugin::app::original_lifecycle_step(cx, grant, close_demands(app_instance_id)?, |cx, grant| {
        let step = close_step_turn(app_instance_id, cx, grant)?;
        Ok((step, terminal_is_empty(app_instance_id)))
    })
}
```

## free::close_demands

### B to H

```diff
```

### Incoming whole function

```rust
pub fn close_demands(app_instance_id: u32) -> Result<RetirementDemand, ValueError> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let mut demand = RetirementDemand { copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() };
        for shell in &registry.shells {
            if let Ok(owner) = shell.try_borrow() {
                if let Some(state) = owner.as_ref().filter(|state| state.identity.app_instance_id == app_instance_id) { state.child_close_demands(&mut demand)?; }
            }
        }
        for recovery in &registry.recoveries {
            let owner = recovery.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(state) = owner.as_ref().filter(|state| state.identity.app_instance_id == app_instance_id) { state.child_close_demands(&mut demand)?; }
        }
        Ok(demand)
    })
}
```

## implMountedState::child_close_demands

### Incoming whole function

```rust
    fn child_close_demands(&self, demand: &mut RetirementDemand) -> Result<(), ValueError> {
        if let Some(child) = self.numerical.as_ref() { merge_close_demand(demand, child.close_demands()?); }
        Ok(())
    }
```

## free::result_request

### Incoming whole function

```rust
pub fn result_request(doc: &Fem3dSnapshot, source: Option<&str>, mode: Option<usize>) -> Result<Fem3dResultRequest, &'static str> {
    if mode.is_some_and(|index| index >= doc.analysis.modal_count) { return Err("fem3d.results-mode-missing"); }
    let source = match source {
        None => Fem3dResultSource::FirstCase,
        Some(identity) => {
            let source = semio_s_fem_analyses_model::borrow_result_source_identity(identity).map_err(|_| "fem3d.results-source-identity")?;
            match source.kind {
                "loadCase" => Fem3dResultSource::Case(doc.load_cases.iter().position(|case| case.id == source.id).ok_or("fem3d.results-source-missing")?),
                "combination" => Fem3dResultSource::Combination(doc.combinations.iter().position(|combination| combination.id == source.id).ok_or("fem3d.results-source-missing")?),
                _ => return Err("fem3d.results-source-kind"),
            }
        }
    };
    Ok(Fem3dResultRequest { source, mode, buckling: None })
}
```

## free::retire_request_one

### Incoming whole function

```rust
fn retire_request_one(app: u32, grant: RetainedCloneGrant) -> Option<PluginLifecycleStep> {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        for slot in 0..ACTIVE_CAPACITY {
            let Some((owner, scope, _)) = registry.requests.at(slot) else { continue };
            if scope != app || registry.requests.references(owner) != 0 { continue; }
            if registry.pending[slot].take().is_some() { return Some(session_pending(1, 0)); }
            if registry.requested[slot].take().is_some() { return Some(session_pending(1, 0)); }
            if registry.request_specs[slot].take().is_some() { return Some(session_pending(1, 0)); }
            if registry.retained_results[slot].is_some() { return Some(if registry.retire_retained_results(app, slot) { session_pending(1, 0) } else { PluginLifecycleStep::Blocked { reason: "FEM3D request result close capacity" } }); }
            if let Some(current) = registry.current[slot] {
                if !registry.retain_retiring(app, current.shell) { return Some(PluginLifecycleStep::Blocked { reason: "FEM3D request child close capacity" }); }
                if let Ok(child) = registry.shells[current.shell as usize].try_borrow() { if let Some(state) = child.as_ref() { state.cancel.cancel_now(); } }
                registry.current[slot] = None;
                return Some(session_pending(1, 0));
            }
            if registry.shell_request.iter().any(|request| *request == Some(owner)) { continue; }
            if let Some(progress) = registry.requests.retire_request_step(owner, grant) { return Some(PluginLifecycleStep::Progress(progress)); }
            assert!(registry.requests.release(owner), "FEM3D original request lease survives through its child retirement");
            return Some(session_pending(1, 0));
        }
        None
    })
}
```

## free::maintenance_step_turn

### Incoming whole function

```rust
fn maintenance_step_turn(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
    if maximum_items == 0 {
        return Ok(session_pending(0, 0));
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, cx, grant))? {
        return Ok(step);
    }
    if let Some(progress) = MOUNTED.with(|registry| registry.borrow_mut().requests.retire_consumer_step(app_instance_id, grant)) { return Ok(PluginLifecycleStep::Progress(progress)); }
    if let Some(step) = retire_request_one(app_instance_id, grant) { return Ok(step); }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return Ok(session_pending(items, bytes));
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return Ok(session_pending(items, bytes));
    }
    retire_one(app_instance_id, cx, grant)
}
```

## free::close_step_turn

### Incoming whole function

```rust
fn close_step_turn(app_instance_id: u32, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
    let (maximum_items, maximum_bytes) = (grant.maximum_items, grant.maximum_release_bytes);
    if maximum_items == 0 {
        return Ok(session_pending(0, 0));
    }
    if let Some(step) = MOUNTED.with(|registry| recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, cx, grant))? {
        return Ok(step);
    }
    if let Some((items, bytes)) = world3d_snapshot_recovery_close_step(maximum_bytes) {
        return Ok(session_pending(items, bytes));
    }
    if let Some((items, bytes)) = close_recovered_fem3d_backing(maximum_bytes) {
        return Ok(session_pending(items, bytes));
    }
    if terminal_is_empty(app_instance_id) {
        return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(lease) = registry.app_slots.lookup(app_instance_id) else { return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))};
        let slot = lease.index();
        if registry.requests.remove_one_scoped_consumer(app_instance_id).is_some() { return Ok(session_pending(1, 0)); }
        if let Some(progress) = registry.requests.retire_consumer_step(app_instance_id, grant) { return Ok(PluginLifecycleStep::Progress(progress)); }
        let owns_request = (0..ACTIVE_CAPACITY).any(|index| registry.requests.at(index).is_some_and(|(_, scope, _)| scope == app_instance_id));
        if owns_request { drop(registry); return Ok(if let Some(step) = retire_request_one(app_instance_id, grant) { step } else { retire_one(app_instance_id, cx, grant)? }); }
        if !registry.requests.has_consumer(app_instance_id) && registry.retiring_count[slot] == 0 && registry.recoveries.iter().all(|recovery| !recovery.contains(app_instance_id)) {
            assert!(registry.app_slots.release(app_instance_id, lease), "FEM3D original app lease remains live through child retirement");
            return Ok(session_pending(1, 0));
        }
        drop(registry);
        retire_one(app_instance_id, cx, grant)
    })
}
```
