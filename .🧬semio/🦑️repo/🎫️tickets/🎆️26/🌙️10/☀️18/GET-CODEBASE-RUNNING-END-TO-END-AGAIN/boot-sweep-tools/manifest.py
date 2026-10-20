"""Shared manifest of the boot-sweep staged set: staged file name -> repo-relative path. Folder names are emoji + U+FE0F + word."""
import os
HERE=os.path.dirname(os.path.abspath(__file__))
ST=HERE+"/staged"
def n(code,word):return chr(code)+"️"+word
S="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
PROCESS="🧰️framework/🔨️modules/🏃️process"
BOOT=S+"/🌐️platform/🥾️boot"
REC=BOOT+"/"+n(0x1FA79,"recovery")
JS="🟦️.ts"
NEW={
 "index.ts":REC+"/"+JS,
 "plan.ts":REC+"/"+n(0x1F4CB,"plan")+"/"+JS,
 "ledger.ts":REC+"/"+n(0x1F4D2,"ledger")+"/"+JS,
 "observe.ts":REC+"/"+n(0x1F4E1,"observe")+"/"+JS,
 "act.ts":REC+"/"+n(0x1F9EF,"act")+"/"+JS,
 "clock.ts":REC+"/"+n(0x1F570,"clock")+"/"+JS,
 "heal.ts":REC+"/"+n(0x1FA7A,"heal")+"/"+JS,
}
NEW["schema.json"]=REC+"/"+n(0x1F9EC,"schema")+"/"+chr(0x1F523)+"️.json"
TESTS=REC+"/"+n(0x1F9EA,"tests")
NEW.update({
 "test.ts":TESTS+"/"+JS,
 "corpus.json":TESTS+"/"+chr(0x1F523)+"️.json",
 "oracle.py":TESTS+"/"+n(0x1F40D,"oracle")+".py",
 "heal-test.ts":TESTS+"/"+n(0x1FA7A,"heal")+"/"+JS,
 "wiring-test.ts":TESTS+"/"+n(0x1F517,"wiring")+"/"+JS,
})
CRASH=TESTS+"/"+n(0x1F4A5,"crash")
NEW.update({
 "crash-preload.ts":CRASH+"/"+n(0x1F50C,"preload")+".ts",
 "crash-child.ts":CRASH+"/"+n(0x1F3C3,"child")+".ts",
 "crash-scenarios.ts":CRASH+"/"+n(0x1F3AC,"scenarios")+".ts",
 "crash-test.ts":CRASH+"/"+JS,
})
