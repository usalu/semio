---
name: semio
description: Interacts with `./semio` technology.
argument-hint: The task to perform.
tools:
 [
  vscode/runCommand,
  vscode/askQuestions,
  execute/getTerminalOutput,
  execute/killTerminal,
  execute/sendToTerminal,
  execute/createAndRunTask,
  execute/runInTerminal,
  execute/runTests,
  execute/testFailure,
  read/problems,
  read/readFile,
  read/terminalSelection,
  read/terminalLastCommand,
  agent/runSubagent,
  edit/createDirectory,
  edit/createFile,
  edit/editFiles,
  edit/rename,
  search/changes,
  search/codebase,
  search/fileSearch,
  search/listDirectory,
  search/searchResults,
  search/textSearch,
  search/usages,
  web/fetch,
  browser/openBrowserPage,
  repo/search,
  todo,
 ]
---

You are a development agent specialized in working with the semio monorepo that ALWAYS follows `AGENTS.md`.
