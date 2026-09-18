#!/usr/bin/env python3
"""🐍️ Ticket-local runner for wfc3d's two Python halves — the mutation second implementation and the
mount contract. It runs the SAME files that live in the artifact tree (it does not duplicate them),
so a green run here is a green run of what is committed.

    python3 🐍️wfc3d-oracle-runner.py
"""

import os
import subprocess
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
SUBSET = os.path.join(ROOT, "✏️s", "🔌️plugins", "🀄️wfc", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any")


def run(label, argv):
    print(f"── {label}")
    result = subprocess.run([sys.executable, *argv], cwd=SUBSET)
    print()
    return result.returncode


def main():
    failures = 0
    failures += run("mutation second implementation (15 vectors)", [os.path.join("🧪️tests", "🧩️mutate-wfc3d-1", "🐍️.py"), "--fixtures", os.path.join("🧫️fixtures", "🧬️mutations")])
    failures += run("mount contract", [os.path.join("🧪️tests", "🧩️mount-contract", "🐍️.py")])
    print("ALL GREEN" if failures == 0 else f"{failures} failing lane(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
