"""🧬 Validate the positive material/carrier regression and the complete independent forward oracle."""
from pathlib import Path
import runpy,sys
T=Path(__file__).resolve().parent
for name,args in [('validate-paint-lineage.py',['green']),('validate-fragment-ray-oracle.py',[])]:
    sys.argv=[str(T/name),*args]
    try:runpy.run_path(str(T/name),run_name='__main__')
    except SystemExit as status:
        if status.code:raise
