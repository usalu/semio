# Action

A function to perform an action. Actions run headless.

## Interaction

An interaction is a descriptive static state-machine for describing interactions with the renderer.
A state can display information based on a set of predefined draw

# Model

A model contains objects.

# Object

An object has geometry. Optionally it can provide geometry for different views.

# View

A view is a different perspective on a model.
A view performs transformations.
Inside a view, geometry cant be edited because it is not the source geometry.

## Structural Analysis View

Used for structural analysis.

### Transformation

Join/unions touching geometry with bond attributes.
e.g. two curves with bondable endpoints are joined into one curve.
e.g. two surfaces that touch are joined into one surface.
e.g. two solids with touching faces are boolean unioned.

## Energy Modelling View

Used for energy calculations.

### Transformation

External surfaces are joined to form a closed shell. The volume is calculated.
For display the windows are removed from the closed shell and yield an open shell.
The window surface are added separately.

# Attribute

An attribute is attachable metadata to geometry.

## Bondable

Two bondable contacts means they are unified geometrically.

## Concrete

The concrete class can be attatched to any curve, surface or solid.

## U-Value

A U-Value can be attatched to any surface.

## External

Any surface can be marked as external.

## Window

A surface marked as a window will be trimmed into external walls.

## G-Value

A G-Value can be attatched to any surface with the window attribute.

## Slab

A horizontal surface can be marked as slab.
