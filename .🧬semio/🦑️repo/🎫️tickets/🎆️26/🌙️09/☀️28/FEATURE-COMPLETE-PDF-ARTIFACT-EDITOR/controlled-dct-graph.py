"""🗂️ Exercise the actual bounded PDF byte graph with the ticket-only DCT stream planner."""

import importlib.util
import json
from pathlib import Path
import os
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"


def exercise(base):
    spec = importlib.util.spec_from_file_location("codec_probe", TICKET / "📜️cff-file-codec-probe.py")
    probe = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(probe)
    original_stage = probe.probes.stage_paths
    os.environ["PDF_CONTROLLED_LIVE"] = "1"
    staged = json.loads((OUTPUT / "dct-stage" / "stages.json").read_text())
    decoded = Path(staged["🔨️modules/🗜️filters/🎚️decoded-stream/🦀️.rs"]).read_text()

    def stage_paths(base, scratch, extra):
        extra = original_stage(base, scratch, extra)
        (scratch / "decoded-stream.rs").write_text(decoded)
        return extra

    probe.probes.stage_paths = stage_paths
    body = (TICKET / "controlled-dct-graph.rs").read_text().replace("@ASSETS@", (OUTPUT / "dct-assets").as_posix())
    probe.exercise(base, "unused.pdf", body=body)


if __name__ == "__main__":
    sys.dont_write_bytecode = True
    exercise(Path(sys.argv[1]))
