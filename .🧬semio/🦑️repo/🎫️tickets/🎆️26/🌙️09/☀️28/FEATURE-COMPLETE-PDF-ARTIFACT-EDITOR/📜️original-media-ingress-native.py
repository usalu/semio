"""🧪 Compile the native media sender and drive the shared neutral transfer law."""

import json
from pathlib import Path
import os
import subprocess

TICKET = Path(__file__).resolve().parent
FIXTURE = json.loads((TICKET / "🔣️original-media-ingress-cases.json").read_text(encoding="utf8"))
TARGET = TICKET / "🗑️generated" / ("media-ingress-test.exe" if os.name == "nt" else "media-ingress-test")


def verify():
    """🔬 Check native reads, acknowledgments and terminal cancellation from the exact same cases."""
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                    "--edition=2021", "--crate-name", "media_ingress_test", str(TICKET / "📜️original-media-ingress-test.rs"), "-o", str(TARGET)], check=True)
    loan = FIXTURE["loan"]
    address = [loan[key] for key in ("operationId", "generation", "instanceId", "documentId", "baseRevision")]
    for row in FIXTURE["cases"]:
        trigger = next((row[key] for key in ("cancelAfter", "staleAfter", "shortAt", "rejectAt") if key in row), "")
        address[3] = row.get("documentId", loan["documentId"])
        args = [*address, row["name"], row["size"], row["limit"], trigger, row["acknowledged"], row["result"],
                row.get("filename", "picked-image.jpg"), row.get("mediaType", "image/jpeg")]
        subprocess.run([str(TARGET), *map(str, args)], check=True)


if __name__ == "__main__":
    verify()
