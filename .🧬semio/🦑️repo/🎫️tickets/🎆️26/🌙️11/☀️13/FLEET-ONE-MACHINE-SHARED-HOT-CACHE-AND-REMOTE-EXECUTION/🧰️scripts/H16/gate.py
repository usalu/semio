import re,subprocess,sys,time
def free_percent():
    m=re.search(r"free percentage: (\d+)%",subprocess.run(["memory_pressure"],capture_output=True,text=True).stdout);return int(m.group(1)) if m else 100
def swap_free_mb():
    m=re.search(r"free = ([\d.]+)M",subprocess.run(["sysctl","-n","vm.swapusage"],capture_output=True,text=True).stdout);return float(m.group(1)) if m else 1e9
def wait(minutes=10,step=30):
    end=time.time()+minutes*60
    while free_percent()<20 or swap_free_mb()<1024:
        if time.time()>=end:return False
        time.sleep(step)
    return True
if __name__=="__main__":print("gate",wait(float(sys.argv[1]) if len(sys.argv)>1 else 10))
