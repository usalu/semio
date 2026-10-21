"""Target map of the staged conc-tests set: staged file name -> repo-relative target (new files) and anchored edits."""
import os
V="️"
def e(cp,slug): return chr(cp)+V+slug
FW=e(0x1F9F0,"framework")
MODS=e(0x1F528,"modules")
PROC=e(0x1F3C3,"process")
TESTING=e(0x1F9EA,"testing")
SB_DIR=f"{FW}/{MODS}/{PROC}/{TESTING}/{e(0x1F3D6,'sandbox')}"
IDX=chr(0x1F7E6)+V+".ts"
JSON=chr(0x1F523)+V+".json"
SCHEMA=e(0x1F9EC,"schema")
TESTS=e(0x1F9EA,"tests")
FIX=e(0x1F9EB,"fixtures")
def d(name): return SB_DIR+"/"+name
LAYOUT=e(0x1F4D0,"layout"); IMAGES=e(0x1F5BC,"images"); PORTS=e(0x1F50C,"ports"); RECLAIM=e(0x267B,"reclaim"); ROOT=e(0x1F4C1,"root")
GUARD=e(0x1F6E1,"guard"); PRELOAD=e(0x1F50C,"preload"); SHIMFS=e(0x1FA7B,"fs"); SHIMFSP=e(0x1FA7B,"fs-promises"); SHIMCP=e(0x1FA7B,"child-process")
SHIMSQL=e(0x1FA7B,"sqlite"); SCAN=e(0x1F50D,"scan"); LAW=e(0x2696,"law"); LEDGER=chr(0x1F4D2)+V+"ledger.json"; ORACLE=chr(0x1F40D)+V+"oracle.py"
NEW={
 "sb-protocol.json": d(JSON),
 "sb-schema.json": d(f"{SCHEMA}/{JSON}"),
 "sb-index.ts": d(IDX),
 "sb-layout.ts": d(f"{LAYOUT}/{IDX}"),
 "sb-images.ts": d(f"{IMAGES}/{IDX}"),
 "sb-ports.ts": d(f"{PORTS}/{IDX}"),
 "sb-reclaim.ts": d(f"{RECLAIM}/{IDX}"),
 "sb-root.ts": d(f"{ROOT}/{IDX}"),
 "sb-guard.ts": d(f"{GUARD}/{IDX}"),
 "sb-preload.ts": d(f"{GUARD}/{PRELOAD}/{IDX}"),
 "sb-shim-fs.ts": d(f"{GUARD}/{SHIMFS}/{IDX}"),
 "sb-shim-fsp.ts": d(f"{GUARD}/{SHIMFSP}/{IDX}"),
 "sb-shim-cp.ts": d(f"{GUARD}/{SHIMCP}/{IDX}"),
 "sb-shim-sqlite.ts": d(f"{GUARD}/{SHIMSQL}/{IDX}"),
 "sb-scan.ts": d(f"{SCAN}/{IDX}"),
 "sb-test.ts": d(f"{TESTS}/{IDX}"),
 "sb-corpus.json": d(f"{TESTS}/{FIX}/{JSON}"),
 "sb-oracle.py": d(f"{TESTS}/{ORACLE}"),
 "sb-law.ts": d(f"{TESTS}/{LAW}/{IDX}"),
 "sb-ledger.json": d(f"{TESTS}/{FIX}/{LEDGER}"),
}
if __name__=="__main__":
    for k,v in NEW.items(): print(k,len(v),v)
