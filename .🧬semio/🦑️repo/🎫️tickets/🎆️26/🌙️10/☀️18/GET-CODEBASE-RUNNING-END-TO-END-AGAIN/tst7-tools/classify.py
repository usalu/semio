import json,re,collections,sys
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/tst7/"
RULES=[
("missing-dependency",r"Cannot find (package|module) '(?!\.)"),
("browser",r"ms-playwright|chromium|Executable doesn't exist|playwright"),
("envelope-gated",r"Original [A-Za-z ]*(required|already received|exhausted|closed|unavailable)|Live Original|Exact original|Exact same-wire|Nested persistent image|Actual workspace capability|Required original|bounded Nx namespace|Build test law body deadline|Isolated build test failed|readIssuedScriptProcessEnvelopeV1|phase\.leaseControl|Dashboard is not installed|Original immutable SDK|Original Nx SDK|Original Source workspace|Exact owned|Actual finite|Caller-owned ticket output|Bounded original|Original Script deadline|Reducing finite Source|Explicit Vitest policy|receiver admission closed|immutable Nx tooling identity|Invalid transaction fixture run identity|physical-cargo-fixture|\\[script\\] \\{\"owner\"|live Store canonical original-owner|Exact finite scanner"),
("needs-cargo-wasm-native",r"cargo metadata|cargo (check|test|build)|rustc|wasm32|wasm-bindgen|native-build receipt|compiler ingress|\.node\b|dlopen|failed to select a version for the requirement|Rust crate Source boundary|debugger-lldb"),
("real-defect",r"ReferenceError|SyntaxError: Export named|is not defined|exceeds 256|JSON Parse error: Unexpected EOF|Cannot find module '\.|ENOENT: no such file"),
]
def cls(r,txt):
    for n,rx in RULES:
        if re.search(rx,txt): return n
    return None
def main():
    d={}
    for n in ("results.jsonl","results2.jsonl","results3.jsonl"):
        for l in open(G+n,encoding="utf-8"):
            r=json.loads(l); d[r["file"]]=r
    R=list(d.values())
    out=[]
    for r in R:
        if r["rc"]==0: r["class"]="green"
        else:
            try: txt=open(r["log"],encoding="utf-8").read()
            except Exception: txt=""
            c=cls(r,txt)
            r["class"]=c or "unclassified"
        out.append(r)
    json.dump(out,open(G+"classified.json","w",encoding="utf-8"),ensure_ascii=False)
    print(collections.Counter(r["class"] for r in out))
main()
