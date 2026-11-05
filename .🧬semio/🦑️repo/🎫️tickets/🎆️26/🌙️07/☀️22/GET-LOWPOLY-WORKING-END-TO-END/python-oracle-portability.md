# Python Oracle Portability

Read-only investigation, 2026-11-05. No install, PATH/environment change or source edit.

## Smallest Registered Tool Interface

Use runToolSyncV1 from process owner 🧰️tools/🏃️run/🟦️.ts. From the admission test its exact relative import is ../../../../../🧰️tools/🏃️run/🟦️.ts.

Signature: runToolSyncV1(toolId:string,args:readonly string[],options:ToolRunOptionsV1 & {encoding:BufferEncoding}):ToolSyncResultV1<string>. Call with toolId python, arguments -I,-c,model,JSON.stringify(rows), and options encoding:utf8,caller:import.meta.url,timeout:10000. Result fields status,signal,stdout:string,stderr:string,error,resolution. Assert status zero and JSON.parse(stdout). Optional check:true throws on failure. No trusted, admission:false or env override is needed. Sync interface joins the enclosing grant and is intended for brief queries.

Registry entry process /🧰️tools/🔣️.json declares Python on Windows/macOS/Linux, allowedCallers **, and candidates python3/python/py. It resolves repository-selected interpreter through existing tool environment or PATH without changes. Tool run wraps executable choice and environment sanitation; missing or broken interpreter fails explicitly.

Existing clean precedent: repo library /⚖️laws/🧰️direct-tool-declarations/🧪️tests/🟦️.ts:61 calls runToolSyncV1 with python, -I, encoding:utf8 and caller:import.meta.url. Process package exports @semio-tech/process/tools/run, but direct relative owner import avoids alias setup assumptions.

## Existing Test Resolver

Process /🧪️testing/🧰️toolchain/🟦️.ts exports testPythonV1():string|null and requireTestToolV1(tool,port?):string. It probes real Python 3, skips Windows Store stubs/Python 2, and converts Windows py -3 to its absolute interpreter. Tests/fixtures cover all these cases. This helper is portable but subsequent direct spawn does not use the requested registered tool execution interface. Prefer runToolSyncV1 for this change.

Important limitation: general registry resolveToolV1 selects the first existing executable; it does not perform the specialized test resolver's Python-version/stub probes during selection. Thus runToolSyncV1 removes the literal python3 assumption but does not independently guarantee avoiding a Windows Store stub if that precedes a real interpreter. It should fail visibly in that circumstance; do not hide it with skip/fallback or mutate environment. Root/runtime can distinguish an actual registry resolution issue if encountered.
