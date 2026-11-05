import json,psutil
rows=[]
for pid,birth in [(29416,1793851018.8864615),(8048,1793851019.021866),(36580,1793851021.8016136)]:
 try:
  p=psutil.Process(pid);actual=p.create_time();rows.append({"pid":pid,"expected_birth":birth,"actual_birth":actual,"owned_present":abs(actual-birth)<0.01})
 except psutil.NoSuchProcess:rows.append({"pid":pid,"owned_present":False})
print(json.dumps(rows))
