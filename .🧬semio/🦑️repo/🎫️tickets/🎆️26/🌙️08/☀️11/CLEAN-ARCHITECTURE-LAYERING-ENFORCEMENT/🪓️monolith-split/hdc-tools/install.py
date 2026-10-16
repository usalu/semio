"""Hash-guarded install of the staged files into the repo: refuses to overwrite a file that changed since the snapshot."""
import os, sys, json, hashlib
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *

STAGE = SC + "/hdc-stage"
ORIG = SC + "/hdc-orig/🌎️hub/📇️directory"
sha = lambda t: hashlib.sha256(t.encode("utf-8")).hexdigest()


def main():
    done = []
    for dp, dn, fn in os.walk(STAGE):
        for f in fn:
            s = os.path.join(dp, f)
            rel = os.path.relpath(s, STAGE)
            dest = os.path.join(DIR, rel)
            o = os.path.join(ORIG, rel)
            if os.path.exists(dest):
                assert os.path.exists(o), "no snapshot for existing " + rel
                assert sha(read(dest)) == sha(read(o)) or read(dest) == read(s), "live file changed since snapshot: " + rel
            write(dest, read(s))
            done.append("🌎️hub/📇️directory/" + rel)
    json.dump(sorted(done), open(F + "/hdc-tools/installed-files.json", "w"), ensure_ascii=False, indent=1)
    print("installed", len(done))


if __name__ == "__main__":
    main()
