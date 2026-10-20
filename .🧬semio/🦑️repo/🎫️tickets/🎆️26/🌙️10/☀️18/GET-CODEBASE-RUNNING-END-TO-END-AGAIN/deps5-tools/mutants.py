#!/usr/bin/env python3
import os, subprocess, sys, json, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import apply as A
MUT = {
    "close-check-removed": ('if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency directory changed");light();}finally', 'light();}finally'),
    "close-check-ino-only": ('if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency directory changed");light();}finally', 'if(facts.ino!==lstatSync(path,{bigint:true}).ino)throw Error("Original installed dependency directory changed");light();}finally'),
    "frame-checks-removed": ('nextProgress=work+policy.scanProgressWorkUnits;check();await control.publish({resource:"javascript-installed-paths:"+entries,mode:"shared",elapsedMs:performance.now()-started});check();await control.yieldContinuation();check();', 'nextProgress=work+policy.scanProgressWorkUnits;await control.publish({resource:"javascript-installed-paths:"+entries,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();'),
    "listing-after-close-only": ('const names=await withArtifactReadDirectoryV1(path,light,', 'const names=await withArtifactReadDirectoryV1(path,light,'),
    "second-replay-removed": ('await replay();await context.recheck(control);check();await replay();return Object.freeze', 'await replay();await context.recheck(control);check();return Object.freeze'),
    "replays-removed": ('await replay();await context.recheck(control);check();await replay();return Object.freeze', 'await context.recheck(control);check();return Object.freeze'),
    "retain-memo-ignores-full": ('if(cached&&(!full||cached.row.full))return cached;', 'if(cached)return cached;'),
    "file-self-check-removed": ('if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency entry changed");await advance();return;', 'await advance();return;'),
}
if __name__ == "__main__":
    mirror, name = sys.argv[1], sys.argv[2]
    old, new = MUT[name]
    text = A.patched_texts(False)["storage"]
    assert text.count(old) == 1, (name, text.count(old))
    mutated = text.replace(old, new)
    target = os.path.join(mirror, A.TARGET["storage"])
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(mutated)
    print("mutated", name)
