import re,sys
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs"
s=open(p,encoding='utf8').read()
assert "ApplyBatchGroup" not in s
s=s.replace("use crate::lifted_store::ArtifactStore;","use semio_framework_os_store_engine_live::lifted_store::StoreParts;",1)
head_old="""impl<P, Mu> ArtifactStore<P, Mu>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mu: Clone + ToValue + FromValue + Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
"""
assert head_old in s
sigs=[]
def strip(m):
    sig=m.group(1)
    sigs.append(sig)
    return "    fn "+sig+" {"
s=re.sub(r"^    pub fn ((?:stage|adopt|abort)_apply_batch_group\(.*?)\s*\{$",strip,s,flags=re.M)
assert len(sigs)==3,sigs
trait="/// 🤝️ Publishes one store's member group under a common decision: stage while the decision is pending, adopt after it commits, abort after it refuses.\npub trait ApplyBatchGroup<P, Mu> {\n"+"".join("    fn "+x+";\n" for x in sigs)+"}\n\n"
head_new="impl<P, Mu> ApplyBatchGroup<P, Mu> for StoreParts<'_, P, Mu>\nwhere\n    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,\n    Mu: Clone + ToValue + FromValue + Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,\n{\n"
s=s.replace(head_old,trait+head_new,1)
assert s.count("&mut self.parts()")==3
s=s.replace("&mut self.parts()","self")
assert s.count("self.reserve_edit_history_slot()")==1
s=s.replace("self.reserve_edit_history_slot()","self.owners.reserve_edit_history_slot(self.log)")
open(p,"w",encoding='utf8').write(s)
print("ok",len(s.encode()))
