# Core

Extendable framework with geometric functionality, renderer, etc.

# Extension

Extensions are data-only. They dont contain any executable code.

Everything below this section is part of an extension.

## Action

An action is a headless operation exposed by an extension. Actions mutate or derive spatial data without defining renderer behavior. All calls to geometric operations happens over a predefined set of functions and special variables accessible.

## Interaction

An interaction is a renderer-facing state machine exposed by an extension. It gathers input, previews intent, and usually resolves into one or more actions.

## Model

A model is the root document for spatial data. It stores objects, their topology references, and the extension data attached to them.

## Topology

Topology is the persistent geometric core. It stores the connected spatial graph that objects and derived views reference.

## Object

An object is a node in the model that binds extension data to topology. Objects can form hierarchies and can contribute to derived views.

## View

A view is a computed perspective on a model. Views derive objects from source objects, preserve traceability back to their sources, and can attach their own extension data.

## Attribute

An attribute is extension data attached to topology or objects. Attributes describe authored metadata rather than derived results.

## Property

A property is extension data derived from geometry, topology, objects, or views. Properties describe computed results and should be reproducible from source data.

All concrete actions, interactions, views, attributes, and properties live as JSON extension assets under `spatial/assets/extension`.
