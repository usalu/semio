---
slug: WINDOW-TABLE-ACTIVE-BG
prompt: Every app has windows and there is always one active window. Make sure that the background of the table is set to active background color.
summary: Use active window background for table
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-18T15:34:11.986Z"
    finished: "2025-12-18T16:32:26.549Z"
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
    - prompt: Every app has windows and there is always one active window. Make sure that the background of the table is set to active background color.
      model: gpt-5.2-codex
      date:
        started: "2025-12-18T16:02:05.033Z"
        ended: "2025-12-18T16:32:16.412Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
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
            - path: ""
            - path: ""
            - path: ""
      lines:
        added: 644
        removed: 885
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
        - path: ""
        - path: ""
        - path: ""
lines:
    added: 650
    removed: 885
---


# Previously

- The kit app renders a table inside a multi-window layout, but there was no explicit active-window state for the app.
- The table header background was not guaranteed to match the window surface background when rendered inside GoldenLayout windows.

# Plan

- Track `activeWindow` for the kit app and keep it valid across window kinds.
- Highlight the active window surface and make the table background follow the active window background.
- Document the active-window + background mechanism in root docs.

# Changes

- Updated `LayoutCanvas` to apply an active-window background tint to the active window root.
- Updated the kit app multi-window entrypoint to maintain and pass `activeWindow` into `LayoutCanvas`.
- Updated the shared `Table` component to inherit its surface background so it matches the window surface.
