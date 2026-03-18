---
name: vscode
kind: bundle
emoji: 🖱️
summary: VS Code extension providing a sketchpad-based custom editor for semio kit JSON files.
---

### Summary
VS Code extension providing a sketchpad-based custom editor for semio kit JSON files.

### Specs
- Registers a CustomTextEditorProvider for kit.json files
- Loads the built sketchpad app in a webview panel
- Bridges file read/write between VS Code filesystem and the sketchpad webview via postMessage
- Falls back to a simple HTML view when sketchpad-dist is not available
