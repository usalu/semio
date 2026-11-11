"""🧪 Master freshness includes the current presentation fingerprint."""
import json
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media"))
import lib

CASES = json.loads((HERE / "media-freshness-cases.json").read_text(encoding="utf-8"))


class MasterFreshness(unittest.TestCase):
    """🖼️ Unchanged source clips cannot make an old frontend current."""

    def test_presentation_changes(self):
        folder = HERE / "🗑️generated" / "master-freshness"
        folder.mkdir(parents=True, exist_ok=True)
        master = folder / "master.mp4"
        master.write_bytes(b"fixture")
        state = {"sources": {"a": "b"}}
        declared = {"script": "fixture", "scene": "Scene"}
        saved = {"content": {"chapters": [{"segments": [{**state, "segment": declared}]}]}, "presentation": CASES["savedPresentation"], "cursor": "cursor"}
        master.with_suffix(".render.json").write_text(json.dumps(saved), encoding="utf-8")
        status = {"video": "fixture", "master": master, "content": folder / "content.mp4", "settings": {"language": "en"}, "segments": [{"fresh": True, "state": state, "declared": declared}]}
        for key, current in (("unchanged", CASES["savedPresentation"]), ("changed", CASES["changedPresentation"])):
            runtime = unittest.mock.Mock()
            runtime.presentation_state.return_value = current
            runtime.state_inputs_current.return_value = True
            runtime.reusable_clip.return_value = True
            runtime._cursor_hash.return_value = "cursor"
            with self.subTest(case=key), patch.object(lib, "runtime", return_value=runtime):
                np.testing.assert_equal(lib.master_fresh(status), CASES["expected"][key])


if __name__ == "__main__":
    unittest.main()
