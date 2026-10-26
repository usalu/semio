"""🧭 Independently verify the frozen teaching relocation contract and taxonomy."""
import argparse
import ast
import fnmatch
import json
from pathlib import Path
import re
import sys
import unittest

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "AGENTS.md").is_file())
TAXONOMY = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
PLAN = json.loads((TICKET / "taxonomy-paths.cases.json").read_text(encoding="utf-8-sig"))
TAX = json.loads(TAXONOMY.read_text(encoding="utf-8"))
MOVES = PLAN["moves"]
TARGETS = [m["new"] for m in MOVES if m["new"]]


def directory_kinds(name, parents):
    """🌳 Resolve registered semantic names with parent-aware admission."""
    direct = set()
    for kind, rule in TAX["semanticDirectoryKinds"].items():
        allowed = rule.get("parentKindIds")
        if allowed and not set(allowed) & parents:
            continue
        emoji = rule["emoji"]
        if name.startswith(emoji) and re.fullmatch(rule["slugPattern"], name[len(emoji):]):
            direct.add(kind)
    members = set()
    for kind, rule in TAX["semanticDirectoryMemberKinds"].items():
        if set(rule["ownerKindIds"]) & parents and name in rule["memberNames"]:
            members.add(kind)
    return direct | members


class LayoutTest(unittest.TestCase):
    def test_frozen_move_coverage(self):
        self.assertEqual(len(MOVES), 141)
        self.assertEqual(len(TARGETS), 114)
        self.assertEqual(len({m["old"] for m in MOVES}), 141)
        self.assertEqual(len({p.casefold() for p in TARGETS}), 114)
        for move in MOVES:
            with self.subTest(path=move["old"]):
                self.assertFalse((ROOT / move["old"]).exists())
                if move["new"]:
                    self.assertTrue((ROOT / move["new"]).is_file())
                    self.assertLessEqual(len(str(ROOT / move["new"]).encode("utf-16-le")) // 2, 256)
                else:
                    self.assertTrue(move["old"].endswith("/manim.cfg"))

    def test_registered_directory_paths(self):
        checked = set()
        for target in TARGETS:
            parents = set()
            path = Path(target)
            for i, name in enumerate(path.parts[:-1]):
                key = path.parts[:i + 1]
                resolved = directory_kinds(name, parents)
                if key not in checked:
                    with self.subTest(directory="/".join(key)):
                        self.assertTrue(resolved, "No parent-aware semantic kind or member admission")
                    checked.add(key)
                parents = resolved

    def test_kind_only_or_exact_contract_leaves(self):
        for target in TARGETS:
            path = Path(target)
            canonical = any(path.name == rule["emoji"] + extension for rule in TAX["fileKinds"].values() for extension in rule["extensionChains"])
            fixed = []
            for key, contract in TAX["fixedFilenameContracts"].items():
                pattern = contract["pathPattern"]
                if not (fnmatch.fnmatchcase(target, pattern) or (pattern.startswith("**/") and fnmatch.fnmatchcase(target, pattern[3:]))):
                    continue
                scope = contract["scope"]
                if scope["kind"] == "path-pattern":
                    fixed.append(key)
                elif scope["kind"] == "package-root" and scope.get("ecosystemId") == "🐍️python" and path.parent.parts[-2:] == ("📦️packages", "🐍️python"):
                    fixed.append(key)
            with self.subTest(path=target):
                self.assertTrue(canonical or fixed, "Not a kind-only leaf or exact scoped contract")

    def test_manifest_closure(self):
        import jsonschema
        manifest_path = ROOT / PLAN["energy"] / "🎬️clip/🔣️.json"
        manifest = json.loads(manifest_path.read_text(encoding="utf-8-sig"))
        schema_path = ROOT / PLAN["authoring"] / "🎬️clip/🧬️schema/🔣️.schema.json"
        schema = json.loads(schema_path.read_text(encoding="utf-8-sig"))
        jsonschema.validators.validator_for(schema)(schema).validate(manifest)
        videos = manifest["videos"]
        self.assertEqual(len(videos), 5)
        self.assertEqual([len(v["chapters"]) for v in videos], [1, 6, 6, 6, 7])
        self.assertEqual(sum(len(c["segments"]) for v in videos for c in v["chapters"]), 112)
        teaching = ROOT / "🎓️teaching"
        wrappers = set()
        intros = set()
        for video in videos:
            wrapper = teaching / video["script"]
            wrappers.add(wrapper)
            self.assertTrue(wrapper.relative_to(ROOT).as_posix() in TARGETS, "Noncanonical wrapper: " + str(wrapper))
            self.assertTrue(wrapper.is_file())
            descriptors = [video["intro"], *(d for c in video["chapters"] for d in c["segments"])]
            intros.add((video["intro"]["script"], video["intro"]["scene"]))
            for descriptor in descriptors:
                source = teaching / descriptor["script"]
                self.assertTrue(source.relative_to(ROOT).as_posix() in TARGETS, "Noncanonical segment: " + str(source))
                tree = ast.parse(source.read_text(encoding="utf-8-sig"))
                classes = {node.name for node in tree.body if isinstance(node, ast.ClassDef)}
                self.assertIn(descriptor["scene"], classes)
            classes = {node.name for node in ast.parse(wrapper.read_text(encoding="utf-8-sig")).body if isinstance(node, ast.ClassDef)}
            self.assertTrue({c["scene"] for c in video["chapters"]} <= classes)
        self.assertEqual(len(wrappers), 5)
        self.assertEqual(len(intros), 5)

    def test_ticket_scope_is_explicitly_exempt(self):
        self.assertEqual(TAX["areas"][".🧬semio/🦑️repo/🎫️tickets"], "exempt")
        self.assertEqual(TAX["areas"]["🎓️teaching"], "clean")


if __name__ == "__main__":
    unittest.main()
