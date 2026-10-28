"""🧪 Run cursor interaction contracts with every output confined to this ticket."""

import faulthandler
import importlib
import io
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")
presentation = len(sys.argv) > 1 and sys.argv[1] == "--presentation"
runtime = len(sys.argv) > 1 and sys.argv[1] == "--runtime"
OUTPUT = Path(__file__).with_name("🗑️generated") / "cursor-tests"
if presentation:
    OUTPUT /= "presentation"
elif runtime:
    OUTPUT /= "runtime"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "fonts")
package = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
module_name = package + (".🧪️tests.🐍️" if presentation else ".🖱️interaction.🧪️tests.🐍️")
if runtime:
    module_name = package.rsplit(".🪟️presentation", 1)[0] + ".🧪️tests.🐍️"
debug = (OUTPUT / "stacks.log").open("w", encoding="utf-8")
faulthandler.dump_traceback_later(30, repeat=True, file=debug)
module = importlib.import_module(module_name)
arguments = sys.argv[2:] if presentation or runtime else sys.argv[1:]
name = arguments[0] if arguments else None
suite = unittest.defaultTestLoader.loadTestsFromName(name, module) if name else unittest.defaultTestLoader.loadTestsFromModule(module)
class Transcript(io.StringIO):
    def write(self, value):
        sys.stdout.write(value)
        sys.stdout.flush()
        return super().write(value)


transcript = Transcript()
result = unittest.TextTestRunner(stream=transcript, verbosity=2).run(suite)
(OUTPUT / ("tests-" + name.split(".")[0] + ".log" if name else "tests.log")).write_text(transcript.getvalue(), encoding="utf-8")
faulthandler.cancel_dump_traceback_later()
debug.close()
raise SystemExit(not result.wasSuccessful())
