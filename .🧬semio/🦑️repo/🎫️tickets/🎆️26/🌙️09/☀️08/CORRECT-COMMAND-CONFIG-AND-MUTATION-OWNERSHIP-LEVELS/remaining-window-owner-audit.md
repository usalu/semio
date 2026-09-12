# Remaining Window Owner Audit

## Home Finding Superseded — 2026-09-12

The Home section below is historical. The refreshed source trace in `home-host-read-model-owner-refresh.md` establishes that `active_panel_tab` belongs to the host session's panel carriage, not an exact Home window. Root verified the browser's host-controller interception and `session.viewState.panelJson` update. Do not implement the older Home WindowConfig recommendation. `DirectoryHomeOwnerV1` orchestrates actions/ACKs but does not retain the directory read model; the OS projection and read-only guest boundary still require implementation. Native panel dispatch/persistence is receiving a separate read-only follow-up before removal of the Home mirror.

## Scope and method

This is a read-only follow-up over the currently mounted Forms, Shooting,
Architect Program, and Home editor routes. It traces a field from its concrete
config declaration through a command producer and a rendering consumer. A
field with no mounted producer or consumer is excluded from the execution
slice. No source was changed and no native build was run.

An exact window owner is an instance identity supplied by the host. The fixed
window kind strings in the sources below are insufficient: two instances of
the same kind must retain independent local configuration and transient state.

## Verified misplaced live state

### Forms: Try state is retained app-wide

`FormsConfig` persists both `current_step_index` and `try_values` at
`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:414-422`.
The sole Try window renders both at
`🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs:182-205`, while navigation
writes the shared config at `🎮️commands/▶️next-step/🦀️.rs:14-23`.

| Current field | Concrete consumer | Correct owner | Required boundary |
| --- | --- | --- | --- |
| `current_step_index` | `forms-try` wizard render, `▶️try/🦀️.rs:188-190` | persisted local configuration of the exact Try window | Move the read and next/previous/reset writers to the concrete Try window. |
| `try_values` | the same render uses it at `▶️try/🦀️.rs:190-201` | transient state of the exact Try window | Do not serialize unfinished answers as application preference or VCS config. Clear it when that window closes or is replaced. |

The batch continuations are also app/document scoped, not window scoped:
`🎮️commands/🗃️set-try-values/🦀️.rs:321-324` keys the active batch by app
instance and parent document, and `:333-341` resumes against the shared
`try_values` root. The single-value route does the same at
`🎮️commands/🎯️set-try-value/🦀️.rs:1314-1318`. Include the concrete window
identity in the transient/operation lease; merely moving the final field while
leaving these keys unchanged would preserve cross-window cancellation and
replacement races.

`contributions_json` is not a window migration candidate. It is produced by
the host-facing `set-contributions` route
(`🎮️commands/🧩️set-contributions/🦀️.rs:10-16`) and feeds the catalogue and
extension render paths. It needs a host/context projection rather than either
document or exact-window storage.

The Forms presence schema is deliberately not mounted:
`FormsPlayApp` selects `NoPresence` at `✏️editor/🦀️.rs:716-726`. Its
empty `👥️presence` facet is therefore excluded.

### Shooting: scene and inspector state share one application record

`ShootingConfig` retains all fields at
`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:26-47`.
The Scene window draws from that record:

- camera and centre/refit signal: `🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️.rs:209-222`;
- save-camera label: `:68-81`;
- camera load/save/config writers: `🎮️commands/🎥️camera/🦀️.rs:46-55,71-108`;
- centre/refit writers: `🎮️commands/🗂️selection/🦀️.rs:76-83`.

| Current field | Concrete consumer | Correct owner |
| --- | --- | --- |
| `camera`, `center_model` | Scene viewport | persisted local configuration of the exact Scene window |
| `fit_revision`, `camera_draft_label` | Scene refit event and save label | transient state of the exact Scene window |
| `selected_shot_ids` | Inspection panel selects the inspected shot at `📌️panels/🔍️inspection/🦀️.rs:56-69` | transient state of that exact inspection surface, or typed framework interaction when the panel gains one |

The Icon window also reads the shared camera at
`🎭️modes/✏️edit/🪟️windows/🖼️icon/🦀️.rs:68-71`. It must either use its own
exact-window camera or a selected saved-shot camera from the document; it must
not sample another Scene instance's working camera.

`default_shot_format`, `default_shot_shape`, and
`default_asset_format` have no mounted non-config producer or consumer:
the direct source search found only their declaration/default and generated
config surfaces. They are residual configuration, so this audit does not
prescribe a migration for them.

