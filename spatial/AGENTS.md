---
technology: spatial
---

# Core

Extendable spatial framework around persistent models, runtime execution, and rendering.

## Scalar

## Primitive

A primitive is either geometry or topology.
Primitives consist of scalars, child primitive, referenced primitives and attributes.
Every primitive has an always up-to-date hash.
The hash is derived from scalars, child primitive, referenced primitives.
The hash does not not include attributes.

### Geometry

#### Point

##### Curve

###### Line

###### Circle

###### Ellipse

###### Parabola

###### Hyperbola

###### B-Spline

###### Bezier

##### Surface

##### Plane

##### Cylinder

##### Cone

##### Sphere

##### Torus

##### B-Spline

##### Bezier

### Topology

### Vertex

### Edge

### Wire

### Face

### Shell

### Solid

### CompSolid

### Compound

## Attribute

An attribute is attachable authored metadata to primitives.

## ModelSpace

A model space is a container for logically the same model but different model definitions.
In a model space, editing models is as much linked as possible.
When a primitive is edited inside a model space, then all primitives withe same hash, are also edited.

e.g. Assume there is a shape model definition where a Box object with a box primitive geometry is created with createBox interactions that yields a constructBoxFrom3Points action. Then the user creates a new building energy model from the shape model with the general `spatial.shape_to_aec.building.energy`. That transformation creates 4 Wall objects with attribute exposure external attached to the primitive, one Roof and BasePlate (both same principle as Wall). When a vertical edge of the box is moved, the vertical edge of the walls of the energy model should also move. If the user tries to change something that is affecting primitives that cant be linked back, give the users a warning that the models are no longer linked.

e.g.

```json
{ "models": "spatial.shape", "objects":[{…}] }
```

## Model

A model is an instance of a model definition.
A model contains objects.

e.g.

```json
{ "modelDefinition": "spatial.shape", "objects":[{…}] }
```

### Object

An object is an instance of a typology.
An object contains primitives and properties

e.g.

```json
{ "typology": "aec.building.energy.Roof", "primitives":{"surface":{…}}, "properties":[{"uValue":0.25}] }
```

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

## Action

An action is a declarative headless operation document for a typology. It describes how the runtime reads, writes, derives, or transforms spatial data by invoking predefined capabilities and variables.
Actions can call other actions.
When an action only affects one typologie it is part of that typology.
When an action affects more than one typologe then it is part of the model defintion.

## Interaction

An interaction is a declarative static state machine for the renderer for a typology. It describes prompts, picks, transient state, previews, and transitions that usually resolve into one or more actions.
Interactions can call other actions.
A state can call another interaction, then the current interaction is paused until the called interaction finishes.
When an interaction only affects one typologie it is part of that typology.
When an interaction affects more than one typologe then it is part of the model defintion.

## AttributeDefinition

A attribute definition is a schema for the attribute shape of a attribute.

## PropertyDefinition

A property definition is a schema for the property shape of a property.

## Transformation

A transformation derives a target model for a target model definition from a source model for a source model definition.
