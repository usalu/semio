#!/usr/bin/env python3
"""Copy of the storage-seam set: the g2 row that turns `nativeImageSpaceV1` into a const named only the prefix `nativeImageS`, which also matches `nativeImageServiceEntryV1` (a peer added it next to it). (The platform image `opening` scope the first version of this copy carried is now in the owner's set.)"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/storage-seam-tools", "storage-seam")
patch_if(T + "/apply.py", 'sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..")))', 'sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..", "..", "..")))')
patch_if(T + "/groups/g2.py", "'export function nativeImageS', 'export const nativeImageS'),", "'export function nativeImageSpaceV1', 'export const nativeImageSpaceV1'),")
G2 = T + "/groups/g2.py"
g2 = open(G2, encoding="utf8").read()
REQ = "requireNativeRecomputableRetirementSettledV1(options.directory,options.control);"
for old, new, want in (
    ("options=Object.freeze({...options});const backend=nativeStoragePlatformV1", "options=Object.freeze({...options});" + REQ + "const backend=nativeStoragePlatformV1", 1),
    ("options=Object.freeze({...options});const backend=nativePlatformBackendV1", "options=Object.freeze({...options});" + REQ + "const backend=nativePlatformBackendV1", 1),
    ("filesystemBytes:number;maximumPhysicalBytes:number}>;control:ResourceLeaseControl", "filesystemBytes:number;maximumPhysicalBytes:number;maximumLeaseDatabaseBytes?:number}>;control:ResourceLeaseControl", 2),
    ("maximumPhysicalBytes:options.maximumPhysicalBytes}),control:options.control", "maximumPhysicalBytes:options.maximumPhysicalBytes,...(options.maximumLeaseDatabaseBytes===undefined?{}:{maximumLeaseDatabaseBytes:options.maximumLeaseDatabaseBytes})}),control:options.control", 2),
):
    if g2.count(old) == want:
        g2 = g2.replace(old, new)
open(G2, "w", encoding="utf8").write(g2)
g2 = open(G2, encoding="utf8").read()
S_ = "\U0001F9F0\ufe0fframework/\U0001F528\ufe0fmodules/\U0001F3C3\ufe0fprocess/\U0001F4E6\ufe0fartifacts/\U0001F3D7\ufe0fnative-build/\U0001F4BE\ufe0fstorage"
rows = [
    (S_ + "/\U0001F310\ufe0fplatform/\U0001F7E6\ufe0f.ts", '()=>nativeStoragePlatformV1(process.platform)==="macos"?retireNativeImageStorageV1(scope.options,request,token):retireNativeDirectoryImageStorageV1(scope.options,request,token)', '()=>nativeStorageOperationsV1(nativePlatformBackendV1(scope.options.directory,undefined,false)).retire(scope.options,request,token)'),
]
head = "EDITS = [\n"
assert g2.count(head) == 1
g2 = g2.replace(head, head + "".join("    %r,\n" % (row,) for row in rows), 1)
open(G2, "w", encoding="utf8").write(g2)
import winpath
_ops = winpath.resolve(os.path.join(T, "staged", "files"), S_ + "/\U0001F50C\ufe0fbackends/\U0001F4CB\ufe0fcontract/\U0001F50C\ufe0foperations/\U0001F7E6\ufe0f.ts")
_text = open(_ops, encoding="utf8").read()
for _old, _new in ((' retire(options:NativeFixedPlatformOptionsV1,request:NativeImageRetirementRequestV1):Promise<boolean>;', ' retire(options:NativeFixedPlatformOptionsV1,request:NativeImageRetirementRequestV1,recomputable?:NativePlatformRecomputableRetirementV1):Promise<boolean>;'), ('import type {NativeImageCleanupReadV1,', 'import type {NativePlatformRecomputableRetirementV1} from "../../../\U0001F310\ufe0fplatform/\U0001F7E6\ufe0f.ts";\nimport type {NativeImageCleanupReadV1,')):
    assert _text.count(_old) == 1, _old
    _text = _text.replace(_old, _new)
open(_ops, "w", encoding="utf8").write(_text)
print("storage-seam merged copy:", T)
