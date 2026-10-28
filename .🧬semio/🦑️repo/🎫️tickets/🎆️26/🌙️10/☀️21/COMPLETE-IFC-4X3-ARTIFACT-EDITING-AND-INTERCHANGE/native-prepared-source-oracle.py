"""🧪 Verify exact prepared identity leaves against independent released IFC populations."""
import argparse
import json
import platform
from typing import Protocol


class PopulationOracle(Protocol):
    """🔌 Expose only built-in population facts across the third-party boundary."""
    def facts(self, source: str, expected: str) -> dict: ...


class IfcOpenShellOracle:
    """🏗️ Read actual additions, rooted identities and full EXPRESS diagnostics."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.validate
        self.api = ifcopenshell
        self.validation = ifcopenshell.validate

    def facts(self, source: str, expected: str) -> dict:
        before = self.api.file.from_string(source)
        after = self.api.file.from_string(expected)
        original = {entity.id() for entity in before}
        additions = {entity.id(): getattr(entity, "GlobalId", None) for entity in after if entity.id() not in original}
        logger = self.validation.json_logger()
        self.validation.validate(after, logger, express_rules=True)
        return {"additions": additions, "errors": len(logger.statements)}


def leaves(value, path=()):
    """🧭 Enumerate complete original carrier addresses without manufacturing wrappers."""
    if isinstance(value, str):
        yield path, value
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from leaves(item, path + (str(index),))
    elif isinstance(value, dict):
        for key, item in value.items():
            yield from leaves(item, path + (key,))
    else:
        raise AssertionError("unsupported source representation")


def main():
    """🧾 Preserve exact environment and source-only native proof qualifications."""
    parser = argparse.ArgumentParser()
    parser.add_argument("cases")
    args = parser.parse_args()
    with open(args.cases, encoding="utf-8-sig") as stream:
        cases = json.load(stream)["cases"]
    adapter = IfcOpenShellOracle()
    oracle: PopulationOracle = adapter
    results = []
    for case in cases:
        facts = oracle.facts(case["source"], case["expected"])
        assert facts["errors"] == 0, (case["name"], facts)
        identities = case["application"]["identities"]
        assert set(facts["additions"]) == {int(row["id"]) for row in identities}
        for row in identities:
            assert facts["additions"][int(row["id"])] == row.get("guid")
        fields = list(leaves(case["application"]))
        pages = []
        for offset in range(0, len(fields), 16):
            count = min(16, len(fields) - offset)
            navigation = int(offset > 0) + int(offset + 16 < len(fields))
            assert count <= 16 and navigation <= 2 and count + navigation <= 18
            pages.append({"offset": str(offset), "content": count, "pages": navigation})
        results.append({"sourceCase": case["name"], "scalarLeaves": len(fields), "identityLeavesMatch": True, "expressErrors": 0, "pages": pages})
    print(json.dumps({"oracle": "IfcOpenShell", "version": adapter.api.version, "python": platform.python_version(), "nativeExecuted": False, "cases": results}))


if __name__ == "__main__":
    main()
