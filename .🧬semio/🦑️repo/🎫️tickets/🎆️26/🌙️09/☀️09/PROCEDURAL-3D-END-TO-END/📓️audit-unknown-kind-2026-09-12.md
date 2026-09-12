# Audit: `unknown kind: brep.curve.polygon` / `math.vector` / `brep.solid.extrude` after restage (2026-09-12)

Read-only audit. No source edited, no git-mutating command run, no ticket opened/closed. Every claim
below is either quoted from source with file:line, or backed by a `cargo test` run I executed myself
in this session (commands and output shown).

## 0. Headline finding

**The served plugin wasm is almost certainly stale relative to current source, not broken by a
still-live bug in `set-contributions`/`FlowHost`.** The exact native law that reproduces the served
guest's shape (nothing linked, everything contributed) — `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
— **passes** against current HEAD, painting `meshes=3` with zero `unknown kind`. But the dev-served
wasm predates the `set-contributions` command file itself by mtime:

| artifact | mtime |
|---|---|
| served dev wasm: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm` | **2026-09-11 15:51:26** |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` (the `apply`/invalidate/rearm handler) | **2026-09-11 15:58:47** — 7 min newer than the served wasm |
| `…/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs` (the `host_shaped` law added specifically to test the JSON.stringify shape the host sends) | **2026-09-11 16:33:29** — 42 min newer |
| `git log` HEAD (`989582baab`, a squashed auto-commit) | **2026-09-12 01:07:58** |

`✏️editor/🖥️host/🦀️.rs` and `📔️registry/🦀️.rs` (the registry-generation invalidation machinery) are
older than the wasm (2026-09-10), so those parts were already baked in — but the plugin-side command
file that actually calls `invalidate_for_flow_extension_registry` and re-arms was edited AFTER the
wasm the browser is running. The `📓️unknown-kind-after-restage-2026-09-11.md` probe's "restage wasm
14:15" timestamp is close to, but before, this 15:51/15:58/16:33 sequence — the ticket's own record of
"restage required: yes" for this exact file was written, then the file kept changing for another 40+
minutes. **A fresh restage (rebuild the procedural component + re-serve to 6018) is the single most
likely fix**, and it costs nothing to try before chasing a new hypothesis.

This does not prove the browser fault would disappear on restage — that requires actually restaging
and re-probing, which this read-only audit does not do — but every native law that exercises the
served guest's exact shape currently passes, and none of the "still broken" mechanisms hypothesized
in `📓️unknown-kind-after-restage-2026-09-11.md` reproduce natively (see §3, §4).

---

## 1. Where `unknown kind` originates and what table it needs

`grep -rn "unknown kind"` across `🧰️framework` and `✏️s` turns up exactly one producer of this literal
string (all the `unknown kind {other}` hits elsewhere are unrelated stdio/pdf/gif generators):

```
🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:975   UnknownKind(String),
🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:987   EvalError::UnknownKind(k) => write!(f, "unknown kind: {k}"),
🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:1536  let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
```

`Registry::dispatch` (`⚙️engine/🦀️.rs:1535-1548`) is the ONE call site. It looks up
`self.operator(operator_id)` — `self.operators: BTreeMap<String, OperatorRecord>` (`⚙️engine/🦀️.rs:1386`,
accessor at `:1479-1480`). This map is the "table that should contain `brep.curve.polygon`" after a
contributed manifest registration; it is populated only by `Registry::register_operator` calls, one of
which is `register_contributed_manifest` (below). `EvalError` is not cached in `NeuralCache` — its
`get_or_insert_with` (`⚙️engine/🦀️.rs:1696-1710`) only ever `seed`s a **successful** `Dictionary`, so a
persisted `unknown kind` cannot be a stale cache entry surviving an invalidation; every observation of
it is a genuinely fresh lookup failure at the moment it is printed.

Consequence for diagnosis: **if the operator is truly registered, `dispatch` never returns
`UnknownKind` for it** — it returns `Ok` (if a real installer is linked) or
`Err(EvalError::PendingExtension{..})` (if only a `ContributedExtensionStub` is registered — see
§2). The browser's `unknown kind: brep.curve.polygon` therefore means the operator id was **not**
present in `self.operators` in the `neural::Registry` snapshot the eval used — not a race, not a
schema mismatch downstream, not a cache issue.

