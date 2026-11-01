import sys,re
# usage: stepsplit.py <file> <trait-impl-start-line-substring>
p,anchor=sys.argv[1],sys.argv[2]
s=open(p).read();L=s.split('\n')
i=next(k for k,l in enumerate(L) if anchor in l)
# header lines till '{' alone or ending '{'
h=i
while not L[h].rstrip().endswith('{'):h+=1
hdr=L[i:h+1]
# step fn
a=next(k for k in range(h,len(L)) if re.match(r"\s*fn step<'a>\(&'a mut self,\s*cx",L[k]))
indent=re.match(r'\s*',L[a]).group(0)
b=a
while True:
    if L[b]==indent+'}' and b>a:break
    b+=1
body=L[a:b+1]
end=next(k for k in range(b,len(L)) if L[k]=='}')
inh=[x.replace('semio_framework_job::InteractiveJob for ','',1) for x in hdr]
inh_hdr='\n'.join(inh).replace('impl<','impl<',1)
body[0]=body[0].replace("fn step<'a>","fn step_original<'a>",1)
new_step=[indent+"fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>, _: &mut semio_framework_job::OriginalJobReturnClaim<'_>) -> semio_framework_job::InteractiveJobStep<'a> {",indent+"    semio_framework_job::InteractiveJobStep::from_callback(self.step_original(cx))",indent+"}"]
out=L[:a]+new_step+L[b+1:end+1]+['',*inh,*body,'}']+L[end+1:]
open(p,'w').write('\n'.join(out))
print('hdr:',hdr)
