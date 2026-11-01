"""🌐️ Python twin of 🟦️.ts: every user-visible text of the demonstrator closure exists in English AND German, driven by the same 🧫️cases.json."""

import json
import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
CASES = json.loads((HERE / "🧫️cases.json").read_text(encoding="utf8"))
LOCALES = ("en", "de")


def read(path):
    return (ROOT / path).read_text(encoding="utf8")


def read_json(path):
    return json.loads(read(path))


def placeholders(text, pattern):
    return sorted(re.findall(pattern, text))


def rust_sources(directory, skip):
    for path in sorted(directory.rglob("*.rs")):
        if not any(part in skip for part in path.relative_to(directory).parts[:-1]):
            yield path


class DemonstratorLocale(unittest.TestCase):
    def setUp(self):
        self.catalog = read_json(CASES["catalog"])

    def test_catalog_schema_of_record(self):
        try:
            import jsonschema
        except ImportError:
            self.skipTest("jsonschema oracle not installed")
        schema = read_json(CASES["schema"])
        jsonschema.validate(self.catalog, schema)
        key, cells = next(iter(self.catalog["texts"].items()))
        for hostile in ({"en": cells["en"]}, {"en": cells["en"], "de": "  "}):
            with self.assertRaises(jsonschema.ValidationError):
                jsonschema.validate({**self.catalog, "texts": {**self.catalog["texts"], key: hostile}}, schema)

    def test_every_key_in_every_locale_with_same_placeholders(self):
        self.assertEqual(self.catalog["locales"], list(LOCALES))
        for key, cells in self.catalog["texts"].items():
            self.assertEqual(sorted(cells), sorted(LOCALES), key)
            for locale in LOCALES:
                self.assertTrue(cells[locale].strip(), f"{key} {locale}")
            self.assertEqual(placeholders(cells["en"], r"\{\{(\w+)\}\}"), placeholders(cells["de"], r"\{\{(\w+)\}\}"), key)

    def test_labels_pair_normal_and_beginner(self):
        keys = set(self.catalog["texts"])
        for key in keys:
            if key.endswith(".label.normal"):
                self.assertIn(key[: -len("normal")] + "beginner", keys, key)
            if key.endswith(".label.beginner"):
                self.assertIn(key[: -len("beginner")] + "normal", keys, key)

    def test_no_locale_is_locked_and_none_is_declared_statically(self):
        self.assertNotIn("lockedLocale", self.catalog)
        html = read(CASES["html"])
        free = CASES["lockFreeSources"]
        self.assertIsNone(re.search(free["html"]["forbidden"], html))
        self.assertIn(free["html"]["required"], html)
        for path in free["paths"]:
            for rule in free["forbidden"]:
                found = [match.group(0) for match in re.finditer(rule["pattern"], read(path))]
                self.assertEqual(found, [], f"{path} {rule['id']}")

    def test_viewer_language_is_the_first_supported_candidate_otherwise_ask(self):
        for row in CASES["localeResolution"]:
            expected = next((tag.lower().split("-")[0] for tag in row["candidates"] if tag and tag.lower().split("-")[0] in LOCALES), None)
            self.assertEqual(expected, row["expected"], row["id"])

    def test_components_hold_no_hardcoded_literal_or_locale_ternary(self):
        for path in CASES["componentSources"]:
            for rule in CASES["forbiddenSourcePatterns"]:
                found = [match.group(0) for match in re.finditer(rule["pattern"], read(path))]
                self.assertEqual(found, [], f"{path} {rule['id']}")

    def test_composed_manifests_name_every_label_in_both_locales(self):
        for manifest in CASES["composedManifests"]:
            cells = 0
            bad = []

            def visit(node, path):
                nonlocal cells
                if isinstance(node, list):
                    for index, child in enumerate(node):
                        visit(child, f"{path}/{index}")
                elif isinstance(node, dict):
                    keys = ",".join(sorted(node))
                    if keys == "native,reuse":
                        for terminology in ("native", "reuse"):
                            row = node[terminology]
                            for locale in LOCALES:
                                cells += 1
                                if not isinstance(row, dict) or len(row) != 2 or not isinstance(row.get(locale), str) or not row[locale].strip():
                                    bad.append(f"{path}/{terminology}/{locale}")
                    elif keys == "de,en":
                        for locale in LOCALES:
                            cells += 1
                            if not isinstance(node[locale], str) or not node[locale].strip():
                                bad.append(f"{path}/{locale}")
                    else:
                        for name, child in node.items():
                            visit(child, f"{path}/{name}")

            visit(read_json(manifest["path"]), "")
            self.assertEqual(bad[:5], [], manifest["path"])
            self.assertGreaterEqual(cells, manifest["minLabelCells"], manifest["path"])

    def test_framework_fault_notices_in_both_locales(self):
        for fixture in CASES["noticeFixtures"]:
            rows = read_json(fixture["path"])["notices"]
            self.assertGreaterEqual(len(rows), fixture["minRows"])
            self.assertEqual(len({row["code"] for row in rows}), len(rows))
            for row in rows:
                for locale in LOCALES:
                    self.assertTrue(row[locale].strip(), f"{row['code']} {locale}")
                self.assertNotEqual(row["en"], row["de"], f"{row['code']} is translated")
                self.assertEqual(placeholders(row["en"], r"\{(\w+)\}"), placeholders(row["de"], r"\{(\w+)\}"), row["code"])
            codes = [row["code"] for row in rows]
            for code in fixture["requiredCodes"]:
                self.assertIn(code, codes)

    def test_introduction_interaction_labels_are_pairs(self):
        forbidden = re.compile(CASES["pluginRoots"]["forbiddenPatterns"][0])
        localized = re.compile(CASES["pluginRoots"]["introductionLabelPattern"])
        labels = 0
        plain = []
        for root in CASES["pluginRoots"]["paths"]:
            for path in rust_sources(ROOT / root, CASES["pluginRoots"]["skipDirectories"]):
                source = path.read_text(encoding="utf8")
                if forbidden.search(source):
                    plain.append(str(path))
                labels += len(localized.findall(source))
        self.assertEqual(plain, [])
        self.assertGreaterEqual(labels, CASES["pluginRoots"]["minIntroductionLabels"])

    def test_brand_manifests_use_translated_catalog_keys_only(self):
        brand = CASES["brandSources"]
        used = set()
        for path in brand["paths"]:
            source = read(path)
            self.assertEqual([m.group(0) for m in re.finditer(brand["forbiddenPattern"], source)], [], path)
            self.assertEqual([m.group(0) for m in re.finditer(brand["forbiddenPatternMultiline"], source)], [], path)
            used.update(re.findall(brand["keyPattern"], source))
        keys = {key for key in self.catalog["texts"] if key.startswith(brand["keyPrefix"])}
        self.assertGreaterEqual(len(keys), brand["minKeys"])
        self.assertEqual(used, keys)
        for key in keys:
            if key not in brand["identicalAllowed"]:
                cells = self.catalog["texts"][key]
                self.assertNotEqual(cells["en"], cells["de"], key)

    def test_framework_sources_assume_no_language(self):
        for rule in CASES["frameworkSources"]:
            source = read(rule["path"])
            for pattern in rule["forbidden"]:
                self.assertIsNone(re.search(pattern, source), f"{rule['path']} {pattern}")
            for pattern in rule["required"]:
                self.assertIsNotNone(re.search(pattern, source), f"{rule['path']} {pattern}")

    def test_plugin_sources_pass_localized_labels_in_both_locales(self):
        pattern = re.compile(CASES["pluginRoots"]["pairPattern"])
        pairs = 0
        bad = []
        for root in CASES["pluginRoots"]["paths"]:
            for path in rust_sources(ROOT / root, CASES["pluginRoots"]["skipDirectories"]):
                for match in pattern.finditer(path.read_text(encoding="utf8")):
                    pairs += 1
                    if not match.group(1).strip() or not match.group(2).strip():
                        bad.append(f"{path}: {match.group(0)}")
        self.assertEqual(bad[:5], [])
        self.assertGreaterEqual(pairs, CASES["pluginRoots"]["minPairs"])


if __name__ == "__main__":
    unittest.main()
