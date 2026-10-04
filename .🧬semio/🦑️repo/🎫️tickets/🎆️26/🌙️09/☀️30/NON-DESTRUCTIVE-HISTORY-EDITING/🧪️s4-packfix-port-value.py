import sys,re
twin,orig,dst=sys.argv[1:4]
s=open(twin).read()
def rep(old,new,n=1):
    global s
    c=s.count(old)
    if c!=n: sys.exit(f'COUNT {c}!={n}: {old[:80]}')
    s=s.replace(old,new)
rep('use semio_framework_pack_error::PackRefusal;\n','')
rep('use crate::{write_varint_i64, write_varint_u64, ByteReader, ChunkId, CodecId, PackLimits};','use crate::os_pack::{write_varint_i64, write_varint_u64, ByteReader, ChunkId, CodecId, PackLimits, PackRefusal};')
rep('#[path = "🫳️preflight/🦀️.rs"]\nmod borrowed_preflight;\npub use borrowed_preflight::measure_document_borrowed;\n\n','')
s=re.sub(r'crate::(format::|ByteRange|KIND_)',r'crate::os_pack::\1',s)
head,sep,_=s.partition('//#region 🧪️Tests\n')
if not sep: sys.exit('no tests region')
o=open(orig).read()
_,sep2,tail=o.partition('//#region 🧪️Tests\n')
if not sep2: sys.exit('no orig tests region')
s=head+sep+tail
open(dst,'w').write(s)
print('ok',s.count('\n'))
