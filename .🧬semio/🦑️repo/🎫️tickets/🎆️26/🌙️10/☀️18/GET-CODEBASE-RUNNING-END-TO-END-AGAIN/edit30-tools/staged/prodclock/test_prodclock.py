"""⏱ Law of batch `prodclock`: product.py's `wall_s` is the child's exit time to within 0.5 s (base: the next 5 s poll tick). The oracle is the child's own sleep length.
usage: python3 -I test_prodclock.py <product.py>"""
import importlib.util, json, os, subprocess, sys, tempfile, unittest

PRODUCT = os.path.abspath(sys.argv.pop(1))


class Clock(unittest.TestCase):
    def test_wall_is_exit_time(self):
        self.assertIn('if __name__ == "__main__":', open(PRODUCT, encoding="utf8").read(), "product.py must not start a build on import")
        spec = importlib.util.spec_from_file_location("product_under_test", PRODUCT)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        out = tempfile.mkdtemp(prefix="prodclock-")
        module.OUT = out
        real = subprocess.Popen
        module.subprocess.Popen = lambda args, **kw: real(["sleep", "1.3"], **{k: v for k, v in kw.items() if k != "cwd"})
        try:
            module.main("law", "target")
        finally:
            module.subprocess.Popen = real
        wall = json.load(open(os.path.join(out, "law.json")))["wall_s"]
        self.assertGreaterEqual(wall, 1.3)
        self.assertLess(wall, 1.8)


unittest.main(argv=[sys.argv[0]])
