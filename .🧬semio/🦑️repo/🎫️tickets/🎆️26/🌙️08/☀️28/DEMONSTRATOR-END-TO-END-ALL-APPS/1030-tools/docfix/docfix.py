#!/usr/bin/env python3
"""🔧 Assigns unique meaning-based leading emoji to duplicate docstrings (docstring-only edits)."""

import collections
import os
import re
import sys
import time
import unicodedata

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "docaudit"))
import docaudit as da

ROOT = da.ROOT
FE0F = "️"
QUIET_S = 20 * 60

KEYS = """cache:🗄 hash:#️⃣|🔑 digest:🧮 test:🧪 parse:🔍 error:🚨 fault:💥 fail:❌ config:⚙ setting:🎚 theme:🎨 color:🌈 colour:🌈 text:📝 string:🧵 number:🔢 time:⏱ clock:🕐 date:📅 list:📋 map:🗺 tree:🌳 graph:🕸 node:🔵 edge:🔗 id:🪪 identity:🪪 key:🗝 lock:🔒 unlock:🔓 path:🛤 file:📄 folder:📁 dir:📂 net:🌐 http:🌍 url:🔗 event:📣 command:🎮 query:❓ state:🔄 schema:🧬 render:🖼 draw:✏ paint:🖌 mesh:🕸 shape:🔷 vertex:📍 matrix:🔲 vector:➡ audio:🔊 wav:🎵 sound:🔉 image:🖼 pixel:🟦 video:🎬 camera:📷 light:💡 material:🧱 texture:🪟 undo:↩ redo:↪ history:📜 select:👆 hover:🖱 drag:✋ scroll:📜 zoom:🔍 grid:▦ table:📊 row:➖ column:🏛 button:🔘 menu:🍔 panel:🗂 window:🪟 dialog:💬 input:⌨ keyboard:⌨ mouse:🐭 touch:👉 language:🗣 locale:🌐 access:♿ progress:📈 cancel:🛑 job:💼 task:✅ worker:👷 thread:🧶 queue:🚏 batch:📦 stream:🌊 buffer:🪣 memory:🧠 arena:🏟 pool:🏊 alloc:📐 size:📏 limit:🚧 count:🔢 sum:➕ sort:↕ filter:🔎 search:🔭 find:🔦 diff:🔀 patch:🩹 merge:🧩 copy:📑 move:🚚 delete:🗑 remove:🧹 insert:📥 add:➕ create:🆕 new:✨ build:🏗 load:📤 save:💾 export:📤 import:📥 encode:🔐 decode:🔓 serial:📼 pack:🎁 zip:🤐 version:🔖 law:⚖ rule:📏 policy:📜 kind:🏷 type:🔤 trait:🧬 plugin:🔌 app:📱 host:🏠 guest:🧳 wasm:🕸 bridge:🌉 adapter:🔌 bind:🪢 registry:📒 catalog:📚 manifest:🧾 descriptor:🪪 profile:👤 user:🙋 session:🎟 auth:🛂 token:🎫 permission:🛡 secure:🔐 safe:🦺 unsafe:☢ transmute:🦎 repr:🧱 float:🎈 int:🔟 bool:🔀 byte:🧊 bit:⚛ utf:🔡 char:🔠 unicode:🔣 emoji:😀 label:🏷 title:📰 name:📛 icon:🪧 sprite:👾 font:🔤 glyph:🅰 layout:📐 flex:🤸 align:📏 margin:🔳 border:🔲 shadow:🌑 blur:🌫 opacity:👻 animate:🎞 transition:🎢 frame:🎞 tick:⏲ timer:⏳ delay:🐢 debounce:🛗 throttle:🚦 retry:🔁 loop:♻ iter:🎠 cursor:🖱 range:↔ span:📏 slice:🍕 split:✂ join:🤝 concat:⛓ trim:✂ replace:🔄 format:🧾 print:🖨 log:🪵 trace:🔬 metric:📉 stat:📊 bench:🏋 perf:🚀 fast:⚡ slow:🐌 cold:🧊 hot:🔥 warm:♨ fresh:🌱 stale:🥀 clean:🧼 dirty:🧽 flush:🚽 drain:🚰 fill:🪣 empty:🫙 full:🈵 open:📂 close:🚪 start:▶ stop:⏹ pause:⏸ resume:⏯ run:🏃 exec:⚡ spawn:🥚 kill:🔪 wait:⌛ notify:🔔 signal:📡 channel:📺 message:✉ send:📨 recv:📬 reply:💌 request:🙏 response:🤝 route:🧭 dispatch:🚀 handle:🤲 handler:🎛 hook:🪝 callback:📞 observer:👀 watch:⌚ subscribe:🔔 publish:📢 emit:📤 apply:🖇 reduce:📉 project:🗂 view:👁 model:🧩 controller:🕹 store:🏪 repo:🗃 db:🛢 sql:🗄 sqlite:🪶 row:🧾 index:🔖 offset:↗ length:📏 width:↔ height:↕ depth:🕳 area:🔶 volume:🧊 angle:📐 rotate:🔃 scale:⚖ translate:🧭 transform:🪄 pose:🕴 camera:🎥 scene:🎭 world:🌍 body:🧍 physics:🪐 force:💪 mass:🏋 collision:💢 ray:🔦 hit:🎯 pick:⛏ snap:🧲 gizmo:🎛 gumball:🍬 tool:🛠 brush:🖌 pen:🖊 eraser:🧽 stroke:〰 fill:🪣 curve:➰ spline:🎢 surface:🏞 solid:🧊 face:🙂 boolean:🔀 union:🔗 cut:✂ extrude:🏗 sweep:🧹 loft:🛫 fillet:🌀 chamfer:📐 step:👣 iges:🏭 cad:📐 fem:🧮 solver:🧠 mesh:🕸 element:🔹 stiffness:🪨 load:🏋 constraint:⛓ energy:🔋 weather:⛅ solar:☀ wind:🌬 heat:🌡 thermal:🌡 sim:🎲 random:🎰 seed:🌱 noise:📻 wfc:🀄 puzzle:🧩 piece:♟ forms:📋 form:📝 field:🌾 note:🗒 draw:🎨 raster:🟫 layer:🥞 brush:🖌 print:🖨 tex:🧾 pdf:📕 doc:📄 markdown:Ⓜ writer:✒ page:📃 paragraph:¶ heading:🔠 link:🔗 fold:🗜 typing:⌨ splice:🧬 editor:📝 play:🎮 demo:🎪 hub:🛞 publish:📰 trust:🤝 sign:✍ verify:✔ check:☑ assert:‼ prove:🧷 law:⚖ oracle:🔮 fixture:🪑 golden:🥇 snapshot:📸 baseline:📏 probe:🩺 harness:🪢 mock:🎭 stub:🪵 fake:🃏 helper:🆘 util:🧰 common:🤲 core:🎯 base:🧱 root:🌱 leaf:🍃 child:👶 parent:👪 sibling:👫 owner:👑 borrow:🤲 lease:📃 share:🔗 shared:🫂 local:🏡 remote:🛰 sync:🔄 async:⏳ await:🧘 future:🔮 poll:🗳 pump:⛽ reactor:⚛ turn:🔁 round:⭕ phase:🌓 stage:🎭 pipeline:🚰 flow:🌊 gate:🚧 guard:💂 policy:🧑‍⚖ budget:💰 cost:💲 price:🏷 quota:🍰 cap:🧢 capacity:🧳 ceiling:🏠 floor:🪜 min:🔽 max:🔼 default:🏁 option:🔘 choice:🔀 variant:🎭 enum:🔢 struct:🏛 union:🇪🇺 field:🏞 member:🧑 attr:🏷 derive:🧬 macro:🪄 generic:🧰 lifetime:⏳ reference:🔖 pointer:👈 ptr:👈 box:📦 arc:🌈 rc:🧮 cell:🔋 mutex:🔐 atomic:⚛ ordering:🔢 sealed:🏺 seal:🦭 immutable:🧊 mutable:🌡 clone:🐑 drop:💧 free:🆓 leak:🚰 retire:🛌 release:🎉 acquire:🎣 reserve:🔖 grant:🎁 deny:🙅 admit:🚪 refuse:⛔ reject:🚫 accept:🙆 allow:✅ block:🧱 wake:⏰ sleep:😴 idle:🛋 busy:🐝 live:💚 dead:💀 zombie:🧟 ghost:👻 phantom:🫥 orphan:🪹 stray:🐕 lost:🧭 found:🏆 missing:❔ unknown:❓ invalid:🙅 valid:✔ ok:🆗 done:🏁 ready:🟢 pending:🟡 queued:🚏 running:🏃 settled:⚓ stalled:🛑 wedged:🪤 stuck:🧲 hang:🪂 timeout:⏰ deadline:📆 expire:⌛ ttl:⏳ age:🧓 epoch:🕰 era:🏺 tick:✔ beat:💓 pulse:💗 heart:❤ ping:🏓 pong:🏓 echo:🔁 mirror:🪞 reflect:🪞 project:📽 proj:📽 window:🪟 doc:📚 """

