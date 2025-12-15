---
date:
  created: '2025-11-26T17:12:27.700Z'
  updated: '2025-11-26T17:12:27.700Z'
slug: UI-ELEMENT-IDS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Add missing IDs to all Sketchpad UI elements
model: claude-sonnet-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

Task: Analyze the Sketchpad codebase and add missing IDs to all UI elements following the `semio.sketchpad.*` naming convention.

# Plan

1. Search for existing ID patterns in the codebase
2. Identify components missing IDs (Button, Toggle, Action, Input, Select, Combobox, TreeItem, TreeSection)
3. Add IDs following the established naming convention
4. Update documentation in AGENTS.md

# Changes

## Design.tsx

- Added ID `semio.sketchpad.app.design.diagram.clusterMenu.cluster` to Cluster button in ClusterMenu component
- Added ID `semio.sketchpad.app.design.diagram.expandMenu.expand` to Expand button in ExpandMenu component

## Tutorials.tsx

- Added ID `semio.sketchpad.tutorial.controls.stop` to tutorial stop button
- Added ID `semio.sketchpad.tutorial.controls.previous` to previous milestone button
- Added ID `semio.sketchpad.tutorial.controls.playPause` to play/pause button
- Added ID `semio.sketchpad.tutorial.controls.next` to next milestone button
- Added ID `semio.sketchpad.recording.controls.playPause` to recording play/pause button
- Added ID `semio.sketchpad.recording.controls.stop` to recording stop button

## elements.tsx

- Added ID `semio.sketchpad.docs.navigation.previous` to page navigation previous button
- Added ID `semio.sketchpad.docs.navigation.next` to page navigation next button

## Verified Files (No Changes Needed)

- Kit.tsx: All interactive components already have IDs
- Type.tsx: All interactive components already have IDs
- Home.tsx: All interactive components already have IDs
- Docs.tsx: All interactive components already have IDs
- Quality.tsx: All interactive components already have IDs
- Sketchpad.tsx: All interactive components already have IDs

## i18n Translations Added

### en.json

- `semio.sketchpad.app.design.diagram.clusterMenu.cluster` - Cluster button translations
- `semio.sketchpad.app.design.diagram.expandMenu.expand` - Expand button translations
- `semio.sketchpad.tutorial.controls.stop` - Tutorial stop button
- `semio.sketchpad.tutorial.controls.previous` - Previous milestone button
- `semio.sketchpad.tutorial.controls.playPause` - Play/pause button
- `semio.sketchpad.tutorial.controls.next` - Next milestone button
- `semio.sketchpad.recording.controls.playPause` - Recording play/pause button
- `semio.sketchpad.recording.controls.stop` - Recording stop button
- `semio.sketchpad.docs.navigation.previous` - Docs previous page button
- `semio.sketchpad.docs.navigation.next` - Docs next page button

### de.json

- Same keys with German translations
