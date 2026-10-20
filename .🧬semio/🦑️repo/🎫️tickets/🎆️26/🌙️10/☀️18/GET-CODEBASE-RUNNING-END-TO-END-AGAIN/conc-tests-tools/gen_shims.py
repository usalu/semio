#!/usr/bin/env python3
"""Generates the three guard shim modules from staged/protocol.json (guard.writeTargets, guard.openFunctions). usage: gen_shims.py"""
import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
proto=json.load(open(HERE+"/staged/sb-protocol.json",encoding="utf8"))
PROMISE_ONLY=["writeFile","appendFile","mkdir","rm","rmdir","unlink","rename","copyFile","cp","truncate","symlink","link"]
SYNC_NAMES=[n for n in proto["guard"]["writeTargets"] if n not in PROMISE_ONLY]
HEAD='''type Guard = { write(fn: string, args: readonly unknown[]): void; open(fn: string, args: readonly unknown[]): void; spawn(command: unknown, args: readonly unknown[]): void; exec(command: unknown): void };
const guard = (): Guard | undefined => (globalThis as { __semioTestGuardV1?: Guard }).__semioTestGuardV1;
'''
def fs_shim():
    names=SYNC_NAMES+PROMISE_ONLY
    names=list(dict.fromkeys(names))
    out='import * as real from "node:fs";\nimport * as guardedPromises from "../🩻️fs-promises/🟦️.ts";\n\nexport * from "node:fs";\n'+HEAD
    out+='const writes = <Fn extends (...args: never[]) => unknown>(name: string, fn: Fn): Fn => ((...args: unknown[]) => { guard()?.write(name, args); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;\n'
    out+='const opens = <Fn extends (...args: never[]) => unknown>(name: string, fn: Fn): Fn => ((...args: unknown[]) => { guard()?.open(name, args); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;\n'
    for n in names: out+=f'export const {n} = writes("{n}", real.{n});\n'
    for n in proto["guard"]["openFunctions"]: out+=f'export const {n} = opens("{n}", real.{n});\n'
    out+='export const promises = guardedPromises.default;\n'
    out+='export default { ...real, '+", ".join(names+proto["guard"]["openFunctions"])+', promises };\n'
    return out
def fsp_shim():
    names=PROMISE_ONLY
    out='import * as real from "node:fs/promises";\n\nexport * from "node:fs/promises";\n'+HEAD
    out+='const writes = <Fn extends (...args: never[]) => unknown>(name: string, fn: Fn): Fn => ((...args: unknown[]) => { guard()?.write(name, args); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;\n'
    out+='const opens = <Fn extends (...args: never[]) => unknown>(name: string, fn: Fn): Fn => ((...args: unknown[]) => { guard()?.open(name, args); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;\n'
    for n in names: out+=f'export const {n} = writes("{n}", real.{n});\n'
    out+='export const open = opens("open", real.open);\n'
    out+='export default { ...real, '+", ".join(names)+', open };\n'
    return out
def cp_shim():
    out='import * as real from "node:child_process";\n\nexport * from "node:child_process";\n'+HEAD
    out+='const spawns = <Fn extends (...args: never[]) => unknown>(fn: Fn): Fn => ((...args: unknown[]) => { guard()?.spawn(args[0], Array.isArray(args[1]) ? args[1] : []); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;\n'
    out+='const execs = <Fn extends (...args: never[]) => unknown>(fn: Fn): Fn => ((...args: unknown[]) => { guard()?.exec(args[0]); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;\n'
    for n in ["spawn","spawnSync","execFile","execFileSync"]: out+=f'export const {n} = spawns(real.{n});\n'
    for n in ["exec","execSync"]: out+=f'export const {n} = execs(real.{n});\n'
    out+='export default { ...real, spawn, spawnSync, execFile, execFileSync, exec, execSync };\n'
    return out
for name,fn in [("sb-shim-fs.ts",fs_shim),("sb-shim-fsp.ts",fsp_shim),("sb-shim-cp.ts",cp_shim)]:
    open(HERE+"/staged/"+name,"w",encoding="utf8").write(fn())
print("shims ok")
