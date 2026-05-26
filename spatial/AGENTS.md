# Action

A function to perform an action. Actions run headless.

## Interaction

An interaction is a descriptive static state-machine for describing interactions with the renderer.
A state can display information based on a set of predefined draw

# Model

A model contains objects. Objects can contain child objects.

```json
{ "objects": [{…}] }
```

# Typology

A typology is a class of objects.
Every typology has one or many action to create an object of that typology.
Every typology may have multiple interactions to derive the paramters for that canonical action.

## Wall

```json
{ "id":"wall","name":"Wall","allowedAttributes":["exposure",…],"derivedProperties":["volume",…],"actions": [{"name":"constructVerticalWall","args":{"height":2.7, "curve":…}},
    {"name":"constructWallFromBottomAndTop","args":{"bottomCurve":…, "topCurve":…}},
    {"name":"constructWallFromHorizontalPathAndProfile","args":{"pathCurve":…, "profileCurve":…}},
    {"name":"constructWallFromHorizontalPathAndProfiles","args":{"pathCurve":…, "profileCurves":[…]}}],
 "interactions":…}
```

## MushroomColumn

A mushroom column. One solid geometry.

```json
{ "actions": [ {"name":"constructMushroomColumn","args":{"solid":…}},
    {"name":"constructExtrudedMushroomColumn","args":{"columnProfile":…,"height":2.7, "heightIs":"total","slabSolid":…}},
    {"name":"constructRectangularMushroomColumnWithQuadraticSlab","args":{"rectangularColumnWidth":0.4, "rectangularColumnBreadth":0.6, "quadraticSlabWidth":2.3}},
    {"name":"constructRectangularMushroomColumnWithQuadraticSlab","args":{"height":2.7, "heightIs":"column","slabHeight":0.3, "rectangularColumnWidth":0.4, "rectangularColumnBreadth":0.6, "quadraticSlabWidth":2.3}},
    {"name":"constructFullyQuadraticMushroomColumn","args":{"height":2.7, "heightIs":"total","slabHeight":0.3, "quadraticColumnWidth":0.4, "quadraticSlabWidth":2.3}}],
 "interactions":…}
```

### View

#### Structural

A vertical line for the column and a planaer horizontal surface for the top part.
Additionally has lines for joists.

# Object

An object has geometry and a typology. Optionally it can provide geometry for different views.

```json
{"geometry":{"points":[{"id":}]}}
```

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

```json
{"id":"hull","name":"Hull","properties":["volume","heatedvolume",…]}
```

#### BasePlate

The lowester external horizontal surface.

#### Roof

The highest external horizontal surface.

#### ExternalWall

All joined touching external surfaces of the same material with the windows cut out.

#### Windows

All external windows surfaces.

## Structure

### Derived Objects

#### ReinforcedConcreteColumn

#### OneWayReinforcedConcreteSlab

#### ReinforcedConcreteInternalWall

#### ReinforcedConcreteExternalWall

## LineFEM

A view used for Finite-Element-Analysis with line elements.

```json
{ "id":"linefem","name":"Classic Structural","allows":["lines"],…}
```

### Derived Objects

#### LineElement

## SurfaceFEM

A view used for Finite-Element-Analysis with surface elements.

```json
{ "id":"surfacefem","name":"Classic Structural","allows":["surfaces"],…}
```

### Derived Objects

#### SurfaceElement

## SolidFEM

A view used for Finite-Element-Analysis with solid elements.

```json
{ "id":"structural","name":"Classic Structural","allows":["solids"],…}
```

### Derived Objects

#### SolidElement

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

# Property

A property is a derived attribute.

## Volume

The volume is derived from a solid.

```json
{ "id": "volume", "name": "Volume", "unit": "volume" }
```

## HeatedVolume

```json
{ "id": "heatedvolume", "name": "Heated Volume", "unit": "volume" }
```
