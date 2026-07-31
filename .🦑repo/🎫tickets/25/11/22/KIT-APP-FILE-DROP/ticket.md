# Ticket

## Todos

# Kit App File Drop Implementation

## 1. Problem Description

When a zip file is dropped onto the kit app, we need to:

1. Check if it contains a `.compose` folder (indicating it's a kit)
2. If it's a kit: Import the entire kit
3. If it's not a kit: Import just the files into the current kit

## 2. Analysis

### 2.1. Current State

- Kit app exists in `js/compose/sketchpad/apps/kit/App.tsx`
- Kit import functionality exists via `compose.kit.import` command
- File import functionality exists via file operations
- Need to add drop zone handling to kit app canvas

### 2.2. Required Changes

1. **Add drop zone to kit app canvas** ✅
   - Handle file/folder drops on the canvas area
   - Accept `.zip` files
   - Extract and inspect zip contents

2. **Inspect zip contents** ✅
   - Check for `.compose` folder in root
   - Determine if it's a kit or just files

3. **Execute appropriate import** ✅
   - If `.compose` folder exists: Call `compose.kit.import` with the kit data
   - If no `.compose` folder: Call file import commands for individual files

## 3. Implementation Summary

### 3.1. Changes Made

1. **Added JSZip import** (`js/compose/sketchpad/apps/kit/App.tsx`)
   - Imported JSZip library for zip file handling

2. **Created KitDropZone component** (`js/compose/sketchpad/apps/kit/App.tsx`)
   - Handles drag over, drag leave, and drop events
   - Detects when zip files are being dragged
   - Shows visual feedback during drag
   - Inspects dropped zip files for `.compose/kit.db`
   - Routes to appropriate import logic based on contents

3. **Wrapped App canvas with KitDropZone** (`js/compose/sketchpad/apps/kit/App.tsx`)
   - All kit app content is now wrapped in the drop zone

4. **Added i18n translations** (`js/compose/sketchpad/locales/en.json`, `js/compose/sketchpad/locales/de.json`)
   - `compose.sketchpad.app.kit.dropzone.label`
   - `compose.sketchpad.app.kit.dropzone.description`

### 3.2. How It Works

1. User drags a `.zip` file over the kit app
2. Drop zone activates and shows visual feedback
3. User drops the file
4. System reads the zip file and checks for `.compose/kit.db`
5. **If kit detected** (has `.compose/kit.db`):
   - Calls `compose.kit.import` command with the zip's ArrayBuffer
   - Entire kit is imported (types, designs, files, etc.)
6. **If not a kit** (no `.compose/kit.db`):
   - Extracts all files from the zip
   - Calls `compose.kit.addFile` for each file
   - Files are added to the current kit

### 3.3. Visual Feedback

When dragging a zip file over the kit app:

- Overlay appears with semi-transparent background
- Document icon displayed
- Label: "Drop zip file to import"
- Description: "Kits with .compose folder will be imported, others will be added as files"

## 4. Files Modified

1. ✅ `js/compose/sketchpad/apps/kit/App.tsx`
   - Added JSZip import
   - Added KitDropZone component
   - Wrapped canvas with drop zone

2. ✅ `js/compose/sketchpad/locales/en.json`
   - Added dropzone labels and descriptions

3. ✅ `js/compose/sketchpad/locales/de.json`
   - Added German translations for dropzone

## 5. Testing Notes

To test this feature:

1. Navigate to a kit in the kit app
2. Drag a `.compose.zip` kit file (with `.compose` folder) onto the canvas
   - Should import the entire kit
3. Drag a regular `.zip` file (without `.compose` folder) onto the canvas
   - Should add all files to the current kit
4. Drag non-zip files
   - Should not trigger the drop zone

## 6. Future Enhancements

- Add progress indicator for large zip files
- Add error messages when import fails
- Support drag-and-drop of individual files (not just zips)
- Add undo support for file imports

## Changes

## Log

## Summary
