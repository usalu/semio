import os

ART = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets"

SUBSET_VECTORS = {
    "document": [("rename-note", "🏷️rename-note", "retitles-the-document")],
    "canvas": [
        ("change-grid-visible", "👁️change-grid-visible", "hides-the-grid"),
        ("change-grid-spacing", "📏️change-grid-spacing", "widens-grid-spacing"),
        ("change-grid-subdivisions", "🔢️change-grid-subdivisions", "doubles-grid-subdivisions"),
        ("change-grid-opacity", "🌫️change-grid-opacity", "raises-grid-opacity"),
        ("change-snap-enabled", "🧲️change-snap-enabled", "enables-snap"),
        ("change-snap-grid-spacing", "📐️change-snap-grid-spacing", "halves-snap-grid-spacing"),
    ],
    "ink": [
        ("change-pencil-width", "✏️change-pencil-width", "thickens-pencil"),
        ("change-eraser-radius", "🧽️change-eraser-radius", "enlarges-eraser"),
        ("change-block-ink-width", "🖊️change-block-ink-width", "thickens-the-sketch-stroke"),
        ("edit-block-ink-stroke", "🎨️edit-block-ink-stroke", "redraws-the-sketch-polyline"),
    ],
    "asset": [
        ("create-asset", "🆕️create-asset", "adds-a-second-image-asset"),
        ("replace-asset-payload", "🔁️replace-asset-payload", "swaps-logo-payload-for-svg"),
        ("delete-asset", "🗑️delete-asset", "removes-the-logo-asset"),
    ],
    "block": [
        ("create-block", "➕️create-block", "inserts-a-photo-block-at-root-index-2"),
        ("delete-block", "❌️delete-block", "removes-the-math-block"),
        ("delete-blocks", "🧺️delete-blocks", "removes-the-ink-and-image-blocks"),
        ("duplicate-block", "🎯️duplicate-block", "copies-the-math-block-right-after-its-source"),
        ("duplicate-blocks", "👥️duplicate-blocks", "copies-ink-and-table-with-shifting-indices"),
        ("move-block-to-container", "🚚️move-block-to-container", "reparents-ink-into-the-callout-group"),
        ("drag-blocks", "🤏️drag-blocks", "nudges-ink-and-the-whole-group-subtree"),
        ("rename-block", "🔖️rename-block", "renames-the-table-block"),
        ("change-block-visible", "👀️change-block-visible", "hides-the-image-block"),
        ("change-block-locked", "🔒️change-block-locked", "locks-the-callout-group"),
        ("move-block", "📍️move-block", "repositions-the-math-block"),
        ("resize-block", "↔️resize-block", "enlarges-the-image-block"),
        ("change-block-font-size", "🔤️change-block-font-size", "enlarges-the-intro-font"),
    ],
    "text": [("edit-block-text", "📝️edit-block-text", "replaces-the-intro-paragraphs")],
    "math": [("edit-block-math", "🧮️edit-block-math", "replaces-the-tex-with-pythagoras")],
    "table": [
        ("insert-table-row", "⬇️insert-table-row", "appends-a-blank-third-row"),
        ("remove-table-row", "⬆️remove-table-row", "drops-the-trailing-blank-row"),
        ("insert-table-column", "➡️insert-table-column", "appends-the-lettered-column-c"),
        ("remove-table-column", "⬅️remove-table-column", "drops-the-trailing-column-b"),
    ],
}
SUBSET_ORDER = ["document", "canvas", "ink", "asset", "block", "text", "math", "table"]
assert sum(len(v) for v in SUBSET_VECTORS.values()) == 33

