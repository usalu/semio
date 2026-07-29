# Summary

Core library containing all representation definitions, validation, serialization, and the Meta class for reflection-based metadata.

# Docs

## Compose.cs

Core library containing all representation definitions, validation, serialization, and the Meta class for reflection-based metadata.

## Compose.Grasshopper.cs

Grasshopper plugin providing components for constructing, deconstructing, and modifying compose representations.

### Architecture

The program uses a component hierarchy with base classes that provide default behavior:

- **`RepresentationComponent<TParam, TGoo, TRepresentation>`**: Base class for representation components with virtual methods for customization
- **`IdComponent`**, **`DiffComponent`**: Specialized base classes for Id and Diff representation types
- **`SerializeComponent`**, **`DeserializeComponent`**: Base classes for serialization components

### Component Structure

Each representation type has a set of classes:

- **`*Goo`**: Grasshopper wrapper for the representation type with cast methods
- **`*Param`**: Grasshopper parameter definition
- **`*Component`**: Main representation component for construct/deconstruct/modify
- **`Serialize*Component`**: JSON serialization component
- **`Deserialize*Component`**: JSON deserialization component

### Hardcoded Parameters

Components use virtual methods to define their inputs/outputs:

- `RegisterRepresentationInputParams(pManager)`: Define input parameters
- `RegisterRepresentationOutputParams(pManager)`: Define output parameters
- `GetRepresentationData(DA, representation)`: Read input data into representation
- `SetRepresentationData(DA, representation)`: Write representation data to outputs

Components can override these to hardcode their parameter structure, ensuring stable input/output definitions across schema changes.

# 💯Requirements
