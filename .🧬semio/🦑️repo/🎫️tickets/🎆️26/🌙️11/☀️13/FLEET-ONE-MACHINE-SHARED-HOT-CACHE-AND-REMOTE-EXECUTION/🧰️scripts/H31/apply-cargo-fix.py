import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from patchlib import rep, sha
P = sys.argv[1]
print(P[-30:], sha(P))
rep(P, 'async next=>{await current();if(await identity(root,signal)!==tool)throw new CargoPreparationManifestChangedV1("Prepared tool changed");await settle(next,true);}', 'async next=>{await current();await settle(next,true);}')
print("cargo fix patched")
