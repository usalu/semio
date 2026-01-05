---
slug: SKETCHPAD-POLICY
prompt: 'Add a new policy for sketchpad covering: 1) Third party imports must only be in elements.tsx which reexports reusable UI elements, 2) State management rules - single state machine (createMachine once, no createActor, Yjs only for kit data sync not app state), 3) UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()'
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-03T10:14:40Z"
commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
model: claude-opus-4-5
iterations:
    - prompt: 'Add a new policy for sketchpad covering: 1) Third party imports must only be in elements.tsx which reexports reusable UI elements, 2) State management rules - single state machine (createMachine once, no createActor, Yjs only for kit data sync not app state), 3) UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()'
      model: claude-opus-4-5
      date:
        started: "2026-01-03T10:14:40Z"
        ended: "2026-01-03T10:20:12Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      files:
        updated:
            - path: go/repo/main.go
              lines:
                added: 177
                removed: 11
            - path: prompts/ueli.md
              lines:
                added: 1
                removed: 1
        created: []
        removed: []
      lines:
        added: 178
        removed: 12
---
# Previously

# Plan

# Changes