import re,sys
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/hostcheck/rc")
from ed import R
X="crate::RetirementPollExit"
SIG="fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||{"
NEW="fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();let mut work=0;let result=(||->Result<RetainedCloneStep,"+X+">{"
TAIL="})();RetainedWorkOutcome{result,work}}"
def port(path):
    p=R+path;s=open(p,encoding="utf-8").read()
    s=s.replace("source.bind(&mut self.source,grant)","source.bind(&mut self.source,effect)")
    s=s.replace("start_child_close(child,&mut self.child_closing,grant)","start_child_close(child,&mut self.child_closing,effect)")
    i=s.index(SIG)
    tail=TAIL if TAIL in s[i:] and (("})();crate::RetainedWorkOutcome{result,work}}" not in s[i:]) or s.index(TAIL,i)<s.index("})();crate::RetainedWorkOutcome{result,work}}",i)) else "})();crate::RetainedWorkOutcome{result,work}}"
    j=s.index(tail,i)
    body=s[i:j+len(tail)]
    body=body.replace(SIG,NEW).replace(tail,"})();"+X+"::finish(result,work)}")
    TL=len(tail)
    body,n1=re.subn(r"let turn=child\.close_step\((.*?)\);work\+=turn\.work;",lambda m:"let turn="+X+"::settled(child.close_step("+m.group(1)+"))?;work+=turn.work;",body)
    o="let turn=self.close.step_granted(effect);work+=turn.work;return turn.result;"
    n2=body.count(o);body=body.replace(o,"let turn="+X+"::settled(self.close.step_granted(effect))?;work+=turn.work;return turn.result.map_err(Into::into);")
    o="{let turn=close_retained_binding(&mut self.source,effect);work+=turn.work;turn.result}"
    n3=body.count(o);body=body.replace(o,"{let turn="+X+"::settled(close_retained_binding(&mut self.source,effect))?;work+=turn.work;turn.result.map_err(Into::into)}")
    s=s[:i]+body+s[j+TL:]
    open(p,"w",encoding="utf-8").write(s)
    print(path,n1,n2,n3)
if __name__=="__main__":
    for f in sys.argv[1:]: port(f)
