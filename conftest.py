import pathlib
import sys

root = pathlib.Path(__file__).parent
sys.path.insert(0, str(root / "semio" / "py"))
sys.path.insert(0, str(root / "semio" / "engine"))

import semio  # noqa: E402, F401
import engine  # noqa: E402, F401

semio.__path__ = [str(root / "semio")]