DESCRIPTIONS = {
    "document": (
        "This subset owns the one document-level identity field: `rename-note` sets `title` directly, "
        "the simplest of the nine document-level scalar setters rule 1 of the derivation rules "
        "describes."
    ),
    "canvas": (
        "This subset owns the six canvas-presentation scalars: grid visibility, spacing, subdivisions "
        "and opacity, plus snap-to-grid enablement and its own spacing. All six are document-level "
        "fields with no block addressing at all — rule 1's document-level scalar setters."
    ),
    "ink": (
        "This subset owns everything about how ink is DRAWN: `change-pencil-width` and "
        "`change-eraser-radius` are document-level tool scalars, while `change-block-ink-width` and "
        "`edit-block-ink-stroke` reach INSIDE a stroke block's own typed content — the polyline and "
        "its width — rather than moving the block at all."
    ),
    "asset": (
        "This subset owns the id-keyed asset table: `create-asset` and `delete-asset` add and remove "
        "a member outright — `delete-asset` removes the `assets` MEMBER entirely when it empties the "
        "table rather than leaving an empty map — and `replace-asset-payload` swaps one entry in place."
    ),
    "block": (
        "This subset owns the block TREE itself: creation, deletion (singular and bulk), duplication "
        "(singular and bulk), re-parenting, dragging a subtree, renaming, visibility, locking, "
        "absolute move, resize and font size. Blocks are id-keyed but their ORDER is the z-order the "
        "canvas paints in, so every tree inverse here restores POSITION and not merely membership: "
        "`delete-blocks` puts its blocks back in ASCENDING index order so each lands where it was, and "
        "`move-block-to-container` inverts to a re-parent back to the original container AT the "
        "original index. `drag-blocks` translates a named block and its whole SUBTREE.\n\n"
        "  📌️ A FINDING THE REFERENCE MADE WHILE IT WAS BEING WRITTEN. `duplicate-blocks` computes "
        "each copy's insertion index against the PRE-MUTATION list and does not re-base it as earlier "
        "copies land. Its committed vector duplicates `blk-ink` (root index 1) and `blk-table` (root "
        "index 2) in one mutation, and the committed after-snapshot orders the root `blk-text, "
        "blk-ink, blk-ink-copy, blk-table-COPY, blk-table, …` — the second copy lands BEFORE its own "
        "source, where the singular `duplicate-block` places its copy after. Both implementations "
        "reproduce the committed order; naming it here is what keeps it from passing as intended "
        "behaviour."
    ),
    "text": (
        "This subset owns `edit-block-text`, and it is the one kind BOTH implementations refuse by "
        "clause rather than work around. A text block does not hold its paragraphs: it holds a "
        "COMPOSED CHILD HANDLE `{childId, target}` into an `s.stdio.semio@v1/text` document, and the "
        "committed vector's whole observable effect is that handle's `childId` moving from "
        "`note-text-eea42a3b80b1052b` to `note-text-938222b3522927c6` — a content address of the "
        "child AFTER the new paragraphs are written, computed by a function no document in this "
        "repository states. `mutate-program-1` reports the identical blocker over "
        "`knowledge`/`benchmarks`, `mutate-block-3d-1` over `catalog`, and `mutate-en1990-1`'s two red "
        "scenarios are the same finding again."
    ),
    "math": (
        "This subset owns `edit-block-math`, which reaches INSIDE a math block's own typed content — "
        "its `tex` field — rather than moving the block at all."
    ),
    "table": (
        "This subset owns a table block's rows and columns: `insert-table-row`/`insert-table-column` "
        "append BLANK cells, which is what makes their inverses exact against the committed trailing-"
        "blank vectors, and `remove-table-row`/`remove-table-column` drop the trailing row or column."
    ),
}

for subset in SUBSET_ORDER:
    vectors = SUBSET_VECTORS[subset]
    kind_width = max(len(k) for k, _, _ in vectors + [("id", None, None)])
    vector_width = max(len(f"{d}/🧪️tests/{f}") for _, d, f in vectors)

    def row(k, d, f):
        vec = f"{d}/🧪️tests/{f}" if d else ""
        return f"      | {k.ljust(kind_width)} | {vec.ljust(vector_width)} |"

    header = f"      | {'id'.ljust(kind_width)} | {'vector'.ljust(vector_width)} |"
    mutate_rows = "\n".join(row(k, d, f) for k, d, f in vectors)
    inverse_rows = mutate_rows

    feature = f"""@capability-note-1-{subset}-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
@mutations-note-1-{subset}
Feature: Apply every typed note document {subset} mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-note-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds have a subset-owned test. The reference is `🐍️.py` in this directory:
  a second implementation of the `s.note.note` document and this subset's typed mutations, written in
  Python from `../../../✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2, 3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the committed vectors. It imports nothing from this repository's Rust. It carries the FULL
  document shape — not only this subset's own members — because every scenario validates the whole
  document, not merely the fields this subset's own kinds write.

  {DESCRIPTIONS[subset]}

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_note_mutation_outcome
      \"\"\"
      {{"kind": "<id>", "vector": "<vector>"}}
      \"\"\"
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
{header}
{mutate_rows}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_note_mutation_outcome
      \"\"\"
      {{"kind": "<id>", "vector": "<vector>"}}
      \"\"\"
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
{header}
{inverse_rows}
"""
    case_dir = f"{ART}/✳️{subset}/🧪️tests/mutate-note-1-{subset}"
    os.makedirs(case_dir, exist_ok=True)
    with open(f"{case_dir}/🥒️.feature", "w", encoding="utf-8") as fh:
        fh.write(feature)
    print("wrote", f"{case_dir}/🥒️.feature")
