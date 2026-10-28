from make_lib import *

# ---- D: physical lock attempts
K = P + "/🔒️leases/📁️physical"
lock, sch, win = K + "/🟦️.ts", K + "/🧬️schema/🔣️.json", K + "/🧪️tests/🪟️windows/🟦️.ts"
edit(sch, '        "retryMilliseconds": 40,\n', '        "retryMilliseconds": 40,\n        "attempts": 3,\n')
edit(lock, """  tryLock:()=>{if(library.symbols.flock!(held.at(-1)!.fd,protocol.exclusive|protocol.nonblocking)===0)return true;const pointer=library.symbols[errnoName]!();if(pointer===null||read.i32(pointer)!==protocol.contentionErrno[platform as"darwin"|"linux"])throw new PhysicalLockErrorV1("admission");return false;},""",
"""  tryLock:()=>{for(let attempt=0;attempt<protocol.attempts;attempt++){if(library.symbols.flock!(held.at(-1)!.fd,protocol.exclusive|protocol.nonblocking)===0)return true;const pointer=library.symbols[errnoName]!();if(pointer!==null&&read.i32(pointer)===protocol.contentionErrno[platform as"darwin"|"linux"])return false;}throw new PhysicalLockErrorV1("admission");},""")
edit(lock, """  tryLock:()=>{const row=rows.at(-1)!;if(range(row.handle,win.exclusive|win.failImmediately)!==0){row.locked=true;return true;}const code=Number(symbols.GetLastError!());if(code===win.contentionError)return false;throw new PhysicalLockErrorV1("admissionWindows",code);},""",
"""  tryLock:()=>{const row=rows.at(-1)!;let code=0;for(let attempt=0;attempt<schema.definitions.Protocol.const.attempts;attempt++){if(range(row.handle,win.exclusive|win.failImmediately)!==0){row.locked=true;return true;}code=Number(symbols.GetLastError!());if(code===win.contentionError)return false;}throw new PhysicalLockErrorV1("admissionWindows",code);},""")
edit(lock, "/** 🧱️ One host's kernel lock mechanism: `retain` opens and identifies a file, `tryLock` takes the exclusive lock on the file retained last (false = another holder),",
 "/** 🧱️ One host's kernel lock mechanism: `retain` opens and identifies a file, `tryLock` takes the exclusive lock on the file retained last (false = another holder; the failure code is read by a second foreign call, so a failure that is not the contention code is repeated up to `Protocol.attempts` times before it is refused),")
edit(win, "failLock:0,failOpen:0};", "failLock:0,failOpen:0,stuck:false};")
edit(win, "     if(state.failLock>0&&--state.failLock===0)return fail(5,0);\n", "     if(state.failLock>0&&--state.failLock===0)state.stuck=true;\n     if(state.stuck)return fail(5,0);\n")
newfile(K + "/🧪️tests/🔁️lock-attempts/🟦️.ts", "lock-attempts.ts")
