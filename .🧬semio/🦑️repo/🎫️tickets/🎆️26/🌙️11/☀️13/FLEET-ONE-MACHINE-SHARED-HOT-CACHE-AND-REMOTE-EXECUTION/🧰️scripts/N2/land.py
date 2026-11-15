"""🛬 Package N2 landing: anchored edits of the path envelope, the lane directory and the compiler wrapper; every anchor must match the live text exactly once."""
import hashlib, json, os, sys, time, tempfile
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/N2/"
W = "/Users/Shared/semio/"
LOCK = W + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landing.lock"
PE = W + "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📏️path-envelope/"
UC = W + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/"
SUITES = W + "🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json"
CACHING = W + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🔣️.json"
log = []
pending = []
def sha(text): return hashlib.sha256(text.encode("utf8")).hexdigest()[:12]
def read(path): return open(path, encoding="utf8", newline="").read()
def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path))
    with os.fdopen(handle, "w", encoding="utf8", newline="") as out: out.write(text)
    os.chmod(temporary, 0o644); os.replace(temporary, path)
def edit(path, pairs):
    text = read(path); before = sha(text)
    for old, new in pairs:
        if text.count(old) != 1: raise SystemExit("ANCHOR " + str(text.count(old)) + " in " + path + ": " + old[:80])
        text = text.replace(old, new)
    pending.append((path, text)); log.append((path.replace(W, ""), before, sha(text)))
def create(path, text):
    if os.path.exists(path): raise SystemExit("EXISTS " + path)
    pending.append((path, text)); log.append((path.replace(W, ""), "-", sha(text)))
def acquire():
    for attempt in range(720):
        try:
            os.mkdir(LOCK); open(LOCK + "/owner", "w").write("N2 " + time.strftime("%H:%M") + " package N2 (path envelope law, lane temporaries)\n"); return
        except FileExistsError:
            time.sleep(5)
    raise SystemExit("lock busy")
def release():
    os.remove(LOCK + "/owner"); os.rmdir(LOCK)
def main():
    acquire()
    try:
        run()
    except BaseException:
        release(); raise
def run():
    scratch_schema = read(S + "pe/🧬️schema/🔣️.json")
    block = scratch_schema[scratch_schema.index('        "maximumDiagnosticPaths": 16,'):scratch_schema.index('    "Unit": {')]
    edit(PE + "🧬️schema/🔣️.json", [('        "maximumDiagnosticPaths": 16\n      }\n    },\n    "Unit": {', block + '    "Unit": {')])
    scratch_code = read(S + "pe/🟦️.ts"); live_code = read(PE + "🟦️.ts")
    old_line = live_code[live_code.index('  paths.add(path.join(out,name+"-"+hash+"."+codegen+"."+disambiguator+".rcgu.o"));'):]
    old_line = old_line[:old_line.index("\n")]
    new_start = scratch_code.index('  const stem=name+"-"+hash,session=')
    new_text = scratch_code[new_start:scratch_code.index("\n", scratch_code.index('  for(const extension of["","exe","pdb"])', new_start))]
    comment_old = "/** 📏️ Models every known selected Cargo metadata, native-library and codegen-object suffix before compiler writes. */"
    comment_new = scratch_code[scratch_code.index("/** 📏️ Models every selected"):scratch_code.index("\n", scratch_code.index("/** 📏️ Models every selected"))]
    edit(PE + "🟦️.ts", [(old_line, new_text), (comment_old, comment_new)])
    scratch_py = read(S + "pe/🧪️tests/🐍️.py"); live_py = read(PE + "🧪️tests/🐍️.py")
    py_old = live_py[live_py.index('        paths.add(str(out / (name + "-" + fingerprint'):]; py_old = py_old[:py_old.index("\n") + 1]
    py_new = scratch_py[scratch_py.index("        stem = name"):scratch_py.index('        for extension in ("", "exe", "pdb")')] if '        for extension in ("", "exe", "pdb")' in scratch_py else None
    if py_new is None: raise SystemExit("py anchor")
    edit(PE + "🧪️tests/🐍️.py", [(py_old, py_new), ('"""📏️ Independent pathlib witness of selected Cargo output path extents."""', '"""📏️ Independent pathlib witness of selected Cargo output path extents, with the codegen temporaries in the lane temporary directory."""')])
    fixtures = PE + "🧪️tests/🧫️fixtures/🔣️.json"; live = read(fixtures)
    if sha(live) != "f54330682df8": raise SystemExit("fixtures changed")
    new_fixtures = read(S + "pe/🧪️tests/fixtures.new.json") + "\n"; pending.append((fixtures, new_fixtures)); log.append((fixtures.replace(W, ""), "f54330682df8", sha(new_fixtures)))
    for relative in ("📐️budget/🟦️.ts", "📐️budget/🧪️tests/🟦️.ts", "📐️budget/🧪️tests/🐍️.py"): create(PE + relative, read(S + "pe/" + relative))
    edit(SUITES, [('        "📦️artifacts/🏗️native-build/📏️path-envelope/🧪️tests/🟦️.ts",\n', '        "📦️artifacts/🏗️native-build/📏️path-envelope/🧪️tests/🟦️.ts",\n        "📦️artifacts/🏗️native-build/📏️path-envelope/📐️budget/🧪️tests/🟦️.ts",\n')])
    edit(UC + "🧬️schema/🔣️.json", [('"digestDigits": 11,', '"digestDigits": 6,'), ('"live": "v",', '"live": "v",\n      "compilerTemporaries": "z",')])
    edit(UC + "🧵️lane/🟦️.ts", [("build:base,live:join(base,layout.live),", "build:base,temporaries:join(base,layout.compilerTemporaries),live:join(base,layout.live),"),
        ("\n/** 🔑️ The affinity key of a crate set", "\n/** 🧹️ Hands rustc the short temporary directory of a lane for its codegen files (`-Z temps-dir`): their names carry the crate name up to three times, so below the unit directory they overran the 256 character bound that the path envelope (`📏️path-envelope`) now models at this place. Other invocations (version and cfg probes) get nothing. */\nexport function compilerTemporaryArgumentsV1(directory:string,args:readonly string[]):readonly string[]{\n if(!args.includes(\"--crate-name\"))return[];\n mkdirSync(directory,{recursive:true});return[\"-Z\",\"temps-dir=\"+directory];\n}\n\n/** 🔑️ The affinity key of a crate set")])
    edit(UC + "📜️script.ts", [('import {lanePathsV1} from "./🧵️lane/🟦️.ts";', 'import {compilerTemporaryArgumentsV1,lanePathsV1} from "./🧵️lane/🟦️.ts";'),
        ("return await run(settings.inner,[...argv,...linkRootArgumentsV1(args,resolve(cwd,unit.outDir))],environment,capture);", "return await run(settings.inner,[...argv,...linkRootArgumentsV1(args,resolve(cwd,unit.outDir)),...compilerTemporaryArgumentsV1(settings.laneRoot?join(settings.laneRoot,policy.layout.compilerTemporaries):lanePathsV1(settings.root,settings.lane).temporaries,args)],environment,capture);")])
    edit(UC + "🧵️lane/🏊️pool/🎯️product/🧪️tests/🟦️.ts", [("key.slice(0,11)", "key.slice(0,6)")])
    create(UC + "🧵️lane/🧪️tests/🟦️.ts", read(S + "new/lane-test.ts"))
    edit(CACHING, [('        "🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts",\n', '        "🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts",\n        "🦀️cargo/🗃️unit-cache/🧵️lane/🧪️tests/🟦️.ts",\n')])
    for path, text in pending: write(path, text)
    for row in log: print(*row)
main()
