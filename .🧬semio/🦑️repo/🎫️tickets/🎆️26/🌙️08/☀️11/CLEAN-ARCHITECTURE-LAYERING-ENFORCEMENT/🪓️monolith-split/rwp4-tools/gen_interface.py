# 📣️ WP4 interface publisher: writes the component interface section of RESULT-RENDERER-WP4.md from the frozen plan tables.
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
import json as _j, collections as _c
_P = _j.load(open(HERE + '/../🗑️generated/rwp4-t1-plan.json'))
_cnt = _c.Counter(v['struct'] for v in _P.values() if v['tier1'] and v.get('recv') in ('mut','ref'))
comps = components()
ab = appendix_b()
rank = ["shell-chrome-state","shell-surfaces","shell-plugins","shell-history","shell-prefs","shell-directory","shell-sync","shell-docs","shell-doc-open","shell-windows","shell-dock","shell-input","shell-commands"]
out = []
out.append("## 2. Published component interface (frozen; WP5 codes against this)\n")
out.append("Source of truth: plan section 7.3 (fields, unchanged names), Appendix B (method homing), Appendix F (crate DAG). Field names do not change: `state.dock.dock_tabs`, `state.prefs.locale_id`. The composer `ShellState` (crate `shell-state`) is exactly the 15 component structs as `pub` fields, named by the snake-case field below, plus `Drop`.\n")
out.append("| composer field | struct | crate | rank | fields | methods of the struct |")
out.append("|---|---|---|---:|---:|---:|")
names = {"ChromeState":"chrome","SurfaceState":"surfaces","TutorialState":"tutorial","SettleState":"settle","PluginState":"plugins","HistoryState":"history","PrefsState":"prefs","DirectoryState":"directory","SyncState":"sync","DocumentState":"docs","OpeningState":"opening","WindowState":"windows","ShellDockState":"dock","ShellInputState":"input","CommandState":"commands"}
for i, c in enumerate(comps):
    out.append(f"| `{names[c['struct']]}` | `{c['struct']}` | `{c['crate']}` | {rank.index(c['crate'])} | {len(c['fields'])} | {_cnt[c['struct']]} |")
out.append("")
out.append("Fields per component:\n")
for c in comps:
    out.append(f"* `{names[c['struct']]}: {c['struct']}` ({c['crate']}): " + ", ".join(f"`{f}`" for f in c['fields']))
open(HERE + "/../🗑️generated/rwp4-interface.md", "w").write("\n".join(out) + "\n")
json.dump(dict(components=comps, names=names, rank=rank), open(HERE + "/../🗑️generated/rwp4-components.json", "w"), ensure_ascii=False, indent=1)
print("\n".join(out)[:1500])
