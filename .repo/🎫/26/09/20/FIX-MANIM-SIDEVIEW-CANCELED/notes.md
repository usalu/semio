# Fix Manim Sideview errno 89

`manim-sideview.runOnSave` SIGTERM’d a live render, then the next import stalled reading `.venv/.../click-.../METADATA` (`OSError: [Errno 89] Operation canceled`).

Changes:

- `runOnSave`: false
- manim path: `manim` (Sideview already prefixes `.venv/bin/`; `${workspaceFolder}` is not expanded)
- version: v0.21.0
- replaced wedged click dist-info inode
- launch entry for Beat3_Konvektion