`ShootingPresence` is selected by the app
(`✏️editor/🦀️.rs:383-393`) but has no non-schema `Emit::presence` producer
or render consumer. Its overlapping shot/camera fields at
`👥️presence/🦀️.rs:13-21` are not an alternate owner to migrate into.

### Architect Program: four window states and one duplicate report payload

`ArchitectConfig` combines all candidate fields at
`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:24-42`.
The confirmed concrete routes are:

| Current fields | Producer and consumer | Correct owner |
| --- | --- | --- |
| `active_register` | writer `🎮️commands/📋️register/🦀️.rs:17-20`; Register window `🎭️modes/✏️edit/🪟️windows/📋️register/🦀️.rs:61-64` | persisted local configuration of the exact Register window |
| `adjacency_kind_filter` | writer `🎮️commands/↔️adjacency/🦀️.rs:94-97`; Adjacency window `🪟️windows/↔️adjacency/🦀️.rs:45-70` | persisted local configuration of the exact Adjacency window |
| `graph_camera_x/y/zoom` | writer `🎮️commands/🕸️graph/🦀️.rs:71-79`; Graph window `🪟️windows/🕸️graph/🦀️.rs:96-99` | persisted local configuration of the exact Graph window |
| `active_report_json` | writer also creates a document `ReportRecord`, `🎮️commands/🔬️analysis/🦀️.rs:69-77`; Report window reads the JSON, `🪟️windows/📓️report/🦀️.rs:37-40` | exact Report-window selection of the authored document record; never a second report payload |

`search_query`, `search_history_json`, `last_result_json`, and
`last_analysis_json` have writers, but no mounted render consumer in the
current tree. The config itself calls the last analysis value write-only at
`🎚️config/🦀️.rs:35-38`; no window move is justified until a result surface
and lifecycle are designed. If retained search is desired, it needs an
operation/result projection, not a whole-config snapshot.

The app mounts `ArchitectPresence` at `✏️editor/🦀️.rs:1031-1041`, yet no
editor command emits presence and no render reads it. The duplicated
register/filter/camera declaration at `👥️presence/🦀️.rs:18-29` is dormant
and excluded from the migration.

### Home: one window preference is mixed with OS session authority

`HomeConfig` stores tab, authenticated directory projection, receipt
authority, and signed-in identity together at
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:122-140`.
The evidence separates them:

| Current fields | Producer / consumer | Correct owner |
| --- | --- | --- |
| `active_panel_tab` | config writer `🎮️commands/⚙️set-active-panel-tab/🦀️.rs:15-17`; Home main is the only window kind at `🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:30-41` | persisted local configuration of the exact Home main window |
| `directory_json`, `directory_session_binding_sha256`, `directory_authorization_generation`, `directory_receipt_sha256` | authenticated page folds and replaces them at `🎮️commands/📬️apply-directory-event-page/🦀️.rs:18-35`; integrity/rebootstrap logic is `🎚️config/🦀️.rs:161-190` | OS/session directory read-model projection |
| `client_id`, `client_name` | shell identity bootstrap writes them at `🎮️commands/🪪️set-client/🦀️.rs:1-22`; studio creation consumes them at `🎮️commands/🏗️create-studio/🦀️.rs:37-65` | OS client identity |

The directory and identity move cannot be completed by creating a second
plugin-local record. They must be projected from the existing OS
directory/identity owners, with the Home window consuming that projection.
Home's empty presence type has no field-level work.

## Ranked execution slices

1. **Forms Try window — next bounded slice.** Move `current_step_index` to
   exact-window config and `try_values` plus its continuation/session keys to
   exact-window transient/operation state. Preserve `contributions_json` as
   host projection. The acceptance law opens two Try windows on one form,
   advances and stages a value in one, closes it, and proves the other
   window's step/answers and document bytes are unchanged.
2. **Shooting Scene and inspection.** Split Scene camera/preference from
   Scene transient refit/label and inspection transient shot selection. Resolve
   the Icon camera explicitly. Test two Scene and two inspection instances,
   including save-camera capturing only the caller's camera.
3. **Architect four-window decomposition.** Establish exact Register,
   Adjacency, Graph, and Report owners; replace report JSON with a selected
   `ReportRecord` identity. Defer the write-only search/analysis cache until
   it has an actual result owner.
4. **Home host-boundary extraction.** Move directory receipt/frontier and
   identity to the existing OS services, then retain only Home-main tab
   configuration under its concrete window. This requires host owner work and
   should not be attempted as a plugin-only config edit.

Every slice needs a language-agnostic two-concrete-window fixture validated by
the independent TypeScript/Ajv route and a native lifecycle test for reload,
window close/reset, and unchanged document bytes.
