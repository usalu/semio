---
slug: SCHEMA-TAGS-CONCEPTS-MODELS
summary: "Tags, Concepts become kit entities; Models link to files with guid"
prompt: "Tags, Concepts become kit entities; Models link to files with guid"
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.796Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Tags were simple strings on models, and concepts were simple strings on types/designs/kits. Models referenced files via a string path/url.

# Plan

1. Make Tag a proper kit entity with guid, name, description, icon, attributes
2. Make Concept a proper kit entity with guid, name, description, icon, attributes
3. Update Model to reference File by FileId (guid) instead of string path
4. Update Model to reference Tags by TagId (guid) instead of string names
5. Update Type/Design to reference Concepts by ConceptId (guid) instead of string names
6. Update Kit to contain tags and concepts arrays as entity collections

# Changes

## dataarchitecture.pu

- Added `tags` table with guid, name, description, icon, kit_id FK
- Added `concepts` table with guid, name, description, icon, kit_id FK
- Updated `models` table to use file_id FK instead of url string
- Added `model_tags` junction table referencing tag_id
- Updated `type_concepts` and `design_concepts` to use concept_id FK
- Added `kit_concepts` junction table

## interfacearchitecture.txt

- Updated Model to have `tags: *TagId[]` and `file: !FileId`
- Added Tag entity with guid, name, description, icon, attributes
- Added Concept entity with guid, name, description, icon, attributes
- Updated Type/Design to use `concepts: *ConceptId[]`

## softwarearchitecture.pu

- Added Tag class with guid, name, description, icon, attributes
- Added TagId, TagDiff, TagsDiff classes
- Added Concept class with guid, name, description, icon, attributes
- Added ConceptId, ConceptDiff, ConceptsDiff classes
- Updated Model to use `tags: list[TagId]` and `file: FileId`
- Updated Type/Design to use `concepts: list[ConceptId]`
- Updated Kit to include `tags: list[Tag]` and `concepts: list[Concept]`
- Added Tag/Concept to SemioEntityKind enum
- Added relationships: Kit defines Tags/Concepts, Model references Tags/File, Type/Design categorized by Concepts

## js/js/semio.ts

- Added TagId, ConceptId types and schemas
- Added TagSchema, TagDiff, TagsDiff with full diffing functions
- Added ConceptSchema, ConceptDiff, ConceptsDiff with full diffing functions
- Updated ModelSchema to use `tags: TagIdSchema[]` and `file: FileIdSchema`
- Updated TypeSchema/DesignSchema to use `concepts: ConceptIdSchema[]`
- Updated KitSchema to include `tags: TagSchema[]` and `concepts: ConceptSchema[]`
- Updated KitDiffSchema with TagsDiff and ConceptsDiff
- Updated getKitDiff, inverseKitDiff, mergeKitDiff, applyKitDiff
- Added findTagInKit, addTagToKit, setTagInKit, removeTagFromKit helpers
- Added findConceptInKit, addConceptToKit, setConceptInKit, removeConceptFromKit helpers
- Renamed getAllTagsFromModels to getAllTagGuidsFromModels, filterModelsByTags to filterModelsByTagGuids

## sql/sqlite/schema.sql

- Added `tag` table with guid, name, description, icon, kit_guid FK
- Updated `model` table to use file_guid FK instead of file string
- Updated `model_tag` to reference tag_guid FK instead of tag string
- Updated `concept` table to be kit entity with guid, name, description, icon, kit_guid FK
- Updated `type_concept` to use concept_guid FK with proper FKs
- Updated `design_concept` to use concept_guid FK with proper FKs
- Added tag_guid and concept_guid columns to attribute table with FKs

## Additional Fixes

- Fixed `applyLocationDiff` - added required `guid` property to result
- Fixed `applyPropDiff` - added required `value` property to result
- Fixed `applyPortDiff` - added required `t` property to result

## Pending

- py/engine/engine.py needs similar updates for Python Tag/Concept entities
- net/Semio/Semio.cs needs similar updates for C# Tag/Concept classes
