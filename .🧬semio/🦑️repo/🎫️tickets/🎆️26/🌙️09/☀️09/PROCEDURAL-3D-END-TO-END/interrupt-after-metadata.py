"""🔁 Emit a Cargo artifact receipt before a one-time interrupted compiler exit."""
import os
from pathlib import Path
import signal
import sys
mark = Path(sys.argv[1])
print('{"$message_type":"artifact","artifact":"fixture.rmeta","emit":"metadata"}',file=sys.stderr,flush=True)
if not mark.exists():
    mark.touch()
    os.kill(os.getpid(),signal.SIGTERM)
print('{"$message_type":"artifact","artifact":"fixture.rlib","emit":"link"}',file=sys.stderr,flush=True)
