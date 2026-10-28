"""🎭 Run focused BIM oracle cases with immediate durable subcase tracebacks."""
import contextlib
import importlib.util
from pathlib import Path
import sys
import time
import traceback
import unittest

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
DRIVER = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🐍️.py"


class Tee:
    """📝 Preserve console progress and durable owned diagnostic evidence."""

    def __init__(self, stream, log):
        """🔌 Bind two local streams without background threads or signals."""
        self.stream, self.log = stream, log

    def write(self, value):
        """✍️ Persist each actual diagnostic before returning to the oracle."""
        self.stream.write(value)
        self.log.write(value)
        self.log.flush()
        return len(value)

    def flush(self):
        """💧 Flush both owned streams."""
        self.stream.flush()
        self.log.flush()


class Result(unittest.TextTestResult):
    """🧾 Surface a real native failure as soon as its subcase finishes."""

    def addSubTest(self, test, subtest, error):
        """🔎 Preserve unittest results and print the same actual exception immediately."""
        super().addSubTest(test, subtest, error)
        if error is not None:
            print("[DEBUG] BIM native subcase failed", subtest, flush=True)
            traceback.print_exception(*error)


def main():
    """🧪 Run selected ordinary tests with durable diagnostics and no asynchronous watchdog."""
    output = TICKET / "🗑️generated/ib"
    output.mkdir(parents=True, exist_ok=True)
    path = output / ("native-" + str(time.time_ns()) + ".log")
    print("[DEBUG] BIM native log", path, flush=True)
    with path.open("w", encoding="utf-8", buffering=1) as log:
        with contextlib.redirect_stdout(Tee(sys.stdout, log)), contextlib.redirect_stderr(Tee(sys.stderr, log)):
            spec = importlib.util.spec_from_file_location("inspection_bim_native_contract", DRIVER)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            suite = (unittest.defaultTestLoader.loadTestsFromNames(sys.argv[1:], module)
                     if len(sys.argv) > 1 else unittest.defaultTestLoader.loadTestsFromModule(module))
            return not unittest.TextTestRunner(resultclass=Result, verbosity=1).run(suite).wasSuccessful()


if __name__ == "__main__":
    sys.exit(main())
