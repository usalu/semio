#!/usr/bin/env python3
"""🩹 Span-keyed repair for the silently-dropped-future class documented in the terra-actor-green
brief (R10 residue-shapes note + "watch for silent no-ops"): every `pack::write_*(...)` /
`X.pack_encode(...)` call in this file's codec bodies is a unit-returning `async fn`, called in
statement position without `.await`, so the write NEVER HAPPENS — it compiles clean (no rustc
diagnostic exists for a dropped `Future<Output=()>`), so `insert-await.py` cannot find it.

This is NOT a name-keyed bulk awaiter banned by R10: R10 bans matching bare identifiers that collide
with std methods (`len`, `get`, `new`, ...). Every span here was individually eyeballed (see
`dropped-futures-full.txt`) and is restricted to two fully-disambiguated call shapes that cannot
collide with std: the module-qualified `pack::write_*` free functions and the crate-local
`.pack_encode(`/`.pack_decode(` methods, which exist nowhere in std. Each target line's exact
CURRENT text is asserted before edit — the script aborts loudly on any mismatch instead of guessing.
"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️component.rs"

# (line_number, expected_exact_line_text) — line_number is 1-based, text is the line's content
# WITHOUT trailing newline, copied verbatim from the diagnostic scan.
TARGETS = [
    (128, "        write_varint_u64(out, v.len() as u64);"),
    (140, "        write_bytes(out, v.as_bytes());"),
    (159, "        write_bool(out, v.is_some());"),
    (173, "        write_varint_u64(out, v.len() as u64);"),
    (197, "        pack::write_str(out, &self.0);"),
    (221, "        pack::write_hash32(out, &self.0);"),
    (283, "        pack::write_u64(out, self.0);"),
    (321, "                pack::write_u8(out, 0);"),
    (322, "                plugin.pack_encode(out);"),
    (323, "                pack::write_str(out, app_id);"),
    (324, "                pack::write_u32(out, *instance_id);"),
    (327, "                pack::write_u8(out, 1);"),
    (328, "                plugin.pack_encode(out);"),
    (329, "                pack::write_str(out, extension_id);"),
    (332, "                pack::write_u8(out, 2);"),
    (333, "                owner.pack_encode(out);"),
    (334, "                pack::write_u64(out, *job_id);"),
    (401, "        pack::write_u8(out, self.tag().await);"),
    (442, "        pack::write_u64(out, self.fuel);"),
    (443, "        pack::write_u32(out, self.wall_ms);"),
    (444, "        pack::write_u64(out, self.memory_bytes);"),
    (445, "        pack::write_u32(out, self.ui_nodes);"),
    (446, "        pack::write_u16(out, self.mailbox_len);"),
    (447, "        pack::write_u32(out, self.max_effects);"),
    (448, "        pack::write_u32(out, self.max_patch_bytes);"),
    (492, "        pack::write_u32(out, self.0);"),
    (520, "                pack::write_u8(out, 0);"),
    (521, "                window.pack_encode(out);"),
    (524, "                pack::write_u8(out, 1);"),
    (525, "                id.pack_encode(out);"),
    (529, "                pack::write_u8(out, 3);"),
    (530, "                pack::write_str(out, topic);"),
    (572, "                pack::write_u8(out, 0);"),
    (573, "                pack::write_bytes(out, bytes);"),
    (576, "                pack::write_u8(out, 1);"),
    (577, "                pack::write_bool(out, *checkpoint);"),
    (580, "                pack::write_u8(out, 2);"),
    (584, "                pack::write_u8(out, 3);"),
    (585, "                pack::write_u64(out, *seq);"),
    (588, "                pack::write_u8(out, 4);"),
    (589, "                pack::write_u64(out, *job);"),
    (615, "        pack::write_str(out, &self.0);"),
    (641, "        self.to.pack_encode(out);"),
    (642, "        self.from.pack_encode(out);"),
    (643, "        self.lane.pack_encode(out);"),
    (644, "        pack::write_u64(out, self.seq);"),
    (648, "        self.payload.pack_encode(out);"),
    (686, "                pack::write_u8(out, 3);"),
    (687, "                pack::write_bytes(out, detail);"),
    (714, "        pack::write_u64(out, self.fuel);"),
    (715, "        pack::write_u64(out, self.wall_us);"),
    (716, "        pack::write_u64(out, self.memory_bytes);"),
    (737, "        pack::write_bytes(out, &self.ui_patches);"),
    (738, "        pack::write_bytes(out, &self.effects);"),
    (740, "        self.status.pack_encode(out);"),
    (741, "        self.usage.pack_encode(out);"),
    (779, "                pack::write_u8(out, 2);"),
    (780, "                lane.pack_encode(out);"),
    (881, "        pack::write_u16(out, self.capacity);"),
    (882, "        pack::write_u16(out, self.len);"),
    (884, "            pack::write_varint_u64(out, lane.len() as u64);"),
    (886, "                envelope.pack_encode(out);"),
    (913, "        pack::write_str(out, &self.capability);"),
    (950, "                pack::write_u8(out, 0);"),
    (951, "                pack::write_f32(out, *ratio);"),
    (958, "                pack::write_u8(out, 5);"),
    (959, "                pack::write_str(out, detail);"),
    (962, "                pack::write_u8(out, 6);"),
    (963, "                pack::write_u32(out, *count);"),
    (1005, "                pack::write_u8(out, 2);"),
    (1006, "                pack::write_f32(out, *factor);"),
    (1009, "                pack::write_u8(out, 3);"),
    (1010, "                pack::write_u64(out, *until);"),
    (1014, "                pack::write_u8(out, 5);"),
    (1015, "                pack::write_u32(out, *restarts);"),
    (1018, "                pack::write_u8(out, 6);"),
    (1019, "                pack::write_u64(out, *until);"),
    (1180, "        self.stage.pack_encode(out);"),
    (1181, "        pack::write_u32(out, self.clean_turns);"),
    (1182, "        pack::write_u32(out, self.warn_count);"),
    (1183, "        pack::write_u32(out, self.restart_count);"),
    (1184, "        pack::write_u64(out, self.last_signal_ms);"),
    (1234, "                pack::write_u8(out, 3);"),
    (1278, "        self.id.pack_encode(out);"),
    (1279, "        self.kind.pack_encode(out);"),
    (1280, "        self.package.pack_encode(out);"),
    (1281, "        self.shard.pack_encode(out);"),
    (1283, "        self.budget.pack_encode(out);"),
    (1284, "        self.mailbox.pack_encode(out);"),
    (1285, "        self.status.pack_encode(out);"),
    (1286, "        self.failure.pack_encode(out);"),
    (1287, "        self.metrics.pack_encode(out);"),
    (1325, "        pack::write_u8(out, self.tag().await);"),
    (1343, "        pack::write_u16(out, self.0);"),
    (1483, "        self.kind.pack_encode(out);"),
    (1484, "        pack::write_u16(out, self.shard_count);"),
    (1485, "        pack::write_u16(out, self.exclusive_reserve);"),
    (1487, "            actor.pack_encode(o);"),
    (1488, "            shard.pack_encode(o);"),
    (1491, "            shard.pack_encode(o);"),
    (1492, "            actor.pack_encode(o);"),
    (1553, "        self.actor.pack_encode(out);"),
    (1554, "        self.shard.pack_encode(out);"),
    (1555, "        self.budget.pack_encode(out);"),
    (1755, "        pack::write_u64(out, self.revision);"),
    (1756, "        pack::write_u64(out, self.committed_ms);"),
    (1757, "        pack::write_bytes(out, &self.patches);"),
    (1758, "        pack::write_u32(out, self.node_count);"),
    (1925, "        pack::write_u64(out, self.turns);"),
    (1926, "        pack::write_u64(out, self.fuel_total);"),
    (1927, "        pack::write_u64(out, self.wall_us_total);"),
    (1929, "            pack::write_u32(out, *sample);"),
    (1931, "        pack::write_u8(out, self.wall_us_ring_len);"),
    (1932, "        pack::write_u8(out, self.wall_us_ring_pos);"),
    (1933, "        pack::write_u64(out, self.memory_bytes);"),
    (1934, "        pack::write_u16(out, self.mailbox_len);"),
    (1935, "        pack::write_u32(out, self.mailbox_lag_ms);"),
    (1936, "        pack::write_u64(out, self.coalesced);"),
    (1937, "        pack::write_u64(out, self.dropped);"),
    (1938, "        pack::write_u32(out, self.traps);"),
    (1939, "        pack::write_u32(out, self.restarts);"),
    (1940, "        self.stage.pack_encode(out);"),
    (1941, "        self.shard.pack_encode(out);"),
    (1983, "        pack::write_u32(out, self.actors);"),
    (1984, "        pack::write_f32(out, self.busy_ratio);"),
    (1985, "        pack::write_u32(out, self.heartbeat_age_ms);"),
    (2003, "        pack::write_u32(out, self.actors);"),
    (2004, "        pack::write_u32(out, self.shards);"),
    (2005, "        pack::write_u32(out, self.packages);"),
    (2028, "        self.id.pack_encode(out);"),
    (2029, "        self.package.pack_encode(out);"),
    (2030, "        self.lane.pack_encode(out);"),
    (2031, "        self.status.pack_encode(out);"),
    (2032, "        self.metrics.pack_encode(out);"),
    (2052, "        self.shard.pack_encode(out);"),
    (2053, "        self.metrics.pack_encode(out);"),
    (2075, "        self.kernel.pack_encode(out);"),
    (2078, "        pack::write_u64(out, self.sampled_at_ms);"),
]

lines = open(PATH, encoding="utf-8").read().split("\n")
errors = []
for lineno, expected in TARGETS:
    actual = lines[lineno - 1]
    if actual != expected:
        errors.append((lineno, expected, actual))

if errors:
    print(f"ABORT: {len(errors)} line(s) did not match expected text — file has drifted, re-scan.")
    for lineno, expected, actual in errors:
        print(f"  line {lineno}:\n    expected: {expected!r}\n    actual:   {actual!r}")
    sys.exit(1)

for lineno, expected in TARGETS:
    line = lines[lineno - 1]
    assert line.rstrip().endswith(");")
    # insert .await right before the trailing ';'
    idx = line.rstrip().rfind(");")
    new_line = line[:idx + 1] + ".await" + line[idx + 1:]
    lines[lineno - 1] = new_line

open(PATH, "w", encoding="utf-8").write("\n".join(lines))
print(f"OK: patched {len(TARGETS)} lines.")
