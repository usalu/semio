import os,sys,time,datetime
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
text=open(sys.argv[1],encoding="utf8").read();lock=T+"/landing.lock";end=time.time()+float(sys.argv[2])
while time.time()<end:
    try:os.mkdir(lock);break
    except FileExistsError:time.sleep(5)
else:sys.exit("lock not free")
try:
    open(lock+"/owner","w").write("R87-note-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
    open(T+"/landings.md","a",encoding="utf8").write(text)
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
print("noted")
