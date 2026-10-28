"""🧪 Law of batch `productparse` (lane dashnoop): the product runner records Bazel's summary line in both grammatical numbers, so a run that executed nothing (`INFO: 1 process: 1 internal.`, `1 total action`) is not recorded as an empty graph.
Independent oracle: every summary line of every recorded run log, split by plain string operations (no regular expression); the runner's patterns must match exactly the lines the oracle finds and agree on the count and the breakdown.
usage: python3 -I test_product_patterns.py [--expect-red]"""
import glob, importlib.util, os, sys, unittest
HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.dirname(HERE)
spec = importlib.util.spec_from_file_location("product", os.path.join(TOOLS, "product.py"))
product = importlib.util.module_from_spec(spec); spec.loader.exec_module(product)
def oracle(line):
    if not line.startswith("INFO: "): return None
    head, sep, rest = line[6:].partition(": ")
    count, _, noun = head.partition(" ")
    if not sep or not count.isdigit() or noun not in ("process", "processes"): return None
    return int(count), rest.rstrip("\n").rstrip(".")
def total(line):
    prefix = "INFO: Build completed successfully, "
    if not line.startswith(prefix): return None
    count, _, noun = line[len(prefix):].rstrip("\n").partition(" ")
    return int(count) if count.isdigit() and noun in ("total action", "total actions") else None
class ProductPatterns(unittest.TestCase):
    def test_both_numbers_in_samples(self):
        for line, count, rest in (("INFO: 1 process: 1 internal.", "1", "1 internal"), ("INFO: 3 processes: 1 disk cache hit, 1 internal, 1 worker.", "3", "1 disk cache hit, 1 internal, 1 worker"), ("INFO: 1 process: 2 action cache hit, 1 internal.", "1", "2 action cache hit, 1 internal")):
            match = product.PATTERNS["processes"].search(line)
            self.assertIsNotNone(match, line)
            self.assertEqual(match.groups(), (count, rest))
        for line, count in (("INFO: Build completed successfully, 1 total action", "1"), ("INFO: Build completed successfully, 3 total actions", "3")):
            match = product.PATTERNS["actions"].search(line)
            self.assertIsNotNone(match, line)
            self.assertEqual(match.groups(), (count,))
    def test_every_recorded_summary_line_is_found(self):
        logs = sorted(glob.glob(os.path.join(TOOLS, "run", "*.log")))
        seen = {"process": 0, "action": 0}
        for path in logs:
            with open(path, encoding="utf8", errors="replace") as stream:
                for line in stream:
                    expected = oracle(line)
                    match = product.PATTERNS["processes"].search(line)
                    if expected is None: self.assertIsNone(match, line); continue
                    self.assertIsNotNone(match, path + ": " + line)
                    self.assertEqual((int(match.group(1)), match.group(2)), expected)
                    seen["process"] += 1
                    count = total(line)
                    self.assertIsNone(product.PATTERNS["actions"].search(line) if count is None else None)
            with open(path, encoding="utf8", errors="replace") as stream:
                for line in stream:
                    count = total(line)
                    match = product.PATTERNS["actions"].search(line)
                    self.assertEqual(None if match is None else int(match.group(1)), count, line)
                    seen["action"] += count is not None
        if logs: self.assertGreater(seen["process"], 0)
if __name__ == "__main__":
    sys.argv = sys.argv[:1] + [a for a in sys.argv[1:] if a != "--expect-red"]
    unittest.main()
