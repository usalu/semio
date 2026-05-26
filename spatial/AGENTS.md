# Core

Extendable spatial framework around persistent topology, runtime execution, and rendering.

## Geometry

Directly represented as in [brepjs](https://andymai.github.io/brepjs).

# Extension

An extension is a data-only namespace packaged by `extension.json`. It groups declarative assets such as actions, interactions, views, attributes, and properties. Extensions do not ship executable code; the runtime interprets their data against predefined capabilities.

All concrete actions, interactions, views, attributes, and properties live as JSON extension assets under `spatial/assets/extension`.
Each extension has `extension.json` manifest.
Each view lives in a `<view-id>` folder with a `view.json` file.
Extension-level Actions are stored in `action/<categoryFolders..>/<action-id>.json`. View-level actions are stored in `<view-id>/action/<categoryFolders..>/<action-id>.json`. Same for interactions, attributes and properties.

Everything below this section is part of an extension.

## Action

An action is a declarative headless operation document. It describes how the runtime reads, writes, derives, or transforms spatial data by invoking predefined capabilities and variables.

## Interaction

An interaction is a declarative static state machine for the renderer. It describes prompts, picks, transient state, previews, and transitions that usually resolve into one or more actions.

## Model

A model contains objects.

## Typology

A typology is a class of objects.
Every typology has one or many actions to construct an object of that typology.
Every typology has one or many interactions to construct an object of that typology.

## Object

An object is an instance of a typology.

## View

A view is a different computed perspective on a model.
A view derives new objects and doesnt show the source objects anymore.
It keeps the link of all involved source objects to the target objects.
Inside a view, geometry cant be edited because it is not the source geometry.

## Attribute

An attribute is attachable authored metadata to geometry.
brepjs currently doesnt support custom metdata, hence you must maintain an id-based external Map.

## Property

A property is a derived attribute that is not authored.
