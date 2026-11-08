import re
def wrap_outcomes(span:str)->str:
    """wrap every brace-matched `crate::RetainedWorkOutcome{...}` / `RetainedWorkOutcome{...}` literal in RetainedWorkPoll::Complete(...)"""
    out=[];i=0
    pat=re.compile(r"(?:crate::)?RetainedWorkOutcome\s*\{")
    while True:
        m=pat.search(span,i)
        if not m: out.append(span[i:]);break
        out.append(span[i:m.start()])
        j=m.end();d=1
        while d:
            c=span[j]
            if c=="{":d+=1
            elif c=="}":d-=1
            j+=1
        lit=span[m.start():j]
        out.append("crate::RetainedWorkPoll::Complete("+lit+")")
        i=j
    return "".join(out)
