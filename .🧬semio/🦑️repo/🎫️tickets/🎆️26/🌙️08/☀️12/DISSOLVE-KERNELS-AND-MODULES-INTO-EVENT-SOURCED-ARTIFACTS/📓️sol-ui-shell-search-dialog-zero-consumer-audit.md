# UI Shell Search Dialog Zero-Consumer Audit

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Component SHA-256: `f0b1939a7bd2fc7a657c8c7de5248c487597d47633b07d5bd0b7c5b9bba30b4c`, dirty only from the accepted ShellFindDialog doc-referrer cleanup.
- Story SHA-256: `2412f921416afd201c01c6223f7a41e4460f79ca20f83d69c25ef0d9135175b2`, clean.
- React index at audit time: `a0331a0e40d7c2861f5e80304d359c67cbc57c823071862ab6e3572c72bf0ce2`.
- Storybook smoke spec after ShellFind cleanup: `033f9e508d157e9019317d757c87fe3b3a861a204c2aaf89b4632dcadd608484`.

No active production component imports or renders ShellSearchDialog or its `ShellCommandResult`/`ShellSearchDialogProps` contracts. The remaining closure is the implementation, exclusive story, mechanical barrel, one exclusive UI package minimal-render test, and three Storybook smoke IDs. Those are story/test/glue evidence and do not qualify as production consumers.

Decision: after the ShellFind lease releases its accepted doc update, delete ShellSearchDialog and its exclusive story, then remove its barrel/test and three Storybook inventory IDs in a serialized registrar. Do not create a shared row module, wrapper, alias, replacement, or compatibility export.
