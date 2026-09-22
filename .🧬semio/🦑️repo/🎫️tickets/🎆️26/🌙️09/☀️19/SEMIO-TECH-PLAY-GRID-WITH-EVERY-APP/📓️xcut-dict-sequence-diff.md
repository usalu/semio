# 🎬️ PROPOSED DIFF — `🎬️sequence` guest (bucket XCUT-DICT)

For peer slice S10 / ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, written by the `xcut-dict`
agent of ticket `26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP` on 2026-09-22. Tracked deliberately:
the untracked `🗑️generated/xcut-dict/` copy (with the run logs quoted below) was swept by the
repo workspace-cleanup at ~16:05–16:25, so the evidence is reproduced inline here.

Companion: `📓️xcut-dict.md` (the bucket's protocol write-up, the fixes landed elsewhere, and the
per-crate numbers).

---

## 1. Symptom

`semio-s-artifact-sequence-sequence`, native `cargo test --lib`, 2026-09-22 (log
`🗑️generated/xcut-dict/run2.txt`, since swept): **200 ok / 7 failed**, of which **5** are

```
panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:101:13:
final Dictionary ownership must be explicitly retired or owned by a cold boundary
```

| failing law |
|---|
| `editor::sequence::component::unit_tests::replace_snapshot_preserves_next_serial_and_selection` |
| `editor::sequence::component::unit_tests::repeated_drops_after_replace_snapshot_use_distinct_ids` |
| `editor::sequence::component::unit_tests::set_step_params_json_updates_step_params` |
| `standards::v1::subsets::any::schema::operations::tests::store_applies_and_undoes_step_create` |
| `editor::sequence::modes::edit::windows::main::config::tests::sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows` |

The other two reds of that run (`import_media_steps_in_inserts_a_new_step_from_an_object_payload`,
`import_media_steps_in_wraps_a_bare_scalar_payload`) are a DIFFERENT bucket —
`Fault { code: "interactive-job.missing-reserved-builder", message: "media port 'steps:in' is
registered but has no concrete resumable importer" }` — and are not addressed here.

All five laws already use the CORRECT harness (`neural_engine::ColdOwner::new(SequenceHost::default())`,
`new_sequence_store(create_document_envelope(…))`). Nothing is stale test-side; the abort happens
inside guest production code.

## 2. Evidence — the backtraces (`RUST_BACKTRACE=1`)

`replace_snapshot_preserves_next_serial_and_selection`:

```
 2: <neural_engine::component::Dictionary as core::ops::drop::Drop>::drop
 3: core::ptr::drop_glue::<neural_engine::component::Dictionary>
 4: core::ptr::drop_glue::<semio_s_artifact_sequence_sequence::StepParams>
 5: core::ptr::drop_glue::<semio_s_artifact_sequence_sequence::SequenceStep>
 6: core::ptr::drop_glue::<[semio_s_artifact_sequence_sequence::SequenceStep]>
 7: <alloc::vec::Vec<…::SequenceStep> as core::ops::drop::Drop>::drop
 9: core::ptr::drop_glue::<…::schema::snapshot::component::SequenceHostSnapshot>
10: <…::editor::sequence::component::SequenceHost>::replace_snapshot
```

`store_applies_and_undoes_step_create` — the decisive one:

```
 4: core::ptr::drop_glue::<semio_s_artifact_sequence_sequence::StepParams>
 5: core::ptr::drop_glue::<semio_s_artifact_sequence_sequence::SequenceStep>
 8: core::ptr::drop_glue::<alloc::vec::Vec<…::SequenceStep>>
 9: core::ptr::drop_glue::<semio_s_artifact_sequence_sequence::SequenceWorkingScene>
10: core::ptr::drop_glue::<dyn core::any::Any + Send + Sync>
11: <alloc::sync::Arc<dyn core::any::Any + Send + Sync>>::drop_slow
15: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactChild<
                             semio_s_artifact_stdio_semio::…::SemioFlowSnapshot>>
16: core::ptr::drop_glue::<…::schema::snapshot::component::SequenceSnapshot>
17: core::mem::drop::<…::schema::snapshot::component::SequenceSnapshot>
```

