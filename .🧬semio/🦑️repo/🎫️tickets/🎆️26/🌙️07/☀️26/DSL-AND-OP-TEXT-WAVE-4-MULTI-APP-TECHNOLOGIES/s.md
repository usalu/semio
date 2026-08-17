# s (Wave 4) — scratch notes

## Real types found
- `s/rs/lib.rs` (crate `s_studio`): `SStudioProjection` (programs, active_program_id,
  active_alternative_id, app_instances: Vec<SAppInstance>, media_graph: SMediaGraph),
  `StudioOperation` (7 variants: SetActiveProgram, SetActiveAlternative, SpawnAppInstance,
  RemoveAppInstance, ConnectMediaPorts, DisconnectMediaEdge, MoveMediaNode), `StudioDiff`
  (OperationDiff impl already existed at ~line 229, Operation impl at ~line 251).
- `s/plugin/rs/lib.rs` (crate `s-plugin`, inside `pub mod app_home`): `SHomeDocument`
  (schema, catalog_generation: u64), `SHomeOperation` (NoOperation, SetCatalogGeneration{value}),
  its own Diff (`type Diff = Self`). No JSON fixture — always built programmatically
  (`SHomeDocument { schema: "s.home".into(), catalog_generation: 0 }`).

## Implemented
- `s/rs/lib.rs`: added `//#region 🔖️Dsl` (`mod studio_dsl` hand-rolled lexer/printer +
  `impl vcs::DocumentDsl for SStudioProjection`, extension `sstudio`) and
  `//#region 🔖️OpText` (`impl vcs::OpText for StudioOperation`), plus round-trip tests
  in the existing `//#region 🧪️Tests`.
- `s/plugin/rs/lib.rs`: added `//#region 🔖️Dsl` (`mod home_dsl` + `impl vcs::DocumentDsl for
  SHomeDocument`, extension `shome`) and `//#region 🔖️OpText` (`impl vcs::OpText for
  SHomeOperation`) inside `app_home`, plus round-trip tests in that module's
  `//#region 🧪️Tests`.

## Blocker found — demo.s.json / DEMO_STUDIO_JSON is NOT typed to SStudioProjection

Verified by reading, not assumed:
- `s-plugin`'s `Cargo.toml` does not depend on the `s_studio` crate at all — `SStudioProjection`/
  `StudioOperation` (s/rs) are never referenced anywhere in `s/plugin/rs/lib.rs`.
- `parse_demo_studio_document()` (s/plugin/rs/lib.rs ~line 33) deserializes `DEMO_STUDIO_JSON`
  into `semio_framework_os::OsProjection` (framework/product/os/core/rs/lib.rs), not
  `s_studio::SStudioProjection`. `OsProjection` has extra fields `SStudioProjection` lacks
  (parameters, parameter_bindings; nodes carry width/height; ports carry direction; edges carry
  a `contract: MediaContract`). The two "studio projection" shapes have diverged — `s_studio`'s
  is a standalone, not-yet-wired-in core crate (matches the file's own comment on
  `StudioStore::attach_backbone`: native wiring is "s's own DocumentApp migration (WS-F's last
  wave)", i.e. not this wave).
- `S_STUDIO_EXAMPLES` (s/plugin/rs/lib.rs ~line 827, used at ~3207/~4051) feeds
  `DEMO_STUDIO_JSON` verbatim into `App::example(id, label, json)` — a framework/plugin manifest
  API that (as far as this crate is concerned) expects a JSON string blob.

Consequence: implementing `vcs::DocumentDsl for OsProjection` to retarget `DEMO_STUDIO_JSON` is
not possible from `s/` — `OsProjection` and `vcs::DocumentDsl` are both foreign to `s-plugin`
(orphan rule), and the only place that could legally add the impl is
`framework/product/os/core/rs/lib.rs`, explicitly out of scope for this ticket ("do not touch
framework/*, report blockers instead"). Hand-rolling a duplicate OsProjection parser inside
`s-plugin` instead would leak framework's technology/data model into `s`'s codebase outside of
testing (forbidden — "You MUST NOT leak fixtures or other technologies into the code outside of
testing") and duplicate logic that belongs in framework (breaks single-source-of-truth).

The existing `ensure_studio_fixtures_registered` comment already documents this exact class of
problem for draw/writer's fixtures ("this registry is still JSON-shaped ... tracked for the
Wave 6 lock step"). `demo.s.json` / `DEMO_STUDIO_JSON` falls in the same bucket — it's framework
(`OsProjection`) shaped data, migrating it to a handcrafted DSL is `framework/product/os`'s Wave
6 job, not `s`'s Wave 4 job. Left `s/example/demo.s.json` and `DEMO_STUDIO_JSON` untouched;
reported as a blocker in the final report instead of forcing a broken or leaky wiring.
