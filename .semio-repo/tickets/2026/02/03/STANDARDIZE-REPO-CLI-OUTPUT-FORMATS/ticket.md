# Ticket

## Summary

Standardized markdown output formats for `ticket` and `goal` commands.

## Changes

- Modified `@semio-repo/cli/main.go` to use `semiorepo://ticket/SLUG` URI format for ticket links in markdown output of `ticket tree` command.

## Log

- Diagnosed `Goal_Tree_MD` double dash issue (found it was likely resolved or non-existent in current codebase, verified test pass).
- Diagnosed `Ticket_Tree_MD` failure (missing `semiorepo://` markers).
- Fixed `treeCmd` in `main.go` to use ticket URIs.
- Verified all tests in `TestMarkdownOutput` passed.

## Todos

- [x] Fix `Goal_Tree_MD`
- [x] Fix `Ticket_Tree_MD`
- [ ] Implement Unified ID System
  - [ ] Project: `<kind>@{project-code}` (👤, 🧰, 🔬)
  - [ ] Bundle: `<kind>@{project-code}/{code}` (📚, 🛂, ⌨️, 🖱️, 🌐, 🏪)
  - [ ] Folder: `<kind>{path*}` (🗃️, 📁)
  - [ ] File: `<kind>{path*}` (💻, 🧪, 📃, ⚙️, 💾, ⚖️)
  - [ ] Section: `🔖{path*}`
  - [ ] Definition: `<kind>{file-path*}#{section-path*}§{path*}` (🛠️, ✂️, 🪨)
  - [ ] Ticket: `📅{year}/{month}/{day}/{slug}`
  - [ ] Goal: `🎯{path*}`
  - [ ] Draft: `✍️{slug}`
  - [ ] Todo: `📝{slug}`
  - [ ] Policy: `🛡️/{slug}`
  - [ ] ViolationKind: `🚫{policy-slug}/{slug}`
  - [ ] Contributor: `👤{github}`
  - [ ] Commit: `🔀{sha}`
- [ ] Implement Standardized Output Formats
  - [ ] List Human: `<id> <semantic-description> ...`
  - [ ] List Markdown: `- [<id>](<uri>) - <semantic-description> ...`
- [ ] Update `main.go` commands
  - [ ] `bundle list` / `tree`
  - [ ] `folder list` / `tree`
  - [ ] `file list` / `tree`
  - [ ] `section list` / `tree`
  - [ ] `definition list`
  - [ ] `ticket list` / `tree`
  - [ ] `goal tree`
  - [ ] `policy list`
  - [ ] `contributor list`
- [ ] Verify functionality and tests

## Plan

1.  **Define ID Generation Logic**: Implement functions in `main.go` (or a helper) to generate IDs and Icons based on artifact properties.
2.  **Define Renderers**: Implement renderers for List (Human/MD) and Tree (Text/MD) that use the new ID generation logic.
3.  **Refactor Commands**: Update each command to use the unified ID system and renderers.
4.  **Update Tests**: Update `main_test.go` to match the new output formats.
5.  **Verify**: Run all tests.
