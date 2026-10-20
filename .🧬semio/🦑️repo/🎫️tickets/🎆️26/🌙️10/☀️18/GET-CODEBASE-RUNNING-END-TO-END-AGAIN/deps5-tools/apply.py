#!/usr/bin/env python3
import hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def E(code):
    return chr(code) + "️"


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
J = lambda *parts: os.path.join(*parts)
LIB = J(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
DEPS = J(LIB, E(0x26A1) + "caching", E(0x1F680) + "bootstrap", E(0x1F4E6) + "dependencies", E(0x1F4BE) + "storage")
TS, JSON_, TESTS, SCHEMA = E(0x1F7E6) + ".ts", E(0x1F523) + ".json", E(0x1F9EA) + "tests", E(0x1F9EC) + "schema"
PATHS, FENCE, PROVIDERS = E(0x1F4CF) + "paths", E(0x1F6A7) + "fence", E(0x1F52D) + "providers"

CACHING = J(LIB, E(0x26A1) + "caching")
TARGET = {
    "storage": J(DEPS, TS),
    "caching-script": J(CACHING, E(0x1F9EA) + "tests", E(0x1F52C) + "script-commands", E(0x1F7E6) + ".ts"),
    "caching-project": J(CACHING, E(0x1F4CB) + "project.json"),
    "install": J(DEPS, SCHEMA, E(0x1F4E5) + "install.json"),
}
NEW = {
    "paths.test.ts": J(DEPS, TESTS, PATHS, TS),
    "paths.json": J(DEPS, TESTS, PATHS, JSON_),
    "fence.test.ts": J(DEPS, TESTS, FENCE, TS),
    "fence.json": J(DEPS, TESTS, FENCE, JSON_),
    "providers.test.ts": J(DEPS, TESTS, PROVIDERS, TS),
    "providers.json": J(DEPS, TESTS, PROVIDERS, JSON_),
}

SCAN_CHECK_OLD = 'const check=()=>{requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);checkCustody();for(const row of ancestors)if(!same(row.facts,lstatSync(row.path,{bigint:true})))throw Error("Original installed dependency directory changed");};\n const advance=async()=>{check();if(work>=nextProgress){nextProgress=work+policy.progressWorkUnits;await control.publish({resource:"javascript-installed-paths:"+entries,mode:"shared",elapsedMs:performance.now()-started});check();await control.yieldContinuation();check();}};'
SCAN_CHECK_NEW = 'const light=()=>{requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);checkCustody();};\n const check=()=>{light();for(const row of ancestors)if(!same(row.facts,lstatSync(row.path,{bigint:true})))throw Error("Original installed dependency directory changed");};\n const advance=async()=>{light();if(work>=nextProgress){nextProgress=work+policy.scanProgressWorkUnits;check();await control.publish({resource:"javascript-installed-paths:"+entries,mode:"shared",elapsedMs:performance.now()-started});check();await control.yieldContinuation();check();}};'

FENCE_OLD = 'fence=async(full:boolean=true)=>{checkpoint();await store.recheck();if(full){await custody.recheck();for(const row of projection.plans)recheckJavascriptDependencyInputsV1(row.plan.sources,control);}const current=await readJavascriptDependencyPreservationSourceV1(workspace,context,control,ownership);if(JSON.stringify(current.rows)!==JSON.stringify([...expected.values()])||current.parents.length!==parents.size||current.parents.some((row,index)=>row.path!==[...parents.keys()][index]||!sameJavascriptDependencyParentIdentityV1(row.identity,parents.get(row.path)!))||JSON.stringify(current.ancestors)!==JSON.stringify([...ancestors].map(([path,identity])=>({path,identity}))))throw Error("Exact current Store Source fence changed");if(full){for(const receipt of inputReceipts)await verifyJavascriptDependencyDescriptorGroupV1(store.root,receipt,control,checkpoint);await recheckJavascriptDependencyPreservationsDescriptorV1(held,checkpoint);}checkpoint();},material='
FENCE_NEW = 'fence=async()=>{checkpoint();await store.recheck();await custody.recheck();for(const row of projection.plans)recheckJavascriptDependencyInputsV1(row.plan.sources,control);requireJavascriptDependencyPreservationSourceUnchangedV1(await readJavascriptDependencyPreservationSourceV1(workspace,context,control,ownership),expected,parents,ancestors);for(const receipt of inputReceipts)await verifyJavascriptDependencyDescriptorGroupV1(store.root,receipt,control,checkpoint);await recheckJavascriptDependencyPreservationsDescriptorV1(held,checkpoint);checkpoint();},material='

LOOP_OLD = 'activeIntentSha256=null;await fence(++published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0);}'
LOOP_NEW = 'activeIntentSha256=null;if(++published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{checkpoint();requireJavascriptDependencyEndpointFenceV1(binding.sourceDirectory,expected,parents,ancestors);checkpoint();}}'

PARENT_ANCHOR = '/** \U0001FA9E️ Compares a displaced lock by inode'
PARENT_ANCHOR = '/** \U0001FAAA️ Compares a displaced lock by inode, mode, link count, size and mtime only'
FENCE_FUNCTIONS = (
    '/** \U0001F6A7️ Re-observes only the endpoint a binding just published, its parent identity and its ancestor anchors; every other retained row waits for the next full fence. */\n'
    'export function requireJavascriptDependencyEndpointFenceV1(endpoint:string,expected:ReadonlyMap<string,Readonly<{kind:string;stamp:string|null;target:string|null}>>,parents:ReadonlyMap<string,string>,ancestors:ReadonlyMap<string,string>):void{\n'
    ' const stamp=(row:BigIntStats)=>JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String)),row=expected.get(endpoint),parent=dirname(endpoint),identity=parents.get(parent);\n'
    ' if(!row||row.kind!=="link"||row.stamp===null||identity===undefined)throw Error("Exact current Store Source fence changed");\n'
    ' const facts=lstatSync(endpoint,{bigint:true,throwIfNoEntry:false}),parentFacts=lstatSync(parent,{bigint:true,throwIfNoEntry:false});\n'
    ' if(!facts||!facts.isSymbolicLink()||stamp(facts)!==row.stamp||readProducerBindingLinkV1(endpoint)!==row.target||!parentFacts||!parentFacts.isDirectory()||parentFacts.isSymbolicLink()||!sameJavascriptDependencyParentIdentityV1(stamp(parentFacts),identity))throw Error("Exact current Store Source fence changed");\n'
    ' for(let path=parent;;path=dirname(path)){const known=ancestors.get(path);if(known!==undefined){const current=lstatSync(path,{bigint:true,throwIfNoEntry:false});if(!current||!current.isDirectory()||current.isSymbolicLink()||JSON.stringify([current.dev,current.ino,current.mode].map(String))!==known)throw Error("Exact current Store Source fence changed");}if(path===dirname(path))break;}\n'
    '}\n'
    '/** \U0001F6A6️ Compares a complete fresh preservation Source observation with every retained row, parent identity and ancestor anchor of the running synchronization. */\n'
    'export function requireJavascriptDependencyPreservationSourceUnchangedV1(current:Readonly<{rows:readonly unknown[];parents:readonly Readonly<{path:string;identity:string}>[];ancestors:readonly unknown[]}>,expected:ReadonlyMap<string,unknown>,parents:ReadonlyMap<string,string>,ancestors:ReadonlyMap<string,string>):void{\n'
    ' const keys=[...parents.keys()];\n'
    ' if(JSON.stringify(current.rows)!==JSON.stringify([...expected.values()])||current.parents.length!==parents.size||current.parents.some((row,index)=>row.path!==keys[index]||!sameJavascriptDependencyParentIdentityV1(row.identity,parents.get(row.path)!))||JSON.stringify(current.ancestors)!==JSON.stringify([...ancestors].map(([path,identity])=>({path,identity}))))throw Error("Exact current Store Source fence changed");\n'
    '}\n'
)

RETAIN_OLD_HEAD = 'retain=(path:string,full=false)=>{charge();const facts=lstatSync(path,{bigint:true,throwIfNoEntry:false}),kind='
RETAIN_NEW_HEAD = 'memo=new Map<string,Readonly<{facts:BigIntStats|undefined;row:Readonly<{stamp:string|null;anchor:string|null;kind:string;target:string|null;full:boolean}>}>>(),manifests=new Map<string,Readonly<{debits:number;profile:ReturnType<typeof readJavascriptDependencySourceOwnerManifestProfileV1>}>>(),retain=(path:string,full=false)=>{charge();const cached=memo.get(path);if(cached&&(!full||cached.row.full))return cached;const facts=lstatSync(path,{bigint:true,throwIfNoEntry:false}),kind='
RETAIN_OLD_TAIL = 'else if(useFull&&!before.full)observations.set(path,row);return{facts,row};}'
RETAIN_NEW_TAIL = 'else if(useFull&&!before.full)observations.set(path,row);const retained=Object.freeze({facts,row:observations.get(path)!});memo.set(path,retained);return retained;}'

MANIFEST_OLD_HEAD = 'throw Error("Bounded physical declared provider manifest required");const bytes=withArtifactReadFileSyncV1(leaf.path,check,file=>{'
MANIFEST_NEW_HEAD = 'throw Error("Bounded physical declared provider manifest required");const key=leaf.path+"\\0"+stamp(leaf.facts),hit=manifests.get(key);if(hit){for(let debit=0;debit<hit.debits;debit++)charge();return hit.profile;}const bytes=withArtifactReadFileSyncV1(leaf.path,check,file=>{'
MANIFEST_OLD_TAIL = 'same();retain(leaf.path,true);return bytes;});return readJavascriptDependencySourceOwnerManifestProfileV1(bytes.toString(),charge);};'
MANIFEST_NEW_TAIL = 'same();retain(leaf.path,true);return bytes;});let debits=0;const profile=readJavascriptDependencySourceOwnerManifestProfileV1(bytes.toString(),()=>{debits++;charge();});manifests.set(key,Object.freeze({debits,profile}));return profile;};'

LINEAGE_ANCHOR = '/** \U0001F9F3 Applies only an exact current Source descriptor transition; no Native writer is projected. */'
LINEAGE_FUNCTION = (
    '/** \U0001F6E4\ufe0f Lists every strict ancestor of one path up to the filesystem root, the only directories whose identity can redirect an operation inside it. */\n'
    'export function javascriptDependencyStrictAncestorsV1(path:string):ReadonlySet<string>{const rows=new Set<string>();let current=path;for(;;){const parent=dirname(current);if(parent===current)break;rows.add(parent);current=parent;}return rows;}\n'
)
ANCESTRY_OLD = 'parent=source.parents.find(row=>row.path===parentPath),ancestry=()=>{checkpoint();for(const row of source.ancestors){if(row.path===parentPath)continue;const identity=lstatSync(row.path,{bigint:true});'
ANCESTRY_NEW = 'parent=source.parents.find(row=>row.path===parentPath),lineage=javascriptDependencyStrictAncestorsV1(parentPath),chain=source.ancestors.filter(row=>lineage.has(row.path)),ancestry=()=>{checkpoint();for(const row of chain){const identity=lstatSync(row.path,{bigint:true});'

SUITES = (("javascript-installed-paths", "paths"), ("javascript-endpoint-fence", "fence"), ("javascript-provider-walk", "providers"))
SCRIPT_ANCHOR = '"javascript-storage":"\U0001F680\ufe0fbootstrap/\U0001F4E6\ufe0fdependencies/\U0001F4BE\ufe0fstorage/\U0001F9EA\ufe0ftests/\U0001F7E6\ufe0f.ts",'
SCRIPT_NEW = SCRIPT_ANCHOR + "".join('"%s":"\U0001F680\ufe0fbootstrap/\U0001F4E6\ufe0fdependencies/\U0001F4BE\ufe0fstorage/\U0001F9EA\ufe0ftests/%s/\U0001F7E6\ufe0f.ts",' % (name, {"paths": E(0x1F4CF) + "paths", "fence": E(0x1F6A7) + "fence", "providers": E(0x1F52D) + "providers"}[key]) for name, key in SUITES)
PROJECT_ANCHOR = '    "test-cache-scope": {\n'
PROJECT_NEW = "".join('    "test-%s": {\n      "executor": "repo:script",\n      "cache": false,\n      "outputs": [],\n      "dependsOn": [],\n      "options": {\n        "cwd": "\U0001F9F0\ufe0fframework/\U0001F6CD\ufe0fproducts/\U0001F991\ufe0frepo/\U0001F528\ufe0fmodules/\U0001F4DA\ufe0flibrary/\u26a1\ufe0fcaching",\n        "command": "bun ./\U0001F4DC\ufe0fscript.ts test %s",\n        "forwardAllArgs": false\n      },\n      "metadata": {\n        "semio": {\n          "execution": {\n            "version": 1,\n            "kind": "source"\n          }\n        }\n      }\n    },\n' % (name, name) for name, _ in SUITES) + PROJECT_ANCHOR

PATCHES = {
    "storage": [
        ('work=0,nextProgress=policy.progressWorkUnits;', 'work=0,nextProgress=policy.scanProgressWorkUnits;'),
        (SCAN_CHECK_OLD, SCAN_CHECK_NEW),
        ('  check();if(depth>policy.maximumDepth||path.length>policy.maximumPathUnits||[...path].length>policy.maximumPathUnits)throw Error("Installed dependency path capacity exceeded");', '  light();if(depth>policy.maximumDepth||path.length>policy.maximumPathUnits)throw Error("Installed dependency path capacity exceeded");'),
        ('const facts=lstatSync(path,{bigint:true});check();if(device!==undefined', 'const facts=lstatSync(path,{bigint:true});if(device!==undefined'),
        ('try{const names=await withArtifactReadDirectoryV1(path,check,async directory=>{const names:string[]=[],seen=new Set<string>();for(;;){check();const row=directory.read();check();charge(1);', 'try{const names=await withArtifactReadDirectoryV1(path,light,async directory=>{const names:string[]=[],seen=new Set<string>();for(;;){light();const row=directory.read();charge(1);'),
        ('check();for(const name of names){await visit(join(path,name),portable==="."?name:portable+"/"+name,depth+1,facts.dev);check();}check();}finally{ancestors.pop();}', 'light();for(const name of names){await visit(join(path,name),portable==="."?name:portable+"/"+name,depth+1,facts.dev);light();}if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency directory changed");light();}finally{ancestors.pop();}'),
        (PARENT_ANCHOR, FENCE_FUNCTIONS + PARENT_ANCHOR),
        (FENCE_OLD, FENCE_NEW),
        (LOOP_OLD, LOOP_NEW),
        (RETAIN_OLD_HEAD, RETAIN_NEW_HEAD),
        (RETAIN_OLD_TAIL, RETAIN_NEW_TAIL),
        (LINEAGE_ANCHOR, LINEAGE_FUNCTION + LINEAGE_ANCHOR),
        (ANCESTRY_OLD, ANCESTRY_NEW),
        (MANIFEST_OLD_HEAD, MANIFEST_NEW_HEAD),
        (MANIFEST_OLD_TAIL, MANIFEST_NEW_TAIL),
    ],
    "caching-script": [(SCRIPT_ANCHOR, SCRIPT_NEW)],
    "caching-project": [(PROJECT_ANCHOR, PROJECT_NEW)],
    "install": [
        ('        "maximumWork": 16777216,\n        "progressWorkUnits": 64\n      }\n    },\n    "InstalledPathCase": {', '        "maximumWork": 16777216,\n        "progressWorkUnits": 64,\n        "scanProgressWorkUnits": 4096\n      }\n    },\n    "InstalledPathCase": {'),
    ],
}


def staged(name):
    with open(os.path.join(STAGED, name), "rb") as handle:
        return handle.read()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def absolute(relative):
    return os.path.join(ROOT, relative)


def bases_path():
    return os.path.join(HERE, "bases.json")


def apply_ops(text, ops, name):
    for index, (anchor, replacement) in enumerate(ops):
        found, done = text.count(anchor), text.count(replacement)
        if done == 1 and (found == 0 or anchor in replacement):
            continue
        if found == 1 and done == 0:
            text = text.replace(anchor, replacement)
            continue
        raise SystemExit("anchor %s[%d] occurs %d times and its replacement %d times (expected one of them exactly once): %s" % (name, index, found, done, anchor[:90]))
    return text


def patched_texts(strict=True):
    bases = json.load(open(bases_path()))
    out = {}
    for name, ops in PATCHES.items():
        raw = read(absolute(TARGET[name]))
        if strict and sha(raw) != bases[name]:
            raise SystemExit("target changed since staging: %s (rerun with --rebase to re-anchor the edits onto the current file)" % TARGET[name])
        out[name] = apply_ops(raw.decode("utf-8"), ops, name)
    return out


def sync_running():
    listing = subprocess.run(["ps", "-ww", "-eo", "pid,command"], capture_output=True, text=True).stdout
    found = []
    for line in listing.splitlines()[1:]:
        pid, _, command = line.strip().partition(" ")
        words = command.split()
        if not words or str(os.getpid()) == pid:
            continue
        executable = os.path.basename(words[0])
        if executable == "bun" and ("dependencies sync" in command or "dependencies lock" in command):
            found.append(line)
        elif executable in ("sh", "bash", "zsh") and any(name in command for name in ("deps4-run.sh", "deps4-loop.sh", "deps4-loop2.sh")) and "-c" not in words[1:2]:
            found.append(line)
    return found


def cmd_bases():
    data = {name: sha(read(absolute(path))) for name, path in TARGET.items()}
    json.dump(data, open(bases_path(), "w"), indent=1)
    print(json.dumps(data, indent=1))


def cmd_check(args):
    for name, text in patched_texts("--rebase" not in args).items():
        print("ok", name, len(text))
    for key, path in NEW.items():
        print("new", key, "EXISTS" if os.path.exists(absolute(path)) else "absent", "staged" if os.path.exists(os.path.join(STAGED, key)) else "MISSING")


def cmd_apply(args):
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        raise SystemExit(2)
    for key, path in NEW.items():
        if os.path.exists(absolute(path)) and read(absolute(path)) != staged(key) and "--overwrite-new" not in args:
            raise SystemExit("refusing: new target already exists with different bytes: " + path)
    texts = patched_texts("--rebase" not in args)
    if "--dry-run" in args:
        for name, text in texts.items():
            print("would patch" if read(absolute(TARGET[name])).decode("utf-8") != text else "unchanged", name)
        for key in NEW:
            print("would write", key)
        return
    for name, text in texts.items():
        path = absolute(TARGET[name])
        if read(path).decode("utf-8") == text:
            print("unchanged", name)
            continue
        tmp = path + ".deps5.tmp"
        with open(tmp, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(tmp, path)
        print("patched", name)
    for key, path in NEW.items():
        target = absolute(path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        data = staged(key)
        if not os.path.exists(target) or read(target) != data:
            with open(target, "wb") as handle:
                handle.write(data)
            print("wrote", key)


def overlay(real, mirror, entries):
    os.makedirs(mirror, exist_ok=True)
    children = {}
    for parts, payload in entries:
        children.setdefault(parts[0], []).append((parts[1:], payload))
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name in children:
                continue
            link = os.path.join(mirror, name)
            if not os.path.lexists(link):
                os.symlink(os.path.join(real, name), link)
    for name, rest in children.items():
        if len(rest) == 1 and not rest[0][0]:
            with open(os.path.join(mirror, name), "wb") as handle:
                handle.write(rest[0][1])
        else:
            overlay(os.path.join(real, name), os.path.join(mirror, name), rest)


INSTRUMENT_TAIL = "\nconst __deps5Lstat=(...value:any[])=>{(globalThis as any).__deps5.lstat++;return (lstatSync as any)(...value);};\nconst __deps5Read=(...value:any[])=>{(globalThis as any).__deps5.readFile++;return (withArtifactReadFileSyncV1 as any)(...value);};\n"


def instrument(text):
    import re
    text = re.sub(r"(?<![A-Za-z0-9_$.])lstatSync\(", "__deps5Lstat(", text)
    text = re.sub(r"(?<![A-Za-z0-9_$.])withArtifactReadFileSyncV1\(", "__deps5Read(", text)
    return text + INSTRUMENT_TAIL


def cmd_mirror(args):
    mirror = os.path.abspath(args[0])
    if "--old" in args:
        texts = {name: read(absolute(TARGET[name])).decode("utf-8") for name in PATCHES}
    else:
        texts = patched_texts("--rebase" not in args)
    if "--instrument" in args:
        texts["storage"] = instrument(texts["storage"])
    entries = [(TARGET[name].split(os.sep), text.encode("utf-8")) for name, text in texts.items()]
    entries += [(path.split(os.sep), staged(key)) for key, path in NEW.items() if os.path.exists(os.path.join(STAGED, key))]
    for relative in [args[i + 1] for i, value in enumerate(args) if value == "--copy" and i + 1 < len(args)]:
        for directory, _names, filenames in os.walk(absolute(relative)):
            for filename in filenames:
                full = os.path.join(directory, filename)
                if not os.path.islink(full):
                    entries.append((os.path.relpath(full, ROOT).split(os.sep), read(full)))
    if os.path.lexists(mirror):
        subprocess.run(["rm", "-rf", mirror], check=True)
    overlay(ROOT, mirror, entries)
    print("mirror", mirror)


def cmd_render(args):
    name, out = args[0], os.path.abspath(args[1])
    with open(out, "w", encoding="utf-8") as handle:
        handle.write(patched_texts("--rebase" not in args)[name])
    print("rendered", name, out)


FLAGS = {"--dry-run", "--ignore-sync", "--rebase", "--overwrite-new", "--old", "--instrument"}


def refuse_unknown(args):
    unknown = [word for index, word in enumerate(args) if word.startswith("-") and word not in FLAGS and word != "--copy" and not (index and args[index - 1] == "--copy")]
    if unknown:
        raise SystemExit("refusing unknown flag(s) " + " ".join(unknown))


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    refuse_unknown(sys.argv[2:])
    if command == "bases":
        cmd_bases()
    elif command == "check":
        cmd_check(sys.argv[2:])
    elif command == "apply":
        cmd_apply(sys.argv[2:])
    elif command == "mirror":
        cmd_mirror(sys.argv[2:])
    elif command == "render":
        cmd_render(sys.argv[2:])
    else:
        raise SystemExit("usage: apply.py bases|check [--rebase]|apply [--dry-run] [--rebase] [--ignore-sync] [--overwrite-new]|mirror DIR [--rebase] [--copy RELATIVE_DIR]...|render storage|install OUT")
