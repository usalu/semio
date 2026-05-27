# Core

Extendable spatial framework around persistent models, runtime execution, and rendering.

## Geometry

Directly represented as in [brepjs](https://andymai.github.io/brepjs) but additionally attributes can be added on every entity.

## Model

A model contains objects.

## Object

An object is an instance of a typology.

## Attribute

An attribute is attachable authored metadata to geometry.

## Property

A property is a derived attribute that is not authored.

# Extension

An extension is a data-only namespace packaged by `extension.json`. It groups declarative assets such as actions, interactions, views, attributes, and properties. Extensions do not ship executable code; the runtime interprets their data against predefined capabilities.

All concrete actions, interactions, views, attributes, and properties live as JSON extension assets under `spatial/assets/extension`.
Each extension has `extension.json` manifest.
Each view lives in a `<view-id>` folder with a `view.json` file.
Extension-level typologies are stored in `typology/<categoryFolders..>/<typology-id>.json`.
Extension-level Actions are stored in `action/<categoryFolders..>/<action-id>.json`. View-level actions are stored in `<view-id>/action/<categoryFolders..>/<action-id>.json`. Same for interactions, attributes and properties.

Everything below this section is part of an extension.

## Action

An action is a declarative headless operation document. It describes how the runtime reads, writes, derives, or transforms spatial data by invoking predefined capabilities and variables.

## Interaction

An interaction is a declarative static state machine for the renderer. It describes prompts, picks, transient state, previews, and transitions that usually resolve into one or more actions.

## Typology

A typology is a class of objects.
Every typology has one or many actions to construct an object of that typology.
Every typology has one or many interactions to construct an object of that typology.

## ModelDefinition

A model definition defines typologies, attribute kinds, property kinds.

## Transformation

A transformation derives a target model for a target model definition from a source mode for a source model definition.

## AttributeDefinition

An attribute is attachable authored metadata to geometry.
brepjs currently doesnt support custom metdata, hence you must maintain an id-based external Map.

## PropertyDefinition

A property is a derived attribute that is not authored.
