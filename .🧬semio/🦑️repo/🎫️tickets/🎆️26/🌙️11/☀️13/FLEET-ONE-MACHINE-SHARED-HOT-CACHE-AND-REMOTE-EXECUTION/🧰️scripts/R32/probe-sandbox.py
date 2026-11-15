"""R32 (a): the Seatbelt profile of the repo (read oracle: deny file-read-data below the root, directories listable; remote execution: deny file-read-data of hidden trees) never refuses metadata. usage: probe-sandbox.py <scratch dir>. Builds <scratch>/g with image.json, runs python3 under sandbox-exec with the profile and prints the errno of: fstatat of an absent name, fstatat of the existing file, open of the existing file."""
import os,subprocess,sys
scratch=os.path.realpath(sys.argv[1]);g=scratch+"/g";os.makedirs(g,exist_ok=True);open(g+"/image.json","w").write("{}")
profile='(version 1)(allow default)(deny file-read-data (subpath "%s"))(allow file-read-data (vnode-type DIRECTORY))'%scratch
code='''import os,sys
fd=os.open(sys.argv[1],os.O_RDONLY|os.O_DIRECTORY)
def t(label,fn):
    try:fn();print(label,"ok")
    except OSError as e:print(label,e.errno,e.strerror)
t("fstatat absent .reset.json",lambda:os.stat(".reset.json",dir_fd=fd,follow_symlinks=False))
t("fstatat existing image.json",lambda:os.stat("image.json",dir_fd=fd,follow_symlinks=False))
t("open existing image.json",lambda:os.open("image.json",os.O_RDONLY,dir_fd=fd))
'''
r=subprocess.run(["sandbox-exec","-p",profile,"python3","-I","-c",code,g],capture_output=True,text=True);print(r.stdout+r.stderr)
