#!/usr/bin/env python3
import hashlib, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))


def E(code):
    return chr(code) + "\ufe0f"


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
FRAMEWORK = E(0x1F9F0) + "framework"
PRODUCTS = E(0x1F6CD) + "products"
REPO = E(0x1F991) + "repo"
MODULES = E(0x1F528) + "modules"
LIBRARY = E(0x1F4DA) + "library"
CACHING = E(0x26A1) + "caching"
BOOT = E(0x1F680) + "bootstrap"
DEPS = E(0x1F4E6) + "dependencies"
STORAGE = E(0x1F4BE) + "storage"
GENERATION = E(0x1F9FE) + "generation"
GLUE = E(0x1F4BE) + "storage"
SCHEMA = E(0x1F9EC) + "schema"
TESTS = E(0x1F9EA) + "tests"
TS = E(0x1F7E6) + ".ts"
JSON_ = E(0x1F523) + ".json"
CJS = E(0x1F7E8) + ".cjs"
WORKSPACES = E(0x1F5C2) + "workspaces"
BUN = E(0x1F7E6) + "bun"
BUILD = E(0x1F3D7) + "build"

LIB = os.path.join(FRAMEWORK, PRODUCTS, REPO, MODULES, LIBRARY)
DEP_DIR = os.path.join(LIB, CACHING, BOOT, DEPS)
TARGET = {
    "storage": os.path.join(DEP_DIR, STORAGE, TS),
    "bootstrap": os.path.join(LIB, BUILD, BOOT, TS),
    "cjs": os.path.join(LIB, WORKSPACES, BUN, CJS),
    "bunts": os.path.join(LIB, WORKSPACES, BUN, TS),
}
NEW = {
    "core.ts": os.path.join(DEP_DIR, GENERATION, TS),
    "glue.ts": os.path.join(DEP_DIR, GENERATION, GLUE, TS),
    "schema.json": os.path.join(DEP_DIR, GENERATION, SCHEMA, JSON_),
    "test.ts": os.path.join(DEP_DIR, GENERATION, TESTS, TS),
    "fixtures.json": os.path.join(DEP_DIR, GENERATION, TESTS, JSON_),
}

GENERATION_IMPORT = "../" + GENERATION + "/" + GLUE + "/" + TS
BOOT_GENERATION_IMPORT = "../../" + CACHING + "/" + BOOT + "/" + DEPS + "/" + GENERATION + "/" + GLUE + "/" + TS

