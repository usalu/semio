# UI Shell Search Dialog Registrar Acceptance

- React index pre-edit SHA-256: `a0331a0e40d7c2861f5e80304d359c67cbc57c823071862ab6e3572c72bf0ce2`.
- Storybook smoke spec pre-edit SHA-256: `033f9e508d157e9019317d757c87fe3b3a861a204c2aaf89b4632dcadd608484`.
- Terra confirmed component/story absent and authored directory empty.

The coordinator removed the ShellSearchDialog import/re-export region, exclusive minimal-render describe block, and three deleted-story IDs. No neighboring command behavior, test, or Storybook ID changed.

Evidence:

- React index post-edit SHA-256: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.
- Storybook smoke spec post-edit SHA-256: `0ed906d63e572030e6615cad3b1d2867e3d4c697e6c4446d7167c0f080c94fd1`.
- Registrar scan for case-insensitive ShellSearchDialog identity, direct path, and `ShellCommandResult`: zero matches.
- Scoped ordinary and cached `git diff --check`: pass.

Final active-source scans and UI Nx validation remain Terra-owned.
