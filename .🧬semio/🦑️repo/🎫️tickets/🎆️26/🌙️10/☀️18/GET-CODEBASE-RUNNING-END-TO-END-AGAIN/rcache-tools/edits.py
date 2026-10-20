# rcache edit set: the single source of truth for overlay.py (verification copies) and apply.py (live repo).
# edit(file, old, new): replace exactly one occurrence of old. region(file, start, end, new): replace [start, end) (end None = end of file).
# New files live under staged/ and are copied as they are.
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"

EDITS = []


def edit(file, old, new):
    EDITS.append({"file": file, "old": old, "new": new})


def region(file, start, end, new):
    EDITS.append({"file": file, "start": start, "end": end, "new": new})


edit(B + "/🔣️policy.json", '  "defaultTool": "nx",\n', r'''  "defaultTool": "nx",
  "remoteCache": {
    "version": 1,
    "environment": {
      "endpoint": "SEMIO_REMOTE_CACHE_ENDPOINT",
      "instance": "SEMIO_REMOTE_CACHE_INSTANCE",
      "access": "SEMIO_REMOTE_CACHE_ACCESS",
      "upload": "SEMIO_REMOTE_CACHE_UPLOAD",
      "compression": "SEMIO_REMOTE_CACHE_COMPRESSION",
      "download": "SEMIO_REMOTE_CACHE_DOWNLOAD",
      "timeoutSeconds": "SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS",
      "retries": "SEMIO_REMOTE_CACHE_RETRIES",
      "tokenEnvironment": "SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT"
    },
    "defaults": {
      "endpoint": null,
      "instance": "main",
      "access": "read-only",
      "upload": true,
      "compression": true,
      "download": "toplevel",
      "timeoutSeconds": 30,
      "retries": 3,
      "tokenEnvironment": "SEMIO_REMOTE_CACHE_TOKEN"
    },
    "server": {
      "host": "127.0.0.1",
      "port": 6090,
      "directory": ".cache/remote-cache",
      "maximumBytes": 4294967296,
      "maximumEntryBytes": 536870912,
      "maximumAgeDays": 30,
      "tokenEnvironment": {
        "read": "SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN",
        "write": "SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN"
      }
    }
  },
''')

SCHEMA = BZ + "/🧬️schema/🔣️.json"
edit(SCHEMA, r'''        "repositoryContents": {
          "garbageCollectionMaximumAgeDays": 3
        },
        "remote": {
          "endpointEnvironment": "SEMIO_BAZEL_REMOTE_CACHE",
          "uploadEnvironment": "SEMIO_BAZEL_REMOTE_CACHE_UPLOAD",
          "credentialHelperEnvironment": "SEMIO_BAZEL_REMOTE_CACHE_CREDENTIAL_HELPER",
          "compression": true,
          "timeoutSeconds": 30,
          "retries": 3
        }
      }
    },
''', r'''        "repositoryContents": {
          "garbageCollectionMaximumAgeDays": 3
        }
      }
    },
''')

region(SCHEMA, '    "CacheEnvironment": {\n', '    "HostPaths": {\n', r'''    "CacheFlags": {
      "description": "Closed Bazel flags derived from the storage layout, the products budget and the neutral remote cache contract. The startup flag binds one output base, and therefore one server, to one workspace directory; the disk cache garbage collector only runs inside such a persistent server, never under --batch. The remote flags are the Bazel rendering of the contract in the build policy; the access token never appears in a flag: it travels as the one environment entry of the Bazel client that the credential helper reads, and the helper holds only the variable name.",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "startup",
        "command",
        "remote",
        "environment"
      ],
      "properties": {
        "startup": {
          "type": "array",
          "minItems": 1,
          "uniqueItems": true,
          "items": {
            "$ref": "#/definitions/CacheFlags/definitions/Flag"
          }
        },
        "command": {
          "type": "array",
          "minItems": 1,
          "uniqueItems": true,
          "items": {
            "$ref": "#/definitions/CacheFlags/definitions/Flag"
          }
        },
        "remote": {
          "type": "boolean"
        },
        "environment": {
          "type": "object",
          "propertyNames": {
            "pattern": "^[A-Z][A-Z0-9_]{0,63}$"
          },
          "additionalProperties": {
            "type": "string",
            "pattern": "^[A-Za-z0-9._~+/=-]{1,4096}$"
          }
        },
        "helper": {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "path",
            "content",
            "mode"
          ],
          "properties": {
            "path": {
              "type": "string",
              "minLength": 1,
              "maxLength": 256
            },
            "content": {
              "type": "string",
              "minLength": 1,
              "maxLength": 1024
            },
            "mode": {
              "enum": [
                420,
                493
              ]
            }
          }
        }
      },
      "definitions": {
        "Flag": {
          "type": "string",
          "pattern": "^--(no)?[a-z][a-z_]*(=[^\\u0000]+)?$",
          "maxLength": 1024
        }
      }
    },
''')

