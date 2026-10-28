"""🧹 Remove only this ticket's generated verification output after review."""

from pathlib import Path
import os
import shutil

TICKET = Path(__file__).resolve().parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
TARGET = TICKET / "🗑️generated"
if TARGET.exists():
    resolved = TARGET.resolve(strict=True)
    if resolved != TARGET or not resolved.is_relative_to(TICKET) or not resolved.is_relative_to(WORKSPACE):
        raise RuntimeError(f"Unsafe generated-output target: {resolved}")
    for current, directories, files in os.walk(resolved, followlinks=False):
        for name in directories + files:
            item = Path(current) / name
            if item.is_symlink() or item.is_junction():
                raise RuntimeError(f"Linked generated output requires separate review: {item}")
    shutil.rmtree(resolved)
    if TARGET.exists():
        raise RuntimeError("Generated-output cleanup did not finish")
print("[DEBUG] Ticket generated output is absent; inputs and reports are retained.")
