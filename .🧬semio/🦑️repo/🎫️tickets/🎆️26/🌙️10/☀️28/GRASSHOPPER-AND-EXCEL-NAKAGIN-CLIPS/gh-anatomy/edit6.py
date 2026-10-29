import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep("import importlib\nimport re", "import dataclasses\nimport importlib\nimport re")
rep("        full = _gh.evaluate(defn)\n", "        full = _gh.evaluate(defn)\n        shown = dataclasses.replace(full, messages={k: tuple({**m, \"de\": m[\"en\"]} for m in v) for k, v in full.messages.items()})\n")
rep('warning = self._sheet_view(defn, full, "warning", (0.0, 0.0))', 'warning = self._sheet_view(defn, shown, "warning", (0.0, 0.0))')
rep('error = self._sheet_view(defn, full, "error", (0.0, 0.0))', 'error = self._sheet_view(defn, shown, "error", (0.0, 0.0))')
rep('culprit.get_bottom() + np.array([0.0, -0.22, 0.0])', 'culprit.get_right() + np.array([0.75, 0.0, 0.0])')
rep('at=_at(1.0, 1.5))\n        wp_wire', 'at=_at(1.0, 1.38))\n        wp_wire')
rep('buff=0.08, color="#2E8B2E", stroke_width=3)\n        self.step("panel"', 'buff=0.04, color="#2E8B2E", stroke_width=3)\n        self.step("panel"')
open(p, "w", encoding="utf-8").write(s)
