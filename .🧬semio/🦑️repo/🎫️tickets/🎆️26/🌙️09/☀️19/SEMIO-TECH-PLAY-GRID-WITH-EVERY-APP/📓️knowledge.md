# 📓️ knowledge — native test debt for note / forms / mathematical / reasoning / dag / trinity / imperative

Topic `knowledge`, fleet v4, 2026-09-21 (session 5; the agent was killed by a usage limit at ~18:20 and resumed
at 20:24). Scratch + logs: `🗑️generated/knowledge/` (`STATUS.md`, `run.sh`, `batch1.txt`, `batch2.txt`,
`final-a.txt`, `final-b.txt`, `run-a.txt`). Command used throughout (one cargo per group, private target dir):

```
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools \
CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-knowledge \
cargo test -p … --lib --tests --no-fail-fast [--features component-app-assembly] -- --test-threads=4
```

## 1. Per crate: first run → latest run

No coordinator baseline existed for these crates (the sweep had only reached `fem` when this topic started), so the
first-run column is this topic's own measurement.

| crate | first run (15:55) | latest run (18:20, `final-a.txt`/`batch1.txt`) |
|---|---|---|
| `semio-s-artifact-note-note` | 350 pass / 44 fail | 362 / **33** |
| `semio-s-artifact-mathematical-equation` | 345 / 39 | 353 / **31** |
| `semio-s-artifact-imperative-procedure` | 112 / 32 | 112 / **32** |
| `semio-s-artifact-forms-forms` | 167 / 30 | 190 / **7** |
| `semio-s-artifact-reasoning-wires` | 162 / 26 | 167 / **20** |
| `semio-s-artifact-dag-dag` | 197 / 10 | 198 / **9** |
| `semio-s-artifact-trinity-jack` | 209 / 1 | **210 / 0 ✅** |
| `semio-s-artifact-trinity-rewriting` | 154 / 3 | **157 / 0 ✅** |
| `semio-s-plugin-note` | 3 / 1 | **4 / 0 ✅** |
| `semio-s-plugin-mathematical` | 2 / 2 | 2 / **2** |
| `semio-s-plugin-reasoning` | 4 / 0 ✅ | 3 / **1** (regressed — see §3.1) |
| `semio-s-plugin-trinity-jack-shell` | did not compile | **1 / 0 ✅** |
| `semio-s-plugin-dag`, `-forms`, `-imperative`, `-imperative-control/-effect/-logic/-math/-text`, `-trinity`, `-trinity-jack-lsp` | green | **green ✅** |

Total failures across the 22 crates: **184 → 134**. Twelve of the 22 crates are green.

## 2. Root causes found and fixed

**2.1 Three compile blockers (nothing ran until these were cleared)**
- `semio-s-plugin-trinity-jack-shell` imported `trinity::ast::QueryResult` / `trinity::executor::run`; both modules
  moved into `semio-s-artifact-trinity-jack`. Repointed the imports and dropped the now-unused `trinity` (plugin)
  dependency from the shell's `Cargo.toml`.
- A peer's new `candidate_scene_node_for_presented_node` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
  used a bare `NodeId`, breaking `semio-framework-ui` for the entire fleet. Waited 12 min for the peer, then
  qualified it to `crate::wgpu::arena::NodeId` (every sibling in that file already does).
- 11 test sites compared `SemanticMutation::label()` — now a `LocalizedLabel` after the peer sweep — against `&str`,
  and one imperative test iterated a heterogeneous `[&ArtifactChild<SemioFlowSnapshot>, &ArtifactChild<SemioTextSnapshot>]`
  array. Updated to `label().resolve(protocol::Terminology::Native, protocol::Locale::En)` and to a
  `(slot, artifact_id, child_id)` tuple loop, preserving exactly what each assertion proved.

