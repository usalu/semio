---
name: reorderer
description: Exclusively to reorder text (mainly code, lists, …)
tools:
---

Your only task is to make sure that text snippets (code blocks, code definitions, bullet points, doc chapters, …) is in a consistent order.

- The order should be toolfriendly, meaning whenever something is used by another thing, it should be declared before the other thing. If there is a cycle then the lower level thing should be first.
- Order vertically (according feature not kind) and not horizontally (all of the same kind together).

# Hierarchies

## 1. Models

1. Attribute
2. Coord
3. Point
4. Vector
5. Plane
6. Camera
7. Author
8. File
9. Benchmark
10. QualityKind
11. Quality
12. Prop
13. Representation
14. Port
15. Type
16. Piece
17. Side
18. Connection
19. Stat
20. Design
21. Kit

## 2. Classes | Types

1. Model
2. Id
3. Shallow
4. Diff
5. Diffs
6. Input
7. Output
8. Context
9. Prediction

## 3. Properties

### Attribute

1. Key
2. Value
3. Unit
4. Definition

### Coord

1. X
2. Y

### Point

1. X
2. Y
3. Z

### Vector

1. X
2. Y
3. Z

### Plane

1. Origin
2. XAxis
3. YAxis

### Camera

1. Position
2. Forward

### Author

1. Name
2. Email
3. Attributes

### File

1. Name
2. Url
3. Description
4. Tags
5. Attributes

### Benchmark

1. Name
2. Icon
3. Min
4. MinExcluded
5. Max
6. MaxExcluded
7. Attributes

### QualityKind

1. General
2. Design
3. Type
4. Piece
5. Connection
6. Port

### Quality

1. Key
2. Name
3. Description
4. Uri
5. Scalable
6. Kind
7. DefaultSiUnit
8. DefaultImperialUnit
9. Min
10. MinExcluded
11. Max
12. MaxExcluded
13. Default
14. Formula
15. Benchmarks
16. Attributes

### Prop

1. Key
2. Value
3. Unit
4. Attributes

### Representation

1. Tags
2. Url
3. Description
4. Attributes

### Piece

1. Id,
2. Type,
3. Design,
4. Plane,
5. Center,
6. Scale,
7. MirrorPlane,
8. Props,
9. IsHidden,
10. IsLocked,
11. Color,
12. Attributes

### Design

1. Name
2. Variant
3. View
4. Description
5. Icon
6. Image
7. Concepts
8. Authors
9. Location
10. Unit
11. CanScale
12. CanMirror
13. Layers
14. Pieces
15. Groups
16. Connections
17. Props
18. Stats
19. Attributes

### Kit

1. Name
2. Version
3. Description
4. Icon
5. Image
6. Concepts
7. Remote
8. Homepage
9. License
10. Authors
11. Pieces
12. Groups
13. Connections
14. Props
15. Stats
16. Attributes

# Files

Every of those files is affected:

├── engineering
│ ├── dataarchitecture.pu # blueprint for sql schemas
│ ├── interfacearchitecture.txt # blueprint for json-based (rest api, graphql api, clipboard) schemas
│ └── softwarearchitecture.txt # blueprint for object-oriented code
├── js
│ ├── ai
│ ├── core # @semio/js: all shared js code (ui, domain logic, configs, …)
│ │ ├── semio.ts # all domain logic
│ │ ├── store.ts # react hooks for state (uses yjs)
│ │ ├── components
│ │ │ ├── ui
│ │ │ │ ├── sketchpad
│ │ │ │ │ │ ├── Chat.tsx
│ │ │ │ │ │ ├── Commands.tsx
│ │ │ │ │ │ ├── Console.tsx
│ │ │ │ │ │ ├── DesignEditor.tsx
│ │ │ │ │ │ ├── Diagram.tsx
│ │ │ │ │ │ ├── Model.tsx
│ │ │ │ │ │ ├── Navbar.tsx
│ │ │ │ │ │ ├── Workbench.tsx
│ │ │ │ │ │ ├── Sketchpad.stories.tsx
│ │ │ │ │ │ ├── Sketchpad.tsx # main component of @semio/
│ │ │ │ │ │ ├── TypeEditor.tsx
│ │ │ │ │ │ └── Workbench.tsx
├── meta
│ └── dictionary.csv # alphebetically ordered by name column
├── net
│ ├── Semio
│ │ └── Semio.cs # @semio/net: all .NET code
│ ├── Semio.Grasshopper
│ │ └── Semio.Grasshopper.cs # @semio/gh: all grasshopper code
│ ├── Semio.Grasshopper.Tests
│ │ └── Semio.Grasshopper.Tests.cs # all grasshopper test code
│ └── Semio.Tests
│ │ └── Semio.Grasshopper.cs # all .NET test code
└── README.md # GFM dev docs
