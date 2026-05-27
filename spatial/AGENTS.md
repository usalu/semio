# Core

Extendable spatial framework around persistent models, runtime execution, and rendering.

## Model

A model contains objects.

## Object

An object is an instance of a typology.

## Attribute

An attribute is attachable authored metadata to objects.

## Property

A property is a derived attribute that is not authored.

# ModelDefinition

An model definition is a data-only namespace packaged by `modelDefinition.json`. It groups declarative assets such as actions, interactions, transformations, attribute definitions, and property definitions. Extensions do not ship executable code; the runtime interprets their data against predefined capabilities.

All assets are under `spatial/assets/extension`.
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

A model definition is a schema for the model shape of a model.
A model definition contains typologies, attribute definitions and property definitions.

## AttributeDefinition

A attribute definition is a schema for the attribute shape of a attribute.

## PropertyDefinition

A property definition is a schema for the property shape of a property.

## Transformation

A transformation derives a target model for a target model definition from a source model for a source model definition.