STORAGE = BZ + "/💾️storage/🟦️.ts"
edit(STORAGE,
     'import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";\n',
     'import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";\nimport {resolveRemoteCacheContractV1} from "../../../🌐️remote-cache/🟦️.ts";\nimport {bazelRemoteCacheV1,type BazelCredentialHelperV1} from "../🌐️remote-cache/🟦️.ts";\n')
region(STORAGE, '/** 🌐️ Derives the closed startup and command flags', None, r'''/** 🌐️ Derives the closed startup and command flags for Bazel's disk, repository-content and optional remote caches; the remote part is the Bazel rendering of the neutral remote cache contract (`🌐️remote-cache`) and stays off until an endpoint is configured. The token travels only in `environment`, which the caller hands to the Bazel client and never persists. */
export function bazelCacheFlagsV1(layout:ReturnType<typeof bazelCacheLayoutV1>,workspace:string,options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;skipCache?:boolean;skipRemoteCache?:boolean}>={}):Readonly<{startup:readonly string[];command:readonly string[];remote:boolean;environment:Readonly<Record<string,string>>;helper?:BazelCredentialHelperV1}>{
 const input=options.environment??{},contract=resolveRemoteCacheContractV1(input),command=[`--disk_cache=${layout.actions}`,`--experimental_disk_cache_gc_max_size=${bazelDiskCacheMaximumBytesV1()}`];
 if(options.skipCache)command.push("--noremote_accept_cached");
 const rendering=contract!==undefined&&!options.skipRemoteCache?bazelRemoteCacheV1(contract,input,{helperDirectory:layout.temporary,restoresOutputs:true}):undefined;
 if(rendering)command.push(...rendering.flags);
 const flags={startup:Object.freeze([`--output_base=${bazelServerBaseV1(layout,workspace)}`]),command:Object.freeze(command),remote:rendering!==undefined,environment:rendering?.environment??Object.freeze({}),...(rendering?.helper?{helper:rendering.helper}:{})};
 if(validateJsonSchemaSubset(schema.definitions.CacheFlags,flags,schema).length)throw Error("Closed Bazel cache flags required");return Object.freeze(flags);
}
''')

SCRIPT = BZ + "/📜️script.ts"
edit(SCRIPT, 'await writeChanged(join(workspace,"MODULE.bazel.lock"),modules.lock);',
     'if(cache.helper){await mkdir(dirname(cache.helper.path),{recursive:true});await writeChanged(cache.helper.path,cache.helper.content);await chmod(cache.helper.path,cache.helper.mode);}await writeChanged(join(workspace,"MODULE.bazel.lock"),modules.lock);')
edit(SCRIPT, '(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));',
     '(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));const client=()=>({...environment,...cache.environment});')
edit(SCRIPT, '{signal,control:phaseControl,env:environment})).trim()', '{signal,control:phaseControl,env:client()})).trim()')
edit(SCRIPT, 'signal:control.commandSignal,env:environment,onLine:', 'signal:control.commandSignal,env:client(),onLine:')
edit(SCRIPT, 'signal:control.commandSignal,env:environment}).finally(', 'signal:control.commandSignal,env:client()}).finally(')

NXBOOT = L + "/⚡️caching/🚀️bootstrap/📜️script.ts"
edit(NXBOOT, 'import {resolveBuildSelectionV1} from "../../🏗️build/🧭️invocation/🟦️.ts";\n',
     'import {resolveBuildSelectionV1} from "../../🏗️build/🧭️invocation/🟦️.ts";\nimport {resolveRemoteCacheContractV1} from "../../🏗️build/🌐️remote-cache/🟦️.ts";\nimport {nxRemoteCacheV1} from "./🛠️tools/🌐️remote-cache/🟦️.ts";\n')
