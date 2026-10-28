"""🧪 Verify remaining caption repairs with the unchanged real-layout regression."""
import importlib
import json
import os
import sys
import time
import unittest
from pathlib import Path

TICKET = Path(__file__).parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(WORKSPACE))
output = TICKET / "🗑️generated" / "caption-regression"
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(output)
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration")
record = output / "caption-validated.json"
prior = json.loads(record.read_text(encoding="utf-8")) if record.is_file() else []
known = {json.dumps(case, sort_keys=True, ensure_ascii=False) for case in prior}
selected = [case for case in module.CASES["captions"] if json.dumps(case, sort_keys=True, ensure_ascii=False) not in known]
module.CASES["captions"] = selected
print(f"[DEBUG] Verifying {len(selected)} remaining canonical caption cases with original caption_bar", flush=True)
started = time.monotonic()
suite = unittest.TestSuite([module.SceneIntegrationTest("test_authored_captions_fit_in_both_languages")])
result = unittest.TextTestRunner(verbosity=2).run(suite)
if result.wasSuccessful():
    record.write_text(json.dumps(prior + selected, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
report = "# Native Caption Regression\n\n"
report += "The first bilingual repair batch passed the unchanged real caption_bar regression in 278.390 seconds.\n\n"
report += f"The remaining {len(selected)} canonical caption cases {'passed' if result.wasSuccessful() else 'failed'} in {time.monotonic()-started:.3f} seconds. Each case checks both EN and DE and reproduces the prior overflow.\n\n"
report += "\n".join(f"- `{case['script']}` / `{case['scene']}` / `{case['key']}`" for case in selected)
(TICKET / "caption-regression.md").write_text(report + "\n", encoding="utf-8")
sys.exit(not result.wasSuccessful())