PATCHES = {
    "storage": [
        (
            "function javascriptDependencySourceOwnerRuntimeV1():string{",
            "export function javascriptDependencySourceOwnerRuntimeV1():string{",
        ),
        (
            "return Object.freeze({repository,model,generations,selectedScopes,relations:Object.freeze(relations),runtimeSha256:javascriptDependencySourceOwnerRuntimeV1()});",
            "return Object.freeze({repository,model,generations,selectedScopes,relations:Object.freeze(relations),runtimeSha256:javascriptDependencySourceOwnerRuntimeV1(),observations:Object.freeze([...observations].map(([path,row])=>Object.freeze({path,stamp:row.stamp,anchor:row.anchor,kind:row.kind,target:row.target,full:row.full})))});",
        ),
        (
            'return Object.freeze({...data,sha256:createHash("sha256").update(text).digest("hex")});',
            'return Object.freeze({...data,sha256:createHash("sha256").update(text).digest("hex"),observations:observed.observations});',
        ),
        (
            'return Object.freeze({version:1 as const,required,projection,receipts:Object.freeze(receipts),sha256:createHash("sha256").update(JSON.stringify([receipts,[...before],final.scopes])).digest("hex")});',
            'return Object.freeze({version:1 as const,required,projection,receipts:Object.freeze(receipts),sha256:createHash("sha256").update(JSON.stringify([receipts,[...before],final.scopes])).digest("hex"),evidence:Object.freeze({scope,execution:final,bindings:Object.freeze([...before])})});',
        ),
        (
            'async readerControl=>withCurrentBunWorkspaceOwnershipV1(workspace,{kind:"all"},{signal:readerControl.signal,',
            'async readerControl=>readJavascriptDependencyGenerationElseV1(workspace,store,readerControl,scope,body,stop,()=>withCurrentBunWorkspaceOwnershipV1(workspace,{kind:"all"},{signal:readerControl.signal,',
        ),
        (
            'changed during consumption");return result;\n }),stop)));',
            'changed during consumption");return result;\n })),stop)));',
        ),
        (
            "/** \U0001F510\ufe0f Captures a mutable produced Bun lock separately from immutable staged-input observations. */",
            "/** \U0001FA99\ufe0f Consumes a verified generation receipt in place of the complete walk; any mismatch runs the complete walk unchanged. */\n"
            "async function readJavascriptDependencyGenerationElseV1<T>(workspace:string,store:RepositoryCacheStoreV1,readerControl:ResourceLeaseControl,scope:string,body:(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>Promise<T>,stop:AbortController,complete:()=>Promise<T>):Promise<T>{\n"
            ' const glue=await import("' + GENERATION_IMPORT + '"),context=await javascriptDependencyReadContextV1(workspace,store),before=await glue.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,readerControl,scope);\n'
            ' if(before.status!=="match")return complete();\n'
            " const result=await withJavascriptDependencyReaderScopeV1(readerControl,actualControl=>body(store,actualControl),stop);await store.recheck();\n"
            " const after=await glue.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,readerControl,scope);\n"
            ' if(after.status!=="match"||after.sha256!==before.sha256)throw Error("Current shared Source-owner generation changed during consumption");\n'
            " return result;\n"
            "}\n"
            "/** \U0001F4EE\ufe0f Publishes each completed scope read as a receipt without ever failing the synchronization that proved it. */\n"
            "async function publishJavascriptDependencyGenerationReceiptsElseV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,custody:Parameters<Parameters<typeof withPublishedBunWorkspaceOwnershipV1>[3]>[0],startedAtMs:number,reads:readonly Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>[]):Promise<void>{\n"
            ' try{const glue=await import("' + GENERATION_IMPORT + '"),observed=custody.observed();for(const read of reads){checkpoint();await glue.publishJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,read,startedAtMs,observed);}}\n'
            ' catch(error){if(control.signal.aborted)throw error;await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency generation receipt not published "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-failure",message:String((error as Error)?.message??error).slice(0,300)})+"\\n",control);}\n'
            "}\n\n"
            "/** \U0001F510\ufe0f Captures a mutable produced Bun lock separately from immutable staged-input observations. */",
        ),
        (
            "started=performance.now(),progress={signal:control.signal,",
            "started=performance.now(),wallStartedAt=Date.now(),progress={signal:control.signal,",
        ),
        (
            "for(const scope of requested)await readJavascriptDependencySourceOwnerScopeReadV1(workspace,context,control,ownership,scope);await fence();",
            "const finals:Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>[]=[];for(const scope of requested)finals.push(await readJavascriptDependencySourceOwnerScopeReadV1(workspace,context,control,ownership,scope));await fence();await publishJavascriptDependencyGenerationReceiptsElseV1(workspace,store,control,checkpoint,custody,wallStartedAt,finals);",
        ),
    ],
    "bootstrap": [
        (
            'custody=>readJavascriptDependencySourceOwnerScopeReadV1(workspace,context,actualControl,{version:1,scopes:custody.scopes},bootstrap.scope));let owner:Awaited<ReturnType<typeof currentOwner>>;try{owner=await currentOwner();}catch(error){if(publishReceipt)throw error;return "material-required";}',
            'async custody=>{const read=await readJavascriptDependencySourceOwnerScopeReadV1(workspace,context,actualControl,{version:1,scopes:custody.scopes},bootstrap.scope);return Object.freeze({...read,observed:custody.observed});}),generation=await import("' + BOOT_GENERATION_IMPORT + '"),walkStartedAt=Date.now(),fast=await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);'
            'let walk:Awaited<ReturnType<typeof currentOwner>>|undefined,owner:Pick<Awaited<ReturnType<typeof currentOwner>>,"receipts"|"sha256">;try{owner=fast.status==="match"?fast:(walk=await currentOwner());}catch(error){if(publishReceipt)throw error;return "material-required";}',
        ),
        (
            'await store.recheck();check();if((await currentOwner()).sha256!==owner.sha256)throw new Error("Current parser Source-owner custody changed");',
            'if(!publishReceipt&&walk)return "receipt-required";await store.recheck();check();const confirmed=walk?await currentOwner():await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);if(("sha256" in confirmed?confirmed.sha256:undefined)!==owner.sha256)throw new Error("Current parser Source-owner custody changed");',
        ),
        (
            'check();if((await currentOwner()).sha256!==owner.sha256)throw new Error("Parser Source-owner changed after receipt");return "ready";',
            'check();const settled=walk?await currentOwner():await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);if(("sha256" in settled?settled.sha256:undefined)!==owner.sha256)throw new Error("Parser Source-owner changed after receipt");if(walk)await generation.publishJavascriptDependencyGenerationReceiptV1(workspace,store,actualControl,check,walk,walkStartedAt);return "ready";',
        ),
    ],
    "cjs": [
        (
            'const kindOf = entry => entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";',
            'const kindOf = entry => entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";\n'
            "/** \U0001F52C\ufe0f Projects the only directory entries that can change ownership, so a listing digest decides whether a cached derivation still holds. */\n"
            'const projectListing = (root, path, entries) => entries.filter(entry => entry.name === "package.json" || !entry.name.startsWith(".") && !outputName(entry.name) && (entry.kind === "symlink" || entry.kind === "directory" && !collectionRoot(root, join(path, entry.name))));\n'
            "/** \U0001F4CB\ufe0f Lists one directory exactly as the ownership walk does and returns its projection. */\n"
            "function readListingProjection(root, path) {\n"
            "  const entries = fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: kindOf(entry) }));\n"
            "  entries.sort(byName);\n"
            "  return projectListing(root, path, entries);\n"
            "}",
        ),
        (
            'record.projected ??= record.entries.filter(entry => entry.name === "package.json" || !entry.name.startsWith(".") && !outputName(entry.name) && (entry.kind === "symlink" || entry.kind === "directory" && !collectionRoot(root, join(path, entry.name))))',
            "record.projected ??= projectListing(root, path, record.entries)",
        ),
        (
            "/** \U0001F9EE\ufe0f Rederives the selected membership and refuses only when scopes, packages or manifests drift, so unrelated concurrent Source edits stay admissible. */",
            "/** \U0001F52D\ufe0f Exposes the exact path states, directory listing projections and manifest digests one ownership computation retained, so a later process can prove the same topology by stamps. */\n"
            "function observedOwnership(owner) {\n"
            '  const paths = [...owner.observations.paths.values()].map(record => Object.freeze({ path: record.path, state: record.state, ...(record.list ? { list: record.list } : {}) })), texts = [...owner.texts].map(([path, text]) => Object.freeze({ path, sha256: createHash("sha256").update(text).digest("hex") }));\n'
            "  return Object.freeze({ root: owner.root, paths: Object.freeze(paths), texts: Object.freeze(texts) });\n"
            "}\n"
            "/** \U0001F9EE\ufe0f Rederives the selected membership and refuses only when scopes, packages or manifests drift, so unrelated concurrent Source edits stay admissible. */",
        ),
        (
            "return { report, recheck: async () => { await validate(); await currentFence(owner, operations, control, current); } };",
            "return { report, owner, recheck: async () => { await validate(); await currentFence(owner, operations, control, current); } };",
        ),
        (
            "try { const result = await body(Object.freeze({ scopes, recheck })); await recheck(); return result; }",
            "try { const result = await body(Object.freeze({ scopes, recheck, observed: () => observedOwnership(current.owner) })); await recheck(); return result; }",
        ),
        (
            "try { result = await body(Object.freeze({ scopes, recheck })); }",
            "try { result = await body(Object.freeze({ scopes, recheck, observed: () => observedOwnership(owner) })); }",
        ),
        (
            "module.exports = { withCurrentBunWorkspaceOwnershipV1,",
            "module.exports = { readListingProjection, withCurrentBunWorkspaceOwnershipV1,",
        ),
    ],
    "bunts": [
        (
            "export type BunWorkspaceInstallationCustodyV1 = Readonly<{scopes:readonly Readonly<{manifest:string;packages:readonly string[]}>[];recheck:()=>Promise<void>}>;",
            "/** \U0001F52D\ufe0f Reports the exact directory listings, path states and manifest digests one ownership computation retained. */\n"
            "export type BunWorkspaceObservedOwnershipV1 = Readonly<{ root: string; paths: readonly Readonly<{ path: string; state: readonly (string | number)[]; list?: readonly Readonly<{ name: string; kind: string }>[] }>[]; texts: readonly Readonly<{ path: string; sha256: string }>[] }>;\n"
            "export type BunWorkspaceInstallationCustodyV1 = Readonly<{scopes:readonly Readonly<{manifest:string;packages:readonly string[]}>[];recheck:()=>Promise<void>;observed:()=>BunWorkspaceObservedOwnershipV1}>;",
        ),
        (
            "  readonly bunRepositoryPackages: (root: string) => readonly string[];",
            "  readonly bunRepositoryPackages: (root: string) => readonly string[];\n  readonly readListingProjection: (root: string, path: string) => readonly Readonly<{ name: string; kind: string }>[];",
        ),
        (
            "=physical.withCurrentBunWorkspaceOwnershipV1;",
            "=physical.withCurrentBunWorkspaceOwnershipV1;\n\n"
            "/** \U0001F52C\ufe0f Lists one directory exactly as the ownership walk does and returns the entries that can change ownership. */\n"
            "export const readBunWorkspaceListingProjectionV1: (root: string, path: string) => readonly Readonly<{ name: string; kind: string }>[] = physical.readListingProjection;",
        ),
    ],
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def absolute(relative):
    return os.path.join(ROOT, relative)


def apply_ops(text, ops, name):
    for index, (anchor, replacement) in enumerate(ops):
        found = text.count(anchor)
        if text.count(replacement) >= 1:
            continue
        elif found == 1:
            text = text.replace(anchor, replacement)
        else:
            raise SystemExit("anchor %s[%d] occurs %d times (expected 1): %s" % (name, index, found, anchor[:90]))
    return text


def bases_path():
    return os.path.join(HERE, "bases.json")


def cmd_bases():
    data = {name: sha(read(absolute(path))) for name, path in TARGET.items()}
    json.dump(data, open(bases_path(), "w"), indent=1)
    print(json.dumps(data, indent=1))


def patched_texts(strict=True):
    bases = json.load(open(bases_path()))
    out = {}
    for name, ops in PATCHES.items():
        path = absolute(TARGET[name])
        raw = read(path)
        if sha(raw) != bases[name] and strict:
            raise SystemExit("target changed since staging: %s (rerun with --rebase to apply anchored edits onto the current file)" % TARGET[name])
        out[name] = apply_ops(raw.decode("utf-8"), ops, name)
    return out


def cmd_check(strict):
    texts = patched_texts(strict)
    for name, text in texts.items():
        print("ok", name, len(text))
    for key, path in NEW.items():
        exists = os.path.exists(absolute(path))
        print("new", key, "EXISTS" if exists else "absent")


def sync_running():
    listing = subprocess.run(["ps", "-ww", "-eo", "pid,command"], capture_output=True, text=True).stdout
    found = []
    for line in listing.splitlines()[1:]:
        pid, _, command = line.strip().partition(" ")
        words = command.split()
        if not words or str(os.getpid()) == pid:
            continue
        executable = os.path.basename(words[0])
        if executable == "bun" and "dependencies sync" in command:
            found.append(line)
        elif executable in ("sh", "bash", "zsh") and any(name in command for name in ("deps4-run.sh", "deps4-loop.sh")) and "-c" not in words[1:2]:
            found.append(line)
    return found


def cmd_apply(args):
    strict = "--rebase" not in args
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        raise SystemExit(2)
    for key, path in NEW.items():
        if os.path.exists(absolute(path)) and read(absolute(path)) != read(os.path.join(HERE, "staged", key)) and "--overwrite-new" not in args:
            raise SystemExit("refusing: new target already exists with different bytes: " + path)
    texts = patched_texts(strict)
    if "--dry-run" in args:
        for name, text in texts.items():
            print("would patch" if read(absolute(TARGET[name])).decode("utf-8") != text else "unchanged", name)
        for key, path in NEW.items():
            print("would write", key, path)
        return
    for name, text in texts.items():
        path = absolute(TARGET[name])
        if read(path).decode("utf-8") == text:
            print("unchanged", name)
            continue
        tmp = path + ".walkperf.tmp"
        with open(tmp, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(tmp, path)
        print("patched", name)
    for key, path in NEW.items():
        target = absolute(path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        data = read(os.path.join(HERE, "staged", key))
        if not os.path.exists(target) or read(target) != data:
            with open(target, "wb") as handle:
                handle.write(data)
            print("wrote", key)


LITERAL_ASSET = re.compile(r"""(["'])(\.{1,2}/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1""")
IMPORT_SPECIFIER = re.compile(r"""(?:from\s*|import\s*\(\s*|require\(\s*|import\s+)["']([^"']+)["']""")
SCHEMA_DIRECTORY = E(0x1F9EC) + "schema"


def closure_entries():
    schema = json.load(open(os.path.join(HERE, "staged", "schema.json"), encoding="utf-8"))
    implementation = schema["$defs"]["Protocol"]["const"]["implementation"]
    return implementation["entries"], implementation["extraFiles"]


def closure_files(virtual):
    entries, extra = closure_entries()

    def read(path):
        if path in virtual:
            return virtual[path]
        full = absolute(path)
        return read_bytes(full) if os.path.isfile(full) else None

    seen, pending = {}, list(entries) + list(extra)
    while pending:
        path = pending.pop()
        if path in seen:
            continue
        data = read(path)
        if data is None:
            raise SystemExit("closure file missing: " + path)
        seen[path] = data
        directory = os.path.dirname(path)
        schema_dir = os.path.join(directory, SCHEMA_DIRECTORY)
        if os.path.isdir(absolute(schema_dir)):
            for base, _names, filenames in os.walk(absolute(schema_dir)):
                for filename in filenames:
                    full = os.path.join(base, filename)
                    if filename.endswith(".json") and not os.path.islink(full):
                        pending.append(os.path.relpath(full, ROOT))
        if path.endswith(".json"):
            continue
        text = data.decode("utf-8", errors="replace")
        for match in LITERAL_ASSET.finditer(text):
            target = os.path.normpath(os.path.join(directory, match.group(2)))
            if read(target) is not None:
                pending.append(target)
        for specifier in IMPORT_SPECIFIER.findall(text):
            if specifier.startswith("."):
                target = os.path.normpath(os.path.join(directory, specifier))
                if read(target) is not None and re.search(r"\.(?:[cm]?[jt]sx?|json)$", target):
                    pending.append(target)
    return seen


def read_bytes(path):
    with open(path, "rb") as handle:
        return handle.read()


def overlay(real, mirror, entries, depth_rel):
    os.makedirs(mirror, exist_ok=True)
    children = {}
    for parts, payload in entries:
        children.setdefault(parts[0], []).append((parts[1:], payload))
    names = set(children)
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name in names:
                continue
            link = os.path.join(mirror, name)
            if not os.path.lexists(link):
                os.symlink(os.path.join(real, name), link)
    for name, rest in children.items():
        if len(rest) == 1 and not rest[0][0] and rest[0][1] is None:
            os.makedirs(os.path.join(mirror, name), exist_ok=True)
        elif len(rest) == 1 and not rest[0][0]:
            with open(os.path.join(mirror, name), "wb") as handle:
                handle.write(rest[0][1])
        else:
            overlay(os.path.join(real, name), os.path.join(mirror, name), rest, depth_rel + [name])


def cmd_mirror(args):
    mirror = os.path.abspath(args[0])
    texts = patched_texts("--rebase" not in args)
    entries = []
    for name, text in texts.items():
        entries.append((TARGET[name].split(os.sep), text.encode("utf-8")))
    for key, path in NEW.items():
        entries.append((path.split(os.sep), read(os.path.join(HERE, "staged", key))))
    policy = os.path.join(LIB, WORKSPACES, E(0x1F523) + "policy.json")
    entries.append((policy.split(os.sep), read(absolute(policy))))
    if "--closure" in args:
        staged = {path: payload for parts, payload in entries for path in ["/".join(parts)]}
        chosen = closure_files(staged)
        for path, payload in chosen.items():
            if path not in staged:
                entries.append((path.split("/"), payload))
        print("closure", len(chosen))
        for path in list(chosen):
            schema_dir = os.path.join(os.path.dirname(path), SCHEMA_DIRECTORY)
            for base, directories, filenames in os.walk(absolute(schema_dir)) if os.path.isdir(absolute(schema_dir)) else []:
                for directory in directories:
                    relative = os.path.relpath(os.path.join(base, directory), ROOT)
                    if relative not in chosen and relative not in staged and not os.path.islink(os.path.join(base, directory)) and not os.listdir(os.path.join(base, directory)):
                        entries.append((relative.split("/"), None))
                        chosen[relative] = b""
                if base == absolute(schema_dir) and not filenames and not directories:
                    entries.append((os.path.relpath(base, ROOT).split("/"), None))
                for filename in filenames:
                    relative = os.path.relpath(os.path.join(base, filename), ROOT)
                    if relative not in chosen and relative not in staged and not os.path.islink(os.path.join(base, filename)):
                        entries.append((relative.split("/"), read(os.path.join(base, filename))))
                        chosen[relative] = b""
    copies = [args[i + 1] for i, value in enumerate(args) if value == "--copy" and i + 1 < len(args)]
    for relative in copies:
        base = absolute(relative)
        for directory, _names, filenames in os.walk(base):
            for filename in filenames:
                full = os.path.join(directory, filename)
                if os.path.islink(full):
                    continue
                entries.append((os.path.relpath(full, ROOT).split(os.sep), read(full)))
    if os.path.lexists(mirror):
        subprocess.run(["rm", "-rf", mirror], check=True)
    overlay(ROOT, mirror, entries, [])
    print("mirror", mirror)


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    if command == "bases":
        cmd_bases()
    elif command == "check":
        cmd_check("--rebase" not in sys.argv)
    elif command == "apply":
        cmd_apply(sys.argv[2:])
    elif command == "mirror":
        cmd_mirror(sys.argv[2:])
    else:
        raise SystemExit("usage: walkperf.py bases|check [--rebase]|apply [--dry-run] [--rebase] [--ignore-sync] [--overwrite-new]|mirror DIR [--rebase] [--closure] [--copy RELATIVE_DIR]...")