### Registration path

```
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:124
fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) {
    let Ok(manifest) = crate::os_pack::json::from_json_str::<FlowExtensionManifest>(manifest_json) else { return };
    for schema in manifest.contributes.schemas { ... }
    for info in manifest.contributes.operators {
        if registry.operator_info(&info.id).is_some() { info.retire_cold(); continue; }
        let invocation_address = plugin_id.to_string();
        let operator_id = info.id.clone();
        registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(ContributedExtensionStub { invocation_address, operator_id }) }], &[]);
    }
    registry.finalize();
}
```

`ContributedExtensionStub::evaluate` (`📔️registry/🦀️.rs:107-112`) always returns
`Err(EvalError::PendingExtension{extension_id, operator_id, node_hash})`, **never** `UnknownKind` — so
a manifest that DID register would show up in the browser as a `PendingExtension`/`invokeExtension`
sequence (which is exactly what the ticket's OTHER, separate symptom —
`invokeExtension dispatch failed … window-transient publication is retiring a rejected authority`
— looks like: an operator that WAS resolved past `UnknownKind` but faulted on the host round trip).
The two console lines in `📓️unknown-kind-after-restage-2026-09-11.md` are therefore evidence of **two
different faults on two different nodes/generations**, not one: the widgets reporting `unknown kind`
never got past `Registry::dispatch`'s lookup at all.

`build_flow_extension_registry` (`📔️registry/🦀️.rs:141-153`) is the composition root: it installs
built-ins (no-op today, `:89-91`), then every `LINKED_FLOW_EXTENSION_INSTALLERS` entry (`:144-147`),
then folds every `ContributedFlowExtension` through `register_contributed_manifest` (`:148-150`). This
is rebuilt exactly once per `flow_extension_registry_generation()` bump (`:419-421`), on
`sync_host_flow_extension_contributions`/`_page`, `install_flow_extension_manifest`,
`uninstall_flow_extension`, or `install_flow_extension`.

---

## 2. Does `register_contributed_manifest` silently return on parse failure, and does the shape match?

**Yes, unconditionally.** `📔️registry/🦀️.rs:125`:

```rust
let Ok(manifest) = crate::os_pack::json::from_json_str::<FlowExtensionManifest>(manifest_json) else { return };
```

No error is surfaced, no fault raised, no counter incremented — a malformed `manifestJson` for one
extension silently contributes zero operators and zero schemas, while `registry.finalize()` still
runs and the registry replacement still publishes (bumping the generation), so a caller cannot
distinguish "installed 40 operators" from "installed 0 because the JSON didn't parse" without
inspecting the catalogue afterward. This is the exact failure mode the ticket names in
`📓️unknown-kind-after-restage-2026-09-11.md` §"Native tests are not the served shape".

**But the shape check comes back clean.** `FlowExtensionManifest` (used both to encode and decode)
lives in ONE crate, `semio-framework-os-flow`, aliased as `flow_extension_sdk` from every extension
crate's `Cargo.toml` (verified: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml:30`
and the sibling `math` crate). `build_manifest_json`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:70-79`) and
`register_contributed_manifest`'s `from_json_str::<FlowExtensionManifest>` decode the SAME Rust type —
not two independently-maintained mirrors — so a byte-for-byte encode/decode mismatch inside the Rust
boundary is structurally impossible.

The topic payload the host actually assembles is `{appId, extensionId, label, iconId, manifestJson}`
(`flow_extension_topic_contribution`, `🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:143-157`), and the fold on the
consuming side only ever extracts `manifestJson`:

```rust
🌊️flow/📔️registry/🦀️.rs:236-240
struct FlowExtensionTopicPayload { manifest_json: String }   // #[value(rename_all = "camelCase")]
```

so the four extra fields (`appId`/`extensionId`/`label`/`iconId`) are simply ignored by
`FromValue` — no exact-shape requirement there. The host's `scopeContributionsJson`
(`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:299-317`) forwards each contributor's OWN
`entry.manifest.topicContributions[i]` object verbatim (no host-side reshaping), and
`ProgramContributionEntry{pluginId, topicContribution}` (`🛂️manifest/🦀️.rs:3577-3583`) /
`TopicContribution{topic, payload}` (`:3601`) is the wire contract on both ends, again one Rust type,
not two.

I compiled this claim into a test rather than trust it: **I ran the exact test the ticket says was
added to check this ("the served shell sends page 0 of 1 as JSON.stringify-shaped contributions ...
not the ToValue round-trip")**:

```
$ cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib \
    -- set_contributions --test-threads=1
