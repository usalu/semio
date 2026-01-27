# Ticket

## Todos

- [x] Find extractSections function and understand the issue
- [x] Fix section and definition parsing logic

## Changes

- `js/vscode/extension.ts` - Fixed `extractSections()` to handle `result.data.file.sections` structure
- `js/vscode/extension.ts` - Added `extractDefinitions()` function for consistent definition extraction
- `js/vscode/extension.ts` - Updated `loadFileContent()` to use `extractDefinitions()`

## Log

1. Repo binary returns nested structure: `{ data: { file: { sections: [...] } } }`
2. `extractSections()` checked for `data.sections` and `result.file.sections` but not `data.file.sections`
3. Added check for `data.file.sections` in `extractSections()`
4. Found same issue with definitions - `loadFileContent()` called `defResult.data.map()` but `defResult.data` is an object, not array
5. Created `extractDefinitions()` function to properly extract from `data.file.definitions`

## Summary

Fixed extractSections and added extractDefinitions to handle nested data.file structure from repo binary
