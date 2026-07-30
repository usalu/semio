---
technology: reasoning
path: 🧠reasoning🔗wires
bundle:
 name: wires
 emoji: 🔗
 description: WIRES mindmap specialization.
 kind: library
---

# WIRES

WIRES (o*W*ns, *I*s, *Re*ferences, ha*S*) is a specialized [mindmap](../AGENTS.md#mindmap) with exactly four **relationship kinds** over a flexible set of **identity kinds**.

## Glossary

| Term                 | Meaning                                                                                                                 |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| **Identity**         | One vertex in the WIRES graph (a topic instance).                                                                       |
| **IdentityKind**     | Registered kind for identities: shape, color, icon (`identityKind` in fixture; `nodeKind` on the puzzle board adapter). |
| **Relationship**     | One directed link between two identities.                                                                               |
| **RelationshipKind** | One of `Owns`, `Is`, `References`, `Has` (`kind` in fixture; `edgeKind` `wires.{kind}` on the board).                   |

Puzzle **Wire** is only the transient link-drag cable in ported puzzle 2d mode — never use “wire” for a WIRES relationship in user-facing copy.

## Adapter

Rendering and play chrome use `@puzzle/2d` in `graphPortMode: "normal"`. WIRES fixture fields map to puzzle fields at the board boundary only.
