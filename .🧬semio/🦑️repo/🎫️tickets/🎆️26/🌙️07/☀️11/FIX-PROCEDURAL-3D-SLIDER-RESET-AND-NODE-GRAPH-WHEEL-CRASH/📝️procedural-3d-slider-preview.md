# Procedural 3D Slider Preview

## Report

Moving the demonstrator Generator height slider appeared to leave the column preview at its initial height of `6`.

## Diagnosis

The current procedural evaluation path does invalidate and retessellate after a widget mutation. Browser inspection changed the height slider from `6` to `10`; the emitted preview mesh payload changed and its maximum Z coordinate changed from `6` to `10`.

The demonstrator had a separate runtime/build inconsistency. Its pane metadata names the branded variant `generator`, while the React pane deliberately boots `procedural3d`. The build script and Vite asset selection only used the branded variant, so they built and packaged `demonstrator` while the Generator requested the independently staged `procedural` module. That allowed an old procedural WebAssembly artifact to remain in use across demonstrator rebuilds.

## Resolution

- Added one `demonstratorPaneRuntimeVariant` mapping and reused it for React boot, plugin build selection, and Vite production asset selection.
- Made the demonstrator build explicitly rebuild the procedural plugin used by Generator and restore the primary demonstrator registry session afterward.
- Included both branded and resolved runtime variants in the production plugin/module asset union.
- Added a procedural regression test that evaluates the initial flow, applies a height widget mutation, reevaluates, and asserts that both evaluation JSON and tessellated preview meshes change.

## Validation

- Live browser interaction: height `6` → `10`; preview mesh maximum Z `6` → `10`; mesh payload changed.
- Live rendering: screenshots at height `10` and `0` differed.
- `git diff --check` completed without errors for all touched implementation and test files.
- `SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1 bun nx build @semio-tech/mit-bestand-demonstrator` passed registry/script loading, then failed in an unrelated shared file, `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts:589`, because Node strip-only TypeScript does not support its existing constructor parameter property.
- `bun nx test @semio-tech/procedural-plugin` did not produce a test result: the repository budget stopped Cargo after 1,200,000 ms while it was waiting on shared target-directory build contention.
- A full plugin-building demonstrator validation was started to confirm the new build path, but remained behind the same shared Cargo work and was stopped without a result.
