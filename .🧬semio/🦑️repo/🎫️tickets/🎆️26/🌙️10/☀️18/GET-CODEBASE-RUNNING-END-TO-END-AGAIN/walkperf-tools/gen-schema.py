import json, sys, os
E = lambda cp: chr(cp) + "️"
cache, boot, dep, store, gen = E(0x26A1), E(0x1F680), E(0x1F4E6), E(0x1F4BE), E(0x1F9FE)
ws, bun, pay, ts_, cjs_, js_, pol, schm, kind_json, install = E(0x1F5C2), E(0x1F7E6), E(0x1F4E6), E(0x1F7E6), E(0x1F7E8), E(0x1F523), E(0x1F523), E(0x1F9EC), E(0x1F523), E(0x1F4E5)
def p(*a): return "/".join(a)
dep_dir = p(cache + "caching", boot + "bootstrap", dep + "dependencies")
W = ws + "workspaces"
LIB = p(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
build = E(0x1F3D7) + "build"
entries = [
  p(LIB, cache + "caching", boot + "bootstrap", dep + "dependencies", store + "storage", ts_ + ".ts"),
  p(LIB, W, bun + "bun", cjs_ + ".cjs"),
  p(LIB, W, bun + "bun", ts_ + ".ts"),
  p(LIB, cache + "caching", boot + "bootstrap", dep + "dependencies", gen + "generation", ts_ + ".ts"),
  p(LIB, cache + "caching", boot + "bootstrap", dep + "dependencies", gen + "generation", E(0x1F4BE) + "storage", ts_ + ".ts"),
  p(LIB, build, boot + "bootstrap", ts_ + ".ts"),
]
entries = sorted(entries, key=lambda value: value.encode("utf-8"))
mismatch = ["receipt-invalid","evidence-invalid","implementation-changed","runtime-changed","configuration-changed","conditions-open","binding-changed","ready-changed","input-changed","manifest-changed","membership-changed","membership-churn","provider-changed","unreadable","sha-changed"]
sha = {"type":"string","pattern":"^[a-f0-9]{64}$"}
ref = lambda n: {"$ref":"#/$defs/"+n}
strict = lambda props, req=None: {"type":"object","additionalProperties":False,"required":req or list(props.keys()),"properties":props}
schema = {
 "$schema":"http://json-schema.org/draft-07/schema#",
 "$id":"semio:dependencies/generation-receipt/v1",
 "title":"Dependency generation receipt",
 "description":"Schema-first contract of the content-addressed receipt that lets ordinary commands prove an unchanged dependency generation by stat and bounded reads, falling back to the complete ownership and provider walk on any mismatch.",
 "$defs":{
  "Protocol":{"const":{
    "version":1,
    "purpose":"dependency-generation-receipt",
    "directory":"generation",
    "receiptSuffix":".receipt.json",
    "frontierSuffix":".frontier.ndjson",
    "executionSuffix":".execution.ndjson",
    "maximumReceiptBytes":4194304,
    "maximumEvidenceBytes":134217728,
    "maximumRows":524288,
    "maximumSourceFileBytes":8388608,
    "maximumRelist":4096,
    "settleMilliseconds":5000,
    "checkWork":256,
    "progressWork":4096,
    "stampFields":["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"],
    "anchorFields":["dev","ino","mode"],
    "projectionDigestUnits":32,
    "implementation":{
      "entries":entries,
      "extraFiles":[],
      "closureSuffix":".closure.ndjson",
      "maximumFiles":2048,
      "reader":"captureBuildSourceClosureV1"
    },
    "producer":{
      "scopeFiles":[".npmrc"],
      "ambientHomeFiles":[".npmrc",".bunfig.toml"],
      "ambientConfigFiles":[".bunfig.toml"],
      "ambientConfigFallback":".config",
      "storeRoleFiles":{"bun-home":[".npmrc",".bunfig.toml"],"bun-config":[".bunfig.toml"]},
      "maximumFileBytes":1048576,
      "environmentPrefixes":["BUN_","NPM_CONFIG_"],
      "environmentNames":["HTTP_PROXY","HTTPS_PROXY","ALL_PROXY","NO_PROXY","NODE_EXTRA_CA_CERTS","NODE_TLS_REJECT_UNAUTHORIZED","SSL_CERT_FILE","SSL_CERT_DIR"],
      "environmentIgnored":["NPM_CONFIG_USER_AGENT","BUN_INSPECT","BUN_INSPECT_CONNECT_TO","BUN_INSPECT_NOTIFY"]
    },
    "mismatchClasses":mismatch,
    "tiers":[
      {"name":"implementation","cost":"O(closure files)","covers":"every file of the runtime import closure of the walk and fence entries, their module-owned schemas and literal assets, by content digest"},
      {"name":"runtime","cost":"O(1)","covers":"bun version and revision, node version, platform, architecture, installer recipe"},
      {"name":"producer","cost":"O(config files)","covers":"npmrc and bunfig files every installer level could read, and the environment variables that select registries, caches or transport"},
      {"name":"bindings","cost":"O(bindings)","covers":"Source endpoint links, physical generation ancestry, ready receipts"},
      {"name":"inputs","cost":"O(manifests)","covers":"manifests, locks, configuration, patches and their ancestry"},
      {"name":"membership","cost":"O(directories)","covers":"workspace topology by directory stamps with projection-digest relisting"},
      {"name":"execution","cost":"O(observations)","covers":"provider resolution by replaying every recorded physical observation"}
    ]
  }},
  "Sha256":sha,
  "Location":{"type":"string","pattern":"^[wsa]:.+$","maxLength":2048},
  "Scope":{"type":"string","minLength":1,"maxLength":256},
  "MismatchClass":{"enum":mismatch},
  "Evidence":strict({"name":{"type":"string","pattern":"^[a-f0-9]{64}\\.(frontier|execution|closure)\\.ndjson$"},"sha256":sha,"bytes":{"type":"integer","minimum":0},"rows":{"type":"integer","minimum":0}}),
  "Runtime":strict({"bun":{"type":"string"},"revision":{"type":"string"},"node":{"type":"string"},"platform":{"type":"string"},"arch":{"type":"string"},"sha256":sha}),
  "Source":strict({"path":ref("Location"),"kind":{"enum":["file","directory","missing"]},"dev":{"type":"string"},"ino":{"type":"string"},"mode":{"type":"integer","minimum":0},"sha256":{"anyOf":[sha,{"type":"null"}]}}),
  "Implementation":strict({"entries":{"type":"array","minItems":1,"maxItems":64,"items":{"type":"string","minLength":1,"maxLength":2048}},"evidence":ref("Evidence")}),
  "ProducerFile":strict({"path":ref("Location"),"kind":{"enum":["missing","file","other"]},"bytes":{"type":"integer","minimum":0},"sha256":{"anyOf":[sha,{"type":"null"}]}}),
  "ProducerVariable":strict({"name":{"type":"string","minLength":1,"maxLength":256},"sha256":sha}),
  "Producer":strict({"files":{"type":"array","maxItems":4096,"items":ref("ProducerFile")},"environment":{"type":"array","maxItems":512,"items":ref("ProducerVariable")}}),
  "Input":strict({"scope":ref("Scope"),"sources":{"type":"array","maxItems":65536,"items":ref("Source")}}),
  "Text":strict({"path":{"type":"string","minLength":1,"maxLength":2048},"sha256":sha}),
  "Binding":strict({"scope":ref("Scope"),"pairs":{"type":"array","maxItems":4096,"items":strict({"source":ref("Location"),"target":ref("Location")})},"observationSha256":sha}),
  "Ready":strict({"scope":ref("Scope"),"generation":sha,"location":ref("Location"),"fingerprint":{"type":"string","minLength":1,"maxLength":4096}}),
  "Receipt":strict({
    "version":{"const":1},
    "purpose":{"const":"dependency-generation-receipt"},
    "scope":ref("Scope"),
    "implementation":ref("Implementation"),
    "runtime":ref("Runtime"),
    "producer":ref("Producer"),
    "recipeSha256":sha,
    "ownershipSha256":sha,
    "required":{"type":"array","minItems":1,"maxItems":4096,"items":ref("Scope")},
    "inputs":{"type":"array","minItems":1,"maxItems":4096,"items":ref("Input")},
    "membership":strict({"frontier":ref("Evidence"),"texts":{"type":"array","maxItems":65536,"items":ref("Text")}}),
    "bindings":{"type":"array","minItems":1,"maxItems":4096,"items":ref("Binding")},
    "ready":{"type":"array","minItems":1,"maxItems":4096,"items":ref("Ready")},
    "execution":strict({"evidence":ref("Evidence"),"scopes":{"type":"array","maxItems":4096,"items":strict({"scope":ref("Scope"),"sha256":sha})},"runtimeSha256":sha}),
    "scopeReadSha256":sha,
    "sha256":sha
  }),
  "FrontierRows":{"description":"Newline-delimited JSON arrays. Directory row: [\"d\",path,dev,ino,mode,mtimeNs,ctimeNs,settled,projectionDigest]. State row: [\"s\",path,kind,dev,ino,mode] or [\"s\",path,\"missing\"]. Paths are posix relative to the workspace.","type":"array"},
  "ClosureRows":{"description":"Newline-delimited JSON arrays, one per file of the runtime import closure sorted by UTF-8 path bytes: [\"f\",path,sha256,bytes]. Paths are posix relative to the repository root.","type":"array"},
  "ExecutionRows":{"description":"Newline-delimited JSON arrays: [\"o\",location,kind,anchor,stamp,target,full] with kind one of l,d,f,x,m and anchor/stamp as colon-joined decimal fields or null.","type":"array"}
 }
}
out = sys.argv[1]
json.dump(schema, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
open(out, "a").write("\n")
print(len(entries), out)
