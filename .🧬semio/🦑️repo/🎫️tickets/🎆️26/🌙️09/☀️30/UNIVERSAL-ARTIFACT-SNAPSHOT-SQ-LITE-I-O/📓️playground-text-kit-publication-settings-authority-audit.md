# Playground Text Kit Publication Settings Authority

Read-only current source audit, no execution. Editor main render28 accepts UiPublicationRevision and forwards it to TextEditView. Plugin render_editable36428 forwards it to TextDraftView; render_draft36465 encodes TextDraftSettings.publication_revision as its full u64 decimal String. TextEditorScene1728 has settings_json, no publication_revision field. Canonical app-window-kits Native leaf116 already asserts settings["publicationRevision"] against u64::MAX.to_string().

Exact editor test pairing: replace invalid scene.publication_revision assertion17 by settings["publicationRevision"].as_str()==Some("37") immediately after existing parsed settings binding19. Retain full text/language/readOnly/commit/action assertions and the independently supplied render argument37. This preserves publication authority through its actual settings carrier.

Viewer test19 incorrectly demands the same nonexistent field despite viewer render(document) receiving no revision. Its already present exact readonly settings assertion21 ({"readOnly":true}) is the actual public readonly contract. Do not invent viewer revision input or mirror field. Both paths are canonical compiler prerequisites, not Snapshot SQL functionality or runtime credit.
