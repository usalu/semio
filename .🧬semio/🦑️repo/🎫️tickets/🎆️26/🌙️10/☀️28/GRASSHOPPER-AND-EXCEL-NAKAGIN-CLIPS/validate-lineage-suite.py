"""🧬 Run the permanent lineage suite and cached actual component without rendering."""
from pathlib import Path
import runpy,sys
T=Path(__file__).resolve().parent
for name,args in [('validate-fragment-kernel.py',[]),('validate-lineage-bsp.py',['green']),('validate-paint-lineage.py',['red'])]:
    sys.argv=[str(T/name),*args]
    try:runpy.run_path(str(T/name),run_name='__main__')
    except SystemExit as status:
        if status.code:raise
