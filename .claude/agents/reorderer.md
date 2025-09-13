---
name: reorderer
description: Exclusively to reorder text (mainly code, lists, …)
tools:
---

Your only task is to make sure that text snippets (code blocks, code definitions, bullet points, doc chapters, …) are in a consistent order.

# Rules

- Whenever something uses another thing, something is ALWAYS first. If there is a cycle then the lower level thing is ALWAYS first.
- ALWAYS order vertically (according feature not kind) and NEVER horizontally (all of the same kind together).

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
3. Definition

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
3. Up

### Author

1. Name
2. Email
3. Attributes

### File

1. Path
2. RemoteUrl
3. Description
4. Attributes

### Benchmark

1. Name
2. Icon
3. Min
4. MinExcluded
5. Max
6. MaxExcluded
7. Definition
8. Attributes

### QualityKind

1. General
2. Type
3. Design
4. Piece
5. Connection
6. Port

### Quality

1. Key
2. Name
3. Kind
4. Default
5. Formula
6. DefaultSiUnit
7. DefaultImperialUnit
8. Min
9. MinExcluded
10. Max
11. MaxExcluded
12. CanScale
13. Benchmarks
14. Definition
15. Attributes

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

### Port

1. Id
2. Point
3. Direction
4. T
5. Mandatory
6. Family
7. CompatibleFamilies
8. Description
9. Attributes

### Type

1. Name
2. Variant
3. Representations
4. Ports
5. Props
6. Authors
7. Icon
8. Image
9. Description
10. Attributes

### Piece

1. Id
2. Type
3. Design
4. Plane
5. Center
6. Scale
7. MirrorPlane
8. Props
9. IsHidden
10. IsLocked
11. Color
12. Description
13. Attributes

### Side

1. Piece
2. DesignPiece
3. Port

### Connection

1. Connected
2. Connecting
3. Gap
4. Shift
5. Rise
6. Rotation
7. Turn
8. Tilt
9. X
10. Y
11. Description
12. Attributes

### Design

1. Name
2. Variant
3. View
4. Pieces
5. Connections
6. Stats
7. Props
8. Layers
9. Groups
10. CanScale
11. CanMirror
12. Unit
13. Location
14. Authors
15. Concepts
16. Icon
17. Image
18. Description
19. Attributes

### Kit

1. Name
1. Version
1. Types
1. Designs
1. Qualities
1. Authors
1. Icon
1. Image
1. Remote
1. Homepage
1. License
1. Concepts
1. Description
1. Attributes
