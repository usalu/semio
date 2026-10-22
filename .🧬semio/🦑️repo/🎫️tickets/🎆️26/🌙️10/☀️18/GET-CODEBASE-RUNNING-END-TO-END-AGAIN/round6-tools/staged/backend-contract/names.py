import os
R=os.environ.get("BKC_ROOT","/Users/ueli/Documents/semio")
V=chr(0xFE0F)
def n(cp,word): return chr(cp)+V+word
def f(cp,ext): return chr(cp)+V+ext
FRAMEWORK=n(0x1F9F0,"framework");PRODUCTS=n(0x1F6CD,"products");REPO=n(0x1F991,"repo");MODULES=n(0x1F528,"modules");LIBRARY=n(0x1F4DA,"library");BUILD=n(0x1F3D7,"build")
L=f"{FRAMEWORK}/{PRODUCTS}/{REPO}/{MODULES}/{LIBRARY}";B=f"{L}/{BUILD}"
TS=f(0x1F7E6,".ts");JSON=f(0x1F523,".json");PY=f(0x1F40D,".py");MJS=f(0x1F7E8,".mjs");SCRIPT=f(0x1F4DC,"script.ts")
T={ # token -> directory word
 "BACKENDS":n(0x1F50C,"backends"),"REGISTRY":n(0x1F4CB,"registry"),"SCHEMA":n(0x1F9EC,"schema"),"TESTS":n(0x1F9EA,"tests"),"FIXTURES":n(0x1F9EB,"fixtures"),
 "TASKMODEL":n(0x1F9EE,"task-model"),"CONF":n(0x1F4D0,"conformance"),"SCENARIOS":n(0x1F4CB,"scenarios"),"WORKSPACE":n(0x1F9EB,"workspace"),"STUB":n(0x1F3AD,"stub"),
 "RUNNER":n(0x1F3C3,"runner"),"COMMAND":n(0x2328,"command"),"ORACLE":n(0x1F40D,"oracle"),"TOOLS":n(0x1F527,"tools"),"SEMIO":n(0x1F419,"semio"),"NX":n(0x1F9F6,"nx"),"ENVELOPE":n(0x1F4E8,"envelope"),"BAZEL":n(0x1F3D7,"bazel"),
 "STORE":n(0x1F5C4,"store"),"REMOTE":n(0x1F310,"remote"),"SCHED":n(0x1F3C1,"scheduler"),"EXEC":n(0x1F680,"exec"),"BACKEND":n(0x1F50C,"backend"),"OPERATIONS":n(0x2699,"operations"),
 "OPS":n(0x2699,"operations"),"REMOTECACHE":n(0x1F310,"remote-cache"),"STORAGE":n(0x1F4BE,"storage"),"INCR":n(0x1F4CF,"incremental"),"PARITY":n(0x1FA9E,"parity"),"CIREPORT":n(0x1F9FE,"ci-report"),
 "INVOCATION":n(0x1F9ED,"invocation"),"CALLER":n(0x1F4E5,"caller"),"ORCH":n(0x1F4E4,"orchestration"),"HELP":n(0x1F4D6,"help"),"EXECUTION":n(0x1F3C3,"execution"),"ROUTING":n(0x1F9ED,"routing"),"LEASES":n(0x1F512,"leases"),"GRAPH":n(0x1F578,"graph"),"MODEMOD":n(0x1F39A,"mode"),"BUILDMOD":n(0x1F3D7,"build"),"STORERECEIPT":n(0x1F9FE,"store-receipt"),"REPORTING":n(0x1F4CA,"reporting"),"PY_ORACLE_DIR":n(0x1F40D,"oracle"),"PARITY_DIR":n(0x1FA9E,"parity"),"CHILD":n(0x1F4E4,"child"),"OBSERVERS":n(0x1F441,"observers"),"EXECCORE":n(0x1F39B,"execution"),"HERMETIC":n(0x1F3C3,"execution"),"SELECTION":n(0x1F9ED,"selection"),"PROCESS":n(0x1F3C3,"process"),"PACKAGES":n(0x1F4E6,"packages"),"TYPESCRIPT":n(0x1F7E6,"typescript"),"SCHEMAMOD":n(0x1F9EC,"schema"),"VALIDATOR":n(0x2705,"validator"),"EXECMOD":n(0x1F39B,"execution"),
}
TOKENS=dict(T);TOKENS.update({"TS":TS,"JSON":JSON,"PY":PY,"MJS":MJS,"SCRIPT":SCRIPT,"B":B,"L":L})
def p(*parts): return "/".join(parts)