`set_step_params_json_updates_step_params` is the same stack ending at
`drop_glue::<SequenceWorkingScene>` inside the test body.

Every frame 4 is the same: **`drop_glue::<StepParams>` — there is no cold boundary on that type.**

## 3. Root cause

`StepParams` (`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs:52`) and `SequenceStep`
declare `neural_engine::ColdRetire` but **no `Drop` boundary**, unlike the sibling
`imperative_engine::Step` (`✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs:53`), whose docstring states
exactly why the boundary must live on the type:

> "A `Step` is cloned, moved into and out of `Path`s, parked inside a composed child's local owner,
> carried through mutation payloads and edit history, and dropped by the framework wherever a
> snapshot dies — there is no single call site that could own that retirement, and in the wasm guest
> a missed one is an abort, not a test failure."

`store_applies_and_undoes_step_create` is literally that sentence: the `SequenceWorkingScene` dies
inside an `Arc<dyn Any>` held by the framework's own `ArtifactChild` local-owner machinery, when the
store drops a displaced `SequenceSnapshot`. **No call-site fix in `🎬️sequence` can reach it** — only
a `Drop` on the type can.

And the protocol itself: `Dictionary::drop` calls `OrderedMap::release_shared()`
(`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:136`), which returns `Ok` only when the root is
`None` (an EMPTY dictionary) or another `Arc` still shares it. So the last owner of a NON-EMPTY
dictionary must always leave through an explicit cold boundary.

## 4. The diff

### 4.1 `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs`

```diff
 #[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
 #[value(transparent)]
 pub struct StepParams(pub Dictionary);
 
+/// 🧊️ Same fail-closed boundary `imperative_engine::Step` declares for its own `params`
+/// (`✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs`): a `StepParams` is cloned into host snapshots,
+/// working scenes, mutation payloads, edit history and editor transients, and is dropped by the
+/// framework wherever a snapshot dies — a `SequenceWorkingScene` parked behind an `Arc<dyn Any>` in
+/// an `ArtifactChild`'s local owner is released by the STORE, not by any call site here. There is
+/// therefore no single place that could own the retirement, and in the wasm guest a missed one is
+/// an abort, not a test failure (`final Dictionary ownership must be explicitly retired or owned by
+/// a cold boundary`, ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, bucket XCUT-DICT).
+///
+/// Because this type now has a `Drop`, its field can no longer be moved out —
+/// `std::mem::take(&mut …)` it instead (see §4.2).
+impl Drop for StepParams {
+    fn drop(&mut self) {
+        neural_engine::ColdRetire::retire_cold(std::mem::take(&mut self.0));
+    }
+}
+
+/// 🧊️ Kept so `SequenceStep`/`Vec<SequenceStep>`/`SequenceWorkingScene` still satisfy `ColdRetire`;
+/// the work itself is the `Drop` above, so consuming the value IS the retirement.
 impl neural_engine::ColdRetire for StepParams {
-    fn retire_cold(self) {
-        self.0.retire_cold();
-    }
+    fn retire_cold(self) {
+        drop(self);
+    }
 }
```

```diff
 impl StepParams {
     pub fn new() -> Self {
         Self(Dictionary::new())
     }
 
-    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
-        Self(self.0.insert(key, value))
-    }
+    /// 🧊️ `self`'s own `Drop` runs at the end of this call, so the root is TAKEN out first and the
+    /// emptied husk is what drops (an empty `Dictionary` releases cleanly).
+    pub fn insert(mut self, key: impl Into<String>, value: Value) -> Self {
+        Self(std::mem::take(&mut self.0).insert(key, value))
+    }
 }
```

`impl neural_engine::ColdRetire for SequenceStep` (same file, just below) moves `params` out of a
`SequenceStep`, which has no `Drop` — it compiles unchanged.

### 4.2 `…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — the five `.0` moves

With a `Drop` on `StepParams`, `E0509 cannot move out of type … which implements the Drop trait`
fires at exactly five places. Each takes the root instead of moving the newtype's field (the binding
needs `mut`):

