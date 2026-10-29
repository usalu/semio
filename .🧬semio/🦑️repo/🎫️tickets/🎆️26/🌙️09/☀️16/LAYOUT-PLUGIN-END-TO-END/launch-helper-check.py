import ast
from pathlib import Path
import runpy
import subprocess
import sys
from types import SimpleNamespace
from unittest.mock import patch

script=Path(__file__).with_name("launch-layout.py").resolve()
ast.parse(script.read_text(),filename=str(script))
for operation in ("activate","serve","probe"):
 for tool in (None,"nx","bazel"):
  argv=[str(script),operation]+([] if tool is None else ["--tool",tool])
  with patch.object(sys,"argv",argv),patch.object(subprocess,"run",return_value=SimpleNamespace(returncode=0)) as child:
   try:
    runpy.run_path(str(script),run_name="__main__")
   except SystemExit as result:
    assert result.code==0
  args=child.call_args.args[0]
  assert args[0]=="repo"
  assert [word for word in args if word.startswith("--tool=")]==([] if tool is None else ["--tool="+tool])
  assert child.call_args.kwargs["cwd"]==script.parent.parents[6]
  assert child.call_args.kwargs["env"]["SEMIO_PLUGIN_CATALOG"]=="✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
print("[DEBUG] Launch helper syntax and nine backend forwarding cases passed; no child executed")
