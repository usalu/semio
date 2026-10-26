"""🧩 Break mutable Manim SVG cache hard links without stopping active renders."""

from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import time
from xml.etree import ElementTree

generated = (Path(__file__).resolve().parent / "🗑️generated").resolve()
files = [*generated.glob("videos/*/t/*.svg"), *generated.glob("pre/*/t/*.svg")]


def isolate(svg):
    """🔒 Atomically replace a shared generated SVG with a validated private inode."""
    if not svg.resolve().is_relative_to(generated):
        raise ValueError("Cache path escaped the ticket")
    for _ in range(40):
        try:
            if svg.stat().st_nlink <= 1:
                return "private"
            data = svg.read_bytes()
            ElementTree.fromstring(data)
            temporary = svg.with_suffix(".private")
            temporary.write_bytes(data)
            os.replace(temporary, svg)
            return "isolated"
        except (OSError, ElementTree.ParseError):
            time.sleep(0.025)
    return str(svg)


with ThreadPoolExecutor(max_workers=12) as pool:
    results = list(pool.map(isolate, files))
summary = {"isolated": results.count("isolated"), "already_private": results.count("private"),
           "unresolved": [result for result in results if result not in ("isolated", "private")]}
(generated / "cache-isolation.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
print("[DEBUG]", json.dumps(summary), flush=True)
