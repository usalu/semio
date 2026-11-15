import sys
root=sys.argv[1]
def edit(path,pairs):
    s=open(root+"/"+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60])
        s=s.replace(a,b)
    open(root+"/"+path,"w").write(s)
edit("🗺️topology/🔣️.json",[('"arguments": ["agent"], "roles": ["member", "server"], "requires": "platform-peer"','"arguments": ["agent"], "roles": ["member", "server"], "requires": "none"')])
edit("🗺️topology/🧪️tests/🐍️.py",[('("build-agent",["agent"],["member","server"],"platform-peer")','("build-agent",["agent"],["member","server"],"none")')])
edit("🔁️replica/🧪️tests/🧷️sidecars.ts",[('["build-agent",["agent"],["member","server"],"platform-peer"]','["build-agent",["agent"],["member","server"],"none"]')])
edit("README.md",[("both scripts of `📚️library/🏗️build/🌐️remote-execution/🛰️sidecar`.","both scripts of `📚️library/🏗️build/🌐️remote-execution/🛰️sidecar`. `build-agent` has `requires: \"none\"` (2026-11-14): every machine runs it and reports its load, also the only machine of its platform (a Linux or Windows worker without a same-platform peer used to report nothing and was not shown); whether work may be offloaded to a machine stays a per-platform decision of the build scheduler, not of the sidecar mechanism. `requires: \"platform-peer\"` remains available to children that need a peer.")])
