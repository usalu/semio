---
slug: VSCODE-TICKET-TREE-ACTIONS
prompt: 'vscode extension: Remove Open Ticket button from tickets. Instead add reopen and close icons and execute the command once pressed. Remove status emoji from ticket. Add commit tree item. Just show description on ticket tree item hover.'
status: closed
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-03T00:11:53Z"
    finished: "2026-01-03T00:33:52Z"
commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
iterations:
    - prompt: Remove Open Ticket button from tickets
      model: ""
      date:
        started: "2026-01-03T00:12:02Z"
        ended: "2026-01-03T00:17:05Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      files:
        updated:
            - path: go/repo/main.go
              lines: null
            - path: js/vscode/extension.ts
              lines: null
            - path: js/vscode/package.json
              lines: null
        created: []
        removed: []
      lines: null
    - prompt: Remove status emoji from ticket
      model: ""
      date:
        started: "2026-01-03T00:17:42Z"
        ended: "2026-01-03T00:19:58Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      files:
        updated:
            - path: js/vscode/extension.ts
              lines: null
        created: []
        removed: []
      lines: null
    - prompt: Add commit tree item
      model: ""
      date:
        started: "2026-01-03T00:20:05Z"
        ended: "2026-01-03T00:20:57Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      files:
        updated:
            - path: js/vscode/extension.ts
              lines: null
        created: []
        removed: []
      lines: null
    - prompt: Just show description on ticket tree item hover
      model: ""
      date:
        started: "2026-01-03T00:21:06Z"
        ended: "2026-01-03T00:21:20Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      files:
        updated:
            - path: js/vscode/extension.ts
              lines: null
        created: []
        removed: []
      lines: null
---
# Previously

# Plan

# Changes
- Updated VS Code ticket actions, hover tooltip behavior, and commit tree items.
- Added repo ticket reopen command.
- Documented ticket tree behavior in README and AGENTS.
