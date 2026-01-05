---
slug: COMMENT-MULTILINE
prompt: The code analysis and fixing hook doesnt identify and delete multi-line comments. Extend them.
summary: Extend code hook to detect and strip multi-line comments
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-22T21:50:41.507Z"
    finished: "2025-12-22T21:56:03.179Z"
commit: 4ff5d7c32877787f9c7f359939d1b045760a9410
model: gpt-5.2-codex
iterations:
    - prompt: Extend code hook to detect and strip multi-line comments.
      model: gpt-5.2-codex
      date:
        started: "2025-12-22T21:52:12.318Z"
        ended: "2025-12-22T21:55:26.714Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 4ff5d7c32877787f9c7f359939d1b045760a9410
      bundles:
        '@semio':
            files:
                "":
                    sections: {}
      files:
        updated:
            - path: ""
            - path: ""
            - path: ""
      lines:
        added: 162
        removed: 98
bundles:
    '@semio':
        files:
            "":
                sections: {}
files:
    updated:
        - path: ""
        - path: ""
        - path: ""
lines:
    added: 162
    removed: 98
---


# Previously

Code hook missed multi-line/JSDoc comments during scan and fix.

# Plan

Extend TypeScript comment collection to include JSDoc ranges and update dev docs.

# Changes

Collected JSDoc comment ranges in the TypeScript comment scan/strip logic and documented multi-line comment enforcement in README/AGENTS.
