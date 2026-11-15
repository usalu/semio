import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
s=open(w+"/README.md",encoding="utf8").read()
i=s.index("- A `put` that republishes content the path already had earlier is a **stale republish**")
j=s.index("\n",i)
new='''- A `put` that republishes content the path already had earlier is a **stale republish** when its `base` is not the content it replaces (a replica that lost its state republishes its old tree with fresh clocks and `base` null). The decision is a pure function of the events of the path (`staleRepublishV1` in `🔁️replica/⚖️clash`, fixture vectors replayed by TypeScript and by the Python oracle): the event is stale when it leaves content, differs from its predecessor (the event with the greatest smaller hlc), was not written on top of that predecessor (`base` differs) and leaves content that an earlier event left already. A deliberate revert by a machine that held the newest content has `base` equal to the predecessor and is an edit. A machine whose register still equals the predecessor's content does not take a stale event, reports `stale-republish` and publishes the newer content again as an ordinary change, so the fleet converges to the newer content even though the stale event has the greater hlc; the manifest itself is never changed by the rule. The events come from the batch being read, from the manifest entry the path held, and from a cache of the last 6 events of the last 20 000 changed paths that is kept with the local state (memo `stale.history`, the newest 1000 paths, written at most every 30 s and at close), so a restarted machine decides the same. A machine that joins later with only a snapshot has no older content to compare and applies the stale content, then takes the re-publication of the others.'''
s=s[:i]+new+s[j:]
open(w+"/README.md","w",encoding="utf8").write(s)
edit(w+"/README.md",[
 ('- Rewrite: every local commit not in the incoming history, oldest first, is re-created','- Rewrite: every local commit not in the incoming history, oldest first, is re-created (a commit whose author line and message are already in the incoming history is dropped, whatever its tree: it is the earlier rewrite of the same commit, and re-creating it would put it into the history twice)'),
 ('- **Clocks.** Stamps more than 60 s ahead','- **Silent drift heals itself.** Every two minutes (`driftMilliseconds`) the daemon reads the attestations of the fleet; when its own origin is `diverged` in the manifest against the settled origins it reports `drift` (fields, check number) and runs the repair: it takes over the store registers it lacks or holds older, drops manifest rows the complete store fold does not have, queues every manifest entry whose applied row is missing or carries another hlc, and forgets applied rows without a manifest entry (the next walk publishes the file if it still exists). Repeated findings back off from two minutes to one hour. Differences in code, git state or view are never repaired automatically.\n- **Clocks.** Stamps more than 60 s ahead'),
])
edit(w+"/README.md",[
 ('compaction --plan` reports the same). All texts exist in English and German.','compaction --plan` reports the same). All texts exist in English and German.\n\n**Litter.** macOS clients leave AppleDouble sidecars (`._<name>`) and `.DS_Store` files in the store (about 230,000 of them next to the blobs on the live store, one per blob); no name of the store begins with a dot, so `compaction --apply` removes every such entry older than `safetyHours` from the blob, head, snapshot, origin and segment directories and reports it as a `litter` progress count.'),
])
