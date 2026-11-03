#!/usr/bin/env python3
"""Apply from the repo root after the freeze: cold (control-less) replay output reservation.
Root cause of HistoryReplaying on the first Supersede in an unscheduled store: `reserve_output!` and the edit-settlement block in
`advance_with_operation_cap` demand a NativeDecodeControl, which the cold authority never has."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs"
s=open(p).read()
A="let control=control.as_deref_mut().ok_or_else(||VcsError::HistoryReplaying)?;control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;"
B="let control=control.as_deref_mut().ok_or(VcsError::HistoryReplaying)?;control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;"
N="match control.as_deref_mut(){Some(control)=>{control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;}None if grant==Self::cold_authority_grant()=>{}None=>return Err(VcsError::HistoryReplaying)}"
assert s.count(A)==1 and s.count(B)==1
s=s.replace(A,N).replace(B,N)
open(p,'w').write(s)
