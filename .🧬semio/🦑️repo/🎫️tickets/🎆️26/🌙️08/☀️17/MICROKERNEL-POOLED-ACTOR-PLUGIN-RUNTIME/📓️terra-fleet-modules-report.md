# 🔎 `terra` — `fleet-modules-sweep` census + repair report

Scope: everything under `✏️s/🔨️modules/**` EXCEPT `🏗️fem` (owned by another agent, per brief — left
untouched, zero edits, verified with `git status` below).

## 1. Census — the actual size of the blind spot

Tool: `.🧬semio/…/terra-fleet-modules-census.py` (python3 over absolute paths, per R21 — cross-checked
with independent `\bdyn\b` / raw `fn ` grep passes, results identical, see §3).

`✏️s/🔨️modules` contains **four** module trees besides `🏗️fem`:

| module | contents | in scope |
|---|---|---|
| `🌐️spatial-kernel` | 3 TypeScript `component.ts` files (geometry, spatial, brepjs) | TS — no first-party Rust `dyn`/`async fn` surface, censused only |
| `🏗️fem` | 20 Rust `component.rs` files, 20 `dyn` uses | **excluded**, another agent's |
| `💭️mindmap` | `AGENTS.md` only, **no code files** | n/a — empty module |
| `📜️imperative` | 6 Rust `component.rs`/`glue.rs` files + 1 crate's `Cargo.toml`/`project.json`/`script.ts` | **Rust census + repair target** |

### Rust census — `📜️imperative` (6 files, crate `semio-s-imperative`)

| file | async fn | plain fn | first-party `dyn` | `🚫️async:` tags | `#[test] async fn` residue | `#[async_trait]` | `block_on` |
|---|---:|---:|---:|---:|---:|---:|---:|
| `⚙️engine/🦀️component.rs` | 16 | 0 | 0 | 0 | 0 | 0 | 0 |
| `📇️registry/🦀️component.rs` | 17 | 0 | 0 | 0 | 0 | 0 | 0 |
| `📝️compiler/🦀️component.rs` | 6 | 0 | 0 | 0 | 0 | 0 | 0 |
| `🧩️extension_sdk/🦀️component.rs` | 6 | 0 | 0 | 0 | 0 | 0 | 0 |
| `📦️packages/🦀️rust/📦️glue.rs` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `🧩️extension_sdk/📦️packages/🦀️rust/📦️glue.rs` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **total** | **45** | **0** | **0** | **0** | **0** | **0** | **0** |

**100% async, 0 first-party `dyn`, 0 `#[test] async fn` residue, 0 `async_trait`.** The two `glue.rs`
files are pure `pub mod`/`pub use` wiring (`#[path]` re-exports), no `fn` bodies. Both trait impls in
this crate (`neural_engine::Operator` for `ContributedExtensionStub`, `protocol::Identified<String>` /
`protocol::Patchable<Dictionary>` for `Step`) already use plain AFIT — no `dyn` anywhere, consistent
with O1 without further de-dyn work needed. Test fns already use
`#[semio_framework_async_macros::async_test]`, never bare `#[test] async fn` (0 residue, confirmed).

**No `🚫️async:` tags exist or are needed** — nothing in this crate matches E1–E5 (no external-trait
impls with fixed non-async signatures, no `const fn`/`extern "abi"`, no fn-pointer-slot values besides
the pre-existing `NativeRegistrar = fn(&mut Registry)` / `DEFAULT_CONTRIBUTIONS: OnceLock<fn() -> String>`
which were already plain fn pointers untouched by asyncify, no `block_on`/executor bridge).

`💭️mindmap` has no code to census — confirmed by directory walk, only `AGENTS.md` present.

### TS census (informational — no Rust `dyn`/async-tag rules apply)

| file | size | `async` markers (function/method) |
|---|---:|---:|
| `🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts` | 17,437 B | 0 |
| `🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts` | 139,131 B | 50 |
| `🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` | 173,099 B | 2 |

