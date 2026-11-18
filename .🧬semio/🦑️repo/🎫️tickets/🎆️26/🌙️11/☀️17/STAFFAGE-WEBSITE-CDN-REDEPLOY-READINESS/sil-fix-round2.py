"""Silhouette repairs round 2 (bar-stool 1:200 legs follow the drawn foot at 0.21 m)."""
import importlib.util
spec = importlib.util.spec_from_file_location('s', 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
d = s.Doc('🛋️furniture/bar-stool')
d.rep(s.child(d.find('200', 'leg'), 'p1'), '[0.20875, 0.0125, 0.0]')
d.commit()
