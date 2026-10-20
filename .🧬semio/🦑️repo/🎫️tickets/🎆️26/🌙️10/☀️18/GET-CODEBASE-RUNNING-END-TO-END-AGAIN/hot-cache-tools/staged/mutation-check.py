#!/usr/bin/env python3
"""mutation-check.py: breaks the implementation in one place at a time and requires the named suite to fail.
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
    ("service: result filed under the key computed after the run", "service/index.mjs", "state.memo.set(id,{id,key:keyBefore,", "state.memo.set(id,{id,key:keyAfter,", "service"),
    ("service: environment binding ignored", "service/index.mjs", "if(requested!==actual&&(requested!==undefined||actual!==undefined))return refuse", "if(false)return refuse", "service"),
    ("governor: unbounded queue", "service/governor.mjs", "if(queued>=maximumQueued)return Promise.reject(new BusyErrorV1(inFlight,queued));", "", "service"),
    ("governor: fifo instead of per-lane round robin", "service/governor.mjs", "const lane=ring[(cursor+step)%ring.length]", "const lane=ring[step]", "service"),
    ("watch: a fence resolves before its event", "watch/index.mjs", "return Promise.race([entry.promise.then(()=>\"event\"),timeout])", "return Promise.resolve(\"event\")", "watch"),
    ("capability: tampered chains verify", "protocol/capability.mjs", "return mac.length===parts.mac.length&&timingSafeEqual(mac,parts.mac)?parts.payload.caveats:undefined;", "return parts.payload.caveats;", "protocol"),
    ("protocol: unknown properties accepted", "protocol/frames.mjs", "else if(schema.additionalProperties===false)errors.push(path+\": unexpected \"+key);", "", "protocol"),
    ("hooks: dirty paths ignore the parent directory", "graph-hooks/index.mjs", "exact.add(cut<0?\"\":change.path.slice(0,cut));", "", "graph-hooks"),
    ("delegate: request env is not applied", "delegate/index.mjs", "for(const[key,value]of Object.entries(request.env??{})){if(value===null)delete process.env[key];else process.env[key]=value;}", "", "delegate"),
    ("client: eligibility accepts any flag", "client/repo.mjs", "if(!POLICY_V1.eligibleFlags.some(", "if(false&&!POLICY_V1.eligibleFlags.some(", "client"),
]
mapping = {"state": "state.test.ts", "service": "service.test.ts", "watch": "watch.test.ts", "protocol": "protocol.test.ts", "graph-hooks": "graph.test.ts", "delegate": "delegate.test.ts", "client": "client.test.ts"}
survivors = 0
for label, file, old, new, suite in MUTATIONS:
    scratch = tempfile.mkdtemp(prefix="mutation-")
    shutil.copytree(os.path.join(here, "dev"), os.path.join(scratch, "dev"), ignore=shutil.ignore_patterns("__pycache__"))
    path = os.path.join(scratch, "dev", "resident", file)
    text = open(path, encoding="utf-8").read()
    if old not in text:
        print("ANCHOR MISSING", label)
        survivors += 1
        continue
    open(path, "w", encoding="utf-8").write(text.replace(old, new, 1))
    env = dict(os.environ, SEMIO_REPO_WATCH_BINARY=native + "/semio-repo-watch", SEMIO_REPO_FORWARD_BINARY=native + "/semio-repo-forward", SEMIO_REPO_FAKETOOL_BINARY=native + "/faketool", SEMIO_REPO_NATIVE_DIR=native)
    test = os.path.join(scratch, "dev", "resident", "tests", mapping[suite])
    out = subprocess.run(["nice", "-n", "10", "bun", "test", test], capture_output=True, text=True, env=env, cwd=os.path.dirname(test), timeout=300)
    killed = out.returncode != 0
    print("%-8s %s" % ("killed" if killed else "SURVIVED", label))
    survivors += 0 if killed else 1
    shutil.rmtree(scratch, ignore_errors=True)
print("%d of %d mutations killed" % (len(MUTATIONS) - survivors, len(MUTATIONS)))
sys.exit(1 if survivors else 0)
