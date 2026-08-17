# Terra UI Shell Search Dialog Zero-Active-Consumer Dissolution Acceptance

## Scope And Ownership

Terra verified the leased source artifacts before deletion and removed only:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔍️ShellSearchDialog/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔍️ShellSearchDialog/🧪️story.tsx`

The verified pre-deletion SHA-256 values were:

- component: `f0b1939a7bd2fc7a657c8c7de5248c487597d47633b07d5bd0b7c5b9bba30b4c`
- story: `2412f921416afd201c01c6223f7a41e4460f79ca20f83d69c25ef0d9135175b2`

The existing component-only doc-reference cleanup was included in the accepted component hash. The registrar subsequently removed the exclusive barrel/test closure and the three Storybook smoke IDs. No protected renderer, manifest, lock, generated census, plugin, or unrelated UI behavior was edited by Terra.

## Final Structural Gates

The final active-source scans excluded `.git`, `.nx`, `node_modules`, `dist`, `target`, `storybook-static`, and this ticket tree. Each returned zero hits:

- case-insensitive `ShellSearchDialog` identity
- `ShellCommandResult`
- `ShellSearchDialogProps`
- JSX `<ShellSearchDialog …>` and closing-tag forms
- `🖱️ui⚛️react-shellsearchdialog--default`, `--filtered`, and `--empty`
- case-insensitive component/direct-path names `ShellSearchDialog` and `shell-search-dialog`

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔍️ShellSearchDialog` remains present and empty. The scoped `git diff --check` gate is clean.

The registrar-owned files remain exactly at their accepted post-registrar SHA-256 values:

- React index `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`
- Storybook smoke spec `.storybook/ui-new-stories.spec.ts`: `0ed906d63e572030e6615cad3b1d2867e3d4c697e6c4446d7167c0f080c94fd1`

## Nx Validation

Each required command was run exactly once after registrar completion:

| Command | Result | Evidence |
| --- | --- | --- |
| `bun nx run @semio-tech/ui-react:lint` | Pass | Nx completed successfully. |
| `bun nx run @semio-tech/ui-react:typecheck` | Blocked | Existing cross-workspace errors, including missing `PluginRegistryEntry`/`PluginSourceEvent`, statechart `eventCount` mismatches, plugin manifest contract errors, and unrelated UI barrel/type errors. No ShellSearchDialog stale reference was reported. |
| `bun nx run @semio-tech/ui-react:test-quick` | Blocked | 510 passed, 10 failed, and 2 unhandled errors. Failures cover gumball camera duck-typing, icon hover CSS, canvas pointer handling, panel spacing, tree rendering, and virtual file system rendering; none names or imports ShellSearchDialog. |
| `bun nx run @semio-tech/ui-react:build` | Blocked | Root Storybook cannot resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/🌳OntologyTree.stories.tsx`. |

## Conclusion

The zero-active-consumer dissolution is structurally accepted: its component, story, barrel/test closure, and three smoke IDs are absent; all final stale-reference/path/type/JSX/story-ID gates are zero; the accepted registrar files are unchanged; and the scoped diff is clean. The non-lint Nx failures are unrelated active workspace failures and were not retried or repaired under this packet.
