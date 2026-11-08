import sys
p=sys.argv[1];s=open(p).read()
a=s.index(" fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)");b=s.index(" fn requires_original_birth(&self)")
old=s[a:b]
dem_start=old.index(" fn demands(");dem_end=old.index(" fn advance(")
demands=old[dem_start:dem_end].replace("fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)","fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)").replace("nested(self.child()?.demands(original,body)?,m)","nested(self.child()?.demands(original,body,maximum_work_units)?,m)")
cd_start=old.index(" fn close_demands(");closedem=old[cd_start:old.index(" fn close_work_demand(")]
close_start=old.index(" fn close_work_demand(");close_work=old[close_start:old.index(" fn close_step(")]
mid=open(sys.argv[2]).read();cs=open(sys.argv[3]).read()
s=s[:a]+demands+mid+closedem+close_work+cs+s[b:]
s=s.replace("crate::Settle::settled(","crate::Polled::polled(")
open(p,'w').write(s)
