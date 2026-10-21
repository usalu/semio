#!/usr/bin/env python3
"""🧬 mutation-check.py: breaks the implementation in one place at a time and requires the named suite to fail.
usage: mutation-check.py [NATIVE_DIR]; works on a scratch copy of dev/ (never touches it)."""
import os, shutil, subprocess, sys, tempfile
here = os.path.dirname(os.path.abspath(__file__))
native = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else ""
MUTATIONS = [
    ("state: events never mark owners dirty", "state/index.mjs", 'else if(target.owner!==undefined){const owner=owners.get(target.owner);owner.seq=++seq;owner.dirtyFiles.add(path);}', 'else if(target.owner!==undefined){}', "state"),
    ("state: a remembered result ignores its key", "state/index.mjs", "entry.key===await taskKey(id)", "true", "service"),
    ("state: a gap does not force stat proof", "state/index.mjs", "if(owner){owner.seq=batch;owner.statCheck=true;}", "if(owner){owner.seq=batch;}", "state"),
    ("state: the journal vouches across a gap", "state/index.mjs", "if(entry.lost)return{complete:false,reason:\"lost\",scope:entry.scope};", "if(entry.lost)continue;", "state"),
    ("state: parent directories are not re-stat'ed", "state/index.mjs", "for(const parent of parents){", "for(const parent of []){", "state"),
    ("state: output events do not invalidate", "state/index.mjs", "if(!running.has(producer))invalidOutputs.add(producer);continue;}", "continue;}", "state"),
    ("service: no fence before answering", "service/index.mjs", "const result=await watcher.fence();\n   await watcher.drained();", "const result={ok:true};", "service"),
    ("service: capability not checked", "service/index.mjs", "if(!caveats)return refuse(connection,frame.id,\"capability\"", "if(false)return refuse(connection,frame.id,\"capability\"", "service"),
    ("service: caveats not enforced", "service/index.mjs", "if(denial)return refuse", "if(false)return refuse", "service"),
    ("service: result filed under the key computed after the run, guard removed", "service/index.mjs", ["if(keyAfter===keyBefore){","state.memo.set(id,{id,key:keyBefore,"], ["if(true){","state.memo.set(id,{id,key:keyAfter,"], "service"),
    ("service: environment binding ignored", "service/index.mjs", "if(requested!==actual&&(requested!==undefined||actual!==undefined))return refuse", "if(false)return refuse", "service"),
    ("governor: unbounded queue", "service/governor.mjs", "if(queued>=maximumQueued)return Promise.reject(new BusyErrorV1(inFlight,queued));", "", "service"),
    ("governor: fifo instead of per-lane round robin", "service/governor.mjs", "const lane=ring[(cursor+step)%ring.length]", "const lane=ring[step]", "service"),
    ("watch: a fence resolves before its event", "watch/index.mjs", "return Promise.race([entry.promise.then(()=>\"event\"),timeout])", "return Promise.resolve(\"event\")", "watch"),
    ("capability: tampered chains verify", "../core/session.mjs", "return mac.length===parts.mac.length&&timingSafeEqual(mac,parts.mac)?parts.payload.caveats:undefined;", "return parts.payload.caveats;", "protocol"),
    ("protocol: unknown properties accepted", "protocol/frames.mjs", "else if(schema.additionalProperties===false)errors.push(path+\": unexpected \"+key);", "", "protocol"),
    ("hooks: dirty paths ignore the parent directory", "graph-hooks/index.mjs", "exact.add(cut<0?\"\":change.path.slice(0,cut));", "", "graph-hooks"),
    ("delegate: request env is not applied", "delegate/index.mjs", "for(const[key,value]of Object.entries(request.env??{})){if(value===null)delete process.env[key];else process.env[key]=value;}", "", "delegate"),
    ("client: eligibility accepts any flag", "client/repo.mjs", "if(!POLICY_V1.eligibleFlags.some(", "if(false&&!POLICY_V1.eligibleFlags.some(", "client"),
    ("host seam: unknown dirty set trusts every row", "graph-hooks/host/index.mjs", "if(!set)return holds(rows,signal);", "if(!set)return true;", "graph-hooks"),
    ("host seam: journal answers are not checked for completeness", "graph-hooks/index.mjs", "if(!previous||!reply||reply.complete!==true)return undefined;", "if(!previous||!reply)return undefined;", "graph-hooks"),
    ("service: frontier adoption ignores the instance", "service/index.mjs", "||owner!==instance||", "||", "service"),
    ("executor: environment of one command leaks into the next", "executor/index.mjs", "for(const key of Object.keys(state.env))if(!(key in savedEnvironment))delete state.env[key];", "", "executor"),
    ("executor pool: an authority refusal does not switch the mode off", "delegate/executor.mjs", 'if(frame.refusal==="authority")refused=true;', "", "executor"),
    ("provision: a stale pin is accepted", "watch/provision/index.mjs", "if(row.sha256!==digest||row.bytes!==readFileSync(sourcePath).length)", "if(false)", "provision"),
    ("provision: an unprobed build is installed", "watch/provision/index.mjs", "if(!probe(staged)){", "if(false){", "provision"),
    ("routing: a refused executor does not fall through", "delegate/index.mjs", 'if(!(result.refusal==="authority"||result.lost&&!wrote))', "if(true)", "delegate"),
    ("client: leading service option is not skipped", "client/repo.mjs", 'while(at<args.length&&args[at].startsWith("--service="))at++;', "", "client"),
    ("service: parallel tasks interleave inside a line", "service/index.mjs", "return forward(stream,data);}});", "return context.output(stream,data);}});", "service"),
    ("service: a revoked session is still served", "service/index.mjs", "if(revocations.has(session)||revocations.hasAgent(connection.lane))return refuse", "if(false)return refuse", "service"),
    ("service: revoking does not cancel the running commands", "service/index.mjs", 'controller.abort(Object.assign(new Error("revoked"),{code:"revoked"}));aborted++;', "aborted++;", "service"),
    ("service: the client identity is not passed to the execution port", "service/index.mjs", "env:context.env,client:context.client,", "env:context.env,", "service"),
    ("service: a refusal of the core is ignored", "service/index.mjs", "if(result.refusal)throw new ServiceRefusedErrorV1(result.refusal.code,result.refusal.fields);", "", "service"),
    ("service: the lane is not bound to the agent caveat", "service/index.mjs", "projects:client.named,worktree,agent:connection.lane}", "projects:client.named,worktree}", "service"),
    ("service: issued tokens are not bound to the worktree", "service/index.mjs", 'caveats.push({k:"worktree",v:worktree},', 'caveats.push(', "service"),
    ("delegate: a refusal of the service invocation falls through to the warm worker", "delegate/index.mjs", 'if(result.refusal==="service"){timing("refused");return{exit:result.exit,refusal:{code:result.code,fields:result.fields}};}', "", "delegate"),
    ("executor: a refusal of the service invocation retires the executor", "executor/index.mjs", 'if(refusal==="authority")break;', "if(refusal)break;", "executor"),
    ("client: the launcher token is not bound to the agent", "client/index.mjs", '...(identity.agent?[{k:"agent",v:identity.agent}]:[])', "", "client"),
    ("ports: the task closure ignores transitive dependencies", "ports/graph.mjs", "if(!seen.has(next)){seen.add(next);pending.push(next);}", "seen.add(next);", "service"),
    ("session: revocation by agent is ignored", "../core/session.mjs", "hasAgent:agent=>{reload();return agents.has(agent);},", "hasAgent:()=>false,", "service"),
    ("session: the worktree caveat is not enforced", "../core/session.mjs", 'else if(caveat.k==="worktree"){if(request.worktree!==caveat.v)return', 'else if(caveat.k==="worktree"){if(false)return', "service"),
    ("session: the agent caveat is not enforced", "../core/session.mjs", 'else if(caveat.k==="agent"){if(request.agent!==caveat.v)return', 'else if(caveat.k==="agent"){if(false)return', "service"),
]
mapping = {"state": "state.test.ts", "service": "service.test.ts", "watch": "watch.test.ts", "protocol": "protocol.test.ts", "graph-hooks": "graph.test.ts", "delegate": "delegate.test.ts", "client": "client.test.ts", "executor": "executor.test.ts", "provision": "provision.test.ts"}
survivors = 0
only = os.environ.get("ONLY")
MUTATIONS = [row for row in MUTATIONS if not only or only in row[0]]
for label, file, old, new, suite in MUTATIONS:
    scratch = tempfile.mkdtemp(prefix="mutation-")
    shutil.copytree(os.path.join(here, "dev"), os.path.join(scratch, "dev"), ignore=shutil.ignore_patterns("__pycache__"))
    path = os.path.join(scratch, "dev", "resident", file)
    text = open(path, encoding="utf-8").read()
    olds, news = (old, new) if isinstance(old, list) else ([old], [new])
    if any(item not in text for item in olds):
        print("ANCHOR MISSING", label)
        survivors += 1
        continue
    for item, replacement in zip(olds, news):
        text = text.replace(item, replacement, 1)
    open(path, "w", encoding="utf-8").write(text)
    env = dict(os.environ, SEMIO_REPO_WATCH_BINARY=native + "/semio-repo-watch", SEMIO_REPO_FAKETOOL_BINARY=native + "/faketool", SEMIO_REPO_NATIVE_DIR=native)
    test = os.path.join(scratch, "dev", "resident", "tests", mapping[suite])
    out = subprocess.run(["nice", "-n", "10", "bun", "test", test], capture_output=True, text=True, env=env, cwd=os.path.dirname(test), timeout=300)
    killed = out.returncode != 0
    print("%-8s %s" % ("killed" if killed else "SURVIVED", label))
    survivors += 0 if killed else 1
    shutil.rmtree(scratch, ignore_errors=True)
print("%d of %d mutations killed" % (len(MUTATIONS) - survivors, len(MUTATIONS)))
sys.exit(1 if survivors else 0)
