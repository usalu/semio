import os
from pathlib import Path

outer = Path(__file__).with_name("current-2d-general.py").resolve()
os.environ["PUZZLE_REFRESH_2D_V1"] = "1"
os.environ["PUZZLE_V1_DEST"] = "live-2d-ui-actor-typecheck"
os.environ["PUZZLE_GENERAL_OUTPUT"] = "ui-actor-unit.rmeta"
source = outer.read_text()
source = source.replace(
    "exec(compile(script,",
    '''script = script.replace('--emit=link,metadata', '--emit=metadata')
script = script.replace('puzzlecurrent.rlib', 'puzzlecurrent.rmeta')
script = script.replace("if lib.with_suffix('.rmeta').is_file():", "if lib.suffix == '.rlib' and lib.with_suffix('.rmeta').is_file():")
script = script.replace("subprocess.run(args+['-Z','threads=1'],", "subprocess.run(args+['-Z','threads=1','--emit=metadata'],")
script = script[:script.index('sys.exit(subprocess.run')]
exec(compile(script,''',
)
exec(compile(source, str(outer), "exec"), {"__file__": str(outer)})
