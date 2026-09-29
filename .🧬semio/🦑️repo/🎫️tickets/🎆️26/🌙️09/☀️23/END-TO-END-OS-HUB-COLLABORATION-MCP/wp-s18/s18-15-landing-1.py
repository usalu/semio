"""🧾️ S18 §15: landing row for the actor-sections set (host TS only) under `# Session 15` in `📓️landing.md`."""
from pathlib import Path

p = Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
t = p.read_text(encoding="utf-8")
head = "# Session 15\n\n| Slice | Set | Files | Crates / checks | Result + capture | Time |\n|---|---|---|---|---|---|\n"
assert t.count(head) == 1
marker = "| S18 | 14c→15 (C13 P1"
if marker in t:
    raise SystemExit("already")
start = t.index(head) + len(head)
end = t.find("\n\n", start)
end = len(t) if end == -1 else end
row = (
    "| S18 | 14c→15 (C13 P1, approved 17:36): an actor-bound hub document's engagements / measures / tool measures / catalogue come from the "
    "browser actor's reserved section surfaces (they froze at the opening because ShellHost read them from the LOCAL instance) — worker "
    "visible-surface contract `🪟️visible-surfaces` (live sections on every turn, catalogue at mount), shared decoder `retainedSectionValueV1`, "
    "ShellHelpers section stores + `sectionsChanged` + `browserActorSectionValuesV1` + `withoutUiRefreshSectionsV1`, ShellHost "
    "`publishUiRefreshSectionsV1` (actor sections win; the local refresh no longer fetches them for an actor-served session). Applied "
    "17:46–18:14 by the predecessor (codemods `wp-s18/s18-14c-actor-sections{,-corpus,-worker-law}.py`); **host TS only, no Rust** "
    "| `💻️os/🔨️modules/🏪️store/👷️worker/{🟦️.ts,🪟️visible-surfaces/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}}`, "
    "`📺️renderer/…/🧱️elements/{🔌️PluginRuntime,🛠️ShellHelpers,🏛️ShellHost}/🟦️.tsx`, `🛠️ShellHelpers/{🧫️fixtures,🧪️tests}/🎭️browser-actor-panels`, "
    "`💻️os/🧪️tests/{🪟️visible-surfaces,🧪️space-artifact-creation-owner,🎚️config}/🟦️.ts` "
    "| tsc (`wp-s18/tsc/tsconfig-14c.json`, 31 files) rc 0, 81 s; boot 6540 → Home 14 s, 0 pageerrors; served modules carry the set "
    "| `.🧬semio/🌐hub/s14-s18-captures/s18-15-{tsc-reconcile-1,boot-1}.txt`; laws QUEUED native (`s14-s18-logs/vitest-actor-sections-15-1.txt`); live proof pending "
    "| 19:3x (applied 17:46–18:14) |"
)
p.write_text(t[:end] + "\n" + row + t[end:], encoding="utf-8")
print("ok")
