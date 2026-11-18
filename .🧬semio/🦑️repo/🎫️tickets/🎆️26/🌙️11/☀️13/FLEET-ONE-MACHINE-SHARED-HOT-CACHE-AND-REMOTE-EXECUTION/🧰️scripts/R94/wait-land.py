import time,datetime,subprocess,sys,os
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landing.lock"
end=time.time()+2400
while time.time()<end:
    n=datetime.datetime.now(datetime.timezone.utc)
    if n.minute%30<5 and n.minute%30>=0 and not (n.hour==20 and n.minute<5) and not os.path.exists(T):
        print("start",n.isoformat(),flush=True)
        c=subprocess.call(["python3","land.py"]+sys.argv[1:]);sys.exit(c)
    time.sleep(2)
print("not yet",flush=True);sys.exit(3)