edit(NXBOOT, ' if(environment.SEMIO_BUILD_GRAPH_DIGEST){env.NX_FORCE_REUSE_CACHED_GRAPH="false";delete env.SEMIO_NX_GRAPH_SEAL_REUSE;}\n return env;',
     ' if(environment.SEMIO_BUILD_GRAPH_DIGEST){env.NX_FORCE_REUSE_CACHED_GRAPH="false";delete env.SEMIO_NX_GRAPH_SEAL_REUSE;}\n Object.assign(env,nxRemoteCacheV1(resolveRemoteCacheContractV1(environment),environment).environment);\n return env;')

BSCRIPT = B + "/📜️script.ts"
edit(BSCRIPT, r'''    const {withRepositoryCacheStoreV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");
    await withRepositoryCacheStoreV1(root,"dependencies",parent,phaseControl,async(_dependencies,phaseControl)=>{
    if(command==="tools"){''', r'''    if(command==="cache"&&["serve","remote"].includes(resolved.args[1]??"")){
      const {runRemoteCacheCommandV1}=await import("./🌐️remote-cache/⌨️command/🟦️.ts");
      const tool=async()=>selected.tool==="nx"?(await import("../⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts")).nxRemoteCacheToolV1():(await import("./🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts")).bazelRemoteCacheToolV1();
      const result=await runRemoteCacheCommandV1(resolved.args.slice(1),{root,environment:process.env,signal:phaseControl.signal,progress:async stage=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(invocation,"build:cache:"+stage,"running");},write:line=>console.log(line),tool});
      if(result!==undefined)console.log(JSON.stringify(result,null,2));
      return;
    }
    const {withRepositoryCacheStoreV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");
    await withRepositoryCacheStoreV1(root,"dependencies",parent,phaseControl,async(_dependencies,phaseControl)=>{
    if(command==="tools"){''')

edit(".bazelrc", "build --remote_cache_compression\nbuild --remote_timeout=30s\nbuild --remote_retries=3\n", "")


STORAGE_FIXTURE = BZ + "/💾️storage/🧪️tests/🧫️fixtures/🔣️.json"
STORAGE_TEST = BZ + "/💾️storage/🧪️tests/🟦️.ts"

region(STORAGE_FIXTURE, '      "build --remote_cache_compression",\n', '    ],\n    "workspaces": [\n', "")
edit(STORAGE_FIXTURE, '      "build --repo_contents_cache_gc_max_age=3d",\n    ],', '      "build --repo_contents_cache_gc_max_age=3d"\n    ],')
region(STORAGE_FIXTURE, '    "accepted": [\n', '  "hostPaths": [\n', r'''    "accepted": [
      {"name": "default is disk only", "environment": {}, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080"], "remote": false, "tokenEnvironment": {}, "helper": false},
      {"name": "empty values are unset", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "", "SEMIO_REMOTE_CACHE_ACCESS": ""}, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080"], "remote": false, "tokenEnvironment": {}, "helper": false},
      {"name": "remote is read only by default and spells every switch", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090"}, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080", "--remote_cache=http://127.0.0.1:6090/main", "--remote_instance_name=main", "--noremote_upload_local_results", "--remote_cache_compression", "--remote_download_outputs=toplevel", "--remote_timeout=30s", "--remote_retries=3"], "remote": true, "tokenEnvironment": {}, "helper": false},
      {"name": "read-write with a token uses a credential helper and keeps the token out of every flag", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "https://cache.example.test:8443", "SEMIO_REMOTE_CACHE_INSTANCE": "team-a", "SEMIO_REMOTE_CACHE_ACCESS": "read-write", "SEMIO_REMOTE_CACHE_TOKEN": "tok-123"}, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080", "--remote_cache=https://cache.example.test:8443/team-a", "--remote_instance_name=team-a", "--remote_upload_local_results", "--remote_cache_compression", "--remote_download_outputs=toplevel", "--remote_timeout=30s", "--remote_retries=3", "--credential_helper=cache.example.test={helper}"], "remote": true, "tokenEnvironment": {"SEMIO_REMOTE_CACHE_TOKEN": "tok-123"}, "helper": true},
      {"name": "skip remote cache drops the endpoint and the token", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_TOKEN": "tok-123"}, "skipRemoteCache": true, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080"], "remote": false, "tokenEnvironment": {}, "helper": false},
      {"name": "skip cache refuses cached results", "environment": {}, "skipCache": true, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080", "--noremote_accept_cached"], "remote": false, "tokenEnvironment": {}, "helper": false},
      {"name": "skip cache also asks the remote cache nothing", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090"}, "skipCache": true, "command": ["--disk_cache={actions}", "--experimental_disk_cache_gc_max_size=10307502080", "--noremote_accept_cached", "--remote_cache=http://127.0.0.1:6090/main", "--remote_instance_name=main", "--noremote_upload_local_results", "--remote_cache_compression", "--remote_download_outputs=toplevel", "--remote_timeout=30s", "--remote_retries=3"], "remote": true, "tokenEnvironment": {}, "helper": false}
    ],
    "refused": [
      {"name": "endpoint without scheme", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "cache.example.test:443"}, "message": "Closed remote cache contract required"},
      {"name": "endpoint with a foreign scheme", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "grpc://cache.example.test"}, "message": "Closed remote cache contract required"},
      {"name": "endpoint with whitespace", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://cache.example.test --remote_executor=grpc://evil"}, "message": "Closed remote cache contract required"},
      {"name": "upload is not boolean", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_UPLOAD": "yes"}, "message": "Closed remote cache contract required"},
      {"name": "token looks like a flag", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_TOKEN": "a b --evil"}, "message": "Closed remote cache token required"},
      {"name": "the task adapter restores snapshots, so minimal download is refused", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_DOWNLOAD": "minimal"}, "message": "needs remote cache download toplevel or all"}
    ]
  },
''')

