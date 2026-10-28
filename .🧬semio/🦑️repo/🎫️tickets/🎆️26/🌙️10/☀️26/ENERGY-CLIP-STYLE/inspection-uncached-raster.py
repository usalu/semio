"""🔬 Retain the uncached native raster diagnostic separately from accepted publication tests."""
import importlib.util
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
driver = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🧩️chapters/🐍️.py"
spec = importlib.util.spec_from_file_location("boundary_diagnostic", driver)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
module.RecursiveChapterBrowserContract().capture_uncached_browser_process_endpoint_pixels()
