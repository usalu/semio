# Direct ArtifactApp Proof Parser

## Result

The official tool-job verifier now recognizes both supported runtime owner shapes:

- wrapped editor owners declared by `impl ArtifactEditor for T` and `type Owner = EditorApp<T>`;
- hand-written direct owners declared by `impl ArtifactApp for T` and `type Owner = T`.

Owner-file, document-schema, factory, registration, builder, and exact migrated-row checks remain fail-closed. A hostile direct-owner fixture that substitutes `ForgedDirectOwner` is rejected.

## Verification

`bun ./📜️script.ts verify interactivity tool-jobs --self-test` exited successfully with `self-tests=468 clean`, and `git diff --check -- 📜️script.ts` was clean.