running 5 tests
test ...::a_one_page_host_shaped_run_indexes_contributed_operators ... ok
test ...::a_paged_run_installs_the_contributed_registry ... ok
test ...::an_invalid_page_address_is_refused ... ok
test ...::an_out_of_order_run_is_refused_and_discarded ... ok
test ...::the_packaged_brep_manifest_parses_as_a_flow_extension_manifest ... ok
test result: ok. 5 passed; 0 failed
```

`the_packaged_brep_manifest_parses_as_a_flow_extension_manifest` specifically feeds the REAL
`semio_s_plugin_flow_extension_brep::extension_manifest_json()` output through BOTH a third-party
`serde_json::Value` parse and the first-party `FlowExtensionManifest::from_value` path, and asserts
`brep.curve.polygon` survives both — this is CLAUDE.md's "same output with a third-party library"
requirement, already satisfied for this exact question. Byte-shape mismatch is not the live bug.

Compared against native tests feeding this: `dispatch()` in the same test file drives the payload
through the REAL `handle()`/`SetContributions` command, i.e. exactly the code path the served plugin
runs — no test-only shortcut.

---

## 3. `FlowHost::evaluate_step` and stale incremental baselines

Yes, `evaluate_step` does have a fast-path that skips re-dispatch, but it is keyed correctly by the
SESSION's baseline, which the contributions-rearm fix already clears:

```rust
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1074-1085
pub fn evaluate_step(&mut self, budget: usize) -> Vec<String> {
    self.drain_displaced();
    self.pending_extension_eval = None;
    let tree = self.build_tree();
    let seeds = self.build_seeds();
    let snapshot = TreeSnapshot::capture(&tree, &seeds);
    let dirty = compute_dirty_set(self.previous_snapshot.as_ref(), &snapshot);
    if dirty.is_empty() && self.previous_channels.is_some() && !self.outputs.is_empty() {
        tree.retire_cold();
        seeds.retire_cold();
        return Vec::new();
    }
    ...
```

`self.previous_snapshot`/`self.previous_channels` on a fresh `FlowHost` are seeded from the
**session's** copies via `flow_host_with_session` (`🖥️host/🦀️.rs:3248-3255`):

```rust
pub fn flow_host_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache_and_infos(fixture.clone(), session.neural_cache(), flow_neuron_kind_info_map());
    session.install_baseline_into(&mut host);
    if !session.eval_json().is_empty() { host.last_eval_json = session.eval_json().to_string(); }
    host
}
```

`FlowEvalSession::invalidate_for_flow_extension_registry` (`🖥️host/🦀️.rs:2767-2780`) is the fix from
`📓️contributions-rearm-2026-09-10.md`: it bumps the session's stored generation, sweeps the
`NeuralCache` epoch, and calls `set_eval_json(String::new())`, which (per its own body,
`🖥️host/🦀️.rs` around 2705-2725) takes `previous_snapshot`/`previous_channels` via `.take()` and
retires them. So on the NEXT `flow_host_with_session` call, `install_baseline_into` hands the fresh
host `None`/`None`, `compute_dirty_set(None, snapshot)` cannot be empty, and `evaluate_step`'s
fast-path is skipped — full re-dispatch happens. This is exactly what the still-passing
`a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted` law measures
end-to-end (§4): it reads `meshes=3` after the late install, which could not happen if the stale
baseline had survived. **This mechanism is not the live bug either**, at least not against current
source.

---

## 4. Native test coverage: which tests are linked, which is the one unlinked law, and its gap

`✏️editor/🧪️tests/🔬️test-support/🦀️.rs:9`:

```rust
pub fn lock() -> MutexGuard<'static, ()> {
    crate::flow_operators::installed();   // links brep + math via register_linked_flow_extension_installer
    TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
