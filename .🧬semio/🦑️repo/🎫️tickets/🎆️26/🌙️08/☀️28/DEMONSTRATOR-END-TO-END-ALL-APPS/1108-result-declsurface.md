# Lane declsurface result (semio-plugin-decl-surface, 48 errors in W-flowdoc3-3)

## State (verified: W-declsurface-3 wasm32-wasip2 via hub rc0, W-declsurface-4 native rc0)
- natural-file (`declaration/📥️natural-file`): `NaturalFileImportJob::close_step` now runs `close_original_effect_step`; `close_owned_turn(effect)` returns `OriginalRetirementPoll` (outcomes `close_step(effect)`, `close_factory_ticket(slot,body,effect)`, `admit_owned_retirement(&mut Option,effect)` via local `admit`, controlled decoder close charged with `record_retirement_work`).
- original (`surface/🌱️original`): `RetireOwned` for EditorApp/ViewerApp via `RetirementCursor = <String as RetireOwned>::RetirementCursor` + `retirement_seed`; controller close through `close_original_retained_effect_step` + `record_retirement_work`.
- editor trait: `DraftMutation` requires `::replication::OpBinaryReceiving` (mirrors decl-app).

## Debts
- `NaturalFileDecodeCursor::close_step` (decl-app) and `OriginalArtifactAppBirth::admit_controller/controller_close_step` stay grant/StepContext-shaped; decl-surface charges them through the effect (`effect.grant()` + recorded work).
Rerun: `chk.sh W-declsurface-<n> 🌎️hub/Cargo.toml wasm -p semio-plugin-decl-surface`

STATUS: FIXED-verified W-declsurface-3 (wasm), W-declsurface-4 (native)
