from pathlib import Path
import sys
script=Path(__file__).with_name('law-relink-3d.py')
source=script.read_text().replace("'🗑️generated/law-relink-3d'", "'🗑️generated/current-window-scene'")
sys.argv=[str(script),'the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity','semio_s_artifact_puzzle_3d_retained_action','semio_s_plugin_puzzle_jobs']
exec(compile(source,str(script),'exec'))
