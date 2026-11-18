import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
PATH=LIB+"/🏗️build/📖️texts/🔣️.json"
def patch(t):
    return sub1(t,'    "graph.farm-directory-kind-required": {','    "graph.unknown-source-input": {\n      "en": "Unknown selected named source input: {input} (owner {owner})",\n      "de": "Unbekannte benannte Quelleingabe: {input} (Eigentümer {owner})"\n    },\n    "graph.farm-directory-kind-required": {')
if __name__=="__main__":
    t=read(PATH);n=patch(t);print(sha(t),"->",sha(n));write(sys.argv[1],n)
