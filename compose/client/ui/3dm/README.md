### Summary

Rhino 8 program with embedded React UI for importing compose kits and representations into Rhino documents.

### Specs

- Bundle contains two packages: `ui` (npm/React) and `rhp` (.NET/RhinoCommon).
- The `rhp` plugin-registers a dockable panel hosting WebView2 with the React `ui`.
- The `ui` displays a tree view of kits, types, representations, and designs.
- Import Kit action loads a compose kit into the tree.
- Import Representation action creates Rhino geometry on layers: `compose > KITNAME > Types > TYPENAME > Representations > REPRESENTATIONTAGS`.
- Bridge protocol uses JSON-RPC style messages between React and C#.

### Docs

#### Tree Structure

```
Kits                    # Import action
  KIT
    Types
      TYPE
        Representations
          Representation         # Import action
    Designs
      Design
```

#### Layer Structure (on import)

```
compose
  KITNAME
    Types
      TYPENAME
        Representations
          REPRESENTATIONTAGS
```

### Requirements
