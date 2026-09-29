# Example Switch

The navbar lists a subset's registered examples on every editor, and choosing one loads that document.

## Picker

`appOffersRegisteredExamples` shows the dropdown when the focused app is an editor and the dialect catalogue is non-empty. A missing `setActiveExample` declaration no longer hides it. `#lowpoly` registers `hexagonal-cut-concrete-forest-left` and is an editor, so the control is required.

A viewer still needs the action. It does not own the editor's example bodies.

`frameworkOwnsExampleSwitch` lets that undeclared editor dispatch through. The shell no longer drops it as an undeclared action.

## Load

`EditorApp` answers `catalogue_example_document` from `ArtifactEditor::examples`. An empty id loads `initial_snapshot`. A `{` body is the JSON a deferred producer emits; any other body is the subset DSL.

`dispatch_action` uses that body only when the registry has no `setActiveExample`. An editor that already handles the verb keeps its own command, including an unknown-id fault.

The load is `Effect::LoadDocument` with an edit-free `.spr`. `dispatch_emit` stamps the live document identity, same as a hand-written example command.

## Still open

Committed descriptors are unchanged. The shell no longer reads the action out of them to decide visibility. Production `play.semio-tech.com` shows this only after the renderer and the guest are rebuilt.

Subsets that register no examples still have no rows. Registration stays `ArtifactEditor::examples` on the subset.

## Subset catalogue

`project_artifact_declarations` remembers each subset's `examples` by dialect. An editor whose own `examples()` is empty loads that catalogue on `setActiveExample`. Sequence's `demo` is registered on the subset and already published in its descriptor, so the navbar can list it and the guest can open it without a hand-written editor method.

`subset_examples_reach_the_editor_catalogue_without_an_editor_method` passed.
