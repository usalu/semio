"""🏙️ Start a fresh Cargo graph after inputs change during a long-running BIM guest build."""
from pathlib import Path
import os, sys, time

ticket=Path(__file__).resolve().parent
generated=ticket/'🗑️generated'
captures=generated/'compiler-retry'
process=int(sys.argv[1])
started=float(sys.argv[2])
while True:
    try: os.kill(process,0)
    except ProcessLookupError: break
    for report in captures.glob('w-*.error'):
        if report.stat().st_mtime<started: continue
        details=report.read_text(errors='replace')
        if ('OriginalGbxml' in details or 'original_gbxml' in details or 'OpeningProjection' in details or 'original_document' in details) and ('unresolved' in details or 'not found' in details or 'cannot find' in details):
            print('A changed gbXML API requires a new Cargo dependency graph.',report.name,flush=True)
            report.with_suffix('.stop').touch()
    time.sleep(4)
