---
description: Help senior developer working.
tools:  [
    'changes',
    'codebase',
    'editFiles',
    'fetch',
    'findTestFiles',
    'problems',
    'readCellOutput',
    'runCommands',
    'runNotebooks',
    'runTasks',
    'runTests',
    'search',
    'searchResults',
    'terminalLastCommand',
    'terminalSelection',
    'testFailure',
    'updateUserPreferences',
    'usages',
    'vscodeAPI',
    'markitdown'
  ]
model: Gemini 2.5 Pro (Preview)
---

# Project

An ecosystem for designing kit-of-parts architecture together.

# Guidelines

- Toolfriendly over intuitive.
- Almost everything is in a single file if possible.
- Folders are avoided if possible.
- No need to ask in between. Be opionionated and just go for it. Try to do as much as you can.
- Don't ask to execute commands in the cli. Assume all the tools are properly setup.
- No need to delete files, you can tell me when once you are 100% done with everything and I will do it manually.
- You are allowed to change everything (names, apis, creating/modifying/deleting fields, props, …) and don't have to worry about breaking compatiblity. Choose the most elegant approach even if it requires changing more than necessary.
- Don't create additional example files and implement it directly in the dependent parts.
- Don't remove code that is commented out.
- README.md files are for developers which are GFM.
- Assume all dev servers and debugging processes are running. No need to ask to start them.

# Glossary

## Nouns

- Kit: A collection of types and designs. Can be either static (a special .zip file) or dynamic (bound to a runtime).
- Design: An undirected graph of pieces (nodes) and connections (edges).
- Type: A reusable component with different representations and ports.
- Piece: An instance of either a type or a design.
- Port: A conceptual connection point with an outwards direction.
- Connection: A 3D-Link between two pieces with translation parameters (gap, shift, rise) and rotation parameters (rotation, turn, tilt).
- Representation: A tagged url to a resource with an optional description.
- Quality: Metadata with a name, an optional value, an optional unit and an optional definition (url or text).
- Tag: A kebab-cased name.
- Plane: A location (origin) and orientation (x-axis, y-axis and derived z-axis) in 3D space.
- Url: Either relative (to the root of the .zip file) or remote (http, https, ftp, …) string.
- Cluster: A group of connected pieces.
- Hierarchy: The length of the shortest path to the next fixed piece.
- Vector: A vector in 3D space.
- Point: A point in 3D space.

## Adjectives

- A `fixed` piece is a piece with a plane.
- A `linked` piece is a piece that is not fixed and is connected with a connection.
- A `connected` piece that is not `fixed` and is connected to at least one other piece.
- A `flat` design has no connections and all pieces are fixed.
- A `mandatory` port is a port that must be connected in a design.
- A `static` kit is a special .zip file.
- A `dynamic` kit is bound to a runtime.
- A `relative` url is relative to the root of the .zip file.
- A `remote` url is http, https, ftp, etc.
- A `default` representation has no tags.
- A `default` port family means the port is compatible with all other ports.
- A `virtual` type is an intermediate type that needs other `virtual` types to form a `physical` type.
