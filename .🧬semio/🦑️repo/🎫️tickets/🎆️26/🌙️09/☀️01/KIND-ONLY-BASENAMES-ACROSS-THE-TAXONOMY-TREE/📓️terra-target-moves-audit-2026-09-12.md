# Target-Move Closure Audit

Date: 2026-09-12

## Scope And Method

This read-only independent audit examined the completed UI WGPU/TUI move and the target-first UI React, OS renderer React, puzzle 5D React, OS renderer WGPU bridge, services, and Flow WASM packets. It inspected the live tree and `git diff HEAD`, resolved physical Rust and TypeScript referents, loaded the live taxonomy, checked workspace resolution, and ran only the focused UI React package-export test. Other agents were editing and staging files during the audit; no source, Git, ticket, or goal state was changed here.

## Verified Closure

- `git diff HEAD --find-renames=85%` identifies 31 UI WGPU source rename pairs from `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` into `🖱️ui/🎯️targets/🧊️wgpu/<semantic-domain>/🦀️.rs`. The prior `🔁️reconcile.rs` is now `🔀️reconcile/🦀️.rs`.
- The canonical TUI leaf exists at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⌨️tui/🦀️.rs`; the former `📦️packages/🦀️rust/🎯️targets` tree is absent.
- A local resolver checked all 34 Rust files in the UI package root, WGPU target, and TUI target. Every `#[path]`, `include_str!`, and `include_bytes!` referent exists. The check found zero missing referents.
- The WGPU target has 30 direct semantic domain directories, and every one is registered in `members-of-wgpu-target`. The schema also admits target families owned by other WGPU implementations, so its extra registered names are not a UI discrepancy.
- All three relocated React package entry leaves resolve physically to the target-owned authored leaves. `Bun.resolveSync` resolves `@semio-tech/ui-react`, `@semio-tech/framework-renderer-react`, and `@semio-tech/puzzle-5d-react` through root workspace links to their target-first package roots.
- `loadCatalogTaxonomy()` completed. The live readme/license owner-authority catalog SHA-256 is `b60f16acbde34545d29abfeb50a55d5db6115b2d316b33011db07da34ca7afe9`, exactly matching the live taxonomy pin.
- `bun 🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts test <ui-react-package-export-test> --run --reporter=dot` passed: one file and one test. This independently confirms the UI package entry and self-alias use the canonical target-first source.

The rename comparison found nine WGPU leaves byte-identical to `HEAD`; the other rename pairs contain expected mount/referent rebases or concurrent live edits. The referent check above verifies current resolution, but this audit does not claim that every moved source body is byte-identical.

## Concrete Closure Finding

`✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/package.json` has no `semio` role/id marker. `discoverPackages(process.cwd(), loadCatalogTaxonomy())` reports 220 packages but no row for this active target package. Its predecessor manifest at `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/package.json` also lacked the marker, so the relocation did not introduce the omission; nevertheless, the moved package is not discoverable and needs a schema-valid marker repair.

The same package has a live empty `🧪️tests` directory at `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/🧪️tests`. It contains no non-dependency files and was absent from the predecessor's `HEAD` file tree. The directory is sufficient for the strict package discovery to report an invalid package subdirectory. Its creator should be checked before removing this empty artifact.

## Remaining Scope, Not A Regression Assignment

The OS WGPU Rust package retains `📦️packages/🦀️rust/🟦️typescript/📚️library/🟦️.ts`. It was already present in `HEAD` (last path commit `fe7c8a8`, 2026-09-05), has no current diff, exports only the canonical `🎬️renderer-boot/🟦️.ts`, and is consumed by the OS dev Vite alias and Rust package export. The current package classifier correctly reports it as an invalid nested package/language topology. It must move in the next WGPU package-glue/source packet, with its exports and Vite consumer rewired; this audit makes no claim that the completed lane introduced it.

The current classifier also reports configuration-body diagnostics for relocated React package `vitest.config.ts`/`postcss.config.ts` leaves and pre-existing Flow WASM JavaScript package bodies. Those are separate configuration/package-body work, not automatic source moves in this bounded target audit.

## Limits

The audit deliberately did not repeat the broad UI WGPU suite or attribute its known hostile-fixture assertion to the taxonomy move. It did not run package-body remediation or change live schema, manifests, lockfiles, generated assets, or source references.
