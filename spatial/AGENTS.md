# Action

A function to perform an action. Actions run headless.

## Interaction

An interaction is a descriptive static state-machine for describing interactions with the renderer.
A state can display information based on a set of predefined draw

# Model

A model contains objects. Objects can contain child objects.

# Typology

# Wall

# Mushroom

A mushroom column. One solid geometry.

## View

### Structural

A vertical line for the column and a planaer horizontal surface for the top part.
Additionally has lines for joists.

# Object

An object has geometry and a typology. Optionally it can provide geometry for different views.

# View

A view is a different computed perspective on a model.
A view derives new objects and doesnt show the source objects anymore.
It keeps the link of all involved source objects to the target objects.
Inside a view, geometry cant be edited because it is not the source geometry.

## Energy

Used for energy calculations.

### Derived Objects

#### Hull

All external surfaces joined to a closed shell.

#### BasePlate

The lowester external horizontal surface.

#### Roof

The highest external horizontal surface.

#### ExternalWall

All joined touching external surfaces of the same material with the windows cut out.

#### Windows

All external windows surfaces.

## Structural

A view used for structural analysis.

### Transformation

Join/unions touching geometry with bond attributes.
e.g. two curves with bondable endpoints are joined into one curve.
e.g. two surfaces that touch are joined into one surface.
e.g. two solids with touching faces are boolean unioned.

#### Columns

#### OneWaySlab

# Attribute

An attribute is attachable metadata to geometry.

## Bondable

Two bondable contacts means they are unified geometrically.

```json
{ "bondable": true }
```

## Material

A material can be attatched to any curve, surface or solid.

```json
{"material":"concrete"}
{"material":{"concrete":"C30/37"}}
```

## UValue

A U-Value can be attatched to any surface.

```json
{ "uValue": "0.158" }
```

## Exposure

Any surface can be marked with an exposure.

```json
{"expsoure":"internal"}
{"expsoure":"external"}
```

## Opening

A surface marked as a opening will be trimmed into walls.

```json
{"opening":"window"}
{"opening":"door"}
{"opening":"passage"}
{"opening":"view"}
```

## GValue

A G-Value can be attatched to any surface with the window attribute.

```json
{ "gValue": "0.6" }
```
