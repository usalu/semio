"""Anchored edits of the live Bazel adapter for the hub set: the Go toolchain identity joins the action key of every Go-family task."""
import re

CARGO_IMPORT=re.compile(r'^import \{bazelCargoClassV1,bazelCargoToolchainDigestV1,BAZEL_CARGO_RULE_V1\} from "\./[^"]*cargo/[^"]*";\n',re.M)
KEY='digest.update(await known);}'
KEY_NEW='digest.update(await known);}if(bazelGoClassV1(task)){const key="\\0go";let known=memo.get(key);if(!known){known=bazelGoToolchainDigestV1({...process.env,...original});memo.set(key,known);}digest.update(await known);}'

def once(text,needle,label):
    count=text.count(needle)
    if count!=1: raise SystemExit(f'anchor {label}: expected exactly one occurrence, found {count}')

def apply(text,go_import='./\U0001F439️go/\U0001F7E6️.ts'):
    imports=CARGO_IMPORT.findall(text)
    if len(imports)!=1: raise SystemExit(f'anchor cargo import: expected exactly one line, found {len(imports)}')
    once(text,KEY,'fingerprint key')
    out=text.replace(imports[0],imports[0]+f'import {{bazelGoClassV1,bazelGoToolchainDigestV1}} from "{go_import}";\n',1)
    out=out.replace(KEY,KEY_NEW,1)
    return out

BINDING='bazelCargoToolchainDigestV1:async()=>""'
BINDING_NEW='bazelCargoToolchainDigestV1:async()=>"",bazelGoClassV1:()=>false,bazelGoToolchainDigestV1:async()=>""'

def apply_fingerprint_test(text):
    if text.count(BINDING)!=2: raise SystemExit(f'anchor fingerprint test bindings: expected exactly two occurrences, found {text.count(BINDING)}')
    return text.replace(BINDING,BINDING_NEW)

def anchors(): return ['cargo import','fingerprint key','fingerprint test bindings x2']
