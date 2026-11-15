import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/README.md",[
 ('| `originId`, `groupId`, `platform`, `wallMs`, `hlc` | Who wrote it, on which OS, and when (wall clock of the writer). |','| `originId`, `groupId`, `platform`, `wallMs`, `hlc` | Who wrote it, on which OS (`linux`, `macos` or `windows`, like the topology), and when (wall clock of the writer). |'),
 ('| `queue.incoming` | Store winners the tree does not hold yet. |','| `queue.incoming` | Store winners the tree does not hold yet. |\n| `pending`, `failures`, `fallback` | Only when present: a refused or reverted code (`pending`), the counted recurring failures, and a daemon that runs fallback code (`fallback`). |\n\nThe attestation of code that predates the guards (older still: the latency report) is read as present with `guards: null` and the warning `guards-unknown`; `fleet` compares it on code, manifest, head, stage tree and view like any other and never calls it `missing`.'),
])
