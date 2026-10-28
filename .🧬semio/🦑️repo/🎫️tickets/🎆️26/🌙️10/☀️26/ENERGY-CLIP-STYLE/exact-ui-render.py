"""📸 Render actual UI chrome for direct browser-reference comparison."""
import importlib,os,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
p=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️')
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT'])
settings={'quality':'-ql','resolution':'1600,900','fps':8,'language':'de','audience':'spectator'}
p.render_overview(output/'overview.png',settings)
p.render_interaction_frame('physical_fundamentals','focus',.26,output/'click.png',settings)
p.render_interaction_frame('physical_fundamentals','restore',0,output/'focus.png',settings)
print('[DEBUG] Actual UI overview/click/focus rendered',flush=True)
