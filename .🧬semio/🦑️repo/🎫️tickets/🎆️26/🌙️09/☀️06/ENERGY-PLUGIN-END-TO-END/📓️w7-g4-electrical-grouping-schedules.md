# ⚡️ G4 — electrical & renewables (700–799), grouping (800–899), schedules (900–999)

Lane G4 of `26/09/06/ENERGY-PLUGIN-END-TO-END`. 84 mutation kinds, 168 committed fixture vectors, all
declared as rows inside `# region 🔖️G4` of `🐍️generate-mutation-leaves.py` and emitted from there —
no aggregate file, crate mount, oracle catalog entry, feature table or Python oracle row was
hand-edited.

## 1. Pattern vocabulary this lane added (FOR W-D0, the generator custodian)

Six reusable functions, all in `# region 🔖️G4` before the spec rows. They are written to be
group-neutral; the `g4_` prefix is only a lane marker, exactly as `g2_` is.

| Function | What it generalizes | Why the shared helpers could not express it |
|---|---|---|
| `g4_ref_guard_rs` / `_py` / `g4_ref_refused_rs` | referential-integrity guards against **any** collection, for a single id field (`"one"`) or a list of ids (`"many"`), with `"schedules"` meaning the five-group union | `g2_ref_guard_rs` hard-codes three targets — zones, schedules, materials — and its list branch is hard-coded to `materials`. G4 references `pv_systems`, `battery_storage`, `ideal_loads`, `spaces` and `zones`. |
| `g4_create` | the shared `create` shape (stated `index`, duplicate-id refusal, delete-partner inverse) behind the general ref guard above | same reason |
| `g4_membership` (`mode="add"` / `"remove"`) | one set-like `Vec<EntityId>` edge between an owner and a member collection | new shape. **`add` carries a FINAL-state `index`**: the list is a set semantically (the vocabulary gives it no `reorder`) but a JSON array positionally, so without the index `remove`'s inverse would put the member back at the end and the committed before-snapshot would not compare equal. |
| `g4_scalar` / `g4_count` | numeric guards the shared table lacks — `tilt` (0…90°), `azimuth` (0…360°), `hot-water` (0…100 °C), `finite`, and a `u32` population count that must not be zero | `G2_RS_TEST` carries only positive / non-negative / fraction / unit-positive |
| `g4_entity_reference` | an `EntityId` reference field pointing into an arbitrary collection | `g2_reference` speaks only zones and schedules |
| `g4_enum` | a closed-vocabulary (`dsl::DslScalar`) field, where the only possible refusal is an absent target | — |
| `g4_schedule_create` / `g4_schedule_delete` / `g4_schedule_element` | the same three shapes for `model.schedules.<group>`: `ScheduleId`-keyed, nested one level deeper than a top-level `Model` collection, id unique across all five groups, delete refused while referenced | every shared helper addresses `base.model.<collection>` keyed by `EntityId` |
| `g4_titles` / `G4_PAST` | past-tense record names for `insert` and `replace` | `G2_PAST` has neither; the schedule group is the first to need them |

**Cross-lane API notice.** G4 CALLS G2's `g2_element_kind`, `g2_create`, `g2_delete`, `g2_rename`,
`g2_scalar` and `g2_reference` rather than copying them. Renaming those now breaks G4's rows.
Recommendation for W-D0: promote both prefixes into one unprefixed `# region 🔖️Patterns` block once
the lanes are done, since the `g2_`/`g4_` split is historical, not semantic.

## 2. Decisions that are not mechanical

