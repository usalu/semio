#!/usr/bin/env python3
"""🪟 Edits of batch `native` applied in place to a root (a scratch copy of the live files or a mirror): python3 -I edit.py <root>. The session resolver reads the Windows parent chain from a kernel process snapshot (Toolhelp32 for id, parent and image name; one query handle per ancestor for the start time and the command line) through the runtime's foreign-function port instead of starting PowerShell and a CIM query on every command; the PowerShell program stays as the fallback for a host without the foreign-function interface. New module, independent-oracle laws and the schema constants come with it."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = sys.argv[1]
A = "🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions"
def edit(rel, pairs):
    path = os.path.join(ROOT, rel); text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text: continue
        if text.count(old) != 1: sys.exit("anchor %d× in %s: %s" % (text.count(old), rel, old[:70]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)
def full(rel, source):
    os.makedirs(os.path.dirname(os.path.join(ROOT, rel)), exist_ok=True)
    shutil.copyfile(os.path.join(HERE, "src", source), os.path.join(ROOT, rel))
full(A + "/🧭️ancestry/🪟️native/🟦️.ts", "native.ts")
full(A + "/🧭️ancestry/🪟️native/🧪️tests/🟦️.ts", "native.test.ts")
full(A + "/🧭️ancestry/🪟️native/🧪️tests/🐍️oracle/🐍️.py", "oracle.py")
SCHEMA = '''     "ancestryDepth": 16,
     "ancestry": {
      "windows": {
       "libraries": {
        "kernel": "kernel32.dll",
        "nt": "ntdll.dll"
       },
       "architectures": [
        "x64",
        "arm64"
       ],
       "pointerBytes": 8,
       "snapshotProcesses": 2,
       "queryLimitedInformation": 4096,
       "invalidHandle": "18446744073709551615",
       "entry": {
        "bytes": 568,
        "processId": 8,
        "parentId": 32,
        "name": 44,
        "nameUnits": 260
       },
       "commandLine": {
        "informationClass": 60,
        "headerBytes": 16,
        "capacityBytes": 65568
       },
       "fileTime": {
        "ticksPerSecond": 10000000,
        "microsecondTicks": 10,
        "unixEpochTicks": "116444736000000000",
        "maximumUnixSeconds": "253402300799"
       }
      }
     },
'''
edit(A + "/🧬️schema/🔣️.json", [('     "ancestryDepth": 16,\n', SCHEMA)])
NEW_FUNCTIONS = '''/** 🌉️ Walks the parent chain of one kernel process snapshot exactly like the CIM listing walk (an id of 4 or less, a missing parent and a parent that started after its child end the chain; the command is the command line followed by the image name): undefined when a process the snapshot lists refuses a query handle or exited meanwhile, so the caller asks the shell instead of reporting a shorter chain than the machine has. */
export function windowsHostAncestryV1(host:WindowsProcessHostV1,first:number=process.ppid,limit:number=depth):readonly AncestorV1[]|undefined{
 const byPid=new Map(host.entries().map(row=>[row.pid,row] as const)),found:AncestorV1[]=[];let pid=first,child:string|undefined;
 for(let step=0;step<limit&&pid>4;step++){
  const row=byPid.get(pid);if(!row)break;
  const detail=host.detail(pid);if(!detail)return undefined;
  if(child!==undefined&&detail.started>child)break;
  found.push({pid,started:detail.started,command:(detail.commandLine+" "+row.name).trim()});child=detail.started;pid=row.parent;
 }
 return found;
}

/** ⚡️ Reads the Windows parent chain in-process from the kernel (a Toolhelp32 snapshot and one query handle per ancestor, about a millisecond) instead of a PowerShell start plus a CIM query (hundreds of milliseconds on every `repo` process); null when the kernel path is unavailable or incomplete and the shell must answer. See https://learn.microsoft.com/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes */
export function windowsNativeAncestryV1(first:number,limit:number,options?:WindowsProcessOptionsV1):readonly AncestorV1[]|null{
 return withWindowsProcessHostV1(host=>windowsHostAncestryV1(host,first,limit),options)??null;
}

'''
edit(A + "/🧭️ancestry/🟦️.ts", [
 ('import {join} from "node:path";\n', 'import {join} from "node:path";\nimport {withWindowsProcessHostV1,type WindowsProcessHostV1,type WindowsProcessOptionsV1} from "./🪟️native/🟦️.ts";\n'),
 ('/** 🪪️ Reads the parent chain from `first`', NEW_FUNCTIONS + '/** 🪪️ Reads the parent chain from `first`'),
 ('one PowerShell CIM listing on Windows; a platform', 'one kernel process snapshot on Windows (the PowerShell CIM listing where the kernel path is unavailable); a platform'),
 ('export function systemAncestryV1(platform:NodeJS.Platform=process.platform,first:number=process.ppid,limit:number=depth):readonly AncestorV1[]{',
  'export function systemAncestryV1(platform:NodeJS.Platform=process.platform,first:number=process.ppid,limit:number=depth,native:(first:number,limit:number)=>readonly AncestorV1[]|null=windowsNativeAncestryV1):readonly AncestorV1[]{'),
 ('if(platform==="win32"){\n  const root=process.env.SystemRoot,', 'if(platform==="win32"){\n  const direct=native(first,limit);if(direct!==null)return direct;\n  const root=process.env.SystemRoot,'),
])
edit(A + "/🟦️.ts", [
 ('psAncestryV1,systemAncestryV1,windowsAncestryV1,type AncestorV1,type AncestryPortV1} from "./🧭️ancestry/🟦️.ts";',
  'psAncestryV1,systemAncestryV1,windowsAncestryV1,windowsHostAncestryV1,windowsNativeAncestryV1,type AncestorV1,type AncestryPortV1} from "./🧭️ancestry/🟦️.ts";\nexport {createWindowsProcessHostV1,openWindowsProcessBindingV1,windowsFileTimeIsoV1,withWindowsProcessHostV1,type WindowsProcessBindingV1,type WindowsProcessDetailV1,type WindowsProcessEntryV1,type WindowsProcessHostV1,type WindowsProcessOptionsV1} from "./🧭️ancestry/🪟️native/🟦️.ts";'),
])
edit("🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json", [
 ('      "id": "agent-sessions",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "files": [\n        "🪪️agent-sessions/🧪️tests/🟦️.ts"\n      ],',
  '      "id": "agent-sessions",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "files": [\n        "🪪️agent-sessions/🧪️tests/🟦️.ts",\n        "🪪️agent-sessions/🧭️ancestry/🪟️native/🧪️tests/🟦️.ts"\n      ],'),
])
print("native edits applied to", ROOT)
