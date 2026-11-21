from pathlib import Path
import re
import sys

root = Path("🧰️framework")
admission = (root / "🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️admission/🦀️.rs").read_text()
store = Path(sys.argv[1]).read_text()
sealer = (root / "🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs").read_text()
assert "with_allocated_effect_reported(demand,work_demand" in sealer
assert 'if !crate::retained_clone_grant_admits(effect.grant(),demand)' in admission
minimum_work = dict((phase, int(work)) for phase, work in re.findall(r'"(payloadInitialize|publication)"=>(\d+)', admission))
assert minimum_work == {"payloadInitialize": 4, "publication": 4}
assert "size_of::<Option<RetainedCloneSource<T,A>>>()+size_of::<Option<u64>>()" in admission
assert "size_of::<Option<SealedShared<A>>>()*2+size_of::<Option<RetainedCloneBinding>>()*2+size_of::<Option<u64>>()*2" in admission
assert all(f"{phase}=>self.{owner}_admission.as_ref().map_or(Ok(1),admission_work)?" in store for phase,owner in [(0,"edit"),(1,"authority")]), "Source payload or publication requires four units and fixed three cannot advance"
assert "_ => admission.advance_work_demand()" in store
assert all(line in store for line in ["admission.take_output_work_demand()", "admission.take_output_demand()"])
assert "0=>self.edit_admission.as_ref().map_or_else(||nested(copy(size_of::<Box<Edit<M>>>())),admission_demand)?" in store
assert "1=>self.authority_admission.as_ref().map_or_else(||nested(copy(size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>())),admission_demand)?" in store
print("[DEBUG] Both native canonical sources fund the selected original Source work and physical transfer before each callback.")
