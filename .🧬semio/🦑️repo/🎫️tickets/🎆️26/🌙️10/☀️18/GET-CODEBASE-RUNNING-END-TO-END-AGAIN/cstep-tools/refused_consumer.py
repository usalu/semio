"""cstep: collapse `let step = X.close_step(g)?; cx.consume_retained(step.progress())?; if let ICS::Refused{kind,progress}=step {return Err(..)}`
into a match that debits the refusal's own receipt. usage: refused_consumer.py [--write] listfile"""
import re,sys
write='--write' in sys.argv
pat=re.compile(r'(?P<i>[ \t]*)let step = (?P<call>[^;\n]*?\.close_step\([^;\n]*\))\?;\n(?P=i)(?P<cx>\w+)\.consume_retained\(step\.progress\(\)\)\?;\n(?P=i)if let (?:[\w:]*::)?InteractiveJobCloseStep::Refused \{ kind, progress \} = step \{\n[^\n]*\n(?P=i)\}\n')
tot=0
for p in open([a for a in sys.argv[1:] if not a.startswith('--')][0]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    def rep(m):
        i=m.group('i');cx=m.group('cx')
        return f"{i}let step = match {m.group('call')} {{ Ok(step) => step, Err(error) => {{ {cx}.consume_retained(error.retained_progress())?; return Err(error) }} }};\n{i}{cx}.consume_retained(step.progress())?;\n"
    ns,n=pat.subn(rep,s)
    if n:
        tot+=n;print(p[-70:],n)
        if write: open(p,'w',encoding='utf-8').write(ns)
print('total',tot)
