"""🔬 Reproduce the retained portable precision diagnostic without production-source dependencies."""
import runpy
from pathlib import Path
runpy.run_path(str(Path(__file__).with_name("floor-beam-unique-overlap-r92.py")),run_name="__main__")