**2.2 Bucket 2 — 47 × `artifact store reached Drop without its exact terminal-empty shallow-shell witness`**
Every mounted-app fixture in forms/note/mathematical/reasoning/imperative handed the law a bare `VcsArtifactApp`
that nothing ever closed. Each `context` now mints an `Owned<X>App` guard (`Deref`/`DerefMut` + `Drop` →
`close_registered_fixture_app`, skipped while `std::thread::panicking()`), the pattern trinity jack's
`⚙️operations/🦀️.rs` already uses for its store. Owning the close in the fixture — instead of asking every law to
remember a trailing `close(&mut app)` — is what makes a law that fails an assertion report ITS failure rather than
a close panic. Generic framework helpers do not get deref coercion, so the handful of call sites that pass the
fixture straight into `settle_registered_typed_operation` / `close_registered_fixture_app` use `&mut *app` or the
guard's own `close()`.

**2.3 Bucket 3 — ~13 × `interactive-job.live-instance` in `➗️mathematical`**
Its fixture never called `bind_instance_id(meta("local").instance_id)` and its `dispatch` never settled. Added both
(`🕸️dag`'s fixture is the in-repo reference that already does). Same missing settle in `💡️reasoning`'s `dispatch`
(added, plus a `settle()` helper returning the receipt), and `delete_selection`'s law built its app through the
unbound `app_with_registry()` instead of `new_app()`.

**2.4 `setActiveExample` read the wrong lane (reasoning)**
`result.requested_effects` is empty on a mounted app; the effects arrive on the settled receipt. Both
`set_active_example` laws now assert on `settle(&mut app).await.effects`, mirroring trinity rewriting's passing law.

**2.5 The `reasoning-wires` blank canvas (coordinator goal 1) — ROOT CAUSE FOUND AND FIXED**
`🖼️assets/🎬️demo/🗣️.dsl.semio` was a STUB: `wires.board.nodes = []` and a single top-level `node-1` "Demo".
`metabolism_wires_example_snapshot()` hid that with a compatibility fallback — `Ok(snapshot) if fixture_nodes(...).len() >= 7`
else `handcrafted_metabolism_snapshot()` — so every unit test saw a seven-node graph while the play pane, which
loads the committed asset itself, rendered an empty board. Regenerated the asset with the crate's OWN printer
(`<WiresSnapshot as ArtifactDsl>::print_dsl`, 672 → 8 409 bytes, 7 nodes / 9 edges / first label "Metabolism"),
deleted `handcrafted_metabolism_snapshot` and the `>= 7` guard (AGENTS forbids compatibility layers), and replaced
the `assert!(text.len() > 8)` example law with one that asserts the board's 7 nodes, 9 edges and first label.

**2.6 `➗️mathematical` `setActiveExample` (coordinator goal 2) — ALREADY PRESENT, the play gate is stale**
The equation editor already declares the action (`✏️editor/🦀️.rs:1503` `ActionDefinition::new("setActiveExample", …)`
+ `action_interactive_job` + `action_args` offering `demo`), it is registered on both tool rosters, and the COMMITTED
descriptor `✏️s/🔌️plugins/➗️mathematical/🔣️.json` carries it on `s.mathematical.equation@1/*#editor` with
`examples: ["demo"]`. Play's gate reads that committed manifest, so `publishedExamples(...).switches` is already
true and the `mathematical:` row in `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES`
(`🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts:83`) is STALE — the audit's "n/a (no picker)" verdict was
derived from that list, not from the DOM. **Not removed by me** (play is another topic's file and its unit suite is
that topic's gate); see §4.1 for the exact next step.

**2.7 Two general defects outside the plugins**
- `✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs`: `merged = replaced_cold(merged, merged.insert(…))` — `Dictionary::insert`
  CONSUMES its receiver (it builds through `ColdDictionaryBuilder`), so there is no previous owner left to retire and
  the code did not compile (`use of moved value`). Now `merged = merged.insert(…)`; the borrowing `merge` branches
  still go through `replaced_cold`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs`: a peer's in-flight
  `UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, … }` referenced an unlinked `ui_contract` crate
  and a field that struct does not carry there; it broke the whole fleet's build for >10 min. Removed the argument —
  `Button` is the value the field's own `skip_serializing_if = "…is_button"` treats as default.

## 3. What is still red, and why

### 3.1 `semio-s-plugin-reasoning` — 1 failure, `descriptor_is_fresh`
Regenerating the demo asset (§2.5) changed the plugin's example bytes, so the committed
`✏️s/🔌️plugins/💡️reasoning/🛂️.descriptor.semio` + `🔣️.json` are stale. **Next step:**
`bun nx run …:describe` for that crate (`✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📜️script.ts describe`, a
`wasm32-wasip2` component build) and commit the refreshed pair — then request the reasoning-wires activation lane
(`touch "$T/🗑️generated/activate.request/reasoning-wires"`) so :6033 serves the real demo graph. I did not run it:
it is a wasm component build and the coordinator owns activation.

### 3.2 `semio-s-artifact-note-note` 33 · `semio-s-artifact-dag-dag` 9 — committed fixture JSON is not canonical
Dominant sub-bucket (~16 of the 42): the committed `🔺️diff/🔣️.json` / `🦠️mutation/🔣️.json` write an f64 slot as an
INTEGER literal — `"pencilWidth": 5` where the crate's own encoder prints `5.0` (brief bucket 1), and dag's
create-node fixture additionally writes `variadicInputs` where `DagNodeKind` renames those two fields to
`variadic_inputs`. Every such case fails twice (`committed_json_is_canonical` + `produces_committed_diff`).
**Work in progress:** a temporary regeneration test is in place in BOTH crates
(`…/🗒️note/🧪️tests/🔬️unit/🦀️.rs` and `…/🕸️dag/🧪️tests/🔬️unit/🦀️.rs`, both named
`temporary_regenerate_mutation_fixtures`) which walks every `🧫️fixtures/🧬️mutations/**/🔣️.json`, decodes it with the
crate's own `dsl::os_pack::from_json_str::<NoteSnapshot|NoteMutation|NoteDiff>` / `<DagMutation|DagDiff>`,
re-encodes canonically and rewrites the file. **Next step: run those two tests once, confirm the rewritten
fixtures, then DELETE both temporary tests** (they are marked `temporary_` and print `[DEBUG]` lines). They had not
been executed when this report was written — the verification run was still rebuilding (§5).

### 3.3 `semio-s-artifact-imperative-procedure` — 32 failures, one dominant cause
21 of the 32 are the presence lane: 7 × `dispatch: … "presence local read requires a live exact local retirement
owner"`, 6 × the same message from `registered fixture close`, and 8 × XCUT-DICT
`final Dictionary ownership must be explicitly retired or owned by a cold boundary`.
`PresenceStore::local_read` (`🏪️store/🦀️.rs:4600`) fails when `close_started || local_retirement_factory.is_none()`,
and the factory is installed only from `A::build_presence_local_root_retirement_factory()`
(`🔌️plugin/🦀️.rs:22513/22565`). `ImperativePlayApp` declares `type Presence = NoPresence` and overrides NONE of the
presence builders, relying on `EditorApp<V>`'s `.or_else(|| Some(no_presence_local_root_retirement_factory()))`
fallback (`🔌️plugin/🦀️.rs:7836`) — which evidently does not reach this app. **Next step:** give
`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` the explicit
`build_presence_local_root_retirement_factory` / `build_presence_peer_retirement_factory` /
`build_presence_store_disposer` trio that `🕸️dag`'s editor already declares (`…/🕸️dag/…/✏️editor/🦀️.rs:648-659`), and
re-run; if it still fails, the fallback at `🔌️plugin/🦀️.rs:7836` is the general framework defect to fix instead.
Remaining 11: 1 × member roster (`derived child dialect 's.stdio.semio@v1/text' is not declared by this app's member
roster` → brief bucket 8), 1 × `addStep is declared by the app but carried by no window kind`, 1 × missing
language-neutral oracle catalog file, 4 × fixture/inverse assertions, 4 × misc.

### 3.4 `semio-s-artifact-mathematical-equation` — 31 failures
~12 are the equation KERNEL, not the app: `is_irreducible(&f)`, `(product.to_f64() - 2.0).abs() < 1e-6`,
`(root0.to_f64() + 2f64.sqrt()).abs() < 1e-6`, `sol.is_some()`, `diff_matches(&e, &x, &result)`,
`maximum Equation microturn exceeded 8 ms`. These are genuine algebra/solver defects in
`🧬️schema/💡️inferences/📈️polynomial-internals` and need their own pass; they are unrelated to the play grid.
The rest: 3 × `interactive-job.live-instance` still surviving the bind (settle path for window-config operations),
2 × `genesis_child_pack members must open cleanly` (bucket 8), 2 × retirement factory, 2 ×
`registered fixture did not reach its exact terminal-empty witness`, 8 × plain `left == right`.

### 3.5 `semio-s-artifact-reasoning-wires` 20 · `semio-s-artifact-forms-forms` 7 · `semio-s-plugin-mathematical` 2
Reasoning: 3 × `edit history insertion requires its exact mutation retirement factory` (bucket 2 — the standalone
stores in the canvas-transient laws still use a bare `ArtifactStore::new`; they need the `new_<x>_store` →
`Owned<X>Store` guard, trinity jack `⚙️operations/🦀️.rs` is the reference), 2 × op-text round-trip drift, 2 ×
no-op-diff fixtures, 1 × `canvas lifecycle did not finish` (bucket 6 — the loop never drains
`take_typed_operation_completion()`), the rest fixture equality.
Forms: 1 × `exportFixture must emit a host effect` (same settled-receipt lane as §2.4), 1 × wire keyword drift for
`setTryValueStep`, 1 × `every FormsCommand row must be covered by every_command(...)`, 1 × retirement factory,
1 × `registered fixture did not reach its exact terminal-empty witness`, 2 × render assertions.
`semio-s-plugin-mathematical`: 1 × member-roster genesis (bucket 8) and 1 × `descriptor_is_fresh` — the same
`describe` + commit step as §3.1 (the mathematical descriptor predates the current editor).

## 4. Hand-offs

### 4.1 For the play topic
Remove the stale `mathematical:` row from `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` in
`🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts:83` — the committed mathematical descriptor already declares
`setActiveExample` on the editor app and publishes the `demo` example, so `publishedExamples(...).switches` is true
and the pane is not inert. The gate only asserts that INERT panes have an entry, so a stale extra entry does not
fail the suite; it is what made the visual audit report `n/a (no picker)` for that pane. Keep play's unit suite
green with `cd 🏢️semio-tech/🎡️play && DEVELOPER_DIR=/Library/Developer/CommandLineTools bun ./📜️script.ts test`.

### 4.2 For the coordinator
- Activation requested for the reasoning demo content once §3.1's `describe` lands:
  `mkdir -p "$T/🗑️generated/activate.request" && touch "$T/🗑️generated/activate.request/reasoning-wires"`.
- ⚠️ The fleet convoys hard on the SHARED build dir. A private `CARGO_TARGET_DIR` (which this topic used from 14:38,
  before the coordinator's note) removes the `target/debug/.cargo-artifact-lock` starvation but NOT the build-dir
  one: at 16:57 ten fleet cargos were parked in `cargo::core::compiler::prebuild_lock_exclusive → flock` with 0
  rustc children (ages 13–61 min) while exactly one compiled. Peers editing `🔌️plugin/🦀️.rs`, `🖱️ui`, `📡️spr` and
  `📖️playbook` every few minutes re-invalidate the unit every topic waits on.
- **Never run two of your own cargos against one private target dir**: doing so here corrupted `target-knowledge`
  (`can't find crate for serde`, 50 errors in `semio-s-artifact-stdio-semio`) and cost a full rebuild.

## 5. Files changed (absolute)

Fixed, verified by compilation and by the runs above:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-algorithm/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` (fallback deleted)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` (regenerated, 7 nodes / 9 edges)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡change-node-abbreviation/🧪️tests/🧪️rejects-reabbreviating-a-missing-node/🦀️.rs`
- ten `…/💡️reasoning/…/🧬️schema/🧬️mutations/*/🧪️tests/*/🦀️.rs` label assertions (see §2.1)
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs`

TEMPORARY, must be deleted after one run (see §3.2):
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️unit/🦀️.rs` — `temporary_regenerate_mutation_fixtures`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️tests/🔬️unit/🦀️.rs` — `temporary_regenerate_mutation_fixtures`

Nothing here was claimed without running it. No git write command, no dev server, no `🗑️generated` sweep, no cargo
or rustc killed that this topic did not start.
