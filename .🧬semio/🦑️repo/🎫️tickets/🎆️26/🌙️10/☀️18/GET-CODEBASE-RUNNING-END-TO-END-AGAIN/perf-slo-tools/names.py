import importlib.util,os,sys
here=os.path.dirname(os.path.abspath(__file__))
_spec=importlib.util.spec_from_file_location("conc_names",os.path.join(here,"..","conc-harness-tools","names.py"));_conc=importlib.util.module_from_spec(_spec);_spec.loader.exec_module(_conc)
globals().update({k:v for k,v in vars(_conc).items() if not k.startswith("__")})
BENCH=n(0x1F3CE,"bench"); SLO=n(0x1F3AF,"slo"); BENCHDIR=f"{I}/{BENCH}"
TOKENS=dict(TOKENS);TOKENS.update({"BENCH":BENCH,"SLO":SLO,"BENCHDIR":BENCHDIR,"SCRIPTPATH":B+"/"+SCRIPT})
def check_bench():
    must=[f"{R}/{I}/{TS}",f"{R}/{I}/{COMMAND}/{TS}",f"{R}/{I}/{RUNNER}/{TS}",f"{R}/{I}/{OBSERVERS}/{TS}",f"{R}/{M}/{COMMAND}/{TS}",f"{R}/{M}/{TS}",f"{R}/{B}/{OPERATIONS}/{CATALOG}"]
    return [p for p in must if not os.path.exists(p)]
if __name__=="__main__": print("missing:",check_bench())
