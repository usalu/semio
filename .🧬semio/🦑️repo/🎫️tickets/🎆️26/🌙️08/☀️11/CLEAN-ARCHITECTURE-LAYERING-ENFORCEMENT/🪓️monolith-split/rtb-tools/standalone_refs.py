"""For each include!d standalone test host: its top-level items and the other test files that name them."""
import sys
from lib import *
hosts = {
    "renderer-standalone": E + "/🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs",
    "present-stall": E + "/🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs",
    "os-host-standalone": E + "/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs",
    "scenes-standalone": EL + "/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs",
    "engine-standalone": EL + "/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs",
}
allsrc = [REPO + "/" + p for p in test_sources()]
tok = {p: tokens(read(p)) for p in allsrc if os.path.exists(p)}
for name, hp in hosts.items():
    text = read(hp)
    tbl = [i for i in itm.table(text) if i["kind"] not in ("impl", "use")]
    print("=====", name, len(tbl), "items")
    for it in tbl:
        users = sorted({os.path.relpath(p, E).split("/🧪️tests/")[-1][:40] if "🧪️tests" in p else os.path.relpath(p, E)[-40:] for p, ts in tok.items() if p != hp and it["name"] in ts})
        if it["kind"] == "fn" and "test" in " ".join(it["attrs"]) and "#[test]" in it["attrs"]:
            continue
        print(f"  {it['kind']:7} {it['name']:55} {it['end']-it['start']+1:4}l users={len(users)} {users[:6] if users else ''}")
