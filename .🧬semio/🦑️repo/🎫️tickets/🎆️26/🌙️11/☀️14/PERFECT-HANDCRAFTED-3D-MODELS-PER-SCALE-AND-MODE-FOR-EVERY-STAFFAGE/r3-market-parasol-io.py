"""📦 Loads the market parasol kit (hyphenated file name) for the other r3-market-parasol scripts.

See ./r3-objects-market-parasol.md
"""
import importlib.util
from pathlib import Path


def kit():
    """🧰 The module r3-market-parasol-kit.py."""
    spec = importlib.util.spec_from_file_location("parasol_kit", Path(__file__).resolve().parent / "r3-market-parasol-kit.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
