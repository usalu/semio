"""🦀 Retry a captured compiler unit with the repository's serial LLVM settings."""
import json
import os
from pathlib import Path
import sys
request=json.loads(Path(sys.argv[1]).read_text())
os.chdir(request["cwd"])
os.environ.update(request["cargo"])
arguments=request["arguments"]
if "-Zno-parallel-backend" not in arguments:
    arguments.extend(["-Zno-parallel-backend","-Zthreads=1"])
os.execv(request["compiler"], [request["compiler"], *arguments])
