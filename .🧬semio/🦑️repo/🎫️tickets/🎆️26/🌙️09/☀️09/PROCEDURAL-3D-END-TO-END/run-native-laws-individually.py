"""🔬 Executes each listed law in isolation so a failed original custody assertion retains its evidence."""
from pathlib import Path
import subprocess,sys,concurrent.futures
binary=Path(sys.argv[1]).resolve()
out=Path(sys.argv[2]);out.mkdir(parents=True,exist_ok=True)
rows=subprocess.check_output([binary,'--list'],text=True).splitlines()
def law(name):
    destination=out/(name.rsplit('::',1)[1]+'.log')
    with destination.open('w') as log:
        result=subprocess.run([binary,'--exact',name,'--nocapture'],stdout=log,stderr=subprocess.STDOUT)
    print(result.returncode,name,flush=True)
    return result.returncode
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
    failures=list(pool.map(law,[row[:-6] for row in rows if row.endswith(': test')]))
sys.exit(any(failures))
