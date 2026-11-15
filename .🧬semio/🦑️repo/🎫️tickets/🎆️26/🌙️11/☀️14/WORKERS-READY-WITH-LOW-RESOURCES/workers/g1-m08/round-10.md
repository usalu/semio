# Round 10 — g1-m08 — Complete

**Ready: false.** Final native_rc=7; completion marker exists: False. Wrapper exit is not used as native_rc.

Try 1 received 413 of 520 requested projects and reached inference. The native project guard stopped @semio-tech/framework-job-rs (language:cargo) at 2696 MB growth against its 1756 MB budget. An inner command exited 1 and a new host started. Its final request of 201 projects received 0 within 901 s: native_rc=75. After 120 s try 2 received 413 of 424 and advanced graph verification; coordinator round 11 superseded it. Exactly one observer SIGINT at 23:53:07 UTC, then a minute wait; native_rc=7. No resource stop, consent result or outage exemption.

## Machine and pre-run observations

Memory 17179869184 bytes (16 GiB); macOS 27.2 build 26B123, Apple M4 arm64. Replica digest eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc; bootVersion 6; health age 2.368 seconds. Replica running, observer replica kickstarts 0; graph kickstarts 0; gc 0. Full processes and last eight sidecar lines: round-10-preflight-observation.json.

Free disk before:
~~~text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  85109872 114418944    43%  832825 1144189440    0%   /System/Volumes/Data
~~~

Free disk after:
~~~text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  85227032 114301784    43%  832999 1143017840    0%   /System/Volumes/Data
~~~

## Attempts

| Try | Start UTC | Controller end UTC | Wall s | native_rc | Wanted / arrived / missing / fleet s | Counted | Outage exempt | Retry wait s |
|---|---|---|---:|---:|---|---:|---|---:|
| 1 | 2026-11-14T23:16:35Z | 2026-11-14T23:43:01Z | 1586.192 | 75 | 201 / 0 / 201 / 901 | 1 | False | 120.005 |
| 2 | 2026-11-14T23:45:01Z | 2026-11-14T23:54:08Z | 546.598 | 7 | No final refusal totals | 2 | False | 0 |

Final native logged end: 23:53:07 UTC; observed wall includes the grace minute. No try 3 after supersession. Known fleet listeners 2049, 6094 and 8981 reachable before and at both completions, so no server-move outage is inferred.

## Try 1

Launcher **UNCHANGED** during the try and from preceding final identity (measured flags False, False). Inode 1262266, mtime 2026-11-14T21:00:20.886970+00:00, target inode 1262265, target mtime_ns 1794690020871943656, readlink versions/cf25f2b751f83336/repo. Required exact ls -li/readlink/codesign outputs:
~~~json
{
  "before": {
    "at": "2026-11-14T23:16:35Z",
    "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
    "paths": {
      "/Users/ueli/.local/share/semio/launcher/bin/repo": {
        "exists": true,
        "is_symlink": true,
        "inode": 1262266,
        "mtime_ns": 1794690020886970333,
        "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
        "target_inode": 1262265,
        "target_mtime_ns": 1794690020871943656,
        "size": 6523360,
        "readlink": "versions/cf25f2b751f83336/repo"
      },
      "/Users/ueli/.local/bin/repo": {
        "exists": true,
        "is_symlink": false,
        "inode": 1256211,
        "mtime_ns": 1794688211341948235,
        "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
        "target_inode": 1256211,
        "target_mtime_ns": 1794688211341948235,
        "size": 18657778,
        "readlink": null
      }
    },
    "codesign_rc": 0,
    "codesign_lines": [
      "Identifier=tech.semio.repo",
      "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
      "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "TeamIdentifier=not set"
    ]
  },
  "after": {
    "at": "2026-11-14T23:43:01Z",
    "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
    "paths": {
      "/Users/ueli/.local/share/semio/launcher/bin/repo": {
        "exists": true,
        "is_symlink": true,
        "inode": 1262266,
        "mtime_ns": 1794690020886970333,
        "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
        "target_inode": 1262265,
        "target_mtime_ns": 1794690020871943656,
        "size": 6523360,
        "readlink": "versions/cf25f2b751f83336/repo"
      },
      "/Users/ueli/.local/bin/repo": {
        "exists": true,
        "is_symlink": false,
        "inode": 1256211,
        "mtime_ns": 1794688211341948235,
        "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
        "target_inode": 1256211,
        "target_mtime_ns": 1794688211341948235,
        "size": 18657778,
        "readlink": null
      }
    },
    "codesign_rc": 0,
    "codesign_lines": [
      "Identifier=tech.semio.repo",
      "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
      "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "TeamIdentifier=not set"
    ]
  }
}
~~~

Peak run-owned RSS 5071712 KiB (4.837 GiB); machine-wide sampled peak 5133056 KiB. Required 30-second top-three samples 53, including PID/PPID/full commands and ancestry. Five-second pressure counts {"1": 314}; non-normal 30-second samples []. No resource stop; thresholds RSS 8 GiB, critical 4 twice at 30 seconds, warning 2 six times.

Succeeded setup tasks: none emitted. Failed Nx setup tasks: none emitted. Not-run task names: not measured, because no downstream result list was emitted. Internal graph inference-unit failure is quoted separately.

First / last observed arrived lines:
~~~text
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
~~~

