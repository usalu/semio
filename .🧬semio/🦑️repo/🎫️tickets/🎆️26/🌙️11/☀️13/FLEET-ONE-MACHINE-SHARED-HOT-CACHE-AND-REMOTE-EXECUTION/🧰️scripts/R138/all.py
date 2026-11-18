import sys,importlib.util,os
D=os.path.dirname(os.path.abspath(__file__))
def load(name):
 sp=importlib.util.spec_from_file_location(name,D+"/"+name+".py");m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
steps=[load(n) for n in("land1","land2","land3")]
def apply():
 done=[]
 try:
  for m in steps:m.apply();done.append(m)
 except BaseException:
  for m in reversed(done):m.restore()
  raise
def restore():
 for m in reversed(steps):
  try:m.restore()
  except Exception as error:print("restore",error)
if __name__=="__main__":
 {"apply":apply,"restore":restore}[sys.argv[1]]()
