#!/usr/bin/env python3
"""✂️ [audit12] anchored edits of the audit12 fix batches (each anchor must occur exactly once; an already-applied edit is skipped): python3 -I edits.py <batch> <root>  (exit 1 = anchor lost)"""
import sys
P = "🧰️framework/🔨️modules/🏃️process"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
RESET = 'finally { if (chosen === undefined) delete process.env.SEMIO_LANGUAGE; else process.env.SEMIO_LANGUAGE = chosen; }'
BATCHES = {
 "sigexit": {f"{P}/🧭️routing/📥️invocation/🏃️process/🟦️.ts": [(
  'if(ended!==undefined){try{process.stderr.write(scriptProcessWatchdogNoticeV1("cancelled",{signal:ended.signal},firstFailureLanguagesV1(process.env)));}catch{}process.exit(ended.code);}throw error;',
  'if(ended!==undefined){const code=ended.code,quit=(reason:unknown)=>{if(reason!==error)throw reason;process.exit(code);};try{process.stderr.write(scriptProcessWatchdogNoticeV1("cancelled",{signal:ended.signal},firstFailureLanguagesV1(process.env)));}catch{}process.once("uncaughtException",quit);process.once("unhandledRejection",quit);}throw error;')]},
 "farmexports": {f"{L}/🏗️build/👁️observation/🧪️tests/🧫️fixtures/🔣️.json": [
  ('      "adoptVerifiedBlockV1",\n', '      "adoptInferenceAuthorityV1",\n      "adoptVerifiedBlockV1",\n'),
  ('      "inferenceAuthorityDigestV1",\n', '      "inferenceAuthorityDigestV1",\n      "inferenceAuthorityRowsV1",\n')]},
 "saytest": {f"{P}/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts": [(
  f']); }} {RESET}\n  expect(diagnostics.finish(true, path).slice(1)).toEqual(rows.slice(-spec.newest));\n',
  f']); expect(diagnostics.finish(true, path).slice(1)).toEqual(rows.slice(-spec.newest)); }} {RESET}\n')]},
 "typedecl": {
  f"{L}/🏗️build/📥️caller/🛰️service/🔑️session/🟨️.d.mts": [('export function canonicalRootV1(root:string):string;', 'export function canonicalRootV1(root:string,platform?:string):string;')],
  f"{L}/🏗️build/🚪️door/🟦️.ts": [('languages=tag==="en"||tag==="de"?[tag]:["en","de"] as const', 'languages:readonly ("en"|"de")[]=tag==="en"||tag==="de"?[tag]:["en","de"]')],
  f"{L}/🏗️build/🗂️suites/🧪️tests/🟦️.ts": [('expect(["prototype","release"]).toContain(facts.buildMode)', 'expect(["prototype","release"]).toContain(String(facts.buildMode))')]},
 "privparent": {f"{L}/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts": [
  ('import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";\n', 'import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";\nimport {tmpdir} from "node:os";\n'),
  ("/** 📍️ The scratch parents a private repository may live below, shortest first: an explicit parent, the checkout's own test scratch and the artifact directory; the deepest store path must fit 256 units, so the first parent that leaves room wins. */",
   "/** 📍️ The scratch parents a private repository may live below, shortest first: an explicit parent, the checkout's own test scratch, the artifact directory and a short system scratch (`/tmp/sg`; Windows: `<temp>/sg`); the deepest store path (an observation block's staging file: pid of up to 7 and a millisecond clock of 13 digits) must fit 256 units, so the first parent that leaves room wins. */"),
  ('...(artifacts?[artifacts]:[])].map(', '...(artifacts?[artifacts]:[]),join(process.platform==="win32"?tmpdir():"/tmp","sg")].map('),
  ('".br.9999999.99.tmp").length;', '".br."+"9".repeat(7)+"."+"9".repeat(13)+".tmp").length;')]},
}
def main():
    batch, root = sys.argv[1], sys.argv[2]
    for rel, pairs in BATCHES[batch].items():
        path = root + "/" + rel
        text = open(path, encoding="utf8").read()
        for old, new in pairs:
            if text.count(new) == 1 and (old not in new or text.count(old) == 1): continue
            if text.count(old) != 1: sys.exit(f"anchor lost ({text.count(old)}x): {rel}: {old[:60]}")
            text = text.replace(old, new)
        open(path, "w", encoding="utf8").write(text)
if __name__ == "__main__": main()