```

Every test that calls `test_support::lock()` — including all 5 tests in
`…/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs` I ran in §2 — runs with brep/math **already linked**
into the registry before any contribution is folded. `register_contributed_manifest`'s own dedup
guard (`📔️registry/🦀️.rs:130-133`, `if registry.operator_info(&info.id).is_some() { ...continue; }`)
then SKIPS installing the `ContributedExtensionStub` for every one of those ids, because the linked
installer already put a real, working `OperatorImpl` there. So `assert_contributed_kind("brep.curve.polygon")`
(reading `flow_neuron_kind_infos_json()`, itself built off the SAME registry) passes whether or not
`register_contributed_manifest`'s JSON-parsing path runs at all — these tests can be green while the
served, link-nothing guest is completely broken. This is the exact gap the ticket names.

**The one unlinked law**: `a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1531-1585`). It wraps the run in `UnlinkedFlowExtensions::take()`
(`:1642-1646`ff, an RAII guard around `unregister_linked_flow_extension_installer` /
`register_linked_flow_extension_installer`, `📔️registry/🦀️.rs:74-86`) so NOTHING is linked — the
served guest's real shape — then: loads the hexagonal-mushroom-column example, drains ticks, asserts
`phase: "faulted"`/`meshes=0` BEFORE any contribution; pushes the real paged `SetContributions`
command through the actual `dispatch_with_view`/retained job ladder; asserts the LAST page owes
exactly one `flowEvalTick` re-arm; drains that chain; and asserts every widget reads `"ok"` and
`meshes >= 1` — no test-only re-render, no manual tick, no direct field poke.

I ran it (it needs a larger stack — see §6 note):

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib -- a_late_contributions_install_re_arms --test-threads=1
test editor::generation3d::component::tests::a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted ... ok
test result: ok. 1 passed; 0 failed
```

