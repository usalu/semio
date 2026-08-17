# UI Shell Find Dialog React Index Registrar Acceptance

- React index pre-edit SHA-256: `50c0bcd05afc285101da820bb3fcae8dd0d8cf8046e64cacdf9dcfce1c6b859f`.
- Terra confirmed the component/story deleted and the ShellSearchDialog doc referrer updated without touching the index.

The coordinator removed only the ShellFindDialog import/re-export region, its exclusive minimal-render describe block, and the two deleted-story IDs from the existing Storybook smoke inventory. The neighboring ShellSearchDialog smoke test and story IDs remain intact.

Evidence:

- React index post-edit SHA-256: `a0331a0e40d7c2861f5e80304d359c67cbc57c823071862ab6e3572c72bf0ce2`.
- Storybook smoke spec post-edit SHA-256: `033f9e508d157e9019317d757c87fe3b3a861a204c2aaf89b4632dcadd608484`.
- Index scan for direct path, `ShellFindDialog`, `ShellFindDialogProps`, and JSX: zero matches.
- Scoped ordinary and cached `git diff --check`: pass.

Final active-source scans and Nx validation remain Terra-owned.
