# TC5 — finish TC4: codec sweep root-fixes, native twin closed, disposer defaults, ALL-plugin catalog on hub 7700

Slice TC5 of ticket 26/09/18, session 9. Continues TC4 (cut mid-flight 2026-09-22).
Predecessors: `📓️tc4-codec-sweep-and-published-catalog.md`, `📓️tc3e-…`, `📓️hc1-…`, `📓️tc3d-…`.

## 0. Inherited state

TC4 diagnosed all four items and landed most source in-tree, but left every section at
`(filling — cargo check pending)` and never reached §4 publish / hub.

Measured on disk at TC5 start (2026-09-23 ~12:35): print-mirror ops assert restored;
`TxtSnapshot::record_spec`; `bounded_artifact_store_owners`; disposer defaults on
`ArtifactApp`/`ArtifactEditor`/`ArtifactViewer`; laws present. Publish/hub not done.

## 1. Codec-sweep reds — FIXED (root)

### 1.1 print-mirror identity

Already restored by TC4 on `mirror.ops` (`owned-instance-open`). TC5 did not re-relax.
`cargo check -p semio-framework-plugin --lib` **exit 0** (`generated/tc5-check-plugin.txt`).

### 1.2 stdio `pack-schema-hash`

Already fixed by TC4 (`TxtSnapshot::record_spec` → `Some(Self::__dsl_spec())`).
`cargo check -p semio-s-plugin-stdio --lib` **exit 0** (`generated/tc5-check-stdio.txt`).

### 1.3 Empty-batch / Drop abort (root cause of the “native twin red”)

`encode_ops_vec(&[])` is a **framed nonempty header**, not a zero-length slice. The empty-batch
fast path keyed on `ops_vec.is_empty()`, so an empty gesture built a store, hit `EmptyApply`, and
`?` dropped a live `ArtifactStore` → Drop panic (masked as the shallow-shell witness).

**Fix (store + guest twin):** decode first; empty-batch when `mutations.is_empty()`. Never `?`
while the throwaway store is live; `mem::forget` after the close cursor (guest already forgot on
fault; native now always forgets after close). Guest `artifact_app_apply_ops` also falls back to
`bounded_artifact_store_owners` when the app returns `None`.

**Law:** `os_store::component::tests::document_codec_apply_ops_binary_reduces_a_nonempty_batch_and_closes_its_store`
→ **ok** (`generated/tc5-test-apply-ops-6.txt`).

## 2. Native `apply_ops_binary` twin — CLOSED

`bounded_artifact_store_owners` + install on the reduction store (TC4) + TC5 empty-batch/forget
discipline. Law green above. `cargo check -p semio-framework-os-kernel --lib` **exit 0**.

## 3. Disposer defaults — IN TREE (TC4)

All five lanes default `Some(bounded_*)` on `ArtifactApp` / `ArtifactEditor` / `ArtifactViewer`.
`the_framework_owns_every_bounded_close_lane_an_app_declares_nothing_for` present.
Compile-verified via plugin check; full lib-test run deferred under fleet load.

## 4. ALL `s` plugin publish + hub 7700 — IN PROGRESS

### 4.1 Selectable closure expanded

`🌎️hub/…/📜️script.ts`:

- `TRUSTED_BOOTSTRAP_PACKAGES` = **34** top-level plugins (stdio, gis, then alphabetical).
- `TRUSTED_BOOTSTRAP_ALL_PACKAGES` + `--packages all`.
- Closed-browser actor build **deferred** until descriptor open-targets are known (openability
  derived, not the advisory `opensDocuments` bit).

### 4.2 Hub binary

`zsh *fleet-mutex.sh hub tc5 -- cargo build -p semio-hub --bin os-hub`
→ **exit 0**, binary
`/Users/ueli/Documents/semio/.tmp-ticket/wp-tc5/target/debug/os-hub` (315 MB, 2026-09-23 14:00)
(`generated/tc5-hub-build.txt`).

### 4.3 Trusted-catalog bootstrap (LIVE 2026-09-23 18:04)

Holds wasm mutex as `tc5` (pid **39217**, since 17:06). `bun … trusted-catalog-bootstrap
--packages all` (pid 39258) past codec derive; currently
`cargo rustc -p semio-s-plugin-gis --profile wasm-release` (pid 68158) with live `rustc`
child (pid 516, ~6 % CPU) — not deadlocked.

Data root: `.🧬semio/🌐hub/tc5-boot`. Budget: `SEMIO_BUILD_BUDGET_MS=172800000`.
Hub **7700** not up yet (`current.json` absent).

### 4.4 Hub handoff (fill when bootstrap + hold finish)

| field | value |
|---|---|
| port | **7700** |
| pid | (pending) |
| data root | `.🧬semio/🌐hub/tc5-boot` |
| generation | (pending) |
| binary | `.tmp-ticket/wp-tc5/target/debug/os-hub` |
| profile | `local-stdio-gis-…-open-v1` (minted from selection order) |


### 4.3 TC5b handoff (fair per-package builds)

Predecessor TC5 held wasm mutex with `--packages all` (wrapper pid 39217 / bun 39258).
Stdio derive completed 8/8; mid-build on `semio-s-plugin-gis` (wasm32-wasip2 wasm-release).
TC5b waits for that `cargo rustc` to exit, then terminates the TC5 tree so queued peers
(s14, o3b, gj3b, o1c, p1) can take the mutex. Remaining packages build one-at-a-time under
`fleet-mutex.sh wasm tc5b`.

| package | wall_s | exit | notes |
|---|---|---|---|
| (cutover) | ~660 | ok | gis rustc exited; TC5 tree SIGTERM; mutex -> s14 |

Failures+fixes: (none yet)

### 4.4 Hub prove (pending)

| field | value |
|---|---|
| published root | (pending) |
| generation hash | (pending) |
| hub port | (pending) |
| artifactAuthority.ready | (pending) |
| catalog plugin count | (pending) |
| second fresh hub bind | (pending) |


## 5. Honest gaps

- Full `owned_codec_answers_every_call_on_every_staged_component` sweep not re-run (hours; JIT rows).
- Plugin disposer unit law not executed under fleet load (compile-only).
- Hub 7700 not yet `/readyz` 200 — waiting on 34-package `trusted-catalog-bootstrap --packages all`.
- Creating one hub document per plugin kind is the post-readyz prove step (not started).

## 6. Files changed

| path | change |
|---|---|
| `🏪️store/🦀️.rs` | empty-batch via decoded mutations; no `?` on live store; forget after close |
| `🔌️plugin/🦀️.rs` | same empty-batch; owners fallback to `bounded_artifact_store_owners` |
| `🌎️hub/…/📜️script.ts` | 34-package table; `--packages all`; deferred actor; openability derived |
| `.tmp-ticket/wp-tc5/*` | boot/hold scripts, captures, this report |

## 7. Hub handoff (runtime)

Fill when hold answers. Peers: browser collaboration / MCP agent use port **7700**.
