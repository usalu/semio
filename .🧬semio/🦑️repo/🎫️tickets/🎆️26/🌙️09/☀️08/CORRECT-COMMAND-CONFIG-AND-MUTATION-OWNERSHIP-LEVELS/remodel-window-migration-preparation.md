# Remodel Window Migration Preparation

## Ownership decision

The existing `RemodelingConfig` is an app-level union of three unrelated concrete-window states. It must be replaced by three exact owners:

- `remodeling-main`: `camera` and `layers` in `RemodelingModelWindowConfig`.
- `remodeling-frames`: `frame_cursor` in `RemodelingFramesWindowConfig`.
- `remodeling-report`: `report_table` in `RemodelingReportWindowConfig`.

The app becomes `NoConfig` and `NoPresence`. The current `RemodelingPresence` duplicates the same camera, frame cursor, and report selection and has no independent producer or consumer, so its schema and mutation tree are retired.

## Existing producers and consumers

- `setCamera` and `setLayerVisibility` are view commands and currently publish app-config mutations. Their exact destination is the concrete `remodeling-main` instance that invoked them.
- `setFrameCursor` is a view command and belongs to the invoking `remodeling-frames` instance.
- `setReportTable` is a view command and belongs to the invoking `remodeling-report` instance.
- The retained reducer currently receives only the app config and discards `context.window_config`; it needs bounded command work that decodes the concrete window snapshot and emits an addressed `WindowConfig` mutation.
- The direct `ArtifactEditor::handle` path must perform the same routing from the concrete `ViewModel` identity.
- Model rendering, Model window measures, Frames rendering, and Report rendering currently consume the app config. Each must decode its exact window config from `ConfigView.window`.

## Schema and verification plan

Each owner requires Rust, JSON Schema, TypeScript, GraphQL, and Proto facets under its window taxonomy. The native proof must register two instances of each same window kind, dispatch the actual retained commands, demonstrate isolated render/chrome output, persist both exact config packs, reopen them under new app instances, and reject stale or wrong-kind addresses. A neutral JSON vector is validated both by the repository implementation and by Ajv plus `fast-json-patch`.

The existing app-level config and presence schema facets are deleted after all consumers are retargeted. No parent document facet changes are part of this migration.
