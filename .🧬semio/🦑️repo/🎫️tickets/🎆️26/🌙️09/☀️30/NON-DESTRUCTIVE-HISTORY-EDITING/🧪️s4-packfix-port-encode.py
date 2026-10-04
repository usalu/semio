import sys
src,dst=sys.argv[1:3]
s=open(src).read()
def rep(old,new,n=1):
    global s
    c=s.count(old)
    if c!=n: sys.exit(f'COUNT {c}!={n}: {old[:80]}')
    s=s.replace(old,new)
rep('#[derive(Debug)]\nenum OutputError { Refusal(ValueError), Pack(PackError) }\nimpl From<ValueError> for OutputError { fn from(error:ValueError)->Self{Self::Refusal(error)} }\nimpl OutputError { fn into_pack(self)->PackError{match self{Self::Refusal(error)=>PackError::ValueRefusal(error),Self::Pack(error)=>error}} }\n','')
rep('.map_err(OutputError::Pack)','',6)
rep('.map_err(OutputError::into_pack)','',3)
rep('OutputError::Refusal(','PackRefusal::ValueRefusal(',1)
rep('.map_err(OutputError::from)','.map_err(PackRefusal::from)',s.count('.map_err(OutputError::from)'))
s=s.replace('OutputError','PackRefusal')
rep('Result<Vec<u8>,PackError>','Result<Vec<u8>,PackRefusal>',3)
rep('return Err(PackError::LimitExceeded("Pack output admission exceeds max_total_alloc"))','return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"Pack output admission exceeds max_total_alloc"})')
rep('let text=semio_framework_dsl_record::print_expr_controlled(value,self.control)?;','let text=semio_framework_dsl_record::print_expr_controlled(value,self.control).map_err(PackRefusal::from)?;')
rep('let result=writer.finish(&manifest,control);\n        result','writer.finish(&manifest,control)')
open(dst,'w').write(s);print('ok')
