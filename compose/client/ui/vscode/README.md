---
name: vscode
kind: bundle
emoji: 🖱️
summary: VS Code extension providing a sketchpad-based custom editor for compose kit JSON files.
---

### Summary
VS Code extension providing a sketchpad-based custom editor for compose kit JSON files.

### Specs
- Registers a CustomTextEditorProvider for `*.kit.json`, `kit_*.json`, `kit-*.json`, and `**/.compose/kit.json`
- Loads the built sketchpad app in a webview panel from bundled `sketchpad-dist` or sibling `../sketchpad/dist`
- Bridges file read/write between VS Code filesystem and the sketchpad webview via postMessage
- Falls back to a simple HTML view when sketchpad-dist is not available
