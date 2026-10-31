"""🔻️ Execute the native volume regression without filtering its assertions."""
import importlib,sys
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root))
m=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔻️volume');m.test_positive_near_coplanar_volume_matches_geos_and_affinely_conditioned_scipy();print('[DEBUG] native volume contract GREEN')
