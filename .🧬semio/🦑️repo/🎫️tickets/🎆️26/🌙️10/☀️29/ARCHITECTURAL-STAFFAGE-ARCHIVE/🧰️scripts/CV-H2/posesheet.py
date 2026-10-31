import sys, json; sys.path.insert(0,'.')
from lib import *
from sheet import sheet
# usage: posesheet.py asset scale pose1,pose2,.. bounds den_dpi
aid, sc, poses = sys.argv[1], sys.argv[2], sys.argv[3].split(',')
bounds = tuple(map(float, sys.argv[4].split(',')))
dpi = int(sys.argv[5]) if len(sys.argv) > 5 else 1000
p="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"+aid+"/🔣️.json"
d=json.load(open(p))['asset']; r=d['representations'][sc]
items=[]
for ps in poses:
    w=pose_world(d,ps)
    items.append((r['geometry']['elevation'],w,int(sc),r['pens'],bounds,'line'))
print(sheet(f"{aid}-{sc}-poses",items,dpi))
