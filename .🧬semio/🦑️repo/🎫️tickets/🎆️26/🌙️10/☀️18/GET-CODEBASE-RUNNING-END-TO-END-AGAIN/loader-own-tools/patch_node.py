#!/usr/bin/env python3
"""Anchored patch of closure/🟢️node/🟨️.mjs: installed-package targets pass through to the native resolver. Usage: patch_node.py <file>."""
import sys
path = sys.argv[1]
t = open(path, encoding="utf8").read()
def rep(a, b):
    global t
    n = t.count(a)
    assert n == 1, (n, a[:120])
    t = t.replace(a, b)
rep('for(const row of modules.values())for(const[specifier,target]of Object.entries(row.imports))if(typeof specifier!=="string"||!modules.has(pathToFileURL(safe(target)).href))throw Error("Complete captured Source import required");',
    'const installedTarget=target=>typeof target==="string"&&/^installed:(?:@[a-z0-9._-]+[/])?[a-z0-9._-]+@[^\\s]{1,64}$/u.test(target),installedRoots=new Set();for(const row of modules.values())for(const[specifier,target]of Object.entries(row.imports))if(typeof specifier!=="string"||!installedTarget(target)&&!modules.has(pathToFileURL(safe(target)).href))throw Error("Complete captured Source import required");')
rep('return{url:pathToFileURL(safe(parent.imports[specifier])).href,shortCircuit:true};}if(modules.has(specifier))return{url:specifier,shortCircuit:true};throw Error("Only captured Source modules may execute");},',
    'if(installedTarget(parent.imports[specifier])){const resolved=next(specifier,context),at=resolved.url.lastIndexOf("/node_modules/");if(at>=0)installedRoots.add(resolved.url.slice(0,at+14));return resolved;}return{url:pathToFileURL(safe(parent.imports[specifier])).href,shortCircuit:true};}if(modules.has(specifier))return{url:specifier,shortCircuit:true};if(typeof context.parentURL==="string"&&[...installedRoots].some(prefix=>context.parentURL.startsWith(prefix)))return next(specifier,context);throw Error("Only captured Source modules may execute");},')
rep('const row=modules.get(url);if(!row)throw Error("Only captured Source module bytes may load");',
    'const row=modules.get(url);if(!row){if([...installedRoots].some(prefix=>url.startsWith(prefix)))return next(url,context);throw Error("Only captured Source module bytes may load");}')
open(path, "w", encoding="utf8").write(t)
print("patched", path)
