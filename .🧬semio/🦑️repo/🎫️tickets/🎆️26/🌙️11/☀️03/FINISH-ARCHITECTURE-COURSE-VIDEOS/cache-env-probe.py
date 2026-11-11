import importlib
import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path.cwd()))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
expected = os.environ["SEMIO_BIM_SLAB_CACHE"]
print("parent", expected, flush=True)
code = "import os,sys; actual=os.environ.get('SEMIO_BIM_SLAB_CACHE'); print('manim-child',actual,flush=True); assert actual==sys.argv[1]"
runtime.run_command([sys.executable, "-c", code, expected], cwd=Path.cwd(), env={"SEMIO_TEACHING_LANGUAGE":"en"})
