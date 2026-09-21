"""🏷️ Give every `next_server_frame` wait inside one named law its own phase label."""
import io, re, sys

PATH = "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
LAW = "fn admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen()"
PHASES = [
    "member session frame after the sync-hello bootstrap drain",
    "observer welcome",
    "observer session frame after the sync-hello bootstrap drain",
    "observer normalized presence of the member",
    "member echo of its own normalized presence",
    "observer withdrawal presence after the admin removal",
    "observer presence after the removal, still live",
]

src = io.open(PATH, encoding="utf-8").read()
start = src.index(LAW)
end = src.index("\n#[test]\n", start)
body = src[start:end]
hits = list(re.finditer(r"next_server_frame\(&mut ([bc])\)", body))
if len(hits) != len(PHASES):
    sys.exit(f"expected {len(PHASES)} waits in the law, found {len(hits)}")
out, cursor = [], 0
for hit, phase in zip(hits, PHASES):
    out.append(body[cursor:hit.start()])
    out.append(f'next_server_frame_at(&mut {hit.group(1)}, "{phase}")')
    cursor = hit.end()
out.append(body[cursor:])
io.open(PATH, "w", encoding="utf-8").write(src[:start] + "".join(out) + src[end:])
print(f"named {len(hits)} waits")
