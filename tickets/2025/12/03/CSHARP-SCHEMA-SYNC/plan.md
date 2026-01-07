# Previously

The C# schema in Semio.cs was out of date compared to the TypeScript semio.ts. Several entity reference types (IDs) were stored as strings in C# but as objects with `{ guid: string }` in TypeScript.

# Plan

1. Update QualityId to use Guid instead of Key
2. Add Guid to Quality and QualityDiff
3. Add TagId class and Tag class
4. Add ConceptId, Concept, ConceptDiff, ConceptsDiff classes
5. Update Model.File from string to FileId
6. Update Model.Tags from List<string> to List<TagId>
7. Update File.Folder from string to FolderId
8. Update Prop to use QualityId instead of Key string
9. Update Type/Design.Concepts from List<string> to List<ConceptId>
10. Update Kit.Concepts from List<string> to List<Concept>
11. Update KitDiff.Concepts from List<Concept> to ConceptsDiff
12. Fix Grasshopper components for new types

# Changes

## Semio.cs

- **QualityId**: Changed from `Key` (string) to `Guid` (string)
- **QualityDiff**: Added `Guid` property
- **Quality**: Added `Guid` property
- **TagId**: New class with `Guid` property for referencing tags
- **Tag**: New class for tag entities (guid, name, description, icon, attributes)
- **ConceptId**: New class with `Guid` property for referencing concepts
- **Concept**: New class for concept entities (guid, name, description, icon, attributes)
- **ConceptDiff**: New class for concept diffs
- **ConceptsDiff**: New class for multiple concepts diff (removed/added/updated)
- **Prop**: Changed from `Key` (string) to `Quality` (QualityId), added `Guid`
- **ModelId**: Changed from `Tags` (List<string>) to `Guid` (string)
- **Model.File**: Changed from `string` to `FileId`
- **Model.Tags**: Changed from `List<string>` to `List<TagId>`
- **ModelDiff**: Updated to use `FileId?` and `List<TagId>`
- **File.Folder**: Changed from `string?` to `FolderId?`
- **FileDiff.Folder**: Changed from `string?` to `FolderId?`
- **Type.Concepts**: Changed from `List<string>` to `List<ConceptId>`
- **TypeDiff.Concepts**: Changed from `List<string>?` to `List<ConceptId>?`
- **Design.Concepts**: Changed from `List<string>` to `List<ConceptId>`
- **DesignDiff.Concepts**: Changed from `List<string>?` to `List<ConceptId>?`
- **Kit.Concepts**: Changed from `List<string>` to `List<Concept>`
- **KitDiff.Concepts**: Changed from `List<Concept>?` to `ConceptsDiff?`

## Semio.Grasshopper.cs

- **QualityIdGoo**: Updated to use `Guid` instead of `Key`
- **PropGoo**: Updated to use `Quality.Guid` instead of `Key`
- **ModelGoo**: Updated cast to use `Guid` instead of `Tags`
- **ModelIdGoo**: Updated to use `Guid` instead of `Tags`
- **ModelComponent**: Updated to handle `FileId` and `List<TagId>` types
- **FileComponent**: Updated to handle `FolderId` type
