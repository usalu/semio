import sys
sys.path.insert(0, sys.argv[2])
from edit_py_helper import edit
p = sys.argv[1]
edit(p, [
('_INDEXES: dict[Path, dict] = {}\n', '_INDEXES: dict[Path, dict] = {}\n_CHECKED: set[Path] = set()\n'),
('def asset_revision(slug', '@lru_cache(maxsize=None)\ndef asset_revision(slug'),
('        _provenance(cast, _index(folder))\n', '        if folder not in _CHECKED:\n            _provenance(cast, _index(folder))\n            _CHECKED.add(folder)\n'),
('''    folder = ensure_casts([cast])[cast]
    group = _place(folder, 0, unit, anchor, color, opacity, mirror)
    group = group if window is None else _crop(group, unit, anchor, window)
    return _Figure(group, folder, unit, anchor, mirror) if window is None and "figureBoundsM" in _index(folder) else group
''', '''    folder = ensure_casts([cast])[cast]
    group = _place(folder, 0, unit, anchor, color, opacity, mirror)
    group = group if window is None else _crop(group, unit, anchor, window)
    group = _Figure(group, folder, unit, anchor, mirror) if window is None and "figureBoundsM" in _index(folder) else group
    group.provenance = _drawn(cast, None)
    return group
'''),
('''    return _Motion(ensure_casts([cast])[cast], unit, anchor, rate, color, window, mirror)''',
'''    motion = _Motion(ensure_casts([cast])[cast], unit, anchor, rate, color, window, mirror)
    motion.provenance = _drawn(cast, rate)
    return motion'''),
])
