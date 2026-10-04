import sys
src,dst=sys.argv[1:3]
s=open(src).read()
R=[
('impl From<ValueError> for OutputError { fn from(error:ValueError)->Self{Self::Refusal(error)} }\nimpl OutputError { fn into_pack(self)->PackError{match self{Self::Refusal(error)=>PackError::ValueRefusal(error),Self::Pack(error)=>error}} }',
 'impl From<ValueError> for OutputError { fn from(error:ValueError)->Self{Self::Refusal(error)} }\nimpl From<PackRefusal> for OutputError { fn from(error:PackRefusal)->Self{Self::Pack(PackError::Refusal(error))} }\nimpl OutputError { fn into_pack(self)->PackError{match self{Self::Refusal(error)=>PackError::from(error),Self::Pack(error)=>error}} }',1),
('writer.chunk(piece,self.control).map_err(OutputError::Pack)','writer.chunk(piece,self.control).map_err(OutputError::from)',1),
('ControlledPackWriter::begin(&write,&options.limits,control).map_err(OutputError::Pack)','ControlledPackWriter::begin(&write,&options.limits,control).map_err(OutputError::from)',1),
('symbols.entries.len()as u64,control).map_err(OutputError::Pack)','symbols.entries.len()as u64,control).map_err(OutputError::from)',1),
('writer.segment(crate::KIND_DOCUMENT,frame,control).map_err(OutputError::Pack)','writer.segment(crate::KIND_DOCUMENT,frame,control).map_err(OutputError::from)',1),
('writer.finish(&manifest,control).map_err(OutputError::Pack)','writer.finish(&manifest,control).map_err(OutputError::from)',1),
('return Err(PackError::LimitExceeded("Pack output admission exceeds max_total_alloc"))','return Err(PackError::Refusal(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"Pack output admission exceeds max_total_alloc"}))',1),
]
for old,new,n in R:
    c=s.count(old)
    if c!=n: sys.exit(f'COUNT {c}!={n}: {old[:90]}')
    s=s.replace(old,new)
open(dst,'w').write(s)
print('ok')
