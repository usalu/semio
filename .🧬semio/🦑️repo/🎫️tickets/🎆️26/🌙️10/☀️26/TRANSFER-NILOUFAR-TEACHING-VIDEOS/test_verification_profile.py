"""🧪 Check independent verifier profiles from a language-neutral case table."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import unittest


class VerificationProfileTest(unittest.TestCase):
    def test_profiles_and_invalid_arguments(self):
        ticket = Path(__file__).parent
        cases = json.loads((ticket / "verification-profile.cases.json").read_text(encoding="utf-8"))
        for script in ("verify-videos.py", "make-previews.py"):
            spec = importlib.util.spec_from_file_location("verification_profile", ticket / script)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            base = ["videos", "--teaching-root", "teaching", "--ffprobe", "ffprobe"]
            base.extend(["--report" if script == "verify-videos.py" else "--output", "report"])
            for case in cases["profiles"]:
                with self.subTest(script=script, profile=case):
                    args = module.parse_args([*base, *case["arguments"]])
                    self.assertEqual((args.width, args.height, args.fps), (case["width"], case["height"], case["fps"]))
                    self.assertEqual(module.movie_suffix(args), case["suffix"])
            for invalid in cases["invalid"]:
                with self.subTest(script=script, invalid=invalid), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                    module.parse_args([*base, *invalid])


if __name__ == "__main__":
    unittest.main()
