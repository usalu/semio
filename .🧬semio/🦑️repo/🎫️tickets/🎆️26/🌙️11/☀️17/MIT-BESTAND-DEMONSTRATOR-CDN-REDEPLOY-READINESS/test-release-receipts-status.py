"""🧪️ The release receipt gate names every unconfirmed roster module and whether it was staged."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("release_receipts", TICKET / "release-receipts.py")
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class MissingReport(unittest.TestCase):
    def test_lists_missing_modules_with_staging_state(self):
        cases = {"pluginIds": ["alpha", "beta"], "extensionIds": ["gamma"]}
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "modules" / "🅰️beta").mkdir(parents=True)
            (root / "modules" / "🅰️beta" / gate.RECEIPT).write_text("{}")
            (root / "confirmed").mkdir()
            (root / "confirmed" / "alpha.json").write_text("{}")
            missing, report = gate.missing_report(root / "modules", root / "confirmed", cases)
        self.assertEqual(missing, ["beta", "gamma"])
        self.assertEqual(report.splitlines(), ["1 of 3 roster modules confirmed; 2 still missing:", "  beta: staged, awaiting confirmation", "  gamma: no staged release"])


if __name__ == "__main__":
    unittest.main()
