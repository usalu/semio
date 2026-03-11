### Summary

Rhino 8 plugin with embedded React UI for importing semio kits and models into Rhino documents.

### Specs

- Bundle contains two packages: `ui` (npm/React) and `rhp` (.NET/RhinoCommon).
- The `rhp` plugin registers a dockable panel hosting WebView2 with the React `ui`.
- The `ui` displays a tree view of kits, types, models, and designs.
- Import Kit action loads a semio kit into the tree.
- Import Model action creates Rhino geometry on layers: `semio > KITNAME > Types > TYPENAME > Models > MODELTAGS`.
- Bridge protocol uses JSON-RPC style messages between React and C#.

### Docs

#### Tree Structure

```
Kits                    # Import action
  KIT
    Types
      TYPE
        Models
          Model         # Import action
    Designs
      Design
```

#### Layer Structure (on import)

```
semio
  KITNAME
    Types
      TYPENAME
        Models
          MODELTAGS
```

### Requirements
