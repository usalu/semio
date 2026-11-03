import json,sys
sys.stdout.reconfigure(encoding='utf8')
S='C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/'
d=json.load(open(S+sys.argv[1]+'/🔣️.json',encoding='utf8'))
a=d['asset']
def sh(o,depth=0,maxd=3):
    if isinstance(o,dict):
        for k,v in o.items():
            if isinstance(v,(dict,list)) and depth<maxd:
                print('  '*depth+k+(' [%d]'%len(v)))
                sh(v,depth+1,maxd)
            else:
                print('  '*depth+k+': '+json.dumps(v,ensure_ascii=False)[:150])
    elif isinstance(o,list):
        for i,v in enumerate(o[:3]):
            print('  '*depth+'- #%d'%i)
            sh(v,depth+1,maxd)
sh(a,0,int(sys.argv[2]) if len(sys.argv)>2 else 3)
