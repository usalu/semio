import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import c4p12 as P
n=sys.argv[1]
txt=open(os.path.join(P.T,'orig',n+'.rs'),encoding='utf-8').read()
P.listing(P.Doc(txt))
