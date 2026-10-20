import json,sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0,os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)),"..")))
import patches
from winpath import iter_logical
files=[t for t in patches.EDITS if t.endswith(".ts")]
staged=os.path.join(os.path.dirname(os.path.abspath(__file__)),"staged")
files += [rel for rel in iter_logical(staged) if rel.endswith(".ts")]
files += [t for t in patches.REWRITES if t.endswith(".ts")]
files = sorted(set(files))
json.dump({"compilerOptions":{"target":"ESNext","module":"ESNext","lib":["ESNext","DOM"],"moduleResolution":"bundler","strict":True,"esModuleInterop":True,"isolatedModules":True,"resolveJsonModule":True,"allowImportingTsExtensions":True,"skipLibCheck":True,"noEmit":True,"types":["bun"]},"files":files},open("tsconfig.conc.json","w"),ensure_ascii=False)
