"""🧟️ Mutation check of the gesture-drive law: five semantic mutants of `driveGesture` (TS twin of `drive_gesture`), each
judged by a copy of `🧪️tests/🧪️gesture-drive-law/🟦️.ts` under `🗑️generated/s4-tools-a/mutants/`; every mutant must fail.

Usage: python3 🧪️s4-tools-a-gesture-drive-mutants.py   (cwd: repo root)
"""

import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
MODULE = REPO / "🧰️framework/🔨️modules/🛠️tool-machine"
OUT = pathlib.Path(__file__).resolve().parent / "🗑️generated/s4-tools-a/mutants"

MUTANTS = {
    "m1-abort-before-start": ("  const opened: GestureToolResult", "  if (interrupted !== undefined) resumed?.abort(interrupted);\n  const opened: GestureToolResult"),
    "m2-verb-before-base": ('resumed === undefined ? undefined : moved ? "baseMoved" : resumed.verb !== verb', 'resumed === undefined ? undefined : resumed.verb !== verb ? "captureLost" : moved ? "baseMoved" : resumed.verb !== verb'),
    "m3-unrestorable-refuses": ("  const resumed = restored?.ok", '  if (restored?.ok === false && (phase.kind === "stream" || phase.kind === "commit")) return { ok: false, refusal: restored.refusal };\n  const resumed = restored?.ok'),
    "m4-same-ignored": ('persisted !== undefined && kind.same(gesture, persisted) ? { kind: "unchanged" }', 'false ? { kind: "unchanged" }'),
    "m5-refused-tick-clears": ("  if (!sent.ok) return { ok: false, refusal: sent.refusal };", "  if (!sent.ok) return persisted === undefined ? { ok: false, refusal: sent.refusal } : dropped;"),
}


def main():
    twin = (MODULE / "🟦️.ts").read_text(encoding="utf-8")
    law = (MODULE / "🧪️tests/🧪️gesture-drive-law/🟦️.ts").read_text(encoding="utf-8")
    law = law.replace('from "../../🟦️.ts"', 'from "./twin.ts"').replace('"../../🧫️fixtures/', f'"{MODULE}/🧫️fixtures/').replace('"../../🧬️schema/', f'"{MODULE}/🧬️schema/')
    survivors = []
    for name, (old, new) in MUTANTS.items():
        if twin.count(old) != 1:
            raise SystemExit(f"{name}: the mutation site moved")
        folder = OUT / name
        folder.mkdir(parents=True, exist_ok=True)
        (folder / "twin.ts").write_text(twin.replace(old, new), encoding="utf-8")
        (folder / "law.test.ts").write_text(law, encoding="utf-8")
        run = subprocess.run(["bun", "test", str(folder / "law.test.ts")], cwd=REPO, capture_output=True, text=True)
        tally = [line.strip() for line in (run.stdout + run.stderr).splitlines() if line.strip().endswith(("pass", "fail")) and line.strip()[0].isdigit()]
        print(f"{name}: {' '.join(tally)}")
        if run.returncode == 0:
            survivors.append(name)
    if survivors:
        raise SystemExit(f"surviving mutants: {survivors}")
    print(f"all {len(MUTANTS)} mutants killed")


if __name__ == "__main__":
    sys.exit(main())
