"""📄 Independently model reservation, charge, and backing across original turns."""
from pathlib import Path
from itertools import accumulate
import json
import sys


def advance(phase, grant):
    return phase + (grant == "full" or (grant == "short-capacity" and phase < 2))


if __name__ == "__main__":
    fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    count = 0
    for case in fixture["cases"]:
        turns = case["turns"]
        phases = list(accumulate((turn["grant"] for turn in turns), advance, initial=0))
        expected = [dict(advance=current > previous, held=current >= 2, staged=current == 3)
                    for previous, current in zip(phases, phases[1:])]
        actual = [{key: turn[key] for key in ("advance", "held", "staged")} for turn in turns]
        if expected != actual or phases[-1] != 3:
            raise SystemExit(case["id"])
        count += len(turns)
    print(f"{count} original event-phase rows agree with the independent oracle")
