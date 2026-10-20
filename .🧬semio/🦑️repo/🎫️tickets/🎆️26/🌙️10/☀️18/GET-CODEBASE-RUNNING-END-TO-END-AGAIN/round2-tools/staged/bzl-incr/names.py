import os
R="/Users/ueli/Documents/semio"
def e(*cps): return "".join(chr(c) for c in cps)
V=chr(0xFE0F)
def n(cp,word): return chr(cp)+V+word
FRAMEWORK=n(0x1F9F0,"framework"); MODULES=n(0x1F528,"modules"); SCHEMA=n(0x1F9EC,"schema"); VALIDATOR=n(0x2705,"validator")
PACKAGES=n(0x1F4E6,"packages"); TYPESCRIPT=n(0x1F7E6,"typescript"); PROCESS=n(0x1F3C3,"process"); ROUTING=n(0x1F9ED,"routing")
EXECUTION=n(0x1F39B,"execution"); LEASES=n(0x1F512,"leases"); PRODUCTS=n(0x1F6CD,"products"); REPO=n(0x1F991,"repo"); LIBRARY=n(0x1F4DA,"library")
BUILD=n(0x1F3D7,"build"); OPERATIONS=n(0x2699,"operations"); HELP=n(0x1F4D6,"help"); REPORTING=n(0x1F4CA,"reporting"); TOOLS=n(0x1F527,"tools"); BAZEL=n(0x1F3D7,"bazel")
CACHING=n(0x26A1,"caching"); BOOTSTRAP=n(0x1F680,"bootstrap"); TESTS=n(0x1F9EA,"tests"); FIXTURES=n(0x1F9EB,"fixtures"); CALLER=n(0x1F4E5,"caller"); CHILD=n(0x1F4E4,"child")
INVOCATION=n(0x1F9ED,"invocation"); PREFLIGHT=n(0x1FA7A,"preflight"); AFFECTED=n(0x1F3AF,"affected")
TS=chr(0x1F7E6)+V+".ts"; JSON=chr(0x1F523)+V+".json"; SCRIPT=chr(0x1F4DC)+V+"script.ts"; CATALOG=chr(0x1F523)+V+"catalog.json"
PY=chr(0x1F40D)+V+".py"; NXTOOLS=n(0x1F6E0,"tools")
INCR=n(0x1F4CF,"incremental"); OBSERVERS=n(0x1F441,"observers"); RUNNER=n(0x1F3C3,"runner"); COMMAND=n(0x2328,"command"); SCENARIOS=n(0x1F4CB,"scenarios")
L=f"{FRAMEWORK}/{PRODUCTS}/{REPO}/{MODULES}/{LIBRARY}"
B=f"{L}/{BUILD}"
M=f"{B}/{INCR}"
TOKENS={
 "SIX":"../../../../../..", "SEVEN":"../../../../../../..",
 "MODULES":MODULES,"SCHEMA":SCHEMA,"VALIDATOR":VALIDATOR,"PACKAGES":PACKAGES,"TYPESCRIPT":TYPESCRIPT,"PROCESS":PROCESS,"ROUTING":ROUTING,"EXECUTION":EXECUTION,"LEASES":LEASES,
 "TS":TS,"JSON":JSON,"REPORTING":REPORTING,"TESTS":TESTS,"FIXTURES":FIXTURES,"OBSERVERS":OBSERVERS,"RUNNER":RUNNER,"COMMAND":COMMAND,"SCENARIOS":SCENARIOS,
 "OPERATIONS":OPERATIONS,"HELP":HELP,"INCR":INCR,"TOOLS":TOOLS,"BAZEL":BAZEL,"SCRIPT":SCRIPT,"FRAMEWORK":FRAMEWORK,"CACHING":CACHING,"BOOTSTRAP":BOOTSTRAP,"PY":PY,
 "BZCOMMANDS":TOOLS+"/"+BAZEL+"/"+n(0x1F9ED,"commands")+"/"+TS,"BUILDDIR":B,"PYDIR":n(0x1F40D,"oracle"),"FW":FRAMEWORK,"CATALOG":CATALOG,"NXTOOLS":NXTOOLS,"BUILDSCRIPT":f"{B}/{SCRIPT}",
}
def check():
    must=[f"{R}/{FRAMEWORK}/{MODULES}/{SCHEMA}/{VALIDATOR}/{PACKAGES}/{TYPESCRIPT}/{TS}", f"{R}/{FRAMEWORK}/{MODULES}/{PROCESS}/{PACKAGES}/{TYPESCRIPT}/{ROUTING}/{TS}",
      f"{R}/{FRAMEWORK}/{MODULES}/{PROCESS}/{PACKAGES}/{TYPESCRIPT}/{EXECUTION}/{TS}", f"{R}/{FRAMEWORK}/{MODULES}/{PROCESS}/{PACKAGES}/{TYPESCRIPT}/{LEASES}/{TS}",
      f"{R}/{B}/{REPORTING}/{TS}", f"{R}/{B}/{OPERATIONS}/{CATALOG}", f"{R}/{B}/{OPERATIONS}/{HELP}/{TS}", f"{R}/{B}/{TOOLS}/{BAZEL}/{SCRIPT}", f"{R}/{B}/{SCRIPT}",
      f"{R}/{B}/{CALLER}/{CHILD}/{TS}", f"{R}/{B}/{PREFLIGHT}/{TS}", f"{R}/{L}/{CACHING}/{BOOTSTRAP}/{NXTOOLS}/{OPERATIONS}/{TS}", f"{R}/{B}/{OPERATIONS}/{SCHEMA}/{JSON}"]
    bad=[p for p in must if not os.path.exists(p)]
    return bad
if __name__=="__main__":
    print("missing:",check())