Not touched — this ticket's dyn/async-literal program (R1–R11) is Rust-specific; no TS-side rule in
`📌️important.md` calls for action here, so these are reported for completeness only.

## 2. Repair — what "standard treatment" found and fixed

**De-dyn (R11):** nothing to do — 0 first-party `dyn` in the whole `📜️imperative` tree (cross-checked,
§3). **Tagging (R2):** nothing to do — 0 E1–E5 candidates.

**What WAS wrong:** signatures were 100% asyncified but the **await-insertion pass had never run** on
this crate — every one of the four `component.rs` files had first-party async calls used as if sync
(bare values, not futures), which is silent, compiles-nowhere-until-checked breakage. Found by hand
(insert-await.py could not run — see §4 blocker) via full-file reads plus a differently-implemented
python cross-check for `async fn` names used as call-sites without a following `.await` (name-collision
aware per R10: only in-crate `async fn` names were tested, not blindly regexed against std methods).

Fixed, file by file (all edits re-read from disk before applying, re-verified after):

- **`⚙️engine/🦀️component.rs`**: `Executor::run`/`run_steps`/`run_step` chain was calling itself and
  `read_string_param`/`read_number_param`/`read_scope_bool`/`merge_output_into_scope` without `.await`
  throughout (11 call sites); 3 tests called `Executor::new(...)` and `.run(...)` unawaited.
- **`📇️registry/🦀️component.rs`**: `native_registrars()`/`registry_state()` (OnceLock accessors),
  `compose_registry`, `register_manifest_operators`, `merge_catalogue_sections`, `imperative_module_fields`,
  `ensure_bootstrapped` all called unawaited throughout production code, not just tests (13 call sites).
  One genuine **R10 residue-shape-#1** case: `ensure_bootstrapped`'s exactly-once guard used
  `OnceLock::get_or_init(|| { ...sync_imperative_module_contributions(...)... })` — a **sync** closure
  wrapping a call to an async fn, illegal as `.await`. Restructured to a manual check-then-set
  (`if BOOTSTRAPPED.get().is_some() { return }` … `BOOTSTRAPPED.set(())`) — benign under the
  single-threaded guest executor per R3, no atomicity lost that mattered (idempotent default-contributions
  seed).
