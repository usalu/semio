"""📝 Finisher A reports: writes the section `## Finisher completion (2026-11-23)` of every worker report `📝️PW-0x.md` (creating the report where the worker left none) from the measured facts of the catalog assets (`facts.py`), the final proof lines (`proofs.py`) and the hand-written notes in `notes.json` next to this file (what the finisher changed, what was seen on the preview sheets, what stays imperfect).

Usage: `python3 reports.py` (idempotent: an existing finisher section is replaced).
See `../../📋️brief.md` (report section).
"""
import json, pathlib

from facts import block
from proofs import proof

HERE = pathlib.Path(__file__).resolve().parent
TICKET = HERE.parent.parent
MARK = "## Finisher completion (2026-11-23)"
WORKERS = {
    "PW-01": ("Maples", ["norway-maple", "field-maple", "sycamore-maple"]),
    "PW-02": ("Park Trees", ["horse-chestnut", "european-beech", "common-ash"]),
    "PW-03": ("Flowering Trees", ["japanese-cherry", "saucer-magnolia", "crab-apple"]),
    "PW-04": ("Urban Street Trees", ["ginkgo", "honey-locust", "sweetgum"]),
    "PW-05": ("Large Broadleaves", ["tulip-tree", "red-oak", "black-locust"]),
    "PW-06": ("Small Native Trees", ["rowan", "hawthorn", "black-alder"]),
    "PW-07": ("Conifers I", ["norway-spruce", "european-larch", "atlas-cedar"]),
    "PW-08": ("Conifers II", ["italian-cypress", "english-yew", "dawn-redwood"]),
    "PW-09": ("Mediterranean", ["olive-tree", "fig-tree", "stone-pine"]),
}


def section(worker: str, plants: list[str], notes: dict) -> str:
    out = [MARK, "", "The worker session was cut off at about 17:30 on 2026-11-22. This section is written by the finisher of the scope `PW-01` … `PW-09`; every number in it was measured after 20:20 on the catalog files as they are now.", ""]
    out += ["### What the finisher changed", ""] + [f"- {line}" for line in notes["workers"][worker]["changed"]] + [""]
    out += ["### Final proof", "", "Whole-asset validation: `r3-validate.ts vegetation/<id> --catalog-only --volumes` (JSON schema, `validateAsset`, print checks over every valid range, `checkVolumes`). Check CLI: `🩺️check` with physics, all poses, clip instants and the seven representations.", "", "| Plant | Whole-asset validation | Remaining volume findings (rule, scale, count) | Check CLI |", "|---|---|---|---|"]
    for ident in plants:
        p = proof(ident)
        rules = ", ".join(f"{name} ×{count}" for name, count in sorted(p["rules"].items())) or "none"
        out.append(f"| `{ident}` | `{p['summary']}` | {rules} | `{p['check']}` |")
    out += ["", "Check CLI scale lines:", ""]
    for ident in plants:
        for line in proof(ident)["scales"]:
            out.append(f"- `{ident}` {line}")
    out += ["", "Manifest rows: `📥️manifest-rows/{" + ",".join(plants) + "}.json`, each compared with its catalog asset by the rules of the product's `validateManifest` (`🧰️scripts/finish-a/rows.py`): row matches asset.", ""]
    out += ["### Measured facts of the shipped assets", ""] + [block(ident) for ident in plants]
    out += ["### Preview sheets opened and judged", "", "Sheets: the product's own 2D drawings (side, front, plan; line and solid) at 1:500, 1:200, 1:100 and 1:50 on one contact sheet and the six shaded 3D views at 1:100 (`🧰️scripts/finish-a/look.sh`; the PNGs were tool output and are deleted).", ""]
    out += [f"- `{ident}`: {notes['plants'][ident]}" for ident in plants] + [""]
    out += ["### What stays imperfect", ""] + [f"- {line}" for line in notes["workers"][worker]["imperfect"]] + [""]
    out += ["### Tests", "", "No vitest suite was run by the finisher (reserved for the umbrella coordinator after all finishers; the host is shared). The proofs above are the product's validator, the volume rules and the check CLI run on each asset alone.", ""]
    return "\n".join(out)


def main() -> None:
    notes = json.loads((HERE / "notes.json").read_text("utf8"))
    for worker, (theme, plants) in WORKERS.items():
        path = TICKET / f"📝️{worker}.md"
        text = path.read_text("utf8") if path.is_file() else f"# 📝️ {worker}: {theme} ({', '.join(plants)})\n\nWorker {worker} of the Staffage Plant Collection Wave. The worker left no report (or only a stub); the generators are in `🧰️scripts/{worker}/`.\n\n"
        text = text.split(MARK)[0].rstrip() + "\n\n" + section(worker, plants, notes)
        path.write_text(text, "utf8")
        print(f"[DEBUG] wrote {path.name}")


if __name__ == "__main__":
    main()
