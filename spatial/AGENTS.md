# Core

Extendable spatial framework around persistent models, runtime execution, and rendering.

# Primitive

A primitive is hashed.

## Geometry

### Point

### Curve

#### Line

#### Circle

#### Ellipse

#### Parabola

#### Hyperbola

#### B-Spline

#### Bezier

### Surface

#### Plane

#### Cylinder

#### Cone

#### Sphere

#### Torus

#### B-Spline

#### Bezier

## Topology

### Vertex

### Edge

### Wire

### Face

### Shell

### Solid

### CompSolid

### Compound

## ModelSpace

A model space is a container for linked models.
Every geometry is hashed (every vertex)

## Model

A model contains objects.

### Object

An object is an instance of a typology along with primitives.

#### Attribute

An attribute is attachable authored metadata to primitives.

#### Property

A property is a derived metadata for objects that is not authored.

# ModelDefinition

A model definition is a declarative schema for the model shape of a model.
A model definition contains typologies, attribute definitions and property definitions.
A model definition do not ship executable code; the runtime interprets their data against predefined capabilities.

## Typology

A typology is a class of objects.
A typology has a set of allowed primitive kinds.
Every typology has one or many actions to construct an object of that typology.
Every typology has one or many interactions to construct an object of that typology.

### Action

An action is a declarative headless operation document for a typology. It describes how the runtime reads, writes, derives, or transforms spatial data by invoking predefined capabilities and variables.
Actions can call other actions.

### Interaction

An interaction is a declarative static state machine for the renderer for a typology. It describes prompts, picks, transient state, previews, and transitions that usually resolve into one or more actions.
Interactions can call other actions.
A state can call another interaction, then the current interaction is paused until the called interaction finishes.

## AttributeDefinition

A attribute definition is a schema for the attribute shape of a attribute.

## PropertyDefinition

A property definition is a schema for the property shape of a property.

## Transformation

A transformation derives a target model for a target model definition from a source model for a source model definition.
