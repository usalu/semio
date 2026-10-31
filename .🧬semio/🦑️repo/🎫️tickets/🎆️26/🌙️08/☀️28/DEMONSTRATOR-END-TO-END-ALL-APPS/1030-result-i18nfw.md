# 1030 result i18nfw
STATUS: PARTIAL-DONE. (1) no implicit English and (3) brand en+de are done and tested; (2) is done for `IntroductionInteraction.label` only. `ArtifactPresentation.name`, `MediaPortSpec.label` and `WindowLayoutWindowNode.title` are deferred, with the reasons below.

## 1. Locale resolution without a default language
- Engine `🖱️ui/🌐️i18n/🔡️instance/🟦️.ts`: `fallbackLanguage` and `language` are now optional. An instance with no language is undecided and answers every key with the key itself. With no fallback it never answers in a language nobody chose.
- Port `🌐️i18n/🟦️.ts`:
  - The shared options no longer carry a fallback.
  - `detectShellLocale(tag)` returns `ShellLocale | undefined`. `de-*` and `en-*` map; `fr` and `""` give `undefined`.
  - New `requireShellLocale(language)` throws for an undecided or unsupported language. `UIIntroduction` and `UIDialog` use it.
  - `resolveRequestedUiLocale` chain: stored choice, then shared instance, then platform language. It can end undecided.
- `ShellHost/🟦️.tsx`: `FrameworkOsShell` resolves lock, then stored preference, then host option, then platform language.
  - If none of these yields a locale it renders the new `🌐️locale-picker` (English and Deutsch buttons, each in its own `lang`, nothing preselected).
  - The choice is persisted with `setLocale`, then the shell mounts as `FrameworkOsShellMounted`.
- Other implicit-English spots removed:
  - `currentShellLocaleV1` throws instead of returning `SHELL_LOCALES[0]`.
  - `frozenLabelText` throws for an unsupported locale.
  - `resolveUiLocalizedText` no longer falls back to `en` or any other cell.
  - The `?? "en"` in ChromePanels is gone.
- Rust: the wgpu host already returned an `Option` for the host locale and `shell_language_axes` returns `Result`. Nothing to change.
- Left alone:
  - `World3dHost/📏️modelling` still selects English for an unknown locale. Its own test asserts that default, and it can run without a shell scope.
  - `🔒️boot-and-locks` passes `locale: "en"` to `resolveUiPreferences`, but only to read `customThemes`.

## 2. Plugin manifest schema
- Rust: `IntroductionInteraction.label` is now a `LocalizedLabel`, and all eight constructors take `impl Into<LocalizedLabel>`.
- New `LocalizedLabel::has_blank_cell()`. The `try-build-definition` validation now refuses any blank (terminology x locale) cell.
- The wgpu tour overlay resolves the label with the active terminology and locale.
- The TS projection (`rows-h-t`) now has `label: unknown`, the same as the step `title`. `resolveIntroductionDefinition` and `UIIntroduction` resolve the label through the manifest or UI label resolver.
- Manifests migrated: the puzzle3d editor (Zoom/Zoomen, Pan/Verschieben, Orbit/Orbitieren, Add an object/Objekt hinzufügen, Activate Transform/Transformieren aktivieren). Test constructors in onboarding, plugin builder and the two wgpu tour tests were ported.
- Generated descriptors and composed manifest JSON were not hand-edited. They regenerate on the coordinator's next activation.
- The coordinator reported `has_blank_cell` missing. At my last check the definition (`🏷️label/🦀️.rs`) and the caller were both present.

### Deferred
- `ArtifactPresentation.name`:
  - It has about 45 literal sites across all plugins, including ◻️2d (puzzle2d, fem2d, block2d, generation2d). I was told to avoid that tree, and a type change would break those crates' builds.
  - Nothing renders the field today.
  - Do it as one atomic sweep with `LocalizedLabel::native(en, de)` once the ◻️2d migration lands.
- `MediaPortSpec.label`:
  - Besides the ~35 plugin sites, the type derives `CanonicalJsonTree`.
  - `WorkflowMediaPort` persists the label as `Shape::Text` in the workflow record and sqlite snapshot encoders.
  - So changing the type changes a stored format.
  - Proposed design: workflow ports store the port id only, and the display label is resolved from the manifest.
- `WindowLayoutWindowNode.title` (two structs: ui layout-contract and tui layout): the renderer overrides it with the window-kind label, so it is not user-visible.

