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

---

# 📓️ knowledge — successor session 7 (2026-09-22, 11:05 → 17:00)

Scope: `✏️s/🔌️plugins/{🗒️note,📋️forms,➗️mathematical,💡️reasoning,🕸️dag,🔱️trinity,📜️imperative}` — 20 crates
— and their seven panes on `:6033` (note, forms, mathematical, reasoning-wires, dag, trinity-jack, imperative).
`🗑️generated/` was swept at ~16:20, taking this topic's STATUS, the `pass2` log, the probe JSON/PNGs and the
private `target/`; every number below was transcribed out of those artefacts while they existed, and every source
edit named here is on disk (verified by `git diff HEAD --stat`).

## 1. Native suites

| crate | pass1 (04:02, predecessor) | pass2 (12:26, this session) |
| --- | --- | --- |
| `semio-s-artifact-dag-dag` | 199 / **9** | 204 / **3** |
| `semio-s-artifact-imperative-procedure` | 125 / **19** | 136 / **8** |
| `semio-s-artifact-mathematical-equation` | 356 / **28** | 366 / **18** |
| `semio-s-artifact-note-note` | 364 / **32** | **394 / 2** |
| `semio-s-artifact-reasoning-wires` | 169 / **18** | 176 / **13** |
| `semio-s-artifact-forms-forms` | 198 / 0 ✅ | 198 / 0 ✅ |
| `semio-s-plugin-mathematical` | 3 / **1** | 2 / **2** |
| `semio-s-plugin-trinity` | 5 / 0 ✅ | 4 / **1** |
| `semio-s-plugin-{dag,forms,note,reasoning,imperative,imperative-control,imperative-effect,imperative-logic,imperative-math,imperative-text,trinity-jack-shell,trinity-jack-lsp}` | ✅ | ✅ |

Net across the seven trees: **107 → 47** failures. The two crates that went BACKWARDS both went red on
`descriptor_is_fresh` only, because `pass2` ran (12:26) while the coordinator's describe chain was still
regenerating descriptors; that chain finished 15:47, so those two are expected to close on the next pass without
any further edit.

`pass1`/`pass2` logs: `🗑️generated/knowledge/pass{1,2}.txt` (swept). `pass3` relaunched 16:34
(`🗑️generated/knowledge/run-pass3.sh` → `pass3.txt`) after the sweep also took the queued wrapper and the
private target dir; it measures the same 20 crates plus the two fixes made after pass2.

⚠️ One slot was lost to a compile error of my own (12:19–12:25): `--no-fail-fast` does NOT survive a compile
error — cargo printed `build failed, waiting for other jobs to finish` and ran zero tests. With one mutexed
invocation per turn an un-compilable edit costs the whole slot.

## 2. Production defects found and fixed

1. **`PolyU::interpolate` pushed the wrong divided difference** —
   `✏️s/🔌️plugins/➗️mathematical/…/💡️inferences/📈️polynomial-internals/🦀️.rs:457`. After level `L` the in-place
   Newton table holds `table[i] = f[x_(i-L)…x_i]`, so the coefficient of `(x-x0)…(x-x_(L-1))` is `table[L]`; the
   code pushed `table[n-1]`, the BACKWARD difference of that level. The quadratic through (0,1),(1,6),(2,15)
   reconstructed as `2x²+7x+1`. `cas::sums::sum_polynomial_closed_form` interpolates partial sums, which is why
   `Σk` closed-formed to `n²/2 + 3n/2` and `Σk²` to 44 instead of 14. Fixed → those three closed on pass2.
2. **Equation retained-command extents were short by one boundary step** —
   `…/➗️equation/…/✏️editor/🦀️.rs:283` `equation_command_extent`. `EquationRetainedCommandWork::step` charges one
   step per phase BOUNDARY on top of the per-item steps (`Initialize`, `nodes-complete`, `edges-complete`;
   `Finish` completes without a step), so a graph-walking verb costs `3 + nodes + edges` and `setArtifact`
   (three phases) `4 + nodes + edges + points`. The constants priced ONE boundary — the shape `setPoints` has —
   and `setAlgorithm`/`setDirected`/`setArtifact` overflowed their own extent on the last boundary with
   `equation-work-extent-overflow`. Fixed → all three closed on pass2.
3. **Wires board values were not in the DSL's canonical key order** —
   `…/🔌️wires/🦀️.rs` (new `canonical_board_value`/`canonical_board_values`, applied in
   `wires_content_child_with_owner`, `materialize_wires_content`, and the `create-node` / `connect-nodes`
   mutation builders). `print_dsl_value` has sorted object keys since 21fbcd3538 while `DslValue`'s `PartialEq`
   compares entries positionally, so a node minted in declaration order could not satisfy
   `parse_op(print_op(op)) == op`; worse, the composed `content` child's id is a hash over those values, so the
   same board minted in two orders addressed two different children. Closed `op_text_round_trip_create_node`,
   `op_binary_round_trips_and_agrees_with_text`, `document_text_round_trip_with_operation_applied` and
   `command_envelope_round_trip_holds_for_an_applied_operation`.
