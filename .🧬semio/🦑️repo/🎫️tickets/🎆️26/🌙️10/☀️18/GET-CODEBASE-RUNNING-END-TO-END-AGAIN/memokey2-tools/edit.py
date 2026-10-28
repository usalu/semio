"""🎛️ memokey2: the graph memo key drops the options whose values are execution settings (`--parallel=<n>`, `--parallel <n>`: the admission appends the granted job count to the arguments, so every identical request had another key).
Usage: python3 -I edit.py <root> apply|check|revert   (backups in base/<n>, taken on the first apply)."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/"
EDITS = [
    (M + "🧬️schema/🔣️.json", [(
        '        "environmentDenied": [\n',
        '        "argumentsDenied": [\n          "--parallel"\n        ],\n        "environmentDenied": [\n')]),
    (M + "🟦️.ts", [
        (' environmentPrefixes:readonly string[];environmentDenied:readonly string[];',
         ' argumentsDenied:readonly string[];environmentPrefixes:readonly string[];environmentDenied:readonly string[];'),
        ('/** 🔑️ The identity of one request: what is asked, by which implementation, in which environment. Anything that can change the planned slice is a member; volatile process state (pins, pids, plan files) is not. */',
         '/** ✂️ The arguments of a request without the options whose values are execution settings (`--name=value` and `--name value`), which the admission sets per run. */\nexport function memoArgumentsV1(args:readonly string[],denied:readonly string[]=GRAPH_MEMO_POLICY_V1.argumentsDenied):readonly string[]{\n const kept:string[]=[];\n for(let index=0;index<args.length;index++){const name=args[index]!.split("=",1)[0]!;if(!denied.includes(name)){kept.push(args[index]!);continue;}if(!args[index]!.includes("=")&&/^\\d+$/u.test(args[index+1]??""))index++;}\n return kept;\n}\n\n/** 🔑️ The identity of one request: what is asked, by which implementation, in which environment. Anything that can change the planned slice is a member; volatile process state (pins, pids, plan files, the granted job count) is not. */'),
        ('root:parts.root,args:parts.args,', 'root:parts.root,args:memoArgumentsV1(parts.args,policy.argumentsDenied),'),
    ]),
    (M + "🧪️tests/🟦️.ts", [
        ('SEMIO_BUILD_GRAPH_YIELD_ROUND:"2"}})).toBe(key);',
         'SEMIO_BUILD_GRAPH_YIELD_ROUND:"2"}})).toBe(key);\n expect(graphMemoKeyV1({...base,args:[...base.args,"--parallel=6"]})).toBe(key);\n expect(graphMemoKeyV1({...base,args:[...base.args,"--parallel","3"]})).toBe(key);'),
    ]),
]
root, mode = sys.argv[1], sys.argv[2]
status = 0
for n, (rel, pairs) in enumerate(EDITS):
    path, backup = os.path.join(root, rel), os.path.join(HERE, "base", str(n))
    text = open(path, encoding="utf-8").read()
    if mode == "revert":
        shutil.copyfile(backup, path); print("reverted", rel[-50:]); continue
    new = text
    for old, replacement in pairs:
        if replacement in new: continue
        if new.count(old) != 1: print("ANCHOR", rel[-50:], new.count(old), old[:60]); status = 2; continue
        new = new.replace(old, replacement)
    if mode == "check": print("applied" if new == text else "pending", rel[-50:]); continue
    if new != text:
        if not os.path.exists(backup): shutil.copyfile(path, backup)
        tmp = path + ".memokey2"; open(tmp, "w", encoding="utf-8").write(new); os.replace(tmp, path); print("applied", rel[-50:])
sys.exit(status)
