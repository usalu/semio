import os,sys,importlib
from pathlib import Path
sys.dont_write_bytecode=True
root=Path.cwd()
sys.path.insert(0,str(root))
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]=str(root / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-BIM-CLIP" / "🗑️generated" / "probe")
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
os.environ["SEMIO_TEACHING_AUDIENCE"]="spectator"
os.environ["LAYOUT_CHECK"]="1"
module=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️")
import manim
with manim.tempconfig({"dry_run":True,"skip_animations":True,"disable_caching":True,"progress_bar":"none","pixel_width":480,"pixel_height":270,"frame_rate":10}):
    module.Beat1_DataToInformation().render()
