"""🧪️ Unit tests of the norm step layer; `numpy` and `jsonschema` are the independent third-party oracles for the tolerance rule and the grammar schema.

Run: `.venv/bin/python -I <this file>`.
"""

import importlib.util
import json
import os
import re
import sys
import unittest

import jsonschema
import numpy

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("norm_steps", os.path.join(HERE, "..", "..", "🐍️.py"))
steps = importlib.util.module_from_spec(spec)
sys.modules["norm_steps"] = steps
spec.loader.exec_module(steps)


class Grammar(unittest.TestCase):
    def test_grammar_validates_against_its_own_schema(self):
        with open(steps.GRAMMAR_PATH, encoding="utf8") as handle:
            document = json.load(handle)
        schema = {key: document[key] for key in ("type", "required", "properties")}
        jsonschema.validate(document, schema)

    def test_every_template_matches_a_regex_built_from_it(self):
        for name, family in steps.grammar().items():
            for entry in family["steps"]:
                template = entry["template"]
                pattern = "^" + re.sub(r"\\\{(\w+)\\\}", r"(?P<\1>.+?)", re.escape(template)) + "$"
                sample = re.sub(r"\{(\w+)\}", lambda hit: "v_" + hit.group(1), template)
                ours = steps.match_template(template, sample)
                theirs = re.match(pattern, sample).groupdict()
                self.assertEqual(ours, theirs, name)

    def test_families_are_unambiguous(self):
        for name, family in steps.grammar().items():
            texts = [{"text": re.sub(r"\{(\w+)\}", "x", entry["template"])} for entry in family["steps"]]
            self.assertEqual(steps.parse_steps(texts)[0], name)

    def test_unknown_wording_is_refused(self):
        with self.assertRaises(AssertionError):
            steps.parse_steps([{"text": "something else"}])


class Tolerance(unittest.TestCase):
    def test_within_agrees_with_numpy_isclose(self):
        for actual, expected, tolerance in [(8.9e6, 8.935e6, 1e-3), (1.0, 1.0001, 1e-3), (0.0, 0.0, 1e-9), (5.0, 5.2, 1e-2), (1e-15, 0.0, 1e-9)]:
            reference = bool(numpy.isclose(actual, expected, rtol=tolerance, atol=1e-12))
            self.assertEqual(steps.within(actual, expected, tolerance), reference, (actual, expected))

    def test_inputs_split_numbers_and_text(self):
        self.assertEqual(steps.parse_inputs("fb_pa=20e6;material:clay;gamma_m=1.5"), ({"fb_pa": 20e6, "gamma_m": 1.5}, {"material": "clay"}))


class Projection(unittest.TestCase):
    def test_projection_has_exactly_the_grammar_keys(self):
        self.assertEqual(list(steps.projection("clause-vector", vector="E01", quantity="chi_z", value=0.5)), ["vector", "quantity", "value"])
        with self.assertRaises(AssertionError):
            steps.projection("clause-vector", vector="E01")


class Outcome(unittest.TestCase):
    def test_exact_row_wins_and_a_family_is_judged_by_its_worst_row(self):
        rows = {"a.wall": ("pass", 0.4), "b.wall.lc1": ("pass", 0.5), "b.wall.lc2": ("fail", 1.2), "b.wallet": ("pass", 9.0)}
        self.assertEqual(steps.check_outcome(rows, "a.wall"), {"status": "pass", "utilisation": 0.4})
        self.assertEqual(steps.check_outcome(rows, "b.wall"), {"status": "fail", "utilisation": 1.2})
        with self.assertRaises(AssertionError):
            steps.check_outcome(rows, "c.wall")

    def test_pass_or_fail_splits_at_one(self):
        self.assertEqual([steps.pass_or_fail(value) for value in (0.0, 1.0, 1.0001)], ["pass", "pass", "fail"])


if __name__ == "__main__":
    unittest.main()
