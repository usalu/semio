import re,sys
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/hostcheck/rc")
from ed import R
def port(path):
    p=R+path;s=open(p,encoding="utf-8").read()
    s,n=re.subn(r"(?<!settled\()(?<!::)\bchild\.close_step\((.*?)\);work\+=turn\.work;",lambda m:"settle_child_close(child.close_step("+m.group(1)+"));work+=turn.work;",s)
    # wrap: "let turn=settle_child_close(child.close_step(..));" requires prefix
    open(p,"w",encoding="utf-8").write(s);print(path,n)
