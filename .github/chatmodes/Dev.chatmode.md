---
description: Help senior developer working.
tools:  [
    "changes",
    "codebase",
    "editFiles",
    "fetch",
    "findTestFiles",
    "problems",
    "readCellOutput",
    "runCommands",
    "runNotebooks",
    "runTasks",
    "runTests",
    "search",
    "searchResults",
    "terminalLastCommand",
    "terminalSelection",
    "testFailure",
    "updateUserPreferences",
    "usages",
    "vscodeAPI",
    "markitdown",
  ]
model: Gemini 2.5 Pro (Preview)
---

# Project

An ecosystem for designing kit-of-parts architecture together.

# Guidelines

- Toolfriendly over intuitive.
- Almost everything is in a single file if possible.
- Folders are avoided if possible.
- No need to ask in between. Be opionionated and just go for it. Try to do as much as you can.
- Don't ask to execute commands in the cli. Assume all the tools are properly setup.
- No need to delete files, you can tell me when once you are 100% done with everything and I will do it manually.
- You are allowed to change everything (names, apis, creating/modifying/deleting fields, props, …) and don't have to worry about breaking compatiblity. Choose the most elegant approach even if it requires changing more than necessary.
- Don't create additional example files and implement it directly in the dependent parts.
- Don't remove code that is commented out.
- README.md files are for developers which are GFM.
