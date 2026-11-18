import subprocess,sys,time,os
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
stage,label,budget=sys.argv[1],sys.argv[2],float(sys.argv[3])
end=time.time()+budget
while time.time()<end:
    if os.path.exists(T+"/landing.lock"):time.sleep(8);continue
    r=subprocess.run([sys.executable,os.path.dirname(os.path.abspath(__file__))+"/land.py",stage,label,"--land"],text=True,capture_output=True);out=r.stdout+r.stderr
    if "landing.lock is held" in out:time.sleep(8);continue
    print(out[-6000:]);sys.exit(r.returncode)
print("still waiting for landing.lock");sys.exit(3)