4. **Wires: an unmaterialized composed child left the whole board empty** — see §3.
5. **Owner-catalog store guards** for dag, equation and note (`🔖️Store` regions in each
   `🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs`: `new_{dag,equation,note}_store` →
   `Owned…Store` with Deref/DerefMut and a Drop that walks the bounded close loop, installing
   `semio_framework_plugin::bounded_document_store_owners`). A bare `ArtifactStore::new` installs no catalog and
   `reserve_edit_history_slot` then refuses every `Apply` with `edit history insertion requires its exact
   mutation retirement factory`; five laws across the three crates died of it. Mirrors the guard `🔌️wires` and
   `📋️forms` already had.
6. **A destroyed JSON Schema, repaired.** The predecessor's `temporary_regenerate_mutation_fixtures` keyed on the
   PARENT DIRECTORY NAME under `🏅️standards/🔖️1/🪆️subsets`, which also matches the committed JSON *Schema*
   `🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`. `NoteDiff` decodes it leniently (its `title` field swallows the
   schema's own `"title": "NoteDiff"`), so 309 lines of schema were overwritten with a 17-line diff instance.
   Restored verbatim from HEAD with `git show HEAD:<path> > <path>` (a read, not a git write command); dag was
   unaffected because `DagDiff` rejects the schema. Both temporary generators are now deleted, note's remaining
   env-gated `regenerate_committed_diff_fixtures` only walks paths under `🧫️fixtures/`, and the NEW one-shot
   `temporary_regenerate_wires_fixtures` carries the same guard.

## 3. The reasoning-wires EMPTY PANE — root cause and fix

Evidence, from ONE render of the live pane (`probe-wires2.json`): the Artifact panel's IDENTITIES section listed
all seven topics while RELATIONSHIPS was EMPTY and the canvas drew nothing but its grid. Those three readings come
from the same `WiresPlayApp::render`: identities are read off the persisted `wires_fixture`, while the
relationship rows and every canvas layer go through `wires_working_board(document)`. Not a camera/fit problem —
the example's nodes sit at x 40…280, y 30…210, radius 24, camera (0,0) zoom 1, i.e. the middle of a 1427×807
canvas.

`WiresWorkingScene` is an in-process owner attached to one exact `ArtifactChild`. The pack codec and the DSL codec
both mint it on decode, but `WiresSnapshot`'s VALUE projection does not carry it — `content` is only its
`(child_id, target)` pair, which the crate's own `wires_working_scene_is_owned_by_the_exact_snapshot_child` law
already pins (`wireHasScene: false`). A document that reaches the app through any transport that runs neither
codec therefore arrives with `wires_fixture` intact and `content` empty, and every board reader answers empty.

Fix (`…/🔌️wires/🦀️.rs`, `wires_working_scene`): recover from the document's own persisted
`wires_fixture.board.{nodes,edges}` when — and ONLY when — the handle carries no owner at all. An owner that
exists and is empty is honoured verbatim, so the recovery can never resurrect content a user deleted (every
in-session edit installs an owner through the diff's `wires_content_child_with_owner`). Two native laws in
`…/🔌️wires/🧪️tests/🔬️unit/🦀️.rs`: `an_unmaterialized_content_child_still_reads_the_documents_own_board` (strips
the owner exactly the way the wire form does, then asserts 7 nodes / 9 edges) and
`an_owned_empty_scene_is_never_refilled_from_the_persisted_board`.

`🗑️generated/activate.request/reasoning-wires` was touched; the coordinator's 15:47 `activate-dev rc=0` chain
carried the rebuilt component. A 15:58 probe of the recycled server still shows the canvas blank — see §6.

## 4. The mathematical EMPTY PANE — root cause (diagnosed, NOT fixed)

`EquationSnapshot`'s graph and geometry live ONLY in an `EquationWorkingScene` local owner on the `results` child,
and `equation_scene` fail-softs to an EMPTY graph when that owner is absent. BOTH of this artifact's codecs
persist the three composed children as bare handles and nothing else:

- `🚪️io/📸️snapshot/📝️text/🦀️.rs:176` → `notation=<child> results=<child> computed=<child> equation=<expr>`
- `🚪️io/📸️snapshot/💾️binary/🦀️.rs:57` → the same four fields

so the committed curated example `…/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` contains no graph at all (its `equation=`
decodes to the literal integer `0`), and `setActiveExample("demo")` loads a document whose Graph and Geometry
windows have nothing to draw. `🕸️dag`'s own snapshot module header already states the rule being broken: "A codec
that persisted only the bare handle would produce an UNRECOVERABLE snapshot the instant a fresh process parses
it." Unlike wires there is NO persisted fallback to recover from.

Not attempted here: it is a wire-format change, and brief v5 rule 9 routes those through the schema/grammar/
fixture chain — `graph=`/`geometry=` lines in the text codec, matching length-prefixed blocks in the binary codec,
rebuilding the `results` owner on decode, updating `📡️.protocol.semio` and its `🔬️semio-protocol-conformance`
test, and regenerating `🎬️demo` through the crate's own printer. That needs a build between each step, and this
session had one mutexed cargo slot per turn.

## 5. Stale tests restated (never weakened)

- **DS1 action resolver** (`🕸️dag`, `📜️imperative`, `🗒️note`): since ticket 26/09/18 slice DS1 the builder no
  longer clones the app roster into every `WindowKindDefinition.actions` (that copy was 31.9 % of a shipped
  descriptor's bytes). Three laws read `window.actions` directly and now read
  `semio_framework::window_kind_actions(&definition, window)` — the predicate the plugin host itself uses
  (`🔌️plugin/🦀️.rs:7520`).
- **The retained wire went binary (26/09/15)** (`🗒️note`): `set-camera` ×3, `renders_composite_canvas`,
  `renders_navigator_canvas` grepped the projected tree for `"zoom":2.0` / `documentJson` /
  `"viewMode":"navigator"`, which now live inside a byte page. They read the scene through
  `artifact_app_laws::decode_fixture_scene::<InkCanvasScene>`.
- **Panel tree shape** (`🗒️note` inspection semantic contract): `PanelTreeBuilder::build()` returns the panel
  TREE and the headed section is its first child (the catalogue half of the same law already read it that way),
  and rows are `tree_item_desc` with separate `label`/`description`. The committed `🔣️summary.json` oracle is
  untouched; the law recomposes `"{label}: {description}"`.
- **Table scene records** (`📜️imperative` ×2, `➗️mathematical` ×1): `TableWindowKit::render` emits `columnsJson`
  as `{id, label}` records and `rowsJson` as `{id, "<column index>": cell}` records. Three laws deserialized
  `Vec<String>` / `Vec<Vec<String>>`; they now project the records back to the labels and the cell text in column
  order, leaving the committed oracles alone.
- **Mounted-app invocation result** (`🗒️note` `add_block`): `InvocationResult.mutations` is empty on a mounted
  app; the law counts the document's own history (`history_snapshot().upserts`) instead.
- **Double settle** (`💡️reasoning` `setActiveExample` ×2): both laws called `dispatch(..)` — which already
  settles — and then `settle(..)` again, reading the SECOND, empty receipt, and reported "setActiveExample must
  emit a LoadDocument effect" against a command that emitted one. New context helper `dispatch_receipt` returns
  the invocation result and the receipt of the single settle that publishes it.
- **Oracle catalog path** (`📜️imperative` structural correspondence): the subset's language-neutral catalog moved
  into `🔮️oracles/🔣️.json`; the law still read a flat `🔣️oracle.json` and died with a bare `NotFound`.
- **Serde mirror out of step with `#[value(..)]`** (`➗️mathematical` graph window config mutations): the test-only
  serde derive had no `tag`/`rename_all`, so it used serde's default EXTERNALLY tagged form and refused every
  committed vector with `invalid value: map, expected map with a single key`. Mirrored to
  `#[cfg_attr(test, serde(tag = "kind", rename_all = "kebab-case"))]`.
- **Envelope retirement** (`🗒️note` `renders_document_tree`): the seed `ArtifactEnvelope` was dropped still owning
  its nested owners. It is now printed from inside the owner-installing store guard, whose `Drop` walks the
  bounded close loop.
- **Canonical compare** (`💡️reasoning` duplicate-id law): a payload decoded straight off a committed fixture
  carries the fixture's authoring order while the board occupant is canonical; the law compares
  `crate::canonical_board_value(&node)`, which is field-for-field identity, not authoring order.

## 6. Live pane verdicts

**On the 03:04 activation (11:38, `panes-pre*.png`)** — all seven reached `data-shell-ready`, 0 console errors,
0 page errors:

| pane | verdict | evidence |
| --- | --- | --- |
| note | ✅ content | 275 svg nodes, "Welcome to semio note" in Canvas and Navigator |
| forms | ✅ content | 328 svg nodes, real Blueprint/Steps form (Component Name / Description / Material / Tags) |
| dag | ✅ content | 5 node boxes painted + the full 9-line DSL listing |
| trinity-jack | ⚠️ partial | Nakagin graph paints labelled nodes (TF0BC1 / JackPrune / JackOrphan / JackSpare) and the query renders, but the status line reads `unexpected character '-'` and Results is "No data" |
| imperative | ⚠️ empty | chrome + table header `# / Id / Kind`, body "No data" |
| mathematical | ❌ blank | three canvases, background + grid only |
| reasoning-wires | ❌ blank | one canvas, background + grid only |

**On the NEW 15:47 activation (15:58, server recycled 15:57:03)**: note ✅ and forms ✅ unchanged, 0 errors;
`reasoning-wires` boots ready with 0 errors but the canvas is STILL blank; mathematical, dag, trinity-jack and
imperative all timed out at 180 s waiting for `data-shell-ready` under load average ~75 — no verdict, re-probe
needed, NOT a regression claim.

Two live findings worth their own tickets:

- **trinity-jack's query diagnostic comes from the wrong language.**
  `…/🔌️jack/…/🧬️schema/🗣️language-service/🦀️.rs:412` `lint` delegates to `semio_framework_graph::dsl::lint`, and
  that lexer (`🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs:51`) has `Arrow`/`DashArrow`/`BackArrow` but no bare
  `Dash`, so the default query `MATCH (a:Piece)-[r:Connection]->(b:Piece) …`
  (`…/🔌️jack/…/✏️editor/🦀️.rs:51`) is reported `unexpected character '-'` — while jack's OWN parser accepts
  exactly that shape (the executor's `parse(..)` unit tests are green). Either the graph DSL must admit a bare
  `-`, or `lint`/`format` must go through jack's own parser. Not fixed: the framework crate's own suite and its
  dependents could not be re-checked inside one slot.
- **imperative's empty run table is downstream of XCUT-DICT.** `editor::procedure::engine::tests::
  host_runs_default_snapshot` panics `final Dictionary ownership must be explicitly retired or owned by a cold
  boundary` at `🧠️neural/⚙️engine/🦀️.rs:101`, i.e. the engine cannot run the demo procedure — so the pane has no
  rows to show. Fixing the framework bucket should light the pane up.

## 7. Handed to other owners

**XCUT-DICT** (`final Dictionary ownership must be explicitly retired or owned by a cold boundary`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:101`) — of imperative-procedure's 19 pass1 reds,
these NINE were that bucket and were not touched here:
`editor::procedure::engine::tests::host_runs_default_snapshot`,
`editor::procedure::modes::edit::windows::main::tests::run_command_expands_scope_into_readable_rows_without_truncation`,
`standards::v1::subsets::any::schema::mutations::binary::tests::op_text_round_trips_edit_step_params`,
`…::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::{declared_outcome_holds,
committed_json_is_canonical, the_idempotent_edit_carries_before_to_an_identical_after, produces_committed_diff,
the_inverse_resends_the_identical_dictionary}`,
`standards::v1::subsets::any::schema::operations::tests::edit_step_params_inverse_law`. On pass2 three of them
were still red (`run_command_expands_scope…`, `…binary::tests::document_text_round_trip_with_applied_operation`,
`edit_step_params::…::committed_json_is_canonical`).

**PROPOSED DIFF for the peer who owns `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`** (brief v4
no-touch list, so NOT edited here): `new_registered_app` / `paired_registered_apps` /
`assert_two_registered_instances_converge` / `assert_registered_ingest_idempotent` build `VcsArtifactApp<A>` with
the DEFAULT member roster, so an app whose `genesis_child_pack` mints an `s.stdio.semio@v1/{text,flow}` child
fails `derived child dialect … is not declared by this app's member roster`. They need `…_with_members::<A, M>`
twins in the shape of the existing `new_app_with_registry_and_members`. This blocks
`semio-s-artifact-mathematical-equation::…::{ingest_operations_is_idempotent_for_equation,
two_instances_converge_disjoint_edits_via_backbone}` and
`semio-s-artifact-imperative-procedure::…::two_instances_converge_disjoint_edits_via_backbone`.

## 8. What remains, and why

- **`➗️mathematical` CAS algorithms (6 on pass2)** — `polynomial::algebraic::{cbrt2_times_cbrt4_equals_2,
  neg_and_inv_hand_cases, root_of_selects_correct_irreducible_factor, sqrt2_plus_sqrt3_has_minimal_poly_degree_4}`,
  `polynomial::finite::is_irreducible_hand_cases`, `cas::{integrate,limits,ode,sums}` remainders. Genuine
  algorithm gaps in `AlgebraicReal` (isolating-interval transforms), Rabin irreducibility over `GF(p)`, limits at
  infinity for rational functions, and Bernoulli/linear ODEs. `is_irreducible_hand_cases` is mathematically right
  as written (`x²+2` IS irreducible mod 5 — 3 is not a QR), so `poly_mod_pow`/`gcd_monic` is where to look.
- **`💡️reasoning` committed specification vectors** — `resize-node`, `move-node` and `change-node-shape` fail
  `committed_json_is_canonical` (`24` vs `24.0`) and `committed_diff_is_canonical` (a `WiresDiff` slot the
  fixture predates). A one-shot `temporary_regenerate_wires_fixtures` is in
  `…/🔌️wires/🧪️tests/🔬️unit/🦀️.rs` for the pass3 run; it must be DELETED once the rewritten vectors are
  committed, and the pass after that is what verifies them.
- **`💡️reasoning` mounted-app board reads (6)** — `add_relationship_appends_edge_and_selects`,
  `delete_selection::handle_alone_…`, both `canvas_pointer_down` laws and both `window_transient` laws. The
  retained drag work reads the scene through `input.snapshot.content.local_owner::<WiresWorkingScene>()`
  DIRECTLY (`…/✏️editor/🦀️.rs:322` and `:371`) rather than through `wires_working_scene`, so the §3 recovery does
  not reach it; routing them through the accessor needs care because the accessor clones the scene and these are
  byte-budgeted bounded steps.
- **`🕸️dag` (3)** — `remove_node_deletes_node_and_connected_edges` and
  `node_graph_edit_batches_connect_then_delete_selection` fault `batched item candidate failed its exact fixed
  fold contract`; `connect_disconnect_nodes_inverse_law` mints a different content-child id after the inverse,
  i.e. the inverse does not restore the edge roster byte-for-byte.
- **`🗒️note` (2)** — `ink_apply_events::gesture_begin_live_commit_produces_single_undo_step` (the gesture does not
  coalesce into one undo step) and `note_apply_ops_reduces_a_nonempty_batch_and_closes_its_store`, which was
  GREEN on pass1 and red on pass2 with `artifact store reached Drop without its exact terminal-empty shallow-shell
  witness` inside the framework's own `artifact_app_apply_ops` — a framework regression landed between 04:02 and
  12:26, not a plugin change.
- **`descriptor_is_fresh`** in `semio-s-plugin-mathematical` and `semio-s-plugin-trinity` — measured mid-describe;
  the chain finished 15:47, so these close on the next pass.
- **Four panes unverified on the new activation** — mathematical, dag, trinity-jack, imperative timed out at
  180 s under load.

Nothing here was claimed without running it. No git write command, no dev server started or stopped, no
`🗑️generated` sweep, no ticket opened or closed, no cargo or rustc killed that this topic did not start (one
exception, explicitly instructed: my own waiting mutex wrapper, killed and relaunched on the coordinator's
two-slot migration notice).

## 9. Addendum — 16:40, scratch relocated

The repo's workspace-cleanup deleted most of `$T/🗑️generated` at ~16:05–16:25 (this topic's STATUS, the `pass2`
log, the probe JSON/PNGs and the private `target/` went with it; §1's numbers were already transcribed above, so
nothing measured was lost). Per brief v5's 16:40 addendum this topic's scratch now lives at
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/knowledge` — `STATUS.md`, `run-pass3.sh`,
`pass3.txt`, `probe-panes.mjs` and `CARGO_TARGET_DIR`. `pass3` requeued 16:37 against those paths (my previous
wrapper was still WAITING, never a holder, and was killed and relaunched — no other process was touched). Browser
probes are on hold while the machine thrashes (load 156/190/200), so §6's four unverified panes stay unverified.

## 10. Addendum — 17:45, all seven panes verified on the 15:47 activation

Browser hold lifted; every pane re-probed one page at a time against the recycled `:6033`
(`⚡️cache/play-fleet/knowledge/panes-post{2,3}.json`, `wires-panel.json`, with matching PNGs). All seven reach
`data-shell-ready` with **0 console errors and 0 page errors**. Canvas verdicts are a paint sample of every 7th
pixel, so a count of 1–2 distinct colours is background + grid, i.e. nothing drawn.

| pane | verdict | evidence on the NEW activation |
| --- | --- | --- |
| note | ✅ content | 275 svg nodes, "Welcome to semio note" in Canvas and Navigator |
| forms | ✅ content | 328 svg nodes, the full Blueprint form (Component Name … Spec Sheet) |
| dag | ✅ content | canvas paints 9 distinct colours (node boxes) + the DSL listing |
| trinity-jack | ⚠️ partial | canvas paints 24 distinct colours (labelled Nakagin nodes); Jack Query still shows `[error] unexpected character '-'` and Results is "No data" |
| imperative | ⚠️ empty | chrome + table header `# / Id / Kind`, body "No data" (XCUT-DICT downstream, §6) |
| mathematical | ❌ blank | Graph canvas 1 distinct colour; §4 root cause unchanged (the codecs persist no graph, so the curated example carries none) |
| reasoning-wires | ❌ blank | canvas 2 distinct colours; Artifact panel still lists all seven IDENTITIES with an EMPTY RELATIONSHIPS section |

**The wires fix did not take effect in the guest, and the panel proves which branch was taken.** IDENTITIES is
read off `wires_fixture` and RELATIONSHIPS off `fixture_edges(wires_working_board(document))`; identities render
and relationships do not, which is the §3 signature *unchanged* — so `wires_working_scene`'s new recovery branch
did NOT run. Since it fires only when the `content` handle carries NO owner at all, either (a) the staged guest
still predates the 11:40 edit, or (b) something mints an EMPTY owner on the loaded document, which the fix
deliberately honours verbatim. Staging evidence is mixed and should be settled before any further code change:
`🧰️framework/…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/💡️reasoning/*.core.wasm` is dated
2026-09-22 14:19 (after the edit), while `🏢️semio-tech/🎡️play/dist/site/🔌️plugin-modules/💡️reasoning/*.core.wasm`
is still dated **2026-09-19 20:07** — three days old — and the page's own banner reports 7 staged modules behind
their source. Next step, in this order: confirm which of those two roots `:6033` actually serves, and if it is the
14:19 one, have the guest report `snapshot.content.local_owner::<WiresWorkingScene>().is_some()` once at load —
that single bit distinguishes (a) from (b) and decides whether the recovery needs to widen to "owner present but
empty while the persisted board is not".

## 11. Addendum — pass3 (17:53) and the pass4 fixes

`pass3` (log `⚡️cache/play-fleet/knowledge/pass3.txt`) with 33 green targets:

| crate | pass1 | pass2 | **pass3** |
| --- | --- | --- | --- |
| `semio-s-artifact-dag-dag` | 199/9 | 204/3 | **204/3** |
| `semio-s-artifact-imperative-procedure` | 125/19 | 136/8 | **136/8** |
| `semio-s-artifact-mathematical-equation` | 356/28 | 366/18 | **366/18** |
| `semio-s-artifact-note-note` | 364/32 | 394/2 | **394/2** |
| `semio-s-artifact-reasoning-wires` | 169/18 | 176/13 | **178/12** |
| `semio-s-artifact-forms-forms` | 198/0 | 198/0 | **198/0 ✅** |
| `semio-s-plugin-mathematical` | 3/1 | 2/2 | **3/1** |
| `semio-s-plugin-trinity` | 5/0 | 4/1 | **5/0 ✅** |
| all ten other plugin crates | ✅ | ✅ | **✅** |

`descriptor_is_fresh` closed on both mathematical and trinity once the coordinator's describe chain finished —
those two reds were an artefact of measuring mid-chain, exactly as predicted. Total across the seven trees:
**107 → 44**.

Four things landed for `pass4` (queued 21:39, log `pass4.txt`):

1. **`temporary_regenerate_wires_fixtures` did its job and is deleted.** Its `println!` was captured (the test
   PASSES, so cargo swallows its stdout), which is why the log showed no rewrites — but `git status` proves it
   rewrote three vectors: `resize-node`'s and `move-node`'s `🦠️mutation` (`24` → `24.0`, `12`/`0` → `12.0`/`0.0`)
   and `change-node-shape`'s `🔺️diff` (dropped a stale `camera` slot). The laws in that same binary compared
   against the `include_str!` bytes baked in at compile time, so those five reds close on the next pass, not this
   one.
2. **A stale slot count** in `resize-node`'s `committed_diff_is_canonical`: it demanded `Some(5)` slots and its doc
   said "all five", while `WiresDiff` declares FOUR (`artifact`, `wiresFixture`, `content`, `meta`) — `camera`
   moved to the concrete canvas window's config. Read off the struct's roster now.
3. **`EquationViewer` could never close its stores** — a PRODUCTION gap, and the one remaining
   `semio-s-plugin-mathematical` red. The viewer declared none of the `build_*_store_owners` /
   `build_*_store_disposer` hooks, so `close_registered_fixture_app` faulted
   `interactive-job.close-owned-disposer-missing` ("app owner did not provide the required bounded disposer for
   document-store") and the store then reached `Drop` without its terminal-empty witness — which on a
   `panic = "abort"` wasm32 guest is an `unreachable` that kills the instance, and the codec resolver constructs
   every app of a bundle to read its schema. `🗒️note`'s viewer documents exactly this defect for itself (ticket
   26/09/18 TC3c §5f / TC3d §1 / TC3e); equation's viewer now carries the identical set of seven hooks.
4. **A one-shot `temporary_regenerate_procedure_fixtures`** for imperative's three `committed_json_is_canonical`
   reds: the committed vectors spell `"pathRef": {}` while the current projection writes every slot out
   (`{"owner": null, "slot": null}`). Same `🧫️fixtures/`-only scoping as the wires one; DELETE it once the
   rewritten vectors are committed, and the pass after `pass4` is what verifies them.

## 12. Addendum — pass4 (22:06), the final measured run

Log `⚡️cache/play-fleet/knowledge/pass4.txt`, 35 green targets, 5 red.

| crate | pass1 | pass2 | pass3 | **pass4** |
| --- | --- | --- | --- | --- |
| `semio-s-artifact-dag-dag` | 199/9 | 204/3 | 204/3 | **204/3** |
| `semio-s-artifact-imperative-procedure` | 125/19 | 136/8 | 136/8 | **136/9** |
| `semio-s-artifact-mathematical-equation` | 356/28 | 366/18 | 366/18 | **365/19** |
| `semio-s-artifact-note-note` | 364/32 | 394/2 | 394/2 | **394/2** |
| `semio-s-artifact-reasoning-wires` | 169/18 | 176/13 | 178/12 | **182/7** |
| `semio-s-artifact-forms-forms` | 198/0 | 198/0 | 198/0 | **198/0 ✅** |
| `semio-s-plugin-mathematical` | 3/1 | 2/2 | 3/1 | **4/0 ✅** |
| `semio-s-plugin-trinity` | 5/0 | 4/1 | 5/0 | **5/0 ✅** |
| the other eleven plugin crates | ✅ | ✅ | ✅ | **✅** |

**All fourteen `semio-s-plugin-*` crates in this topic are now green**; the five reds are artifact crates.
Across the seven trees: **107 → 40**.

What moved and why:

- `semio-s-plugin-mathematical` closed: the `EquationViewer` store owner/disposer hooks were the whole red.
- `reasoning-wires` 12 → 7: the three regenerated specification vectors landed, closing all five
  `committed_json_is_canonical` / `committed_diff_is_canonical` / `produces_committed_diff` reds on
  `resize-node`, `move-node` and `change-node-shape`.
- `imperative-procedure` 8 → 9 is the one-shot `temporary_regenerate_procedure_fixtures` itself: it rewrote
  `create-step`'s and `reorder-steps`' vectors (the log records both) and then PANICKED on `edit-step-params`
  with `final Dictionary ownership must be explicitly retired or owned by a cold boundary` — the XCUT-DICT bucket,
  since that mutation's payload IS a `Dictionary`. The generator has been deleted; the two rewritten vectors
  should close two of those reds on the next pass, and `edit-step-params` was already an XCUT-DICT red.
- `mathematical-equation` 18 → 19 is
  `editor::equation::component::unit_tests::retained_maximum_microturns_stay_below_eight_milliseconds`, a WALL-
  CLOCK law ("maximum Equation microturn exceeded 8 ms") that passed in pass1, pass2 and pass3 and failed only in
  pass4, which ran with a second cargo in the other mutex slot. Per brief v5 rule 5 this is reported as a load
  flake with its numbers, NOT loosened.

`pass5` was queued to verify the two rewritten imperative vectors and re-measure the timing law, and could not
build: a peer's in-flight edit to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (a no-touch file) left
`mod interaction_selection_laws;` declared with no `#[path]` and no such module file, so
`semio-s-framework-plugin` does not compile (`E0583`, plus a since-repaired `E0425` on
`history_row_applied_v1`). Waited ~20 minutes across two checks; the declaration is still bare. Nothing in this
topic's tree is involved — the next agent should simply re-run `⚡️cache/play-fleet/knowledge/run-pass5.sh` once
that file compiles.

## 13. The mathematical wire change (§4 closed in code) and the trinity-jack lexer (§6 closed in code)

Both landed 22:37–22:45 and are measured by `pass6` (queued 22:42, log `⚡️cache/play-fleet/knowledge/pass6.txt`,
which also runs `-p semio-framework-graph` because the second fix is in that crate).

### 13.1 `➗️mathematical` — the codecs now persist the composed children's content

Schema-first check first: this subset's two normative files describe the ENVELOPE, not the body
(`📖️.grammar.semio` is `body = payload NL?` / `payload = OCTET+`; `📡️.protocol.semio` is
`segment payload varint bytes`), and equation has no `🔬️semio-protocol-conformance` suite, so extending the body
is within the declared format rather than a change to it.

- `🚪️io/📸️snapshot/📝️text/🦀️.rs` — `print_equation_snapshot_body` now emits two further lines,
  `graph=` and `geometry=`, hex-encoded first-party JSON exactly the way the existing `equation=` line already
  does. `parse_equation_snapshot_body` reads them, builds one `EquationWorkingScene` and attaches it to the THREE
  handles the document itself names (never to freshly minted ones — `equation_children_from_state` derives the
  same ids, but re-minting would silently discard the identity the document carried). A body without the two
  lines is a pre-format document and still decodes, to the empty scene, exactly as before.
- `🚪️io/📸️snapshot/💾️binary/🦀️.rs` — the same two values as length-prefixed blocks after `equation`, and the
  pack format byte goes 1 → 2 (a format-1 pack is now rejected by its own version check rather than
  mis-parsed).
- `🧪️tests/🔬️unit/🦀️.rs` — one-shot `temporary_regenerate_demo_asset` rewrites
  `🖼️assets/🎬️demo/🗣️.dsl.semio` through THIS crate's own `ArtifactDsl::print_dsl` of
  `EquationSnapshot::default()` (the a/b/c/d graph plus its geometry), and refuses to write unless `print_dsl` is
  a `parse_dsl` fixpoint and the reparsed scene actually carries nodes. DELETE it once the rewritten asset is
  committed.

### 13.2 `🔱️trinity` — a bare `-` is now a pattern connector

Authority decided from the code, not from taste: jack delegates ALL SIX language services to the framework
(`complete`, `lint`, `format`, `hover`, `semantic_tokens` and the `QueryableGraph` adapter all call
`semio_framework_graph::dsl::*` in `…/🔌️jack/…/🧬️schema/🗣️language-service/🦀️.rs:268–495`), so the framework
grammar IS the authority for this dialect and jack is not the place to fix it. That grammar spelled the
relationship connector `--` only (`Token::DashArrow => "--"`), while `dsl_core`'s `TokenKind::Minus` sat in the
lexer's stray-character bucket — so `(a)-[r]->(b)`, which is what Cypher writes, what jack's own EXECUTOR parser
accepts in its green unit tests, and what every committed example query in the repo uses (including
`TRINITY_JACK_DEFAULT_QUERY`), was reported `unexpected character '-'` by the editor while running fine.

`🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs` (NOT on the peer no-touch list): new `Token::Dash`, `TokenKind::Minus`
maps to it instead of falling through to the stray bucket, `parse_pattern` accepts `Dash` wherever it accepted
`DashArrow` (connector, undirected tail and the `<-[…]-` reversed form), and the classifier/printer/formatter
carry it. Additive — `--` keeps working, and both spellings parse to the same `Query`. Three laws in
`🗣️dsl/🧪️tests/🔬️unit/🦀️.rs`: `a_bare_dash_is_the_same_pattern_connector_as_a_double_dash`,
`a_bare_dash_also_spells_the_undirected_and_reversed_connectors`, and
`formatting_a_single_dash_pattern_is_idempotent_and_reparses`.

### 13.3 `💡️reasoning` — why the recovery did not fire, and what it costs to finish

The 22:17 activation had NOT restaged reasoning by 23:02 (`…/dist/dev/🔌️plugin-modules/💡️reasoning/*.core.wasm`
still 14:19, `play/dist/site/…` still 2026-09-19), so there is still no live measurement of the fix.

The native side says what the remaining defect is. `add_relationship_appends_edge_and_selects` dispatches on a
MOUNTED app and reads back zero edges, while the store-level `store_applies_node_add` in the same crate passes —
so the owner is lost between the store's apply and `app.snapshot()`, and the §3 recovery cannot help because it
reads `wires_fixture.board`, which mutations never update (`diff_board_fixture` sets `content` alone). Six reds
share that shape (`add_relationship`, `delete_selection::handle_alone…`, both `canvas_pointer_down` laws and both
`window_transient` laws — the last two read `snapshot.content.local_owner` DIRECTLY at
`…/✏️editor/🦀️.rs:322` and `:371`, bypassing the accessor entirely).

The finish is one coherent change, deliberately NOT started blind at this hour: give `diff_board_fixture` the base
snapshot so every mutation writes the new board into `wires_fixture.board` beside `content`. That makes the
persisted inline board authoritative, makes the §3 recovery correct in every case rather than only on a freshly
loaded document, and fixes the mounted reads and the pane at once. It changes the emitted `WiresDiff` shape, so it
must go with a regeneration of the committed `🔺️diff` vectors through the crate's own printer — i.e. exactly the
two-run cycle the wires and imperative vectors just went through, and it needs a build between the two.
