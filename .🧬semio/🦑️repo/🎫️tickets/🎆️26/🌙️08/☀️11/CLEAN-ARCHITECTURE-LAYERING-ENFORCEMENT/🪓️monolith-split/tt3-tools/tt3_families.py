"""📚 Count the diagnostics of each fixed family in a baseline and a final tsc output."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks

def load(path): return blocks(path)

FAMILIES = [
    ("Ambient `ProcessEnv.NODE_ENV` from `next-env.d.ts` (direct)", lambda b: "NODE_ENV" in b["text"] and "ProcessEnv" in b["text"]),
    ("Spawn/exec overloads reduced to `never` (cascade of the above)", lambda b: "on type 'never'" in b["text"] and "ChildProcess" in b["text"]),
    ("docklayoutstore converted test (`any` bag, `Command is not generic`)", lambda b: "docklayoutstore" in b["file"]),
    ("TS2769 `expect(...).toBe/toEqual` expected-type overloads", lambda b: b["code"] == "TS2769" and "(expected:" in b["text"]),
    ("`ScriptControl` passed for `ScriptInvocation` (`runArtifactRustTests` callers)", lambda b: "'ScriptControl' is not assignable to parameter of type 'ScriptInvocation" in b["text"]),
    ("`cacheInternals` / inference `.mjs` without declarations", lambda b: ("cacheInternals" in b["text"] or "inference/🟨️.mjs'" in b["text"]) and b["code"] in ("TS18046", "TS7016", "TS2339")),
    ("`actorIdentityGrant` / wire `verb, line` missing in test configs", lambda b: "actorIdentityGrant" in b["text"] or "verb, line" in b["text"]),
    ("`SQLQueryBindings[]` (bun:sqlite `run` bindings)", lambda b: "SQLQueryBindings[]" in b["text"]),
    ("`new Database(bytes)` instead of `Database.deserialize`", lambda b: "Uint8Array<ArrayBufferLike>' is not assignable to parameter of type 'string'" in b["text"]),
    ("`Cannot find name 'row'` (lost loops in converted tests)", lambda b: b["code"] == "TS2304" and "'row'" in b["msg"]),
    ("`.map.map(` typo and other lost-binding residue", lambda b: "on type '<U>(callbackfn" in b["text"] or "Cannot find name 'assets'" in b["text"] or "Cannot find name 'control'" in b["text"] or "policyReadFileSafe" in b["text"] or "toolJobMountedDispatchOneTurnExact" in b["text"]),
    ("Plugin sqlite snapshot tests importing the wrong owner", lambda b: "Sqlite" in b["text"] and "does not exist on type 'typeof import" in b["text"]),
    ("`assert(validate(x))` narrowing `any` to `unknown`", lambda b: b["code"] == "TS18046" and re.search(r"'(fixture|inputs|policies|caller|hostile|corpus)' is of type 'unknown'", b["text"]) is not None),
    ("`VizOptionValue` from continuous scales in `grid(...)` twins", lambda b: "VizOptionValue | null | undefined)[]' is not assignable to parameter of type 'readonly number[]'" in b["text"]),
    ("Generated frame/DSL guards typed required but parsed optional", lambda b: "🧿️semio" in b["file"] and b["code"] == "TS2322" and ("'number | undefined' is not assignable to type 'number'" in b["text"] or "'string | undefined' is not assignable to type 'string'" in b["text"])),
    ("Window-kit boot message missing `catalog`", lambda b: "Property 'catalog' is missing in type '{ bindingsModuleUrl" in b["text"]),
]

if __name__ == "__main__":
    a, b = load(sys.argv[1]), load(sys.argv[2])
    print("| Family | Before | After |\n| --- | --- | --- |")
    for name, pred in FAMILIES:
        print(f"| {name} | {sum(1 for x in a if pred(x))} | {sum(1 for x in b if pred(x))} |")
