# DOCX Run Formatting Controls — 2026-10-03

## Scope

The base, strict, and transitional DOCX document editors expose the existing canonical `SetRunFormatting` mutation as an end-user action named `set-run-formatting`. The action carries the complete canonical XML address plus required `bold`, `italic`, and `underline` booleans.

The text action continues to use a local draft. Existing WordprocessingML runs additionally receive three accessible toggle controls. Labels are localized as Bold/Fett, Italic/Kursiv, and Underline/Unterstrichen. Strict and transitional windows use their own controller IDs while sharing the canonical base renderer and action definition.

Empty paragraphs use the new canonical text-target seam and render a text draft. They do not render run-format controls until the first text edit creates a real run.

`DocumentWindowKit::render_editable_windowed_with_accessory` renders a domain-owned accessory from inside the page row's windowed content closure. Collapsed hosted drafts therefore build neither their text editor nor their toolbar. The DOCX renderer moves each projected text buffer into the shared view once and retains only the address and three formatting bits beside it; accessory lookup is by the shared logical ordinal and does not scan the page collection.

Every base, strict, and transitional editor now starts with one canonical empty paragraph, so a newly created document immediately has one editable target. Base and transitional construction use the canonical minimal serializer. Strict construction uses `DocxStrictBuilderConstruction`, which authors the strict WordprocessingML namespace, conformance marker, and office-document relationship rather than rewriting a transitional package.

## Concurrency and history

Action decoding requires exactly one complete address object. Formatting execution calls `prepare_addressed_xml_mutation`, so the expected expanded element name and revision must still match before a mutation can publish. An unchanged formatting request is elided. A stale address produces a fault and leaves the snapshot unchanged.

The registered retained command work handles both text and formatting tools. Text continues to page copied input. Formatting validates the address and publishes at most one canonical mutation.

Direct run formatting is tri-state. `None` means the run has no direct property and may inherit from a paragraph or character style; `Some(true)` and `Some(false)` are explicit direct overrides. An explicit off request against an absent property is therefore a real mutation. Only an already identical explicit triple is elided. The UI exposes localized accessible status labels for inherited/absent, direct on, and direct off states instead of presenting absence as an effective false value.

## Neutral fixture and native laws

The language-neutral action fixture is `✏️editor/🧫️fixtures/🎨️run-formatting-action/🔣️.json`. It declares the action, EN/DE labels, schema fields, and one state transition. `✏️editor/🧫️fixtures/🌱️initial-document/🔣️.json` declares the single empty paragraph and editable target expected from a new editor document.

Unit laws cover:

- schema/action field parity;
- localized accessible toggle labels and checked state;
- exact controller action plus canonical address and revision arguments for every toggle;
- hosted collapsed rows omit the accessory and hosted expanded rows materialize it;
- no formatting toolbar for an empty paragraph;
- base, strict, and transitional initial snapshots render one empty-paragraph target, with strict conformance validated independently;
- exact base, strict, and transitional command decoding;
- explicit-off behavior over an inherited bold style, identical-direct no-op elision, and stale-revision rejection;
- registered base action publication, DOCX save/reopen, undo, redo, and stale-address refusal.

## Validation

- `rustfmt --emit stdout` parsed the shared `DocumentWindowKit` source and the independent QuickXML oracle source.
- Scoped `git diff --check` for the shared plugin framework source and DOCX tree: passed. The repository-wide check still reports pre-existing whitespace in unrelated concurrent files.
- Native DOCX run 15 reached compilation in 36.5 seconds and exposed only new-test integration errors: a fixture relative path, a non-dependency UI import, owned snapshot borrow sites, and QuickXML string-name comparisons. All diagnosed errors were corrected.
- Native run 16 reached runtime and exposed three formatting integration failures: missing interactive-job classification, missing rendered accessibility text, and an incorrect absent-as-false no-op assumption. The action is now classified, toggles expose accessible text, and the public projection/laws preserve tri-state direct formatting.
- Native run 17 reached compilation and exposed only QuickXML 0.42 string/value API mismatches in the new independent inherited-style oracle. The root coordinator repaired those comparisons and queued run 18. No run-18 result is claimed here.
