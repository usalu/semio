#!/usr/bin/env python3
"""🏗️ Builds the flat staged batch `diskbud` from the CURRENT files of a root (default: the live checkout): new files from `src/`, two anchored edits (graph memo collection, journal service start). Re-run after any drift: `python3 -I make.py [--root DIR]`. Fails (exit 2) when an anchor is missing, never guesses."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "src")
OUT = os.path.join(HERE, "staged", "diskbud")
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
M = f"{L}/🧭️invocation/🕸️graph/🧠️memo"
J = f"{L}/🏡️resident/📜️journal"
NEW = [
    (f"{M}/🧹️retention/🟦️.ts", "memo-retention.ts"),
    (f"{M}/🧹️retention/🧬️schema/🔣️.json", "memo-retention.schema.json"),
    (f"{M}/🧪️tests/🧹️retention/🟦️.ts", "memo-retention-test.ts"),
    (f"{M}/🧪️tests/🧹️retention/🐍️.py", "memo-retention-oracle.py"),
    (f"{J}/🧹️abandoned/🟦️.ts", "abandoned.ts"),
    (f"{J}/🧹️abandoned/🧬️schema/🔣️.json", "abandoned.schema.json"),
    (f"{J}/🧪️tests/🧹️abandoned/🟦️.ts", "abandoned-test.ts"),
    (f"{J}/🧪️tests/🧹️abandoned/🐍️.py", "abandoned-oracle.py"),
]
def once(text, old, new, label):
    if text.count(old) != 1: sys.exit(f"anchor {label}: {text.count(old)} matches (need exactly 1) - the live file drifted")
    return text.replace(old, new)
def edit_memo(text):
    text = once(text, 'import {frontierFactHoldsV1} from "../../../🕸️graph/🕰️generations/🟨️.mjs";\n',
                'import {frontierFactHoldsV1} from "../../../🕸️graph/🕰️generations/🟨️.mjs";\nimport {GRAPH_MEMO_RETENTION_V1,formatGraphMemoCollectionV1,planGraphMemoCollectionV1,type GraphMemoCollectionV1,type GraphMemoFileV1,type GraphMemoRecordV1} from "./🧹️retention/🟦️.ts";\n', "memo import")
    if text.count("collectGraphMemoV1(directory,8);") != 2: sys.exit("anchor memo call sites: need exactly 2")
    text = text.replace("collectGraphMemoV1(directory,8);", "collectGraphMemoV1(directory);")
    start = text.index("/** 🧹️ Keeps the newest `keep` records of a memo directory")
    tail = text[start:]
    if not tail.startswith("/** 🧹️ Keeps the newest `keep` records of a memo directory and removes every plan and relevance file no kept record names (and no younger than a minute). */\nexport function collectGraphMemoV1(directory:string,keep:number):void{") or tail.count("export function") != 1:
        sys.exit("anchor memo collect function: shape drifted")
    new = open(os.path.join(SRC, "memo-collect.txt"), encoding="utf8").read()
    return text[:start] + new
def edit_journal(text):
    text = once(text, '\n} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";\n',
                '\n} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";\nimport {formatJournalSweepV1,sweepAbandonedJournalStatesV1} from "./🧹️abandoned/🟦️.ts";\n', "journal import")
    return once(text, ' takeLock(directory,policy,now());\n',
                ' takeLock(directory,policy,now());\n try{const swept=sweepAbandonedJournalStatesV1(directory,now());if(swept.removed.length)log("sweep",formatJournalSweepV1(swept,"en")+" / "+formatJournalSweepV1(swept,"de"));}catch{}\n', "journal start")
EDITS = [(f"{M}/🟦️.ts", edit_memo), (f"{J}/🟦️.ts", edit_journal)]
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    shutil.rmtree(os.path.join(OUT, "files"), ignore_errors=True); shutil.rmtree(os.path.join(OUT, "base"), ignore_errors=True)
    os.makedirs(os.path.join(OUT, "files")); os.makedirs(os.path.join(OUT, "base"))
    rows = {}
    for index, (rel, source) in enumerate(NEW):
        target = os.path.join(ROOT, rel)
        if os.path.exists(target) and open(target, "rb").read() != open(os.path.join(SRC, source), "rb").read(): sys.exit(f"new file already exists with other bytes: {rel}")
        data = open(os.path.join(SRC, source), "rb").read(); key = f"n{index:02d}"
        open(os.path.join(OUT, "files", key), "wb").write(data); rows[rel] = {"file": key, "base": None, "new": sha(data)}
    for index, (rel, edit) in enumerate(EDITS):
        base = open(os.path.join(ROOT, rel), "rb").read(); key = f"e{index:02d}"
        new = edit(base.decode("utf8")).encode("utf8")
        open(os.path.join(OUT, "base", key), "wb").write(base); open(os.path.join(OUT, "files", key), "wb").write(new)
        rows[rel] = {"file": key, "base": sha(base), "new": sha(new)}
    json.dump(rows, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    title = "🌌 Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern."
    if title not in template: sys.exit("stage template title changed")
    open(os.path.join(OUT, "stage.py"), "w", encoding="utf8").write(template.replace(title, "🧹️ Staged batch `diskbud` (lane diskbud): the graph memo directory stays within a byte budget and a starting journal service removes the state directories of worktrees that no longer run."))
    os.chmod(os.path.join(OUT, "stage.py"), 0o755)
    print("built", len(rows), "files from", ROOT)
main()
