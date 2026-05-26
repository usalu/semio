# Action

An action is a headless operation exposed by an extension. Actions mutate or derive spatial data without defining renderer behavior.

# Interaction

An interaction is a renderer-facing state machine exposed by an extension. It gathers input, previews intent, and usually resolves into one or more actions.

# Model

A model is the root document for spatial data. It stores objects, their topology references, and the extension data attached to them.

```json
{ "objects": [{ "id": "object-1", "extensions": {} }] }
```

# Topology

Topology is the persistent geometric core. It stores the connected spatial graph that objects and derived views reference.

```json
{ "schema": "spatial.topology/v1", "points": [], "edges": [], "faces": [], "cells": [] }
```

# Object

An object is a node in the model that binds extension data to topology. Objects can form hierarchies and can contribute to derived views.

```json
{ "id": "object-1", "topology": { "cells": ["cell-1"] }, "extensions": {} }
```

# View

A view is a computed perspective on a model. Views derive objects from source objects, preserve traceability back to their sources, and can attach their own extension data.

```json
{ "id": "view-1", "objects": [{ "id": "derived-object-1", "sourceObjectIds": ["object-1"] }] }
```

# Attribute

An attribute is extension data attached to topology or objects. Attributes describe authored metadata rather than derived results.

```json
{ "material": "concrete" }
```

# Property

A property is extension data derived from geometry, topology, objects, or views. Properties describe computed results and should be reproducible from source data.

```json
{ "volume": 42.0 }
```

All concrete actions, interactions, views, attributes, and properties live as JSON extension assets under `spatial/assets/extension`.
