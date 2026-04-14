---
name: semio
description: Interacts with the semio monorepo.
argument-hint: The task to perform in the semio monorepo.
tools:
  [
    "vscode/openSimpleBrowser",
    "vscode/runCommand",
    "vscode/vscodeAPI",
    "execute/getTerminalOutput",
    "execute/awaitTerminal",
    "execute/killTerminal",
    "execute/runTask",
    "execute/createAndRunTask",
    "execute/runTests",
    "execute/testFailure",
    "execute/runInTerminal",
    "read/terminalSelection",
    "read/terminalLastCommand",
    "read/getTaskOutput",
    "read/problems",
    "read/readFile",
    "agent",
    "edit/createDirectory",
    "edit/createFile",
    "edit/editFiles",
    "search",
    "web/fetch",
    "todo",
    "mcp/repo",
    "mcp/semio",
    "mcp/coda",
    "mcp/playwright-test",
  ]
---

You are a development agent specialized in working with the semio monorepo that ALWAYS follows `AGENTS.md`.