First native refusal plus 15 preceding lines, or final interruption context if no refusal:
~~~text
Reading the project graph slice (909 s): flush 1 of 1.
Reading the project graph slice (911 s): flush 1 of 1.
Reading the project graph slice (913 s): flush 1 of 1.
Reading the project graph slice (915 s): flush 1 of 1.
Reading the project graph slice (917 s): flush 1 of 1.
Reading the project graph slice (919 s): flush 1 of 1.
Reading the project graph slice (921 s): flush 1 of 1.
Reading the project graph slice (923 s): flush 1 of 1.
Reading the project graph slice (925 s): flush 1 of 1.
Reading the project graph slice (927 s): flush 1 of 1.
Reading the project graph slice (929 s): flush 1 of 1.
Reading the project graph slice (931 s): flush 1 of 1.
[repo] still working after 1550 s; last phase started: door.script
Reading the project graph slice (933 s): flush 1 of 1.
Reading the project graph slice (935 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 201 and delivered 0 within 901 s, 201 are still missing, among them the too heavy project(s) workspace (about 13780 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
~~~

Named-heavy MB values in the exact graph/refusal text are estimates from the native log, not measured RSS.

Observed phase wall, approximately five-second polling; exact internal wall not measured:

| Phase | Start elapsed s | Wall until next phase/controller end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 0.002 | 5.092 |
| [semio] Installing the native repository command… | 5.094 | 10.102 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 15.196 | 15.175 |
| [semio] workspace:setup… | 30.371 | 1555.821 |

The final try interval includes the signal-grace minute. Connectivity at completion:
~~~json
{
  "at": "2026-11-14T23:43:01Z",
  "ports": {
    "2049": {
      "reachable": true
    },
    "6094": {
      "reachable": true
    },
    "8981": {
      "reachable": true
    }
  }
}
~~~

EPERM / exact where / consent / caller-fallback lines, verbatim:
~~~text
None emitted in the native and observed service streams.
~~~

Actual host-incarnation parent chains; no job environment blocks were copied:
~~~json
[
  {
    "metadata_file": "host-9888631afea4.json",
    "at": "2026-11-14T23:16:35Z",
    "pid": 5696,
    "startedAt": 1794697377441,
    "implementation": "4ca26156abdfae8368837ffed431400958fcc38f22faf6b8b44b9488c36f8fe7",
    "parent_chain": [
      {
        "pid": 5696,
        "ppid": 5684,
        "rss_kib": 435856,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \\012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('node:module');\\012const hash=value=>crypto.createHash('sha256').update(value).digest('hex'),check=()=>{const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;if(wire){const deadline=JSON.parse(wire).deadlineEpochMilliseconds;if(deadline!==null&&(!Number.isSafeInteger(deadline)||Date.now()>=deadline))throw Error('Original immutable implementation deadline exhausted');}};\\012const closureRefusalTextV1=(function closureRefusalTextV1(table, code, fields, environment) {\\012  const row = table[code];\\012  if (!row)\\012    return code + \": \" + JSON.stringify(fields);\\012  const tag = environment.SEMIO_LANGUAGE, languages = tag && row[tag] ? [tag] : Object.keys(row), fill = (text) => text.replace(/\\{(\\w+)\\}/gu, (_all, key) => String(fields[key] ?? \"?\"));\\012  return languages.map((language) => fill(row[language].refusal) + \". \" + fill(row[language].fix)).join(\" | \");\\012}),closureRefusalCatalogTableV1=(function closureRefusalCatalogTableV1(encoded, sha256, hash) {\\012  try {\\012    const bytes = Buffer.from(encoded ?? \"\", \"base64\");\\012    if (!encoded || bytes.toString(\"base64\") !== encoded || hash(bytes) !== sha256)\\012      return;\\012    const table = JSON.parse(bytes.toString(\"utf8\"))?.$defs?.Messages?.const;\\012    return table && typeof table === \"object\" ? table : void 0;\\012  } catch {\\012    return;\\012  }\\012}),refusalCatalog={table:undefined,read:()=>undefined},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalCatalog.read()??{},code,fields,process.env));\\012const evaluation=process.execArgv.reduce((indices,arg,index)=>{if(arg==='--eval'||arg==='-e'||arg.startsWith('--eval='))indices.push(index);return indices;},[]);if(evaluation.length!==1||process.execArgv[evaluation[0]]!=='--eval'||typeof process.execArgv[evaluation[0]+1]!=='string'||!process.execArgv[evaluation[0]+1])throw Error('Actual single evaluated immutable loader required');\\012const evaluatedLoader=process.execArgv[evaluation[0]+1],snapshotPath=null,loaderIdentity=(()=>{let known;return()=>known??=hash(evaluatedLoader);})(),[expected,...args]=process.argv.slice(process.argv[1]==='--'?2:1);check();if(!/^[a-f0-9]{64}$/.test(expected??''))throw refusal('digest-required',{});\\012let encoded;const snapshotFromEnv=process.env.SEMIO_GRAPH_WORKER_SNAPSHOT;if(process.env.SEMIO_GRAPH_WORKER==='1'&&(typeof snapshotFromEnv!=='string'||!snapshotFromEnv.length))throw Error('Graph worker snapshot path required');if(typeof snapshotFromEnv==='string'&&snapshotFromEnv.length){const opened=fs.openSync(snapshotFromEnv,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW??0));try{const fileFacts=fs.fstatSync(opened);if(!fileFacts.isFile()||fileFacts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(fileFacts.size);let filled=0;while(filled<encoded.length){check();const count=fs.readSync(opened,encoded,filled,encoded.length-filled,null);if(!count)throw refusal('snapshot-changed',{});filled+=count;}}finally{fs.closeSync(opened);}}else{const facts=fs.fstatSync(3),chunks=[],chunk=Buffer.alloc(65536);let size=0,used=0;if(facts.isFile()){if(facts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(facts.size);while(used<encoded.length){check();const count=fs.readSync(3,encoded,used,encoded.length-used,null);if(!count)throw refusal('snapshot-changed',{});used+=count;}if(fs.readSync(3,Buffer.alloc(1),0,1,null))throw refusal('snapshot-bounded',{});}else{for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,33554432-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>33554432-size)throw refusal('snapshot-bounded',{});size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}encoded=Buffer.concat(chunks,size);}}check();if(hash(encoded)!==expected)throw refusal('snapshot-changed',{});\\012const redirectCapturedBuiltinReadsV1=(function redirectCapturedBuiltinReadsV1(source, commonJs, parser, targets, refuse) {\\012  const names = [...targets.keys()].join(\"|\");\\012  if (!new RegExp(`[\"'\\\\x60](?:` + names + `)[\"'\\\\x60]`).test(source))\\012    return source;\\012  const expected = parser.scan(source).imports.filter((row) => targets.has(row.path)).length;\\012  let count = 0;\\012  const quick = source.replace(new RegExp(`(\\\\b(?:from|import))(\\\\s*)([\"'])(` + names + \")\\\\3|(\\\\b(?:import\" + (commonJs ? \"\" : \"|require|__require\") + `))(\\\\s*\\\\(\\\\s*)([\"'])(` + names + \")\\\\7\", \"g\"), (_match, a, b, _quote, name, c, d, _second, other) => {\\012    count++;\\012    return name !== void 0 ? a + b + JSON.stringify(targets.get(name)) : c + d + JSON.stringify(targets.get(other));\\012  });\\012  if (count === expected && !parser.scan(quick).imports.some((row) => targets.has(row.path)))\\012    return count ? quick : source;\\012  const changes = [], length = source.length;\\012  let previous = \"\", prior = \"\", at = 0;\\012  while (at < length) {\\012    const code = source.charCodeAt(at);\\012    if (code === 32 || code > 8 && code < 14 || code > 127 && /\\s/.test(source[at])) {\\012      at++;\\012      continue;\\012    }\\012    if (code === 47) {\\012      const next = source.charCodeAt(at + 1);\\012      if (next === 47) {\\012        at = source.indexOf(`\\012`, at + 2);\\012        if (at < 0)\\012          break;\\012        continue;\\012      }\\012      if (next === 42) {\\012        const end = source.indexOf(\"*/\", at + 2);\\012        if (end < 0)\\012          throw refuse(\"compiler-comment-unclosed\", {});\\012        at = end + 2;\\012        continue;\\012      }\\012    }\\012    if (code === 96) {\\012      at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === 96)\\012          break;\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code === 34 || code === 39) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === code)\\012          break;\\012      }\\012      if (at - start < 20) {\\012        const value = source.slice(start, at).slice(1, -1);\\012        if (targets.has(value) && (previous === \"from\" || previous === \"import\" || previous === 40 && (prior === \"import\" || !commonJs && (prior === \"require\" || prior === \"__require\"))))\\012          changes.push({ start, end: at, value: JSON.stringify(targets.get(value)) });\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code > 64 && code < 91 || code > 96 && code < 123 || code === 95 || code === 36) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (!(c > 64 && c < 91 || c > 96 && c < 123 || c > 47 && c < 58 || c === 95 || c === 36))\\012          break;\\012        at++;\\012      }\\012      const size = at - start;\\012      prior = previous;\\012      previous = size === 4 || size === 6 || size === 7 || size === 9 ? source.slice(start, at) : -2;\\012      continue;\\012    }\\012    prior = previous;\\012    previous = code;\\012    at++;\\012  }\\012  if (changes.length !== expected)\\012    throw refuse(\"builtin-read-coverage-refused\", {});\\012  if (!changes.length)\\012    return source;\\012  let result = source;\\012  for (const change of changes.reverse())\\012    result = result.slice(0, change.start) + change.value + result.slice(change.end);\\012  if (parser.scan(result).imports.some((row) => targets.has(row.path)))\\012    throw refuse(\"builtin-read-remained\", {});\\012  return result;\\012}),splitCapturedSnapshotV1=(function splitCapturedSnapshotV1(encoded) {\\012  const pieces = [], spans = [], open = Buffer.from(',\"base64\":\"'), next = Buffer.from('}},{\"path\":\"');\\012  let from = 0, at = 0;\\012  for (;; ) {\\012    const key = encoded.indexOf(open, at);\\012    if (key < 0)\\012      break;\\012    const start = key + open.length, end = encoded.indexOf(34, start);\\012    if (end < 0 || encoded.toString(\"latin1\", end, end + 12) !== '\",\"imports\":')\\012      return;\\012    pieces.push(encoded.toString(\"utf8\", from, start));\\012    spans.push([start, end]);\\012    from = end;\\012    const boundary = encoded.indexOf(next, end);\\012    if (boundary < 0)\\012      break;\\012    at = boundary;\\012  }\\012  if (!spans.length)\\012    return;\\012  pieces.push(encoded.toString(\"utf8\", from));\\012  return { text: pieces.join(\"\"), spans };\\012}),admitCapturedImagesV1=(function admitCapturedImagesV1(value, encoded, spans, limits, hash, path, check, refuse) {\\012  const resolutions = new Map, rows = new Map, decoded = new Map, eager = new Map;\\012  let total = 0;\\012  const decode = (name, index) => {\\012    const row = value.files[index], [start, end] = spans[index], text = encoded.toString(\"latin1\", start, end), bytes = Buffer.from(text, \"base64\");\\012    if (bytes.toString(\"base64\") !== text || hash(bytes) !== row.sha256)\\012      throw refuse(\"implementation-image-changed\", {});\\012    decoded.set(name, bytes);\\012    return bytes;\\012  };\\012  for (let index = 0;index < value.files.length; index++) {\\012    const row = value.files[index];\\012    check();\\012    if (!row || Object.keys(row).length !== 4 || !Object.keys(row).every((key) => key === \"base64\" || key === \"imports\" || key === \"path\" || key === \"sha256\") || typeof row.path !== \"string\" || path.isAbsolute(row.path) || row.path.includes(\":\") || row.path.includes(String.fromCharCode(92)) || row.path.includes(String.fromCharCode(0)) || row.path.split(\"/\").some((part) => !part || part === \".\" || part === \"..\") || typeof row.base64 !== \"string\" || !/^[a-f0-9]{64}$/.test(row.sha256 ?? \"\"))\\012      throw refuse(\"implementation-image-required\", {});\\012    const name = path.join(value.root, row.path);\\012    if (spans) {\\012      const [start, end] = spans[index], length = end - start, size = length % 4 ? -1 : length / 4 * 3 - (encoded[end - 1] === 61 ? 1 : 0) - (encoded[end - 2] === 61 ? 1 : 0);\\012      total += size;\\012      if (size < 0 || size > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || rows.has(name))\\012        throw refuse(\"implementation-image-changed\", {});\\012      rows.set(name, index);\\012    } else {\\012      const bytes = Buffer.from(row.base64, \"base64\");\\012      total += bytes.length;\\012      if (bytes.toString(\"base64\") !== row.base64 || bytes.length > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || eager.has(name) || hash(bytes) !== row.sha256)\\012        throw refuse(\"implementation-image-changed\", {});\\012      eager.set(name, bytes);\\012    }\\012    if (!row.imports || typeof row.imports !== \"object\" || Array.isArray(row.imports))\\012      throw refuse(\"import-resolution-required\", {});\\012    resolutions.set(name, row.imports);\\012    if (!spans)\\012      row.base64 = \"\";\\012  }\\012  if (!spans)\\012    return { images: eager, resolutions };\\012  return { images: { has: (name) => rows.has(name), get: (name) => {\\012    const bytes = decoded.get(name);\\012    if (bytes)\\012      return bytes;\\012    const index = rows.get(name);\\012    return index === void 0 ? void 0 : decode(name, index);\\012  }, keys: () => rows.keys(), [Symbol.iterator]: () => decoded.entries() }, resolutions };\\012});\\012const split=splitCapturedSnapshotV1(encoded),value=JSON.parse(split?split.text:encoded.toString('utf8')),spans=split&&Array.isArray(value?.files)&&split.spans.length===value.files.length?split.spans:undefined;if(!value||Object.keys(value).sort().join(',')!=='entry,files,root,version'||value.version!==1||typeof value.root!=='string'||!path.isAbsolute(value.root)||value.root.length>256||typeof value.entry!=='string'||!Array.isArray(value.files)||!value.files.length||value.files.length>2048)throw Error('Closed immutable implementation snapshot required');const catalogAt=value.files.findIndex(row=>row&&row.path===\"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json\"),catalogEager=catalogAt<0||spans?undefined:value.files[catalogAt].base64;refusalCatalog.read=()=>refusalCatalog.table??=catalogAt<0?undefined:closureRefusalCatalogTableV1(spans?encoded.toString('latin1',spans[catalogAt][0],spans[catalogAt][1]):catalogEager,value.files[catalogAt].sha256,hash);const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,{maximumFileBytes:4194304,maximumSourceBytes:20971520},hash,path,check,refusal);for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map)){if(typeof target==='string'&&/^installed:(?:@[a-z0-9._-]+[/])?[a-z0-9._-]+@[^\\s]{1,64}$/.test(target)&&specifier)continue;if(typeof target!=='string'||!specifier||path.isAbsolute(target)||target.split('/').some(part=>!part||part==='.'||part==='..')||target.includes(String.fromCharCode(92))||!images.has(path.join(value.root,target)))throw Error('Closed captured resolution target required');}const entry=path.join(value.root,value.entry);if(!images.has(entry))throw Error('Captured implementation entry required');\\012const capturedSourceImportDiagnosticV1=(function capturedSourceImportDiagnosticV1(kind, specifier, importer, declaringImage, declared, targetImage) {\\012  const clip = (value) => {\\012    if (value == null)\\012      return null;\\012    let text = \"\", count = 0;\\012    for (const character of value) {\\012      text += character;\\012      if (++count === 256)\\012        break;\\012    }\\012    return text;\\012  };\\012  return Object.freeze({ version: 1, purpose: \"captured-source-import-refusal\", kind, importer: clip(importer), specifier: clip(specifier), declaringImage, declared, targetImage });\\012}),rootScheme=\"semio-root:\",installedScheme=\"installed:\",sourceImportRefusal=(kind,specifier,importer,declaringImage,declared,targetImage)=>{const diagnostic=capturedSourceImportDiagnosticV1(kind,specifier,importer&&path.isAbsolute(importer)?path.relative(value.root,importer).split(path.sep).join('/'):importer,declaringImage,declared,targetImage),error=refusal(kind,{specifier:diagnostic.specifier,importer:diagnostic.importer});error.message+=' '+JSON.stringify(diagnostic);return error;};\\012const sourceTraceKeyV1=Symbol.for('semio.sourceTrace.v1'),sourceTraceSeenV1=new Set(),sourceTraceBufferV1=[],sourceTraceSinkV1={file:undefined},sourceTraceRelativeV1=name=>{const relative=path.relative(value.root,name);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)?relative.split(path.sep).join('/'):name;},sourceTraceAppendV1=fs.appendFileSync;\\012globalThis[sourceTraceKeyV1]=Object.freeze({record:(kind,name)=>{const text=kind+' '+(kind==='a'?name:sourceTraceRelativeV1(name));if(sourceTraceSeenV1.has(text))return;sourceTraceSeenV1.add(text);if(sourceTraceSinkV1.file===undefined){if(sourceTraceBufferV1.length<65536)sourceTraceBufferV1.push(text);return;}if(sourceTraceSinkV1.file!==null)try{sourceTraceAppendV1(sourceTraceSinkV1.file,text+'\\n');}catch{}},enable:directory=>{if(sourceTraceSinkV1.file!==undefined)return;try{fs.mkdirSync(directory,{recursive:true});const file=path.join(directory,process.pid+'-'+Date.now()+'.trace');fs.writeFileSync(file,'s '+JSON.stringify({argv:process.argv.slice(2),entry:value.entry,files:value.files.length,sha256:expected})+'\\n'+sourceTraceBufferV1.map(line=>line+'\\n').join(''));sourceTraceBufferV1.length=0;sourceTraceSinkV1.file=file;}catch{sourceTraceSinkV1.file=null;}}});\\012const sourceTraceV1=globalThis[sourceTraceKeyV1];\\012const originalRead=fs.readFileSync,codeDirectories=new Set([...images.keys()].filter(name=>/\\.[cm]?[jt]sx?$/.test(name)).map(name=>path.dirname(name))),capturedSchemaOwner=file=>{if(path.extname(file)!=='.json'||path.basename(file)==='package.json')return false;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return false;const parts=relative.split(path.sep),at=parts.indexOf('\\u{1f9ec}\\uFE0Fschema');return at>=0&&codeDirectories.has(path.join(value.root,...parts.slice(0,at)));},currentImage=(name,expected)=>{check();if(images.get(name)!==expected)throw Error('Captured implementation Source image required: '+name);};\\012const capturedRead=(name,options)=>{check();let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));const bytes=file&&images.get(file);if(bytes){currentImage(file,bytes);sourceTraceV1.record('r',file);const encoding=typeof options==='string'?options:options?.encoding;return encoding?bytes.toString(encoding):Buffer.from(bytes);}if(file&&capturedSchemaOwner(file))throw refusal('schema-read',{specifier:path.relative(value.root,file).split(path.sep).join('/')});if(file)sourceTraceV1.record('x',file);return originalRead(name,options);};\\012const promises=await import('node:fs/promises'),capturedPromises={...promises,readFile:async(name,options)=>{let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));if(file&&(images.has(file)||capturedSchemaOwner(file)))return capturedRead(name,options);if(file)sourceTraceV1.record('x',file);return promises.readFile(name,options);}},capturedFs={...fs,readFileSync:capturedRead,readFile:(name,options,callback)=>{const cb=typeof options==='function'?options:callback;capturedPromises.readFile(name,typeof options==='function'?undefined:options).then(bytes=>cb(null,bytes),error=>cb(error));},promises:capturedPromises};\\012const nativeRequire=require,commonJs=new Map(),installedTarget=target=>typeof target==='string'&&target.startsWith(installedScheme),declaredTarget=(importer,specifier)=>{const map=resolutions.get(importer);if(!map)return undefined;if(Object.hasOwn(map,specifier))return map[specifier];const file=specifier.startsWith('file:')?url.fileURLToPath(specifier):specifier;if(!path.isAbsolute(file))return undefined;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return undefined;const key=relative.split(path.sep).join('/');return Object.hasOwn(map,rootScheme+key)?map[rootScheme+key]:images.has(path.resolve(file))?key:undefined;},installedChecked=new Map(),resolving={depth:0},physicalResolve=(requireFrom,specifier)=>{resolving.depth++;try{return requireFrom.resolve(specifier);}finally{resolving.depth--;}},installedResolve=(requireFrom,specifier,target)=>{const resolved=physicalResolve(requireFrom,specifier);if(installedChecked.get(specifier)!==target){const marker=resolved.lastIndexOf(path.sep+'node_modules'+path.sep),pieces=marker<0?[]:resolved.slice(marker+path.sep.length+13).split(path.sep),segments=target.slice(installedScheme.length).replace(/@[^@]*$/,'').split('/'),directory=marker<0?'':resolved.slice(0,marker+path.sep.length+13)+pieces.slice(0,segments.length).join(path.sep);let version;try{version=JSON.parse(originalRead(path.join(directory,'package.json'),'utf8')).version;}catch{version=undefined;}if(!directory||installedScheme+segments.join('/')+'@'+version!==target)throw Object.assign(refusal('installed-drift',{specifier,detail:target.slice(installedScheme.length)+' -> '+(version??'unreadable')}),{code:'SEMIO_INSTALLED_DRIFT'});installedChecked.set(specifier,target);}return resolved;},capturedCreateRequire=base=>{const name=base instanceof URL||typeof base==='string'&&base.startsWith('file:')?url.fileURLToPath(base):String(base);const actual=awaitModule.createRequire(base),resolve=(specifier,dataOnly)=>{check();if(typeof specifier!=='string')throw Error('Closed captured require specifier required');specifier=({'./.semio-captured-fs':'node:fs','./.semio-captured-fs-promises':'node:fs/promises','./.semio-captured-module':'node:module'})[specifier]??specifier;if(awaitModule.isBuiltin(specifier))return specifier;if(!images.has(path.resolve(name)))return physicalResolve(actual,specifier);const target=declaredTarget(path.resolve(name),specifier);if(!target&&dataOnly)return physicalResolve(actual,specifier);if(!target)throw sourceImportRefusal('require',specifier,name,true,false,false);if(installedTarget(target))return installedResolve(actual,specifier,target);return path.join(value.root,target);},load=specifier=>{if(!images.has(path.resolve(name)))throw sourceImportRefusal('declaring-module',undefined,name,false,false,false);if(installedTarget(declaredTarget(path.resolve(name),specifier))){installedResolve(actual,specifier,declaredTarget(path.resolve(name),specifier));return actual(specifier);}const target=resolve(specifier);if(awaitModule.isBuiltin(target)){if(target==='fs'||target==='node:fs')return capturedFs;if(target==='fs/promises'||target==='node:fs/promises')return capturedPromises;if(target==='module'||target==='node:module')return capturedModule;return actual(target);}if(target.endsWith('.cjs')&&commonJs.has(target))return commonJs.get(target).exports;const loaded=nativeRequire('semio-source:'+target);return target.endsWith('.cjs')?commonJs.get(target)?.exports:target.endsWith('.json')?loaded.default:loaded;};load.resolve=specifier=>resolve(specifier,true);return load;},moduleDefault=awaitModule.default??awaitModule,capturedModule={...awaitModule,__beginCapturedCommonJsV1:name=>{if(!images.has(name)||!name.endsWith('.cjs')||commonJs.has(name))throw Error('Unique captured CommonJS module required');const owner=new(awaitModule.Module??awaitModule.default)(name);owner.filename=name;owner.paths=[];owner.require=capturedCreateRequire(name);commonJs.set(name,owner);return owner;},__failCapturedCommonJsV1:name=>commonJs.delete(name),createRequire:capturedCreateRequire,default:new Proxy(moduleDefault,{get:(target,key)=>key==='createRequire'?capturedCreateRequire:Reflect.get(target,key)})};\\012const redirectParser=new Bun.Transpiler({loader:'js',target:'bun'}),redirectTargets=new Map([['node:fs','./.semio-captured-fs'],['fs','./.semio-captured-fs'],['node:fs/promises','./.semio-captured-fs-promises'],['fs/promises','./.semio-captured-fs-promises'],['node:module','./.semio-captured-module'],['module','./.semio-captured-module']]),redirect=(source,commonJs=false)=>redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets,refusal);\\012const orderedSourceFiles=[...value.files].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),freezeSelection=item=>{if(item&&typeof item==='object'&&!Object.isFrozen(item)){Object.freeze(item);for(const member of Object.values(item))freezeSelection(member);}return item;};Object.defineProperty(globalThis,'__semioCapturedSelectionV1',{value:freezeSelection({physical:{version:1,root:value.root,sources:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256}))},resolutions:{version:1,root:value.root,files:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256,imports:row.imports}))}}),enumerable:false,writable:false,configurable:false});\\012Bun.plugin({name:'closed-immutable-implementation',setup(build){build.onResolve({filter:/.*/,namespace:'semio-source'},item=>{const name=item.path.replace(/^semio-source:/,'');if(!images.has(name))throw Error('Uncaptured require namespace image refused');return{path:name,namespace:'semio-source'};});build.onResolve({filter:/^[.][/][.]semio-captured-module$/},()=>({path:'captured-module',namespace:'semio-immutable'}));build.onLoad({filter:/^captured-module$/,namespace:'semio-immutable'},()=>({loader:'object',exports:capturedModule}));build.onResolve({filter:/^[.][/][.]semio-captured-fs(?:-promises)?$/},item=>({path:item.path.endsWith('-promises')?'captured-fs-promises':'captured-fs',namespace:'semio-immutable'}));build.onResolve({filter:/.*/},item=>{if(item.path.startsWith('semio-capture/')||item.importer?.startsWith('semio-capture:'))return globalThis.__semioPlanResolveV1?.(item);if(resolving.depth||item.path.startsWith('node:')||item.path.startsWith('bun:')||(awaitModule.isBuiltin(item.path)))return;const importer=item.importer?.replace(/^[/]?semio-source:/,''),declared=importer&&declaredTarget(importer,item.path);if(declared&&installedTarget(declared)){installedResolve(awaitModule.createRequire(importer),item.path,declared);return;}if(importer&&!images.has(importer)&&importer.split(path.sep).includes('node_modules'))return;const candidate=declared?path.join(value.root,declared):item.path.startsWith('file:')?url.fileURLToPath(item.path):path.isAbsolute(item.path)?item.path:undefined;if(!candidate||!images.has(candidate)||!declared&&candidate!==entry)throw sourceImportRefusal('import',item.path,importer,Boolean(importer&&images.has(importer)),Boolean(declared),Boolean(candidate&&images.has(candidate)));return{path:candidate,namespace:'semio-source'};});build.onLoad({filter:/^captured-fs$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedFs,default:capturedFs}}));build.onLoad({filter:/^captured-fs-promises$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedPromises,default:capturedPromises}}));build.onLoad({filter:/.*/,namespace:'semio-capture'},item=>globalThis.__semioPlanLoadV1(item));build.onLoad({filter:/.*/,namespace:'semio-source'},item=>{check();const name=path.resolve(item.path),bytes=images.get(name);if(!bytes)throw Error('Uncaptured implementation module refused: '+name);sourceTraceV1.record('m',name);if(path.extname(name)==='.json'){currentImage(name,bytes);return{loader:'object',exports:{default:JSON.parse(bytes.toString('utf8'))}};}if(name.endsWith('.cjs')){const owner=JSON.stringify(name),directory=JSON.stringify(path.dirname(name));return{contents:\"import {__beginCapturedCommonJsV1 as begin,__failCapturedCommonJsV1 as fail} from './.semio-captured-module'; const owner=begin(\"+owner+\");try{(function(module,exports,require,__filename,__dirname){\\n\"+redirect(bytes.toString('utf8').replace(/^#![^\\r\\n]*/,''),true)+\"\\n})(owner,owner.exports,owner.require,\"+owner+\",\"+directory+\");owner.loaded=true;}catch(error){fail(\"+owner+\");throw error;}export default owner.exports;\",loader:'js'};}const loader=/\\.[cm]?tsx?$/.test(name)?'ts':'js',compiler=new Bun.Transpiler({loader,target:'bun',define:{'import.meta.semioCapturedPhysicalSources':'globalThis.__semioCapturedSelectionV1.physical','import.meta.semioCapturedSourceResolutions':'globalThis.__semioCapturedSelectionV1.resolutions','import.meta.semioCapturedImplementationIdentity':JSON.stringify(expected),'import.meta.semioCapturedLoaderIdentity':JSON.stringify(loaderIdentity()??''),'import.meta.url':JSON.stringify(url.pathToFileURL(name).href),'import.meta.path':JSON.stringify(name),'import.meta.filename':JSON.stringify(name),'import.meta.dir':JSON.stringify(path.dirname(name)),'import.meta.dirname':JSON.stringify(path.dirname(name)),'import.meta.main':'false'}}),transformed=compiler.transformSync(bytes.toString('utf8'));check();return{contents:redirect(transformed),loader:'js'};});}});\\012for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map))if(installedTarget(target)&&installedChecked.get(specifier)!==target)try{installedResolve(awaitModule.createRequire(name),specifier,target);}catch(error){if(error?.code==='SEMIO_INSTALLED_DRIFT')throw error;}\\012 const module=await import(entry);check();if(typeof module.runCapturedBuildSourceV1!=='function')throw Error('Closed captured implementation entry required');await module.runCapturedBuildSourceV1(args,{version:1,snapshot:snapshotPath,sha256:expected,loaderSha256:loaderIdentity(),root:value.root,recheck:()=>{for(const[name,bytes]of images)currentImage(name,bytes);check();},text:()=>encoded.toString('utf8')});check();\\012 -- 1c36e3d99a39e2fd8d15c2ba58da9de4f0dab3e0be01fc0b90b78033aec0d3fd service host run"
      },
      {
        "pid": 5684,
        "ppid": 1,
        "rss_kib": 380672,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo service host run"
      },
      {
        "pid": 1,
        "ppid": 0,
        "rss_kib": 19472,
        "args": "/sbin/launchd"
      }
    ],
    "started_by": "coordinator native bootstrap; reused if started during prior try/round"
  },
  {
    "metadata_file": "host-ac6bfe8f337c.json",
    "at": "2026-11-14T23:17:10Z",
    "pid": 14244,
    "startedAt": 1794698229863,
    "implementation": "28e87fbf8be568a33deb5279c63bba36ecaaaf5f280e6e180d23885eed938d00",
    "parent_chain": [
      {
        "pid": 14244,
        "ppid": 14214,
        "rss_kib": 441776,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \\012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('node:module');\\012const hash=value=>crypto.createHash('sha256').update(value).digest('hex'),check=()=>{const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;if(wire){const deadline=JSON.parse(wire).deadlineEpochMilliseconds;if(deadline!==null&&(!Number.isSafeInteger(deadline)||Date.now()>=deadline))throw Error('Original immutable implementation deadline exhausted');}};\\012const closureRefusalTextV1=(function closureRefusalTextV1(table, code, fields, environment) {\\012  const row = table[code];\\012  if (!row)\\012    return code + \": \" + JSON.stringify(fields);\\012  const tag = environment.SEMIO_LANGUAGE, languages = tag && row[tag] ? [tag] : Object.keys(row), fill = (text) => text.replace(/\\{(\\w+)\\}/gu, (_all, key) => String(fields[key] ?? \"?\"));\\012  return languages.map((language) => fill(row[language].refusal) + \". \" + fill(row[language].fix)).join(\" | \");\\012}),closureRefusalCatalogTableV1=(function closureRefusalCatalogTableV1(encoded, sha256, hash) {\\012  try {\\012    const bytes = Buffer.from(encoded ?? \"\", \"base64\");\\012    if (!encoded || bytes.toString(\"base64\") !== encoded || hash(bytes) !== sha256)\\012      return;\\012    const table = JSON.parse(bytes.toString(\"utf8\"))?.$defs?.Messages?.const;\\012    return table && typeof table === \"object\" ? table : void 0;\\012  } catch {\\012    return;\\012  }\\012}),refusalCatalog={table:undefined,read:()=>undefined},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalCatalog.read()??{},code,fields,process.env));\\012const evaluation=process.execArgv.reduce((indices,arg,index)=>{if(arg==='--eval'||arg==='-e'||arg.startsWith('--eval='))indices.push(index);return indices;},[]);if(evaluation.length!==1||process.execArgv[evaluation[0]]!=='--eval'||typeof process.execArgv[evaluation[0]+1]!=='string'||!process.execArgv[evaluation[0]+1])throw Error('Actual single evaluated immutable loader required');\\012const evaluatedLoader=process.execArgv[evaluation[0]+1],snapshotPath=null,loaderIdentity=(()=>{let known;return()=>known??=hash(evaluatedLoader);})(),[expected,...args]=process.argv.slice(process.argv[1]==='--'?2:1);check();if(!/^[a-f0-9]{64}$/.test(expected??''))throw refusal('digest-required',{});\\012let encoded;const snapshotFromEnv=process.env.SEMIO_GRAPH_WORKER_SNAPSHOT;if(process.env.SEMIO_GRAPH_WORKER==='1'&&(typeof snapshotFromEnv!=='string'||!snapshotFromEnv.length))throw Error('Graph worker snapshot path required');if(typeof snapshotFromEnv==='string'&&snapshotFromEnv.length){const opened=fs.openSync(snapshotFromEnv,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW??0));try{const fileFacts=fs.fstatSync(opened);if(!fileFacts.isFile()||fileFacts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(fileFacts.size);let filled=0;while(filled<encoded.length){check();const count=fs.readSync(opened,encoded,filled,encoded.length-filled,null);if(!count)throw refusal('snapshot-changed',{});filled+=count;}}finally{fs.closeSync(opened);}}else{const facts=fs.fstatSync(3),chunks=[],chunk=Buffer.alloc(65536);let size=0,used=0;if(facts.isFile()){if(facts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(facts.size);while(used<encoded.length){check();const count=fs.readSync(3,encoded,used,encoded.length-used,null);if(!count)throw refusal('snapshot-changed',{});used+=count;}if(fs.readSync(3,Buffer.alloc(1),0,1,null))throw refusal('snapshot-bounded',{});}else{for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,33554432-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>33554432-size)throw refusal('snapshot-bounded',{});size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}encoded=Buffer.concat(chunks,size);}}check();if(hash(encoded)!==expected)throw refusal('snapshot-changed',{});\\012const redirectCapturedBuiltinReadsV1=(function redirectCapturedBuiltinReadsV1(source, commonJs, parser, targets, refuse) {\\012  const names = [...targets.keys()].join(\"|\");\\012  if (!new RegExp(`[\"'\\\\x60](?:` + names + `)[\"'\\\\x60]`).test(source))\\012    return source;\\012  const expected = parser.scan(source).imports.filter((row) => targets.has(row.path)).length;\\012  let count = 0;\\012  const quick = source.replace(new RegExp(`(\\\\b(?:from|import))(\\\\s*)([\"'])(` + names + \")\\\\3|(\\\\b(?:import\" + (commonJs ? \"\" : \"|require|__require\") + `))(\\\\s*\\\\(\\\\s*)([\"'])(` + names + \")\\\\7\", \"g\"), (_match, a, b, _quote, name, c, d, _second, other) => {\\012    count++;\\012    return name !== void 0 ? a + b + JSON.stringify(targets.get(name)) : c + d + JSON.stringify(targets.get(other));\\012  });\\012  if (count === expected && !parser.scan(quick).imports.some((row) => targets.has(row.path)))\\012    return count ? quick : source;\\012  const changes = [], length = source.length;\\012  let previous = \"\", prior = \"\", at = 0;\\012  while (at < length) {\\012    const code = source.charCodeAt(at);\\012    if (code === 32 || code > 8 && code < 14 || code > 127 && /\\s/.test(source[at])) {\\012      at++;\\012      continue;\\012    }\\012    if (code === 47) {\\012      const next = source.charCodeAt(at + 1);\\012      if (next === 47) {\\012        at = source.indexOf(`\\012`, at + 2);\\012        if (at < 0)\\012          break;\\012        continue;\\012      }\\012      if (next === 42) {\\012        const end = source.indexOf(\"*/\", at + 2);\\012        if (end < 0)\\012          throw refuse(\"compiler-comment-unclosed\", {});\\012        at = end + 2;\\012        continue;\\012      }\\012    }\\012    if (code === 96) {\\012      at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === 96)\\012          break;\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code === 34 || code === 39) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === code)\\012          break;\\012      }\\012      if (at - start < 20) {\\012        const value = source.slice(start, at).slice(1, -1);\\012        if (targets.has(value) && (previous === \"from\" || previous === \"import\" || previous === 40 && (prior === \"import\" || !commonJs && (prior === \"require\" || prior === \"__require\"))))\\012          changes.push({ start, end: at, value: JSON.stringify(targets.get(value)) });\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code > 64 && code < 91 || code > 96 && code < 123 || code === 95 || code === 36) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (!(c > 64 && c < 91 || c > 96 && c < 123 || c > 47 && c < 58 || c === 95 || c === 36))\\012          break;\\012        at++;\\012      }\\012      const size = at - start;\\012      prior = previous;\\012      previous = size === 4 || size === 6 || size === 7 || size === 9 ? source.slice(start, at) : -2;\\012      continue;\\012    }\\012    prior = previous;\\012    previous = code;\\012    at++;\\012  }\\012  if (changes.length !== expected)\\012    throw refuse(\"builtin-read-coverage-refused\", {});\\012  if (!changes.length)\\012    return source;\\012  let result = source;\\012  for (const change of changes.reverse())\\012    result = result.slice(0, change.start) + change.value + result.slice(change.end);\\012  if (parser.scan(result).imports.some((row) => targets.has(row.path)))\\012    throw refuse(\"builtin-read-remained\", {});\\012  return result;\\012}),splitCapturedSnapshotV1=(function splitCapturedSnapshotV1(encoded) {\\012  const pieces = [], spans = [], open = Buffer.from(',\"base64\":\"'), next = Buffer.from('}},{\"path\":\"');\\012  let from = 0, at = 0;\\012  for (;; ) {\\012    const key = encoded.indexOf(open, at);\\012    if (key < 0)\\012      break;\\012    const start = key + open.length, end = encoded.indexOf(34, start);\\012    if (end < 0 || encoded.toString(\"latin1\", end, end + 12) !== '\",\"imports\":')\\012      return;\\012    pieces.push(encoded.toString(\"utf8\", from, start));\\012    spans.push([start, end]);\\012    from = end;\\012    const boundary = encoded.indexOf(next, end);\\012    if (boundary < 0)\\012      break;\\012    at = boundary;\\012  }\\012  if (!spans.length)\\012    return;\\012  pieces.push(encoded.toString(\"utf8\", from));\\012  return { text: pieces.join(\"\"), spans };\\012}),admitCapturedImagesV1=(function admitCapturedImagesV1(value, encoded, spans, limits, hash, path, check, refuse) {\\012  const resolutions = new Map, rows = new Map, decoded = new Map, eager = new Map;\\012  let total = 0;\\012  const decode = (name, index) => {\\012    const row = value.files[index], [start, end] = spans[index], text = encoded.toString(\"latin1\", start, end), bytes = Buffer.from(text, \"base64\");\\012    if (bytes.toString(\"base64\") !== text || hash(bytes) !== row.sha256)\\012      throw refuse(\"implementation-image-changed\", {});\\012    decoded.set(name, bytes);\\012    return bytes;\\012  };\\012  for (let index = 0;index < value.files.length; index++) {\\012    const row = value.files[index];\\012    check();\\012    if (!row || Object.keys(row).length !== 4 || !Object.keys(row).every((key) => key === \"base64\" || key === \"imports\" || key === \"path\" || key === \"sha256\") || typeof row.path !== \"string\" || path.isAbsolute(row.path) || row.path.includes(\":\") || row.path.includes(String.fromCharCode(92)) || row.path.includes(String.fromCharCode(0)) || row.path.split(\"/\").some((part) => !part || part === \".\" || part === \"..\") || typeof row.base64 !== \"string\" || !/^[a-f0-9]{64}$/.test(row.sha256 ?? \"\"))\\012      throw refuse(\"implementation-image-required\", {});\\012    const name = path.join(value.root, row.path);\\012    if (spans) {\\012      const [start, end] = spans[index], length = end - start, size = length % 4 ? -1 : length / 4 * 3 - (encoded[end - 1] === 61 ? 1 : 0) - (encoded[end - 2] === 61 ? 1 : 0);\\012      total += size;\\012      if (size < 0 || size > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || rows.has(name))\\012        throw refuse(\"implementation-image-changed\", {});\\012      rows.set(name, index);\\012    } else {\\012      const bytes = Buffer.from(row.base64, \"base64\");\\012      total += bytes.length;\\012      if (bytes.toString(\"base64\") !== row.base64 || bytes.length > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || eager.has(name) || hash(bytes) !== row.sha256)\\012        throw refuse(\"implementation-image-changed\", {});\\012      eager.set(name, bytes);\\012    }\\012    if (!row.imports || typeof row.imports !== \"object\" || Array.isArray(row.imports))\\012      throw refuse(\"import-resolution-required\", {});\\012    resolutions.set(name, row.imports);\\012    if (!spans)\\012      row.base64 = \"\";\\012  }\\012  if (!spans)\\012    return { images: eager, resolutions };\\012  return { images: { has: (name) => rows.has(name), get: (name) => {\\012    const bytes = decoded.get(name);\\012    if (bytes)\\012      return bytes;\\012    const index = rows.get(name);\\012    return index === void 0 ? void 0 : decode(name, index);\\012  }, keys: () => rows.keys(), [Symbol.iterator]: () => decoded.entries() }, resolutions };\\012});\\012const split=splitCapturedSnapshotV1(encoded),value=JSON.parse(split?split.text:encoded.toString('utf8')),spans=split&&Array.isArray(value?.files)&&split.spans.length===value.files.length?split.spans:undefined;if(!value||Object.keys(value).sort().join(',')!=='entry,files,root,version'||value.version!==1||typeof value.root!=='string'||!path.isAbsolute(value.root)||value.root.length>256||typeof value.entry!=='string'||!Array.isArray(value.files)||!value.files.length||value.files.length>2048)throw Error('Closed immutable implementation snapshot required');const catalogAt=value.files.findIndex(row=>row&&row.path===\"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json\"),catalogEager=catalogAt<0||spans?undefined:value.files[catalogAt].base64;refusalCatalog.read=()=>refusalCatalog.table??=catalogAt<0?undefined:closureRefusalCatalogTableV1(spans?encoded.toString('latin1',spans[catalogAt][0],spans[catalogAt][1]):catalogEager,value.files[catalogAt].sha256,hash);const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,{maximumFileBytes:4194304,maximumSourceBytes:20971520},hash,path,check,refusal);for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map)){if(typeof target==='string'&&/^installed:(?:@[a-z0-9._-]+[/])?[a-z0-9._-]+@[^\\s]{1,64}$/.test(target)&&specifier)continue;if(typeof target!=='string'||!specifier||path.isAbsolute(target)||target.split('/').some(part=>!part||part==='.'||part==='..')||target.includes(String.fromCharCode(92))||!images.has(path.join(value.root,target)))throw Error('Closed captured resolution target required');}const entry=path.join(value.root,value.entry);if(!images.has(entry))throw Error('Captured implementation entry required');\\012const capturedSourceImportDiagnosticV1=(function capturedSourceImportDiagnosticV1(kind, specifier, importer, declaringImage, declared, targetImage) {\\012  const clip = (value) => {\\012    if (value == null)\\012      return null;\\012    let text = \"\", count = 0;\\012    for (const character of value) {\\012      text += character;\\012      if (++count === 256)\\012        break;\\012    }\\012    return text;\\012  };\\012  return Object.freeze({ version: 1, purpose: \"captured-source-import-refusal\", kind, importer: clip(importer), specifier: clip(specifier), declaringImage, declared, targetImage });\\012}),rootScheme=\"semio-root:\",installedScheme=\"installed:\",sourceImportRefusal=(kind,specifier,importer,declaringImage,declared,targetImage)=>{const diagnostic=capturedSourceImportDiagnosticV1(kind,specifier,importer&&path.isAbsolute(importer)?path.relative(value.root,importer).split(path.sep).join('/'):importer,declaringImage,declared,targetImage),error=refusal(kind,{specifier:diagnostic.specifier,importer:diagnostic.importer});error.message+=' '+JSON.stringify(diagnostic);return error;};\\012const sourceTraceKeyV1=Symbol.for('semio.sourceTrace.v1'),sourceTraceSeenV1=new Set(),sourceTraceBufferV1=[],sourceTraceSinkV1={file:undefined},sourceTraceRelativeV1=name=>{const relative=path.relative(value.root,name);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)?relative.split(path.sep).join('/'):name;},sourceTraceAppendV1=fs.appendFileSync;\\012globalThis[sourceTraceKeyV1]=Object.freeze({record:(kind,name)=>{const text=kind+' '+(kind==='a'?name:sourceTraceRelativeV1(name));if(sourceTraceSeenV1.has(text))return;sourceTraceSeenV1.add(text);if(sourceTraceSinkV1.file===undefined){if(sourceTraceBufferV1.length<65536)sourceTraceBufferV1.push(text);return;}if(sourceTraceSinkV1.file!==null)try{sourceTraceAppendV1(sourceTraceSinkV1.file,text+'\\n');}catch{}},enable:directory=>{if(sourceTraceSinkV1.file!==undefined)return;try{fs.mkdirSync(directory,{recursive:true});const file=path.join(directory,process.pid+'-'+Date.now()+'.trace');fs.writeFileSync(file,'s '+JSON.stringify({argv:process.argv.slice(2),entry:value.entry,files:value.files.length,sha256:expected})+'\\n'+sourceTraceBufferV1.map(line=>line+'\\n').join(''));sourceTraceBufferV1.length=0;sourceTraceSinkV1.file=file;}catch{sourceTraceSinkV1.file=null;}}});\\012const sourceTraceV1=globalThis[sourceTraceKeyV1];\\012const originalRead=fs.readFileSync,codeDirectories=new Set([...images.keys()].filter(name=>/\\.[cm]?[jt]sx?$/.test(name)).map(name=>path.dirname(name))),capturedSchemaOwner=file=>{if(path.extname(file)!=='.json'||path.basename(file)==='package.json')return false;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return false;const parts=relative.split(path.sep),at=parts.indexOf('\\u{1f9ec}\\uFE0Fschema');return at>=0&&codeDirectories.has(path.join(value.root,...parts.slice(0,at)));},currentImage=(name,expected)=>{check();if(images.get(name)!==expected)throw Error('Captured implementation Source image required: '+name);};\\012const capturedRead=(name,options)=>{check();let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));const bytes=file&&images.get(file);if(bytes){currentImage(file,bytes);sourceTraceV1.record('r',file);const encoding=typeof options==='string'?options:options?.encoding;return encoding?bytes.toString(encoding):Buffer.from(bytes);}if(file&&capturedSchemaOwner(file))throw refusal('schema-read',{specifier:path.relative(value.root,file).split(path.sep).join('/')});if(file)sourceTraceV1.record('x',file);return originalRead(name,options);};\\012const promises=await import('node:fs/promises'),capturedPromises={...promises,readFile:async(name,options)=>{let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));if(file&&(images.has(file)||capturedSchemaOwner(file)))return capturedRead(name,options);if(file)sourceTraceV1.record('x',file);return promises.readFile(name,options);}},capturedFs={...fs,readFileSync:capturedRead,readFile:(name,options,callback)=>{const cb=typeof options==='function'?options:callback;capturedPromises.readFile(name,typeof options==='function'?undefined:options).then(bytes=>cb(null,bytes),error=>cb(error));},promises:capturedPromises};\\012const nativeRequire=require,commonJs=new Map(),installedTarget=target=>typeof target==='string'&&target.startsWith(installedScheme),declaredTarget=(importer,specifier)=>{const map=resolutions.get(importer);if(!map)return undefined;if(Object.hasOwn(map,specifier))return map[specifier];const file=specifier.startsWith('file:')?url.fileURLToPath(specifier):specifier;if(!path.isAbsolute(file))return undefined;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return undefined;const key=relative.split(path.sep).join('/');return Object.hasOwn(map,rootScheme+key)?map[rootScheme+key]:images.has(path.resolve(file))?key:undefined;},installedChecked=new Map(),resolving={depth:0},physicalResolve=(requireFrom,specifier)=>{resolving.depth++;try{return requireFrom.resolve(specifier);}finally{resolving.depth--;}},installedResolve=(requireFrom,specifier,target)=>{const resolved=physicalResolve(requireFrom,specifier);if(installedChecked.get(specifier)!==target){const marker=resolved.lastIndexOf(path.sep+'node_modules'+path.sep),pieces=marker<0?[]:resolved.slice(marker+path.sep.length+13).split(path.sep),segments=target.slice(installedScheme.length).replace(/@[^@]*$/,'').split('/'),directory=marker<0?'':resolved.slice(0,marker+path.sep.length+13)+pieces.slice(0,segments.length).join(path.sep);let version;try{version=JSON.parse(originalRead(path.join(directory,'package.json'),'utf8')).version;}catch{version=undefined;}if(!directory||installedScheme+segments.join('/')+'@'+version!==target)throw Object.assign(refusal('installed-drift',{specifier,detail:target.slice(installedScheme.length)+' -> '+(version??'unreadable')}),{code:'SEMIO_INSTALLED_DRIFT'});installedChecked.set(specifier,target);}return resolved;},capturedCreateRequire=base=>{const name=base instanceof URL||typeof base==='string'&&base.startsWith('file:')?url.fileURLToPath(base):String(base);const actual=awaitModule.createRequire(base),resolve=(specifier,dataOnly)=>{check();if(typeof specifier!=='string')throw Error('Closed captured require specifier required');specifier=({'./.semio-captured-fs':'node:fs','./.semio-captured-fs-promises':'node:fs/promises','./.semio-captured-module':'node:module'})[specifier]??specifier;if(awaitModule.isBuiltin(specifier))return specifier;if(!images.has(path.resolve(name)))return physicalResolve(actual,specifier);const target=declaredTarget(path.resolve(name),specifier);if(!target&&dataOnly)return physicalResolve(actual,specifier);if(!target)throw sourceImportRefusal('require',specifier,name,true,false,false);if(installedTarget(target))return installedResolve(actual,specifier,target);return path.join(value.root,target);},load=specifier=>{if(!images.has(path.resolve(name)))throw sourceImportRefusal('declaring-module',undefined,name,false,false,false);if(installedTarget(declaredTarget(path.resolve(name),specifier))){installedResolve(actual,specifier,declaredTarget(path.resolve(name),specifier));return actual(specifier);}const target=resolve(specifier);if(awaitModule.isBuiltin(target)){if(target==='fs'||target==='node:fs')return capturedFs;if(target==='fs/promises'||target==='node:fs/promises')return capturedPromises;if(target==='module'||target==='node:module')return capturedModule;return actual(target);}if(target.endsWith('.cjs')&&commonJs.has(target))return commonJs.get(target).exports;const loaded=nativeRequire('semio-source:'+target);return target.endsWith('.cjs')?commonJs.get(target)?.exports:target.endsWith('.json')?loaded.default:loaded;};load.resolve=specifier=>resolve(specifier,true);return load;},moduleDefault=awaitModule.default??awaitModule,capturedModule={...awaitModule,__beginCapturedCommonJsV1:name=>{if(!images.has(name)||!name.endsWith('.cjs')||commonJs.has(name))throw Error('Unique captured CommonJS module required');const owner=new(awaitModule.Module??awaitModule.default)(name);owner.filename=name;owner.paths=[];owner.require=capturedCreateRequire(name);commonJs.set(name,owner);return owner;},__failCapturedCommonJsV1:name=>commonJs.delete(name),createRequire:capturedCreateRequire,default:new Proxy(moduleDefault,{get:(target,key)=>key==='createRequire'?capturedCreateRequire:Reflect.get(target,key)})};\\012const redirectParser=new Bun.Transpiler({loader:'js',target:'bun'}),redirectTargets=new Map([['node:fs','./.semio-captured-fs'],['fs','./.semio-captured-fs'],['node:fs/promises','./.semio-captured-fs-promises'],['fs/promises','./.semio-captured-fs-promises'],['node:module','./.semio-captured-module'],['module','./.semio-captured-module']]),redirect=(source,commonJs=false)=>redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets,refusal);\\012const orderedSourceFiles=[...value.files].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),freezeSelection=item=>{if(item&&typeof item==='object'&&!Object.isFrozen(item)){Object.freeze(item);for(const member of Object.values(item))freezeSelection(member);}return item;};Object.defineProperty(globalThis,'__semioCapturedSelectionV1',{value:freezeSelection({physical:{version:1,root:value.root,sources:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256}))},resolutions:{version:1,root:value.root,files:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256,imports:row.imports}))}}),enumerable:false,writable:false,configurable:false});\\012Bun.plugin({name:'closed-immutable-implementation',setup(build){build.onResolve({filter:/.*/,namespace:'semio-source'},item=>{const name=item.path.replace(/^semio-source:/,'');if(!images.has(name))throw Error('Uncaptured require namespace image refused');return{path:name,namespace:'semio-source'};});build.onResolve({filter:/^[.][/][.]semio-captured-module$/},()=>({path:'captured-module',namespace:'semio-immutable'}));build.onLoad({filter:/^captured-module$/,namespace:'semio-immutable'},()=>({loader:'object',exports:capturedModule}));build.onResolve({filter:/^[.][/][.]semio-captured-fs(?:-promises)?$/},item=>({path:item.path.endsWith('-promises')?'captured-fs-promises':'captured-fs',namespace:'semio-immutable'}));build.onResolve({filter:/.*/},item=>{if(item.path.startsWith('semio-capture/')||item.importer?.startsWith('semio-capture:'))return globalThis.__semioPlanResolveV1?.(item);if(resolving.depth||item.path.startsWith('node:')||item.path.startsWith('bun:')||(awaitModule.isBuiltin(item.path)))return;const importer=item.importer?.replace(/^[/]?semio-source:/,''),declared=importer&&declaredTarget(importer,item.path);if(declared&&installedTarget(declared)){installedResolve(awaitModule.createRequire(importer),item.path,declared);return;}if(importer&&!images.has(importer)&&importer.split(path.sep).includes('node_modules'))return;const candidate=declared?path.join(value.root,declared):item.path.startsWith('file:')?url.fileURLToPath(item.path):path.isAbsolute(item.path)?item.path:undefined;if(!candidate||!images.has(candidate)||!declared&&candidate!==entry)throw sourceImportRefusal('import',item.path,importer,Boolean(importer&&images.has(importer)),Boolean(declared),Boolean(candidate&&images.has(candidate)));return{path:candidate,namespace:'semio-source'};});build.onLoad({filter:/^captured-fs$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedFs,default:capturedFs}}));build.onLoad({filter:/^captured-fs-promises$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedPromises,default:capturedPromises}}));build.onLoad({filter:/.*/,namespace:'semio-capture'},item=>globalThis.__semioPlanLoadV1(item));build.onLoad({filter:/.*/,namespace:'semio-source'},item=>{check();const name=path.resolve(item.path),bytes=images.get(name);if(!bytes)throw Error('Uncaptured implementation module refused: '+name);sourceTraceV1.record('m',name);if(path.extname(name)==='.json'){currentImage(name,bytes);return{loader:'object',exports:{default:JSON.parse(bytes.toString('utf8'))}};}if(name.endsWith('.cjs')){const owner=JSON.stringify(name),directory=JSON.stringify(path.dirname(name));return{contents:\"import {__beginCapturedCommonJsV1 as begin,__failCapturedCommonJsV1 as fail} from './.semio-captured-module'; const owner=begin(\"+owner+\");try{(function(module,exports,require,__filename,__dirname){\\n\"+redirect(bytes.toString('utf8').replace(/^#![^\\r\\n]*/,''),true)+\"\\n})(owner,owner.exports,owner.require,\"+owner+\",\"+directory+\");owner.loaded=true;}catch(error){fail(\"+owner+\");throw error;}export default owner.exports;\",loader:'js'};}const loader=/\\.[cm]?tsx?$/.test(name)?'ts':'js',compiler=new Bun.Transpiler({loader,target:'bun',define:{'import.meta.semioCapturedPhysicalSources':'globalThis.__semioCapturedSelectionV1.physical','import.meta.semioCapturedSourceResolutions':'globalThis.__semioCapturedSelectionV1.resolutions','import.meta.semioCapturedImplementationIdentity':JSON.stringify(expected),'import.meta.semioCapturedLoaderIdentity':JSON.stringify(loaderIdentity()??''),'import.meta.url':JSON.stringify(url.pathToFileURL(name).href),'import.meta.path':JSON.stringify(name),'import.meta.filename':JSON.stringify(name),'import.meta.dir':JSON.stringify(path.dirname(name)),'import.meta.dirname':JSON.stringify(path.dirname(name)),'import.meta.main':'false'}}),transformed=compiler.transformSync(bytes.toString('utf8'));check();return{contents:redirect(transformed),loader:'js'};});}});\\012for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map))if(installedTarget(target)&&installedChecked.get(specifier)!==target)try{installedResolve(awaitModule.createRequire(name),specifier,target);}catch(error){if(error?.code==='SEMIO_INSTALLED_DRIFT')throw error;}\\012 const module=await import(entry);check();if(typeof module.runCapturedBuildSourceV1!=='function')throw Error('Closed captured implementation entry required');await module.runCapturedBuildSourceV1(args,{version:1,snapshot:snapshotPath,sha256:expected,loaderSha256:loaderIdentity(),root:value.root,recheck:()=>{for(const[name,bytes]of images)currentImage(name,bytes);check();},text:()=>encoded.toString('utf8')});check();\\012 -- 6048bbcd56e51272f0863b7e2d75405ed836796991af5b10b2fe949722d3c958 service host run"
      },
      {
        "pid": 14214,
        "ppid": 1,
        "rss_kib": 528080,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo service host run"
      },
      {
        "pid": 1,
        "ppid": 0,
        "rss_kib": 19568,
        "args": "/sbin/launchd"
      }
    ],
    "started_by": "coordinator native bootstrap; reused if started during prior try/round"
  },
  {
    "metadata_file": "host-ac6bfe8f337c.json",
    "at": "2026-11-14T23:27:24Z",
    "pid": 21242,
    "startedAt": 1794698842200,
    "implementation": "28e87fbf8be568a33deb5279c63bba36ecaaaf5f280e6e180d23885eed938d00",
    "parent_chain": [
      {
        "pid": 21242,
        "ppid": 21227,
        "rss_kib": 427872,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \\012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('node:module');\\012const hash=value=>crypto.createHash('sha256').update(value).digest('hex'),check=()=>{const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;if(wire){const deadline=JSON.parse(wire).deadlineEpochMilliseconds;if(deadline!==null&&(!Number.isSafeInteger(deadline)||Date.now()>=deadline))throw Error('Original immutable implementation deadline exhausted');}};\\012const closureRefusalTextV1=(function closureRefusalTextV1(table, code, fields, environment) {\\012  const row = table[code];\\012  if (!row)\\012    return code + \": \" + JSON.stringify(fields);\\012  const tag = environment.SEMIO_LANGUAGE, languages = tag && row[tag] ? [tag] : Object.keys(row), fill = (text) => text.replace(/\\{(\\w+)\\}/gu, (_all, key) => String(fields[key] ?? \"?\"));\\012  return languages.map((language) => fill(row[language].refusal) + \". \" + fill(row[language].fix)).join(\" | \");\\012}),closureRefusalCatalogTableV1=(function closureRefusalCatalogTableV1(encoded, sha256, hash) {\\012  try {\\012    const bytes = Buffer.from(encoded ?? \"\", \"base64\");\\012    if (!encoded || bytes.toString(\"base64\") !== encoded || hash(bytes) !== sha256)\\012      return;\\012    const table = JSON.parse(bytes.toString(\"utf8\"))?.$defs?.Messages?.const;\\012    return table && typeof table === \"object\" ? table : void 0;\\012  } catch {\\012    return;\\012  }\\012}),refusalCatalog={table:undefined,read:()=>undefined},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalCatalog.read()??{},code,fields,process.env));\\012const evaluation=process.execArgv.reduce((indices,arg,index)=>{if(arg==='--eval'||arg==='-e'||arg.startsWith('--eval='))indices.push(index);return indices;},[]);if(evaluation.length!==1||process.execArgv[evaluation[0]]!=='--eval'||typeof process.execArgv[evaluation[0]+1]!=='string'||!process.execArgv[evaluation[0]+1])throw Error('Actual single evaluated immutable loader required');\\012const evaluatedLoader=process.execArgv[evaluation[0]+1],snapshotPath=null,loaderIdentity=(()=>{let known;return()=>known??=hash(evaluatedLoader);})(),[expected,...args]=process.argv.slice(process.argv[1]==='--'?2:1);check();if(!/^[a-f0-9]{64}$/.test(expected??''))throw refusal('digest-required',{});\\012let encoded;const snapshotFromEnv=process.env.SEMIO_GRAPH_WORKER_SNAPSHOT;if(process.env.SEMIO_GRAPH_WORKER==='1'&&(typeof snapshotFromEnv!=='string'||!snapshotFromEnv.length))throw Error('Graph worker snapshot path required');if(typeof snapshotFromEnv==='string'&&snapshotFromEnv.length){const opened=fs.openSync(snapshotFromEnv,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW??0));try{const fileFacts=fs.fstatSync(opened);if(!fileFacts.isFile()||fileFacts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(fileFacts.size);let filled=0;while(filled<encoded.length){check();const count=fs.readSync(opened,encoded,filled,encoded.length-filled,null);if(!count)throw refusal('snapshot-changed',{});filled+=count;}}finally{fs.closeSync(opened);}}else{const facts=fs.fstatSync(3),chunks=[],chunk=Buffer.alloc(65536);let size=0,used=0;if(facts.isFile()){if(facts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(facts.size);while(used<encoded.length){check();const count=fs.readSync(3,encoded,used,encoded.length-used,null);if(!count)throw refusal('snapshot-changed',{});used+=count;}if(fs.readSync(3,Buffer.alloc(1),0,1,null))throw refusal('snapshot-bounded',{});}else{for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,33554432-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>33554432-size)throw refusal('snapshot-bounded',{});size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}encoded=Buffer.concat(chunks,size);}}check();if(hash(encoded)!==expected)throw refusal('snapshot-changed',{});\\012const redirectCapturedBuiltinReadsV1=(function redirectCapturedBuiltinReadsV1(source, commonJs, parser, targets, refuse) {\\012  const names = [...targets.keys()].join(\"|\");\\012  if (!new RegExp(`[\"'\\\\x60](?:` + names + `)[\"'\\\\x60]`).test(source))\\012    return source;\\012  const expected = parser.scan(source).imports.filter((row) => targets.has(row.path)).length;\\012  let count = 0;\\012  const quick = source.replace(new RegExp(`(\\\\b(?:from|import))(\\\\s*)([\"'])(` + names + \")\\\\3|(\\\\b(?:import\" + (commonJs ? \"\" : \"|require|__require\") + `))(\\\\s*\\\\(\\\\s*)([\"'])(` + names + \")\\\\7\", \"g\"), (_match, a, b, _quote, name, c, d, _second, other) => {\\012    count++;\\012    return name !== void 0 ? a + b + JSON.stringify(targets.get(name)) : c + d + JSON.stringify(targets.get(other));\\012  });\\012  if (count === expected && !parser.scan(quick).imports.some((row) => targets.has(row.path)))\\012    return count ? quick : source;\\012  const changes = [], length = source.length;\\012  let previous = \"\", prior = \"\", at = 0;\\012  while (at < length) {\\012    const code = source.charCodeAt(at);\\012    if (code === 32 || code > 8 && code < 14 || code > 127 && /\\s/.test(source[at])) {\\012      at++;\\012      continue;\\012    }\\012    if (code === 47) {\\012      const next = source.charCodeAt(at + 1);\\012      if (next === 47) {\\012        at = source.indexOf(`\\012`, at + 2);\\012        if (at < 0)\\012          break;\\012        continue;\\012      }\\012      if (next === 42) {\\012        const end = source.indexOf(\"*/\", at + 2);\\012        if (end < 0)\\012          throw refuse(\"compiler-comment-unclosed\", {});\\012        at = end + 2;\\012        continue;\\012      }\\012    }\\012    if (code === 96) {\\012      at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === 96)\\012          break;\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code === 34 || code === 39) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === code)\\012          break;\\012      }\\012      if (at - start < 20) {\\012        const value = source.slice(start, at).slice(1, -1);\\012        if (targets.has(value) && (previous === \"from\" || previous === \"import\" || previous === 40 && (prior === \"import\" || !commonJs && (prior === \"require\" || prior === \"__require\"))))\\012          changes.push({ start, end: at, value: JSON.stringify(targets.get(value)) });\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code > 64 && code < 91 || code > 96 && code < 123 || code === 95 || code === 36) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (!(c > 64 && c < 91 || c > 96 && c < 123 || c > 47 && c < 58 || c === 95 || c === 36))\\012          break;\\012        at++;\\012      }\\012      const size = at - start;\\012      prior = previous;\\012      previous = size === 4 || size === 6 || size === 7 || size === 9 ? source.slice(start, at) : -2;\\012      continue;\\012    }\\012    prior = previous;\\012    previous = code;\\012    at++;\\012  }\\012  if (changes.length !== expected)\\012    throw refuse(\"builtin-read-coverage-refused\", {});\\012  if (!changes.length)\\012    return source;\\012  let result = source;\\012  for (const change of changes.reverse())\\012    result = result.slice(0, change.start) + change.value + result.slice(change.end);\\012  if (parser.scan(result).imports.some((row) => targets.has(row.path)))\\012    throw refuse(\"builtin-read-remained\", {});\\012  return result;\\012}),splitCapturedSnapshotV1=(function splitCapturedSnapshotV1(encoded) {\\012  const pieces = [], spans = [], open = Buffer.from(',\"base64\":\"'), next = Buffer.from('}},{\"path\":\"');\\012  let from = 0, at = 0;\\012  for (;; ) {\\012    const key = encoded.indexOf(open, at);\\012    if (key < 0)\\012      break;\\012    const start = key + open.length, end = encoded.indexOf(34, start);\\012    if (end < 0 || encoded.toString(\"latin1\", end, end + 12) !== '\",\"imports\":')\\012      return;\\012    pieces.push(encoded.toString(\"utf8\", from, start));\\012    spans.push([start, end]);\\012    from = end;\\012    const boundary = encoded.indexOf(next, end);\\012    if (boundary < 0)\\012      break;\\012    at = boundary;\\012  }\\012  if (!spans.length)\\012    return;\\012  pieces.push(encoded.toString(\"utf8\", from));\\012  return { text: pieces.join(\"\"), spans };\\012}),admitCapturedImagesV1=(function admitCapturedImagesV1(value, encoded, spans, limits, hash, path, check, refuse) {\\012  const resolutions = new Map, rows = new Map, decoded = new Map, eager = new Map;\\012  let total = 0;\\012  const decode = (name, index) => {\\012    const row = value.files[index], [start, end] = spans[index], text = encoded.toString(\"latin1\", start, end), bytes = Buffer.from(text, \"base64\");\\012    if (bytes.toString(\"base64\") !== text || hash(bytes) !== row.sha256)\\012      throw refuse(\"implementation-image-changed\", {});\\012    decoded.set(name, bytes);\\012    return bytes;\\012  };\\012  for (let index = 0;index < value.files.length; index++) {\\012    const row = value.files[index];\\012    check();\\012    if (!row || Object.keys(row).length !== 4 || !Object.keys(row).every((key) => key === \"base64\" || key === \"imports\" || key === \"path\" || key === \"sha256\") || typeof row.path !== \"string\" || path.isAbsolute(row.path) || row.path.includes(\":\") || row.path.includes(String.fromCharCode(92)) || row.path.includes(String.fromCharCode(0)) || row.path.split(\"/\").some((part) => !part || part === \".\" || part === \"..\") || typeof row.base64 !== \"string\" || !/^[a-f0-9]{64}$/.test(row.sha256 ?? \"\"))\\012      throw refuse(\"implementation-image-required\", {});\\012    const name = path.join(value.root, row.path);\\012    if (spans) {\\012      const [start, end] = spans[index], length = end - start, size = length % 4 ? -1 : length / 4 * 3 - (encoded[end - 1] === 61 ? 1 : 0) - (encoded[end - 2] === 61 ? 1 : 0);\\012      total += size;\\012      if (size < 0 || size > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || rows.has(name))\\012        throw refuse(\"implementation-image-changed\", {});\\012      rows.set(name, index);\\012    } else {\\012      const bytes = Buffer.from(row.base64, \"base64\");\\012      total += bytes.length;\\012      if (bytes.toString(\"base64\") !== row.base64 || bytes.length > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || eager.has(name) || hash(bytes) !== row.sha256)\\012        throw refuse(\"implementation-image-changed\", {});\\012      eager.set(name, bytes);\\012    }\\012    if (!row.imports || typeof row.imports !== \"object\" || Array.isArray(row.imports))\\012      throw refuse(\"import-resolution-required\", {});\\012    resolutions.set(name, row.imports);\\012    if (!spans)\\012      row.base64 = \"\";\\012  }\\012  if (!spans)\\012    return { images: eager, resolutions };\\012  return { images: { has: (name) => rows.has(name), get: (name) => {\\012    const bytes = decoded.get(name);\\012    if (bytes)\\012      return bytes;\\012    const index = rows.get(name);\\012    return index === void 0 ? void 0 : decode(name, index);\\012  }, keys: () => rows.keys(), [Symbol.iterator]: () => decoded.entries() }, resolutions };\\012});\\012const split=splitCapturedSnapshotV1(encoded),value=JSON.parse(split?split.text:encoded.toString('utf8')),spans=split&&Array.isArray(value?.files)&&split.spans.length===value.files.length?split.spans:undefined;if(!value||Object.keys(value).sort().join(',')!=='entry,files,root,version'||value.version!==1||typeof value.root!=='string'||!path.isAbsolute(value.root)||value.root.length>256||typeof value.entry!=='string'||!Array.isArray(value.files)||!value.files.length||value.files.length>2048)throw Error('Closed immutable implementation snapshot required');const catalogAt=value.files.findIndex(row=>row&&row.path===\"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json\"),catalogEager=catalogAt<0||spans?undefined:value.files[catalogAt].base64;refusalCatalog.read=()=>refusalCatalog.table??=catalogAt<0?undefined:closureRefusalCatalogTableV1(spans?encoded.toString('latin1',spans[catalogAt][0],spans[catalogAt][1]):catalogEager,value.files[catalogAt].sha256,hash);const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,{maximumFileBytes:4194304,maximumSourceBytes:20971520},hash,path,check,refusal);for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map)){if(typeof target==='string'&&/^installed:(?:@[a-z0-9._-]+[/])?[a-z0-9._-]+@[^\\s]{1,64}$/.test(target)&&specifier)continue;if(typeof target!=='string'||!specifier||path.isAbsolute(target)||target.split('/').some(part=>!part||part==='.'||part==='..')||target.includes(String.fromCharCode(92))||!images.has(path.join(value.root,target)))throw Error('Closed captured resolution target required');}const entry=path.join(value.root,value.entry);if(!images.has(entry))throw Error('Captured implementation entry required');\\012const capturedSourceImportDiagnosticV1=(function capturedSourceImportDiagnosticV1(kind, specifier, importer, declaringImage, declared, targetImage) {\\012  const clip = (value) => {\\012    if (value == null)\\012      return null;\\012    let text = \"\", count = 0;\\012    for (const character of value) {\\012      text += character;\\012      if (++count === 256)\\012        break;\\012    }\\012    return text;\\012  };\\012  return Object.freeze({ version: 1, purpose: \"captured-source-import-refusal\", kind, importer: clip(importer), specifier: clip(specifier), declaringImage, declared, targetImage });\\012}),rootScheme=\"semio-root:\",installedScheme=\"installed:\",sourceImportRefusal=(kind,specifier,importer,declaringImage,declared,targetImage)=>{const diagnostic=capturedSourceImportDiagnosticV1(kind,specifier,importer&&path.isAbsolute(importer)?path.relative(value.root,importer).split(path.sep).join('/'):importer,declaringImage,declared,targetImage),error=refusal(kind,{specifier:diagnostic.specifier,importer:diagnostic.importer});error.message+=' '+JSON.stringify(diagnostic);return error;};\\012const sourceTraceKeyV1=Symbol.for('semio.sourceTrace.v1'),sourceTraceSeenV1=new Set(),sourceTraceBufferV1=[],sourceTraceSinkV1={file:undefined},sourceTraceRelativeV1=name=>{const relative=path.relative(value.root,name);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)?relative.split(path.sep).join('/'):name;},sourceTraceAppendV1=fs.appendFileSync;\\012globalThis[sourceTraceKeyV1]=Object.freeze({record:(kind,name)=>{const text=kind+' '+(kind==='a'?name:sourceTraceRelativeV1(name));if(sourceTraceSeenV1.has(text))return;sourceTraceSeenV1.add(text);if(sourceTraceSinkV1.file===undefined){if(sourceTraceBufferV1.length<65536)sourceTraceBufferV1.push(text);return;}if(sourceTraceSinkV1.file!==null)try{sourceTraceAppendV1(sourceTraceSinkV1.file,text+'\\n');}catch{}},enable:directory=>{if(sourceTraceSinkV1.file!==undefined)return;try{fs.mkdirSync(directory,{recursive:true});const file=path.join(directory,process.pid+'-'+Date.now()+'.trace');fs.writeFileSync(file,'s '+JSON.stringify({argv:process.argv.slice(2),entry:value.entry,files:value.files.length,sha256:expected})+'\\n'+sourceTraceBufferV1.map(line=>line+'\\n').join(''));sourceTraceBufferV1.length=0;sourceTraceSinkV1.file=file;}catch{sourceTraceSinkV1.file=null;}}});\\012const sourceTraceV1=globalThis[sourceTraceKeyV1];\\012const originalRead=fs.readFileSync,codeDirectories=new Set([...images.keys()].filter(name=>/\\.[cm]?[jt]sx?$/.test(name)).map(name=>path.dirname(name))),capturedSchemaOwner=file=>{if(path.extname(file)!=='.json'||path.basename(file)==='package.json')return false;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return false;const parts=relative.split(path.sep),at=parts.indexOf('\\u{1f9ec}\\uFE0Fschema');return at>=0&&codeDirectories.has(path.join(value.root,...parts.slice(0,at)));},currentImage=(name,expected)=>{check();if(images.get(name)!==expected)throw Error('Captured implementation Source image required: '+name);};\\012const capturedRead=(name,options)=>{check();let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));const bytes=file&&images.get(file);if(bytes){currentImage(file,bytes);sourceTraceV1.record('r',file);const encoding=typeof options==='string'?options:options?.encoding;return encoding?bytes.toString(encoding):Buffer.from(bytes);}if(file&&capturedSchemaOwner(file))throw refusal('schema-read',{specifier:path.relative(value.root,file).split(path.sep).join('/')});if(file)sourceTraceV1.record('x',file);return originalRead(name,options);};\\012const promises=await import('node:fs/promises'),capturedPromises={...promises,readFile:async(name,options)=>{let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));if(file&&(images.has(file)||capturedSchemaOwner(file)))return capturedRead(name,options);if(file)sourceTraceV1.record('x',file);return promises.readFile(name,options);}},capturedFs={...fs,readFileSync:capturedRead,readFile:(name,options,callback)=>{const cb=typeof options==='function'?options:callback;capturedPromises.readFile(name,typeof options==='function'?undefined:options).then(bytes=>cb(null,bytes),error=>cb(error));},promises:capturedPromises};\\012const nativeRequire=require,commonJs=new Map(),installedTarget=target=>typeof target==='string'&&target.startsWith(installedScheme),declaredTarget=(importer,specifier)=>{const map=resolutions.get(importer);if(!map)return undefined;if(Object.hasOwn(map,specifier))return map[specifier];const file=specifier.startsWith('file:')?url.fileURLToPath(specifier):specifier;if(!path.isAbsolute(file))return undefined;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return undefined;const key=relative.split(path.sep).join('/');return Object.hasOwn(map,rootScheme+key)?map[rootScheme+key]:images.has(path.resolve(file))?key:undefined;},installedChecked=new Map(),resolving={depth:0},physicalResolve=(requireFrom,specifier)=>{resolving.depth++;try{return requireFrom.resolve(specifier);}finally{resolving.depth--;}},installedResolve=(requireFrom,specifier,target)=>{const resolved=physicalResolve(requireFrom,specifier);if(installedChecked.get(specifier)!==target){const marker=resolved.lastIndexOf(path.sep+'node_modules'+path.sep),pieces=marker<0?[]:resolved.slice(marker+path.sep.length+13).split(path.sep),segments=target.slice(installedScheme.length).replace(/@[^@]*$/,'').split('/'),directory=marker<0?'':resolved.slice(0,marker+path.sep.length+13)+pieces.slice(0,segments.length).join(path.sep);let version;try{version=JSON.parse(originalRead(path.join(directory,'package.json'),'utf8')).version;}catch{version=undefined;}if(!directory||installedScheme+segments.join('/')+'@'+version!==target)throw Object.assign(refusal('installed-drift',{specifier,detail:target.slice(installedScheme.length)+' -> '+(version??'unreadable')}),{code:'SEMIO_INSTALLED_DRIFT'});installedChecked.set(specifier,target);}return resolved;},capturedCreateRequire=base=>{const name=base instanceof URL||typeof base==='string'&&base.startsWith('file:')?url.fileURLToPath(base):String(base);const actual=awaitModule.createRequire(base),resolve=(specifier,dataOnly)=>{check();if(typeof specifier!=='string')throw Error('Closed captured require specifier required');specifier=({'./.semio-captured-fs':'node:fs','./.semio-captured-fs-promises':'node:fs/promises','./.semio-captured-module':'node:module'})[specifier]??specifier;if(awaitModule.isBuiltin(specifier))return specifier;if(!images.has(path.resolve(name)))return physicalResolve(actual,specifier);const target=declaredTarget(path.resolve(name),specifier);if(!target&&dataOnly)return physicalResolve(actual,specifier);if(!target)throw sourceImportRefusal('require',specifier,name,true,false,false);if(installedTarget(target))return installedResolve(actual,specifier,target);return path.join(value.root,target);},load=specifier=>{if(!images.has(path.resolve(name)))throw sourceImportRefusal('declaring-module',undefined,name,false,false,false);if(installedTarget(declaredTarget(path.resolve(name),specifier))){installedResolve(actual,specifier,declaredTarget(path.resolve(name),specifier));return actual(specifier);}const target=resolve(specifier);if(awaitModule.isBuiltin(target)){if(target==='fs'||target==='node:fs')return capturedFs;if(target==='fs/promises'||target==='node:fs/promises')return capturedPromises;if(target==='module'||target==='node:module')return capturedModule;return actual(target);}if(target.endsWith('.cjs')&&commonJs.has(target))return commonJs.get(target).exports;const loaded=nativeRequire('semio-source:'+target);return target.endsWith('.cjs')?commonJs.get(target)?.exports:target.endsWith('.json')?loaded.default:loaded;};load.resolve=specifier=>resolve(specifier,true);return load;},moduleDefault=awaitModule.default??awaitModule,capturedModule={...awaitModule,__beginCapturedCommonJsV1:name=>{if(!images.has(name)||!name.endsWith('.cjs')||commonJs.has(name))throw Error('Unique captured CommonJS module required');const owner=new(awaitModule.Module??awaitModule.default)(name);owner.filename=name;owner.paths=[];owner.require=capturedCreateRequire(name);commonJs.set(name,owner);return owner;},__failCapturedCommonJsV1:name=>commonJs.delete(name),createRequire:capturedCreateRequire,default:new Proxy(moduleDefault,{get:(target,key)=>key==='createRequire'?capturedCreateRequire:Reflect.get(target,key)})};\\012const redirectParser=new Bun.Transpiler({loader:'js',target:'bun'}),redirectTargets=new Map([['node:fs','./.semio-captured-fs'],['fs','./.semio-captured-fs'],['node:fs/promises','./.semio-captured-fs-promises'],['fs/promises','./.semio-captured-fs-promises'],['node:module','./.semio-captured-module'],['module','./.semio-captured-module']]),redirect=(source,commonJs=false)=>redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets,refusal);\\012const orderedSourceFiles=[...value.files].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),freezeSelection=item=>{if(item&&typeof item==='object'&&!Object.isFrozen(item)){Object.freeze(item);for(const member of Object.values(item))freezeSelection(member);}return item;};Object.defineProperty(globalThis,'__semioCapturedSelectionV1',{value:freezeSelection({physical:{version:1,root:value.root,sources:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256}))},resolutions:{version:1,root:value.root,files:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256,imports:row.imports}))}}),enumerable:false,writable:false,configurable:false});\\012Bun.plugin({name:'closed-immutable-implementation',setup(build){build.onResolve({filter:/.*/,namespace:'semio-source'},item=>{const name=item.path.replace(/^semio-source:/,'');if(!images.has(name))throw Error('Uncaptured require namespace image refused');return{path:name,namespace:'semio-source'};});build.onResolve({filter:/^[.][/][.]semio-captured-module$/},()=>({path:'captured-module',namespace:'semio-immutable'}));build.onLoad({filter:/^captured-module$/,namespace:'semio-immutable'},()=>({loader:'object',exports:capturedModule}));build.onResolve({filter:/^[.][/][.]semio-captured-fs(?:-promises)?$/},item=>({path:item.path.endsWith('-promises')?'captured-fs-promises':'captured-fs',namespace:'semio-immutable'}));build.onResolve({filter:/.*/},item=>{if(item.path.startsWith('semio-capture/')||item.importer?.startsWith('semio-capture:'))return globalThis.__semioPlanResolveV1?.(item);if(resolving.depth||item.path.startsWith('node:')||item.path.startsWith('bun:')||(awaitModule.isBuiltin(item.path)))return;const importer=item.importer?.replace(/^[/]?semio-source:/,''),declared=importer&&declaredTarget(importer,item.path);if(declared&&installedTarget(declared)){installedResolve(awaitModule.createRequire(importer),item.path,declared);return;}if(importer&&!images.has(importer)&&importer.split(path.sep).includes('node_modules'))return;const candidate=declared?path.join(value.root,declared):item.path.startsWith('file:')?url.fileURLToPath(item.path):path.isAbsolute(item.path)?item.path:undefined;if(!candidate||!images.has(candidate)||!declared&&candidate!==entry)throw sourceImportRefusal('import',item.path,importer,Boolean(importer&&images.has(importer)),Boolean(declared),Boolean(candidate&&images.has(candidate)));return{path:candidate,namespace:'semio-source'};});build.onLoad({filter:/^captured-fs$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedFs,default:capturedFs}}));build.onLoad({filter:/^captured-fs-promises$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedPromises,default:capturedPromises}}));build.onLoad({filter:/.*/,namespace:'semio-capture'},item=>globalThis.__semioPlanLoadV1(item));build.onLoad({filter:/.*/,namespace:'semio-source'},item=>{check();const name=path.resolve(item.path),bytes=images.get(name);if(!bytes)throw Error('Uncaptured implementation module refused: '+name);sourceTraceV1.record('m',name);if(path.extname(name)==='.json'){currentImage(name,bytes);return{loader:'object',exports:{default:JSON.parse(bytes.toString('utf8'))}};}if(name.endsWith('.cjs')){const owner=JSON.stringify(name),directory=JSON.stringify(path.dirname(name));return{contents:\"import {__beginCapturedCommonJsV1 as begin,__failCapturedCommonJsV1 as fail} from './.semio-captured-module'; const owner=begin(\"+owner+\");try{(function(module,exports,require,__filename,__dirname){\\n\"+redirect(bytes.toString('utf8').replace(/^#![^\\r\\n]*/,''),true)+\"\\n})(owner,owner.exports,owner.require,\"+owner+\",\"+directory+\");owner.loaded=true;}catch(error){fail(\"+owner+\");throw error;}export default owner.exports;\",loader:'js'};}const loader=/\\.[cm]?tsx?$/.test(name)?'ts':'js',compiler=new Bun.Transpiler({loader,target:'bun',define:{'import.meta.semioCapturedPhysicalSources':'globalThis.__semioCapturedSelectionV1.physical','import.meta.semioCapturedSourceResolutions':'globalThis.__semioCapturedSelectionV1.resolutions','import.meta.semioCapturedImplementationIdentity':JSON.stringify(expected),'import.meta.semioCapturedLoaderIdentity':JSON.stringify(loaderIdentity()??''),'import.meta.url':JSON.stringify(url.pathToFileURL(name).href),'import.meta.path':JSON.stringify(name),'import.meta.filename':JSON.stringify(name),'import.meta.dir':JSON.stringify(path.dirname(name)),'import.meta.dirname':JSON.stringify(path.dirname(name)),'import.meta.main':'false'}}),transformed=compiler.transformSync(bytes.toString('utf8'));check();return{contents:redirect(transformed),loader:'js'};});}});\\012for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map))if(installedTarget(target)&&installedChecked.get(specifier)!==target)try{installedResolve(awaitModule.createRequire(name),specifier,target);}catch(error){if(error?.code==='SEMIO_INSTALLED_DRIFT')throw error;}\\012 const module=await import(entry);check();if(typeof module.runCapturedBuildSourceV1!=='function')throw Error('Closed captured implementation entry required');await module.runCapturedBuildSourceV1(args,{version:1,snapshot:snapshotPath,sha256:expected,loaderSha256:loaderIdentity(),root:value.root,recheck:()=>{for(const[name,bytes]of images)currentImage(name,bytes);check();},text:()=>encoded.toString('utf8')});check();\\012 -- 6048bbcd56e51272f0863b7e2d75405ed836796991af5b10b2fe949722d3c958 service host run"
      },
      {
        "pid": 21227,
        "ppid": 1,
        "rss_kib": 402528,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo service host run"
      },
      {
        "pid": 1,
        "ppid": 0,
        "rss_kib": 19472,
        "args": "/sbin/launchd"
      }
    ],
    "started_by": "coordinator native bootstrap; reused if started during prior try/round"
  }
]
~~~

Required capacity, graph, taken, inferred, waiting, restored, closure, heavy and access lines, verbatim:
~~~text
NATIVE
[capacity] run: sized at 5% of the default from 14 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1205 MB reserved (9523 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 14 earlier run(s) of its class
[capacity] run: sized at 30% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 30% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 25% of the default from 14 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1510 MB reserved (9346 MB available, 2048 MB floor)
[capacity] dependencies: sized at 25% of the default from 14 earlier run(s) of its class
[capacity] run: sized at 38% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9322 MB available, 2048 MB floor)
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 201 and delivered 0 within 901 s, 201 are still missing, among them the too heavy project(s) workspace (about 13780 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.

RESIDENT SERVICE
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (8565 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:111514:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 111514 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:127017:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 127017 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:24576:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 24576 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1031:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1031 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1023:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1023 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1326:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1326 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1195:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1195 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:191:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 191 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1247:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1247 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 122 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1009:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1009 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2456:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2456 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:775:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 775 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:81920:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 81920 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:770:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 770 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:214:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 214 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:811:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 811 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:290:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 290 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2481:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2481 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:10:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 10 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2803:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2803 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2465:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2465 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:122880:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 122880 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2707:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2707 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2004:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2004 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2848:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2848 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1708:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1708 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:155648:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 155648 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1421:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1421 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1784:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1784 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:20:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 20 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:7867:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 7867 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1136:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1136 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3627:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3627 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1044:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1044 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1312:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1312 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1175:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1175 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1567:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1567 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2174:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2174 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1287:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1287 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1776:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1776 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1610:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1610 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:212992:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 212992 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:227:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 227 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:773:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 773 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:917:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 917 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:294:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 294 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:143:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 143 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1511:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1511 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2962:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2962 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2623:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2623 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:654:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 654 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3411:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3411 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1206:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1206 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:868:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 868 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:286720:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 286720 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:839:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 839 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:34:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 34 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:378:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 378 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1405:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1405 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:310952:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 310952 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:16384:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 16384 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:24576:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 24576 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:28672:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 28672 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:40960:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 40960 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:45056:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 45056 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:53248:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 53248 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:65536:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 65536 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:69632:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 69632 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 86016 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:94208:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 94208 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:106496:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 106496 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:126976:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 126976 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:172032:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 172032 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180224:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180224 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184320:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184320 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:188416:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 188416 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:200704:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 200704 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:204800:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 204800 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:217088:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 217088 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:221184:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 221184 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:233472:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 233472 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:237568:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 237568 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:288083:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 288083 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:292179:292179","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 292179 of 292179 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:4:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 4 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:5:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 5 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:12:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 12 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:14:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 14 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:16:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 16 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:17:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 17 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:24:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 24 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:26:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 26 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:29:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 29 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:37:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 37 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:41:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 41 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:45:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 45 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:59:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 59 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:72:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 72 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:91:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 91 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:106:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 106 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:116:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 116 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:118:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 118 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:123:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 123 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:135:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 135 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:137:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 137 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:139:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 139 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:146:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 146 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:148:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 148 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:149:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 149 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:150:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 150 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:151:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 151 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:152:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 152 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:154:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 154 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:157:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 157 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:166:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 166 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:170:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 170 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:177:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 177 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:179:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 179 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:183:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 183 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:185:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 185 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:187:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 187 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:194:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 194 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:201:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 201 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:215:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 215 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:216:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 216 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:226:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 226 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:230:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 230 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:231:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 231 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:235:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 235 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:253:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 253 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1387","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1387:1387","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:5","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:5:5","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:272:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 272 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:288:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 288 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:306:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 306 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:325:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 325 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:339:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 339 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:353:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 353 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:357:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 357 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:364:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 364 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:377:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 377 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:397:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 397 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:415:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 415 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:432:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 432 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:446:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 446 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:447:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 447 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:463:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 463 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:478:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 478 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:500:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 500 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:759","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:759:759","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:2","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:2:2","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:530:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 530 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:550:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 550 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:569:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 569 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:582:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 582 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:601:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 601 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:619:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 619 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:638:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 638 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:656:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 656 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:675:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 675 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:692:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 692 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:710:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 710 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:727:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 727 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:749:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 749 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1728","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1728:1728","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:8","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:8:8","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:789:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 789 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:809:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 809 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:822:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 822 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:838:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 838 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:855:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 855 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:875:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 875 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:897:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 897 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:919:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 919 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:934:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 934 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:950:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 950 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:972:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 972 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:990:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 990 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1009:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1009 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1020:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1020 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:1681","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1681:1681","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:9","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:9:9","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 614 stale projects, 94 taken from the fleet, 6 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/print (about 3628 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 520
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 0 of 520 projects arrived
graph: waiting for the fleet: 413 of 520 projects arrived
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"lease-sweep:claims","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"lease-sweep:resources","mode":"exclusive","elapsedMs":7}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"lease-sweep:nested","mode":"exclusive","elapsedMs":12}}
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:97265:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 97265 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:127017:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 127017 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:24576:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 24576 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1031:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1031 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1023:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1023 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1326:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1326 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1195:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1195 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:191:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 191 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1247:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1247 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 122 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1009:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1009 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2456:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2456 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:775:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 775 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:770:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 770 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:214:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 214 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:90112:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 90112 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:811:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 811 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:290:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 290 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2481:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2481 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:10:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 10 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2803:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2803 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2465:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2465 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2707:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2707 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:131072:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 131072 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2004:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2004 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2848:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2848 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1708:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1708 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1421:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1421 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1784:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1784 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:20:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 20 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:7867:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 7867 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:172032:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 172032 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1136:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1136 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3627:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3627 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1044:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1044 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1312:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1312 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1175:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1175 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1567:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1567 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2174:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2174 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1287:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1287 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1776:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1776 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1610:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1610 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:227:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 227 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:773:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 773 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:221184:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 221184 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:917:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 917 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:294:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 294 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:143:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 143 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1511:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1511 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2962:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2962 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2623:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2623 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:654:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 654 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3411:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3411 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1206:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1206 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:868:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 868 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:286720:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 286720 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:839:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 839 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:34:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 34 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:378:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 378 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1405:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1405 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:310952:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 310952 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:40960:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 40960 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:61440:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 61440 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:69632:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 69632 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:90112:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 90112 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:126976:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 126976 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:155648:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 155648 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:172032:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 172032 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:176128:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 176128 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180224:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180224 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184320:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184320 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:192512:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 192512 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:200704:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 200704 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:204800:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 204800 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:208896:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 208896 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:221184:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 221184 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:233472:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 233472 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:237568:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 237568 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:286720:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 286720 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:292175:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 292175 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:12:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 12 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:17:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 17 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:26:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 26 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:37:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 37 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:46:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 46 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:75:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 75 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:80:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 80 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:84:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 84 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:90:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 90 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:108:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 108 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:116:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 116 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:135:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 135 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:146:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 146 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:183:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 183 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:227:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 227 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2242:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:4658:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:7346:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9948:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:12661:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:14268:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:267:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 267 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:277:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 277 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:284:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 284 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:287:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 287 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:289:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 289 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:295:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 295 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:296:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 296 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:323:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 323 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:363:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 363 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:409:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 409 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:466:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 466 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9723:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9861:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:588:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 588 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:595:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 595 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:628:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 628 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:663:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 663 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:703:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 703 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:721:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 721 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11371","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11371:11371","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:896:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 896 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1020:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1020 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11573","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11573:11573","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:252","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:252:252","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: refreshing 614 projects (generation 62)
graph: 97 stale facts (analysis 71, file 25, exists 1) leave 614 projects stale
graph: @semio-tech/filesystem-observation is stale: the workspace file 🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🧭️denied-entry/🟦️.ts was added
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🟦️.ts changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🟦️.ts changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/⌨️command/🌐️host/🟨️.mjs changed
graph: @semio-tech/framework-rs is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/⌨️command/🌐️host/🟨️.mjs changed
graph: @semio-tech/s-dev-composition is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts changed
graph: 82 more stale inputs are not listed
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614","mode":"shared","elapsedMs":0}}
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614:1:0:%40semio-tech%2Fframework-async-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[capacity] admitted exec with 2 of 8 job(s): the host cannot carry more right now
graph: inferring projects with 2 worker processes (1536 MB each)
[capacity] waiting 0s for exec: not enough free memory (need 1536 MB; 613 MB spare above the 2048 MB floor, load 5.72; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 444 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614:2:0:%40semio-tech%2Fframework-async-rs|language%3Acargo|0|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
graph: waiting for memory headroom (882 of 1024 MB reserved, 2 projects running)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614:3:0:%40semio-tech%2Fframework-async-rs|language%3Acargo|0|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|0|0,%40semio-tech%2Fframework-editor-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614:6:5:%40semio-tech%2Fframework-async-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|0|0,%40semio-tech%2Fframework-editor-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
graph: waiting for memory headroom (2046 of 2089 MB reserved, 7 projects running)
[capacity] waiting 6s for exec: not enough free memory (need 1536 MB; 520 MB spare above the 2048 MB floor, load 5.5; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 444 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":1}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614:8:5:%40semio-tech%2Fframework-async-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-editor-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":1030}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":1072}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":1060}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":2089}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":2628}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":2616}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:614:8:10:%40semio-tech%2Fframework-async-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-editor-rs|language%3Acargo|5|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":3186}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:1:614:8:10:%40semio-tech%2Fframework-async-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-editor-rs|language%3Acargo|5|0","mode":"shared","elapsedMs":0}}
graph: waiting for memory headroom (2547 of 2379 MB reserved, 7 projects running)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:3:614:7:10:%40semio-tech%2Fframework-async-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-editor-rs|language%3Acargo|5|0","mode":"shared","elapsedMs":0}}
[capacity] waiting 12s for exec: not enough free memory (need 1536 MB; 300 MB spare above the 2048 MB floor, load 5.38; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 444 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:5:614:5:10:%40semio-tech%2Fframework-async-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-actor-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:7:614:4:10:%40semio-tech%2Fframework-async-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-diagnostic-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-actor-rs|language%3Acargo|5|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:8:614:3:10:%40semio-tech%2Fframework-async-rs|language%3Acargo|10|0,%40semio-tech%2Fframework-actor-rs|language%3Acargo|5|0,%40semio-tech%2Ffilesystem-observation|project|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:8:614:3:15:%40semio-tech%2Fframework-async-rs|language%3Acargo|15|0,%40semio-tech%2Fframework-actor-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-compiler-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
graph: waiting for memory headroom (1916 of 1918 MB reserved, 3 projects running)
[capacity] waiting 17s for exec: not enough free memory (need 1536 MB; 124 MB spare above the 2048 MB floor, load 5.43; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 590 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:9:614:3:15:%40semio-tech%2Fframework-async-rs|language%3Acargo|15|0,%40semio-tech%2Fframework-actor-rs|language%3Acargo|5|0,%40semio-tech%2Fframework-compiler-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:10:614:2:15:%40semio-tech%2Fframework-async-rs|language%3Acargo|15|0,%40semio-tech%2Fframework-actor-rs|language%3Acargo|5|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:11:614:3:15:%40semio-tech%2Fframework-async-rs|language%3Acargo|15|0,%40semio-tech%2Fframework-machine|language%3Acargo|0|0,%40semio-tech%2Fframework-canvas-scene-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:13:614:2:0:%40semio-tech%2Fframework-canvas-camera-rs|language%3Arust|0|0,%40semio-tech%2Fframework-canvas-assets-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:16:614:1:0:%40semio-tech%2Fframework-canvas-icons-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
graph: headroom of 521 MB is below one project (639 MB), inferring one project at a time
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:17:614:1:0:%40semio-tech%2Fframework-deflate-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:18:614:1:0:%40semio-tech%2Fframework-dsl-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:19:614:1:0:%40semio-tech%2Fframework-dsl-record-derive-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
graph: headroom of 732 MB again (639 MB taken), resuming
graph: waiting for memory headroom (639 of 732 MB reserved, 1 projects running)
[capacity] waiting 22s for exec: not enough free memory (need 1536 MB; 132 MB spare above the 2048 MB floor, load 5.07; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 590 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:20:614:1:0:%40semio-tech%2Fframework-dsl-record-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:21:614:2:0:%40semio-tech%2Fframework-artifact-reference-rs|language%3Acargo|0|0,%40semio-tech%2Fframework-io-control-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
graph: headroom of 59 MB is below one project (633 MB), inferring one project at a time
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:23:614:1:0:%40semio-tech%2Fframework-io-base64-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
graph: headroom of 708 MB again (633 MB taken), resuming
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:24:614:1:0:%40semio-tech%2Fframework-io-schema-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:25:614:2:0:%40semio-tech%2Fframework-io-sqlite-model-rs|language%3Arust|0|0,%40semio-tech%2Fframework-job-worker-authority-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:25:614:2:0:%40semio-tech%2Fframework-job-worker-authority-rs|language%3Arust|0|0,%40semio-tech%2Fframework-job-worker-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
graph: waiting for memory headroom (881 of 973 MB reserved, 2 projects running)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:26:614:2:0:%40semio-tech%2Fframework-job-worker-authority-rs|language%3Arust|0|0,%40semio-tech%2Fframework-job-worker-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:27:614:1:0:%40semio-tech%2Fframework-job-worker-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[capacity] waiting 28s for exec: not enough free memory (need 1536 MB; -48 MB spare above the 2048 MB floor, load 4.99; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 746 MB), workspace:setup (quiet, 1 job(s), 44 MB)
graph: headroom of -932 MB is below one project (583 MB), inferring one project at a time
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:27:614:1:0:%40semio-tech%2Fframework-job-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
graph: waiting for memory headroom (2265 of -267 MB reserved, 1 projects running)
graph: project @semio-tech/framework-job-rs (language:cargo) grew by 2696 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
[capacity] waiting 34s for exec: not enough free memory (need 1536 MB; -1475 MB spare above the 2048 MB floor, load 4.83; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 746 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:27:614:1:5:%40semio-tech%2Fframework-job-rs|language%3Acargo|5|0:stopped|2696|1756|language%3Acargo","mode":"shared","elapsedMs":0}}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (9409 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:32600:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 32600 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:47062:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 47062 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:60242:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 60242 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:94236:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 94236 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:127017:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 127017 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1031:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1031 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1023:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1023 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:40960:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 40960 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1326:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1326 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1195:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1195 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:191:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 191 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1247:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1247 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 122 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1009:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1009 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2456:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2456 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:775:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 775 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:770:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 770 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:214:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 214 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:811:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 811 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:290:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 290 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2481:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2481 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:10:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 10 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:106496:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 106496 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2803:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2803 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2465:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2465 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2707:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2707 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2004:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2004 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:139264:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 139264 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2848:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2848 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1708:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1708 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1421:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1421 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1784:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1784 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:20:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 20 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:7867:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 7867 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1136:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1136 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3627:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3627 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:180224:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 180224 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1044:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1044 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1312:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1312 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1175:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1175 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1567:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1567 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2174:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2174 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1287:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1287 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1776:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1776 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1610:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1610 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:227:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 227 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:773:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 773 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:917:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 917 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:294:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 294 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:143:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 143 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1511:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1511 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:237568:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 237568 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2962:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2962 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2623:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2623 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:654:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 654 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3411:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3411 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1206:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1206 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:868:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 868 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:839:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 839 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:34:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 34 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:378:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 378 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1405:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1405 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:310952:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 310952 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:16384:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 16384 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:32768:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 32768 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:49152:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 49152 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:65536:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 65536 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:69632:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 69632 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 86016 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:98304:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 98304 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:143360:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 143360 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:172032:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 172032 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180224:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180224 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:188416:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 188416 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:200704:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 200704 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:212992:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 212992 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:221184:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 221184 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:233472:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 233472 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:237568:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 237568 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:245760:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 245760 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:279887:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 279887 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:292175:292175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 292175 of 292175 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:12:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 12 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:23:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 23 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:84:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 84 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:121:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 121 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:135:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 135 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:146:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 146 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:177:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 177 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:183:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 183 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:222:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 222 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:237:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 237 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:242:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 242 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:256:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 256 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:502:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1289:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:2010:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:3109:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:4315:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:5188:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:6533:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9132:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:12132:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:14268:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:421:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 421 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9861:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:726:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 726 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11371","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11371:11371","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:942:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 942 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:947:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 947 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:954:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 954 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1020:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1020 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11573","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11573:11573","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:252","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:252:252","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 614 stale projects, 413 taken from the fleet, 1 too heavy for this machine (workspace (about 13780 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 201
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: waiting for the fleet: 0 of 201 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 201 stale projects and delivered 0 within 901 s; 201 remain, among them the too heavy project(s) workspace (about 13780 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T23:42:58.043Z graph-wanted-expired {"wanted":"201","arrived":"0","seconds":"901","missing":"201","local":"192","heavy":", among them the too heavy project(s) workspace (about 13780 MB) (this machine can give about 3512 MB per project)"}
[capacity] the memory footprint of running process trees cannot be read on darwin (Command failed: footprint -f bytes --noCategories -p 21345); the resident set is used instead, which can overstate what a process holds
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (Command failed: footprint -f bytes --noCategories -p 21345); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
~~~

Service last 30 lines at completion:

host-9888631afea4.log

~~~text
[repo] still working after 710 s; last phase started: source.served
[repository:command] still running after 710 s
[repository:command] still running after 720 s
[repository:command] still running after 730 s
[repo] still working after 740 s; last phase started: source.served
[repository:command] still running after 740 s
[repository:command] still running after 750 s
[repository:command] still running after 760 s
[repo] still working after 770 s; last phase started: source.served
[repository:command] still running after 770 s
[repository:command] still running after 780 s
[repository:command] still running after 790 s
[repo] still working after 800 s; last phase started: source.served
[repository:command] still running after 800 s
[repository:command] still running after 810 s
[repository:command] still running after 820 s
[repo] still working after 830 s; last phase started: source.served
[repository:command] still running after 830 s
[repository:command] still running after 840 s
[repository:command] still running after 850 s
[repo] still working after 860 s; last phase started: source.served
[repository:command] still running after 860 s
[repository:command] still running after 870 s
[repository:command] still running after 880 s
[repo] still working after 890 s; last phase started: source.served
[repository:command] still running after 890 s
[repository:command] still running after 900 s
[repository:command] still running after 910 s
2026-11-14T23:18:12.762Z shutdown superseded
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
~~~

host-ac6bfe8f337c.log

~~~text
[repository:command] still running after 830 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 840 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 850 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repo] still working after 860 s; last phase started: source.served
[repository:command] still running after 860 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 870 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 880 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repo] still working after 890 s; last phase started: source.served
[repository:command] still running after 890 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 900 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 910 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repo] still working after 920 s; last phase started: source.served
[repository:command] still running after 920 s
graph: waiting for the fleet: 0 of 201 projects arrived
[repository:command] still running after 930 s
graph: waiting for the fleet: 0 of 201 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 201 stale projects and delivered 0 within 901 s; 201 remain, among them the too heavy project(s) workspace (about 13780 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T23:42:58.043Z graph-wanted-expired {"wanted":"201","arrived":"0","seconds":"901","missing":"201","local":"192","heavy":", among them the too heavy project(s) workspace (about 13780 MB) (this machine can give about 3512 MB per project)"}
[capacity] the memory footprint of running process trees cannot be read on darwin (Command failed: footprint -f bytes --noCategories -p 21345); the resident set is used instead, which can overstate what a process holds
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (Command failed: footprint -f bytes --noCategories -p 21345); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
[repository:command] still running after 940 s
~~~

Internal inference-unit guard first line with 15 preceding lines and inner exit, verbatim:
~~~text
graph: headroom of -932 MB is below one project (583 MB), inferring one project at a time
[script] {"owner":"repository:build","command":"build:projects","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:27:48:1:0:%40semio-tech%2Fframework-job-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 27 of 48 projects · 1 running, longest 0 s: @semio-tech/framework-job-rs (language:cargo, 0 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:27:614:1:0:%40semio-tech%2Fframework-job-rs|language%3Acargo|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
graph: waiting for memory headroom (2265 of -267 MB reserved, 1 projects running)
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
graph: project @semio-tech/framework-job-rs (language:cargo) grew by 2696 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
[capacity] waiting 34s for exec: not enough free memory (need 1536 MB; -1475 MB spare above the 2048 MB floor, load 4.83; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 746 MB), workspace:setup (quiet, 1 job(s), 44 MB)
[repository:command] still running after 610 s
~~~

## Try 2

Launcher **UNCHANGED** during the try and from preceding final identity (measured flags False, False). Inode 1262266, mtime 2026-11-14T21:00:20.886970+00:00, target inode 1262265, target mtime_ns 1794690020871943656, readlink versions/cf25f2b751f83336/repo. Required exact ls -li/readlink/codesign outputs:
~~~json
{
  "before": {
    "at": "2026-11-14T23:45:01Z",
    "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
    "paths": {
      "/Users/ueli/.local/share/semio/launcher/bin/repo": {
        "exists": true,
        "is_symlink": true,
        "inode": 1262266,
        "mtime_ns": 1794690020886970333,
        "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
        "target_inode": 1262265,
        "target_mtime_ns": 1794690020871943656,
        "size": 6523360,
        "readlink": "versions/cf25f2b751f83336/repo"
      },
      "/Users/ueli/.local/bin/repo": {
        "exists": true,
        "is_symlink": false,
        "inode": 1256211,
        "mtime_ns": 1794688211341948235,
        "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
        "target_inode": 1256211,
        "target_mtime_ns": 1794688211341948235,
        "size": 18657778,
        "readlink": null
      }
    },
    "codesign_rc": 0,
    "codesign_lines": [
      "Identifier=tech.semio.repo",
      "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
      "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "TeamIdentifier=not set"
    ]
  },
  "after": {
    "at": "2026-11-14T23:54:08Z",
    "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
    "paths": {
      "/Users/ueli/.local/share/semio/launcher/bin/repo": {
        "exists": true,
        "is_symlink": true,
        "inode": 1262266,
        "mtime_ns": 1794690020886970333,
        "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
        "target_inode": 1262265,
        "target_mtime_ns": 1794690020871943656,
        "size": 6523360,
        "readlink": "versions/cf25f2b751f83336/repo"
      },
      "/Users/ueli/.local/bin/repo": {
        "exists": true,
        "is_symlink": false,
        "inode": 1256211,
        "mtime_ns": 1794688211341948235,
        "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
        "target_inode": 1256211,
        "target_mtime_ns": 1794688211341948235,
        "size": 18657778,
        "readlink": null
      }
    },
    "codesign_rc": 0,
    "codesign_lines": [
      "Identifier=tech.semio.repo",
      "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
      "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
      "TeamIdentifier=not set"
    ]
  }
}
~~~

Peak run-owned RSS 2749744 KiB (2.622 GiB); machine-wide sampled peak 2749744 KiB. Required 30-second top-three samples 19, including PID/PPID/full commands and ancestry. Five-second pressure counts {"1": 109}; non-normal 30-second samples []. No resource stop; thresholds RSS 8 GiB, critical 4 twice at 30 seconds, warning 2 six times.

Succeeded setup tasks: none emitted. Failed Nx setup tasks: none emitted. Not-run task names: not measured, because no downstream result list was emitted. Internal graph inference-unit failure is quoted separately.

First / last observed arrived lines:
~~~text
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 413 of 424 projects arrived
~~~

First native refusal plus 15 preceding lines, or final interruption context if no refusal:
~~~text
Reading the project graph slice (432 s): flush 1 of 1.
Reading the project graph slice (434 s): files 0 of 0.
Reading the project graph slice (436 s): doubt 310952 of 310952.
Reading the project graph slice (438 s): facts 86016 of 292176.
[repo] still working after 440 s; last phase started: door.script
Reading the project graph slice (440 s): facts 180224 of 292176.
Reading the project graph slice (442 s): facts 188416 of 292176.
Reading the project graph slice (444 s): facts 204800 of 292176.
Reading the project graph slice (446 s): facts 225280 of 292176.
Reading the project graph slice (448 s): facts 225280 of 292176.
Reading the project graph slice (450 s): facts 233472 of 292176.
Reading the project graph slice (452 s): facts 292176 of 292176.
Reading the project graph slice (454 s): facts 292176 of 292176.
Reading the project graph slice (456 s): verification 1 of 1020.
Reading the project graph slice (458 s): verification 1 of 1020.
Reading the project graph slice (460 s): verification 1 of 1020.
~~~

Named-heavy MB values in the exact graph/refusal text are estimates from the native log, not measured RSS.

Observed phase wall, approximately five-second polling; exact internal wall not measured:

| Phase | Start elapsed s | Wall until next phase/controller end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 5.278 | 0.0 |
| [semio] Installing the native repository command… | 5.278 | 10.1 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 15.378 | 10.201 |
| [semio] workspace:setup… | 25.579 | 521.019 |

The final try interval includes the signal-grace minute. Connectivity at completion:
~~~json
{
  "at": "2026-11-14T23:54:08Z",
  "ports": {
    "2049": {
      "reachable": true
    },
    "6094": {
      "reachable": true
    },
    "8981": {
      "reachable": true
    }
  }
}
~~~

EPERM / exact where / consent / caller-fallback lines, verbatim:
~~~text
None emitted in the native and observed service streams.
~~~

Actual host-incarnation parent chains; no job environment blocks were copied:
~~~json
[
  {
    "metadata_file": "host-ac6bfe8f337c.json",
    "at": "2026-11-14T23:45:01Z",
    "pid": 21242,
    "startedAt": 1794698842200,
    "implementation": "28e87fbf8be568a33deb5279c63bba36ecaaaf5f280e6e180d23885eed938d00",
    "parent_chain": [
      {
        "pid": 21242,
        "ppid": 21227,
        "rss_kib": 510864,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \\012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('node:module');\\012const hash=value=>crypto.createHash('sha256').update(value).digest('hex'),check=()=>{const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;if(wire){const deadline=JSON.parse(wire).deadlineEpochMilliseconds;if(deadline!==null&&(!Number.isSafeInteger(deadline)||Date.now()>=deadline))throw Error('Original immutable implementation deadline exhausted');}};\\012const closureRefusalTextV1=(function closureRefusalTextV1(table, code, fields, environment) {\\012  const row = table[code];\\012  if (!row)\\012    return code + \": \" + JSON.stringify(fields);\\012  const tag = environment.SEMIO_LANGUAGE, languages = tag && row[tag] ? [tag] : Object.keys(row), fill = (text) => text.replace(/\\{(\\w+)\\}/gu, (_all, key) => String(fields[key] ?? \"?\"));\\012  return languages.map((language) => fill(row[language].refusal) + \". \" + fill(row[language].fix)).join(\" | \");\\012}),closureRefusalCatalogTableV1=(function closureRefusalCatalogTableV1(encoded, sha256, hash) {\\012  try {\\012    const bytes = Buffer.from(encoded ?? \"\", \"base64\");\\012    if (!encoded || bytes.toString(\"base64\") !== encoded || hash(bytes) !== sha256)\\012      return;\\012    const table = JSON.parse(bytes.toString(\"utf8\"))?.$defs?.Messages?.const;\\012    return table && typeof table === \"object\" ? table : void 0;\\012  } catch {\\012    return;\\012  }\\012}),refusalCatalog={table:undefined,read:()=>undefined},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalCatalog.read()??{},code,fields,process.env));\\012const evaluation=process.execArgv.reduce((indices,arg,index)=>{if(arg==='--eval'||arg==='-e'||arg.startsWith('--eval='))indices.push(index);return indices;},[]);if(evaluation.length!==1||process.execArgv[evaluation[0]]!=='--eval'||typeof process.execArgv[evaluation[0]+1]!=='string'||!process.execArgv[evaluation[0]+1])throw Error('Actual single evaluated immutable loader required');\\012const evaluatedLoader=process.execArgv[evaluation[0]+1],snapshotPath=null,loaderIdentity=(()=>{let known;return()=>known??=hash(evaluatedLoader);})(),[expected,...args]=process.argv.slice(process.argv[1]==='--'?2:1);check();if(!/^[a-f0-9]{64}$/.test(expected??''))throw refusal('digest-required',{});\\012let encoded;const snapshotFromEnv=process.env.SEMIO_GRAPH_WORKER_SNAPSHOT;if(process.env.SEMIO_GRAPH_WORKER==='1'&&(typeof snapshotFromEnv!=='string'||!snapshotFromEnv.length))throw Error('Graph worker snapshot path required');if(typeof snapshotFromEnv==='string'&&snapshotFromEnv.length){const opened=fs.openSync(snapshotFromEnv,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW??0));try{const fileFacts=fs.fstatSync(opened);if(!fileFacts.isFile()||fileFacts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(fileFacts.size);let filled=0;while(filled<encoded.length){check();const count=fs.readSync(opened,encoded,filled,encoded.length-filled,null);if(!count)throw refusal('snapshot-changed',{});filled+=count;}}finally{fs.closeSync(opened);}}else{const facts=fs.fstatSync(3),chunks=[],chunk=Buffer.alloc(65536);let size=0,used=0;if(facts.isFile()){if(facts.size>33554432)throw refusal('snapshot-bounded',{});encoded=Buffer.allocUnsafe(facts.size);while(used<encoded.length){check();const count=fs.readSync(3,encoded,used,encoded.length-used,null);if(!count)throw refusal('snapshot-changed',{});used+=count;}if(fs.readSync(3,Buffer.alloc(1),0,1,null))throw refusal('snapshot-bounded',{});}else{for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,33554432-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>33554432-size)throw refusal('snapshot-bounded',{});size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}encoded=Buffer.concat(chunks,size);}}check();if(hash(encoded)!==expected)throw refusal('snapshot-changed',{});\\012const redirectCapturedBuiltinReadsV1=(function redirectCapturedBuiltinReadsV1(source, commonJs, parser, targets, refuse) {\\012  const names = [...targets.keys()].join(\"|\");\\012  if (!new RegExp(`[\"'\\\\x60](?:` + names + `)[\"'\\\\x60]`).test(source))\\012    return source;\\012  const expected = parser.scan(source).imports.filter((row) => targets.has(row.path)).length;\\012  let count = 0;\\012  const quick = source.replace(new RegExp(`(\\\\b(?:from|import))(\\\\s*)([\"'])(` + names + \")\\\\3|(\\\\b(?:import\" + (commonJs ? \"\" : \"|require|__require\") + `))(\\\\s*\\\\(\\\\s*)([\"'])(` + names + \")\\\\7\", \"g\"), (_match, a, b, _quote, name, c, d, _second, other) => {\\012    count++;\\012    return name !== void 0 ? a + b + JSON.stringify(targets.get(name)) : c + d + JSON.stringify(targets.get(other));\\012  });\\012  if (count === expected && !parser.scan(quick).imports.some((row) => targets.has(row.path)))\\012    return count ? quick : source;\\012  const changes = [], length = source.length;\\012  let previous = \"\", prior = \"\", at = 0;\\012  while (at < length) {\\012    const code = source.charCodeAt(at);\\012    if (code === 32 || code > 8 && code < 14 || code > 127 && /\\s/.test(source[at])) {\\012      at++;\\012      continue;\\012    }\\012    if (code === 47) {\\012      const next = source.charCodeAt(at + 1);\\012      if (next === 47) {\\012        at = source.indexOf(`\\012`, at + 2);\\012        if (at < 0)\\012          break;\\012        continue;\\012      }\\012      if (next === 42) {\\012        const end = source.indexOf(\"*/\", at + 2);\\012        if (end < 0)\\012          throw refuse(\"compiler-comment-unclosed\", {});\\012        at = end + 2;\\012        continue;\\012      }\\012    }\\012    if (code === 96) {\\012      at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === 96)\\012          break;\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code === 34 || code === 39) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (c === 92) {\\012          at += 2;\\012          continue;\\012        }\\012        at++;\\012        if (c === code)\\012          break;\\012      }\\012      if (at - start < 20) {\\012        const value = source.slice(start, at).slice(1, -1);\\012        if (targets.has(value) && (previous === \"from\" || previous === \"import\" || previous === 40 && (prior === \"import\" || !commonJs && (prior === \"require\" || prior === \"__require\"))))\\012          changes.push({ start, end: at, value: JSON.stringify(targets.get(value)) });\\012      }\\012      prior = previous;\\012      previous = -1;\\012      continue;\\012    }\\012    if (code > 64 && code < 91 || code > 96 && code < 123 || code === 95 || code === 36) {\\012      const start = at++;\\012      while (at < length) {\\012        const c = source.charCodeAt(at);\\012        if (!(c > 64 && c < 91 || c > 96 && c < 123 || c > 47 && c < 58 || c === 95 || c === 36))\\012          break;\\012        at++;\\012      }\\012      const size = at - start;\\012      prior = previous;\\012      previous = size === 4 || size === 6 || size === 7 || size === 9 ? source.slice(start, at) : -2;\\012      continue;\\012    }\\012    prior = previous;\\012    previous = code;\\012    at++;\\012  }\\012  if (changes.length !== expected)\\012    throw refuse(\"builtin-read-coverage-refused\", {});\\012  if (!changes.length)\\012    return source;\\012  let result = source;\\012  for (const change of changes.reverse())\\012    result = result.slice(0, change.start) + change.value + result.slice(change.end);\\012  if (parser.scan(result).imports.some((row) => targets.has(row.path)))\\012    throw refuse(\"builtin-read-remained\", {});\\012  return result;\\012}),splitCapturedSnapshotV1=(function splitCapturedSnapshotV1(encoded) {\\012  const pieces = [], spans = [], open = Buffer.from(',\"base64\":\"'), next = Buffer.from('}},{\"path\":\"');\\012  let from = 0, at = 0;\\012  for (;; ) {\\012    const key = encoded.indexOf(open, at);\\012    if (key < 0)\\012      break;\\012    const start = key + open.length, end = encoded.indexOf(34, start);\\012    if (end < 0 || encoded.toString(\"latin1\", end, end + 12) !== '\",\"imports\":')\\012      return;\\012    pieces.push(encoded.toString(\"utf8\", from, start));\\012    spans.push([start, end]);\\012    from = end;\\012    const boundary = encoded.indexOf(next, end);\\012    if (boundary < 0)\\012      break;\\012    at = boundary;\\012  }\\012  if (!spans.length)\\012    return;\\012  pieces.push(encoded.toString(\"utf8\", from));\\012  return { text: pieces.join(\"\"), spans };\\012}),admitCapturedImagesV1=(function admitCapturedImagesV1(value, encoded, spans, limits, hash, path, check, refuse) {\\012  const resolutions = new Map, rows = new Map, decoded = new Map, eager = new Map;\\012  let total = 0;\\012  const decode = (name, index) => {\\012    const row = value.files[index], [start, end] = spans[index], text = encoded.toString(\"latin1\", start, end), bytes = Buffer.from(text, \"base64\");\\012    if (bytes.toString(\"base64\") !== text || hash(bytes) !== row.sha256)\\012      throw refuse(\"implementation-image-changed\", {});\\012    decoded.set(name, bytes);\\012    return bytes;\\012  };\\012  for (let index = 0;index < value.files.length; index++) {\\012    const row = value.files[index];\\012    check();\\012    if (!row || Object.keys(row).length !== 4 || !Object.keys(row).every((key) => key === \"base64\" || key === \"imports\" || key === \"path\" || key === \"sha256\") || typeof row.path !== \"string\" || path.isAbsolute(row.path) || row.path.includes(\":\") || row.path.includes(String.fromCharCode(92)) || row.path.includes(String.fromCharCode(0)) || row.path.split(\"/\").some((part) => !part || part === \".\" || part === \"..\") || typeof row.base64 !== \"string\" || !/^[a-f0-9]{64}$/.test(row.sha256 ?? \"\"))\\012      throw refuse(\"implementation-image-required\", {});\\012    const name = path.join(value.root, row.path);\\012    if (spans) {\\012      const [start, end] = spans[index], length = end - start, size = length % 4 ? -1 : length / 4 * 3 - (encoded[end - 1] === 61 ? 1 : 0) - (encoded[end - 2] === 61 ? 1 : 0);\\012      total += size;\\012      if (size < 0 || size > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || rows.has(name))\\012        throw refuse(\"implementation-image-changed\", {});\\012      rows.set(name, index);\\012    } else {\\012      const bytes = Buffer.from(row.base64, \"base64\");\\012      total += bytes.length;\\012      if (bytes.toString(\"base64\") !== row.base64 || bytes.length > limits.maximumFileBytes || total > limits.maximumSourceBytes || name.length > 256 || eager.has(name) || hash(bytes) !== row.sha256)\\012        throw refuse(\"implementation-image-changed\", {});\\012      eager.set(name, bytes);\\012    }\\012    if (!row.imports || typeof row.imports !== \"object\" || Array.isArray(row.imports))\\012      throw refuse(\"import-resolution-required\", {});\\012    resolutions.set(name, row.imports);\\012    if (!spans)\\012      row.base64 = \"\";\\012  }\\012  if (!spans)\\012    return { images: eager, resolutions };\\012  return { images: { has: (name) => rows.has(name), get: (name) => {\\012    const bytes = decoded.get(name);\\012    if (bytes)\\012      return bytes;\\012    const index = rows.get(name);\\012    return index === void 0 ? void 0 : decode(name, index);\\012  }, keys: () => rows.keys(), [Symbol.iterator]: () => decoded.entries() }, resolutions };\\012});\\012const split=splitCapturedSnapshotV1(encoded),value=JSON.parse(split?split.text:encoded.toString('utf8')),spans=split&&Array.isArray(value?.files)&&split.spans.length===value.files.length?split.spans:undefined;if(!value||Object.keys(value).sort().join(',')!=='entry,files,root,version'||value.version!==1||typeof value.root!=='string'||!path.isAbsolute(value.root)||value.root.length>256||typeof value.entry!=='string'||!Array.isArray(value.files)||!value.files.length||value.files.length>2048)throw Error('Closed immutable implementation snapshot required');const catalogAt=value.files.findIndex(row=>row&&row.path===\"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json\"),catalogEager=catalogAt<0||spans?undefined:value.files[catalogAt].base64;refusalCatalog.read=()=>refusalCatalog.table??=catalogAt<0?undefined:closureRefusalCatalogTableV1(spans?encoded.toString('latin1',spans[catalogAt][0],spans[catalogAt][1]):catalogEager,value.files[catalogAt].sha256,hash);const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,{maximumFileBytes:4194304,maximumSourceBytes:20971520},hash,path,check,refusal);for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map)){if(typeof target==='string'&&/^installed:(?:@[a-z0-9._-]+[/])?[a-z0-9._-]+@[^\\s]{1,64}$/.test(target)&&specifier)continue;if(typeof target!=='string'||!specifier||path.isAbsolute(target)||target.split('/').some(part=>!part||part==='.'||part==='..')||target.includes(String.fromCharCode(92))||!images.has(path.join(value.root,target)))throw Error('Closed captured resolution target required');}const entry=path.join(value.root,value.entry);if(!images.has(entry))throw Error('Captured implementation entry required');\\012const capturedSourceImportDiagnosticV1=(function capturedSourceImportDiagnosticV1(kind, specifier, importer, declaringImage, declared, targetImage) {\\012  const clip = (value) => {\\012    if (value == null)\\012      return null;\\012    let text = \"\", count = 0;\\012    for (const character of value) {\\012      text += character;\\012      if (++count === 256)\\012        break;\\012    }\\012    return text;\\012  };\\012  return Object.freeze({ version: 1, purpose: \"captured-source-import-refusal\", kind, importer: clip(importer), specifier: clip(specifier), declaringImage, declared, targetImage });\\012}),rootScheme=\"semio-root:\",installedScheme=\"installed:\",sourceImportRefusal=(kind,specifier,importer,declaringImage,declared,targetImage)=>{const diagnostic=capturedSourceImportDiagnosticV1(kind,specifier,importer&&path.isAbsolute(importer)?path.relative(value.root,importer).split(path.sep).join('/'):importer,declaringImage,declared,targetImage),error=refusal(kind,{specifier:diagnostic.specifier,importer:diagnostic.importer});error.message+=' '+JSON.stringify(diagnostic);return error;};\\012const sourceTraceKeyV1=Symbol.for('semio.sourceTrace.v1'),sourceTraceSeenV1=new Set(),sourceTraceBufferV1=[],sourceTraceSinkV1={file:undefined},sourceTraceRelativeV1=name=>{const relative=path.relative(value.root,name);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)?relative.split(path.sep).join('/'):name;},sourceTraceAppendV1=fs.appendFileSync;\\012globalThis[sourceTraceKeyV1]=Object.freeze({record:(kind,name)=>{const text=kind+' '+(kind==='a'?name:sourceTraceRelativeV1(name));if(sourceTraceSeenV1.has(text))return;sourceTraceSeenV1.add(text);if(sourceTraceSinkV1.file===undefined){if(sourceTraceBufferV1.length<65536)sourceTraceBufferV1.push(text);return;}if(sourceTraceSinkV1.file!==null)try{sourceTraceAppendV1(sourceTraceSinkV1.file,text+'\\n');}catch{}},enable:directory=>{if(sourceTraceSinkV1.file!==undefined)return;try{fs.mkdirSync(directory,{recursive:true});const file=path.join(directory,process.pid+'-'+Date.now()+'.trace');fs.writeFileSync(file,'s '+JSON.stringify({argv:process.argv.slice(2),entry:value.entry,files:value.files.length,sha256:expected})+'\\n'+sourceTraceBufferV1.map(line=>line+'\\n').join(''));sourceTraceBufferV1.length=0;sourceTraceSinkV1.file=file;}catch{sourceTraceSinkV1.file=null;}}});\\012const sourceTraceV1=globalThis[sourceTraceKeyV1];\\012const originalRead=fs.readFileSync,codeDirectories=new Set([...images.keys()].filter(name=>/\\.[cm]?[jt]sx?$/.test(name)).map(name=>path.dirname(name))),capturedSchemaOwner=file=>{if(path.extname(file)!=='.json'||path.basename(file)==='package.json')return false;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return false;const parts=relative.split(path.sep),at=parts.indexOf('\\u{1f9ec}\\uFE0Fschema');return at>=0&&codeDirectories.has(path.join(value.root,...parts.slice(0,at)));},currentImage=(name,expected)=>{check();if(images.get(name)!==expected)throw Error('Captured implementation Source image required: '+name);};\\012const capturedRead=(name,options)=>{check();let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));const bytes=file&&images.get(file);if(bytes){currentImage(file,bytes);sourceTraceV1.record('r',file);const encoding=typeof options==='string'?options:options?.encoding;return encoding?bytes.toString(encoding):Buffer.from(bytes);}if(file&&capturedSchemaOwner(file))throw refusal('schema-read',{specifier:path.relative(value.root,file).split(path.sep).join('/')});if(file)sourceTraceV1.record('x',file);return originalRead(name,options);};\\012const promises=await import('node:fs/promises'),capturedPromises={...promises,readFile:async(name,options)=>{let file;if(name instanceof URL)file=url.fileURLToPath(name);else if(typeof name==='string'||Buffer.isBuffer(name))file=path.resolve(String(name));if(file&&(images.has(file)||capturedSchemaOwner(file)))return capturedRead(name,options);if(file)sourceTraceV1.record('x',file);return promises.readFile(name,options);}},capturedFs={...fs,readFileSync:capturedRead,readFile:(name,options,callback)=>{const cb=typeof options==='function'?options:callback;capturedPromises.readFile(name,typeof options==='function'?undefined:options).then(bytes=>cb(null,bytes),error=>cb(error));},promises:capturedPromises};\\012const nativeRequire=require,commonJs=new Map(),installedTarget=target=>typeof target==='string'&&target.startsWith(installedScheme),declaredTarget=(importer,specifier)=>{const map=resolutions.get(importer);if(!map)return undefined;if(Object.hasOwn(map,specifier))return map[specifier];const file=specifier.startsWith('file:')?url.fileURLToPath(specifier):specifier;if(!path.isAbsolute(file))return undefined;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return undefined;const key=relative.split(path.sep).join('/');return Object.hasOwn(map,rootScheme+key)?map[rootScheme+key]:images.has(path.resolve(file))?key:undefined;},installedChecked=new Map(),resolving={depth:0},physicalResolve=(requireFrom,specifier)=>{resolving.depth++;try{return requireFrom.resolve(specifier);}finally{resolving.depth--;}},installedResolve=(requireFrom,specifier,target)=>{const resolved=physicalResolve(requireFrom,specifier);if(installedChecked.get(specifier)!==target){const marker=resolved.lastIndexOf(path.sep+'node_modules'+path.sep),pieces=marker<0?[]:resolved.slice(marker+path.sep.length+13).split(path.sep),segments=target.slice(installedScheme.length).replace(/@[^@]*$/,'').split('/'),directory=marker<0?'':resolved.slice(0,marker+path.sep.length+13)+pieces.slice(0,segments.length).join(path.sep);let version;try{version=JSON.parse(originalRead(path.join(directory,'package.json'),'utf8')).version;}catch{version=undefined;}if(!directory||installedScheme+segments.join('/')+'@'+version!==target)throw Object.assign(refusal('installed-drift',{specifier,detail:target.slice(installedScheme.length)+' -> '+(version??'unreadable')}),{code:'SEMIO_INSTALLED_DRIFT'});installedChecked.set(specifier,target);}return resolved;},capturedCreateRequire=base=>{const name=base instanceof URL||typeof base==='string'&&base.startsWith('file:')?url.fileURLToPath(base):String(base);const actual=awaitModule.createRequire(base),resolve=(specifier,dataOnly)=>{check();if(typeof specifier!=='string')throw Error('Closed captured require specifier required');specifier=({'./.semio-captured-fs':'node:fs','./.semio-captured-fs-promises':'node:fs/promises','./.semio-captured-module':'node:module'})[specifier]??specifier;if(awaitModule.isBuiltin(specifier))return specifier;if(!images.has(path.resolve(name)))return physicalResolve(actual,specifier);const target=declaredTarget(path.resolve(name),specifier);if(!target&&dataOnly)return physicalResolve(actual,specifier);if(!target)throw sourceImportRefusal('require',specifier,name,true,false,false);if(installedTarget(target))return installedResolve(actual,specifier,target);return path.join(value.root,target);},load=specifier=>{if(!images.has(path.resolve(name)))throw sourceImportRefusal('declaring-module',undefined,name,false,false,false);if(installedTarget(declaredTarget(path.resolve(name),specifier))){installedResolve(actual,specifier,declaredTarget(path.resolve(name),specifier));return actual(specifier);}const target=resolve(specifier);if(awaitModule.isBuiltin(target)){if(target==='fs'||target==='node:fs')return capturedFs;if(target==='fs/promises'||target==='node:fs/promises')return capturedPromises;if(target==='module'||target==='node:module')return capturedModule;return actual(target);}if(target.endsWith('.cjs')&&commonJs.has(target))return commonJs.get(target).exports;const loaded=nativeRequire('semio-source:'+target);return target.endsWith('.cjs')?commonJs.get(target)?.exports:target.endsWith('.json')?loaded.default:loaded;};load.resolve=specifier=>resolve(specifier,true);return load;},moduleDefault=awaitModule.default??awaitModule,capturedModule={...awaitModule,__beginCapturedCommonJsV1:name=>{if(!images.has(name)||!name.endsWith('.cjs')||commonJs.has(name))throw Error('Unique captured CommonJS module required');const owner=new(awaitModule.Module??awaitModule.default)(name);owner.filename=name;owner.paths=[];owner.require=capturedCreateRequire(name);commonJs.set(name,owner);return owner;},__failCapturedCommonJsV1:name=>commonJs.delete(name),createRequire:capturedCreateRequire,default:new Proxy(moduleDefault,{get:(target,key)=>key==='createRequire'?capturedCreateRequire:Reflect.get(target,key)})};\\012const redirectParser=new Bun.Transpiler({loader:'js',target:'bun'}),redirectTargets=new Map([['node:fs','./.semio-captured-fs'],['fs','./.semio-captured-fs'],['node:fs/promises','./.semio-captured-fs-promises'],['fs/promises','./.semio-captured-fs-promises'],['node:module','./.semio-captured-module'],['module','./.semio-captured-module']]),redirect=(source,commonJs=false)=>redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets,refusal);\\012const orderedSourceFiles=[...value.files].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path))),freezeSelection=item=>{if(item&&typeof item==='object'&&!Object.isFrozen(item)){Object.freeze(item);for(const member of Object.values(item))freezeSelection(member);}return item;};Object.defineProperty(globalThis,'__semioCapturedSelectionV1',{value:freezeSelection({physical:{version:1,root:value.root,sources:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256}))},resolutions:{version:1,root:value.root,files:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256,imports:row.imports}))}}),enumerable:false,writable:false,configurable:false});\\012Bun.plugin({name:'closed-immutable-implementation',setup(build){build.onResolve({filter:/.*/,namespace:'semio-source'},item=>{const name=item.path.replace(/^semio-source:/,'');if(!images.has(name))throw Error('Uncaptured require namespace image refused');return{path:name,namespace:'semio-source'};});build.onResolve({filter:/^[.][/][.]semio-captured-module$/},()=>({path:'captured-module',namespace:'semio-immutable'}));build.onLoad({filter:/^captured-module$/,namespace:'semio-immutable'},()=>({loader:'object',exports:capturedModule}));build.onResolve({filter:/^[.][/][.]semio-captured-fs(?:-promises)?$/},item=>({path:item.path.endsWith('-promises')?'captured-fs-promises':'captured-fs',namespace:'semio-immutable'}));build.onResolve({filter:/.*/},item=>{if(item.path.startsWith('semio-capture/')||item.importer?.startsWith('semio-capture:'))return globalThis.__semioPlanResolveV1?.(item);if(resolving.depth||item.path.startsWith('node:')||item.path.startsWith('bun:')||(awaitModule.isBuiltin(item.path)))return;const importer=item.importer?.replace(/^[/]?semio-source:/,''),declared=importer&&declaredTarget(importer,item.path);if(declared&&installedTarget(declared)){installedResolve(awaitModule.createRequire(importer),item.path,declared);return;}if(importer&&!images.has(importer)&&importer.split(path.sep).includes('node_modules'))return;const candidate=declared?path.join(value.root,declared):item.path.startsWith('file:')?url.fileURLToPath(item.path):path.isAbsolute(item.path)?item.path:undefined;if(!candidate||!images.has(candidate)||!declared&&candidate!==entry)throw sourceImportRefusal('import',item.path,importer,Boolean(importer&&images.has(importer)),Boolean(declared),Boolean(candidate&&images.has(candidate)));return{path:candidate,namespace:'semio-source'};});build.onLoad({filter:/^captured-fs$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedFs,default:capturedFs}}));build.onLoad({filter:/^captured-fs-promises$/,namespace:'semio-immutable'},()=>({loader:'object',exports:{...capturedPromises,default:capturedPromises}}));build.onLoad({filter:/.*/,namespace:'semio-capture'},item=>globalThis.__semioPlanLoadV1(item));build.onLoad({filter:/.*/,namespace:'semio-source'},item=>{check();const name=path.resolve(item.path),bytes=images.get(name);if(!bytes)throw Error('Uncaptured implementation module refused: '+name);sourceTraceV1.record('m',name);if(path.extname(name)==='.json'){currentImage(name,bytes);return{loader:'object',exports:{default:JSON.parse(bytes.toString('utf8'))}};}if(name.endsWith('.cjs')){const owner=JSON.stringify(name),directory=JSON.stringify(path.dirname(name));return{contents:\"import {__beginCapturedCommonJsV1 as begin,__failCapturedCommonJsV1 as fail} from './.semio-captured-module'; const owner=begin(\"+owner+\");try{(function(module,exports,require,__filename,__dirname){\\n\"+redirect(bytes.toString('utf8').replace(/^#![^\\r\\n]*/,''),true)+\"\\n})(owner,owner.exports,owner.require,\"+owner+\",\"+directory+\");owner.loaded=true;}catch(error){fail(\"+owner+\");throw error;}export default owner.exports;\",loader:'js'};}const loader=/\\.[cm]?tsx?$/.test(name)?'ts':'js',compiler=new Bun.Transpiler({loader,target:'bun',define:{'import.meta.semioCapturedPhysicalSources':'globalThis.__semioCapturedSelectionV1.physical','import.meta.semioCapturedSourceResolutions':'globalThis.__semioCapturedSelectionV1.resolutions','import.meta.semioCapturedImplementationIdentity':JSON.stringify(expected),'import.meta.semioCapturedLoaderIdentity':JSON.stringify(loaderIdentity()??''),'import.meta.url':JSON.stringify(url.pathToFileURL(name).href),'import.meta.path':JSON.stringify(name),'import.meta.filename':JSON.stringify(name),'import.meta.dir':JSON.stringify(path.dirname(name)),'import.meta.dirname':JSON.stringify(path.dirname(name)),'import.meta.main':'false'}}),transformed=compiler.transformSync(bytes.toString('utf8'));check();return{contents:redirect(transformed),loader:'js'};});}});\\012for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map))if(installedTarget(target)&&installedChecked.get(specifier)!==target)try{installedResolve(awaitModule.createRequire(name),specifier,target);}catch(error){if(error?.code==='SEMIO_INSTALLED_DRIFT')throw error;}\\012 const module=await import(entry);check();if(typeof module.runCapturedBuildSourceV1!=='function')throw Error('Closed captured implementation entry required');await module.runCapturedBuildSourceV1(args,{version:1,snapshot:snapshotPath,sha256:expected,loaderSha256:loaderIdentity(),root:value.root,recheck:()=>{for(const[name,bytes]of images)currentImage(name,bytes);check();},text:()=>encoded.toString('utf8')});check();\\012 -- 6048bbcd56e51272f0863b7e2d75405ed836796991af5b10b2fe949722d3c958 service host run"
      },
      {
        "pid": 21227,
        "ppid": 1,
        "rss_kib": 403088,
        "args": "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo service host run"
      },
      {
        "pid": 1,
        "ppid": 0,
        "rss_kib": 21616,
        "args": "/sbin/launchd"
      }
    ],
    "started_by": "coordinator native bootstrap; reused if started during prior try/round"
  }
]
~~~

Required capacity, graph, taken, inferred, waiting, restored, closure, heavy and access lines, verbatim:
~~~text
NATIVE
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1191 MB reserved (8781 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] run: sized at 30% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 30% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1510 MB reserved (8612 MB available, 2048 MB floor)
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class
[capacity] run: sized at 38% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8402 MB available, 2048 MB floor)

RESIDENT SERVICE
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (8402 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:62877:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 62877 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:127017:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 127017 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:24576:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 24576 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1031:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1031 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1023:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1023 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1326:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1326 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1195:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1195 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:191:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 191 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1247:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1247 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 122 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1009:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1009 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:73728:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 73728 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2456:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2456 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:775:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 775 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:770:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 770 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:214:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 214 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:811:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 811 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:290:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 290 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2481:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2481 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:10:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 10 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2803:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2803 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:114688:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 114688 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2465:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2465 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2707:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2707 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2004:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2004 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2848:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2848 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:147456:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 147456 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1708:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1708 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1421:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1421 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1784:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1784 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:20:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 20 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:7867:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 7867 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1136:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1136 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3627:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3627 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1044:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1044 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1312:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1312 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1175:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1175 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1567:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1567 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2174:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2174 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1287:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1287 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:204800:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 204800 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1776:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1776 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1610:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1610 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:227:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 227 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:773:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 773 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:917:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 917 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:294:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 294 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:143:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 143 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1511:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1511 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2962:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2962 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2623:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2623 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:654:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 654 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3411:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3411 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:278528:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 278528 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1206:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1206 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:868:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 868 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:839:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 839 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:34:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 34 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:378:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 378 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1405:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1405 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:310952:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 310952 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:12288:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 12288 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:32768:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 32768 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:45056:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 45056 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:57344:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 57344 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:65536:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 65536 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:77824:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 77824 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:81920:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 81920 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 86016 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:94208:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 94208 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:102400:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 102400 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:118784:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 118784 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:135168:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 135168 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:172032:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 172032 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:176128:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 176128 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184320:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184320 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:192512:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 192512 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:200704:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 200704 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:204800:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 204800 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:208896:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 208896 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:212992:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 212992 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:217088:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 217088 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:221184:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 221184 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:233472:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 233472 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:237568:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 237568 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:278528:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 278528 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:292177:292177","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 292177 of 292177 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:12:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 12 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:26:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 26 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:85:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 85 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:121:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 121 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:135:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 135 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:165:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 165 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:183:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 183 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:217:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 217 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:256:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 256 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:1837:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:4556:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:6473:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9278:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:12103:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:14268:14268","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:257:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 257 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:412:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 412 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:9861:9861","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:513:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 513 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11371","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11371:11371","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:256:256","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:769:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 769 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1004:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1004 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1020:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1020 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:0:11573","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:parts:11573:11573","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:invalidation:1:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:0:252","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:rederivation:252:252","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:0:1","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:flush:1:1","mode":"shared","elapsedMs":0}}
graph: 614 stale projects, 190 taken from the fleet, 11 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 27770 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 5 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 424
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 0 of 424 projects arrived
graph: waiting for the fleet: 413 of 424 projects arrived
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"lease-sweep:claims","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"lease-sweep:resources","mode":"exclusive","elapsedMs":6}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"lease-sweep:nested","mode":"exclusive","elapsedMs":9}}
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:0:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 0 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:69723:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 69723 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:membership:127017:127017","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying workspace membership: 127017 of 127017 entries
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:0:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 0 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3295:3295","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3295 of 3295 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2512:2512","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2512 of 2512 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:964:964","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 964 of 964 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:24576:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 24576 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1031:1031","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1031 of 1031 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:16:16","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 16 of 16 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1023:1023","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1023 of 1023 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:471:471","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 471 of 471 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1326:1326","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1326 of 1326 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:238:238","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 238 of 238 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1195:1195","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1195 of 1195 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:191:191","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 191 of 191 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1247:1247","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1247 of 1247 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:122:122","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 122 of 122 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1009:1009","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1009 of 1009 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2456:2456","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2456 of 2456 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:775:775","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 775 of 775 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:81920:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 81920 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:770:770","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 770 of 770 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:214:214","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 214 of 214 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:811:811","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 811 of 811 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:290:290","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 290 of 290 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2481:2481","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2481 of 2481 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:10:10","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 10 of 10 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2803:2803","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2803 of 2803 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2465:2465","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2465 of 2465 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:122880:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 122880 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2707:2707","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2707 of 2707 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2004:2004","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2004 of 2004 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2848:2848","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2848 of 2848 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:147456:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 147456 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1708:1708","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1708 of 1708 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1421:1421","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1421 of 1421 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1784:1784","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1784 of 1784 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:20:20","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 20 of 20 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:7867:7867","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 7867 of 7867 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1136:1136","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1136 of 1136 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:3627:3627","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 3627 of 3627 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1044:1044","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1044 of 1044 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1312:1312","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1312 of 1312 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:188416:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 188416 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1175:1175","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1175 of 1175 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1567:1567","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1567 of 1567 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:2174:2174","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 2174 of 2174 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1287:1287","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1287 of 1287 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1776:1776","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1776 of 1776 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1610:1610","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1610 of 1610 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:227:227","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 227 of 227 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:773:773","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 773 of 773 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:917:917","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 917 of 917 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:294:294","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 294 of 294 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:143:143","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 143 of 143 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1511:1511","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1511 of 1511 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:33:33","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 33 of 33 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2962:2962","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2962 of 2962 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:245760:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 245760 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:11:11","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 11 of 11 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:2623:2623","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 2623 of 2623 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:654:654","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 654 of 654 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:311:311","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 311 of 311 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:3411:3411","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 3411 of 3411 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:0","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 0 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1206:1206","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1206 of 1206 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:868:868","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 868 of 868 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:1:1","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 1 of 1 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:839:839","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 839 of 839 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:34:34","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 34 of 34 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:0:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 0 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:directories:378:378","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking directory stamps: 378 of 378 directories
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:0:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 0 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:files:1405:1405","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Checking file stamps: 1405 of 1405 files
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:doubt:310952:310952","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Classifying graph facts: 310952 of 310952 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:0:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 0 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:40960:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 40960 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:61440:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 61440 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:73728:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 73728 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:94208:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 94208 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:135168:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 135168 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:159744:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 159744 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:172032:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 172032 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:176128:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 176128 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:180224:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 180224 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:184320:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 184320 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:192512:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 192512 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:196608:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 196608 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:200704:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 200704 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:204800:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 204800 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:212992:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 212992 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:221184:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 221184 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:233472:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 233472 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:237568:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 237568 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:278528:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 278528 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:292176:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 292176 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1020 closures
~~~

Service last 30 lines at completion:

host-ac6bfe8f337c.log

~~~text
[script] graph:slice-worker · Verifying graph facts: 221184 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:225280:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 225280 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:229376:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 229376 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:233472:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 233472 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:237568:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 237568 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:278528:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 278528 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:facts:292176:292176","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying graph facts: 292176 of 292176 facts
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:0:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 0 of 1020 closures
[repository:command] still running after 1540 s
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:1:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 1 of 1020 closures
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:verification:2:1020","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Verifying input closures: 2 of 1020 closures
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
[repo] still working after 1550 s; last phase started: source.served
[repository:command] still running after 1550 s
[repository:command] still running after 1560 s
[repository:command] still running after 1570 s
[repo] still working after 1580 s; last phase started: source.served
[repository:command] still running after 1580 s
[repository:command] still running after 1590 s
[repository:command] still running after 1600 s
~~~

## Access attribution

No fstatat EPERM, where: root= entry= code=, consent or explicit app-access fallback line was observed. No access route is inferred; parent chains and launcher identities are measured.

## Replica status

Supported replica status exit 0:
~~~text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 9971)
281932 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Recurring failures of kind round: 4 times, last 1474 s ago.
Sidecar build-agent: running (process 10288, 0 restart(s), 3004 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 3037 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 3037 s in this state, last exit -).
Journal: 10.71 MiB in 14 segments of 10 origins.
Newest snapshot by g1-server: 43659s old, 89.18 MiB.
Reclaimable now: 2 journal segments (2.29 MiB).
Last compaction 53122s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 2 ms 0.7%, directory sweep 461 ms 10%, file sweep 4476 ms 5%, tree walk 22001 ms 1.1%, git view 24011 ms 5%.
Latency over the last 62 publishes and 100 applies: publish p50 317 ms, p95 1395 ms; apply p50 564 ms, p95 1500 ms.
g1-m01: 0 events (0 B) behind, last event 25s ago
g1-m02: 0 events (0 B) behind, last event 6s ago
g1-m03: 0 events (0 B) behind, last event 24s ago
g1-m04: 0 events (0 B) behind, last event 12s ago
g1-m05: 0 events (0 B) behind, last event 19247s ago
g1-m06: 0 events (0 B) behind, last event 269s ago
g1-m07: 0 events (0 B) behind, last event 16s ago
g1-m08: 0 events (0 B) behind, last event 337s ago
g1-m10: 0 events (0 B) behind, last event 40s ago
g1-server: 0 events (0 B) behind, last event 400s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 1s 0/0 cp:1s on eabbc9226732 148/12/874 184/367|324/1733 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m02 macos 3s 0/0 cp:3s on eabbc9226732 148/12/872 265/570|430/1694 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m03 macos 3s 0/0 cp:3s on eabbc9226732 148/12/875 171/525|407/1430 ok
  g1-m04 linux 1s 0/0 cp:1s on eabbc9226732 148/11/875 179/637|455/1781 diverged
  g1-m05 windows 2s 0/4 cp:- on 09732cf2089f 152/12/849 -/-|2072/5539 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 2s 0/0 cp:2s on eabbc9226732 148/12/875 61/52779|304/1269 ok
  g1-m07 macos 2s 0/0 cp:2s on eabbc9226732 148/12/875 240/4443|331/1622 ok
  g1-m08 macos 2s 0/0 cp:2s on eabbc9226732 148/12/875 317/1395|564/1500 ok
  g1-m10 - - - now - - - - missing (no attestation)
  g1-server macos 0s 0/0 cp:0s on eabbc9226732 148/12/875 163/246|326/1384 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  warning: g1-m06 runs git 2.50.1, other machines run another version
  g1-m04 differs in view.digest at a common checkpoint: #16=71c3,#52=6fde,#86=80bb,#114=8b6c,#121=b976,#147=9309,#163=3887,#188=f53c,#21 instead of #16=5d52,#52=44e9,#86=2bba,#114=4eff,#121=09af,#147=414d,#163=e9ed,#188=7973,#21 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (10 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
~~~

Exact final 200 native lines: round-10.tail.txt; every try has its exact tail. All per-try evidence retained. No code, permissions or git state changed by the observer. Proceeding to coordinator round 11.

Report completed 2026-11-14T23:59:53Z.
