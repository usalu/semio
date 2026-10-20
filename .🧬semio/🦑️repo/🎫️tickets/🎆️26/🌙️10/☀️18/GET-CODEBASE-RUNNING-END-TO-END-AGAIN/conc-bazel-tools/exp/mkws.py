#!/usr/bin/env python3
"""mkws.py <dir> <name> [sleep] : fixture workspace with genrule <name> executing a marker line."""
import os,sys,shutil
d,name=sys.argv[1],sys.argv[2];sl=sys.argv[3] if len(sys.argv)>3 else "4"
CB=os.environ.get("CB","/private/tmp/conc-bazel")
os.makedirs(d,exist_ok=True);os.makedirs(CB+"/markers",exist_ok=True)
open(d+"/MODULE.bazel","w").write('module(name = "semio_tasks", version = "0.0.0")\n')
shutil.copy(CB+"/catalog/MODULE.bazel.lock",d+"/MODULE.bazel.lock")
open(d+"/BUILD.bazel","w").write(f'''genrule(
    name = "{name}",
    outs = ["{name}.out"],
    cmd = "echo x >> {CB}/markers/{name}; sleep {sl}; echo {name} > $@",
    tags = ["no-sandbox"],
)
''')
