#!/usr/bin/env python3
"""🧫 core-mutation-check.py: breaks the service invocation of the authority core in one place at a time and requires its suite to fail.
usage: core-mutation-check.py OVERLAY   (an overlay that apply.py has written, with the real library files; the file under mutation is restored after every run)"""
import importlib.util, os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("apply", os.path.join(here, "apply.py"))
apply = importlib.util.module_from_spec(spec)
spec.loader.exec_module(apply)
root = os.path.abspath(sys.argv[1])
TS = "\U0001F7E6️.ts"
SERVICE = os.path.join(root, apply.M["core/service.ts"])
SESSION = os.path.join(root, apply.M["core/session.mjs"])
CALLER = os.path.join(root, apply.CALLER, TS)
INVOCATION = os.path.join(root, apply.B, "\U0001F9ED️invocation", TS)
CHILD = os.path.join(root, apply.CALLER, "\U0001F4E4️child", TS)
LAW = os.path.join(root, apply.M["core/law.ts"])
SUITE = os.path.join(root, apply.M["core/tests/service.test.ts"])
LAW_SUITE = os.path.join(root, apply.M["core/tests/law.test.ts"])
MUTATIONS = [
    ("admission: the key digest of the grant is not compared", SERVICE, "||keyDigestV1(key)!==grant.keyDigest", "", SUITE),
    ("admission: a token that does not verify is admitted", SERVICE, "if(!caveats)throw refuse(\"token\");", "if(!caveats)return undefined as never;", SUITE),
    ("admission: required caveats are not required", SERVICE, "for(const required of policy.requiredCaveats)if(!caveats.some(caveat=>caveat.k===required))throw refuse(\"caveats\",{caveat:required});", "", SUITE),
    ("admission: an expired token is admitted", SERVICE, "if(at>notAfter)throw refuse(\"expired\");", "", SUITE),
    ("admission: a token without a bounded lifetime is admitted", SERVICE, "if(notAfter-at>policy.maximumLifetimeMilliseconds)throw", "if(false)throw", SUITE),
    ("admission: a directory outside the worktree is admitted", SERVICE, "if(!insideWorktree(root,request.cwd))throw refuse(\"worktree\");", "", SUITE),
    ("admission: the caveats of the token are not evaluated", SERVICE, "if(denial){", "if(false){", SUITE),
    ("admission: a project restricted token runs a command that names no project", SERVICE, "if(named.length===0&&caveats.some(caveat=>caveat.k===\"projects\"))throw", "if(false)throw", SUITE),
    ("admission: a revoked session is admitted", SERVICE, "if(revocations.has(session)||revocations.hasAgent(request.agent))throw refuse(\"revoked\");\n   const target", "const target", SUITE),
    ("admission: a ticket outlives the revocation of its session", SERVICE, "check:()=>{if(revocations.has(session)||revocations.hasAgent(request.agent))throw refuse(\"revoked\");if(now()>notAfter)throw refuse(\"expired\");}", "check:()=>undefined", SUITE),
    ("admission: dependencies are not checked against the closure", SERVICE, "if(!reachable.has(target))throw refuse(\"closure\",{task:tasks[0]!});", "", SUITE),
    ("admission: the request budget is not enforced", SERVICE, "state.admitted>=grant.maximumRequests||", "", SUITE),
    ("admission: the concurrency budget is not enforced", SERVICE, "||state.active>=grant.maximumConcurrent", "", SUITE),
    ("admission: an audit failure does not refuse the request", SERVICE, "catch{throw refuse(\"audit\");}", "catch{}", SUITE),
    ("admission: the launches of the one-shot door are replaced by nothing", SERVICE, "try{launches=await declare(request.args);}", "try{launches=(await declare(request.args)).slice(0,0);}", SUITE),
    ("admission: another build implementation is accepted by the door", SERVICE, "door.service!==undefined||door.tool!==caller.tool||door.buildMode!==caller.buildMode||JSON.stringify(door.native)!==JSON.stringify(caller.native)", "false", SUITE),
    ("admission: the grant worktree is not compared", SERVICE, "if(grant.worktree!==worktree||", "if(", SUITE),
    ("arguments: forbidden flags are allowed", SERVICE, "policy.forbiddenFlags.includes(name)||", "", SUITE),
    ("arguments: flags outside the operation are allowed", SERVICE, "||!allowed.has(name)))throw bad", "))throw bad", SUITE),
    ("arguments: the number of task names is not bounded", SERVICE, "if(positionals.length<minimum!||positionals.length>maximum!)throw", "if(false)throw", SUITE),
    ("arguments: task names are not matched", SERVICE, "for(const task of positionals)if(!pattern.test(task))throw bad(\"task name \"+task.slice(0,64));", "", SUITE),
    ("arguments: operations outside the grant are allowed", SERVICE, "if(!grant.operations.includes(operation as ServiceGrantV1[\"operations\"][number]))throw refuse(\"scope\",{reason:operation});", "", SUITE),
    ("arguments: the tool of the caller is not enforced", SERVICE, "if(name===\"--tool\"&&value!==tool)throw refuse(\"tool\");", "", SUITE),
    ("arguments: the named projects come from the client alone", SERVICE, "named=request.named??own;", "named=request.named??[];", SUITE),
    ("hold: an invocation without a grant is held", SERVICE, "if(held.service===undefined)throw refuse(\"grant\");", "", SUITE),
    ("declaration: any grant is accepted for the live ticket", SERVICE, "store.grant!==canonicalJsonV1(service)||", "", SUITE),
    ("declaration: another worktree is accepted for the live ticket", SERVICE, "||canonicalRootV1(rest.workspaceRoot??\"\")!==store.root", "", SUITE),
    ("declaration: a revoked ticket still declares", SERVICE, "store.ticket.check();\n const launches", "const launches", SUITE),
    ("declaration: the grant stays visible to the checks", SERVICE, "if(store===undefined||store.grant!==canonicalJsonV1(service)||canonicalRootV1(rest.workspaceRoot??\"\")!==store.root)return rest as Omit<T,\"service\">;", "if(store===undefined||store.grant!==canonicalJsonV1(service)||canonicalRootV1(rest.workspaceRoot??\"\")!==store.root)return capabilities as Omit<T,\"service\">;", SUITE),
    ("check site: the owned invocation ignores the declaration", CALLER, "capabilities=declaredCapabilitiesV1(original.capabilities as NativeBuildCallerCapabilitiesV1);", "capabilities=original.capabilities as NativeBuildCallerCapabilitiesV1;", SUITE),
    ("check site: the Nx child ignores the declaration", CALLER, "role=declaredCapabilitiesV1(original.capabilities as NativeBuildCallerCapabilitiesV1)", "role=original.capabilities as NativeBuildCallerCapabilitiesV1", SUITE),
    ("check site: the ancestor selection ignores the declaration", INVOCATION, "declaredCapabilitiesV1(value)", "value", SUITE),
    ("check site: the task environment passes the grant on", CALLER, "capabilities=declaredCapabilitiesV1(invocation.capabilities as NativeBuildCallerCapabilitiesV1);", "capabilities=invocation.capabilities as NativeBuildCallerCapabilitiesV1;", SUITE),
    ("door: the executor receives no grant", CHILD, "service:createServiceGrantV1(root,effective)", "service:undefined", SUITE),
    ("registry: the parse is cached without a file state", INVOCATION, "if(hit!==undefined&&hit.signature===signature)return hit.registry;", "if(hit!==undefined)return hit.registry;", SUITE),
    ("registry: a file written moments ago is trusted", INVOCATION, "if(Date.now()-status.mtimeMs>2000)signature", "if(true)signature", SUITE),
    ("session: the revocation registry forgets", SESSION, "has:session=>{reload();return sessions.has(session);},", "has:()=>false,", SUITE),
    ("law: a second reader of the launches is not noticed", LAW, "if(!new RegExp(\"\\\\b\"+name+\"=[^;]*declaredCapabilitiesV1\\\\(\",\"u\").test(source))breach(", "if(false)breach(", LAW_SUITE),
    ("law: another holder of the admission is not noticed", LAW, "&&!holders.includes(file))breach(", "&&false)breach(", LAW_SUITE),
]
survivors = 0
for label, path, old, new, suite in MUTATIONS:
    text = open(path, encoding="utf-8").read()
    if old not in text:
        print("ANCHOR MISSING", label)
        survivors += 1
        continue
    open(path, "w", encoding="utf-8").write(text.replace(old, new, 1))
    try:
        out = subprocess.run(["nice", "-n", "10", "bun", "test", suite], capture_output=True, text=True, cwd=os.path.dirname(suite), timeout=300)
    finally:
        open(path, "w", encoding="utf-8").write(text)
    killed = out.returncode != 0
    print("%-8s %s" % ("killed" if killed else "SURVIVED", label))
    survivors += 0 if killed else 1
print("%d of %d mutations killed" % (len(MUTATIONS) - survivors, len(MUTATIONS)))
sys.exit(1 if survivors else 0)
