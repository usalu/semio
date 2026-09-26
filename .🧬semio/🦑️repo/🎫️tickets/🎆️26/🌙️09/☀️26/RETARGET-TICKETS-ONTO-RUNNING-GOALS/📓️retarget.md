# Retarget Tickets Onto Running Goals

2414 tickets had their `goal` field moved onto a goal in the `RUNNING-FRAMEWORK` tree. The stored id is the compose id, for example `🎯runningframework🎯runningproducts🎯runningos🎯runnings`.

Tickets with no node in the new tree kept their current goal.

## Where they landed

- Running S: 1229. Old sketchpad goals, including the design, type, kit, docs, and home apps.
- Running Repo: 896. The AI-optimized repo tree and its mechanism children.
- Running Flow: 64
- Running Presentation: 42
- Running Framework: 32
- Running CAD: 24
- Running Puzzle 3d: 21
- Running Puzzle 2d: 15
- Running Norm: 13. Includes tickets that name more than one standard.
- Running Puzzle 5d: 9
- Running Trinity: 8
- Running Procedural: 8
- Running Energy: 7
- Running Lowpoly: 7
- Running Animate: 7
- Running GIS Map: 5
- Running Mathematical: 4
- Running Architect: 4
- Running Layout: 4
- Running Puzzle: 3
- Running OS: 3
- Running Print: 3
- Running Forms: 2
- Running EN 1997, Running EN 1991, Running VDI 3805, Running ISO 16757: 1 each

A ticket landed on a norm or puzzle artifact only when it named exactly one standard or exactly one puzzle dimension. A range such as EN 1994–1999 stayed on Running Norm.

## Left in place

- Updated Docs and its children. That work has no node in the new tree.
- R26-03, Running .NET, and Grasshopper.
- Bare `r26-02` tickets whose goal does not name sketchpad, a product, or a plugin.
- UI, compose, coda, elements, geometry, hooks, algorithms, and framework playground, UI, and platform.
- Tickets with an empty goal, and the goal-lifecycle test tickets.
- Three ticket documents that were already invalid JSON.
