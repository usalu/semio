#!/usr/bin/env python3
"""🔭️ e10: witness law - `ps` is exempt only with the exact argument vectors of the planner child."""
import sys
F = sys.argv[1] + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧪️tests/🟦️.ts"
text = open(F, encoding="utf8").read()
def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:80])
    text = text.replace(old, new)
once('''t(()=>spawnSync("ps",["-axo","pid=,ppid="]));t(()=>spawnSync("ps",["-o","etime=","-p","1"]));''', '''t(()=>spawnSync("ps",["-axo","pid=,ppid="]));t(()=>spawnSync("ps",["-o","etime=","-p","1"]));t(()=>spawnSync("ps",["-o","cputime=","-p","1,2,3"]));''')
once('''t(()=>Bun.spawnSync(["/nonexistent/ps","-axo","pid=,ppid=","--extra"]));w.finish();`);''', '''t(()=>Bun.spawnSync(["/nonexistent/ps","-axo","pid=,ppid=","--extra"]));t(()=>spawnSync("ps",["-axo","pid=,ppid=,command="]));t(()=>spawnSync("ps",["-ef"]));t(()=>spawnSync("ps",["-o","etime=","-p","1;x"]));t(()=>spawnSync("ps",["-o","cputime=","-p","1,,2"]));t(()=>spawnSync("ps",["-o","etime=","-p","1","2"]));w.finish();`);''')
once('''expect(closed.tainted.length).toBeGreaterThanOrEqual(3);expect(closed.tainted.some(reason=>/ps -axo/u.test(reason)&&!/nonexistent/u.test(reason))).toBe(false);''', '''expect(closed.tainted.length).toBeGreaterThanOrEqual(3);expect(closed.tainted.some(reason=>/spawned \\S+ ps -axo pid=,ppid=( @|$)/u.test(reason))).toBe(false);
  for(const vector of["ps -axo pid=,ppid=,command=","ps -ef","ps -o etime= -p 1;x","ps -o cputime= -p 1,,2","ps -o etime= -p 1 2"])expect(closed.tainted.some(reason=>reason.includes("spawned")&&reason.includes(vector)),vector+" must taint").toBe(true);''')
open(F, "w", encoding="utf8").write(text)
