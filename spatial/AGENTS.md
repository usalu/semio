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

A view is a different perspective on a model.
A view performs transformations.
A view derives new target objects and doesnt show the source objects.
Inside a view, geometry cant be edited because it is not the source geometry.

## Structural

A view used for structural analysis.

### Transformation

Join/unions touching geometry with bond attributes.
e.g. two curves with bondable endpoints are joined into one curve.
e.g. two surfaces that touch are joined into one surface.
e.g. two solids with touching faces are boolean unioned.

## Energy

Used for energy calculations.

### Transformation

External surfaces are joined to form a closed shell. The volume is calculated.
For display the windows are removed from the closed shell and yield an open shell.
The window surface are added separately.

### Derived Objects

#### Base plate

The lowester external horizontal surface.

#### Roof

The highest external horizontal surface.

#### External Wall

A

####

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

## U-Value

A U-Value can be attatched to any surface.

```json
{ "u-value": "0.158" }
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

## G-Value

A G-Value can be attatched to any surface with the window attribute.

```json
{ "g-value": "0.6" }
```
