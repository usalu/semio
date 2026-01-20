# Plan

- [x] Review the existing codebase and identify the issues.
- [x] Fix Diagnostics: Ensure `repo analyze` works correctly in the extension.
    - [x] Fix `Violation` interface mismatch (object vs string).
    - [x] Ensure `repo analyze` is called correctly.
- [x] Fix Autofixes: Ensure `repo fix` works correctly in the extension.
    - [x] Fix `RepoCodeActionProvider`.
- [x] Fix Tree Views: Investigate why tree items are not loading.
    - [x] Verify `repo` binary provides data.
    - [x] Fix data parsing/handling in `extension.ts` and `repo` binary.
- [x] Align with `repo` binary: Ensure the extension strictly uses the `repo` binary as the single source of truth for all data and operations.
- [x] Document changes.
