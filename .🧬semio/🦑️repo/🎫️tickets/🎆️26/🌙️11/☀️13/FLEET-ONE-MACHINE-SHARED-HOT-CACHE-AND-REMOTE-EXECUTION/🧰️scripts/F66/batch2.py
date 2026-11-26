#!/usr/bin/env python3
"""F66 group: typed harness fixes. host-test `checkPlan` takes the readonly plan rows (`readonly {id,dependsOn}[]`), reservation SQLite oracles take typed rows via `db.query<Row,Params>`."""
import sys;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
B=LIB+"/🏗️build"
host=B+"/🏡️resident/🧪️tests/🌐️host/🟦️.ts"
res=B+"/🧠️memory/🧪️tests/🎫️reservation/🟦️.ts"
got,held=wait_locks({"resident","memory"},int(sys.argv[1]) if len(sys.argv)>1 else 0)
print("held",held)
try:
    if "resident" not in held:
        replace_once(host,"const checkPlan=(tasks:any[],targets:string[])=>{","const checkPlan=(tasks:readonly Readonly<{id:string;dependsOn:readonly string[]}>[],targets:string[])=>{")
    if "memory" not in held:
        P="const oracle=db.query"
        for sql,row,params,get in [
         ('("select ?2 parent,?1-?2-1024 fixed,1024 worker")','{parent:number;fixed:number;worker:number}','[number,number]','.get(reserved,parentBudget);'),
         ('("select ?1-1024 parent,0 fixed,1024 worker")','{parent:number;fixed:number;worker:number}','[number]','.get(reserved);'),
         ('("select case ?1 when \'active\' then 1915 when \'quiet\' then null when \'zero\' then null else 1024 end mb")','{mb:number|null}','[string]','.get(row.state);'),
         ('("select case when json_type(?1,\'$.next\')=\'null\' or json_extract(?1,\'$.next.workers\')<json_extract(?1,\'$.previous.workers\') or json_extract(?1,\'$.next.megabytes\')<json_extract(?1,\'$.previous.megabytes\') then 1 else 0 end retire")','{retire:number}','[string]','.get(JSON.stringify(row));')]:
            replace_once(res,P+sql+get,P+"<"+row+","+params+">"+sql+get[:-1]+"!;")
finally:release(got)
