"""🧹️ Strips the orphaned `mod dbg` debug-ledger instrumentation whose target file no longer exists, restoring the uninstrumented logic."""
import re, sys
R = "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/"
LINE = re.compile(r'^#\[path="[^"]*debug-ledger\.rs"\]#\[allow\(dead_code\)\](?:pub\(crate\) )?mod dbg;\n', re.M)
def edit(path, pairs):
    s = open(path, encoding="utf-8").read()
    s, n = LINE.subn("", s); assert n == 1, (path, "mod", n)
    for old, new in pairs:
        c = s.count(old); assert c == 1, (path, old[:60], c); s = s.replace(old, new)
    assert "dbg" not in s, (path, "dbg left")
    open(path, "w", encoding="utf-8").write(s); print("stripped", path)
def cut(path, pattern):
    s = open(path, encoding="utf-8").read(); m = re.findall(pattern, s, re.S); assert len(m) == 1, (path, pattern, len(m)); return m[0]
rm = R + "🏗️record-machine/🦀️.rs"
leaf = cut(rm, r"\n if dbg::on\(\)\{dbg::add\(format!\(\"leaf:[^\n]*\n")
wrap = cut(rm, r"thread_local!\{static DBG_INNER.*?\n poll\n\}\n")
edit(rm, [(leaf, "\n"), (wrap, ""), ("fn close_poll_inner<", "fn close_poll<")])
og = R + "🧳️original/🦀️.rs"
edit(og, [(cut(og, r'if dbg::on\(\)\{dbg::add\("recvroot"[^}]*\}'), ""), (cut(og, r"\n  let dbg_branch=[^\n]*"), ""), (cut(og, r"if let Some\(b\)=dbg_branch\{.*?dbg::add\(format!\(\"closeroot:\{b\}\"\),work,i,y\)\}"), "")])
pr = R + "🧬️protocol/📦️packages/🦀️rust/🦀️.rs"
edit(pr, [(cut(pr, r'if dbg::on\(\)&&self\.demand\.is_none\(\)\{let n=dbg::strip[^}]*\}'), "")])
rc = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs"
s = open(rc, encoding="utf-8").read()
head = "  let ph=self.phase;let w0=cx.original_work_units_spent();let r0=cx.retained_progress();\n  let out=(||->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{\n  let out=(||->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{\n"
tail = cut(rc, r"\n  \}\)\(\);\n  out\n  \}\)\(\);\n  if dbg::on\(\)\{let w=cx[^\n]*\n  out\n")
edit(rc, [(head, ""), (tail, "\n")])
