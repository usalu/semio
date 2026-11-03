import sys,re
sys.path.insert(0,".")
from ph import *
def sites(crate,suf,w=170):
    f=find(crate,suf);s=open(R+"/"+f).read().split("\n")
    for n,l in enumerate(s,1):
        if l.lstrip().startswith("//"):continue
        for m in re.finditer(r'\.unwrap\(\)|\.expect\(|panic!\(|unreachable!|assert!\(|assert_eq!\(',l):print(n,"|",l[max(0,m.start()-w):m.end()+110])
if __name__=="__main__":sites(sys.argv[1],sys.argv[2],int(sys.argv[3]) if len(sys.argv)>3 else 170)
