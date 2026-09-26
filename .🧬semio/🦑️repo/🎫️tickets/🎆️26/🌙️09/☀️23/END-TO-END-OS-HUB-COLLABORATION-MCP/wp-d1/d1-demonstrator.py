"""🎪️ D1 codemod: agent-facing en/de descriptions for the demonstrator playground's own verbs."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

describe("✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_playground_editor", [
    ("changeSchema", "Replaces the playground's schema metadata string, the whole persistent content of a playground document, with the given text.", "Ersetzt die Schema-Metadaten-Zeichenkette des Playgrounds, den gesamten dauerhaften Inhalt eines Playground-Dokuments, durch den angegebenen Text."),
    ("setActiveExample", "Loads the bundled demo playground's schema, or clears the schema for an empty example id.", "Lädt das Schema des mitgelieferten Demo-Playgrounds oder leert das Schema bei leerer Beispiel-Id."),
], destructive=["changeSchema"])
