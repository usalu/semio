# 🧪️0️⃣ W13d — the three wgpu suites and their Nx targets to zero red

Packet W13d of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Lane: the last red laws of
`semio-framework-ui --features wgpu-engine --lib`, `semio-framework-os-renderer-wgpu --lib` and
`semio-framework-os-infinite --lib world:: terrain`, their three Nx targets, and the TS gates.

Logs: `🗑️generated/w13d-*.txt`.

---

## 1. Before / after

| suite | before | after |
| --- | --- | --- |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -j 4 -- --test-threads=1` | **567 passed, 1 failed** (`w13d-ui-1.txt`) | **568 passed, 0 failed** (`w13d-ui-2.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -j 4` (**parallel**, no `--test-threads`) | **567 passed, 1 failed** (`w13d-ui-parallel.txt`) | **568 / 0, four consecutive runs** (`w13d-ui-parallel-{1..4}.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1` | **904 passed, 2 failed** (`w13d-renderer-1.txt`) | **938 passed, 0 failed** (`w13d-renderer-final.txt`) |
| `cargo test -p semio-framework-os-infinite --lib -j 4 -- --test-threads=1 world:: terrain` | **199 passed, 0 failed** (`w13d-world-1.txt`) | **204 passed, 0 failed** (`w13d-world-final.txt`) |

The renderer's total rises 906 → 938 and the ui's 568 → 571 because peers landed laws throughout the
packet; two of the renderer's new families were red on arrival and are fixed here (§7).

| Nx target (`NX_DAEMON=false … --skip-nx-cache`) | before | after |
| --- | --- | --- |
| `@semio-tech/ui-rs:test-wgpu-engine` | ✅ exit 0 | ✅ **571 run, 571 passed** (`w13d-nx-ui-final.txt`) |
| `semio-framework-os-infinite:test-wgpu-world-terrain` | ✅ exit 0 | ✅ **204 run, 204 passed** (`w13d-nx-world-final.txt`) |
| `@semio-tech/framework-renderer-wgpu:test-wgpu-unit` | ❌ **killed by its own 15 s budget**, no red test in it (`w13d-nx-renderer.txt`) | ✅ **938 run, 938 passed, 47.8 s** (`w13d-nx-renderer-final.txt`) |

| TS gate | result | log |
| --- | --- | --- |
| `nx run-many -t lint,check-browser-worker,check-frame-worker,test-browser-worker -p @semio-tech/framework-renderer-wgpu` | ✅ **all four green**, re-run after every change | `w13d-ts-gates-final.txt` |
| `nx run @semio-tech/ui-styling-tokens:check-generated` | ✅ exit 0 | `w13d-styling-check-final.txt` |

`test-browser-worker` was **89 passed / 2 failed** at `📓️w6a` §gates; it is green on this tree.

---

## 2. The presenter source contract — re-derived, not deleted

`async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied`
(`🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`). Red since `📓️w9a` §7, with the exact seven
failing clauses recorded in `📓️w11b` §4. All seven were re-evaluated mechanically against the current
tree first (scratchpad script, findings only kept here) — the same seven, no more, no fewer.

**The presenter changed shape, and the contract did not follow.** `RuntimePresentationAuthority` grew a
second pair of fields (`admitted_scene_revision` / `admitted_input_generation` / `admitted`) and the
presenter's admission gate moved from the LIVE pair to the ADMITTED one
(`🧊️renderer/🦀️.rs:10016` `admit_build`, `:10023` `admitted()`, `:13694` the gate). That is a real
parity fix — `current()` moves several times per host tick, so a presenter re-reading it faulted frames
it had itself built correctly (`prepared render revision is stale: live=35, packet=27`,
ticket 26/09/09) — and every clause that named `current()` went stale with it.

| stale clause | what the code does now | new clause |
| --- | --- | --- |
| `…witness_for(self.generation.0)` **×1** | the frame transaction reads it TWICE, both keyed on its own generation: the per-step supersession guard (`:12065`) and the `Build` arm that mints the cursor (`:12164`) | count **== 2**, plus `if base_witness != current_witness {`, `self.base_witness = Some(current_witness);`, `runtime.admit_build_presentation_witness(presentation_witness);` and `FrameBuildCursor::new(presentation_witness)` |
| `let expected = self.presentation_authority.current();` **×2** | one gate, reading `admitted()` (`:13694`), covering the native and wasm arms | `…admitted();` **== 1** AND `!glue.contains("let expected = self.presentation_authority.current();")` |
| `packet.scene_revision() != expected.scene_revision \|\| …` | the acknowledge phase compares the packet against the FROZEN `RasterTextureWitness` (`:13835`), which is what "freshness decided once" actually means | the `raster_witness.scene_revision != packet.scene_revision() \|\| raster_witness.preview_generation != packet.preview_generation()` term, plus `raster_operation_authority.begin(expected.scene_revision, expected.input_generation)` |
| `self.presentation_authority.mark_scene_changed();` **×2** | one of the two call sites is the free `enqueue_runtime_completion(…, presentation, …)` (`:10144`), spelled `presentation.mark_scene_changed();` | `glue.matches("mark_scene_changed();").count() == 2` — both spellings, one census |
| `glue.contains("runtime_presentation_authority_and_candidate_identity_change_independently")` | the law is in the async-boundary case, not in the renderer source; the cross-reference never existed there | replaced by a **compile-time** reference, `const _: fn() = runtime_presentation_authority_and_candidate_identity_change_independently;` (`🔬️wgpu-renderer-async-boundary/🦀️.rs`, above the law). Renaming or deleting the law now fails the BUILD instead of quietly narrowing the contract — which is exactly how the text-marker version rotted. |
| `draw.contains("mesh_gpu_retirement_preserves_acknowledged_versions")` | the law moved to `🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs:237` | new `DRAW_LAWS_SOURCE` include, clause `laws.contains("fn mesh_gpu_retirement_preserves_acknowledged_versions()")` — anchored on `fn …()` so a mere mention cannot satisfy it |
| `draw.contains("fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner")` | same, `:13` | `laws.contains("fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner()")` |

**React ref** added to the contract's docstring: R3F's demand frameloop is the same law. `invalidate()`
(`🌐️World3dHost/🟦️.tsx:2358` and its twelve siblings) marks the scene changed and the next raf renders
the scene as it stands when the render BEGINS; React never refuses a queued frame because something
invalidated again in between — the later `invalidate` supersedes it with a new frame. Freshness is the
producer's decision on both targets. The presenter's own docstring (`🧊️renderer/🦀️.rs:13555`) still
claimed it read `current()`; corrected to `admitted()` with the reason.

**The mutation battery was rebuilt, not trimmed.** The sixteen 7-tuples became a flat
`PRESENTER_RETIREMENT_MUTATIONS` table of `(source, from, to, replace_once)` over a
`PresenterContractSource` enum, and the loop now also asserts that each rewrite actually CHANGED its
source — a no-op mutation used to "pass" silently. **28 mutations**, up from 16, including the two the
re-derivation invites: `admitted()` → `current()` (the regression the docstring is about) and dropping
`admit_build_presentation_witness`. Base contract accepts; all 28 are refused; no mutation is a no-op.

---

## 3. The `World3dState` byte-budget receipt — recommitted

`scenes::admitted_surface_map_tests::admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack`.
Red since a peer added `brush_mesh_run: Option<WorldBrushMeshRun>` to `World3dState`
(`📓️w11b` §1.2, `📓️w12d` §7.2). Measured **22 920 / 48 944**, committed 22 752 / 48 608.

- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — the `renderer::scenes` row now
  reads the measured pair.
- `…/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs:85` — docstring paragraph saying what the numbers
  ARE, so the next person does not treat a mismatch as a broken law: they are a **receipt**, not a
  ceiling; `World3dState` is a live element's surface payload and every field moves them; what the law
  defends is the two invariants AROUND the numbers (owner ≪ `capacity × elementSizeBytes`, i.e. the
  slots are heap-first and not an inline `[T; N]`; and the table still constructs inside
  `boundedThreadStackBytes`). Recommitting the measured pair is the correct response; widening the
  guard or dropping the row is not.

---

## 4. The wall-clock budget law — made deterministic

`wgpu::engine::tests::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` asserted
`max_slice < Duration::from_millis(8)`. It measured the MACHINE: 22.9 ms (`📓️w8a`), 27.5 ms and 35.8 ms
(`📓️w2-w6` §6), 174 ms under a full fleet (`📓️w2a`), **31.5 ms in this packet's own baseline**, and
green on a quiet box. A red run carried no information.

The eight milliseconds were a PROXY for the property that actually holds the frame budget: one
`step_layouts` call advances the admission cursor by exactly one unit, so a slice costs the same whether
the tree has ten nodes or ten thousand. The engine already REPORTS those units
(`UiLayoutStep::Yielded { stage, nodes, glyphs }`), and the admission ladder is one unit per call by
construction: `admit_node_one` answers `(1, 0)`, `admit_text_one` shapes ONE scalar and answers `(0, 1)`,
`unwind_one` answers `(1, 0)` when it pops a frame (`📌️mounted_layout/🦀️.rs:483`, `:556`, `:598`).

`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs:215` — renamed to
`large_layout_and_shaping_job_admits_one_work_unit_per_slice`, the clock removed, and asserting instead:

- `slices > 10_000` (kept — the chunking is observable);
- `widest_slice == (1, 1)` — **no slice admits more than one node and one glyph**, over every stage;
- `admitted == (2 × node census, Σ label scalars)` — every node visited and unwound once, every scalar
  shaped once, so the law cannot pass by doing nothing.

Strictly stronger than the duration it replaces, and machine-independent. Two census call sites follow
the rename: `📜️script.ts:10783` (`law(engineSource, …)` markers) and
`🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts:66` (the law-mutation needle).

---

## 5. The two `wgpu::prepared` flakes — isolation-safe

`📓️w9a` recorded that `packet_drop_retires_nested_backings_and_permit_scalars_separately` and
`pending_presenter_witness_rejects_superseding_packet_with_exact_owner` contend under `-j 4` and must be
run serially. **Running the suite in parallel reproduced it here** on the first try
(`w13d-ui-parallel.txt`): `prepared render process permits exhausted (held items 4/64 pages 6144/16383)`.

**Root cause, two halves, both in `🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs`:**

1. The case already had a `Mutex` guard — and only **11 of its 39 laws took it**. The prepared ladder's
   credits (`PREPARED_RENDER_PROCESS_PERMITS`, the atlas pool, the four abandonment rings) are
   PROCESS-wide by design, so any of the other 28 could perturb a law that reads a process counter. The
   two names in `📓️w9a` were the symptom, not the culprits.
2. Holding the lock is not enough. A dropped owner lands in an abandonment ring and keeps its permits
   until something grants it a close step, so a law that ends with owners still queued hands the next
   one a spent budget — which is what the failure above actually was.

**Fix:** the guard is now taken as the first statement of **all 39** laws, is renamed
`prepared_process_guard` / `PREPARED_PROCESS_TEST_LOCK` for what it really guards, and **drains both
rings itself** before handing the lock back, so every law starts from a quiescent process whatever the
law before it left and whatever order the runner picked. Docstring at the guard states the law.

**Proof:** four consecutive fully parallel runs of the whole ui suite, 568 / 0 each
(`w13d-ui-parallel-{1..4}.txt`). This matters because the Nx targets run under nextest with
`--test-threads 7`, not serially.

---

## 6. The `test-wgpu-unit` Nx target — killed by its own budget, not by a test

`@semio-tech/framework-renderer-wgpu:test-wgpu-unit` failed with **no red test in it**:

```
[budget] cargo nextest run … --test-threads 7 --profile fundamental … exceeded 15000ms — killed.
Trim it, or assign it to a higher level (quick/long/exhaustive).
```

`📓️w2h` wired the target with `resolveTestLevel(segments)`, i.e. the `fundamental` 15 s budget. The
suite is 912 laws over the os renderer: ~68 s serial, ~25 s across seven threads on an idle machine, so
it was killed mid-run at **every** invocation.

**Fix:** `…/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:85` — `resolveTestLevel(segments, "long")`,
the parameter that exists for exactly this ("the floor a suite declares when its own fixed cost already
exceeds a lower level's budget, so the suite is levelled honestly instead of being killed at every
invocation"), following the same file's `PreviewGeneratedTestScript` precedent. Docstring records the
measurement and why its two siblings (568 and 199 laws) keep the fundamental budget.

**Verified green** after the fix: 938 tests run, 938 passed, 47.8 s — comfortably inside the `long`
budget and no longer truncated by fail-fast.

Getting there took six blocked attempts between 21:06 and 21:47, every one of them dying in
`semio-s-artifact-stdio-pdf`, a crate a peer was rewriting live (45 errors: `PdfPage::text` field →
method, `PdfSnapshot` gaining 23 fields, `PdfInfo` gaining 4, `base::io::text_document` removed). It
reaches the renderer's test build through the crate's dev-dependency closure and had nothing to do with
this packet. One attempt also reported two errors in the renderer's own lib-test build — `UiTreeNode`
unimported in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14547` (mtime 21:01, W13a's dock / W13b's chrome lane) —
which the same lane cleared on its own. Nothing was touched in either; the run was simply retried until
the tree compiled.

---

## 7. `agent_bridge` — the SSOT grew three frames and the wgpu twin was left behind

Once the budget fix let the target actually RUN, nextest's fail-fast stopped at two families that were
**not in this packet's baseline** — they landed at 21:17–21:22, while it was in flight:

```
AgentToolCall did not decode: UnknownTag(8)
AgentMessage did not decode: UnknownTag(9)
```

A peer extended the bridge protocol with an agent-conversation lane and landed it in three of the four
places it lives: the Rust SSOT `🌉️mcp/🧵️bridge/🦀️.rs` (21:17, `GatewayToShell::AgentToolCall` tag 8,
`AgentToolResult` tag 9, `ShellToGateway::AgentMessage` tag 9), the shared corpus
`🧵️bridge/🧫️fixtures/📨️frames.json` (21:19) and the React twin `🔗️AgentBridge/🟦️.tsx` (21:22). The
**wgpu twin was not touched** (mtime 09-17 23:56, a day old — no live worker in it), and the corpus
replay is precisely the anti-drift mechanism that catches that.

**Fix — the third implementation brought up to the SSOT**, `🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`:

- the three frames on both enums, encoded and decoded byte-for-byte as the SSOT writes them (`8`:
  invocation/tool/arguments; `9` gateway→shell: invocation/tool/`ok`/summary; `9` shell→gateway:
  message-id/text);
- the consumer's conversation mirror, one-for-one with React's `AgentConversationEntry`:
  `AgentConversationEntry::{UserMessage,ToolCall,Approval}`, `AgentToolCallState`,
  `AgentApprovalState`, and `AGENT_CONVERSATION_MAX_ENTRIES = 200` with the same oldest-first trim.
  A tool result settles ITS OWN call in place (React's `updateConversationEntry`), never a second row;
  an approval is one row from request to decision, whether the gateway resolves it or this shell's
  human does; `send_agent_message` mints `msg_<session>_<ordinal>`, echoes the turn the moment the
  frame is queued, and refuses blank text — React's `sendAgentMessage`, exactly.

**Four new laws** in `🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs` cover the in-place settle, the orphan
result that must record nothing, the single approval row, and the echo + bound; the two corpus replays
now assert the full **10** gateway→shell tags and **6** modelled shell→gateway variants. 16 / 16 green.

---

## 8. Skipped, with attribution

- **`semio-s-artifact-stdio-pdf`** (45 compile errors) and **`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`**
  (`UiTreeNode` unimported at `:14547`) — both peers' live lanes, both blocking the renderer's test
  BUILD for ~40 minutes (§6). Not touched; both cleared on their own and the final gates are green.
- **`verify interactivity p5e`** — red at HEAD, before and after this packet, and not one of this
  packet's gates. Two findings while confirming the law-census call sites of §4:
  - `interactivityLiveReconcileSelfTests` throws first ("live reconcile self-test
    `per-surface-credit-cap` made no source mutation") — a peer's live reconcile/reactor lane.
  - `interactivityMountedLayoutTextFailures` had **two mutation needles that no longer match HEAD**, so
    the battery was silently non-discriminating: `slots: [Option<UiSurfaceSlot>; …]` (the registry is
    `Box<[…]>` at HEAD, heap-first) and `StepBudget::new(1, now.saturating_add(1))` (now
    `StepBudget::from_duration(1, now, 1000)`). Both fixed here, in the `requireAll` census
    (`📜️script.ts:10738`, `:10755`) and in the two mutation rows — **all 28 mutations and all 15 law
    needles now discriminate**. The census's CLEAN baseline still reports eight drift failures over
    eight source files (forbidden `Vec<` / `while `, worker-turn, preview-retention, snapshot-authority,
    shared-pool-opportunity and route-order markers). That is a whole stale-census audit of its own and
    is left open, unchanged by this packet.

---

## 9. Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`
  — presenter contract re-derived, `DRAW_LAWS_SOURCE`, 28-row mutation table, compile-time law reference.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `present_step`
  docstring corrected to `admitted()`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs`
  — receipt rationale.
- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — `renderer::scenes` measured pair.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs` — work-unit chunking law.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs` — universal draining guard.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts` — three needles.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`
  — three SSOT frames, conversation mirror, `send_agent_message`.
- `…/🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs` — corpus counts, four conversation laws.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts`
  — `test-wgpu-unit` at the `long` level.
- `📜️script.ts` — three P5c census markers.
