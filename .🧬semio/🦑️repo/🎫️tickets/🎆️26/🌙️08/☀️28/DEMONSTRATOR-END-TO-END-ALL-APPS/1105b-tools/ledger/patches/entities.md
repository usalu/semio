# tool-run entities port (ledger, prepared 07:55, UNAPPLIED)

Apply: `python3 C/1105b-tools/ledger/patches/entities.py` (fail-closed; `--check` verifies anchors only, passes today).
Compile: `C/1105b-tools/ledger/chk2.sh ledger-N --lib -p semio-framework-tool-run-entities`, then `-p semio-framework-tool-run-tick-wire` and the presentation/native crates below.

## Mapping (old -> new)
| Site | Old | New |
|---|---|---|
| retained-map insert/remove cursor `advance` | `(grant)` | `(grant, effect)` (hostcheck-125) |
| cursor `close_step` | `(grant)->Outcome` | `(effect)->OriginalRetirementPoll`; settled by `settle_close` (new helper in entities root) |
| `StagedSourceAdmission::advance` | `(&mut out, grant)` | `(&mut out, effect)->RetainedWorkOutcome` |
| `RetainedOrderedMapCloneCursor::advance` | `(source, body, grant)` | `(source, body, max_work, effect)->OriginalRetirementOutcome` (failure type OriginalRetirementFailure: converted by dropping it and keeping `retained_progress()`) |
| `...::advance_demands` | `(source, body)` | `(source, body, max_work)` (u64::MAX in quotes) |
| `...::take` | `(grant)->Outcome<Option<(T,P)>>` | `(&mut Option<T>, effect, reply)`; reply from `OriginalEffectReply::borrow(&mut slot)`, result read back with `slot.take()??` |
| `ControlledRetirement::step` | `(grant)->Result` | `(effect)->OriginalRetirementPoll` |
| entities `ToolRunEntityEdit::advance`, `ToolRunEntityVersionEdit::advance`, `ToolRunEntityProvenanceEdit::advance` | `(grant)` / `(body, grant)` | `(grant, effect)` / `(body, grant, effect)` |

`ToolRunEntityEditGrant` itself is unchanged (grant.retirement stays the physical grant; `effect.grant()` must equal it).

## Callers to port after the patch (not in the script)
- `⏯️tool-run/📊️progress/🪟️presentation/🧬️update/🦀️.rs:20,60-71`: `advance(self, grant)` -> add `effect`; its inner `admission.advance`, `clone.advance`, `child.advance` take the same replacements as version/provenance (clone `take` via reply slot).
- `.../🪟️presentation/🗃️slot/🦀️.rs:26`: `advance(&mut self, grant)` -> `(grant, effect)`; forwards `update.advance`, `seed.advance`.
- `🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🛬️native/🦀️.rs:75-76`: replace `let result=presentation.advance(grant);` by the job-side provider from the runtime addendum: `cx.with_original_effect(grant.retirement, fuel, |effect| presentation.advance(grant, effect))?` (None = quote not admitted: yield Progress(default), nothing charged); keep the single `consume_retained` debit.
- Tests `👥️entities/🧪️tests/🦀️.rs:11,30,47,65,77`: `job.advance(grant)` -> run each turn inside a fixture effect (copy of `value/retained-clone/🔗️source/🧪️testing` `with_effect`, or the equality `🧪️tests/effect.rs` helpers `eff::with_effect`).

## 08:35 compile result (ledger-140.txt, value lib green): entities has 24 own errors AFTER entities.py; they are a second layer
1. `RetireOwned` shape changed: `fn retirement(self)->Box<dyn RetirementCursor>` is gone; impls need `type RetirementCursor=...; fn retirement_seed(self)->Self::RetirementCursor`.
   - `State<K>` (🦀️.rs:42), `Editor<K>` (:47), version `State`, provenance `State`: replace the manual/`artifact_retire_struct!` impls by `#[derive(semio_framework_value::RetireOwned)]` (generic bounds as on the struct), or `artifact_retire_struct!` for the non-generic ones.
   - `ToolRunEntityEdit<K>` / `ToolRunEntityVersionEdit` / `ToolRunEntityProvenanceEdit`: `type RetirementCursor=<State as RetireOwned>::RetirementCursor; fn retirement_seed(mut self)->Self::RetirementCursor{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement_seed()}`; keep `retirement_birth_bytes`, `controlled_retirement_supported`, `retirement_element_copy_bytes`.
2. `SealedShared` API (value/♻️retirement/🔗️arc-custody/🔒️sealed): `prepare_birth(grant, actual)`, `admit(value, issuer, grant, actual)`, `try_duplicate(effect)`, birth `close_step(effect)`.
   - `publish_sealed(grant, value)` -> needs `actual:&mut Option<usize>` (take it from the caller's effect: `effect.actual()`), `birth.close_step(&mut effect...)`.
   - `ToolRunEntityVersion::admit/capture`, `ToolRunEntityProvenance::admit/capture`: `capture(&self, effect)` (grant = effect.grant()), `admit(map, revision, grant, actual)`.
   - Their callers (take_output chains, presentation update `capture(...)` lines 62 and 10, tick native, panes) change with them; not scripted because the caller set is open-ended.
3. Other crates the presentation depends on still use the old forms (progress clone, seed, step-ring insert, stream update) — separate ports.
