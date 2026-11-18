import time,datetime,subprocess,sys,os
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landing.lock"
def inwin():
    n=datetime.datetime.now(datetime.timezone.utc);return n.minute%30<5
while True:
    if inwin() and not os.path.exists(T):
        print("start",datetime.datetime.now(datetime.timezone.utc).isoformat(),flush=True)
        c=subprocess.call(["python3","land.py"]+sys.argv[1:])
        if c==0 or not os.path.exists(T):sys.exit(c)
        print("exit",c,flush=True)
        if c==1:sys.exit(c)
    time.sleep(2)