| line | current | becomes |
|---|---|---|
| 1532 | `discarded_params = Some(params.0);` | `discarded_params = Some(std::mem::take(&mut params.0));` |
| 1597 | `self.retirement.push_dictionary(params.0);` | `self.retirement.push_dictionary(std::mem::take(&mut params.0));` |
| 2137 | `self.retirement.push_dictionary(params.0);` | `self.retirement.push_dictionary(std::mem::take(&mut params.0));` |
| 2142 | `SequenceMutation::CreateStep(value) => self.retirement.push_dictionary(value.step.params.0),` | `SequenceMutation::CreateStep(mut value) => self.retirement.push_dictionary(std::mem::take(&mut value.step.params.0)),` |
| 2143 | `SequenceMutation::EditStepParams(value) => self.retirement.push_dictionary(value.params.0),` | `SequenceMutation::EditStepParams(mut value) => self.retirement.push_dictionary(std::mem::take(&mut value.params.0)),` |

Borrows are unaffected and need no change: `🦀️.rs:203` (`&step.params.0`), `🦀️.rs:214`
(`StepParams(...)` construction), `✏️editor/🦀️.rs:788, 2394, 2401, 2416, 2431, 2445`
(`step.params.0.clone()` / `&step.params.0`), `🧬️schema/🧬️mutations/🦀️.rs:35`
(`value.params.retire_cold()` — moves `params` out of `EditStepParams`, which has no `Drop`).

### 4.3 The three bare-`Dictionary` scope rebinds (same class, not covered by 4.1)

`SequenceRunState` holds `scope: Dictionary` directly, so the `Drop` above does not reach it. Three
rebinds in `SequenceRunState::advance` plain-drop the displaced root — 2455 runs on **every executed
step of a `run` verb**:

```diff
@@ 2383 (control.repeat cursor) @@
-                self.scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
+                let next = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
+                neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut self.scope, next));

@@ 2426 (nested repeat frame) @@
-                    self.scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(0)));
+                    let next = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(0)));
+                    neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut self.scope, next));

@@ 2455 (step result) @@
-                self.scope = result.scope;
+                neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut self.scope, result.scope));
```

`imperative_engine` already has this exact helper (`replace_scope_cold`,
`✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs:246`) with the rationale in its docstring; a
`pub` re-export or a local twin would read better than three inline `mem::replace`s.

`self.scope = Dictionary::new()` at 2352 assigns over `Dictionary::default()` (empty root) and is
already safe.

### 4.4 Covered automatically by 4.1 (no edit needed once `Drop` lands)

Four plain assignments over a live `StepParams`/`SequenceHostSnapshot` root, all in
`✏️editor/🦀️.rs`: **223** (`step.params = value.params` in the working-scene `EditStepParams` apply),
**545** (`self.snapshot = snapshot` in `SequenceHost::replace_snapshot`), **661**
(`step.params = params` in `set_step_params_json`), **1256** (`step.params = payload.params.clone()`
in the `EditStepParams` inverse path).

## 5. Minimal alternative (NOT sufficient on its own)

If the peer prefers not to add a `Drop`, the four sites of §4.4 can be written as
`neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut <slot>, <next>))`. That fixes
`replace_snapshot_preserves_next_serial_and_selection`,
`repeated_drops_after_replace_snapshot_use_distinct_ids` and
`set_step_params_json_updates_step_params`, but **not**
`store_applies_and_undoes_step_create` or the window-ownership law: those drop through the
framework's `Arc<dyn Any>` child-owner path, which no call site in `🎬️sequence` touches. §4.1 is the
only complete fix.

## 6. Verification the peer should expect

```
zsh "$T/📜️native-test-mutex.sh" <topic> -- env \
  CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/xcut-dict/target \
  CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432 RUST_BACKTRACE=1 \
  DEVELOPER_DIR=/Library/Developer/CommandLineTools \
  cargo test --no-fail-fast -p semio-s-artifact-sequence-sequence -- --test-threads=4
```

Expected: `200 ok / 7 failed` → `205 ok / 2 failed`, zero `Dictionary ownership` panics, the two
remaining reds being the `import_media_steps_in_*` pair of §1.
Guest-side check afterwards: `cargo check -p semio-s-artifact-sequence-sequence --target wasm32-wasip2`
(through the hub wasm mutex — the `xcut-dict` agent is not allowed to run wasm32 targets).