## 3. Brand text in en+de
- `🪧️brand.ts` has no plain-language literal left in `title`, `body`, `label`, `description`, `text` or `tagline`. 97 distinct texts moved into the demonstrator catalog as `demonstrator.brand.*` with an English and a German cell:
  - the landing introduction;
  - the Aggregator tutorial (title, description, chapters, narration);
  - the eight brand introductions with their interaction labels;
  - the pane label, tagline and description.
- Manifest labels use the new `demonstratorLabel(key)`, which returns the `{native, reuse}` x `{en, de}` wire shape. Pane texts use `demonstratorText(key, DEMONSTRATOR_LOCALE)`.
- Brand keys are excluded from the UI translation tree because the shell resolves them as manifest labels.
- The German lock (`lockedLocale`) is unchanged. The English text is shipped but not served until the lock is lifted.
- Left German-only: the proper-noun window titles and the logo `aria-label`.

## Tests
- `🧪️tests/🧪️demonstratorlocale` (shared `🧫️cases.json`, TS and Py), extended with:
  - interaction labels in plugin sources are `LocalizedLabel::native` pairs and never plain strings (5 found, none plain);
  - brand sources: no literal fields, every used key exists in the catalog, no unused brand key, every text translated except the identical names `Generator` and `Aggregator`;
  - framework sources: no fallback language, no `?? "en"`, required schema shape.
- Engine corpus `🔡️instance/🧪️tests/🔣️.json` gained two rows (no fallback; undecided language). The i18next oracle agrees; `fallbackLanguage` and `language` are optional in its schema. `bun test`: 21/21.
- `owned-locale-detector-retirement` test updated for `undefined`. Not run.
- `repo test @semio-tech/mit-bestand-demonstrator`: 80/80. This includes the new tests and the updated `shell-brand` test, which now compares the German cell.
- Python twin: 11 tests OK, 1 skipped (`jsonschema` absent).
- Cargo: `check -p semio-framework-plugin-app-builder -p semio-framework-plugin --lib --keep-going` exited 0 (7m51s).
- Not compiled:
  - the puzzle3d editor (package not in the workspace graph);
  - the onboarding tests and the wgpu shell tests (kernel crates, not workspace members).
  These edits were reviewed by hand.
- Not run: ui vitest (the `ShellHost` picker, the introduction component).

## Follow-up (coordinator round 2)
- The `ArtifactPresentation.name` / `MediaPortSpec.label` sweep was NOT done. fwfix has not reported ◻️2d green (the ◻️2d migration is still in flight), and those types are used by the ◻️2d plugin crates (puzzle2d, fem2d, block2d, generation2d). A type change would break them.
  - Nothing else blocks it. After fwfix goes green, do one atomic sweep: struct change plus about 45 `ArtifactPresentation` literals, about 35 `MediaPortSpec` literals, the workflow-store record encoders, sqlite snapshot and borrowed-record specs, then regenerate descriptors.
- The demonstrator tsc errors (34 in `🪧️brand.ts`) come from the stale gitignored `framework-manifest/🤖️generated/🪪️manifest/🟦️.ts`, which still types `IntroductionInteraction.label: string`. I changed the Rust projection (`rows-h-t`) to `label: unknown`.
  - Regenerating needs `@semio-tech/framework-rs:generate`. It currently fails in a foreign crate: `semio-framework-value-retirement` (`🌱️value/📦️box`: `RetirementStep::Failure` / `ChildProgress` now take an extra `u64`). After that fix, rerun the generate; tsc should then be 0 for those 34. I did not hand-edit the generated file.
  - The one `VirtualFileSystem/🟦️.tsx(568)` error is not from my lane.
- `@semio-tech/ui-react:test` cannot start. `repoToolCacheEnv` in its `📜️script.ts:25` throws `JSON Parse error: Unrecognized token '/'` inside `loadScriptProviderSyncV1`.
  - The demonstrator test target runs fine, so this is a repo-tooling or provider-designation fault, not the i18n change.
  - So the picker, introduction and locale-detector vitest suites are still not run. Direct vitest needs the explicit policy env.
- The puzzle3d editor package is not in the root workspace graph (`package ID specification did not match`). Its `workspace` points at `✏️s/Cargo.toml`, and `--manifest-path ✏️s/Cargo.toml` also did not match. I could not compile it; the onboarding and wgpu tests are non-member kernel crates. The edits are hand-reviewed only.