- **`📝️compiler/🦀️component.rs`**: the harder residue shape. `compile_steps` fed
  `.iter().map(|step| compile_step(...)).collect()` — async call inside a sync `Iterator::map` closure
  (R10 residue #1) — hoisted into an explicit `for` loop. Same problem in `compile_step`'s
  `Option::map(|path| compile_steps(...))` branches (if/while/repeat body compilation) and in the
  operator-params formatting loop (`.map(|key| format!(…, format_value(…)))`) — all three hoisted to
  `match`/`for` with `.await` inside. Also missing awaits on `read_string_param`/`read_number_param`
  and 2 test call sites.
- **`🧩️extension_sdk/🦀️component.rs`**: `evaluate_json`, `build_manifest_json`,
  `imperative_module_topic_contribution` all called unawaited (3 sites).

**Total: ~30 missing `.await` insertions + 1 sync-closure restructure across 4 files, all within owned
paths.** None of this is dyn/tagging work — it is the await-insertion half of the same universal-async
program, apparently skipped by whatever earlier pass touched this crate's signatures.

## 3. Cross-checks (R21 — negative results reproduced with a second, differently-implemented query)

- `\bdyn\b` (unanchored, no first-party filter) over the 4 census'd files: **0** — matches the filtered
  census exactly, so the 0-dyn finding is not an artifact of the allow-list filter.
- Name-collision-aware call-site scan (in-crate `async fn` names only, balanced-paren check for a
  trailing `.await`) re-run after fixes: the only true positive left was `Executor::new` (now fixed,
  confirmed 3/3 call sites carry `.await`); every other flagged `new(` was `Vec::new`/`String::new`/
  `Dictionary::new`/`Registry::new`/`OnceLock::new`/`TopicContribution::new` — external/std types, not
  first-party async fns, correctly not touched (this is exactly the R10-documented name collision, caught
  by checking each hit against the in-crate async-fn set before touching anything).
- Post-fix full-file rereads of all 4 files confirm every remaining bare call to an in-crate async fn now
  carries `.await` (§2 lists were derived from, and match, this final state).

## 4. Blocker — could not get a green (or even reachable) `cargo check` for `semio-s-imperative`

```
$ CARGO_TARGET_DIR=<scratchpad>/target-fleet-modules cargo check -p semio-s-imperative --lib
```
First run: **could not compile `semio-framework-ui`** (transitive dep via `semio-framework-os-kernel`) —
`error[E0308]` ×3 in a **generated** file, `🧰️framework/🔨️modules/🖱️ui/…/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs`
(`Self::from_str`/`self.as_str()` returning `impl Future<...>` where a plain value is used) — outside my
owned paths, mid-flight elsewhere.

Second run (retried later): that error cleared, but **`semio-framework-os-kernel` itself failed to
parse** — `error: expected one of ',', ':', or '}', found '.'` ×3, warning count on that crate dropped
417→9 between the two runs, i.e. **another session is actively editing it right now**. Also outside my
owned paths (`🧰️framework/…`, not `✏️s/🔨️modules/**`).

**Consequence: none of my `.await` insertions in §2 have been compiler-verified.** They were derived from
full manual reads of each function's signature and every call site (not a blind name/regex sweep — R10),
cross-checked per §3, but I cannot paste a passing `cargo check` for this crate right now because the
build never reaches it. This is the documented "Concurrent Cargo Workspace Churn" pattern — not my bug,
not fixable from `✏️s/🔨️modules/**`. **Recommend the coordinator (or whichever packet owns
`semio-framework-ui`/`semio-framework-os-kernel` right now) re-run
`cargo check -p semio-s-imperative --all-targets` once those two crates are green, and report the exit
code** — that is the first real compiler evidence this crate's await-insertion pass will get.

## 5. What changed (owned paths only)

Modified (git status confirms exactly these 4, nothing under `🏗️fem` or outside `✏️s/🔨️modules`):
- `✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/📝️compiler/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs`

Ticket-folder scratch files added:
- `terra-fleet-modules-census.py` — the tool (kept per instruction, diagnostic-driven, not name-keyed
  await insertion — R10 compliant, safe to reuse/extend by a future packet)
- `terra-fleet-modules-census.txt`, `terra-fleet-modules-census-final.txt` — before/after census output
  (identical — hand-fixes touched only call sites, never signatures/dyn/tag counts, as expected)

No `lease-request` needed — no registrar-only file required editing.

## 6. What a sibling/coordinator must know

1. **`semio-s-imperative` cannot be acceptance-tested until `semio-framework-ui` and
   `semio-framework-os-kernel` are both green** — currently blocked by unrelated, in-flight work in both
   (see §4 for exact errors/paths). This is not new damage from me; it predates and outlasted this
   session's two build attempts.
2. **The `#[cfg(feature = "linked-modules")]` test in `📇️registry/🦀️component.rs`
   (`linked_modules_bootstrap_registers_text_operators`) references `super::linked_modules` — a module
   that does not exist anywhere in this file or crate, and `linked-modules` is not a declared feature in
   `Cargo.toml`.** It has presumably never compiled (cfg always false) since nothing declares the
   feature. Fixed its `.await`s for consistency but did not investigate further — orphaned/dead test,
   out of scope for an await-insertion pass; flagging for whoever owns this crate's feature surface.
3. **The de-dyn mandate (R11/O1) is a non-issue for `📜️imperative`** — it was already 0 `dyn` before I
   touched anything. The actual gap here was the await-insertion half of the program, not de-dyn — worth
   noting because the brief's "standard treatment" ordering (de-dyn → tag → tools) implicitly assumes
   dyn is the dominant defect in an uncensused tree; here it was zero and the real defect was elsewhere.
4. `✏️s/🔨️modules/🌐️spatial-kernel` (TypeScript, 3 files) and `✏️s/🔨️modules/💭️mindmap` (no code) needed
   no action under this ticket's rules.
