from ph import *
cc="semio-framework-value-control-contract"
E(cc,"🧬️contract/🦀️.rs","""impl RetainedCloneProgress {
    pub fn checked_add(""","""impl RetainedCloneProgress {
    /// ➕️ Error-path accounting saturates: an overflowed total fits no grant, so it fails closed without a second error channel; success paths keep `checked_add`.
    pub fn saturating_add(self, other: Self) -> Self {
        Self { copied_items: self.copied_items.saturating_add(other.copied_items), copied_bytes: self.copied_bytes.saturating_add(other.copied_bytes), retained_capacity_bytes: self.retained_capacity_bytes.saturating_add(other.retained_capacity_bytes), released_bytes: self.released_bytes.saturating_add(other.released_bytes) }
    }

    pub fn checked_add(""")
no="semio-framework-value-native-owned"
E(no,"🧳️owned/🦀️.rs",'e.progress=self.progress.checked_add(e.progress).expect("actual native progress fits the same grant");','e.progress=self.progress.saturating_add(e.progress);')
E(no,"🧳️owned/🦀️.rs",'self.progress=self.progress.checked_add(p).expect("actual native progress fits the same grant");','self.progress=match self.progress.checked_add(p){Ok(total)=>total,Err(error)=>return Err(self.refuse(error))};')
S="🔎️schema/🦀️.rs"
E(no,S,'let p=error.retained_progress().checked_add(own).expect("one bounded schema probe fits its original grant");error.with_retained_progress(p)','error.with_retained_progress(error.retained_progress().saturating_add(own))')
E(no,S,'let p=progress.checked_add(e.retained_progress()).expect("schema quote refusal retains prior funded progress");e.with_retained_progress(p)','e.with_retained_progress(progress.saturating_add(e.retained_progress()))')
E(no,S,'let p=progress.checked_add(e.retained_progress()).expect("schema failure retains every prior funded probe");e.with_retained_progress(p)','e.with_retained_progress(progress.saturating_add(e.retained_progress()))')
E(no,S,'progress=progress.checked_add(p).expect("schema probes fit their same admitted physical grant");','progress=progress.checked_add(p).map_err(|e|e.with_retained_progress(progress))?;')
E(no,"🧱️bulk/🦀️.rs",'error.progress=error.progress.checked_add(own).expect("one bounded tree probe fits its admitted receipt");','error.progress=error.progress.saturating_add(own);')
if __name__=="__main__":build()
