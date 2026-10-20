# 🏗 Bazel: python, dotnet and cmake tasks are keyed by the pin of their toolchain (the same contract Nx digests); the tests that transpile the fingerprint function receive the new binding.
from layout import *
from patchlib import chain, replace_n

SCRIPT = BZ + "/📜️script.ts"
FINGERPRINT_TEST = BZ + "/🧪️tests/📥️fingerprint/🟦️.ts"
GO_TEST = BZ + "/🐹️go/🧪️tests/🟦️.ts"
FOLD_OLD = r'''if(bazelGoClassV1(task)){const key="\0go";let known=memo.get(key);if(!known){known=bazelGoToolchainDigestV1({...process.env,...original});memo.set(key,known);}digest.update(await known);}'''
FOLD_NEW = FOLD_OLD + r'''{const kind=bazelNativeToolchainClassV1(task);if(kind!==undefined){const key="\0native\0"+kind+"\0"+task.root;let known=memo.get(key);if(!known){known=Promise.resolve(bazelNativeToolchainDigestV1(kind,{...process.env,...original},task.root));memo.set(key,known);}digest.update(await known);}}'''

def script(text):
    return chain(text, [
        ("bazel-native-import", 'import {bazelGoClassV1,bazelGoToolchainDigestV1} from "./🐹️go/🟦️.ts";\n', 'import {bazelGoClassV1,bazelGoToolchainDigestV1} from "./🐹️go/🟦️.ts";\nimport {bazelNativeToolchainClassV1,bazelNativeToolchainDigestV1} from "./🧰️toolchains/🟦️.ts";\n'),
        ("bazel-native-fold", FOLD_OLD, FOLD_NEW),
    ])

def go_test(text):
    return replace_n(text, "bindings={createHash,randomUUID,join,", "bindings={bazelNativeToolchainClassV1:()=>undefined,createHash,randomUUID,join,", 1, "go-test-bindings")

def fingerprint_test(text):
    return replace_n(text, "bindings={createHash,randomUUID,bazelClosureMarkerV1:", "bindings={bazelNativeToolchainClassV1:()=>undefined,createHash,randomUUID,bazelClosureMarkerV1:", 3, "fingerprint-test-bindings")

def edits():
    return [(SCRIPT, script), (GO_TEST, go_test), (FINGERPRINT_TEST, fingerprint_test)]
