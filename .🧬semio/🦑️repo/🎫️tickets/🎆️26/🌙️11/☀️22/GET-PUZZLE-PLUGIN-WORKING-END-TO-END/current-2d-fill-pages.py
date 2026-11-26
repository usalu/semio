from pathlib import Path
import os,sys
ticket=Path(__file__).resolve().parent
fill='semio_s_artifact_puzzle_2d_tests'
retained='semio_s_artifact_puzzle_2d_retained'
root='semio_s_artifact_puzzle_2d_v1'
if 'history' in sys.argv:
    sys.argv.remove('history')
    p=ticket/'current-2d-live-graph.py'
    s=p.read_text().replace("scratch/'live-2d-retained'", "scratch/'live-2d-retained-paged'")
    pos="    retained=scratch/"
    s=s.replace(pos,"    fill=scratch/'live-fill-pages'\n    args.extend(['-L',f'dependency={fill}'])\n    for _,alias,_,h in json.loads(libroot.read_text())['deps']:\n        if alias=='semio_s_artifact_puzzle_2d_tests':artifacts[h.to_bytes(8,'little').hex()]=fill/f'lib{alias}-fillpages.rlib'\n"+pos)
    s=s.replace("build.extend(['-L',f'dependency={fresh}'","build.extend(['-L',f'dependency={fill}','-L',f'dependency={fresh}'")
    exec(compile(s,str(p),'exec'))
else:
    p=ticket/'current-2d-general.py'
    s=p.read_text()
    loc="    if os.environ.get('PUZZLE_V1_DEST'):"
    extra="""    fill = force.replace('semio_s_artifact_puzzle_2d_retained','semio_s_artifact_puzzle_2d_tests').replace('live-2d-retained','live-fill-pages').replace('✏*/🧵️retained/📦*/🦀*','🧬*/🧬*/🔌*/🪢*/🎮*/🧪*/📦*/🦀*').replace('puzzlecurrent','fillpages')
    fill_root=original.replace('semio_s_artifact_puzzle_2d_retained','semio_s_artifact_puzzle_2d_tests')
    snippet=fill_root+fill+'\\n'+snippet.replace("for feature in json.loads", "build.extend(['-L',f'dependency={scratch}/live-fill-pages'])\\nfor feature in json.loads")
    snippet=snippet.replace('live-2d-retained','live-2d-retained-paged')
    snippet+="\\nargs.extend(['-L',f'dependency={scratch}/live-fill-pages','-L',f'dependency={scratch}/live-2d-retained-paged'])"
"""
    assert loc in s;s=s.replace(loc,extra+loc)
    exec(compile(s,str(p),'exec'))
