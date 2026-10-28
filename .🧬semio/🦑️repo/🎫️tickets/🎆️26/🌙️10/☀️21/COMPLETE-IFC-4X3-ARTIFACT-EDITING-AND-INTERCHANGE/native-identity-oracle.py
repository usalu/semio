"""🧪 Compare planned identity requirements with independent actual IFC populations."""
import argparse
import json
from typing import Protocol


class IdentityOracle(Protocol):
    """🔌 Keep third-party types behind built-in population facts."""
    def additions(self, source: str, expected: str) -> dict[int, bool]: ...


class IfcOpenShellOracle:
    """🏗️ Read additions and rooted identity requirements from independent IFC entities."""
    def __init__(self):
        import ifcopenshell
        self.api = ifcopenshell

    def additions(self, source: str, expected: str) -> dict[int, bool]:
        before = self.api.file.from_string(source)
        after = self.api.file.from_string(expected)
        original = {entity.id() for entity in before}
        return {entity.id(): entity.is_a("IfcRoot") for entity in after if entity.id() not in original}


def main():
    """🧾 Emit exact roster agreement and actual library metadata."""
    parser = argparse.ArgumentParser()
    parser.add_argument("cases")
    parser.add_argument("queries")
    args = parser.parse_args()
    with open(args.cases, encoding="utf-8-sig") as stream:
        cases = {row["name"]: row for row in json.load(stream)["cases"]}
    with open(args.queries, encoding="utf-8-sig") as stream:
        queries = json.load(stream)["queries"]
    adapter = IfcOpenShellOracle()
    oracle: IdentityOracle = adapter
    results = []
    for query in queries:
        case = cases[query["sourceCase"]]
        additions = oracle.additions(case["source"], case["expected"])
        identities = case["application"]["identities"]
        assert set(additions) == {int(row["id"]) for row in identities}
        actual = [{"key": row["key"], "globalId": additions[int(row["id"]) ]} for row in identities]
        assert actual == query["requirements"]
        results.append({"sourceCase": query["sourceCase"], "requirements": actual, "matches": True})
    print(json.dumps({"oracle": "IfcOpenShell", "version": adapter.api.version, "nativeExecuted": False, "cases": results}, ensure_ascii=False))


if __name__ == "__main__":
    main()
