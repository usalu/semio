#!/usr/bin/env python3
"""Apply after act-green/freeze-lifted. Fault proven by etest-9 (bestest_load): `EnergyModelSnapshot::from_model` -> receive_stepped -> "native sealed field requires its original construction recipient". The bridge drives receiver.advance without installing a NativeDecodeConstructionRecipient (fem installs one per step)."""
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/🦀️.rs"
s=open(P).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(a[:70],s.count(a));s=s.replace(a,b)
rep("use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,NativeReceive,NativeReceiver};",
"use semio_framework_value_native_owned::native_decoding::{NativeDecodeConstructionRecipient,NativeDecodeControl,NativeDecodeRetirementRecipient,NativeReceive,NativeReceiver};\nuse semio_framework_value::{RetainedCloneProgress,RetirementDemand,retained_clone::retained_clone_grant_admits};")
rep("/// 🌉️ Receives one canonical document value","""/// 🏗️ One funded step wallet that admits each construction demand once against the bounded step grant.
struct StepRecipient{grant:RetainedCloneGrant,progress:RetainedCloneProgress}
impl NativeDecodeConstructionRecipient for StepRecipient{
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{
  let left=RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),..self.grant};
  if retained_clone_grant_admits(left,demand){Ok(left)}else{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"the energy snapshot bridge step grant does not admit a construction"))}
 }
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{
  if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"a bridge construction receipt exceeds its grant").with_retained_progress(progress))}
  self.progress=self.progress.checked_add(progress)?;Ok(())
 }
}

/// 🌉️ Receives one canonical document value""")
rep("receiver.advance(original,&mut native,wallet.maximum_copy_bytes,wallet).result.map_err(bridge_primary)?;",
"let mut recipient=StepRecipient{grant:wallet,progress:Default::default()};native.with_construction_recipient(&mut recipient,|native|receiver.advance(original,native,wallet.maximum_copy_bytes,wallet).result.map_err(bridge_primary))?;")
open(P,'w').write(s);print("applied")