def build_keys():
    m = []
    for tok in KEYS.split():
        k, _, v = tok.partition(":")
        if not v:
            continue
        cands = [c for c in v.split("|")]
        m.append((k, cands))
    return m

KEYMAP = build_keys()
POOL = []
for r in [(0x1F300, 0x1F64F), (0x1F680, 0x1F6D7), (0x1F90C, 0x1F9FF), (0x1FA70, 0x1FAF8)]:
    for c in range(r[0], r[1] + 1):
        if 0x1F3FB <= c <= 0x1F3FF or 0x1F1E6 <= c <= 0x1F1FF or c in (0x1F3F3, 0x1F3F4):
            continue
        if unicodedata.name(chr(c), None):
            POOL.append(chr(c))
MISC = [chr(c) for c in range(0x2600, 0x27BF) if unicodedata.name(chr(c), None) and da.is_emoji(chr(c))]
POOL += MISC


def nk(e):
    return e.replace(FE0F, "").replace("︎", "")


def cand_for(text, used):
    toks = re.findall(r"[a-z]+", text.lower())
    for t in toks:
        for k, cs in KEYMAP:
            if t.startswith(k) and len(t) <= len(k) + 5:
                for c in cs:
                    if c and nk(c) not in used and da.is_emoji(c[0]):
                        return c
    return None


