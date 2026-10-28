"""📦️ quietfc-g: copies the new files of the staged change (`new/<repository path>`) into a root. Usage: python3 install_new.py <root>"""
import os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__));SRC=os.path.join(HERE,"new");root=sys.argv[1]
for base,_,names in os.walk(SRC):
    for name in names:
        source=os.path.join(base,name);rel=os.path.relpath(source,SRC);target=os.path.join(root,rel)
        os.makedirs(os.path.dirname(target),exist_ok=True)
        if os.path.realpath(os.path.dirname(target)).startswith(os.path.realpath(root)+"/"):shutil.copyfile(source,target)
        else:raise SystemExit("refusing to write outside the root: "+target)
print("installed")
