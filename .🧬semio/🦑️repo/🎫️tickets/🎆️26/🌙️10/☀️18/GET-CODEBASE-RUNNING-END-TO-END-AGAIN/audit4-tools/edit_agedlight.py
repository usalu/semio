#!/usr/bin/env python3
"""🕰 audit4 `agedlight`: under explicit promotion a light command whose aged self-healing promotion did not move the snapshot (blocked, unstable, busy or a failing promoter) runs the last good snapshot like a heavy one instead of exiting 75 (anchored, exact-count edits; refuses on a lost anchor; a no-op when applied).
usage: python3 -I edit_agedlight.py <root>"""
import os, stat, sys

S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/"
LAW = (
    "\ntest(\"explicit promotion: a light command whose aged self-healing promotion leaves the snapshot where it was (a blocked tree, a promoter that fails) runs the last good snapshot like a heavy command and is never refused\",async()=>{\n"
    " const root=await fresh(),explicit={SEMIO_REPO_SEAL:\"sealed\",FIXTURE_PROMOTION:\"explicit\"},entry=sealedEntryOfV1(root,fixtureEntryV1);\n"
    " expect(await run(root,[\"x\"],explicit)).toMatchObject({code:0,out:\"DOOR A x\\n\"});\n"
    " setFixtureTreeV1(root,\"S\");backdateV1(root,25*60);removeTrampolineFileV1(entry.blocked,{force:true});\n"
    " const blocked=await run(root,[\"leases\",\"status\"],explicit);\n"
    " expect(blocked,\"a blocked aged tree\").toMatchObject({code:0,out:\"DOOR A leases,status\\n\"});expect(blocked.err).toContain(\"tool/helper.ts:1:16\");\n"
    " setFixtureTreeV1(root,\"B\");backdateV1(root,25*60);removeTrampolineFileV1(entry.blocked,{force:true});\n"
    " const broken=await run(root,[\"leases\",\"status\"],{...explicit,FIXTURE_PROMOTER:\"broken\"});\n"
    " expect(broken,\"a promoter that fails\").toMatchObject({code:0,out:\"DOOR A leases,status\\n\"});expect(broken.err).toContain(\"the promoter does not load\");\n"
    "},300_000);\n"
)
EDITS = {
    S + "🚦️trampoline/🟦️.ts": [
        ("  if(light&&pointer&&Date.now()-pointer.promotedAtMs>=policy.light.refuseAfterMilliseconds){",
         "  if(light&&pointer&&policy.promotion.rule!==\"explicit\"&&Date.now()-pointer.promotedAtMs>=policy.light.refuseAfterMilliseconds){"),
        ("and a snapshot still older than `light.refuseAfterMilliseconds` after the promotion attempt is refused with exit 75.",
         "and a snapshot still older than `light.refuseAfterMilliseconds` after the promotion attempt is refused with exit 75 (under the `drift` rule; under explicit promotion a light command whose aged promotion did not move the snapshot runs the last good one, as a heavy command does)."),
    ],
    S + "🧪️tests/🚦️trampoline/🟦️.ts": [
        ("APPEND", LAW),
    ],
}

def write_keep_mode(path, text):
    mode = stat.S_IMODE(os.stat(path).st_mode); tmp = path + ".a4tmp"
    with open(tmp, "w", encoding="utf8", newline="") as f: f.write(text)
    os.chmod(tmp, mode); os.replace(tmp, path)

def main():
    root = os.path.abspath(sys.argv[1]); plans = {}
    for rel, edits in EDITS.items():
        path = os.path.join(root, rel); text = open(path, encoding="utf8", newline="").read()
        for old, new in edits:
            if old == "APPEND":
                if new.strip() not in text: text = text.rstrip("\n") + "\n" + new
                continue
            if new in text: continue
            if text.count(old) != 1: sys.exit(f"anchor count {text.count(old)} != 1 in {rel[-60:]}: {old[:90]!r}")
            text = text.replace(old, new, 1)
        plans[path] = text
    for path, text in plans.items():
        if open(path, encoding="utf8", newline="").read() != text: write_keep_mode(path, text); print("edited", path[-70:])
        else: print("unchanged", path[-70:])
main()
