"""🧪️ Verb laws: `declare_kinds` names a taxonomy row whose edit lacks a rule instead of crashing."""

import importlib.util
from pathlib import Path

import pytest

HERE = Path(__file__).resolve().parent.parent


def verbs():
    """🔧️ Import the document and verbs modules by path under the names the execution oracle registers."""
    import sys

    for alias, folder in (("semio_norm_vocabulary_document", "📄️document"), ("semio_norm_vocabulary_verbs", "🔧️verbs")):
        spec = importlib.util.spec_from_file_location(alias, HERE.parent / folder / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[alias] = module
        spec.loader.exec_module(module)
    return sys.modules["semio_norm_vocabulary_verbs"]


def test_row_edit_without_rule_is_named():
    module = verbs()
    row = {"artifact": "🏋️en1991", "kind": "insert-fatigue-model", "edit": {"path": "fatigueModels", "item": "fatigue-model"}}
    with pytest.raises(ValueError, match="insert-fatigue-model"):
        module.declare_kinds([row])


def test_valid_rows_are_declared():
    module = verbs()
    module.declare_kinds([
        {"artifact": "x", "kind": "change-a", "edit": {"rule": "set-field", "path": "a[].b", "selectors": []}},
        {"artifact": "x", "kind": "insert-a", "edit": {"rule": "insert-item", "path": "a", "selectors": []}},
        {"artifact": "x", "kind": "plain"},
    ])
    assert set(module.SETS) == {"change-a"} and set(module.DECLARED) == {"insert-a"}