1. **`703`/`704` `add-/remove-electrical-load-center-generator` are deliberately NOT authored.**
   `ElectricalLoadCenter::generator_ids` references a collection `Model` does not have (vocabulary
   §5.4). `⚡️electrical::Generator` exists in the engine but nothing reaches it from the document.
   The ledger allowed "author them with an explicit refusal", but a refusal-only kind cannot carry
   the `✅️` happy vector the generator's `audit()` requires, and adding `generators: Vec<…>` to
   `Model` mid-ticket would invalidate every committed `🔋️model.json` and every oracle-emitted
   document (W-B/W-C/W-F all consume `Model`'s current shape). `generator_ids` IS carried verbatim by
   `create-electrical-load-center` — unchecked, and said so in the leaf docstring — so create/delete
   round-trips a load centre exactly. Numbers stay reserved; author them when a backing collection
   lands. Recorded in `📓️mutation-tag-ledger.md` §7.
2. **`748 change-fault-target-equipment` got a real referential check after all.** The ledger said it
   could not have one until `FaultDefinition::target_equipment_id` became a tagged reference. The
   engine already supplies the referent: `🌰️kernel`'s `SystemSubstepStage::Fault` matches
   `fault.target_equipment_id == ideal.id`, and `🧠️precompute`'s `PrecomputeStage::Faults` keys its
   severity map by the same id. So `ideal_loads` IS the collection that field addresses, and `746`
   and `748` both check it there. The field's *type* still carries no discriminator — that follow-up
   stands — but the check is no longer missing.
3. **`913` renamed `add-annual-schedule-rule` → `insert-annual-schedule-rule`** (ledger §7).
   `📓️taxonomy.md` reserves `add` for set-like membership and `insert` for an ordered,
   index-addressed list. `AnnualSchedule::rules` is measurably ordered: `ScheduleSet::annual_value`
   returns the FIRST rule whose range contains the day. That is also the justification for keeping
   `915 reorder-annual-schedule-rules`, which `derivation-rules.md` rule 2 would otherwise drop.
4. **`delete-annual-schedule` has a cascade inverse.** `create-annual-schedule` states only the two
   fallback profiles; rules and holidays are collections of their own. So the undo is
   `create-annual-schedule` + one `insert-annual-schedule-rule` per rule in order + one
   `add-annual-schedule-holiday` per holiday in order. This is the only multi-step inverse in the
   lane and it is what `taxonomy.md`'s "delete captures cascade" means here.
5. **`AnnualSchedule::holiday_dates` keeps its `Vec<(u16,u8,u8)>` shape.** A tuple has no
   `dsl::DslField`, but the payload never carries the tuple: `add`/`remove-annual-schedule-holiday`
   take flat `year`/`month`/`day`. No model-struct change was needed, unlike
   `replace-airflow-network`'s parallel-list workaround.
6. **`[f64; 24]` and `[ScheduleId; 7]` are carried as `Vec` in payloads, length-validated**, following
   `update-ground-temperature`'s precedent: a wrong length is an addressable `mutation.invariant`
   refusal rather than a decode failure.
7. **Schedule ids are one namespace across all five groups**, because `ScheduleSet::lookup` searches
   constants → annual → weekly → daily → time-series in turn. Every `create-*-schedule` refuses an id
   any group already holds; every `delete-*-schedule` refuses while any of the fourteen
   `ScheduleId`-typed slots in the document still resolves it (schedule-to-schedule references —
   weekly days, annual rules, annual default and holiday profiles — included).

## 3. Engine-model changes this lane made

Both are additive derives, needed because the payload field types must implement `dsl::DslField`:

- `crate::model::FaultType` → `+ dsl::DslScalar` (`⚙️engine/🔋️model/🦀️.rs`).
- `crate::schedule::ScheduleInterpolation` → `+ dsl::DslScalar` (`⚙️engine/🗓️schedule/🦀️.rs`).

Plus three entries per generator type map (`JSON_TYPES`/`PROTO_TYPES`/`TS_TYPES`/`GRAPHQL_TYPES`):
`crate::model::FaultType`, `crate::schedule::ScheduleInterpolation`,
`Vec<crate::model::ScheduleId>`.

## 4. Emoji allocation

The emoji namespace is `pathEmojiPolicy.siblingNamespace: files-and-directories` inside
`🧬️mutations/`, and four lanes were appending to it at once. Four rounds of collisions were resolved
before `audit()` went green; the final assignment is written into `📓️mutation-tag-ledger.md` §5.
Where a lane kept taking the semantically obvious symbol, G4 moved to a family no other lane will
reach for — clock faces for the schedule group, which is at least thematically apt.

## 5. Status
