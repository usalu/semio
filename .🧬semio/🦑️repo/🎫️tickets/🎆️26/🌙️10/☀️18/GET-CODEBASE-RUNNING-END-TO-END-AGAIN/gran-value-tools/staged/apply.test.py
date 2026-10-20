"""🧪 Language-agnostic cases (cases.json) for the anchored edit primitives, cross-checked with Bun's TOML parser."""
import importlib.util, json, os, subprocess, tempfile, unittest

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("apply_mod", HERE + "/apply.py")
apply_mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(apply_mod)
CASES = json.load(open(HERE + "/cases.json", encoding="utf-8"))


def run(case, root):
    plan = apply_mod.Plan(root)
    with open(os.path.join(root, "f"), "w", encoding="utf-8") as f:
        f.write(case["input"])
    getattr(apply_mod, case["op"])(plan, "f", *case["args"])
    return plan.text("f")


def keys(text):
    with tempfile.NamedTemporaryFile("w", suffix=".toml", delete=False, encoding="utf-8") as f:
        f.write(text)
    out = subprocess.run(["bun", HERE + "/toml_keys.ts", f.name], capture_output=True, text=True, check=True).stdout
    os.remove(f.name)
    return json.loads(out)


class Cases(unittest.TestCase):
    def test_manifest_cases(self):
        for case in CASES["manifest"]:
            with tempfile.TemporaryDirectory() as root:
                got = run(case, root)
                self.assertEqual(got, case["expected"], case["name"])
                parsed = keys(got)
                self.assertEqual(parsed["dependencies"], case["dependencies"], case["name"])
                self.assertEqual(parsed["devDependencies"], case["devDependencies"], case["name"])

    def test_refusals(self):
        for case in CASES["refusals"]:
            with tempfile.TemporaryDirectory() as root:
                with self.assertRaises(apply_mod.Refusal, msg=case["name"]):
                    run(case, root)


if __name__ == "__main__":
    unittest.main()
