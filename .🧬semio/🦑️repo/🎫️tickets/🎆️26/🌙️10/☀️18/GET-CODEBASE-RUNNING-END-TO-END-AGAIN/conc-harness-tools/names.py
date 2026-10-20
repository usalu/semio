import os
R="/Users/ueli/Documents/semio"
V=chr(0xFE0F)
def n(cp,word): return chr(cp)+V+word
FRAMEWORK=n(0x1F9F0,"framework"); MODULES=n(0x1F528,"modules"); SCHEMA=n(0x1F9EC,"schema"); VALIDATOR=n(0x2705,"validator")
PACKAGES=n(0x1F4E6,"packages"); TYPESCRIPT=n(0x1F7E6,"typescript"); PROCESS=n(0x1F3C3,"process"); ROUTING=n(0x1F9ED,"routing")
PRODUCTS=n(0x1F6CD,"products"); REPO=n(0x1F991,"repo"); LIBRARY=n(0x1F4DA,"library"); BUILD=n(0x1F3D7,"build")
OPERATIONS=n(0x2699,"operations"); HELP=n(0x1F4D6,"help"); TOOLS=n(0x1F527,"tools"); BAZEL=n(0x1F3D7,"bazel")
TESTS=n(0x1F9EA,"tests"); FIXTURES=n(0x1F9EB,"fixtures"); CALLER=n(0x1F4E5,"caller"); CHILD=n(0x1F4E4,"child"); INVOCATION=n(0x1F9ED,"invocation")
TS=chr(0x1F7E6)+V+".ts"; JSON=chr(0x1F523)+V+".json"; SCRIPT=chr(0x1F4DC)+V+"script.ts"; CATALOG=chr(0x1F523)+V+"catalog.json"; PY=chr(0x1F40D)+V+".py"
INCR=n(0x1F4CF,"incremental"); COMMAND=n(0x2328,"command"); RUNNER=n(0x1F3C3,"runner"); OBSERVERS=n(0x1F441,"observers"); SCENARIOS=n(0x1F4CB,"scenarios")
CONC=n(0x1F91D,"concurrent"); WORKLOADS=n(0x1F3AC,"workloads"); STUB=n(0x1F3AD,"stub"); ORACLE=n(0x1F40D,"oracle")
TERMINATION=n(0x1FA93,"termination"); REQUIRES=n(0x1F6D7,"requires"); LEASES=n(0x1F512,"leases")
L=f"{FRAMEWORK}/{PRODUCTS}/{REPO}/{MODULES}/{LIBRARY}"
B=f"{L}/{BUILD}"
I=f"{B}/{INCR}"
M=f"{I}/{CONC}"
UP="../"
TOKENS={
 "SIX":UP*6,"SEVEN":UP*7,"EIGHT":UP*8,"NINE":UP*9,
 "MODULES":MODULES,"SCHEMA":SCHEMA,"VALIDATOR":VALIDATOR,"PACKAGES":PACKAGES,"TYPESCRIPT":TYPESCRIPT,"PROCESS":PROCESS,"ROUTING":ROUTING,
 "TS":TS,"JSON":JSON,"PY":PY,"TESTS":TESTS,"FIXTURES":FIXTURES,"COMMAND":COMMAND,"RUNNER":RUNNER,"OBSERVERS":OBSERVERS,"SCENARIOS":SCENARIOS,
 "CONC":CONC,"WORKLOADS":WORKLOADS,"STUB":STUB,"ORACLE":ORACLE,"INCR":INCR,"OPERATIONS":OPERATIONS,"CATALOG":CATALOG,"HELP":HELP,
 "TERMINATION":TERMINATION,"FRAMEWORK":FRAMEWORK,"LEASES":LEASES,"BUILDDIR":B,"PRODUCTS":PRODUCTS,"REPO":REPO,"LIBRARY":LIBRARY,"BUILD":BUILD,"SCRIPT":SCRIPT,"BUILDSCRIPT":f"{B}/{SCRIPT}","INVOCATION":INVOCATION,
}
def check():
    must=[f"{R}/{I}/{TS}",f"{R}/{I}/{COMMAND}/{TS}",f"{R}/{I}/{RUNNER}/{TS}",f"{R}/{I}/{OBSERVERS}/{TS}",f"{R}/{I}/{SCHEMA}/{JSON}",f"{R}/{B}/{OPERATIONS}/{CATALOG}",
      f"{R}/{B}/{OPERATIONS}/{SCHEMA}/{JSON}",f"{R}/{B}/{OPERATIONS}/{TESTS}/{FIXTURES}/{HELP}/{JSON}",f"{R}/{B}/{OPERATIONS}/{TESTS}/{REQUIRES}/{JSON}",f"{R}/{B}/{SCRIPT}",f"{R}/{B}/{CALLER}/{CHILD}/{TS}",
      f"{R}/{FRAMEWORK}/{MODULES}/{PROCESS}/{TERMINATION}/{TS}",f"{R}/{FRAMEWORK}/{MODULES}/{SCHEMA}/{VALIDATOR}/{PACKAGES}/{TYPESCRIPT}/{TS}"]
    return [p for p in must if not os.path.exists(p)]
if __name__=="__main__": print("missing:",check())
