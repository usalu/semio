---
slug: GRASSHOPPER-COMPONENTS
summary: Fix Grasshopper entity components to match semio.ts schema
---

# Previously

The Grasshopper components in `Semio.Grasshopper.cs` were missing several entity types (Folder, Tag, Concept, Interface) and the Layer entity used outdated schema (Name instead of Path/Guid).

# Plan

1. Add missing Folder entity region (Goo, Param, Component, Id*, Diff*, sDiff\*, Serialize/Deserialize)
2. Add missing Tag entity region
3. Add missing Concept entity region
4. Add missing Interface entity region
5. Fix Layer component to use new schema (Guid, Path instead of Name)
6. Verify build succeeds

# Changes

## Semio.Grasshopper.cs

### New Entity Regions Added

- **Folder** (after File region): FolderGoo, FolderParam, FolderComponent, FolderIdGoo, FolderIdParam, FolderDiffGoo, FolderDiffParam, FolderDiffComponent, FoldersDiffGoo, FoldersDiffParam, FoldersDiffComponent, Serialize/Deserialize components
- **Tag** (after Quality region): TagGoo, TagParam, TagComponent, TagIdGoo, TagIdParam, Serialize/Deserialize components
- **Concept** (after Port region): ConceptGoo, ConceptParam, ConceptComponent, ConceptIdGoo, ConceptIdParam, Serialize/Deserialize components
- **Interface** (after Concept region): InterfaceGoo, InterfaceParam, InterfaceComponent, InterfaceIdGoo, InterfaceIdParam, Serialize/Deserialize components

### Updated Components

- **LayerComponent**: Updated to use new Layer schema with Guid, Path (instead of Name), IsHidden, IsLocked, Color, Description, Attributes
- **LayerGoo**: Updated to cast to Guid instead of Name