def fix_file(rel):
    full = os.path.join(ROOT, rel)
    if time.time() - os.path.getmtime(full) < QUIET_S:
        return "skip-recent", 0
    mt = os.path.getmtime(full)
    with open(full, encoding="utf-8", newline="") as fh:
        content = fh.read()
    lines = content.split("\n")
    ext = os.path.splitext(rel)[1]
    if ext == ".py":
        docs, _ = da.py_scan(lines)
    else:
        docs = da.rust_or_ts_docs(lines, ext)
    leads = {}
    for ln, text in docs:
        lead = da.lead_emoji(text)
        if lead:
            leads[nk(lead)] = leads.get(nk(lead), 0) + 1
    used = set(leads)
    seen = set()
    changed = 0
    pool_i = 0
    for ln, text in docs:
        lead = da.lead_emoji(text)
        if not lead:
            continue
        key = nk(lead)
        if key not in seen:
            seen.add(key)
            continue
        body = text.strip()[len(lead):].strip() if text.strip().startswith(lead) else text
        new = cand_for(body, used)
        if new is None:
            while pool_i < len(POOL) and nk(POOL[pool_i]) in used:
                pool_i += 1
            if pool_i >= len(POOL):
                return "pool-exhausted", changed
            new = POOL[pool_i]
        if FE0F not in new and not new.endswith(FE0F) and len(new) == 1:
            new += FE0F
        used.add(nk(new))
        done = False
        for j in range(ln - 1, min(ln + 3, len(lines))):
            if lead in lines[j]:
                lines[j] = lines[j].replace(lead, new, 1)
                done = True
                break
        if done:
            changed += 1
    if not changed:
        return "nochange", 0
    if os.path.getmtime(full) != mt:
        return "raced", 0
    with open(full, "w", encoding="utf-8", newline="") as fh:
        fh.write("\n".join(lines))
    return "ok", changed


def main():
    files = sys.argv[1:]
    tot = collections.Counter()
    for rel in files:
        st, n = fix_file(rel)
        tot[st] += 1
        tot["edits"] += n
        if st != "ok" and st != "nochange":
            print(st, rel)
    print(dict(tot))

main()
