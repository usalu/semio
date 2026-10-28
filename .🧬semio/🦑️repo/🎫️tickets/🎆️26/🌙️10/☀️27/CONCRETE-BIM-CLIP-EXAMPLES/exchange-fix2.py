"""🩹 Make every beat remove what the previous beat added through an explicit list."""
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py"
text = target.read_text(encoding="utf-8")
OFF = "*[_engine.FadeOut(m) for m in s.off]"
pairs = [
    ("self._s = SimpleNamespace(context=_engine.VGroup(context, purpose, target), strip=strip, kept=kept)", "self._s = SimpleNamespace(off=[context, purpose, target, strip, kept])"),
    ("_seq((_engine.FadeOut(s.context), _engine.FadeOut(s.strip), _engine.FadeOut(s.kept), _in(plan), _engine.FadeIn(table)),", "_seq((" + OFF + ", _in(plan), _engine.FadeIn(table)),"),
    ("self._s = SimpleNamespace(table=table, view=view, guids=_engine.VGroup(chip_slab, chip_b3, links))", "self._s = SimpleNamespace(off=[table, view, chip_slab, chip_b3, links, note])"),
    ("_seq((_engine.FadeOut(s.table), _engine.FadeOut(s.view), _engine.FadeOut(s.guids), _in(strip), _engine.FadeIn(table),", "_seq((" + OFF + ", _in(strip), _engine.FadeIn(table),"),
    ("self._s = SimpleNamespace(table=table, strip=strip, arrows=arrows, total=total)", "self._s = SimpleNamespace(off=[table, strip, arrows, total])"),
    ("_seq((_engine.FadeOut(s.table), _engine.FadeOut(s.strip), _engine.FadeOut(s.arrows), _engine.FadeOut(s.total), _engine.FadeIn(view),", "_seq((" + OFF + ", _engine.FadeIn(view),"),
    ("self._s = SimpleNamespace(view=_engine.VGroup(view, arrows, estimate, check, *notes, *forces))", "self._s = SimpleNamespace(off=[view, arrows, estimate, check, *notes, *forces])"),
    ("_seq((_engine.FadeOut(s.view), _engine.FadeIn(full), _engine.FadeIn(target), _engine.Create(forward)),", "_seq((" + OFF + ", _engine.FadeIn(full), _engine.FadeIn(target), _engine.Create(forward)),"),
    ("self._s = SimpleNamespace(left=_engine.VGroup(ghosted, chips), right=target, arrows=_engine.VGroup(forward, back, cross, reverse))", "self._s = SimpleNamespace(off=[ghosted, chips, target, forward, back, cross, reverse])"),
    ("_seq((_engine.FadeOut(s.left), _engine.FadeOut(s.right), _engine.FadeOut(s.arrows), _engine.FadeIn(view), _engine.FadeIn(box)),", "_seq((" + OFF + ", _engine.FadeIn(view), _engine.FadeIn(box)),"),
    ("self._s = SimpleNamespace(view=_engine.VGroup(view, box, leads, own, note))", "self._s = SimpleNamespace(off=[view, box, leads, own, note])"),
    ("_seq((_engine.FadeOut(s.view), _in(plan), _engine.FadeIn(card), _engine.FadeIn(rings), _engine.FadeIn(counter)),", "_seq((" + OFF + ", _in(plan), _engine.FadeIn(card), _engine.FadeIn(rings), _engine.FadeIn(counter)),"),
]
for old, new in pairs:
    assert old in text, old[:60]
    text = text.replace(old, new)
target.write_text(text, encoding="utf-8")
print("[DEBUG] fix2 applied")
