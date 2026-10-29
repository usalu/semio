import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("        for case in CASES[\"ranges\"]:")
b = src.index("    def test_reference_tokens_and_colours")
new = '''        for case in CASES["ranges"]:
            with self.subTest(a1=case["a1"]):
                self.assertEqual(list(self.book.parse_range(case["a1"])), case["box"])
                if case["a1"].replace("$", "").split(":")[0] <= case["a1"].replace("$", "").split(":")[-1] and ":" in case["a1"]:
                    self.assertEqual(list(range_boundaries(case["a1"].replace("$", ""))), case["box"])

'''
src = src[:a] + new + src[b:]
a = src.index("    @unittest.skipUnless(importlib.util.find_spec")
b = src.index("    def test_engine_hooks")
src = src[:a] + '    @unittest.skipUnless(ENGINE, "XL engine not built yet")\n' + src[b:]
src = src.replace('RESERVED = {', 'ENGINE = (CLIP / "🧱️model/🐍️.py").is_file()\nRESERVED = {')
open(path, "w", encoding="utf-8").write(src)
