# Resident Primary Lexical Borrow Review — 2026-08-28

## Exact source-only finding

Retained and root independently observed that the new `primary_for_page(page)` use extended the `state.consumers.as_ref()` borrow beyond the cfg(test) `state.consumer_release_interlock.take()` call. Because state is accessed through the guard's Deref/DerefMut, this is a likely compile-time borrow conflict. **No compiler was run and no E0502 was observed.**

The authorized narrow correction moves `let primary = state.primary_for_page(page)?;` immediately after the actual empty-source check, before the unchanged interlock. Its last use of the borrowed ConsumerPage is therefore before mutable guard access. The result is only `Option<NonZeroU64>`, not a borrowed page, pointer authority or cloned owner.

The interlock remains after the same empty-source observation and before the same alias/admission/pin checks. All actual detach, anchor-Releasing, linked-list movement and Release writes remain after the existing grant check. Primary identity validation is retained under the same gate and now executes before the test interlock. No hook is removed; work sums, source pointer reads, pin/alias ordering, tests and other five candidate files are unchanged.

## Exact checkpoints and inverse

- Preserved authority checkpoint: `23516f6485e700392705dc97f62ffb8807212156c8a51dbdb6002da2106d998e`.
- Corrected authority: `28067d28b9b126e6888173405ded79f9b71eca8e12aa3e7523e8ab84bde6e23b`.
- Canonical tests remain `f2336001a31c496606b03a29d65a6d372ce7be13768c249be42922a6f6541e1f`.
- Primary leaf remains `4e79891f6bc1fbcf801a344d196bb9d884208fc2e2ad06b6a1972179f146ee3f`.

[Exact narrow and cumulative six-file forward/inverse with complete preimages/postimages](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-lexical-review-source-delta-2026-08-28.json) retains both the 23516 checkpoint and the full original e23→corrected authority inverse. All hunks replay exactly in both directions in memory; no inverse is applied to disk. The earlier source-review/diff/evidence files remain unchanged and historical.

### Narrow forward

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
@@ -342,6 +342,7 @@
                 return Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<bool>() as u64));
             }
             if !unsafe { (page.empty)(pointer) } { return Ok(ResidentStep::blocked()); }
+            let primary = state.primary_for_page(page)?;
             #[cfg(test)]
             if let Some(interlock) = state.consumer_release_interlock.take() {
                 interlock.observed.try_send(()).map_err(|_| ResidentFault::Identity)?;
@@ -349,7 +350,6 @@
             }
             if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 || header.recovery_pins.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
             let clear_prepared = state.prepared_consumer == Some(pointer);
-            let primary = state.primary_for_page(page)?;
             let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }, if primary.is_some() { size_of::<ResidentPrimaryBacking>() } else { 0 }])?;
             if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
             if primary.is_some() { state.primary.as_mut().unwrap().backing = ResidentPrimaryBacking::Releasing; }
```

### Narrow inverse

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
@@ -342,7 +342,6 @@
                 return Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<bool>() as u64));
             }
             if !unsafe { (page.empty)(pointer) } { return Ok(ResidentStep::blocked()); }
-            let primary = state.primary_for_page(page)?;
             #[cfg(test)]
             if let Some(interlock) = state.consumer_release_interlock.take() {
                 interlock.observed.try_send(()).map_err(|_| ResidentFault::Identity)?;
@@ -350,6 +349,7 @@
             }
             if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 || header.recovery_pins.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
             let clear_prepared = state.prepared_consumer == Some(pointer);
+            let primary = state.primary_for_page(page)?;
             let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }, if primary.is_some() { size_of::<ResidentPrimaryBacking>() } else { 0 }])?;
             if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
             if primary.is_some() { state.primary.as_mut().unwrap().backing = ResidentPrimaryBacking::Releasing; }
```

The candidate remains uncompiled; prospective32 is source enumeration only. No native/Wasm/Nx retry, production Runtime/Store/Opening change or test weakening occurred. The other candidate inputs remain held for the ongoing independent unsafe audit.

