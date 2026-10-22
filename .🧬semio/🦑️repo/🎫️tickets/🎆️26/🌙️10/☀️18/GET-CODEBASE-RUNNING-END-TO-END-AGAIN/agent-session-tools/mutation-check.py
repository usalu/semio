#!/usr/bin/env python3
"""Proves that the tests can fail: applies one mutation at a time to the finished sources of an overlay, runs the suites that must notice it, restores the file and reports killed or survived.
usage: mutation-check.py <overlay>   (exit code 1 when a mutation survives)"""
import os, subprocess, sys

OV = sys.argv[1]
F = "🧰️framework/🔨️modules/🏃️process"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
A = F + "/🪪️agent-sessions"
CORE = A + "/🧪️tests/🟦️.ts"
ADAPT = A + "/🧪️tests/🔗️adapters/🟦️.ts"
CMD = L + "/🏗️build/🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts"
SANDBOX = F + "/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts"
M = [
    ("an explicit name no longer wins", A + "/🪪️identity/🟦️.ts", "if(seed.agent!==undefined){", "if(false){", [CORE]),
    ("the default agent no longer has the fixed session", A + "/🪪️identity/🟦️.ts", "if(agent===identity.defaultAgent)return identity.defaultSession;", "", [CORE]),
    ("client recognition looks at arguments", A + "/🪪️identity/🟦️.ts", "const match=clientPattern.exec(subject===executable?base(executable):subject);", "const match=clientPattern.exec(text);", [CORE]),
    ("idle window off by one", A + "/🪪️identity/🟦️.ts", "facts.ageSeconds<=rules.idleSeconds)return \"live\"", "facts.ageSeconds<rules.idleSeconds)return \"live\"", [CORE]),
    ("share ties favour the larger identifier", A + "/🪪️identity/🟦️.ts", "b.rest-a.rest||(a.id<b.id?-1:a.id>b.id?1:0)", "b.rest-a.rest||(a.id>b.id?-1:a.id<b.id?1:0)", [CORE]),
    ("anchors are not bounded", A + "/🗄️store/🟦️.ts", "if(next.length>live.anchorsMaximum)", "if(false)", [CORE]),
    ("an unchanged session is rewritten", A + "/🗄️store/🟦️.ts", "if(unchanged){touchAgentSessionV1(store,existing.id,now);return {session:existing,created:false};}", "", [CORE]),
    ("the default session is collected", A + "/🗄️store/🟦️.ts", "||row.session.agent.origin===\"default\"||options.only", "||options.only", [CORE]),
    ("a session with kept resources is removed", A + "/🗄️store/🟦️.ts", "removed=result.kept===0&&closeAgentSessionV1", "removed=closeAgentSessionV1", [CORE, CMD]),
    ("the token file is world readable", A + "/🗄️store/🟦️.ts", "atomic(path(store,id,\"token\"),token,0o600)", "atomic(path(store,id,\"token\"),token,0o644)", [CORE]),
    ("the environment address is ignored", A + "/🌿️current/🟦️.ts", "const named=options.agent!==undefined||options.connection!==undefined||options.origin!==undefined,address=named?undefined:parse", "const named=true,address=named?undefined:parse", [CORE, ADAPT]),
    ("the budget weight is ignored by the admission order", F + "/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts", "weight=(waiter:HostWaiterV1)=>waiter.weight??1", "weight=(waiter:HostWaiterV1)=>1", [ADAPT]),
    ("held memory is ignored by the admission order", F + "/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts", "grant.session===waiter.session?grant.reservedMegabytes:0", "0", [ADAPT]),
    ("the Bazel preference is ignored", L + "/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts", "index=(slot:string)=>preference===undefined?", "index=(slot:string)=>true?", [ADAPT]),
    ("the Bazel slot state forgets the owner", L + "/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts", "usedAt:now,...owner===undefined?{}:{session:owner.session,agent:owner.agent}}", "usedAt:now}", [ADAPT]),
    ("sandboxes ignore the session root", F + "/🧪️testing/🏖️sandbox/📁️root/🟦️.ts", "root = join(home, id);", "root = join(parent, id);", [SANDBOX, CMD]),
    ("the lease registry drops the session", F + "/🔒️leases/🧭️order/🗂️registry/🟦️.ts", "...address===undefined?{}:{session:address},", "", [ADAPT]),
    ("a gone session keeps its sandboxes", F + "/🧪️testing/🏖️sandbox/♻️reclaim/🟦️.ts", "if (kept === 0) { try { detach(home); rmSync(home, { recursive: true, force: true }); released++; } catch { kept++; } }", "", [SANDBOX, CMD]),
]
survived = []
for name, rel, old, new, suites in M:
    path = os.path.join(OV, rel)
    text = open(path, encoding="utf8").read()
    if text.count(old) != 1:
        print(f"BROKEN  {name}: anchor found {text.count(old)} times in {rel}"); survived.append(name); continue
    open(path, "w", encoding="utf8").write(text.replace(old, new))
    try:
        failed = False
        for suite in suites:
            p = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + suite], cwd=OV, capture_output=True, text=True, timeout=300)
            if p.returncode != 0:
                failed = True; break
    finally:
        open(path, "w", encoding="utf8").write(text)
    print(("killed  " if failed else "SURVIVED"), name, flush=True)
    if not failed:
        survived.append(name)
print(len(M) - len(survived), "of", len(M), "mutations killed")
sys.exit(1 if survived else 0)
