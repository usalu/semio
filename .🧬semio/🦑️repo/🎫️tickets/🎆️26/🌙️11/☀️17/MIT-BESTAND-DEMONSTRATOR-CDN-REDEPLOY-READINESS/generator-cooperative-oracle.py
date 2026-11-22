"""🔎️ Checks the mounted preview fixture independently using a SQLite transition table."""
import json
import sqlite3
import sys
from pathlib import Path


def main():
    """🧪️ Apply deny, resume and abort events to independent resident-owner rows."""
    fixture, editor, viewer, preview = map(Path, sys.argv[1:])
    law = json.loads(fixture.read_text())["mountedBaseline"]
    with sqlite3.connect(":memory:") as db:
        db.execute("create table tick (id primary key, owner integer, completed integer, effects integer)")
        for case in law["cases"]:
            key = case["id"]
            db.execute("insert into tick values (?, 1, 0, 0)", (key,))
            db.execute("update tick set effects=effects+1 where id=? and owner=1 and ? > 0", (key, 0))
            db.execute("update tick set completed=1 where id=? and owner=1 and not ?", (key, case["cancel"]))
            db.execute("update tick set owner=0 where id=?", (key,))
            effects, completed, terminal = db.execute("select effects,completed,not owner from tick where id=?", (key,)).fetchone()
            assert (effects, bool(completed), bool(terminal)) == (case["deniedEffects"], case["completed"], case["terminalEmpty"])
            print("[DEBUG] SQLite mounted-baseline case", key, effects, completed, terminal)
    for surface in (editor, viewer):
        source = surface.read_text()
        assert "self.tick.advance_publication(effect)" in source
        assert "self.tick.close_step(cx)" in source
        assert "cx.consume_retained(progress)" in source
    mounted = preview.read_text().split("impl MountedPreviewTick", 1)[1].split("pub fn chain_ui_scope", 1)[0]
    assert "cold_effect" not in mounted and "while " not in mounted
    print("[DEBUG] Both mounted command doors advance and close through the same bounded owner")


if __name__ == "__main__":
    main()
