# Move Settings and Chat to Main Container

## Status: OPEN

## Goal
Move Settings and Chat panels from right side panel into main canvas container as windows.

## Plan
1. Gather context: shared.ts (PanelKind, WindowKind, panelKindConfigs), app files, Sketchpad.tsx, elements.tsx
2. Remove SETTINGS and CHAT from PanelKind enum and panelKindConfigs
3. Add SETTINGS and CHAT to WindowKind enum
4. Remove SETTINGS and CHAT panel definitions from all app files (Home, Kit, Type, Design)
5. Add Settings and Chat window content rendering in Sketchpad.tsx
6. Update navbar/footer to open Settings/Chat as windows
7. Update tests
8. Verify build and tests pass

## TODOs
- [ ] Gather context from key files
- [ ] Remove Settings/Chat from panel system
- [ ] Add Settings/Chat to window system
- [ ] Add window content rendering
- [ ] Update navigation
- [ ] Update tests
- [ ] Verify everything works

## Changes
(tracked as work progresses)

## Summary
(filled on close)
