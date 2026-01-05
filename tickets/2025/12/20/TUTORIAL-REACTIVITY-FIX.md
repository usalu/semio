---
slug: TUTORIAL-REACTIVITY-FIX
prompt: 'Fix Smell 5: Non-reactive snapshot reads in Tutorials.tsx - RecordingControlsContent now uses reactive hooks instead of store.snapshot()'
summary: RecordingControlsContent now uses useRecordingState and useActiveRecording hooks for reactivity
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-20T13:19:54.449Z"
    finished: "2025-12-20T13:20:10.293Z"
commit: 4ff6fd77dee713af972c27bd3761939be4302c80
model: claude-opus-4-5
iterations:
    - prompt: 'Fix Smell 5: RecordingControlsContent uses reactive hooks'
      model: claude-opus-4-5
      date:
        started: "2025-12-20T13:20:00.192Z"
        ended: "2025-12-20T13:20:04.826Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 4ff6fd77dee713af972c27bd3761939be4302c80
      bundles:
        '@semio':
            files:
                "":
                    sections: {}
      files:
        updated:
            - path: ""
      lines:
        added: 11
        removed: 23
bundles:
    '@semio':
        files:
            "":
                sections: {}
files:
    updated:
        - path: ""
lines:
    added: 11
    removed: 23
---


# Previously

# Plan

# Changes
