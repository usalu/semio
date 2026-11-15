import sys;sys.path.insert(0,__import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from lib import *
Q='🛡️resilience/🧪️tests/🟦️.ts'
sub(Q,'setDefaultTimeout(60_000);','import {blockedFailuresV1,installBootStageV1,listStoreCodeV1,publishCodeSnapshotV1,pruneStoreCodeV1,readBeatV1,readFallbackV1,storeCodeDirectoryV1,verifyCodeSnapshotV1,yieldingV1} from "../🟦️.ts";\nimport {createHash} from "node:crypto";\nimport {utimesSync} from "node:fs";\n\nsetDefaultTimeout(60_000);')
append(Q,files('resilience-tests-append.ts'))
sub('🔁️replica/🧪️tests/🛟️boot.ts','','') if False else None
open(__import__('os').path.join(root(),'🔁️replica','🧪️tests','🛟️boot.ts'),'w',encoding='utf8').write(files('boot-tests.ts'))
sub('🛡️resilience/🧪️tests/🐍️.py','''try:
    import jsonschema''','''for case in FIXTURE["store"]:
    expected=[{"path":f["path"],"bytes":len(f["text"].encode("utf-8")),"sha256":hashlib.sha256(f["text"].encode("utf-8")).hexdigest()} for f in sorted(case["files"],key=lambda item:item["path"].encode("utf-8"))]
    check(expected==case["expected"],"store "+case["name"])

try:
    import jsonschema''')
sub('🛡️resilience/🧪️tests/🐍️.py','print("ok: %d jitter, %d closure, %d texts, %d prune, %d rotate vectors"%(len(FIXTURE["jitter"]),len(FIXTURE["closure"]),len(FIXTURE["texts"]),len(FIXTURE["prune"]),len(FIXTURE["rotate"])))','print("ok: %d jitter, %d closure, %d texts, %d prune, %d rotate, %d store vectors"%(len(FIXTURE["jitter"]),len(FIXTURE["closure"]),len(FIXTURE["texts"]),len(FIXTURE["prune"]),len(FIXTURE["rotate"]),len(FIXTURE["store"])))')
