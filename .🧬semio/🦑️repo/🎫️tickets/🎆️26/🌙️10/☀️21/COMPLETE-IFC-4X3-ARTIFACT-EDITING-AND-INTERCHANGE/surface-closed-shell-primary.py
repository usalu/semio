"""🧭 Pin released closed-shell normal semantics without changing fixture data."""
from pathlib import Path
import hashlib,urllib.request
p=Path(__file__).parent;url='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcClosedShell.htm';raw=urllib.request.urlopen(url).read();(p/'🗑️generated/surface-geometry/IfcClosedShell.htm').write_bytes(raw);print('[DEBUG] '+hashlib.sha256(raw).hexdigest())
