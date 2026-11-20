# Procedural mesh vertex control

The Generation 3D editor Inspection panel displays Mesh Source positions through the framework vector input. Each vertex is a single labelled group with X, Y and Z numeric fields. The localized parent label, per-axis blur commit, stable field keys and existing structured setWidgetInput paths remain in use. Vertex add, remove and reorder use their existing tree actions.

The audit also found shared WindowMeasure selectors in edit and view preview chrome, shared sun controls in both previews, and shared generation forms and generation tree controls. The mesh vertex row was reachable for an editable mesh source and used a nested tree of generic numeric rows before this change.

The neutral inspection fixture now records group semantics and each component path. A native inspection projection assertion checks the actual component tree against that fixture. The existing portable widget input suite passed 405 checks, including English and German named group and spinbutton checks with Testing Library, and exact mesh mutations compared to fast-json-patch and validated with Ajv. A source route check failed before the replacement and passed afterward. The native projection test was added and awaits verification when build storage is available.
