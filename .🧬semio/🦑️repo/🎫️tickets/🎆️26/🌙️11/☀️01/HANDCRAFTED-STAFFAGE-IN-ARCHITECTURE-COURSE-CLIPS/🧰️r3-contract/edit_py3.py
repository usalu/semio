import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
V = sys.argv[2]
edit(V + "/🧍️staffage/🐍️.py", [
('''_DRAWN: list[dict] = []
''', '''_DRAWN: list[dict] = []
_RECORDING = [True]


@contextlib.contextmanager
def unrecorded():
    """🙈 Draw casts for measuring only: inside this block they are not listed by `drawn_casts`."""
    _RECORDING.append(False)
    try:
        yield
    finally:
        _RECORDING.pop()
'''),
('''    record = staffage_provenance(cast, rate)
    _DRAWN.append(''', '''    record = staffage_provenance(cast, rate)
    if _RECORDING[-1]:
        _DRAWN.append('''),
])
edit(V + "/🐍️.py", [
('''ensure_casts = _staffage.ensure_casts
''', '''ensure_casts = _staffage.ensure_casts
staffage_provenance = _staffage.staffage_provenance
drawn_casts = _staffage.drawn_casts
clear_drawn_casts = _staffage.clear_drawn_casts
'''),
('''    probe = staffage(cast, unit=1.0)
''', '''    with _staffage.unrecorded():
        probe = staffage(cast, unit=1.0)
'''),
])
