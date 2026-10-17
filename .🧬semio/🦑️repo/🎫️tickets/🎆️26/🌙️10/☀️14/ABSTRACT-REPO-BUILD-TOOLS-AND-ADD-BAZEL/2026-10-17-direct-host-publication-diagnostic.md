# Direct Host Native Publication Diagnostic — 2026-10-17

Parent delegated an isolated direct-host diagnostic after the prior declared-host publication retained a strict receipt conflict whose observed bytes were unchanged and ctime alone differed. This helper records evidence; it does not relax `currentStamp`, identify a mutation cause, qualify the prior failure, or expand publication acceptance.

## Authored Contract and Flow

`native-publication/host.schema.json` and `host.fixture.json` were authored before implementation. The closed schema fixes the existing120000ms Original ceiling,32MiB filesystem,33MiB physical envelope,4096-byte receipt cap, seven strict fields (dev,ino,mode,nlink,size,mtimeNs,ctimeNs), ten publication stages and six portable comparison vectors. Vectors cover unchanged identity,ctime-only changes,mtime+ctime,inode,SHA-only and all-field changes. Ajv validates the corpus; independent SQLite `json_each` comparison checks every expected changed-field set against the actual helper comparator.

The existing `native-publication/📜️script.ts` now admits standalone `--host`. It is a separate branch with a fresh120-second Original and the existing bootstrap-issued Native grant; it is not sequenced inside `--run`. It creates one Generated `nh-` fixture with its own `p` image and `l` lease directory. It opens the canonical persistent32MiB/33MiB storage, writes an actual64KiB payload and canonical empty checkpoint, records raw journal bytes, and fully closes the persistent handle. The fresh publication request is captured only after this close. The direct canonical publisher then obtains its own exact exclusive custody under the same real issued control; no parent-image operation is involved.

Awaited stage callbacks read only the bounded host `image.json`, through no-follow descriptors with seven-field descriptor/name stability and SHA256 checks. Every stage retains the strict fields, every observed changed field, and ownership(uid/gid),birthtimeNs and flags only if actually available. Flags are explicitly null when the framework does not expose them. These observations do not alter strict producer comparisons.

Actual independent Node processes read only explicitly named receipt,payload and journal records, with four-job/1MiB per-file/3MiB total bounds, stable no-follow descriptors and bounded transport. Before publication they confirm the fresh request receipt hash. After settled publication, ordinary canonical reopen checks payload bytes,raw journal bytes and an empty current allocation projection; Node independently confirms published receipt SHA and unchanged application/journal hashes. The reopened handle is fully closed. Only after all ten stages,settled qualification,payload/journal verification,reopen and close does exact fresh `expectedImage` canonical retirement run. No raw mount/image/metadata removal is performed.

Failure records `host-diagnostic.json` in the own Generated fixture, including the exact request,stages,independent reads,path/lease directory and custody flags. The helper attempts ordinary persistent-handle close, preserves primary/close failures, retains the fixture, and performs no canonical retirement after failure. Retained paths remain available for a separately authorized canonical recovery. Diagnostic `nh-` records are intentionally not removed by this implementation pass.

## Optional Xattr DATA

Parent additionally requested bounded Python xattr observations if available. The exact reader limits32names,4096total name bytes,255bytes per name,65536bytes per value and262144total value bytes, and checks stable receipt identity around observation. This machine's actual `python3` lacks `os.listxattr`/`getxattr`. The first strict/Source run reached TypeScript success and existing hardlink Source success, then failed with `AttributeError: module 'os' has no attribute 'listxattr'`. The optional diagnostic now explicitly reports `available:false` with the exact absent-API reason; it does not claim empty xattrs or successful xattr capture. Errors with present APIs remain failures. No new ABI,external runtime library,deadline or cap was introduced.

## Executed Source Evidence

The corrected exact existing `--strict` entry completed with terminal0 under its unchanged60-second Original. It ran strict TypeScript over the updated helper, the existing exact Node hardlink diagnostic, and the new host fixture/descriptor Source checks:

```text
[DEBUG] Exact embedded Node trace Source {"aliases":23,"independentFileDistinct":true,"records":27,"logicalBytes":576,"realGeneratedDescriptors":true,"nativeEffects":0}
[DEBUG] Exact host diagnostic Source {"cases":6,"fields":7,"independentNode":true,"pythonXattrsAvailable":false,"nativeEffects":0}
```

The host Source test creates a disposable normal Generated `nhs-` receipt, checks actual helper observation against exact independent Node descriptor output, records actual Python API availability, and removes only its own normal Source directory in finally. No Native image was opened,published,recovered or retired during this implementation pass. Real `--host` execution is pending parent's explicit coordination; Source checks are not Native acceptance.

Both actual `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` contain dedicated EN/DE `--host` entries in `4_gate` at924.2/924.3, adjacent the existing ticket helper region. Registration checks parsed both files and confirmed exact mode,group and all18 inherited override resets. All checked paths,including a potential longest generated journal path,are below256characters (161for the modeled generated checkpoint). Scoped whitespace validation returned0.

## Observed File Checkpoint

Shared files may change after these observed bytes.

| Repository Path | Bytes | SHA256 |
| --- | ---: | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/native-publication/📜️script.ts` | 59249 | `857178d9a567667b3268498ea345c453b013979f56ccad178005f2264133842b` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/native-publication/host.schema.json` | 4148 | `d289921da4a778af693c65f2c5685a61f3e4673b16744c7b7eb26f85849a87ac` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/native-publication/host.fixture.json` | 4423 | `e29427c381667abba73588663de11efc14de6e1dcd1326be16de17e42643e52c` |
| `.vscode/launch.json` | 8812064 | `0392d76ee3d468ae86631266e1890b4a387ceb36c3afdeff13557e2ca25143c4` |
| `.vscode/🧩️launch.seed.jsonc` | 6940201 | `8e6c8a062f391b401c861964ef46b61367200a15eb89660fd317478830df6e24` |
