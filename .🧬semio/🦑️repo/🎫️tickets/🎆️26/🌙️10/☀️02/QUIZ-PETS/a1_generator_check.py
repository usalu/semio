#!/usr/bin/env python3
"""🔍️ Ticket tool of work package A1: runs `generate_behavior_vectors.py` without letting it write a fixture.

Another work package regenerates the same fixtures at the same time, so the split of the stage is proven without
touching them: the generator composes its documents, records every stage trace from the TypeScript subject, lets the
case's Python file verify them and compares each recorded trace with the committed one — all as it always does —,
and where it would write a fixture of the product this tool only says whether the bytes are the committed ones
(`unchanged <path>`) or not (`WOULD WRITE <path>`). Files of the ticket scratch are written as usual.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``; exits with 1 when a fixture would change.
"""

import importlib.util
import os
import sys

sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
TICKET = os.path.dirname(os.path.abspath(__file__))


def main():
    """🚦️ Loads the generator, guards its fixture writes and runs it."""
    spec = importlib.util.spec_from_file_location("generate_behavior_vectors", os.path.join(TICKET, "generate_behavior_vectors.py"))
    generator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(generator)
    fixtures = os.path.join(generator.PETS, "🧫️fixtures")
    dump = generator.dump
    changed = []

    def guarded(path, document):
        if not os.path.abspath(path).startswith(os.path.abspath(fixtures)):
            dump(path, document)
            return
        payload = (generator.json.dumps(document, ensure_ascii=False, indent=2, allow_nan=False) + "\n").encode("utf-8")
        if os.path.exists(path) and open(path, "rb").read() == payload:
            print("unchanged %s" % os.path.relpath(path, generator.ROOT))
            return
        changed.append(path)
        print("WOULD WRITE %s" % os.path.relpath(path, generator.ROOT))

    generator.dump = guarded
    generator.main()
    sys.exit(1 if changed else 0)


if __name__ == "__main__":
    main()
