#!/usr/bin/env python3
"""🔗️ e6: the vouch learns which directories of its paths are symbolic links (found once by a complete look, sealed with the cursor) and never vouches below them; glue passes the discoverer."""
import sys
root = sys.argv[1]
N = root + "/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/"
J = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🪪️vouch/🟦️.ts"
def edit(path, pairs):
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert text.count(old) == 1, (path[-40:], text.count(old), old[:70])
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
edit(N + "🧬️schema/🔣️.json", [
 ('''"required": ["version", "consumer", "key", "cursor"],''', '''"required": ["version", "consumer", "key", "cursor", "linked"],'''),
 ('''        "cursor": {"$ref": "#/definitions/Cursor"}
''', '''        "cursor": {"$ref": "#/definitions/Cursor"},
        "linked": {"type": "array", "items": {"type": "string", "minLength": 1, "maxLength": 4096}, "maxItems": 100000, "$comment": "the workspace-relative directories above the verified paths that are symbolic links: a change made through a link is reported at its target, so nothing below one is vouched by path"}
'''),
 ('"maximumRecordBytes": 4096', '"maximumRecordBytes": 4194304'),
])
edit(N + "🟦️.ts", [
 ('''export type VouchRecordV1=Readonly<{version:1;consumer:string;key:string;cursor:string}>;''', '''export type VouchRecordV1=Readonly<{version:1;consumer:string;key:string;cursor:string;linked:readonly string[]}>;
/** 🔏️ What a seal keeps for a later look: the cursor and the symbolic-link directories above the verified paths. */
export type VouchSealV1=Readonly<{cursor:ChangeCursorV1;linked:readonly string[]}>;'''),
 ('''/** 📖️ Reads the cursor sealed for `consumer` when it names `key`; anything else (absent, torn, foreign, another key) is no record. */
export function readVouchRecordV1(directory:string,consumer:string,key:string):ChangeCursorV1|undefined{
 try{
  const text=readFileSync(join(directory,vouchFileV1(consumer)),"utf8");
  if(text.length>VOUCH_POLICY_V1.maximumRecordBytes)return undefined;
  const value=JSON.parse(text) as Partial<VouchRecordV1>;
  return value.version===1&&value.consumer===consumer&&value.key===key?parseChangeCursorV1(value.cursor):undefined;
 }catch{return undefined;}
}''', '''/** 📖️ Reads the seal kept for `consumer` when it names `key`; anything else (absent, torn, foreign, another key, no link list) is no record. */
export function readVouchSealV1(directory:string,consumer:string,key:string):VouchSealV1|undefined{
 try{
  const text=readFileSync(join(directory,vouchFileV1(consumer)),"utf8");
  if(text.length>VOUCH_POLICY_V1.maximumRecordBytes)return undefined;
  const value=JSON.parse(text) as Partial<VouchRecordV1>;
  const cursor=value.version===1&&value.consumer===consumer&&value.key===key&&Array.isArray(value.linked)&&value.linked.every(link=>typeof link==="string")?parseChangeCursorV1(value.cursor):undefined;
  return cursor===undefined?undefined:{cursor,linked:value.linked!};
 }catch{return undefined;}
}

/** 📖️ Reads the cursor sealed for `consumer` when it names `key`. */
export function readVouchRecordV1(directory:string,consumer:string,key:string):ChangeCursorV1|undefined{return readVouchSealV1(directory,consumer,key)?.cursor;}'''),
 ('''export function writeVouchRecordV1(directory:string,consumer:string,key:string,cursor:ChangeCursorV1):void{''', '''export function writeVouchRecordV1(directory:string,consumer:string,key:string,cursor:ChangeCursorV1,linked:readonly string[]=[]):void{'''),
 ('''consumer,key,cursor:formatChangeCursorV1(cursor)} satisfies VouchRecordV1)''', '''consumer,key,cursor:formatChangeCursorV1(cursor),linked:[...linked]} satisfies VouchRecordV1)'''),
 ('''options:Readonly<{client:ChangeJournalClientV1;directory:string;consumer:string;key:string;insensitive?:boolean;linked?:readonly string[];signal?:AbortSignal}>):Promise<VouchV1>{
 const {client,directory,consumer,key,signal,linked=[]}=options,insensitive=options.insensitive??(process.platform==="darwin"||process.platform==="win32"),stored=readVouchRecordV1(directory,consumer,key);''', '''options:Readonly<{client:ChangeJournalClientV1;directory:string;consumer:string;key:string;insensitive?:boolean;linked?:readonly string[];discoverLinked?:(paths:ReadonlySet<string>)=>Promise<readonly string[]>;signal?:AbortSignal}>):Promise<VouchV1>{
 const {client,directory,consumer,key,signal,discoverLinked}=options,insensitive=options.insensitive??(process.platform==="darwin"||process.platform==="win32"),sealed=readVouchSealV1(directory,consumer,key),stored=sealed?.cursor;
 let linked:readonly string[]=[...new Set([...(options.linked??[]),...(sealed?.linked??[])])];'''),
 ('''    cursor=answer.cursor;anchor??=answer.cursor;rows=mergeChangeRowsV1([...rows,...answer.rows]);
    return provenUnchangedV1(paths,rows,excluded,insensitive,linked);''', '''    cursor=answer.cursor;anchor??=answer.cursor;rows=mergeChangeRowsV1([...rows,...answer.rows]);
    if(mode==="cursor"&&discoverLinked){const asked=new Set(paths);linked=[...new Set([...linked,...await discoverLinked(asked)])];return provenUnchangedV1(asked,rows,excluded,insensitive,linked);}
    return provenUnchangedV1(paths,rows,excluded,insensitive,linked);'''),
 ('''  async seal(){const sealed=anchor??cursor;if(mode!=="walk"&&sealed!==undefined&&!broken)writeVouchRecordV1(directory,consumer,key,sealed);},''', '''  async seal(){const point=anchor??cursor;if(mode!=="walk"&&point!==undefined&&!broken)writeVouchRecordV1(directory,consumer,key,point,linked);},'''),
])
edit(J, [
 ('''import {join} from "node:path";''', '''import {join} from "node:path";
import {symlinkedDirectoriesV1} from "../🔭️world/🟦️.ts";'''),
 ('''export function openJournalVouchV1(root:string,consumer:string,key:string,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal):Promise<VouchV1>{
 const canonical=canonicalRootV1(root);
 return openVouchV1({client:connectChangeJournalV1(canonical,environment),directory:join(changeJournalDirectoryV1(canonical,environment),VOUCH_POLICY_V1.directory),consumer,key,...(signal?{signal}:{})});''', '''export function openJournalVouchV1(root:string,consumer:string,key:string,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal,ports:Readonly<{client?:ChangeJournalClientV1;directory?:string}>={}):Promise<VouchV1>{
 const canonical=canonicalRootV1(root);
 return openVouchV1({client:ports.client??connectChangeJournalV1(canonical,environment),directory:ports.directory??join(changeJournalDirectoryV1(canonical,environment),VOUCH_POLICY_V1.directory),consumer,key,discoverLinked:async paths=>symlinkedDirectoriesV1(canonical,[...paths].map(path=>join(canonical,path))),...(signal?{signal}:{})});'''),
 ('''import {changeJournalDirectoryV1,connectChangeJournalV1} from "../🟦️.ts";''', '''import type {ChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../🟦️.ts";'''),
])
