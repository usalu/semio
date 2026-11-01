import re,sys
H_BUILT="fn built<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{semio_framework_job::admit_original_construction(cx,output,demand,|_|semio_framework_value::RetainedWorkOutcome{result:construct(),work:1})}"
H_WITH="fn built_with<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{semio_framework_job::admit_original_construction(cx,output,demand,|control|semio_framework_value::RetainedWorkOutcome{result:construct(control),work:1})}"
H_RET="fn retired(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}"
noheads='--noheads' in sys.argv
for p in [a for a in sys.argv[1:] if not a.startswith('--')]:
    s=open(p).read()
    pat_u=re.compile(r'(?:semio_framework_job::)?admit_original_construction\(([^|]*)\|_\|')
    pat_c=re.compile(r'(?:semio_framework_job::)?admit_original_construction\(([^|]*)\|control\|')
    t,nu=pat_u.subn(r'built(\1||',s)
    t,nc=pat_c.subn(r'built_with(\1|control|',t)
    t,nr=re.subn(r'(?:semio_framework_job::)?close_original_retained_step\(','retired(',t)
    if nu+nc+nr==0:print('skip',p[-50:]);continue
    heads=[]
    if noheads:
        open(p,'w').write(t);print('ok-nohead',p[-50:],nu,nc,nr);continue
    if nu and 'fn built<' not in t:heads.append(H_BUILT)
    if nc and 'fn built_with<' not in t:heads.append(H_WITH)
    if nr and 'fn retired(' not in t:heads.append(H_RET)
    # insert after last leading use line
    lines=t.split('\n');idx=0
    for i,l in enumerate(lines):
        if l.startswith('use ') or l.startswith('//!') or l.strip()=='' or l.startswith('#['):
            if l.startswith('use '):idx=i
        elif i>idx+3 and not l.startswith('pub use') and not l.startswith('#['):
            if not l.startswith('use '):break
    lines[idx]=lines[idx]+'\n'+'\n'.join(heads) if heads else lines[idx]
    open(p,'w').write('\n'.join(lines));print('ok',p[-50:],nu,nc,nr)