region(STORAGE_TEST, 'test("generated Bazel cache flags agree with the fixture', 'test("the repository .bazelrc carries', r'''test("generated Bazel cache flags agree with the fixture, the closed schema and an independent hash oracle",()=>{
 const layout=layoutOf(),validate=new Ajv({strict:true}).compile({definitions:{CacheFlags:componentSchema.definitions.CacheFlags},$ref:"#/definitions/CacheFlags"}),workspace=cache.workspaces[0]!.workspace,helperPath=join(layout.temporary,"remote-cache-credentials."+(process.platform==="win32"?"cmd":"sh"));
 for(const row of cache.workspaces)expect(bazelServerBaseV1(layout,row.workspace)).toBe(join(layout.server,createHash("sha256").update(row.workspace).digest("hex").slice(0,16))),expect(bazelServerBaseV1(layout,row.workspace)).toBe(join(layout.server,row.sha256Prefix));
 expect(bazelServerBaseV1(layout,cache.workspaces[0]!.workspace)).not.toBe(bazelServerBaseV1(layout,cache.workspaces[1]!.workspace));
 expect(()=>bazelServerBaseV1(layout,"relative")).toThrow("Absolute");
 for(const row of cache.accepted){
  const options={environment:row.environment,skipCache:(row as {skipCache?:boolean}).skipCache,skipRemoteCache:(row as {skipRemoteCache?:boolean}).skipRemoteCache},flags=bazelCacheFlagsV1(layout,workspace,options);
  expect(validate(flags),row.name+JSON.stringify(validate.errors)).toBe(true);
  expect(flags.command,row.name).toEqual(row.command.map(flag=>flag.replace("{actions}",layout.actions).replace("{helper}",helperPath)));expect(flags.remote,row.name).toBe(row.remote);expect(flags.startup).toEqual(["--output_base="+bazelServerBaseV1(layout,workspace)]);
  expect(flags.environment,row.name).toEqual(row.tokenEnvironment);expect(flags.helper?.path,row.name).toBe(row.helper?helperPath:undefined);
  expect(flags.command.some(flag=>flag.startsWith("--remote_cache=")),row.name).toBe(row.remote);
  expect(JSON.stringify(flags.command)+JSON.stringify(flags.helper??{}),row.name+" keeps the token out of flags and helper").not.toContain("tok-123");
 }
 for(const row of cache.refused)expect(()=>bazelCacheFlagsV1(layout,workspace,{environment:row.environment}),row.name).toThrow(row.message);
 expect(bazelCacheFlagsV1(layout,workspace,{environment:{PATH:"/usr/bin",HOME:"/home/x"}}).remote).toBe(false);
});

''')


import glob as _glob
import os as _os
for _name in sorted(_glob.glob(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "edits_*.py"))):
    exec(compile(open(_name, encoding="utf8").read(), _name, "exec"), globals())
