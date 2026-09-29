"""📸️ remodel (P2-X): every remodeling drift refusal (content handle incomplete, content leaf malformed / out of order /
conflicting / over capacity, still referenced) breaks a document rule → `Invariant` at its level; the TypeScript twin
drops the report text like the Rust record and moves to the frozen codes with its suite, fixtures, reference and feature."""
from jsargs import drop_argument

ANY = "📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/"
TWIN = ANY + "🧬️schema/🧬️mutations/🟦️.ts"
REFERENCE = ANY + "🧪️tests/📸️mutate-remodeling-1/🐍️.py"


def twin_without_text(text: str) -> str:
    """🗑️ The twin's report is `{severity, code, target}`: the text argument leaves every `noted`/`refuse` call."""
    if "  message: string;\n  target: string[];" not in text:
        return text
    text = text.replace("  message: string;\n  target: string[];", "  target: string[];")
    text = text.replace("code: string, message: string, target: string[] = []", "code: string, target: string[] = []")
    text = text.replace("messages: [...outcome.messages, { severity, code, message, target }],", "messages: [...outcome.messages, { severity, code, target }],")
    text, _ = drop_argument(text, "noted", 3)
    text, _ = drop_argument(text, "refuse", 2)
    return text


EDITS = [
    (TWIN, " *  first, some the invariant first) and that order is reproduced exactly, because it decides which\n *  message a refusal carries.",
     " *  first, some the invariant first) and that order is reproduced exactly, because it decides which\n *  report a refusal carries."),
    (REFERENCE,
     """    document stores complete, and only a reconstruction commit binds one (`mutation.incomplete-mesh`,
    `mutation.invalid-asset-payload`, `mutation.invalid-reconstruction-*`); a content leaf is bounded,
    contiguous and never rewritten (`mutation.invalid-content-chunk`, `mutation.content-*`).\"\"\"""",
     """    document stores complete, and only a reconstruction commit binds one; a content leaf is bounded,
    contiguous and never rewritten; a record something else still names is never deleted (all three
    `mutation.invariant`).\"\"\""""),
]

TRANSFORMS = [(TWIN, twin_without_text)]

RENAMES = [
    (ANY, {"mutation.invalid-reconstruction-sparse": "Invariant", "mutation.invalid-reconstruction-mesh": "Invariant",
           "mutation.invalid-reconstruction-asset": "Invariant", "mutation.invalid-content-chunk": "Invariant",
           "mutation.content-gap": "Invariant", "mutation.content-kind-mismatch": "Invariant", "mutation.content-conflict": "Invariant",
           "mutation.content-capacity": "Invariant", "mutation.referenced": "Invariant", "mutation.incomplete-mesh": "Invariant",
           "mutation.invalid-asset-payload": "Invariant"}),
]
