from pathlib import Path
import os
ticket = Path(__file__).resolve().parent
script = (ticket/'current-5d-relink.py').read_text()
script = script.replace('current-5d-relink','current-2d-hist-relink').replace('semio_s_artifact_puzzle_5d_v1_unit_tests','semio_s_artifact_puzzle_2d_unit_tests').replace('semio-s-artifact-puzzle-5d-v1-unit-tests/*','semio-s-artifact-puzzle-2d-unit-tests/0db9a7725c910a43').replace('🖐*/🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🔬️unit-2','◻️2d/🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🔬️unit').replace("else 'tests')","else "+repr(os.environ.get("PUZZLE_GENERAL_OUTPUT","general-tests"))+")").replace('subprocess.run(args, env=env,check=True)',"subprocess.run(args+['-Z','threads=1'], env=env,check=True)")
script = script.replace("dependencies = meta['deps']", """dependencies = meta['deps']
import re
metadata_file = next(libs.glob('libsemio_s_artifact_puzzle_2d_v1-*.rmeta'))
listing = subprocess.check_output([str(toolchain/'rustc'), '-Zls=root', str(metadata_file)], text=True)
for package_name, alias, kind, checksum in dependencies:
    key = checksum.to_bytes(8, 'little').hex()
    if key not in artifacts:
        candidates = list(libs.glob('lib' + alias + '-*.rlib'))
        matches = [p for p in candidates if p.stem[3:] + ' hash ' in listing]
        if len(matches) == 1 or len(candidates) == 1:
            artifacts[key] = (matches or candidates)[0]
        elif candidates:
            def agreement(candidate):
                rows = subprocess.check_output([str(toolchain/'rustc'), '-Zls=root', str(candidate.with_suffix('.rmeta'))], text=True)
                return sum(row.split(' hash ')[0].split()[-1] + ' hash ' in listing for row in rows.splitlines() if ' hash ' in row and row.split(' hash ')[0].split())
            artifacts[key] = max(candidates, key=agreement)""")
if os.environ.get('PUZZLE_REFRESH_2D_V1'):
    recipe = (ticket/'current-3d-live-graph.py').read_text()
    snippet = recipe[recipe.index("    alias='semio_s_artifact_puzzle_3d_editor'"):recipe.index('\n"""', recipe.index("    alias='semio_s_artifact_puzzle_3d_editor'"))]
    import textwrap
    snippet = textwrap.dedent(snippet).replace('semio_s_artifact_puzzle_3d_editor','semio_s_artifact_puzzle_2d_v1').replace('🧊*/🏅*/🔖*/🪆*/✳*/✏*/📦*/🦀*','◻️2d/🏅*/🔖*/📦*/🦀*').replace("scratch/'live-editor'", "scratch/'live-2d-v1'")
    before = "rootchecksum=next(r for r in dependencies if r[1]=='semio_s_artifact_puzzle_2d_v1')[3].to_bytes(8,'little').hex()\nlibroot=artifacts[rootchecksum].resolve().parent.parent/'fingerprint/lib-semio_s_artifact_puzzle_2d_v1.json'\n"
    if os.environ.get('PUZZLE_REFRESH_2D_FORCE'):
        force = snippet.replace('semio_s_artifact_puzzle_2d_v1','semio_s_artifact_puzzle_2d_retained').replace('live-2d-v1','live-2d-retained').replace('◻️2d/🏅*/🔖*/📦*/🦀*','◻️2d/🏅*/🔖*/🪆*/✳*/✏*/🧵️retained/📦*/🦀*')
        original = before+"rootchecksum=next(r for r in json.loads(libroot.read_text())['deps'] if r[1]=='semio_s_artifact_puzzle_2d_retained')[3].to_bytes(8,'little').hex()\nlibroot=artifacts[rootchecksum].resolve().parent.parent/'fingerprint/lib-semio_s_artifact_puzzle_2d_retained.json'\n"
        snippet = original + force + '\n' + before + snippet.replace('for feature in json.loads', "build.extend(['-L',f'dependency={scratch}/live-2d-retained'])\nfor feature in json.loads")
    if os.environ.get('PUZZLE_V1_DEST'):
        snippet=snippet.replace("scratch/'live-2d-v1'", "scratch/"+repr(os.environ['PUZZLE_V1_DEST']))
    script = script.replace('for _, alias, _, checksum in dependencies:',before+snippet+'\nfor _, alias, _, checksum in dependencies:')
exec(compile(script,str(ticket/'current-5d-relink.py'),'exec'))
