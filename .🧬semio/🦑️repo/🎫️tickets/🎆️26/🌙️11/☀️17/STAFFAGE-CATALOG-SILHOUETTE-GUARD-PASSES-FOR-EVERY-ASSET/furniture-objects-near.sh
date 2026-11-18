#!/bin/bash
# usage: near.sh asset scale view cx cy r   -> drawn shapes (world) and volume solids whose boxes touch the window
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
R="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
A=$1; S=$2; V=$3; CX=$4; CY=$5; RR=$6
echo "--- drawn $V"
$B $R/shapes-world.ts $A $S $V 2>&1 | python3 -c "
import sys,re
cx,cy,r=$CX,$CY,$RR
for l in sys.stdin:
    m=re.search(r'x (-?[\d.]+) (-?[\d.]+) y (-?[\d.]+) (-?[\d.]+)',l)
    if not m: continue
    x0,x1,y0,y1=map(float,m.groups())
    if x1>=cx-r and x0<=cx+r and y1>=cy-r and y0<=cy+r: print(l.rstrip())
"
echo "--- volume (world x,y,z min|max)"
$B $R/solids-extent.ts $A $S 2>&1 | python3 -c "
import sys,re
cx,cy,r=$CX,$CY,$RR; v='$V'
for l in sys.stdin:
    m=re.match(r'(\S+)\s+(-?[\d.]+),(-?[\d.]+),(-?[\d.]+) \| (-?[\d.]+),(-?[\d.]+),(-?[\d.]+)',l)
    if not m: continue
    n=m.group(1); a=list(map(float,m.groups()[1:4])); b=list(map(float,m.groups()[4:7]))
    ax={'elevation':(0,1),'front':(2,1),'plan':(0,2)}[v]
    x0,x1,y0,y1=a[ax[0]],b[ax[0]],a[ax[1]],b[ax[1]]
    if x1>=cx-r and x0<=cx+r and y1>=cy-r and y0<=cy+r: print(l.rstrip())
"