**What it does NOT cover** (ticket's own words, confirmed by reading it): the evaluate hop that a
`PendingExtension` triggers is answered IN-PROCESS by `🔬️brep-extension`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1525-1530` docstring: "answered in-process by `🔬️brep-extension`"),
never through the host's real `invokeExtension`/`dispatchInvokeExtensionEffect`
(`🏛️ShellHost/🟦️.tsx:1707-1720`) → wasm-boundary pack round trip → window-transient publication
(`🔌️plugin/🪟️window/🫧️transient/🦀️.rs`) that the browser actually exercises. That TS/host layer is
where the SEPARATE `invokeExtension dispatch failed … window-transient publication is retiring a
rejected authority` fault lives (visible in the same 2026-09-11 probe, on `flow-extension-math`) —
this audit did not chase that one to a fix because it is a different failure mode gated behind the
`unknown kind` one actually clearing first, and the ticket already scoped it as a separate follow-up
("Fix host `invokeExtension` so evaluate does not publish through a window-transient authority
retired by the pre-contribution tick").

---

## 5. Proposed fix set, ranked by confidence

1. **(Highest confidence, ~zero risk, do this first) Restage.** Rebuild the procedural component
   (`activate-generation3d-react-dev` per `📓️flow-catalog-authority-2026-09-10.md`'s restage recipe,
   or the ticket's own wasm-restage step) and re-serve to `127.0.0.1:6018`. Evidence: §0's mtimes show
   the served wasm is 7-42 minutes OLDER than the very files that implement and test the fix; every
   native law that reproduces the served shape passes on current source. No code change proposed here
   — this is a build/serve action, out of scope for source edits.
   - **Native proof (already true, no restage needed to check it):** the 5 tests in §2 and the 1 law
     in §4, already green as shown above.
   - **What a browser probe should see afterward:** `[DEBUG] contributions push` still fires with
     `pageCount:1, skipped:null`; the fault line changes from
     `unknown kind: brep.curve.polygon/math.vector/brep.solid.extrude` to either a clean `evalLen`
     with `meshes > 0`, OR (if the SEPARATE `invokeExtension`/window-transient bug in §4 is still
     live) a NEW, DIFFERENT console line — `invokeExtension dispatch failed … rejected authority` —
     with `meshes` still 0. Either outcome is diagnostic progress; seeing the EXACT SAME `unknown
     kind` string after a confirmed-fresh restage would be the one result that reopens this audit's
     conclusion.

2. **(Medium confidence, real but currently-latent defect) Make `register_contributed_manifest` fail
   loudly instead of silently.** `📔️registry/🦀️.rs:125`'s `let Ok(manifest) = ... else { return }`
   should become a `Result`-returning function (propagating a `"flow.extension-manifest-invalid"`-style
   error up through `build_flow_extension_registry` → `sync_host_flow_extension_contributions[_page]`)
   so a malformed manifest for ONE contributor cannot silently zero out its operators while the
   command still reports success. This is not proven to be today's live bug (§2's shape check is
   clean and the byte-identical Rust type is shared on both ends), but it is exactly the kind of
   silent-failure class CLAUDE.md forbids leaving in place, and it is the ticket's own named
   candidate. **Native proof:** add a law that pushes a manifest with an intentionally-broken JSON
   body (e.g. truncate `manifestJson` mid-object) through the SAME unlinked path as §4's law, and
   assert the command now surfaces a `Fault` instead of silently registering 0 operators while
   returning `Ok`. No restage needed to write/run this law — it is purely a `neural::Registry` +
   `sync_host_flow_extension_contributions` unit test.

3. **(Lower confidence, speculative) Harden the host `invokeExtension` window-transient path.**
   `🏛️ShellHost/🟦️.tsx`'s `dispatchInvokeExtensionEffect`/`runCapturedExtensionEffect`/
   `captureExtensionCompletion` (`:1697-1721`) is where "window-transient publication is retiring a
   rejected authority" would have to originate — a completion captured against an activation
   (`captureExtensionCompletion(requestingPlugin, instanceId, req)`) that races a later, valid tick's
   publish once the pre-contribution fault's transient authority is torn down. This audit did not
   trace this fully (out of the "unknown kind" root-cause scope, and the ticket already earmarks it
   separately); flagging it here only so it is not lost, and because item 1's restage may surface it
   as the NEXT visible symptom once `unknown kind` clears. **Native proof:** none exists yet — this
   path is TS-only and would need a `🧪️tests/🔬️engine-contract` (React renderer) law driving two
   overlapping `flowEvalTick`/`invokeExtension` cycles across a contribution install, which is a new
   test, not a fix-and-verify against an existing one. **A wasm restage is not required** to
   investigate or fix this one, since it is host TypeScript, picked up by the dev server on save.

---

## 6. Incidental finding (not this ticket's fault, worth flagging separately)

`a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted` stack-overflows and
aborts (`SIGABRT`) under the DEFAULT test-thread stack size:

```
$ cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib \
    -- a_late_contributions_install_re_arms --test-threads=1
thread '...' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

and only passes with `RUST_MIN_STACK=134217728` (128 MiB), matching the env every `📓️*-2026-09-1*.md`
gate table in this ticket already uses. Anyone re-running this law without that env var will see a
false "it crashed" signal that looks like a regression but is a stack-size artifact of the test
itself (likely the same `--test-threads` default-stack issue prior lanes already worked around, not a
new discovery) — flagging only so it is not re-diagnosed as a fresh bug.

---

## Files read / evidence trail

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️unknown-kind-after-restage-2026-09-11.md`
- `.../📓️contributions-rearm-2026-09-10.md`
- `.../📓️contributions-example-scope-2026-09-11.md`
- `.../📓️flow-catalog-authority-2026-09-10.md`
- `.../📓️status.md` (`## 2026-09-11 *` sections)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` (`UnknownKind`, `Registry::dispatch`, `NeuralCache`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` (registration/registry/paging)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` (`FlowHost`, `FlowEvalSession`, `flow_host_with_session`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs` (`FlowExtensionManifest`, `flow_extension_topic_contribution`, `build_manifest_json`)
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (`ProgramContributionEntry`, `TopicContribution`, `parse_contributions`)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` (`buildContributionsJson`, `scopeContributionsJson`, `resolveDocumentOperatorKinds`, `exampleArtifactSources`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (contributions push loop, `dispatchInvokeExtensionEffect`)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` (`EXTENSION_ID`, `flow_extension_topic_contribution` call site, `extension_manifest_json`)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs` and its `🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs` (`generation3d_catalog_label` — ruled out as the source of "labels resolve")
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️test-support/🦀️.rs` and `.../✏️editor/🧪️tests/🔬️unit/🦀️.rs` (linked vs unlinked laws)

## Commands run (all read-only w.r.t. source; build artifacts only)

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- set_contributions --test-threads=1
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- a_late_contributions_install_re_arms --test-threads=1   # SIGABRT, default stack
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- a_late_contributions_install_re_arms --test-threads=1   # ok
```
