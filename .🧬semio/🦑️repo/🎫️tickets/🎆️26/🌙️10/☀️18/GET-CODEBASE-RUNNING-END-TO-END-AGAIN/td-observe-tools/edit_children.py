"""🪝️ [td-observe] rewrites the fault injection of the two observation law children (`🧱️blocks/👷️child`, `🪪️presence/👷️child`) from
`require("node:fs").X = wrapper; syncBuiltinESMExports()` to a `bun:test` spy on the `node:fs` module namespace.
Why: Bun (1.3.14 and the pinned 1.4.2) binds builtin ESM imports when `node:fs` is first linked and `syncBuiltinESMExports` does not rebind
them, so the production modules (`import {openSync} from "node:fs"`) never saw the injected races (`mutated: false`); a namespace spy is
rebound live (probe: td-observe-tools/probe/sync-probe3.mjs vs spy-probe.mjs). usage: python3 -I edit_children.py <root> (idempotent, refuses
unknown shapes)."""
import re, sys

ROOT = sys.argv[1]
BASE = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/"
HELPERS = {
    "🪪️presence/👷️child/🟦️.ts": ("🪝️", "♻️"),
    "🧱️blocks/👷️child/🟦️.ts": ("🧷️", "🧽️"),
}
RESTORES = {
    "🪪️presence/👷️child/🟦️.ts": "nativeFs.lstatSync=actualLstat;nativeFs.openSync=actualOpen;nativeFs.fstatSync=actualFstat;syncBuiltinESMExports();",
    "🧱️blocks/👷️child/🟦️.ts": "nativeFs.openSync=actualOpen;nativeFs.closeSync=actualClose;syncBuiltinESMExports();",
}
IMPORT_OLD = 'import{createRequire,syncBuiltinESMExports}from"node:module";'
IMPORT_NEW = 'import{createRequire}from"node:module";\nimport * as fsModule from"node:fs";\nimport{spyOn}from"bun:test";'


def helper(emoji_set: str, emoji_restore: str) -> str:
    return (
        f'\n/** {emoji_set} Interposes one `node:fs` member for every module of this process through the module namespace: Bun binds builtin ESM imports when `node:fs` is first linked and `syncBuiltinESMExports` never rebinds them, while a namespace spy is seen by every importer. */\n'
        'const interposedFsV1:{mockRestore():void}[]=[];\n'
        'function interposeFsV1<K extends "lstatSync"|"openSync"|"fstatSync"|"closeSync">(name:K,body:(typeof import("node:fs"))[K]):void{interposedFsV1.push(spyOn(fsModule,name).mockImplementation(body as never));}\n'
        f'/** {emoji_restore} Restores every interposed `node:fs` member, newest first. */\n'
        'function restoreFsV1():void{while(interposedFsV1.length)interposedFsV1.pop()!.mockRestore();}\n'
    )


def rewrite(text: str, rel: str) -> str:
    if "interposeFsV1" in text:
        return text
    if text.count(IMPORT_OLD) != 1:
        raise SystemExit(f"refused: import anchor in {rel}")
    text = text.replace(IMPORT_OLD, IMPORT_NEW)
    out, at = [], 0
    for match in re.finditer(r"nativeFs\.(lstatSync|openSync|fstatSync|closeSync)=\(", text):
        if match.start() < at:
            continue
        name = match.group(1)
        end_token = f")as typeof nativeFs.{name}"
        end = text.find(end_token, match.end())
        if end < 0:
            raise SystemExit(f"refused: no terminator for {name} in {rel}")
        out.append(text[at:match.start()])
        out.append(f'interposeFsV1("{name}",(')
        at = match.end()
    out.append(text[at:])
    text = "".join(out)
    for name in ("lstatSync", "openSync", "fstatSync", "closeSync"):
        text = text.replace(f")as typeof nativeFs.{name};syncBuiltinESMExports();", f")as typeof nativeFs.{name});")
        text = text.replace(f")as typeof nativeFs.{name};", f")as typeof nativeFs.{name});")
    restore = RESTORES[rel]
    count = text.count(restore)
    if count < 1:
        raise SystemExit(f"refused: restore anchor in {rel}")
    text = text.replace(restore, "restoreFsV1();")
    if "syncBuiltinESMExports" in text or re.search(r"nativeFs\.\w+=\(", text):
        raise SystemExit(f"refused: leftover runtime rebinding in {rel}")
    anchor = text.find("\n", text.find("nativeFs=createRequire(import.meta.url)"))
    if anchor < 0:
        raise SystemExit(f"refused: helper anchor in {rel}")
    return text[:anchor] + helper(*HELPERS[rel]) + text[anchor + 1:]


COLD = {
    "🧱️blocks/👷️child/🟦️.ts": [
        ('import{observeInference,observedFs,readInferenceHeaderV1,inferenceObservationBlockPathV1}from"../../../🟨️.mjs";', 'import{observeInference,observedFs,readInferenceHeaderV1,inferenceObservationBlockPathV1,releaseInferenceMemoryV1}from"../../../🟨️.mjs";'),
        ("readInferenceHeaderV1(cachePath);warm=true;let blockPath=", "readInferenceHeaderV1(cachePath);warm=true;releaseInferenceMemoryV1();let blockPath="),
    ],
}


def cold(text: str, rel: str) -> str:
    """🧊️ The third read must be a cold physical read: the process memo of verified blocks would otherwise serve the warm rows without ever opening the leaf the fault replaces."""
    for old, new in COLD.get(rel, []):
        if new in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"refused: cold anchor in {rel}")
        text = text.replace(old, new)
    return text


for rel in HELPERS:
    path = BASE + rel
    with open(path, encoding="utf8") as handle:
        before = handle.read()
    after = cold(rewrite(before, rel), rel)
    if after != before:
        with open(path, "w", encoding="utf8") as handle:
            handle.write(after)
    print(("rewrote " if after != before else "already ") + rel)
