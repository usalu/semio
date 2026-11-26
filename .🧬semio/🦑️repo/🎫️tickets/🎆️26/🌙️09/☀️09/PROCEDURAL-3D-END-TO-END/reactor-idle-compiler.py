#!/usr/bin/env python3
"""🌡️ Compile the four measured reactor owners from the working tree against the native test closure."""
from pathlib import Path
import os,sys
root=Path(__file__).resolve().parents[7]
mirror=root/'.cache/g3d-native/s'
args=sys.argv[1:]
selected={'semio_plugin_reactor_surfaces','semio_plugin_reactor_lifecycle','semio_plugin_runtime_ops_document'}
if '--crate-name' in args and args[args.index('--crate-name')+1] in selected:
 for i,arg in enumerate(args):
  if arg.endswith('.rs'):
   target=Path(arg).resolve()
   if target.is_relative_to(mirror):args[i]=str(root/target.relative_to(mirror))
   break
retry=Path(__file__).parent/'🗑️generated/bridge-current/retry-compiler'
os.execv(retry,[str(retry),*args])
