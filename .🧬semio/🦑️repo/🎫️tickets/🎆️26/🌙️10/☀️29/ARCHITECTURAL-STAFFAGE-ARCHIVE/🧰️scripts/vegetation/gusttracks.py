import sys
# usage: gusttracks.py "id1,id2" -> load tracks with the shared gust envelope (formatting)
ks=[(0,0.05),(1.2,0.45),(2.5,2.75),(3.6,1.4),(5,0.5),(8,0.05)]
print(",\n".join('{"target":"load:%s","ease":"smooth","keys":[%s]}'%(i,",".join('{"t":%s,"v":%s}'%k for k in ks)) for i in sys.argv[1].split(",")))
